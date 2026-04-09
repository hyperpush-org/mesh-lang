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

ARTIFACT_ROOT=".tmp/m054-s03"
ARTIFACT_DIR="$ARTIFACT_ROOT/verify"
PROOF_BUNDLES_DIR="$ARTIFACT_ROOT/proof-bundles"
PHASE_REPORT_PATH="$ARTIFACT_DIR/phase-report.txt"
STATUS_PATH="$ARTIFACT_DIR/status.txt"
CURRENT_PHASE_PATH="$ARTIFACT_DIR/current-phase.txt"
LATEST_PROOF_BUNDLE_PATH="$ARTIFACT_DIR/latest-proof-bundle.txt"
BUILT_HTML_SUMMARY_PATH="$ARTIFACT_DIR/built-html-summary.json"
RETAINED_PROOF_BUNDLE_DIR=""

rm -rf "$ARTIFACT_DIR"
mkdir -p "$ARTIFACT_DIR" "$PROOF_BUNDLES_DIR"
exec > >(tee "$ARTIFACT_DIR/full-contract.log") 2>&1

: >"$PHASE_REPORT_PATH"
printf 'running\n' >"$STATUS_PATH"
printf 'init\n' >"$CURRENT_PHASE_PATH"

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
  local artifact_hint="${4:-}"
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
    fail_phase "$phase" "named test filter ran 0 tests or produced malformed output" "$count_log" "$artifact_hint"
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
    assert_test_filter_ran "$phase" "$log_path" "$label" "$artifact_hint"
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
    assert_test_filter_ran "$phase" "$log_path" "$label" "$artifact_hint"
  fi
  record_phase "$phase" passed
}

copy_fixed_dir_or_fail() {
  local phase="$1"
  local source_dir="$2"
  local dest_dir="$3"
  local description="$4"
  shift 4
  local log_path="$ARTIFACT_DIR/${phase}.copy.log"

  if ! python3 - "$source_dir" "$dest_dir" "$description" "$@" >"$log_path" 2>&1 <<'PY'
from pathlib import Path
import shutil
import sys

source_dir = Path(sys.argv[1])
dest_dir = Path(sys.argv[2])
description = sys.argv[3]
required = sys.argv[4:]

if not source_dir.is_dir():
    raise SystemExit(f"{description}: missing source directory {source_dir}")
for rel in required:
    if not (source_dir / rel).exists():
        raise SystemExit(f"{description}: missing {rel} in {source_dir}")
if dest_dir.exists():
    shutil.rmtree(dest_dir)
dest_dir.parent.mkdir(parents=True, exist_ok=True)
shutil.copytree(source_dir, dest_dir, symlinks=True)
print(f"copied {source_dir} -> {dest_dir}")
PY
  then
    record_phase "$phase" failed
    fail_phase "$phase" "$description" "$log_path" "$source_dir"
  fi
}

copy_file_or_fail() {
  local phase="$1"
  local source_path="$2"
  local dest_path="$3"
  local description="$4"
  local log_path="$ARTIFACT_DIR/${phase}.$(basename "$dest_path").copy.log"

  if ! python3 - "$source_path" "$dest_path" "$description" >"$log_path" 2>&1 <<'PY'
from pathlib import Path
import shutil
import sys

source = Path(sys.argv[1])
dest = Path(sys.argv[2])
description = sys.argv[3]
if not source.is_file():
    raise SystemExit(f"{description}: missing source file {source}")
dest.parent.mkdir(parents=True, exist_ok=True)
shutil.copy2(source, dest)
print(f"copied {source} -> {dest}")
PY
  then
    record_phase "$phase" failed
    fail_phase "$phase" "$description" "$log_path" "$source_path"
  fi
}

assert_built_html_contract() {
  local phase="$1"
  local summary_path="$2"
  local log_path="$ARTIFACT_DIR/${phase}.log"
  if ! python3 - "$ROOT_DIR" "$summary_path" >"$log_path" 2>&1 <<'PY'
from pathlib import Path
import json
import sys

root = Path(sys.argv[1])
summary_path = Path(sys.argv[2])
index_path = root / 'website/docs/.vitepress/dist/index.html'
proof_path = root / 'website/docs/.vitepress/dist/docs/distributed-proof/index.html'
index_html = index_path.read_text(errors='replace')
proof_html = proof_path.read_text(errors='replace')
checks = {
    'index': {
        'new_description': 'One public app URL fronts multiple Mesh nodes. Runtime placement stays server-side, and operator truth stays on meshc cluster.' in index_html,
        'old_description_absent': 'Built-in failover, load balancing, and exactly-once semantics' not in index_html,
    },
    'proof': {
        'boundary': 'A proxy/platform ingress may expose one public app URL in front of multiple nodes, but that is where the public routing story ends.' in proof_html,
        'header_lookup': 'X-Mesh-Continuity-Request-Key' in proof_html and 'meshc cluster continuity &lt;node-name@host:port&gt; &lt;request-key&gt; --json' in proof_html,
        'list_first': 'If you are inspecting startup work or doing manual discovery without a request key yet' in proof_html,
        'non_goals': 'sticky sessions, frontend-aware routing, or client-visible topology claims' in proof_html,
    },
}
summary = {
    'index': {
        'path': str(index_path.relative_to(root)),
        'checks': checks['index'],
    },
    'proof': {
        'path': str(proof_path.relative_to(root)),
        'checks': checks['proof'],
    },
}
summary_path.parent.mkdir(parents=True, exist_ok=True)
summary_path.write_text(json.dumps(summary, indent=2) + '\n')
failed = [
    f'{surface}.{name}'
    for surface, surface_checks in checks.items()
    for name, ok in surface_checks.items()
    if not ok
]
for surface, surface_checks in checks.items():
    for name, ok in surface_checks.items():
        print(f'{surface}.{name}={ok}')
if failed:
    raise SystemExit('failed checks: ' + ', '.join(failed))
PY
  then
    fail_phase "$phase" "built HTML drifted from the bounded public contract" "$log_path" "website/docs/.vitepress/dist"
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

assert_retained_bundle_shape() {
  local phase="$1"
  local bundle_root="$2"
  local log_path="$ARTIFACT_DIR/${phase}.bundle-shape.log"
  if ! python3 - "$bundle_root" "$LATEST_PROOF_BUNDLE_PATH" >"$log_path" 2>&1 <<'PY'
from pathlib import Path
import json
import sys

bundle_root = Path(sys.argv[1])
pointer_path = Path(sys.argv[2])

if not bundle_root.is_dir():
    raise SystemExit(f'{bundle_root}: retained proof bundle directory missing')
expected_pointer = str(bundle_root)
actual_pointer = pointer_path.read_text(errors='replace').strip()
if actual_pointer != expected_pointer:
    raise SystemExit(
        f'latest-proof-bundle pointer drifted: expected {expected_pointer!r}, got {actual_pointer!r}'
    )

for required in [
    'verify-m054-s03.sh',
    'verify-m054-s03-contract.test.mjs',
    'e2e_m054_s03.rs',
    'website.docs.index.md',
    'website.docs._vitepress.config.mts',
    'website.docs.distributed-proof.index.md',
    'website.scripts.generate-og-image.py',
    'source-contract.log',
    'rust-contract.log',
    'generate-og.log',
    'build-docs.log',
    'built-html-assertions.log',
    'built-html-summary.json',
    'retained-og-image-v2.png',
]:
    if not (bundle_root / required).is_file():
        raise SystemExit(f'{bundle_root}: missing retained proof file {required}')

s02 = bundle_root / 'retained-m054-s02-verify'
for required in ['status.txt', 'current-phase.txt', 'phase-report.txt', 'full-contract.log', 'latest-proof-bundle.txt']:
    if not (s02 / required).is_file():
        raise SystemExit(f'{s02}: missing delegated S02 verify marker {required}')
if (s02 / 'status.txt').read_text(errors='replace').strip() != 'ok':
    raise SystemExit(f'{s02 / "status.txt"}: expected ok')
if (s02 / 'current-phase.txt').read_text(errors='replace').strip() != 'complete':
    raise SystemExit(f'{s02 / "current-phase.txt"}: expected complete')
s02_pointer = (s02 / 'latest-proof-bundle.txt').read_text(errors='replace').strip()
if not s02_pointer:
    raise SystemExit(f'{s02 / "latest-proof-bundle.txt"}: expected non-empty delegated proof bundle pointer')
if not Path(s02_pointer).exists():
    raise SystemExit(f'{s02 / "latest-proof-bundle.txt"}: delegated proof bundle path does not exist: {s02_pointer}')
phase_report = (s02 / 'phase-report.txt').read_text(errors='replace')
for marker in [
    'm054-s02-s01-replay\tpassed',
    'm054-s02-e2e\tpassed',
    'm054-s02-bundle-shape\tpassed',
]:
    if marker not in phase_report:
        raise SystemExit(f'{s02 / "phase-report.txt"}: missing delegated phase marker {marker!r}')

site_root = bundle_root / 'retained-site'
for required in ['index.html', 'docs/distributed-proof/index.html']:
    if not (site_root / required).is_file():
        raise SystemExit(f'{site_root}: missing retained built site file {required}')

index_html = (site_root / 'index.html').read_text(errors='replace')
proof_html = (site_root / 'docs/distributed-proof/index.html').read_text(errors='replace')
if 'One public app URL fronts multiple Mesh nodes. Runtime placement stays server-side, and operator truth stays on meshc cluster.' not in index_html:
    raise SystemExit('retained-site/index.html is missing the bounded homepage description')
if 'Built-in failover, load balancing, and exactly-once semantics' in index_html:
    raise SystemExit('retained-site/index.html still contains the stale generic tagline')
for marker in [
    'A proxy/platform ingress may expose one public app URL in front of multiple nodes, but that is where the public routing story ends.',
    'X-Mesh-Continuity-Request-Key',
    'meshc cluster continuity &lt;node-name@host:port&gt; &lt;request-key&gt; --json',
    'If you are inspecting startup work or doing manual discovery without a request key yet',
    'sticky sessions, frontend-aware routing, or client-visible topology claims',
]:
    if marker not in proof_html:
        raise SystemExit(f'retained-site/docs/distributed-proof/index.html missing {marker!r}')

summary_path = bundle_root / 'built-html-summary.json'
summary = json.loads(summary_path.read_text(errors='replace'))
for section in ['index', 'proof']:
    if section not in summary:
        raise SystemExit(f'{summary_path}: missing summary section {section!r}')
    checks = summary[section].get('checks')
    if not isinstance(checks, dict) or not checks:
        raise SystemExit(f'{summary_path}: missing checks map for {section!r}')
    failed = [name for name, ok in checks.items() if not ok]
    if failed:
        raise SystemExit(f'{summary_path}: {section} retained failed checks {failed}')

og_path = bundle_root / 'retained-og-image-v2.png'
if og_path.stat().st_size <= 0:
    raise SystemExit(f'{og_path}: expected non-empty copied OG image')

print('retained-bundle-shape: ok')
PY
  then
    fail_phase "$phase" "missing retained proof artifacts or malformed bundle pointer" "$log_path" "$bundle_root"
  fi
}

begin_phase m054-s03-db-env-preflight
if [[ -z "${DATABASE_URL:-}" ]]; then
  printf 'DATABASE_URL must be set for scripts/verify-m054-s03.sh\n' >"$ARTIFACT_DIR/m054-s03-db-env-preflight.log"
  record_phase m054-s03-db-env-preflight failed
  fail_phase m054-s03-db-env-preflight "DATABASE_URL must be set for scripts/verify-m054-s03.sh" "$ARTIFACT_DIR/m054-s03-db-env-preflight.log"
fi
if [[ "$DATABASE_URL" != postgres://* && "$DATABASE_URL" != postgresql://* ]]; then
  printf 'DATABASE_URL must start with postgres:// or postgresql://\n' >"$ARTIFACT_DIR/m054-s03-db-env-preflight.log"
  record_phase m054-s03-db-env-preflight failed
  fail_phase m054-s03-db-env-preflight "DATABASE_URL must start with postgres:// or postgresql://" "$ARTIFACT_DIR/m054-s03-db-env-preflight.log"
fi
record_phase m054-s03-db-env-preflight passed

run_expect_success m054-s03-source-contract m054-s03-source-contract no 300 scripts/tests/verify-m054-s03-contract.test.mjs \
  node --test scripts/tests/verify-m054-s03-contract.test.mjs
run_expect_success m054-s03-rust-contract m054-s03-rust-contract yes 1800 compiler/meshc/tests/e2e_m054_s03.rs \
  cargo test -p meshc --test e2e_m054_s03 -- --nocapture
run_expect_success_with_database_url m054-s03-s02-replay m054-s03-s02-replay no 7200 .tmp/m054-s02/verify \
  bash scripts/verify-m054-s02.sh
run_expect_success m054-s03-generate-og m054-s03-generate-og no 300 website/scripts/generate-og-image.py \
  npm --prefix website run generate:og
run_expect_success m054-s03-build-docs m054-s03-build-docs no 600 website/docs/.vitepress/config.mts \
  npm --prefix website run build

record_phase m054-s03-built-html-assertions started
assert_built_html_contract m054-s03-built-html-assertions "$BUILT_HTML_SUMMARY_PATH"
record_phase m054-s03-built-html-assertions passed

RETAINED_PROOF_BUNDLE_DIR="$PROOF_BUNDLES_DIR/retained-public-docs-proof-$(python3 - <<'PY'
import time
print(time.time_ns())
PY
)"
mkdir -p "$RETAINED_PROOF_BUNDLE_DIR"
printf '%s\n' "$RETAINED_PROOF_BUNDLE_DIR" >"$LATEST_PROOF_BUNDLE_PATH"

record_phase m054-s03-retain-s02-verify started
copy_fixed_dir_or_fail m054-s03-retain-s02-verify \
  "$ROOT_DIR/.tmp/m054-s02/verify" \
  "$RETAINED_PROOF_BUNDLE_DIR/retained-m054-s02-verify" \
  "retained M054 S02 verify directory is missing or malformed" \
  status.txt \
  current-phase.txt \
  phase-report.txt \
  full-contract.log \
  latest-proof-bundle.txt
record_phase m054-s03-retain-s02-verify passed

record_phase m054-s03-retain-source-and-logs started
copy_file_or_fail m054-s03-retain-source-and-logs \
  "$ROOT_DIR/scripts/verify-m054-s03.sh" \
  "$RETAINED_PROOF_BUNDLE_DIR/verify-m054-s03.sh" \
  "missing retained S03 verifier script"
copy_file_or_fail m054-s03-retain-source-and-logs \
  "$ROOT_DIR/scripts/tests/verify-m054-s03-contract.test.mjs" \
  "$RETAINED_PROOF_BUNDLE_DIR/verify-m054-s03-contract.test.mjs" \
  "missing retained S03 source contract test"
copy_file_or_fail m054-s03-retain-source-and-logs \
  "$ROOT_DIR/compiler/meshc/tests/e2e_m054_s03.rs" \
  "$RETAINED_PROOF_BUNDLE_DIR/e2e_m054_s03.rs" \
  "missing retained S03 Rust verifier contract"
copy_file_or_fail m054-s03-retain-source-and-logs \
  "$ROOT_DIR/website/docs/index.md" \
  "$RETAINED_PROOF_BUNDLE_DIR/website.docs.index.md" \
  "missing retained homepage source"
copy_file_or_fail m054-s03-retain-source-and-logs \
  "$ROOT_DIR/website/docs/.vitepress/config.mts" \
  "$RETAINED_PROOF_BUNDLE_DIR/website.docs._vitepress.config.mts" \
  "missing retained VitePress config source"
copy_file_or_fail m054-s03-retain-source-and-logs \
  "$ROOT_DIR/website/docs/docs/distributed-proof/index.md" \
  "$RETAINED_PROOF_BUNDLE_DIR/website.docs.distributed-proof.index.md" \
  "missing retained Distributed Proof source"
copy_file_or_fail m054-s03-retain-source-and-logs \
  "$ROOT_DIR/website/scripts/generate-og-image.py" \
  "$RETAINED_PROOF_BUNDLE_DIR/website.scripts.generate-og-image.py" \
  "missing retained OG generator source"
copy_file_or_fail m054-s03-retain-source-and-logs \
  "$ARTIFACT_DIR/m054-s03-source-contract.log" \
  "$RETAINED_PROOF_BUNDLE_DIR/source-contract.log" \
  "missing retained source contract log"
copy_file_or_fail m054-s03-retain-source-and-logs \
  "$ARTIFACT_DIR/m054-s03-rust-contract.log" \
  "$RETAINED_PROOF_BUNDLE_DIR/rust-contract.log" \
  "missing retained Rust contract log"
copy_file_or_fail m054-s03-retain-source-and-logs \
  "$ARTIFACT_DIR/m054-s03-generate-og.log" \
  "$RETAINED_PROOF_BUNDLE_DIR/generate-og.log" \
  "missing retained generate:og log"
copy_file_or_fail m054-s03-retain-source-and-logs \
  "$ARTIFACT_DIR/m054-s03-build-docs.log" \
  "$RETAINED_PROOF_BUNDLE_DIR/build-docs.log" \
  "missing retained docs build log"
copy_file_or_fail m054-s03-retain-source-and-logs \
  "$ARTIFACT_DIR/m054-s03-built-html-assertions.log" \
  "$RETAINED_PROOF_BUNDLE_DIR/built-html-assertions.log" \
  "missing retained built HTML assertion log"
copy_file_or_fail m054-s03-retain-source-and-logs \
  "$BUILT_HTML_SUMMARY_PATH" \
  "$RETAINED_PROOF_BUNDLE_DIR/built-html-summary.json" \
  "missing retained built HTML summary"
record_phase m054-s03-retain-source-and-logs passed

record_phase m054-s03-retain-site-evidence started
copy_file_or_fail m054-s03-retain-site-evidence \
  "$ROOT_DIR/website/docs/.vitepress/dist/index.html" \
  "$RETAINED_PROOF_BUNDLE_DIR/retained-site/index.html" \
  "missing retained built homepage HTML"
copy_file_or_fail m054-s03-retain-site-evidence \
  "$ROOT_DIR/website/docs/.vitepress/dist/docs/distributed-proof/index.html" \
  "$RETAINED_PROOF_BUNDLE_DIR/retained-site/docs/distributed-proof/index.html" \
  "missing retained built Distributed Proof HTML"
record_phase m054-s03-retain-site-evidence passed

record_phase m054-s03-retain-og-evidence started
copy_file_or_fail m054-s03-retain-og-evidence \
  "$ROOT_DIR/website/docs/public/og-image-v2.png" \
  "$RETAINED_PROOF_BUNDLE_DIR/retained-og-image-v2.png" \
  "missing retained generated OG asset"
record_phase m054-s03-retain-og-evidence passed

record_phase m054-s03-redaction-drift started
assert_no_secret_leaks m054-s03-redaction-drift "$RETAINED_PROOF_BUNDLE_DIR"
record_phase m054-s03-redaction-drift passed

record_phase m054-s03-bundle-shape started
assert_retained_bundle_shape m054-s03-bundle-shape "$RETAINED_PROOF_BUNDLE_DIR"
record_phase m054-s03-bundle-shape passed

assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^m054-s03-db-env-preflight\tpassed$' "DATABASE_URL preflight did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^m054-s03-source-contract\tpassed$' "Source contract replay did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^m054-s03-rust-contract\tpassed$' "Rust contract replay did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^m054-s03-s02-replay\tpassed$' "Delegated S02 replay did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^m054-s03-generate-og\tpassed$' "generate:og replay did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^m054-s03-build-docs\tpassed$' "docs build replay did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^m054-s03-built-html-assertions\tpassed$' "built HTML assertions did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^m054-s03-retain-s02-verify\tpassed$' "retained S02 verify copy did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^m054-s03-retain-source-and-logs\tpassed$' "retained source/log copy did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^m054-s03-retain-site-evidence\tpassed$' "retained built HTML copy did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^m054-s03-retain-og-evidence\tpassed$' "retained OG copy did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^m054-s03-redaction-drift\tpassed$' "redaction drift check did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^m054-s03-bundle-shape\tpassed$' "retained bundle shape check did not pass" "$ARTIFACT_DIR/full-contract.log"

echo "verify-m054-s03: ok"
echo "artifacts: $(repo_rel "$ARTIFACT_DIR")"
echo "proof bundle: $(repo_rel "$RETAINED_PROOF_BUNDLE_DIR")"
