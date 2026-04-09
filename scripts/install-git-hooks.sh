#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

if ! git rev-parse --show-toplevel >/dev/null 2>&1; then
  echo "install-git-hooks: must run inside a git worktree" >&2
  exit 1
fi

chmod +x .githooks/pre-commit .githooks/pre-push scripts/verify-whitespace.sh

git config core.hooksPath .githooks

echo "install-git-hooks: configured core.hooksPath=.githooks"
echo "install-git-hooks: pre-commit now runs scripts/verify-whitespace.sh --staged --fix"
echo "install-git-hooks: pre-push split guard is active and will no-op when the sibling product repo is absent"
