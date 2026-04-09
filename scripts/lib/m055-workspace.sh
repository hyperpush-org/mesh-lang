#!/usr/bin/env bash
# shellcheck shell=bash

m055_repo_identity_path() {
  local root_dir="$1"
  printf '%s\n' "$root_dir/scripts/lib/repo-identity.json"
}

m055_repo_identity_field() {
  local root_dir="$1"
  local field_path="$2"
  local identity_path="${3:-$(m055_repo_identity_path "$root_dir")}"

  if [[ ! -f "$identity_path" ]]; then
    printf 'verification drift: missing repo identity: %s\n' "$identity_path" >&2
    return 1
  fi

  python3 - "$identity_path" "$field_path" <<'PY'
import json
import sys
from pathlib import Path

identity_path = Path(sys.argv[1])
field_path = sys.argv[2]

try:
    value = json.loads(identity_path.read_text())
except Exception as exc:  # pragma: no cover - surfaced through caller
    raise SystemExit(f'verification drift: {identity_path} is not valid JSON: {exc}')

for part in field_path.split('.'):
    if not isinstance(value, dict) or part not in value:
        raise SystemExit(f'verification drift: {identity_path} missing {field_path}')
    value = value[part]

if not isinstance(value, str) or value == '':
    raise SystemExit(f'verification drift: {identity_path} field {field_path} must be a non-empty string')

print(value)
PY
}

m055_resolve_hyperpush_root() {
  local root_dir="$1"
  local override="${M055_HYPERPUSH_ROOT:-}"
  local identity_path
  identity_path="$(m055_repo_identity_path "$root_dir")"

  local product_dir
  product_dir="$(m055_repo_identity_field "$root_dir" 'productRepo.workspaceDir' "$identity_path")" || return 1

  local language_dir
  language_dir="$(m055_repo_identity_field "$root_dir" 'languageRepo.workspaceDir' "$identity_path")" || return 1

  local candidate source
  if [[ -n "$override" ]]; then
    candidate="$override"
    source='env:M055_HYPERPUSH_ROOT'
  else
    candidate="$root_dir/../$product_dir"
    source="blessed-sibling:${language_dir}->${product_dir}"
  fi

  local resolved_candidate
  resolved_candidate="$(python3 - "$candidate" <<'PY'
import os
import sys

print(os.path.abspath(sys.argv[1]))
PY
)"

  if [[ ! -d "$resolved_candidate" ]]; then
    if [[ -z "$override" && -d "$root_dir/mesher" ]]; then
      printf 'verification drift: missing sibling product repo root %s (source=%s); stale in-repo mesher path %s is not authoritative\n' \
        "$resolved_candidate" "$source" "$root_dir/mesher" >&2
    else
      printf 'verification drift: missing sibling product repo root %s (source=%s)\n' \
        "$resolved_candidate" "$source" >&2
    fi
    return 1
  fi

  if [[ ! -f "$resolved_candidate/mesher/scripts/verify-maintainer-surface.sh" ]]; then
    printf 'verification drift: malformed sibling product repo root %s (source=%s); missing mesher/scripts/verify-maintainer-surface.sh\n' \
      "$resolved_candidate" "$source" >&2
    return 1
  fi

  export M055_HYPERPUSH_ROOT_RESOLVED="$resolved_candidate"
  export M055_HYPERPUSH_ROOT_SOURCE="$source"
  printf '%s\n' "$resolved_candidate"
}

m055_resolve_language_repo_slug() {
  local root_dir="$1"
  local override="${M053_S03_GH_REPO:-}"

  if [[ -n "$override" ]]; then
    export M055_LANGUAGE_REPO_SLUG_RESOLVED="$override"
    export M055_LANGUAGE_REPO_SLUG_SOURCE='env:M053_S03_GH_REPO'
    printf '%s\n' "$override"
    return 0
  fi

  local slug
  slug="$(m055_repo_identity_field "$root_dir" 'languageRepo.slug')" || return 1
  export M055_LANGUAGE_REPO_SLUG_RESOLVED="$slug"
  export M055_LANGUAGE_REPO_SLUG_SOURCE='repo-identity:scripts/lib/repo-identity.json#languageRepo.slug'
  printf '%s\n' "$slug"
}
