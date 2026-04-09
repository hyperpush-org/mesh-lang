#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

ARTIFACT_ROOT=".tmp/m053-s04"
ARTIFACT_DIR="$ARTIFACT_ROOT/verify"
PHASE_REPORT_PATH="$ARTIFACT_DIR/phase-report.txt"
STATUS_PATH="$ARTIFACT_DIR/status.txt"
CURRENT_PHASE_PATH="$ARTIFACT_DIR/current-phase.txt"
LATEST_PROOF_BUNDLE_PATH="$ARTIFACT_DIR/latest-proof-bundle.txt"
LOG_INDEX_PATH="$ARTIFACT_DIR/log-paths.txt"
BUILT_HTML_DIR="$ARTIFACT_DIR/built-html"
BUILT_HTML_SUMMARY_PATH="$BUILT_HTML_DIR/summary.json"
RETAINED_FIRST_CONTACT_BUNDLE_DIR="$ARTIFACT_DIR/retained-m050-s02-verify"

GETTING_STARTED_HTML_PATH="$ROOT_DIR/website/docs/.vitepress/dist/docs/getting-started/index.html"
CLUSTERED_EXAMPLE_HTML_PATH="$ROOT_DIR/website/docs/.vitepress/dist/docs/getting-started/clustered-example/index.html"
TOOLING_HTML_PATH="$ROOT_DIR/website/docs/.vitepress/dist/docs/tooling/index.html"
DISTRIBUTED_HTML_PATH="$ROOT_DIR/website/docs/.vitepress/dist/docs/distributed/index.html"
DISTRIBUTED_PROOF_HTML_PATH="$ROOT_DIR/website/docs/.vitepress/dist/docs/distributed-proof/index.html"

repo_rel() {
  local candidate="$1"
  if [[ "$candidate" == "$ROOT_DIR/"* ]]; then
    printf '%s\n' "${candidate#$ROOT_DIR/}"
  else
    printf '%s\n' "$candidate"
  fi
}

rm -rf "$ARTIFACT_DIR"
mkdir -p "$ARTIFACT_DIR" "$BUILT_HTML_DIR"
exec > >(tee "$ARTIFACT_DIR/full-contract.log") 2>&1

: >"$PHASE_REPORT_PATH"
: >"$LOG_INDEX_PATH"
printf 'running\n' >"$STATUS_PATH"
printf 'init\n' >"$CURRENT_PHASE_PATH"
printf '%s\n' "$ARTIFACT_DIR" >"$LATEST_PROOF_BUNDLE_PATH"

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

note_log() {
  local label="$1"
  local path="$2"
  printf '%s\t%s\n' "$label" "$(repo_rel "$path")" >>"$LOG_INDEX_PATH"
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
for line in lines[:220]:
    print(line)
if len(lines) > 220:
    print(f"... truncated after 220 lines (total {len(lines)})")
PY
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

require_command() {
  local phase="$1"
  local command_name="$2"
  local description="$3"
  local artifact_hint="${4:-}"
  if command -v "$command_name" >/dev/null 2>&1; then
    return 0
  fi

  local log_path="$ARTIFACT_DIR/${phase}.preflight.log"
  {
    echo "preflight: missing required command"
    echo "description: ${description}"
    echo "command: ${command_name}"
  } >"$log_path"
  note_log "${phase}-preflight" "$log_path"
  record_phase "$phase" failed
  fail_phase "$phase" "missing required command: ${command_name}" "$log_path" "$artifact_hint"
}

require_file() {
  local phase="$1"
  local path="$2"
  local description="$3"
  local artifact_hint="${4:-}"
  if [[ -f "$path" ]]; then
    return 0
  fi

  local log_path="$ARTIFACT_DIR/${phase}.preflight.log"
  {
    echo "preflight: missing required file"
    echo "description: ${description}"
    echo "path: $(repo_rel "$path")"
  } >"$log_path"
  note_log "${phase}-preflight" "$log_path"
  record_phase "$phase" failed
  fail_phase "$phase" "missing required file: $(repo_rel "$path")" "$log_path" "$artifact_hint"
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

run_expect_success() {
  local phase="$1"
  local label="$2"
  local timeout_secs="$3"
  local artifact_hint="$4"
  shift 4
  local -a cmd=("$@")
  local log_path="$ARTIFACT_DIR/${label}.log"

  begin_phase "$phase"
  note_log "$label" "$log_path"
  echo "==> ${cmd[*]}"
  if ! run_command "$timeout_secs" "$log_path" "${cmd[@]}"; then
    record_phase "$phase" failed
    fail_phase "$phase" "expected success within ${timeout_secs}s" "$log_path" "$artifact_hint"
  fi
  record_phase "$phase" passed
}

copy_file_or_fail() {
  local phase="$1"
  local source_path="$2"
  local dest_path="$3"
  local description="$4"
  local log_path="$ARTIFACT_DIR/${phase}.copy.log"
  note_log "${phase}-copy" "$log_path"

  if [[ ! -f "$source_path" ]]; then
    {
      echo "copy: missing source file"
      echo "description: ${description}"
      echo "source: $(repo_rel "$source_path")"
    } >"$log_path"
    record_phase "$phase" failed
    fail_phase "$phase" "$description" "$log_path" "$source_path"
  fi

  mkdir -p "$(dirname "$dest_path")"
  cp "$source_path" "$dest_path"
  if [[ ! -s "$dest_path" ]]; then
    {
      echo "copy: destination file is empty"
      echo "description: ${description}"
      echo "source: $(repo_rel "$source_path")"
      echo "destination: $(repo_rel "$dest_path")"
    } >"$log_path"
    record_phase "$phase" failed
    fail_phase "$phase" "$description" "$log_path" "$dest_path"
  fi

  printf 'copied %s -> %s\n' "$(repo_rel "$source_path")" "$(repo_rel "$dest_path")" >>"$log_path"
}

copy_fixed_dir_or_fail() {
  local phase="$1"
  local source_dir="$2"
  local dest_dir="$3"
  local description="$4"
  shift 4
  local log_path="$ARTIFACT_DIR/${phase}.copy.log"
  note_log "${phase}-copy" "$log_path"

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
    path = source_dir / rel
    if not path.exists():
        raise SystemExit(f"{description}: missing {rel} in {source_dir}")
    if path.is_file() and not path.read_text(errors='replace').strip():
        raise SystemExit(f"{description}: empty required file {path}")
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

assert_log_contains() {
  local phase="$1"
  local log_path="$2"
  local needle="$3"
  local artifact_hint="${4:-}"
  local check_log="$ARTIFACT_DIR/${phase}.assert.log"
  note_log "${phase}-assert" "$check_log"

  if ! python3 - "$log_path" "$needle" >"$check_log" 2>&1 <<'PY'
from pathlib import Path
import sys

log_path = Path(sys.argv[1])
needle = sys.argv[2]
text = log_path.read_text(errors='replace')
if needle not in text:
    raise SystemExit(f"missing log marker {needle!r} in {log_path}")
print(f"found {needle!r}")
PY
  then
    record_phase "$phase" failed
    fail_phase "$phase" "wrapped rail output shape drifted" "$check_log" "$artifact_hint"
  fi
}

assert_first_contact_bundle() {
  local phase="$1"
  local source_dir="$2"
  local log_path="$ARTIFACT_DIR/${phase}.assert.log"
  note_log "${phase}-assert" "$log_path"

  if ! python3 - "$source_dir" >"$log_path" 2>&1 <<'PY'
from pathlib import Path
import sys

source_dir = Path(sys.argv[1])
status = (source_dir / 'status.txt').read_text(errors='replace').strip()
current = (source_dir / 'current-phase.txt').read_text(errors='replace').strip()
phase_report = (source_dir / 'phase-report.txt').read_text(errors='replace')
pointer = (source_dir / 'latest-proof-bundle.txt').read_text(errors='replace').strip()
if status != 'ok':
    raise SystemExit(f"expected status ok, found {status!r}")
if current != 'complete':
    raise SystemExit(f"expected current phase complete, found {current!r}")
for marker in [
    'docs-build\tpassed',
    'retain-built-html\tpassed',
    'built-html\tpassed',
    'm050-s02-bundle-shape\tpassed',
]:
    if marker not in phase_report:
        raise SystemExit(f"missing phase marker {marker!r}")
if pointer != '.tmp/m050-s02/verify':
    raise SystemExit(f"latest-proof-bundle pointer drifted: {pointer!r}")
summary_path = source_dir / 'built-html' / 'summary.json'
if not summary_path.is_file():
    raise SystemExit(f"missing built-html summary {summary_path}")
if not summary_path.read_text(errors='replace').strip():
    raise SystemExit(f"empty built-html summary {summary_path}")
print('m050-s02 bundle: ok')
PY
  then
    record_phase "$phase" failed
    fail_phase "$phase" "wrapped first-contact bundle drifted" "$log_path" "$source_dir"
  fi
}

assert_built_html_contract() {
  local phase="$1"
  local summary_path="$2"
  local log_path="$ARTIFACT_DIR/${phase}.assert.log"
  note_log "${phase}-assert" "$log_path"

  if ! python3 - \
    "$GETTING_STARTED_HTML_PATH" \
    "$CLUSTERED_EXAMPLE_HTML_PATH" \
    "$TOOLING_HTML_PATH" \
    "$DISTRIBUTED_HTML_PATH" \
    "$DISTRIBUTED_PROOF_HTML_PATH" \
    "$summary_path" >"$log_path" 2>&1 <<'PY'
from html.parser import HTMLParser
from pathlib import Path
import json
import re
import sys


class MainTextExtractor(HTMLParser):
    def __init__(self):
        super().__init__()
        self.parts = []
        self.skip_depth = 0

    def handle_starttag(self, tag, attrs):
        if tag in {'script', 'style'}:
            self.skip_depth += 1

    def handle_endtag(self, tag):
        if tag in {'script', 'style'} and self.skip_depth:
            self.skip_depth -= 1

    def handle_data(self, data):
        if not self.skip_depth:
            self.parts.append(data)


def load_main(path: Path):
    if not path.is_file():
        raise SystemExit(f'missing built HTML snapshot: {path}')
    html = path.read_text(errors='replace')
    match = re.search(r'<main(?:\s[^>]*)?>(?P<body>[\s\S]*?)</main>', html)
    if not match:
        raise SystemExit(f'missing <main> content in {path}')
    extractor = MainTextExtractor()
    extractor.feed(match.group('body'))
    text = ' '.join(' '.join(extractor.parts).split())
    return html, text


def marker_map(text: str, markers: list[str], label: str):
    positions = {}
    cursor = -1
    for marker in markers:
        index = text.find(marker)
        if index == -1:
            raise SystemExit(f'{label}: missing marker {marker!r}')
        if index <= cursor:
            raise SystemExit(f'{label}: marker order drifted around {marker!r}')
        positions[marker] = index
        cursor = index
    return positions


def require_markers(text: str, markers: list[str], label: str):
    for marker in markers:
        if marker not in text:
            raise SystemExit(f'{label}: missing required marker {marker!r}')


def require_absent(text: str, markers: list[str], label: str):
    for marker in markers:
        if marker in text:
            raise SystemExit(f'{label}: stale marker leaked into built HTML {marker!r}')


def require_count(text: str, marker: str, expected: int, label: str):
    actual = text.count(marker)
    if actual != expected:
        raise SystemExit(f'{label}: expected {expected} instance(s) of {marker!r}, found {actual}')


getting_started_html, getting_started_text = load_main(Path(sys.argv[1]))
clustered_example_html, clustered_example_text = load_main(Path(sys.argv[2]))
tooling_html, tooling_text = load_main(Path(sys.argv[3]))
distributed_html, distributed_text = load_main(Path(sys.argv[4]))
distributed_proof_html, distributed_proof_text = load_main(Path(sys.argv[5]))
summary_path = Path(sys.argv[6])

summary = {
    'getting_started': {
        'path': sys.argv[1],
        'markers': marker_map(
            getting_started_text,
            [
                'Choose your next starter',
                'meshc init --clustered hello_cluster',
                'meshc init --template todo-api --db sqlite todo_api',
                'meshc init --template todo-api --db postgres shared_todo',
                'Production Backend Proof',
            ],
            'getting-started',
        ),
    },
    'clustered_example': {
        'path': sys.argv[2],
        'markers': marker_map(
            clustered_example_text,
            [
                'After the scaffold, pick the follow-on starter',
                'meshc init --template todo-api --db sqlite my_local_todo',
                'meshc init --template todo-api --db postgres my_shared_todo',
                'Production Backend Proof — the maintainer-facing backend proof page after the starter/examples-first ladder, where those deeper proof commands stay behind the proof pages.',
                'Need the retained verifier map?',
                'Distributed Proof',
            ],
            'clustered-example',
        ),
    },
    'tooling': {
        'path': sys.argv[3],
        'markers': marker_map(
            tooling_text,
            [
                'Creating a New Project',
                'meshc init --clustered my_clustered_app',
                'meshc init --template todo-api --db sqlite my_local_todo',
                'meshc init --template todo-api --db postgres my_shared_todo',
                'Assembled first-contact docs verifier',
                'bash scripts/verify-m050-s02.sh',
            ],
            'tooling',
        ),
    },
    'distributed': {
        'path': sys.argv[4],
        'markers': marker_map(
            distributed_text,
            [
                'Clustered proof surfaces:',
                'Distributed Proof',
                'M053 starter-owned staged deploy + failover + hosted-contract proof map',
                'retained read-only Fly reference lane',
            ],
            'distributed',
        ),
    },
    'distributed_proof': {
        'path': sys.argv[5],
        'markers': marker_map(
            distributed_proof_text,
            [
                'This is the only public-secondary docs page that carries the named clustered verifier rails.',
                'bash scripts/verify-m053-s01.sh',
                'bash scripts/verify-m053-s02.sh',
                'bash scripts/verify-m053-s03.sh',
                'bash scripts/verify-m043-s04-fly.sh --help',
            ],
            'distributed-proof',
        ),
    },
}

require_markers(getting_started_text, [
    'honest local-only starter',
    'single-node only',
    'staged deploy + failover proof chain',
    'hosted packages/public-surface contract',
], 'getting-started')
require_markers(clustered_example_text, [
    'This page stays on that scaffold first.',
    'staged deploy + failover proof chain',
    'hosted packages/public-surface contract',
], 'clustered-example')
require_markers(tooling_text, [
    'Keep the public CLI workflow explicit and examples-first',
    'SQLite stays local-only and single-node only here;',
    'staged deploy + failover proof chain',
    'hosted packages/public-surface contract',
    'Assembled first-contact docs verifier',
], 'tooling')
require_markers(distributed_text, [
    'Clustered proof surfaces:',
    'M053 starter-owned staged deploy + failover + hosted-contract proof map',
    'retained read-only Fly reference lane',
], 'distributed')
require_markers(distributed_proof_text, [
    'generated PostgreSQL starter\'s M053 chain',
    'bash scripts/verify-m053-s01.sh',
    'bash scripts/verify-m053-s02.sh',
    'bash scripts/verify-m053-s03.sh',
    'keep Fly as a retained read-only reference/proof lane for already-deployed environments instead of treating it as a coequal public starter surface',
    'The Fly verifier is intentionally read-only and intentionally secondary.',
], 'distributed-proof')
require_absent(tooling_text, [
    '/) -- building distributed systems with Mesh',
], 'tooling')
require_count(tooling_text, 'Distributed Actors -- building distributed systems with Mesh', 1, 'tooling')
require_absent(distributed_text, [
    'bash scripts/verify-m047-s04.sh',
    'CLUSTER_PROOF_FLY_APP=',
], 'distributed')
require_absent(distributed_proof_text, [
    'keep `reference-backend` as the deeper backend proof surface rather than a coequal first-contact clustered starter',
], 'distributed-proof')

summary_path.write_text(json.dumps(summary, indent=2) + '\n')
print('built-html-contract: ok')
PY
  then
    record_phase "$phase" failed
    fail_phase "$phase" "built HTML docs contract drifted" "$log_path" "$BUILT_HTML_DIR"
  fi
}

assert_bundle_shape() {
  local phase="$1"
  local log_path="$ARTIFACT_DIR/${phase}.assert.log"
  note_log "${phase}-assert" "$log_path"

  if ! python3 - "$ARTIFACT_DIR" "$LATEST_PROOF_BUNDLE_PATH" "$LOG_INDEX_PATH" "$BUILT_HTML_SUMMARY_PATH" >"$log_path" 2>&1 <<'PY'
from pathlib import Path
import json
import sys

artifact_dir = Path(sys.argv[1])
pointer_path = Path(sys.argv[2])
log_index_path = Path(sys.argv[3])
summary_path = Path(sys.argv[4])
expected_pointer = str(artifact_dir)
actual_pointer = pointer_path.read_text(errors='replace').strip()
if actual_pointer != expected_pointer:
    raise SystemExit(f'latest-proof-bundle pointer drifted: expected {expected_pointer!r}, got {actual_pointer!r}')

required_files = [
    'status.txt',
    'current-phase.txt',
    'phase-report.txt',
    'full-contract.log',
    'latest-proof-bundle.txt',
    'log-paths.txt',
    'docs-build.log',
    'first-contact-rail.log',
    'proof-surface-rail.log',
    'cluster-proof-fixture.log',
    'm053-s04-contract.log',
    'built-html/summary.json',
    'retained-m050-s02-verify/status.txt',
    'retained-m050-s02-verify/current-phase.txt',
    'retained-m050-s02-verify/phase-report.txt',
    'retained-m050-s02-verify/latest-proof-bundle.txt',
]
for rel in required_files:
    path = artifact_dir / rel
    if not path.exists():
        raise SystemExit(f'missing required artifact {path}')
    if path.is_file() and not path.read_text(errors='replace').strip():
        raise SystemExit(f'expected non-empty artifact {path}')

summary = json.loads(summary_path.read_text(errors='replace'))
for key in ['getting_started', 'clustered_example', 'tooling', 'distributed', 'distributed_proof']:
    if key not in summary:
        raise SystemExit(f'built HTML summary missing key {key!r}')

phase_report = (artifact_dir / 'phase-report.txt').read_text(errors='replace')
for marker in [
    'init\tpassed',
    'docs-build\tpassed',
    'retain-built-html\tpassed',
    'built-html\tpassed',
    'first-contact-rail\tpassed',
    'retain-first-contact-bundle\tpassed',
    'proof-surface-rail\tpassed',
    'proof-surface-output\tpassed',
    'cluster-proof-fixture\tpassed',
    'm053-s04-contract\tpassed',
]:
    if marker not in phase_report:
        raise SystemExit(f'phase report missing marker {marker!r}')

log_index = log_index_path.read_text(errors='replace')
for label in ['docs-build', 'first-contact-rail', 'proof-surface-rail', 'cluster-proof-fixture', 'm053-s04-contract']:
    if f'{label}\t' not in log_index:
        raise SystemExit(f'log index missing {label!r}')

print('bundle-shape: ok')
PY
  then
    record_phase "$phase" failed
    fail_phase "$phase" "assembled verifier bundle drifted" "$log_path" "$ARTIFACT_DIR"
  fi
}

record_phase init started
for command_name in bash node npm cargo python3 rg; do
  require_command init "$command_name" "required command for the M053 S04 docs/reference verifier"
done
for path in \
  "$ROOT_DIR/scripts/tests/verify-m053-s04-contract.test.mjs" \
  "$ROOT_DIR/scripts/verify-m050-s02.sh" \
  "$ROOT_DIR/scripts/verify-production-proof-surface.sh" \
  "$ROOT_DIR/scripts/fixtures/clustered/cluster-proof/tests/work.test.mpl"; do
  require_file init "$path" "required S04 verifier surface"
done
record_phase init passed

run_expect_success docs-build docs-build 2400 "website/docs/.vitepress/dist/docs" \
  npm --prefix website run build

begin_phase retain-built-html
copy_file_or_fail retain-built-html "$GETTING_STARTED_HTML_PATH" "$BUILT_HTML_DIR/getting-started.index.html" "missing built Getting Started HTML snapshot after docs build"
copy_file_or_fail retain-built-html "$CLUSTERED_EXAMPLE_HTML_PATH" "$BUILT_HTML_DIR/clustered-example.index.html" "missing built Clustered Example HTML snapshot after docs build"
copy_file_or_fail retain-built-html "$TOOLING_HTML_PATH" "$BUILT_HTML_DIR/tooling.index.html" "missing built Tooling HTML snapshot after docs build"
copy_file_or_fail retain-built-html "$DISTRIBUTED_HTML_PATH" "$BUILT_HTML_DIR/distributed.index.html" "missing built Distributed HTML snapshot after docs build"
copy_file_or_fail retain-built-html "$DISTRIBUTED_PROOF_HTML_PATH" "$BUILT_HTML_DIR/distributed-proof.index.html" "missing built Distributed Proof HTML snapshot after docs build"
record_phase retain-built-html passed

begin_phase built-html
assert_built_html_contract built-html "$BUILT_HTML_SUMMARY_PATH"
record_phase built-html passed

run_expect_success first-contact-rail first-contact-rail 2400 ".tmp/m050-s02/verify" \
  bash scripts/verify-m050-s02.sh

begin_phase retain-first-contact-bundle
assert_first_contact_bundle retain-first-contact-bundle "$ROOT_DIR/.tmp/m050-s02/verify"
copy_fixed_dir_or_fail retain-first-contact-bundle \
  "$ROOT_DIR/.tmp/m050-s02/verify" \
  "$RETAINED_FIRST_CONTACT_BUNDLE_DIR" \
  "M050 S02 verify bundle is missing or malformed" \
  status.txt \
  current-phase.txt \
  phase-report.txt \
  latest-proof-bundle.txt \
  full-contract.log \
  built-html/summary.json
record_phase retain-first-contact-bundle passed

run_expect_success proof-surface-rail proof-surface-rail 900 "website/docs/docs/distributed-proof/index.md" \
  bash scripts/verify-production-proof-surface.sh

begin_phase proof-surface-output
assert_log_contains proof-surface-output "$ARTIFACT_DIR/proof-surface-rail.log" '[proof-docs] production proof surface verified' "website/docs/docs/distributed-proof/index.md"
record_phase proof-surface-output passed

run_expect_success cluster-proof-fixture cluster-proof-fixture 900 "scripts/fixtures/clustered/cluster-proof/tests" \
  cargo run -q -p meshc -- test scripts/fixtures/clustered/cluster-proof/tests

run_expect_success m053-s04-contract m053-s04-contract 300 "scripts/tests/verify-m053-s04-contract.test.mjs" \
  node --test scripts/tests/verify-m053-s04-contract.test.mjs

begin_phase verifier-bundle
assert_bundle_shape verifier-bundle
record_phase verifier-bundle passed

for expected_phase in \
  init \
  docs-build \
  retain-built-html \
  built-html \
  first-contact-rail \
  retain-first-contact-bundle \
  proof-surface-rail \
  proof-surface-output \
  cluster-proof-fixture \
  m053-s04-contract \
  verifier-bundle; do
  if ! rg -q "^${expected_phase}\\tpassed$" "$PHASE_REPORT_PATH"; then
    fail_phase verifier-status "phase report missing passed marker for ${expected_phase}" "$PHASE_REPORT_PATH"
  fi
done

echo "verify-m053-s04: ok"
echo "artifacts: $(repo_rel "$ARTIFACT_DIR")"
echo "proof bundle: $(repo_rel "$ARTIFACT_DIR")"
