#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
PACKAGE_NAME="todo-postgres"
DEPLOY_SQL_NAME="todo-postgres.up.sql"
BUNDLE_DIR="${1:-}"
SOURCE_SQL="$PROJECT_DIR/deploy/$DEPLOY_SQL_NAME"
SOURCE_APPLY_SCRIPT="$PROJECT_DIR/scripts/apply-deploy-migrations.sh"
SOURCE_SMOKE_SCRIPT="$PROJECT_DIR/scripts/deploy-smoke.sh"
ACCIDENTAL_BINARY="$PROJECT_DIR/$PACKAGE_NAME"
TARGET_BINARY=""
TARGET_SQL=""
TARGET_APPLY_SCRIPT=""
TARGET_SMOKE_SCRIPT=""

usage() {
  echo "usage: bash scripts/stage-deploy.sh <bundle-dir>" >&2
}

fail() {
  echo "[stage-deploy] $1" >&2
  exit 1
}

require_command() {
  local command_name="$1"
  if ! command -v "$command_name" >/dev/null 2>&1; then
    fail "required command missing from PATH: $command_name"
  fi
}

require_file() {
  local label="$1"
  local path="$2"
  if [[ ! -f "$path" ]]; then
    fail "missing required ${label}: $path"
  fi
}

ensure_source_tree_clean() {
  if [[ -e "$ACCIDENTAL_BINARY" ]]; then
    fail "source tree contains an in-place binary: $ACCIDENTAL_BINARY"
  fi
}

if [[ $# -ne 1 || -z "$BUNDLE_DIR" ]]; then
  usage
  exit 1
fi

if [[ -e "$BUNDLE_DIR" && ! -d "$BUNDLE_DIR" ]]; then
  fail "bundle path exists but is not a directory: $BUNDLE_DIR"
fi

require_command meshc
require_file "project manifest" "$PROJECT_DIR/mesh.toml"
require_file "deploy SQL artifact" "$SOURCE_SQL"
require_file "deploy migration script" "$SOURCE_APPLY_SCRIPT"
require_file "deploy smoke script" "$SOURCE_SMOKE_SCRIPT"
ensure_source_tree_clean

mkdir -p "$BUNDLE_DIR"
TARGET_BINARY="$BUNDLE_DIR/$PACKAGE_NAME"
TARGET_SQL="$BUNDLE_DIR/$DEPLOY_SQL_NAME"
TARGET_APPLY_SCRIPT="$BUNDLE_DIR/apply-deploy-migrations.sh"
TARGET_SMOKE_SCRIPT="$BUNDLE_DIR/deploy-smoke.sh"
rm -f "$TARGET_BINARY" "$TARGET_SQL" "$TARGET_APPLY_SCRIPT" "$TARGET_SMOKE_SCRIPT"

printf '[stage-deploy] building starter binary project=%s\n' "$PROJECT_DIR"
(
  cd "$PROJECT_DIR"
  meshc build . --output "$TARGET_BINARY"
)

ensure_source_tree_clean

if [[ ! -f "$TARGET_BINARY" ]]; then
  fail "meshc build reported success but the staged binary is missing: $TARGET_BINARY"
fi

if [[ ! -x "$TARGET_BINARY" ]]; then
  fail "staged binary is not executable: $TARGET_BINARY"
fi

printf '[stage-deploy] staging bundle dir=%s\n' "$BUNDLE_DIR"
cp "$SOURCE_SQL" "$TARGET_SQL"
cp "$SOURCE_APPLY_SCRIPT" "$TARGET_APPLY_SCRIPT"
cp "$SOURCE_SMOKE_SCRIPT" "$TARGET_SMOKE_SCRIPT"
chmod 755 "$TARGET_BINARY" "$TARGET_APPLY_SCRIPT" "$TARGET_SMOKE_SCRIPT"

printf '[stage-deploy] staged layout\n'
for staged_path in \
  "$TARGET_BINARY" \
  "$TARGET_SQL" \
  "$TARGET_APPLY_SCRIPT" \
  "$TARGET_SMOKE_SCRIPT"
do
  printf '[stage-deploy] - %s\n' "$staged_path"
done

printf '[stage-deploy] bundle ready dir=%s\n' "$BUNDLE_DIR"
