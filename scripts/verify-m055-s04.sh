#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

# shellcheck source=scripts/lib/m055-workspace.sh
source "$ROOT_DIR/scripts/lib/m055-workspace.sh"

ARTIFACT_ROOT=".tmp/m055-s04"
ARTIFACT_DIR="$ARTIFACT_ROOT/verify"
WORKSPACE_ROOT="$ARTIFACT_ROOT/workspace"
STAGED_PRODUCT_ROOT="$ROOT_DIR/$WORKSPACE_ROOT/hyperpush-mono"
STAGED_PRODUCT_SUMMARY_PATH="$ROOT_DIR/$WORKSPACE_ROOT/hyperpush-mono.stage.json"
STAGED_PRODUCT_MANIFEST_PATH="$ROOT_DIR/$WORKSPACE_ROOT/hyperpush-mono.manifest.json"
PHASE_REPORT_PATH="$ARTIFACT_DIR/phase-report.txt"
STATUS_PATH="$ARTIFACT_DIR/status.txt"
CURRENT_PHASE_PATH="$ARTIFACT_DIR/current-phase.txt"
FULL_LOG_PATH="$ARTIFACT_DIR/full-contract.log"
LATEST_PROOF_BUNDLE_PATH="$ARTIFACT_DIR/latest-proof-bundle.txt"
LANGUAGE_REPO_METADATA_PATH="$ARTIFACT_DIR/language-repo.meta.json"
PRODUCT_REPO_METADATA_PATH="$ARTIFACT_DIR/product-repo.meta.json"
LANGUAGE_PROOF_BUNDLE_POINTER_PATH="$ARTIFACT_DIR/language-proof-bundle.txt"
PRODUCT_PROOF_BUNDLE_POINTER_PATH="$ARTIFACT_DIR/product-proof-bundle.txt"
RETAINED_PROOF_BUNDLE_DIR="$ARTIFACT_DIR/retained-proof-bundle"
RETAINED_ARTIFACTS_MANIFEST_PATH="$ARTIFACT_DIR/retained-m055-s04-artifacts.manifest.txt"
REAL_PRODUCT_ROOT=""
LANGUAGE_VERIFY_DIR="$ROOT_DIR/.tmp/m055-s03/verify"
PRODUCT_M051_VERIFY_DIR=""
PRODUCT_LANDING_VERIFY_DIR=""

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
  record_phase "$phase" failed
  fail_phase "$phase" "missing required file: $(repo_rel "$path")" "$log_path" "$artifact_hint"
}

require_git_tracked() {
  local phase="$1"
  local repo_root="$2"
  local relative_path="$3"
  local description="$4"
  local artifact_hint="${5:-$repo_root}"
  local log_path="$ARTIFACT_DIR/${phase}.preflight.log"

  if git -C "$repo_root" ls-files --error-unmatch -- "$relative_path" >"$log_path" 2>&1; then
    return 0
  fi

  {
    echo "preflight: required tracked repo path missing"
    echo "description: ${description}"
    echo "repo_root: ${repo_root}"
    echo "relative_path: ${relative_path}"
    echo
    cat "$log_path"
  } >"$log_path.tmp"
  mv "$log_path.tmp" "$log_path"
  record_phase "$phase" failed
  fail_phase "$phase" "required tracked repo path missing: ${relative_path}" "$log_path" "$artifact_hint"
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

failure_reason_for_exit() {
  local exit_code="$1"
  local timeout_secs="$2"
  if [[ "$exit_code" -eq 124 ]]; then
    printf 'command timed out after %ss' "$timeout_secs"
  else
    printf 'command exited with status %s before %ss deadline' "$exit_code" "$timeout_secs"
  fi
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
  echo "==> ${cmd[*]}"
  if run_command "$timeout_secs" "$log_path" "${cmd[@]}"; then
    :
  else
    local exit_code=$?
    record_phase "$phase" failed
    fail_phase "$phase" "$(failure_reason_for_exit "$exit_code" "$timeout_secs")" "$log_path" "$artifact_hint"
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

copy_pointed_bundle_or_fail() {
  local phase="$1"
  local source_repo_root="$2"
  local source_verify_dir="$3"
  local dest_pointer_path="$4"
  local dest_bundle_dir="$5"
  local description="$6"
  shift 6
  local log_path="$ARTIFACT_DIR/${phase}.copy.log"

  if ! python3 - "$source_repo_root" "$source_verify_dir" "$dest_pointer_path" "$dest_bundle_dir" "$description" "$@" >"$log_path" 2>&1 <<'PY'
from pathlib import Path
import shutil
import sys

source_repo_root = Path(sys.argv[1]).resolve()
source_verify_dir = Path(sys.argv[2])
dest_pointer_path = Path(sys.argv[3])
dest_bundle_dir = Path(sys.argv[4])
description = sys.argv[5]
required = sys.argv[6:]

pointer_path = source_verify_dir / 'latest-proof-bundle.txt'
if not pointer_path.is_file():
    raise SystemExit(f"{description}: missing latest-proof-bundle.txt in {source_verify_dir}")
pointer_text = pointer_path.read_text(errors='replace').strip()
if not pointer_text:
    raise SystemExit(f"{description}: empty latest-proof-bundle.txt in {source_verify_dir}")
source_bundle_dir = Path(pointer_text)
if not source_bundle_dir.is_absolute():
    source_bundle_dir = (source_repo_root / source_bundle_dir).resolve()
if not source_bundle_dir.is_dir():
    raise SystemExit(
        f"{description}: bundle pointer {pointer_text!r} resolved to missing directory {source_bundle_dir}"
    )
for rel in required:
    if not (source_bundle_dir / rel).exists():
        raise SystemExit(f"{description}: missing {rel} in {source_bundle_dir}")
if dest_bundle_dir.exists():
    shutil.rmtree(dest_bundle_dir)
dest_bundle_dir.parent.mkdir(parents=True, exist_ok=True)
shutil.copytree(source_bundle_dir, dest_bundle_dir, symlinks=True)
dest_pointer_path.parent.mkdir(parents=True, exist_ok=True)
dest_pointer_path.write_text(str(dest_bundle_dir.resolve()) + '\n')
print(f"copied {source_bundle_dir} -> {dest_bundle_dir}")
PY
  then
    record_phase "$phase" failed
    fail_phase "$phase" "$description" "$log_path" "$source_verify_dir"
  fi
}

capture_repo_metadata_or_fail() {
  local phase="$1"
  local language_meta_path="$2"
  local product_meta_path="$3"
  local language_pointer_path="$4"
  local product_pointer_path="$5"
  local log_path="$ARTIFACT_DIR/${phase}.metadata.log"

  local language_slug
  local product_slug
  language_slug="$(m055_repo_identity_field "$ROOT_DIR" 'languageRepo.slug')" || {
    record_phase "$phase" failed
    fail_phase "$phase" "failed to resolve language repo identity" "$log_path" "scripts/lib/repo-identity.json"
  }
  product_slug="$(m055_repo_identity_field "$ROOT_DIR" 'productRepo.slug')" || {
    record_phase "$phase" failed
    fail_phase "$phase" "failed to resolve product repo identity" "$log_path" "scripts/lib/repo-identity.json"
  }

  local language_workspace_dir
  local product_workspace_dir
  language_workspace_dir="$(m055_repo_identity_field "$ROOT_DIR" 'languageRepo.workspaceDir')"
  product_workspace_dir="$(m055_repo_identity_field "$ROOT_DIR" 'productRepo.workspaceDir')"

  local language_repo_url
  local product_repo_url
  language_repo_url="$(m055_repo_identity_field "$ROOT_DIR" 'languageRepo.repoUrl')"
  product_repo_url="$(m055_repo_identity_field "$ROOT_DIR" 'productRepo.repoUrl')"

  local language_ref
  local product_ref
  language_ref="$(git -C "$ROOT_DIR" rev-parse HEAD 2>/dev/null)" || {
    printf 'failed to resolve git ref for %s\n' "$ROOT_DIR" >"$log_path"
    record_phase "$phase" failed
    fail_phase "$phase" "failed to resolve language repo ref" "$log_path" "$ROOT_DIR"
  }
  product_ref="$(git -C "$REAL_PRODUCT_ROOT" rev-parse HEAD 2>/dev/null)" || {
    printf 'failed to resolve git ref for %s\n' "$REAL_PRODUCT_ROOT" >"$log_path"
    record_phase "$phase" failed
    fail_phase "$phase" "failed to resolve product repo ref" "$log_path" "$REAL_PRODUCT_ROOT"
  }

  if ! python3 - \
    "$ROOT_DIR" \
    "$REAL_PRODUCT_ROOT" \
    "$STAGED_PRODUCT_SUMMARY_PATH" \
    "$STAGED_PRODUCT_MANIFEST_PATH" \
    "$language_meta_path" \
    "$product_meta_path" \
    "$language_pointer_path" \
    "$product_pointer_path" \
    "$language_slug" \
    "$language_workspace_dir" \
    "$language_repo_url" \
    "$language_ref" \
    "$product_slug" \
    "$product_workspace_dir" \
    "$product_repo_url" \
    "$product_ref" \
    "$M055_HYPERPUSH_ROOT_SOURCE" >"$log_path" 2>&1 <<'PY'
from pathlib import Path
import json
import sys

(
    root_dir,
    real_product_root,
    stage_summary_path,
    stage_manifest_path,
    language_meta_path,
    product_meta_path,
    language_pointer_path,
    product_pointer_path,
    language_slug,
    language_workspace_dir,
    language_repo_url,
    language_ref,
    product_slug,
    product_workspace_dir,
    product_repo_url,
    product_ref,
    product_root_source,
) = [Path(sys.argv[1]).resolve(), Path(sys.argv[2]).resolve(), Path(sys.argv[3]), Path(sys.argv[4]), Path(sys.argv[5]), Path(sys.argv[6]), Path(sys.argv[7]), Path(sys.argv[8]), sys.argv[9], sys.argv[10], sys.argv[11], sys.argv[12], sys.argv[13], sys.argv[14], sys.argv[15], sys.argv[16], sys.argv[17]]

stage_summary = json.loads(stage_summary_path.read_text())
stage_manifest = json.loads(stage_manifest_path.read_text())
language_pointer = language_pointer_path.read_text(errors='replace').strip()
product_pointer = product_pointer_path.read_text(errors='replace').strip()
if not language_pointer:
    raise SystemExit('language proof bundle pointer was empty')
if not product_pointer:
    raise SystemExit('product proof bundle pointer was empty')

language_meta = {
    'repoRole': 'language',
    'workspaceDir': language_workspace_dir,
    'slug': language_slug,
    'repoUrl': language_repo_url,
    'repoRoot': str(root_dir),
    'ref': language_ref,
    'refSource': 'git:rev-parse:HEAD',
    'verifierEntrypoint': 'scripts/verify-m055-s03.sh',
    'verifyDir': str((root_dir / '.tmp/m055-s03/verify').resolve()),
    'proofBundlePointer': language_pointer,
}

product_fingerprint = stage_manifest.get('fingerprint')
if not isinstance(product_fingerprint, str) or not product_fingerprint:
    raise SystemExit(f'{stage_manifest_path}: missing manifest.fingerprint')
product_output_root = stage_summary.get('outputRoot')
if not isinstance(product_output_root, str) or not product_output_root:
    raise SystemExit(f'{stage_summary_path}: missing outputRoot')
product_meta = {
    'repoRole': 'product',
    'workspaceDir': product_workspace_dir,
    'slug': product_slug,
    'repoUrl': product_repo_url,
    'repoRoot': str(real_product_root),
    'repoRootSource': product_root_source,
    'ref': product_ref,
    'refSource': 'git:rev-parse:HEAD',
    'materializeCheckOutputRoot': product_output_root,
    'materializeCheckManifestFingerprint': product_fingerprint,
    'materializeCheckManifestPath': str(stage_manifest_path.resolve()),
    'materializeCheckSummaryPath': str(stage_summary_path.resolve()),
    'verifierEntrypoints': [
        'scripts/verify-m051-s01.sh',
        'scripts/verify-landing-surface.sh',
    ],
    'verifyDirs': [
        str((real_product_root / '.tmp/m051-s01/verify').resolve()),
        str((real_product_root / '.tmp/m055-s04/landing-surface/verify').resolve()),
    ],
    'proofBundlePointer': product_pointer,
}

language_meta_path.write_text(json.dumps(language_meta, indent=2) + '\n')
product_meta_path.write_text(json.dumps(product_meta, indent=2) + '\n')
print(f'language ref={language_ref}')
print(f'product ref={product_ref}')
PY
  then
    record_phase "$phase" failed
    fail_phase "$phase" "failed to capture repo/ref metadata" "$log_path" "$REAL_PRODUCT_ROOT"
  fi
}

capture_copied_manifest_or_fail() {
  local phase="$1"
  local manifest_path="$2"
  shift 2
  local log_path="$ARTIFACT_DIR/${phase}.manifest.log"

  if ! python3 - "$manifest_path" "$@" >"$log_path" 2>&1 <<'PY'
from pathlib import Path
import sys

manifest_path = Path(sys.argv[1])
entries = [Path(value) for value in sys.argv[2:]]
lines = []
for entry in entries:
    if entry.is_dir():
        lines.append(f'{entry.name}\t{entry}')
        for child in sorted(entry.rglob('*')):
            if child.is_file():
                lines.append(f'  - {child}')
    elif entry.is_file():
        lines.append(f'{entry.name}\t{entry}')
    else:
        raise SystemExit(f'missing copied artifact for manifest: {entry}')
manifest_path.write_text('\n'.join(lines) + ('\n' if lines else ''))
print(f'wrote manifest with {len(lines)} lines to {manifest_path}')
PY
  then
    record_phase "$phase" failed
    fail_phase "$phase" "failed to write retained artifact manifest" "$log_path" "$RETAINED_PROOF_BUNDLE_DIR"
  fi
}

assert_retained_bundle_shape() {
  local phase="$1"
  local bundle_root="$2"
  local pointer_path="$3"
  local manifest_path="$4"
  local log_path="$ARTIFACT_DIR/${phase}.bundle-check.log"

  if ! python3 - "$bundle_root" "$pointer_path" "$manifest_path" >"$log_path" 2>&1 <<'PY'
from pathlib import Path
import json
import sys

bundle_root = Path(sys.argv[1])
pointer_path = Path(sys.argv[2])
manifest_path = Path(sys.argv[3])
expected_pointer = str(bundle_root)
actual_pointer = pointer_path.read_text(errors='replace').strip()
if actual_pointer != expected_pointer:
    raise SystemExit(
        f'latest-proof-bundle pointer drifted: expected {expected_pointer!r}, got {actual_pointer!r}'
    )

required_top_level = [
    'verify-m055-s04.sh',
    'verify-m055-s04-contract.test.mjs',
    'materialize-hyperpush-mono.mjs',
    'repo-identity.json',
    'product-stage-summary.json',
    'product-stage-manifest.json',
    'language-repo.meta.json',
    'product-repo.meta.json',
    'language-proof-bundle.txt',
    'product-proof-bundle.txt',
    'retained-m055-s04-artifacts.manifest.txt',
]
for rel in required_top_level:
    if not (bundle_root / rel).exists():
        raise SystemExit(f'{bundle_root}: missing required retained file {rel}')

language_meta = json.loads((bundle_root / 'language-repo.meta.json').read_text())
product_meta = json.loads((bundle_root / 'product-repo.meta.json').read_text())
if language_meta.get('repoRole') != 'language':
    raise SystemExit('language repo metadata drifted: repoRole must be language')
if product_meta.get('repoRole') != 'product':
    raise SystemExit('product repo metadata drifted: repoRole must be product')
if language_meta.get('slug') != 'hyperpush-org/mesh-lang':
    raise SystemExit(f'language repo metadata drifted: unexpected slug {language_meta.get("slug")!r}')
if product_meta.get('slug') != 'hyperpush-org/hyperpush-mono':
    raise SystemExit(f'product repo metadata drifted: unexpected slug {product_meta.get("slug")!r}')
if language_meta.get('workspaceDir') != 'mesh-lang':
    raise SystemExit('language repo metadata drifted: workspaceDir must be mesh-lang')
if product_meta.get('workspaceDir') != 'hyperpush-mono':
    raise SystemExit('product repo metadata drifted: workspaceDir must be hyperpush-mono')
if not isinstance(language_meta.get('ref'), str) or len(language_meta['ref']) != 40:
    raise SystemExit(f'language repo metadata drifted: expected 40-char git ref, got {language_meta.get("ref")!r}')
if language_meta.get('refSource') != 'git:rev-parse:HEAD':
    raise SystemExit(f'language repo metadata drifted: unexpected refSource {language_meta.get("refSource")!r}')
if not isinstance(product_meta.get('ref'), str) or len(product_meta['ref']) != 40:
    raise SystemExit(f'product repo metadata drifted: expected 40-char git ref, got {product_meta.get("ref")!r}')
if product_meta.get('refSource') != 'git:rev-parse:HEAD':
    raise SystemExit(f'product repo metadata drifted: unexpected refSource {product_meta.get("refSource")!r}')
if not str(product_meta.get('repoRootSource', '')).startswith(('env:M055_HYPERPUSH_ROOT', 'blessed-sibling:')):
    raise SystemExit(f'product repo metadata drifted: unexpected repoRootSource {product_meta.get("repoRootSource")!r}')
if not str(product_meta.get('materializeCheckOutputRoot', '')).endswith('/.tmp/m055-s04/workspace/hyperpush-mono'):
    raise SystemExit('product repo metadata drifted: materializeCheckOutputRoot must point at the staged workspace output')
if not isinstance(product_meta.get('materializeCheckManifestFingerprint'), str) or not product_meta['materializeCheckManifestFingerprint']:
    raise SystemExit('product repo metadata drifted: missing materializeCheckManifestFingerprint')
if product_meta.get('proofBundlePointer') != (bundle_root / 'product-proof-bundle').resolve().as_posix():
    raise SystemExit('product repo metadata drifted: proofBundlePointer mismatch')
if language_meta.get('proofBundlePointer') != (bundle_root / 'language-proof-bundle').resolve().as_posix():
    raise SystemExit('language repo metadata drifted: proofBundlePointer mismatch')

language_pointer = (bundle_root / 'language-proof-bundle.txt').read_text(errors='replace').strip()
product_pointer = (bundle_root / 'product-proof-bundle.txt').read_text(errors='replace').strip()
expected_language_pointer = str((bundle_root / 'language-proof-bundle').resolve())
expected_product_pointer = str((bundle_root / 'product-proof-bundle').resolve())
if language_pointer != expected_language_pointer:
    raise SystemExit(f'language-proof-bundle pointer drifted: expected {expected_language_pointer!r}, got {language_pointer!r}')
if product_pointer != expected_product_pointer:
    raise SystemExit(f'product-proof-bundle pointer drifted: expected {expected_product_pointer!r}, got {product_pointer!r}')

language_bundle = bundle_root / 'language-proof-bundle'
product_bundle = bundle_root / 'product-proof-bundle'
for rel in [
    'retained-m055-s03-verify/status.txt',
    'retained-m055-s03-verify/current-phase.txt',
    'retained-m055-s03-verify/phase-report.txt',
    'retained-m055-s03-verify/full-contract.log',
    'retained-m055-s03-verify/latest-proof-bundle.txt',
    'retained-m055-s03-proof-bundle/verify-m055-s03.sh',
    'retained-m055-s03-proof-bundle/verify-m055-s03-contract.test.mjs',
]:
    if not (language_bundle / rel).exists():
        raise SystemExit(f'{language_bundle}: missing {rel}')
if (language_bundle / 'retained-m055-s03-verify/status.txt').read_text(errors='replace').strip() != 'ok':
    raise SystemExit(f'{language_bundle}/retained-m055-s03-verify/status.txt: expected ok')
if (language_bundle / 'retained-m055-s03-verify/current-phase.txt').read_text(errors='replace').strip() != 'complete':
    raise SystemExit(f'{language_bundle}/retained-m055-s03-verify/current-phase.txt: expected complete')
if (language_bundle / 'retained-m055-s03-verify/latest-proof-bundle.txt').read_text(errors='replace').strip() != str((language_bundle / 'retained-m055-s03-proof-bundle').resolve()):
    raise SystemExit(f'{language_bundle}/retained-m055-s03-verify/latest-proof-bundle.txt drifted')

for rel in [
    'retained-product-m051-s01-verify/status.txt',
    'retained-product-m051-s01-verify/current-phase.txt',
    'retained-product-m051-s01-verify/phase-report.txt',
    'retained-product-m051-s01-verify/full-contract.log',
    'retained-product-m051-s01-verify/latest-proof-bundle.txt',
    'retained-product-m051-s01-proof-bundle/verify-m051-s01.sh',
    'retained-product-m051-s01-proof-bundle/mesher.README.md',
    'retained-product-m051-s01-proof-bundle/mesher.env.example',
    'retained-product-m051-s01-proof-bundle/e2e_m051_s01.rs',
    'retained-product-m051-s01-proof-bundle/retained-m051-s01-artifacts',
    'retained-product-landing-surface-verify/status.txt',
    'retained-product-landing-surface-verify/current-phase.txt',
    'retained-product-landing-surface-verify/phase-report.txt',
    'retained-product-landing-surface-verify/full-contract.log',
]:
    if not (product_bundle / rel).exists():
        raise SystemExit(f'{product_bundle}: missing {rel}')
if (product_bundle / 'retained-product-m051-s01-verify/status.txt').read_text(errors='replace').strip() != 'ok':
    raise SystemExit(f'{product_bundle}/retained-product-m051-s01-verify/status.txt: expected ok')
if (product_bundle / 'retained-product-m051-s01-verify/current-phase.txt').read_text(errors='replace').strip() != 'complete':
    raise SystemExit(f'{product_bundle}/retained-product-m051-s01-verify/current-phase.txt: expected complete')
if (product_bundle / 'retained-product-m051-s01-verify/latest-proof-bundle.txt').read_text(errors='replace').strip() != str((product_bundle / 'retained-product-m051-s01-proof-bundle').resolve()):
    raise SystemExit(f'{product_bundle}/retained-product-m051-s01-verify/latest-proof-bundle.txt drifted')
if (product_bundle / 'retained-product-landing-surface-verify/status.txt').read_text(errors='replace').strip() != 'ok':
    raise SystemExit(f'{product_bundle}/retained-product-landing-surface-verify/status.txt: expected ok')
if (product_bundle / 'retained-product-landing-surface-verify/current-phase.txt').read_text(errors='replace').strip() != 'complete':
    raise SystemExit(f'{product_bundle}/retained-product-landing-surface-verify/current-phase.txt: expected complete')

manifest_lines = [line for line in manifest_path.read_text(errors='replace').splitlines() if line.strip()]
if not manifest_lines:
    raise SystemExit(f'expected non-empty copied-artifact manifest: {manifest_path}')

print('retained-bundle-shape: ok')
PY
  then
    record_phase "$phase" failed
    fail_phase "$phase" "retained proof bundle pointer or copied child bundle shape drifted" "$log_path" "$bundle_root"
  fi
}

rm -rf "$ARTIFACT_DIR"
mkdir -p "$ARTIFACT_DIR"
exec > >(tee "$FULL_LOG_PATH") 2>&1

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

record_phase init started
for command_name in node python3 bash rg git; do
  require_command init "$command_name" "required command for the M055 S04 assembled replay"
done
for path in \
  "$ROOT_DIR/scripts/materialize-hyperpush-mono.mjs" \
  "$ROOT_DIR/scripts/verify-m055-s03.sh" \
  "$ROOT_DIR/scripts/tests/verify-m055-s04-contract.test.mjs" \
  "$ROOT_DIR/scripts/lib/repo-identity.json"; do
  require_file init "$path" "required M055 S04 surface"
done
if ! m055_resolve_hyperpush_root "$ROOT_DIR" >"$ARTIFACT_DIR/init.product-root.path"; then
  fail_phase init "missing sibling product repo root or stale in-repo mesher path" "$ARTIFACT_DIR/init.product-root.path"
fi
REAL_PRODUCT_ROOT="$M055_HYPERPUSH_ROOT_RESOLVED"
PRODUCT_M051_VERIFY_DIR="$REAL_PRODUCT_ROOT/.tmp/m051-s01/verify"
PRODUCT_LANDING_VERIFY_DIR="$REAL_PRODUCT_ROOT/.tmp/m055-s04/landing-surface/verify"
require_git_tracked init "$REAL_PRODUCT_ROOT" 'scripts/verify-m051-s01.sh' 'tracked product-root wrapper in sibling repo' "$REAL_PRODUCT_ROOT"
require_git_tracked init "$REAL_PRODUCT_ROOT" '.github/workflows/deploy-landing.yml' 'tracked landing deploy workflow in sibling repo' "$REAL_PRODUCT_ROOT"
require_git_tracked init "$REAL_PRODUCT_ROOT" 'scripts/verify-landing-surface.sh' 'tracked landing surface verifier in sibling repo' "$REAL_PRODUCT_ROOT"
require_git_tracked init "$REAL_PRODUCT_ROOT" 'mesher/scripts/verify-maintainer-surface.sh' 'tracked Mesher maintainer verifier in sibling repo' "$REAL_PRODUCT_ROOT"
record_phase init passed

run_expect_success materialize-hyperpush materialize-hyperpush 300 "$WORKSPACE_ROOT" \
  node scripts/materialize-hyperpush-mono.mjs --check
require_file materialize-hyperpush-check "$STAGED_PRODUCT_SUMMARY_PATH" "staged product summary" "$WORKSPACE_ROOT"
require_file materialize-hyperpush-check "$STAGED_PRODUCT_MANIFEST_PATH" "staged product manifest" "$WORKSPACE_ROOT"
require_file materialize-hyperpush-check "$STAGED_PRODUCT_ROOT/scripts/verify-m051-s01.sh" "staged product maintainer wrapper" "$STAGED_PRODUCT_ROOT"
require_file materialize-hyperpush-check "$STAGED_PRODUCT_ROOT/scripts/verify-landing-surface.sh" "staged landing verifier" "$STAGED_PRODUCT_ROOT"

run_expect_success product-m051-wrapper product-m051-wrapper 7200 "$PRODUCT_M051_VERIFY_DIR" \
  bash -c 'cd "$1" && bash scripts/verify-m051-s01.sh' _ "$REAL_PRODUCT_ROOT"
run_expect_success product-landing-wrapper product-landing-wrapper 300 "$PRODUCT_LANDING_VERIFY_DIR" \
  bash -c 'cd "$1" && bash scripts/verify-landing-surface.sh' _ "$REAL_PRODUCT_ROOT"
run_expect_success language-m055-s03-wrapper language-m055-s03-wrapper 7200 "$LANGUAGE_VERIFY_DIR" \
  bash -c 'cd "$1" && M055_HYPERPUSH_ROOT="$2" bash scripts/verify-m055-s03.sh' _ "$ROOT_DIR" "$REAL_PRODUCT_ROOT"

rm -rf "$RETAINED_PROOF_BUNDLE_DIR"
mkdir -p "$RETAINED_PROOF_BUNDLE_DIR/language-proof-bundle" "$RETAINED_PROOF_BUNDLE_DIR/product-proof-bundle"

begin_phase retain-language-m055-s03-verify
copy_fixed_dir_or_fail retain-language-m055-s03-verify \
  "$LANGUAGE_VERIFY_DIR" \
  "$RETAINED_PROOF_BUNDLE_DIR/language-proof-bundle/retained-m055-s03-verify" \
  "language-owned M055 S03 verify artifacts are missing or malformed" \
  status.txt \
  current-phase.txt \
  phase-report.txt \
  full-contract.log \
  latest-proof-bundle.txt
record_phase retain-language-m055-s03-verify passed

begin_phase retain-language-m055-s03-proof-bundle
copy_pointed_bundle_or_fail retain-language-m055-s03-proof-bundle \
  "$ROOT_DIR" \
  "$LANGUAGE_VERIFY_DIR" \
  "$RETAINED_PROOF_BUNDLE_DIR/language-proof-bundle/retained-m055-s03-verify/latest-proof-bundle.txt" \
  "$RETAINED_PROOF_BUNDLE_DIR/language-proof-bundle/retained-m055-s03-proof-bundle" \
  "language-owned M055 S03 proof bundle is missing or malformed" \
  verify-m055-s03.sh \
  verify-m055-s03-contract.test.mjs
record_phase retain-language-m055-s03-proof-bundle passed

begin_phase retain-product-m051-s01-verify
copy_fixed_dir_or_fail retain-product-m051-s01-verify \
  "$PRODUCT_M051_VERIFY_DIR" \
  "$RETAINED_PROOF_BUNDLE_DIR/product-proof-bundle/retained-product-m051-s01-verify" \
  "product-owned M051 S01 verify artifacts are missing or malformed" \
  status.txt \
  current-phase.txt \
  phase-report.txt \
  full-contract.log \
  latest-proof-bundle.txt
record_phase retain-product-m051-s01-verify passed

begin_phase retain-product-m051-s01-proof-bundle
copy_pointed_bundle_or_fail retain-product-m051-s01-proof-bundle \
  "$REAL_PRODUCT_ROOT" \
  "$PRODUCT_M051_VERIFY_DIR" \
  "$RETAINED_PROOF_BUNDLE_DIR/product-proof-bundle/retained-product-m051-s01-verify/latest-proof-bundle.txt" \
  "$RETAINED_PROOF_BUNDLE_DIR/product-proof-bundle/retained-product-m051-s01-proof-bundle" \
  "product-owned M051 S01 proof bundle is missing or malformed" \
  verify-m051-s01.sh \
  mesher.README.md \
  mesher.env.example \
  e2e_m051_s01.rs \
  retained-m051-s01-artifacts
record_phase retain-product-m051-s01-proof-bundle passed

begin_phase retain-product-landing-surface-verify
copy_fixed_dir_or_fail retain-product-landing-surface-verify \
  "$PRODUCT_LANDING_VERIFY_DIR" \
  "$RETAINED_PROOF_BUNDLE_DIR/product-proof-bundle/retained-product-landing-surface-verify" \
  "product-owned landing verify artifacts are missing or malformed" \
  status.txt \
  current-phase.txt \
  phase-report.txt \
  full-contract.log
record_phase retain-product-landing-surface-verify passed

begin_phase repo-metadata
printf '%s\n' "$(realpath "$RETAINED_PROOF_BUNDLE_DIR/language-proof-bundle")" >"$LANGUAGE_PROOF_BUNDLE_POINTER_PATH"
printf '%s\n' "$(realpath "$RETAINED_PROOF_BUNDLE_DIR/product-proof-bundle")" >"$PRODUCT_PROOF_BUNDLE_POINTER_PATH"
capture_repo_metadata_or_fail repo-metadata \
  "$LANGUAGE_REPO_METADATA_PATH" \
  "$PRODUCT_REPO_METADATA_PATH" \
  "$LANGUAGE_PROOF_BUNDLE_POINTER_PATH" \
  "$PRODUCT_PROOF_BUNDLE_POINTER_PATH"
cp "$LANGUAGE_REPO_METADATA_PATH" "$RETAINED_PROOF_BUNDLE_DIR/language-repo.meta.json"
cp "$PRODUCT_REPO_METADATA_PATH" "$RETAINED_PROOF_BUNDLE_DIR/product-repo.meta.json"
cp "$STAGED_PRODUCT_SUMMARY_PATH" "$RETAINED_PROOF_BUNDLE_DIR/product-stage-summary.json"
cp "$STAGED_PRODUCT_MANIFEST_PATH" "$RETAINED_PROOF_BUNDLE_DIR/product-stage-manifest.json"
cp "$LANGUAGE_PROOF_BUNDLE_POINTER_PATH" "$RETAINED_PROOF_BUNDLE_DIR/language-proof-bundle.txt"
cp "$PRODUCT_PROOF_BUNDLE_POINTER_PATH" "$RETAINED_PROOF_BUNDLE_DIR/product-proof-bundle.txt"
record_phase repo-metadata passed

begin_phase m055-s04-bundle-shape
cp "$ROOT_DIR/scripts/verify-m055-s04.sh" "$RETAINED_PROOF_BUNDLE_DIR/verify-m055-s04.sh"
cp "$ROOT_DIR/scripts/tests/verify-m055-s04-contract.test.mjs" "$RETAINED_PROOF_BUNDLE_DIR/verify-m055-s04-contract.test.mjs"
cp "$ROOT_DIR/scripts/materialize-hyperpush-mono.mjs" "$RETAINED_PROOF_BUNDLE_DIR/materialize-hyperpush-mono.mjs"
cp "$ROOT_DIR/scripts/lib/repo-identity.json" "$RETAINED_PROOF_BUNDLE_DIR/repo-identity.json"
capture_copied_manifest_or_fail m055-s04-bundle-shape \
  "$RETAINED_ARTIFACTS_MANIFEST_PATH" \
  "$RETAINED_PROOF_BUNDLE_DIR/language-proof-bundle" \
  "$RETAINED_PROOF_BUNDLE_DIR/product-proof-bundle" \
  "$RETAINED_PROOF_BUNDLE_DIR/language-repo.meta.json" \
  "$RETAINED_PROOF_BUNDLE_DIR/product-repo.meta.json" \
  "$RETAINED_PROOF_BUNDLE_DIR/product-stage-summary.json" \
  "$RETAINED_PROOF_BUNDLE_DIR/product-stage-manifest.json"
cp "$RETAINED_ARTIFACTS_MANIFEST_PATH" "$RETAINED_PROOF_BUNDLE_DIR/retained-m055-s04-artifacts.manifest.txt"
printf '%s\n' "$RETAINED_PROOF_BUNDLE_DIR" >"$LATEST_PROOF_BUNDLE_PATH"
assert_retained_bundle_shape \
  m055-s04-bundle-shape \
  "$RETAINED_PROOF_BUNDLE_DIR" \
  "$LATEST_PROOF_BUNDLE_PATH" \
  "$RETAINED_ARTIFACTS_MANIFEST_PATH"
record_phase m055-s04-bundle-shape passed

for expected_phase in \
  init \
  materialize-hyperpush \
  product-m051-wrapper \
  product-landing-wrapper \
  language-m055-s03-wrapper \
  retain-language-m055-s03-verify \
  retain-language-m055-s03-proof-bundle \
  retain-product-m051-s01-verify \
  retain-product-m051-s01-proof-bundle \
  retain-product-landing-surface-verify \
  repo-metadata \
  m055-s04-bundle-shape; do
  if ! rg -q "^${expected_phase}\\tpassed$" "$PHASE_REPORT_PATH"; then
    fail_phase verifier-status "phase report missing passed marker for ${expected_phase}" "$PHASE_REPORT_PATH"
  fi
done

echo "verify-m055-s04: ok"
echo "artifacts: $(repo_rel "$ARTIFACT_DIR")"
echo "proof bundle: $(repo_rel "$RETAINED_PROOF_BUNDLE_DIR")"
