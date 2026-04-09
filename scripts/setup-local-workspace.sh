#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

# shellcheck source=scripts/lib/m055-workspace.sh
source "$ROOT_DIR/scripts/lib/m055-workspace.sh"

fail() {
  printf 'setup-local-workspace: %s\n' "$1" >&2
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

require_git_remote() {
  local repo_root="$1"
  local expected_url="$2"
  local label="$3"
  local actual_url

  if [[ ! -d "$repo_root/.git" ]]; then
    fail "$label is not a git repo: $repo_root"
  fi

  actual_url="$(git -C "$repo_root" remote get-url origin 2>/dev/null || true)"
  [[ -n "$actual_url" ]] || fail "$label is missing origin remote: $repo_root"

  local normalized_actual normalized_expected
  normalized_actual="$(normalize_git_url "$actual_url")"
  normalized_expected="$(normalize_git_url "$expected_url")"
  [[ "$normalized_actual" == "$normalized_expected" ]] || fail "$label origin remote drifted: expected $normalized_expected, found $normalized_actual"
}

ensure_git_info_exclude() {
  local exclude_path="$ROOT_DIR/.git/info/exclude"
  mkdir -p "$(dirname "$exclude_path")"
  touch "$exclude_path"
  if ! rg -Fxq '/mesher' "$exclude_path"; then
    printf '\n# local M055 compatibility path\n/mesher\n' >>"$exclude_path"
  fi
}

ensure_hooks_path() {
  local repo_root="$1"
  local label="$2"

  [[ -x "$repo_root/.githooks/pre-push" ]] || fail "$label is missing executable .githooks/pre-push"
  git -C "$repo_root" config core.hooksPath .githooks
}

language_workspace_dir="$(m055_repo_identity_field "$ROOT_DIR" 'languageRepo.workspaceDir')"
language_git_url="$(m055_repo_identity_field "$ROOT_DIR" 'languageRepo.gitUrl')"
product_workspace_dir="$(m055_repo_identity_field "$ROOT_DIR" 'productRepo.workspaceDir')"
product_git_url="$(m055_repo_identity_field "$ROOT_DIR" 'productRepo.gitUrl')"

[[ "$(basename "$ROOT_DIR")" == "$language_workspace_dir" ]] || fail "current repo root must be named $language_workspace_dir, got $(basename "$ROOT_DIR")"
require_git_remote "$ROOT_DIR" "$language_git_url" "$language_workspace_dir"

PRODUCT_ROOT="$(m055_resolve_hyperpush_root "$ROOT_DIR")" || exit 1
[[ "$(basename "$PRODUCT_ROOT")" == "$product_workspace_dir" ]] || fail "product repo root must be named $product_workspace_dir, got $(basename "$PRODUCT_ROOT")"
require_git_remote "$PRODUCT_ROOT" "$product_git_url" "$product_workspace_dir"

if [[ -e "$ROOT_DIR/mesher" && ! -L "$ROOT_DIR/mesher" ]]; then
  fail "mesh-lang/mesher exists as a real directory; remove the tracked tree before creating a local-only compatibility path"
fi

TARGET_RELATIVE="../$product_workspace_dir/mesher"
TARGET_ABSOLUTE="$PRODUCT_ROOT/mesher"
[[ -d "$TARGET_ABSOLUTE" ]] || fail "product repo is missing mesher package root: $TARGET_ABSOLUTE"
[[ -f "$TARGET_ABSOLUTE/scripts/verify-maintainer-surface.sh" ]] || fail "product repo is missing mesher/scripts/verify-maintainer-surface.sh"

if [[ -L "$ROOT_DIR/mesher" ]]; then
  current_target="$(readlink "$ROOT_DIR/mesher")"
  if [[ "$current_target" == "$TARGET_RELATIVE" ]]; then
    ensure_git_info_exclude
    ensure_hooks_path "$ROOT_DIR" "$language_workspace_dir"
    ensure_hooks_path "$PRODUCT_ROOT" "$product_workspace_dir"
    printf 'setup-local-workspace: ok (existing compatibility path kept)\n'
    printf 'mesh-lang root: %s\n' "$ROOT_DIR"
    printf 'product root: %s\n' "$PRODUCT_ROOT"
    printf 'compatibility path: mesher -> %s\n' "$TARGET_RELATIVE"
    printf 'hooksPath: mesh-lang=.githooks, %s=.githooks\n' "$product_workspace_dir"
    exit 0
  fi
  rm "$ROOT_DIR/mesher"
fi

ln -s "$TARGET_RELATIVE" "$ROOT_DIR/mesher"
ensure_git_info_exclude
ensure_hooks_path "$ROOT_DIR" "$language_workspace_dir"
ensure_hooks_path "$PRODUCT_ROOT" "$product_workspace_dir"

printf 'setup-local-workspace: ok\n'
printf 'mesh-lang root: %s\n' "$ROOT_DIR"
printf 'product root: %s\n' "$PRODUCT_ROOT"
printf 'compatibility path: mesher -> %s\n' "$TARGET_RELATIVE"
printf 'hooksPath: mesh-lang=.githooks, %s=.githooks\n' "$product_workspace_dir"
