#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

printf '[m036-s01] phase=compiler-truth command=%s\n' 'cargo test -p mesh-lexer string_interpolation -- --nocapture'
cargo test -p mesh-lexer string_interpolation --manifest-path "$ROOT_DIR/Cargo.toml" -- --nocapture

printf '[m036-s01] phase=shared-surface-parity command=%s\n' 'node --test website/scripts/tests/verify-m036-s01-syntax-parity.test.mjs'
node --test "$ROOT_DIR/website/scripts/tests/verify-m036-s01-syntax-parity.test.mjs"
