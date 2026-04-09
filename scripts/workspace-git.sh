#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# shellcheck source=scripts/lib/m055-workspace.sh
source "$ROOT_DIR/scripts/lib/m055-workspace.sh"

fail() {
  printf 'workspace-git: %s\n' "$1" >&2
  exit 1
}

normalize_git_url() {
  python3 - "$1" <<'PY'
import sys
value = sys.argv[1].strip()
if value.endswith('.git'):
    value = value[:-4]
print(value.rstrip('/'))
PY
}

remote_matches_expected() {
  local normalized_actual="$1"
  local normalized_expected="$2"

  if [[ "$normalized_actual" == "$normalized_expected" ]]; then
    return 0
  fi

  if [[ "$normalized_expected" == 'https://github.com/hyperpush-org/hyperpush-mono' && "$normalized_actual" == 'https://github.com/hyperpush-org/hyperpush' ]]; then
    return 0
  fi

  return 1
}

require_git_remote() {
  local repo_root="$1"
  local expected_url="$2"
  local label="$3"
  local actual_url

  [[ -d "$repo_root/.git" ]] || fail "$label is not a git repo: $repo_root"

  actual_url="$(git -C "$repo_root" remote get-url origin 2>/dev/null || true)"
  [[ -n "$actual_url" ]] || fail "$label is missing origin remote: $repo_root"

  local normalized_actual normalized_expected
  normalized_actual="$(normalize_git_url "$actual_url")"
  normalized_expected="$(normalize_git_url "$expected_url")"
  remote_matches_expected "$normalized_actual" "$normalized_expected" || fail "$label origin remote drifted: expected $normalized_expected, found $normalized_actual"
}

LANGUAGE_DIR="$(m055_repo_identity_field "$ROOT_DIR" 'languageRepo.workspaceDir')"
LANGUAGE_GIT_URL="$(m055_repo_identity_field "$ROOT_DIR" 'languageRepo.gitUrl')"
PRODUCT_DIR="$(m055_repo_identity_field "$ROOT_DIR" 'productRepo.workspaceDir')"
PRODUCT_GIT_URL="$(m055_repo_identity_field "$ROOT_DIR" 'productRepo.gitUrl')"
PRODUCT_ROOT="$(m055_resolve_hyperpush_root "$ROOT_DIR")" || exit 1

require_git_remote "$ROOT_DIR" "$LANGUAGE_GIT_URL" "$LANGUAGE_DIR"
require_git_remote "$PRODUCT_ROOT" "$PRODUCT_GIT_URL" "$PRODUCT_DIR"

usage() {
  cat <<EOF
Usage:
  bash scripts/workspace-git.sh status
  bash scripts/workspace-git.sh push <mesh-lang|hyperpush-mono|both> [--dry-run]

Notes:
  - The helper pushes the currently checked-out branch in each target repo.
  - It fails if a target repo has uncommitted changes or the origin remote drifts
    from scripts/lib/repo-identity.json.
EOF
}

repo_root() {
  case "$1" in
    mesh-lang) printf '%s\n' "$ROOT_DIR" ;;
    hyperpush-mono) printf '%s\n' "$PRODUCT_ROOT" ;;
    *) fail "unknown repo '$1' (expected mesh-lang or hyperpush-mono)" ;;
  esac
}

print_repo_status() {
  local name="$1"
  local root
  root="$(repo_root "$name")"

  local branch remote clean_flag
  branch="$(git -C "$root" branch --show-current)"
  remote="$(git -C "$root" remote get-url origin)"
  if [[ -z "$(git -C "$root" status --porcelain)" ]]; then
    clean_flag='clean'
  else
    clean_flag='dirty'
  fi

  printf '=== %s ===\n' "$name"
  printf 'root: %s\n' "$root"
  printf 'branch: %s\n' "${branch:-<detached>}"
  printf 'origin: %s\n' "$remote"
  printf 'state: %s\n' "$clean_flag"
  git -C "$root" status --short --branch
  printf '\n'
}

ensure_pushable_repo() {
  local name="$1"
  local root branch
  root="$(repo_root "$name")"
  branch="$(git -C "$root" branch --show-current)"

  [[ -n "$branch" ]] || fail "$name is on a detached HEAD; check out a branch before pushing"
  [[ -z "$(git -C "$root" status --porcelain)" ]] || fail "$name has uncommitted changes; commit or stash before pushing"
}

push_repo() {
  local name="$1"
  local dry_run="$2"
  local root branch
  root="$(repo_root "$name")"
  branch="$(git -C "$root" branch --show-current)"

  ensure_pushable_repo "$name"

  local -a cmd=(git -C "$root")
  if git -C "$root" rev-parse --abbrev-ref --symbolic-full-name '@{u}' >/dev/null 2>&1; then
    cmd+=(push)
  else
    cmd+=(push -u origin "$branch")
  fi

  printf 'workspace-git: %s -> pushing branch %s\n' "$name" "$branch"
  if [[ "$dry_run" == '1' ]]; then
    printf 'workspace-git: dry-run '
    printf '%q ' "${cmd[@]}"
    printf '\n'
    return 0
  fi

  "${cmd[@]}"
}

main() {
  local action="${1:-status}"
  shift || true

  case "$action" in
    status)
      [[ "$#" -eq 0 ]] || fail 'status does not accept extra arguments'
      print_repo_status mesh-lang
      print_repo_status hyperpush-mono
      ;;
    push)
      local target="${1:-}"
      [[ -n "$target" ]] || fail 'push requires <mesh-lang|hyperpush-mono|both>'
      shift

      local dry_run='0'
      while [[ "$#" -gt 0 ]]; do
        case "$1" in
          --dry-run) dry_run='1' ;;
          -h|--help) usage; exit 0 ;;
          *) fail "unknown push option '$1'" ;;
        esac
        shift
      done

      case "$target" in
        mesh-lang|hyperpush-mono)
          push_repo "$target" "$dry_run"
          ;;
        both)
          push_repo mesh-lang "$dry_run"
          push_repo hyperpush-mono "$dry_run"
          ;;
        *)
          fail "unknown push target '$target' (expected mesh-lang, hyperpush-mono, or both)"
          ;;
      esac
      ;;
    -h|--help|help)
      usage
      ;;
    *)
      fail "unknown action '$action'"
      ;;
  esac
}

main "$@"
