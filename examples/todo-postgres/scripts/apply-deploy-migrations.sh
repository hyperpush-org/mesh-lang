#!/usr/bin/env bash
set -euo pipefail

SQL_PATH="${1:-}"

usage() {
  echo "usage: bash apply-deploy-migrations.sh <deploy-sql-path>" >&2
}

fail() {
  echo "[deploy-apply] $1" >&2
  exit 1
}

if [[ $# -ne 1 || -z "$SQL_PATH" ]]; then
  usage
  exit 1
fi

if ! command -v psql >/dev/null 2>&1; then
  fail "psql is required but was not found on PATH"
fi

if [[ ! -f "$SQL_PATH" ]]; then
  fail "missing deploy SQL artifact: $SQL_PATH"
fi

if [[ -z "${DATABASE_URL:-}" ]]; then
  fail "DATABASE_URL must be set"
fi

printf '[deploy-apply] sql artifact=%s\n' "$SQL_PATH"
printf '[deploy-apply] applying starter schema via psql\n'
psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -f "$SQL_PATH" >/dev/null

todos_table_present="$(psql "$DATABASE_URL" -Atqc "SELECT to_regclass('public.todos') IS NOT NULL")"
if [[ "$todos_table_present" != "t" ]]; then
  fail "todos table missing after apply"
fi

todos_index_present="$(psql "$DATABASE_URL" -Atqc "SELECT to_regclass('public.idx_todos_created_at') IS NOT NULL")"
if [[ "$todos_index_present" != "t" ]]; then
  fail "idx_todos_created_at missing after apply"
fi

printf '[deploy-apply] schema ready table=todos index=idx_todos_created_at\n'
