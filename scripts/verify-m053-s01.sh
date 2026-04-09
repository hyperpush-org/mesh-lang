#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

absolutize_env_path() {
  local name="$1"
  local value="${!name:-}"
  if [[ -z "$value" ]]; then
    return 0
  fi
  case "$value" in
    /*) ;;
    *)
      printf -v "$name" '%s/%s' "$ROOT_DIR" "$value"
      export "$name"
      ;;
  esac
}

absolutize_env_path CARGO_HOME
absolutize_env_path CARGO_TARGET_DIR

ARTIFACT_ROOT=".tmp/m053-s01"
ARTIFACT_DIR="$ARTIFACT_ROOT/verify"
PROOF_BUNDLES_DIR="$ARTIFACT_ROOT/proof-bundles"
PHASE_REPORT_PATH="$ARTIFACT_DIR/phase-report.txt"
STATUS_PATH="$ARTIFACT_DIR/status.txt"
CURRENT_PHASE_PATH="$ARTIFACT_DIR/current-phase.txt"
LATEST_PROOF_BUNDLE_PATH="$ARTIFACT_DIR/latest-proof-bundle.txt"
RETAINED_ARTIFACTS_MANIFEST_PATH="$ARTIFACT_DIR/retained-m053-s01-artifacts.manifest.txt"
RETAINED_PROOF_BUNDLE_DIR=""
PREVIOUS_POINTER=""

if [[ -f "$LATEST_PROOF_BUNDLE_PATH" ]]; then
  PREVIOUS_POINTER="$(<"$LATEST_PROOF_BUNDLE_PATH")"
fi

rm -rf "$ARTIFACT_DIR"
mkdir -p "$ARTIFACT_DIR" "$PROOF_BUNDLES_DIR"
exec > >(tee "$ARTIFACT_DIR/full-contract.log") 2>&1

: >"$PHASE_REPORT_PATH"
printf 'running\n' >"$STATUS_PATH"
printf 'init\n' >"$CURRENT_PHASE_PATH"
if [[ -n "$PREVIOUS_POINTER" ]]; then
  printf '%s\n' "$PREVIOUS_POINTER" >"$LATEST_PROOF_BUNDLE_PATH"
fi

on_exit() {
  local exit_code=$?
  if [[ $exit_code -eq 0 ]]; then
    printf 'ok\n' >"$STATUS_PATH"
    printf 'complete\n' >"$CURRENT_PHASE_PATH"
  elif [[ ! -f "$STATUS_PATH" || "$(<"$STATUS_PATH")" != "failed" ]]; then
    printf 'failed\n' >"$STATUS_PATH"
  fi
}
trap on_exit EXIT

record_phase() {
  printf '%s\t%s\n' "$1" "$2" >>"$PHASE_REPORT_PATH"
}

begin_phase() {
  record_phase "$1" started
  printf '%s\n' "$1" >"$CURRENT_PHASE_PATH"
}

repo_rel() {
  local candidate="$1"
  if [[ "$candidate" == "$ROOT_DIR/"* ]]; then
    printf '%s\n' "${candidate#$ROOT_DIR/}"
  else
    printf '%s\n' "$candidate"
  fi
}

print_log_excerpt() {
  local log_path="$1"
  python3 - "$log_path" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
if not path.exists():
    print(f"missing log: {path}")
    raise SystemExit(0)
lines = path.read_text(errors="replace").splitlines()
head_count = 160
tail_count = 80
if len(lines) <= head_count + tail_count:
    for line in lines:
        print(line)
else:
    for line in lines[:head_count]:
        print(line)
    skipped = len(lines) - head_count - tail_count
    print(f"... skipped {skipped} lines ...")
    for line in lines[-tail_count:]:
        print(line)
PY
}

failure_reason_for_exit() {
  local exit_code="$1"
  local timeout_secs="$2"
  if [[ "$exit_code" -eq 124 ]]; then
    printf 'command timed out after %ss' "$timeout_secs"
  else
    printf 'command exited with status %s before %ss deadline' "$exit_code" "$timeout_secs"
  fi
}

fail_phase() {
  local phase="$1"
  local reason="$2"
  local log_path="${3:-}"
  local artifact_hint="${4:-}"

  printf 'failed\n' >"$STATUS_PATH"
  printf '%s\n' "$phase" >"$CURRENT_PHASE_PATH"
  echo "verification drift: ${reason}" >&2
  if [[ -n "$artifact_hint" ]]; then
    echo "artifact hint: $(repo_rel "$artifact_hint")" >&2
  fi
  if [[ -n "$log_path" ]]; then
    echo "failing log: $(repo_rel "$log_path")" >&2
    echo "--- $(repo_rel "$log_path") ---" >&2
    print_log_excerpt "$log_path" >&2
  fi
  exit 1
}

assert_file_contains_regex() {
  local phase="$1"
  local path="$2"
  local regex="$3"
  local description="$4"
  local log_path="${5:-}"
  if ! python3 - "$path" "$regex" "$description" >"$ARTIFACT_DIR/${phase}.content-check.log" 2>&1 <<'PY'
from pathlib import Path
import re
import sys

path = Path(sys.argv[1])
regex = sys.argv[2]
description = sys.argv[3]
text = path.read_text(errors="replace")
if not re.search(regex, text, re.MULTILINE):
    raise SystemExit(f"{description}: missing regex {regex!r} in {path}")
print(f"{description}: matched {regex!r}")
PY
  then
    fail_phase "$phase" "$description" "$ARTIFACT_DIR/${phase}.content-check.log" "${log_path:-$path}"
  fi
}

run_command() {
  local timeout_secs="$1"
  local log_path="$2"
  shift 2
  local -a cmd=("$@")
  {
    printf '$'
    printf ' %q' "${cmd[@]}"
    printf '\n'
    "${cmd[@]}"
  } >"$log_path" 2>&1 &
  local cmd_pid=$!
  local deadline=$((SECONDS + timeout_secs))
  while kill -0 "$cmd_pid" 2>/dev/null; do
    if (( SECONDS >= deadline )); then
      echo "command timed out after ${timeout_secs}s" >>"$log_path"
      kill -TERM "$cmd_pid" 2>/dev/null || true
      sleep 1
      kill -KILL "$cmd_pid" 2>/dev/null || true
      wait "$cmd_pid" 2>/dev/null || true
      return 124
    fi
    sleep 1
  done
  wait "$cmd_pid"
}

run_command_with_database_url() {
  local timeout_secs="$1"
  local log_path="$2"
  shift 2
  local -a cmd=("$@")
  {
    printf '$ DATABASE_URL=<redacted>'
    printf ' %q' "${cmd[@]}"
    printf '\n'
    env DATABASE_URL="$DATABASE_URL" "${cmd[@]}"
  } >"$log_path" 2>&1 &
  local cmd_pid=$!
  local deadline=$((SECONDS + timeout_secs))
  while kill -0 "$cmd_pid" 2>/dev/null; do
    if (( SECONDS >= deadline )); then
      echo "command timed out after ${timeout_secs}s" >>"$log_path"
      kill -TERM "$cmd_pid" 2>/dev/null || true
      sleep 1
      kill -KILL "$cmd_pid" 2>/dev/null || true
      wait "$cmd_pid" 2>/dev/null || true
      return 124
    fi
    sleep 1
  done
  wait "$cmd_pid"
}

assert_test_filter_ran() {
  local phase="$1"
  local log_path="$2"
  local label="$3"
  local count_log="$ARTIFACT_DIR/${label}.test-count.log"

  if ! python3 - "$log_path" "$label" >"$count_log" 2>&1 <<'PY'
import re
import sys
from pathlib import Path

text = Path(sys.argv[1]).read_text(errors="replace")
label = sys.argv[2]
counts = [int(value) for value in re.findall(r"running (\d+) test", text)]
if not counts:
    raise SystemExit(f"{label}: missing 'running N test' line")
if max(counts) <= 0:
    raise SystemExit(f"{label}: test filter ran 0 tests")
print(f"{label}: running-counts={counts}")
PY
  then
    record_phase "$phase" failed
    fail_phase "$phase" "named test filter ran 0 tests or produced malformed output" "$count_log"
  fi
}

run_expect_success() {
  local phase="$1"
  local label="$2"
  local require_tests="$3"
  local timeout_secs="$4"
  local artifact_hint="$5"
  shift 5
  local -a cmd=("$@")
  local log_path="$ARTIFACT_DIR/${label}.log"

  begin_phase "$phase"
  echo "==> ${cmd[*]}"
  if run_command "$timeout_secs" "$log_path" "${cmd[@]}"; then
    :
  else
    local exit_code=$?
    record_phase "$phase" failed
    fail_phase "$phase" "$(failure_reason_for_exit "$exit_code" "$timeout_secs")" "$log_path" "$artifact_hint"
  fi
  if [[ "$require_tests" == "yes" ]]; then
    assert_test_filter_ran "$phase" "$log_path" "$label"
  fi
  record_phase "$phase" passed
}

run_expect_success_with_database_url() {
  local phase="$1"
  local label="$2"
  local require_tests="$3"
  local timeout_secs="$4"
  local artifact_hint="$5"
  shift 5
  local -a cmd=("$@")
  local log_path="$ARTIFACT_DIR/${label}.log"

  begin_phase "$phase"
  echo "==> DATABASE_URL=<redacted> ${cmd[*]}"
  if run_command_with_database_url "$timeout_secs" "$log_path" "${cmd[@]}"; then
    :
  else
    local exit_code=$?
    record_phase "$phase" failed
    fail_phase "$phase" "$(failure_reason_for_exit "$exit_code" "$timeout_secs")" "$log_path" "$artifact_hint"
  fi
  if [[ "$require_tests" == "yes" ]]; then
    assert_test_filter_ran "$phase" "$log_path" "$label"
  fi
  record_phase "$phase" passed
}

capture_snapshot() {
  local source_root="$1"
  local snapshot_path="$2"
  shift 2
  python3 - "$source_root" "$snapshot_path" "$@" <<'PY'
from pathlib import Path
import sys

source_root = Path(sys.argv[1])
snapshot_path = Path(sys.argv[2])
ignored = set(sys.argv[3:])
names = []
if source_root.exists():
    names = sorted(
        path.name
        for path in source_root.iterdir()
        if path.is_dir() and path.name not in ignored
    )
snapshot_path.write_text(''.join(f"{name}\n" for name in names))
PY
}

copy_new_prefixed_artifacts_or_fail() {
  local phase="$1"
  local before_snapshot="$2"
  local source_root="$3"
  local dest_root="$4"
  local manifest_path="$5"
  shift 5
  if ! python3 - "$before_snapshot" "$source_root" "$dest_root" "$manifest_path" "$@" >"$ARTIFACT_DIR/${phase}.artifact-copy.log" 2>"$ARTIFACT_DIR/${phase}.artifact-copy.err" <<'PY'
from pathlib import Path
import shutil
import sys

before_snapshot = Path(sys.argv[1])
source_root = Path(sys.argv[2])
dest_root = Path(sys.argv[3])
manifest_path = Path(sys.argv[4])
prefixes = sys.argv[5:]

before = {
    line.strip()
    for line in before_snapshot.read_text(errors='replace').splitlines()
    if line.strip()
}
after_paths = {
    path.name: path
    for path in source_root.iterdir()
    if path.is_dir() and path.name != 'verify' and path.name != 'proof-bundles'
}
new_paths = {
    name: path
    for name, path in after_paths.items()
    if name not in before
}
if not new_paths:
    raise SystemExit('expected fresh .tmp/m053-s01 artifact directories from the staged deploy e2e replay')

if dest_root.exists():
    shutil.rmtree(dest_root)
dest_root.mkdir(parents=True, exist_ok=True)
manifest_lines = []
for prefix in prefixes:
    matches = sorted(name for name in new_paths if name.startswith(prefix))
    if len(matches) != 1:
        raise SystemExit(
            f'expected exactly one fresh artifact directory for prefix {prefix!r}, found {matches}'
        )
    name = matches[0]
    src = new_paths[name]
    if not any(src.iterdir()):
        raise SystemExit(f'{src}: expected non-empty artifact directory')
    dst = dest_root / name
    shutil.copytree(src, dst)
    manifest_lines.append(f'{name}\t{src}')
    for child in sorted(src.rglob('*')):
        rel = child.relative_to(src)
        manifest_lines.append(f'  - {name}/{rel}')

manifest_path.write_text('\n'.join(manifest_lines) + ('\n' if manifest_lines else ''))
print('\n'.join(manifest_lines))
PY
  then
    fail_phase "$phase" "missing or malformed copied evidence" "$ARTIFACT_DIR/${phase}.artifact-copy.err" "$source_root"
  fi
}

copy_staged_bundle_or_fail() {
  local phase="$1"
  local source_artifacts_root="$2"
  local dest_root="$3"
  local manifest_path="$4"
  if ! python3 - "$ROOT_DIR" "$source_artifacts_root" "$dest_root" "$manifest_path" >"$ARTIFACT_DIR/${phase}.bundle-copy.log" 2>"$ARTIFACT_DIR/${phase}.bundle-copy.err" <<'PY'
from pathlib import Path
import json
import shutil
import sys

repo_root = Path(sys.argv[1]).resolve()
source_root = Path(sys.argv[2])
dest_root = Path(sys.argv[3])
manifest_path = Path(sys.argv[4])
truth_dirs = sorted(path for path in source_root.iterdir() if path.is_dir() and path.name.startswith('todo-postgres-staged-deploy-truth-'))
if len(truth_dirs) != 1:
    raise SystemExit(f'expected exactly one copied truth artifact directory, found {[path.name for path in truth_dirs]}')
truth_dir = truth_dirs[0]
pointer_path = truth_dir / 'staged-bundle.path.txt'
if not pointer_path.is_file():
    raise SystemExit(f'{truth_dir}: missing staged-bundle.path.txt')
pointed = pointer_path.read_text(errors='replace').strip()
if not pointed:
    raise SystemExit(f'{pointer_path}: empty staged-bundle.path.txt')
staged_source = Path(pointed)
if not staged_source.is_absolute():
    raise SystemExit(f'{pointer_path}: expected absolute staged bundle path, got {pointed!r}')
staged_source = staged_source.resolve()
if not staged_source.exists() or not staged_source.is_dir():
    raise SystemExit(f'{pointer_path}: missing staged bundle directory {staged_source}')
if repo_root in staged_source.parents or staged_source == repo_root:
    raise SystemExit(f'{pointer_path}: staged bundle drifted under repo root: {staged_source}')
if dest_root.exists():
    shutil.rmtree(dest_root)
shutil.copytree(staged_source, dest_root)
required = ['todo-postgres', 'todo-postgres.up.sql', 'apply-deploy-migrations.sh', 'deploy-smoke.sh']
missing = [name for name in required if not (dest_root / name).exists()]
if missing:
    raise SystemExit(f'{dest_root}: copied staged bundle missing required files {missing}')
manifest = {
    'source_pointer_file': str(pointer_path),
    'source_bundle_dir': str(staged_source),
    'copied_bundle_dir': str(dest_root),
    'entries': [],
}
for child in sorted(dest_root.rglob('*')):
    manifest['entries'].append({
        'relative_path': str(child.relative_to(dest_root)),
        'kind': 'dir' if child.is_dir() else 'file',
        'size_bytes': child.stat().st_size if child.is_file() else 0,
    })
manifest_path.write_text(json.dumps(manifest, indent=2) + '\n')
print(dest_root)
PY
  then
    fail_phase "$phase" "missing retained bundle path or malformed staged bundle pointer" "$ARTIFACT_DIR/${phase}.bundle-copy.err" "$source_artifacts_root"
  fi
}

assert_retained_bundle_shape() {
  local phase="$1"
  local bundle_root="$2"
  local artifacts_root="$3"
  local artifacts_manifest="$4"
  local bundle_manifest="$5"
  local log_path="$ARTIFACT_DIR/${phase}.bundle-shape.log"
  if ! python3 - "$ROOT_DIR" "$bundle_root" "$artifacts_root" "$artifacts_manifest" "$bundle_manifest" "$LATEST_PROOF_BUNDLE_PATH" "$ARTIFACT_DIR" >"$log_path" 2>&1 <<'PY'
from pathlib import Path
import json
import sys

repo_root = Path(sys.argv[1]).resolve()
bundle_root = Path(sys.argv[2])
artifacts_root = Path(sys.argv[3])
artifacts_manifest = Path(sys.argv[4])
bundle_manifest = Path(sys.argv[5])
pointer_path = Path(sys.argv[6])
verify_dir = Path(sys.argv[7]).resolve()

if not bundle_root.is_dir():
    raise SystemExit(f'{bundle_root}: retained proof bundle directory missing')
expected_pointer = str(bundle_root)
actual_pointer = pointer_path.read_text(errors='replace').strip()
if actual_pointer != expected_pointer:
    raise SystemExit(
        f'latest-proof-bundle pointer drifted: expected {expected_pointer!r}, got {actual_pointer!r}'
    )
if not actual_pointer:
    raise SystemExit('latest-proof-bundle pointer was empty')
if not artifacts_root.is_dir():
    raise SystemExit(f'{artifacts_root}: missing retained artifact copy root')
if not artifacts_manifest.is_file():
    raise SystemExit(f'{artifacts_manifest}: missing retained artifact manifest')
if not bundle_manifest.is_file():
    raise SystemExit(f'{bundle_manifest}: missing retained staged bundle manifest')
required_top = [
    'verify-m053-s01.sh',
    'todo-postgres.README.md',
    'retained-m053-s01-artifacts',
    'retained-staged-bundle',
    'retained-m053-s01-artifacts.manifest.txt',
    'retained-staged-bundle.manifest.json',
]
for name in required_top:
    if not (bundle_root / name).exists():
        raise SystemExit(f'{bundle_root}: missing retained proof bundle entry {name}')

children = sorted(path for path in artifacts_root.iterdir() if path.is_dir())
if len(children) != 3:
    raise SystemExit(f'{artifacts_root}: expected exactly three copied artifact directories, found {[path.name for path in children]}')

def find_one(prefix: str) -> Path:
    matches = [path for path in children if path.name.startswith(prefix)]
    if len(matches) != 1:
        raise SystemExit(f'{artifacts_root}: expected exactly one directory with prefix {prefix!r}, found {[path.name for path in matches]}')
    return matches[0]

truth = find_one('todo-postgres-staged-deploy-truth-')
invalid_path = find_one('todo-postgres-invalid-bundle-path-')
fail_closed = find_one('todo-postgres-staged-fail-closed-')

required_truth = [
    'stage-deploy.stdout.log',
    'stage-deploy.stderr.log',
    'stage-deploy.meta.txt',
    'staged-bundle.path.txt',
    'staged-bundle.manifest.json',
    'database.json',
    'deploy-apply.stdout.log',
    'deploy-apply.stderr.log',
    'deploy-apply.meta.txt',
    'health.http',
    'health.json',
    'cluster-status.log',
    'cluster-status.json',
    'cluster-continuity-startup-list.log',
    'cluster-continuity-startup-list.json',
    'cluster-continuity-startup-record.log',
    'cluster-continuity-startup-record.json',
    'cluster-continuity-route-list.log',
    'cluster-continuity-route-list.json',
    'cluster-continuity-route-record.log',
    'cluster-continuity-route-record.json',
    'cluster-diagnostics.log',
    'cluster-diagnostics.json',
    'todos-empty.http',
    'todos-empty.json',
    'deploy-smoke.stdout.log',
    'deploy-smoke.stderr.log',
    'deploy-smoke.meta.txt',
    'todos-after-smoke.http',
    'todos-after-smoke.json',
    'cluster-continuity-after-smoke.log',
    'cluster-continuity-after-smoke.json',
    'runtime.stdout.log',
    'runtime.stderr.log',
    'runtime.combined.log',
]
for name in required_truth:
    if not (truth / name).exists():
        raise SystemExit(f'{truth}: missing required retained file {name}')
for required_dir in ['generated-project', 'workspace']:
    if not (truth / required_dir).is_dir():
        raise SystemExit(f'{truth}: missing required retained directory {required_dir}')
for sample in [
    invalid_path / 'stage-deploy-invalid-path.stdout.log',
    invalid_path / 'stage-deploy-invalid-path.stderr.log',
    fail_closed / 'deploy-apply-missing-database-url.stdout.log',
    fail_closed / 'deploy-smoke-malformed-base-url.stdout.log',
    fail_closed / 'cluster-status-not-ready.log',
]:
    if not sample.exists():
        raise SystemExit(f'missing expected fail-closed artifact {sample}')

staged_bundle_copy = bundle_root / 'retained-staged-bundle'
required_bundle = ['todo-postgres', 'todo-postgres.up.sql', 'apply-deploy-migrations.sh', 'deploy-smoke.sh']
for name in required_bundle:
    if not (staged_bundle_copy / name).exists():
        raise SystemExit(f'{staged_bundle_copy}: missing copied staged bundle entry {name}')
if repo_root in staged_bundle_copy.resolve().parents or staged_bundle_copy.resolve() == repo_root:
    pass
if not bundle_manifest.read_text(errors='replace').strip():
    raise SystemExit(f'{bundle_manifest}: expected non-empty retained staged bundle manifest')
if not artifacts_manifest.read_text(errors='replace').strip():
    raise SystemExit(f'{artifacts_manifest}: expected non-empty retained artifact manifest')

print('retained-bundle-shape: ok')
PY
  then
    fail_phase "$phase" "missing retained proof artifacts or malformed bundle pointer" "$log_path" "$bundle_root"
  fi
}

assert_no_secret_leaks() {
  local phase="$1"
  local search_root="$2"
  local log_path="$ARTIFACT_DIR/${phase}.redaction-check.log"
  if ! python3 - "$search_root" "$DATABASE_URL" >"$log_path" 2>&1 <<'PY'
from pathlib import Path
import sys

root = Path(sys.argv[1])
secret = sys.argv[2]
if not root.exists():
    raise SystemExit(f'{root}: missing root for secret scan')
for path in sorted(root.rglob('*')):
    if not path.is_file():
        continue
    try:
        text = path.read_text(errors='replace')
    except Exception:
        continue
    if secret and secret in text:
        raise SystemExit(f'secret leak detected in {path}')
print('redaction: ok')
PY
  then
    fail_phase "$phase" "retained logs leaked DATABASE_URL" "$log_path" "$search_root"
  fi
}

begin_phase m053-s01-db-env-preflight
if [[ -z "${DATABASE_URL:-}" ]]; then
  printf 'DATABASE_URL must be set for scripts/verify-m053-s01.sh\n' >"$ARTIFACT_DIR/m053-s01-db-env-preflight.log"
  record_phase m053-s01-db-env-preflight failed
  fail_phase m053-s01-db-env-preflight "DATABASE_URL must be set for scripts/verify-m053-s01.sh" "$ARTIFACT_DIR/m053-s01-db-env-preflight.log"
fi
if [[ "$DATABASE_URL" != postgres://* && "$DATABASE_URL" != postgresql://* ]]; then
  printf 'DATABASE_URL must start with postgres:// or postgresql://\n' >"$ARTIFACT_DIR/m053-s01-db-env-preflight.log"
  record_phase m053-s01-db-env-preflight failed
  fail_phase m053-s01-db-env-preflight "DATABASE_URL must start with postgres:// or postgresql://" "$ARTIFACT_DIR/m053-s01-db-env-preflight.log"
fi
record_phase m053-s01-db-env-preflight passed

M053_BEFORE="$ARTIFACT_DIR/m053-s01-before.snapshot"
capture_snapshot "$ROOT_DIR/.tmp/m053-s01" "$M053_BEFORE" verify proof-bundles

run_expect_success m053-s01-scaffold-rail m053-s01-scaffold-rail yes 1800 compiler/mesh-pkg/src/scaffold.rs \
  cargo test -p mesh-pkg m049_s01_postgres_scaffold_ -- --nocapture
run_expect_success m053-s01-mesh-rt-staticlib m053-s01-mesh-rt-staticlib no 1800 compiler/mesh-rt/src/lib.rs \
  cargo build -q -p mesh-rt
run_expect_success m053-s01-example-e2e m053-s01-example-e2e yes 1800 compiler/meshc/tests/e2e_m049_s03.rs \
  cargo test -p meshc --test e2e_m049_s03 -- --nocapture
run_expect_success m053-s01-example-parity m053-s01-example-parity no 900 scripts/tests/verify-m049-s03-materialize-examples.mjs \
  node scripts/tests/verify-m049-s03-materialize-examples.mjs --check
run_expect_success_with_database_url m053-s01-staged-deploy-e2e m053-s01-staged-deploy-e2e yes 2400 compiler/meshc/tests/e2e_m053_s01.rs \
  cargo test -p meshc --test e2e_m053_s01 -- --nocapture

RETAINED_PROOF_BUNDLE_DIR="$PROOF_BUNDLES_DIR/retained-starter-deploy-$(python3 - <<'PY'
import time
print(time.time_ns())
PY
)"
mkdir -p "$RETAINED_PROOF_BUNDLE_DIR"

record_phase m053-s01-retain-artifacts started
copy_new_prefixed_artifacts_or_fail \
  m053-s01-retain-artifacts \
  "$M053_BEFORE" \
  "$ROOT_DIR/.tmp/m053-s01" \
  "$RETAINED_PROOF_BUNDLE_DIR/retained-m053-s01-artifacts" \
  "$RETAINED_ARTIFACTS_MANIFEST_PATH" \
  todo-postgres-staged-deploy-truth- \
  todo-postgres-invalid-bundle-path- \
  todo-postgres-staged-fail-closed-
record_phase m053-s01-retain-artifacts passed

record_phase m053-s01-retain-staged-bundle started
copy_staged_bundle_or_fail \
  m053-s01-retain-staged-bundle \
  "$RETAINED_PROOF_BUNDLE_DIR/retained-m053-s01-artifacts" \
  "$RETAINED_PROOF_BUNDLE_DIR/retained-staged-bundle" \
  "$RETAINED_PROOF_BUNDLE_DIR/retained-staged-bundle.manifest.json"
record_phase m053-s01-retain-staged-bundle passed

cp "$ROOT_DIR/scripts/verify-m053-s01.sh" "$RETAINED_PROOF_BUNDLE_DIR/verify-m053-s01.sh"
cp "$ROOT_DIR/examples/todo-postgres/README.md" "$RETAINED_PROOF_BUNDLE_DIR/todo-postgres.README.md"
cp "$RETAINED_ARTIFACTS_MANIFEST_PATH" "$RETAINED_PROOF_BUNDLE_DIR/retained-m053-s01-artifacts.manifest.txt"
printf '%s\n' "$RETAINED_PROOF_BUNDLE_DIR" >"$LATEST_PROOF_BUNDLE_PATH"

record_phase m053-s01-redaction-drift started
assert_no_secret_leaks m053-s01-redaction-drift "$RETAINED_PROOF_BUNDLE_DIR"
record_phase m053-s01-redaction-drift passed

record_phase m053-s01-bundle-shape started
assert_retained_bundle_shape \
  m053-s01-bundle-shape \
  "$RETAINED_PROOF_BUNDLE_DIR" \
  "$RETAINED_PROOF_BUNDLE_DIR/retained-m053-s01-artifacts" \
  "$RETAINED_PROOF_BUNDLE_DIR/retained-m053-s01-artifacts.manifest.txt" \
  "$RETAINED_PROOF_BUNDLE_DIR/retained-staged-bundle.manifest.json"
record_phase m053-s01-bundle-shape passed

assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^m053-s01-db-env-preflight\tpassed$' "DATABASE_URL preflight did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^m053-s01-scaffold-rail\tpassed$' "Postgres scaffold rail did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^m053-s01-mesh-rt-staticlib\tpassed$' "mesh-rt prebuild rail did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^m053-s01-example-e2e\tpassed$' "Postgres example e2e rail did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^m053-s01-example-parity\tpassed$' "Postgres example parity rail did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^m053-s01-staged-deploy-e2e\tpassed$' "Staged deploy e2e rail did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^m053-s01-retain-artifacts\tpassed$' "Retained artifact copy did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^m053-s01-retain-staged-bundle\tpassed$' "Retained staged bundle copy did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^m053-s01-redaction-drift\tpassed$' "Redaction drift check did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^m053-s01-bundle-shape\tpassed$' "Retained bundle shape check did not pass" "$ARTIFACT_DIR/full-contract.log"

echo "verify-m053-s01: ok"
