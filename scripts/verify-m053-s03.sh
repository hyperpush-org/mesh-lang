#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

# shellcheck source=scripts/lib/m055-workspace.sh
source "$ROOT_DIR/scripts/lib/m055-workspace.sh"

VERIFY_ROOT="${M053_S03_VERIFY_ROOT:-$ROOT_DIR/.tmp/m053-s03/verify}"
STATUS_PATH="$VERIFY_ROOT/status.txt"
CURRENT_PHASE_PATH="$VERIFY_ROOT/current-phase.txt"
PHASE_REPORT_PATH="$VERIFY_ROOT/phase-report.txt"
CANDIDATE_REFS_PATH="$VERIFY_ROOT/candidate-refs.json"
REMOTE_RUNS_PATH="$VERIFY_ROOT/remote-runs.json"
FULL_LOG_PATH="$VERIFY_ROOT/full-contract.log"
GH_REPO_OVERRIDE="${M053_S03_GH_REPO:-}"
GH_BIN="${M053_S03_GH_BIN:-gh}"
GIT_BIN="${M053_S03_GIT_BIN:-git}"
GH_REPO=""
GH_REPO_SOURCE=""
LAST_LOG_PATH=""

resolve_repo_slug() {
  if [[ -n "$GH_REPO_OVERRIDE" ]]; then
    GH_REPO="$GH_REPO_OVERRIDE"
    GH_REPO_SOURCE='env:M053_S03_GH_REPO'
    return 0
  fi

  if ! m055_resolve_language_repo_slug "$ROOT_DIR" >/dev/null; then
    return 1
  fi
  GH_REPO="$M055_LANGUAGE_REPO_SLUG_RESOLVED"
  GH_REPO_SOURCE="$M055_LANGUAGE_REPO_SLUG_SOURCE"
}

prepare_verify_root() {
  rm -rf "$VERIFY_ROOT"
  mkdir -p "$VERIFY_ROOT"
  : >"$PHASE_REPORT_PATH"
  printf 'running\n' >"$STATUS_PATH"
  printf 'bootstrap\n' >"$CURRENT_PHASE_PATH"
  exec > >(tee "$FULL_LOG_PATH") 2>&1
}

record_phase() {
  local phase_name="$1"
  local status="$2"
  printf '%s\t%s\n' "$phase_name" "$status" >>"$PHASE_REPORT_PATH"
}

start_phase() {
  local phase_name="$1"
  local detail="$2"
  printf '%s\n' "$phase_name" >"$CURRENT_PHASE_PATH"
  record_phase "$phase_name" started
  echo "==> [${phase_name}] ${detail}"
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
  local phase_name="$1"
  local reason="$2"
  local log_path="${3:-}"
  local artifact_hint="${4:-}"

  printf 'failed\n' >"$STATUS_PATH"
  printf '%s\n' "$phase_name" >"$CURRENT_PHASE_PATH"
  record_phase "$phase_name" failed

  echo "verification drift: ${reason}" >&2
  echo "first failing phase: ${phase_name}" >&2
  echo "artifacts: ${VERIFY_ROOT#$ROOT_DIR/}" >&2
  if [[ -n "$artifact_hint" ]]; then
    echo "artifact hint: ${artifact_hint#$ROOT_DIR/}" >&2
  fi
  if [[ -n "$log_path" ]]; then
    echo "failing log: ${log_path#$ROOT_DIR/}" >&2
    echo "--- ${log_path#$ROOT_DIR/} ---" >&2
    print_log_excerpt "$log_path" >&2
  fi
  exit 1
}

finish_phase() {
  local phase_name="$1"
  record_phase "$phase_name" passed
}

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

run_preflight() {
  local phase_name="gh-preflight"
  local log_path="$VERIFY_ROOT/${phase_name}.log"

  start_phase "$phase_name" "require GH_TOKEN plus gh/git/python3 before remote hosted checks"
  : >"$log_path"
  if ! resolve_repo_slug >>"$log_path" 2>&1; then
    LAST_LOG_PATH="$log_path"
    fail_phase "$phase_name" "missing GH_TOKEN, repo slug, or required executables" "$log_path" "$VERIFY_ROOT"
  fi

  if ! python3 - "$log_path" "$GH_BIN" "$GIT_BIN" "$GH_REPO" "$GH_REPO_SOURCE" <<'PY'
from pathlib import Path
import os
import shutil
import sys

log_path = Path(sys.argv[1])
gh_bin = sys.argv[2]
git_bin = sys.argv[3]
repo_slug = sys.argv[4]
repo_source = sys.argv[5]
messages = []
errors = []

messages.append(f'ok: repository -> {repo_slug} (source={repo_source})')

if not os.environ.get('GH_TOKEN'):
    errors.append('GH_TOKEN must be set for hosted GitHub workflow queries')
else:
    messages.append('ok: GH_TOKEN is present')

for label, candidate in [('gh', gh_bin), ('git', git_bin), ('python3', 'python3')]:
    if os.path.sep in candidate:
        resolved = candidate if os.path.exists(candidate) and os.access(candidate, os.X_OK) else None
    else:
        resolved = shutil.which(candidate)
    if resolved:
        messages.append(f'ok: {label} -> {resolved}')
    else:
        errors.append(f'missing required executable for {label}: {candidate}')

log_path.write_text('\n'.join(messages + ([''] if messages and errors else []) + errors) + ('\n' if messages or errors else ''))
if errors:
    raise SystemExit(1)
PY
  then
    LAST_LOG_PATH="$log_path"
    fail_phase "$phase_name" "missing GH_TOKEN, repo slug, or required executables" "$log_path" "$VERIFY_ROOT"
  fi

  LAST_LOG_PATH="$log_path"
  finish_phase "$phase_name"
}

run_candidate_refs() {
  local phase_name="candidate-refs"
  local log_path="$VERIFY_ROOT/${phase_name}.log"

  start_phase "$phase_name" "derive fresh main and binary-tag refs for hosted workflow evidence"
  if ! python3 - "$ROOT_DIR" "$CANDIDATE_REFS_PATH" "$GH_REPO" "$GH_REPO_SOURCE" >"$log_path" 2>&1 <<'PY'
from pathlib import Path
from datetime import datetime, timezone
import json
import re
import sys

root = Path(sys.argv[1])
output_path = Path(sys.argv[2])
repo_slug = sys.argv[3]
repo_source = sys.argv[4]
errors = []

required_paths = [
    root / 'compiler/meshc/Cargo.toml',
    root / 'compiler/meshpkg/Cargo.toml',
    root / '.github/workflows/authoritative-verification.yml',
    root / '.github/workflows/deploy-services.yml',
    root / '.github/workflows/release.yml',
]
for candidate in required_paths:
    if not candidate.exists():
        errors.append(f'missing required input {candidate.relative_to(root)}')

version_pattern = re.compile(r'^[0-9]+\.[0-9]+\.[0-9]+(?:[-+][0-9A-Za-z.-]+)?$')

def cargo_version(path: Path):
    if not path.exists():
        return None
    match = re.search(r'^version = "([^"]+)"', path.read_text(), re.MULTILINE)
    if not match:
        errors.append(f'missing version in {path.relative_to(root)}')
        return None
    return match.group(1)

meshc_version = cargo_version(root / 'compiler/meshc/Cargo.toml')
meshpkg_version = cargo_version(root / 'compiler/meshpkg/Cargo.toml')

for label, value in [('meshc', meshc_version), ('meshpkg', meshpkg_version)]:
    if value and not version_pattern.fullmatch(value):
        errors.append(f'{label} version is malformed: {value!r}')

if meshc_version and meshpkg_version and meshc_version != meshpkg_version:
    errors.append(
        f'compiler/meshc and compiler/meshpkg versions diverged ({meshc_version!r} vs {meshpkg_version!r})'
    )

binary_tag = f'v{meshc_version}' if meshc_version else None
workflows = [
    {
        'workflowFile': 'authoritative-verification.yml',
        'requiredEvent': 'push',
        'requiredHeadBranch': 'main',
        'expectedRef': 'refs/heads/main',
        'expectedPeeledRef': None,
        'requiredJobs': ['Hosted starter failover proof'],
        'jobAliases': {
            'Hosted starter failover proof': ['Authoritative starter failover proof'],
        },
        'requiredSteps': {},
    },
    {
        'workflowFile': 'deploy-services.yml',
        'requiredEvent': 'push',
        'requiredHeadBranch': 'main',
        'expectedRef': 'refs/heads/main',
        'expectedPeeledRef': None,
        'requiredJobs': ['Deploy mesh-registry', 'Deploy mesh-packages website', 'Post-deploy health checks'],
        'jobAliases': {
            'Deploy mesh-registry': ['Deploy mesh-registry'],
            'Deploy mesh-packages website': ['Deploy mesh-packages website'],
            'Post-deploy health checks': ['Post-deploy health checks'],
        },
        'requiredSteps': {
            'Post-deploy health checks': ['Verify public surface contract'],
        },
        'forbiddenJobs': ['Deploy hyperpush landing'],
        'forbiddenSteps': {
            'Post-deploy health checks': ['Verify hyperpush landing'],
        },
    },
    {
        'workflowFile': 'release.yml',
        'requiredEvent': 'push',
        'requiredHeadBranch': binary_tag,
        'expectedRef': f'refs/tags/{binary_tag}' if binary_tag else None,
        'expectedPeeledRef': f'refs/tags/{binary_tag}^{{}}' if binary_tag else None,
        'requiredJobs': ['Hosted starter failover proof', 'Create Release'],
        'jobAliases': {
            'Hosted starter failover proof': ['Authoritative starter failover proof'],
            'Create Release': ['Create Release'],
        },
        'requiredSteps': {},
    },
]

if not binary_tag:
    errors.append('could not derive the current binary tag from compiler/meshc/Cargo.toml')

for workflow in workflows:
    if not workflow['requiredHeadBranch']:
        errors.append(f"{workflow['workflowFile']} is missing requiredHeadBranch")
    if not workflow['expectedRef']:
        errors.append(f"{workflow['workflowFile']} is missing expectedRef")

artifact = {
    'generatedAt': datetime.now(timezone.utc).isoformat(),
    'repository': repo_slug,
    'repositorySource': repo_source,
    'meshcVersion': meshc_version,
    'meshpkgVersion': meshpkg_version,
    'binaryTag': binary_tag,
    'workflows': workflows,
}
output_path.write_text(json.dumps(artifact, indent=2) + '\n')

print(f'repository: {repo_slug} (source={repo_source})')
print(f'meshc version: {meshc_version}')
print(f'meshpkg version: {meshpkg_version}')
print(f'binary tag: {binary_tag}')
print(f'artifact: {output_path.relative_to(root)}')

if errors:
    print()
    print('errors:')
    for error in errors:
        print(f'- {error}')
    raise SystemExit(1)
PY
  then
    LAST_LOG_PATH="$log_path"
    fail_phase "$phase_name" "candidate ref derivation drifted" "$log_path" "$CANDIDATE_REFS_PATH"
  fi

  LAST_LOG_PATH="$log_path"
  finish_phase "$phase_name"
}

run_remote_evidence() {
  local phase_name="remote-evidence"
  local log_path="$VERIFY_ROOT/${phase_name}.log"

  start_phase "$phase_name" "query fresh hosted runs for authoritative, deploy-services, and release workflows"
  if ! python3 - "$ROOT_DIR" "$CANDIDATE_REFS_PATH" "$REMOTE_RUNS_PATH" "$VERIFY_ROOT" "$GH_REPO" "$GH_REPO_SOURCE" "$GH_BIN" "$GIT_BIN" >"$log_path" 2>&1 <<'PY'
from pathlib import Path
from datetime import datetime, timezone
import json
import os
import shlex
import subprocess
import sys

root = Path(sys.argv[1])
candidate_refs_path = Path(sys.argv[2])
output_path = Path(sys.argv[3])
verify_root = Path(sys.argv[4])
repo = sys.argv[5]
repo_source = sys.argv[6]
gh_bin = sys.argv[7]
git_bin = sys.argv[8]
candidate_refs = json.loads(candidate_refs_path.read_text())
workflow_specs = candidate_refs['workflows']


def shell_join(command):
    return ' '.join(shlex.quote(part) for part in command)


def relative(path: Path) -> str:
    try:
        return path.relative_to(root).as_posix()
    except ValueError:
        return path.as_posix()


def slug_for_workflow(workflow_file: str) -> str:
    return workflow_file.replace('.yml', '').replace('.', '-')


def write_command_log(command, stdout_path: Path, stderr_path: Path, log_path: Path):
    with log_path.open('w', encoding='utf-8') as handle:
        handle.write(f'command: {shell_join(command)}\n')
        handle.write(f'cwd: {root.as_posix()}\n')
        if stdout_path.read_text():
            handle.write('\n[stdout]\n')
            handle.write(stdout_path.read_text())
        if stderr_path.read_text():
            handle.write('\n[stderr]\n')
            handle.write(stderr_path.read_text())


def run_text_command(command, slug: str, suffix: str, timeout_seconds: int = 45):
    stdout_path = verify_root / f'{slug}-{suffix}.stdout'
    stderr_path = verify_root / f'{slug}-{suffix}.stderr'
    log_path = verify_root / f'{slug}-{suffix}.log'
    stdout_text = ''
    stderr_text = ''
    exit_code = 0
    timed_out = False

    try:
        completed = subprocess.run(
            command,
            cwd=root,
            env=os.environ.copy(),
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            timeout=timeout_seconds,
            check=False,
        )
        stdout_text = completed.stdout
        stderr_text = completed.stderr
        exit_code = completed.returncode
    except subprocess.TimeoutExpired as exc:
        stdout_text = exc.stdout or ''
        stderr_text = (exc.stderr or '') + f'\n[verify-timeout] command exceeded {timeout_seconds} seconds\n'
        exit_code = 124
        timed_out = True

    stdout_path.write_text(stdout_text)
    stderr_path.write_text(stderr_text)
    write_command_log(command, stdout_path, stderr_path, log_path)

    return {
        'command': shell_join(command),
        'exitCode': exit_code,
        'timedOut': timed_out,
        'stdoutPath': relative(stdout_path),
        'stderrPath': relative(stderr_path),
        'logPath': relative(log_path),
        'stdout': stdout_text,
        'stderr': stderr_text,
    }


def run_json_command(command, slug: str, suffix: str, timeout_seconds: int = 45):
    result = run_text_command(command, slug, suffix, timeout_seconds=timeout_seconds)
    parsed = None
    parse_error = None
    if result['stdout'].strip():
        try:
            parsed = json.loads(result['stdout'])
        except json.JSONDecodeError as exc:
            parse_error = str(exc)
    result['parsed'] = parsed
    result['parseError'] = parse_error
    return result


def record_query_result(result):
    payload = {
        'command': result['command'],
        'exitCode': result['exitCode'],
        'timedOut': result['timedOut'],
        'stdoutPath': result['stdoutPath'],
        'stderrPath': result['stderrPath'],
        'logPath': result['logPath'],
    }
    if result.get('parseError'):
        payload['parseError'] = result['parseError']
    return payload


def job_name_matches(actual_name, expected_name):
    if not isinstance(actual_name, str):
        return False
    return (
        actual_name == expected_name
        or actual_name.startswith(f'{expected_name} / ')
        or actual_name.endswith(f' / {expected_name}')
    )


def find_matching_job(jobs, aliases):
    for alias in aliases:
        for job in jobs:
            if isinstance(job, dict) and job_name_matches(job.get('name'), alias):
                return job, alias
    return None, None


def resolve_expected_ref(entry, spec, slug):
    ref_candidates = [spec['expectedRef']]
    expected_peeled_ref = spec.get('expectedPeeledRef')
    if expected_peeled_ref:
        ref_candidates.append(expected_peeled_ref)

    command = [git_bin, 'ls-remote', '--quiet', 'origin', *ref_candidates]
    result = run_text_command(command, slug, 'expected-ref', timeout_seconds=20)
    entry['expectedRefQuery'] = record_query_result(result)
    entry['expectedRefCandidates'] = ref_candidates

    if result['exitCode'] != 0:
        if result['timedOut']:
            return None, None, f"{spec['workflowFile']} expected ref resolution timed out for {spec['expectedRef']!r}"
        return None, None, f"{spec['workflowFile']} expected ref resolution failed for {spec['expectedRef']!r}"

    parsed_lines = []
    malformed_lines = []
    for raw_line in result['stdout'].splitlines():
        line = raw_line.strip()
        if not line:
            continue
        parts = line.split()
        if len(parts) != 2:
            malformed_lines.append(raw_line)
            continue
        sha, ref_name = parts
        parsed_lines.append((sha, ref_name))

    if malformed_lines:
        return None, None, f"{spec['workflowFile']} expected ref resolution returned malformed git ls-remote output"

    ref_map = {}
    duplicate_refs = []
    for sha, ref_name in parsed_lines:
        if ref_name in ref_map and ref_map[ref_name] != sha:
            duplicate_refs.append(ref_name)
            continue
        ref_map[ref_name] = sha

    if duplicate_refs:
        return None, None, f"{spec['workflowFile']} expected ref resolution returned ambiguous refs: {sorted(set(duplicate_refs))}"

    if expected_peeled_ref:
        raw_sha = ref_map.get(spec['expectedRef'])
        peeled_sha = ref_map.get(expected_peeled_ref)
        if not raw_sha or not peeled_sha:
            return None, None, f"{spec['workflowFile']} expected ref resolution missing peeled tag data for {spec['expectedRef']!r}"
        return expected_peeled_ref, peeled_sha, None

    expected_head_sha = ref_map.get(spec['expectedRef'])
    if not expected_head_sha:
        return None, None, f"{spec['workflowFile']} could not resolve expected remote ref {spec['expectedRef']!r}"
    return spec['expectedRef'], expected_head_sha, None


results = []
errors = []

for spec in workflow_specs:
    slug = slug_for_workflow(spec['workflowFile'])
    entry = {
        'workflowFile': spec['workflowFile'],
        'repository': repo,
        'requiredEvent': spec['requiredEvent'],
        'requiredHeadBranch': spec['requiredHeadBranch'],
        'expectedRef': spec['expectedRef'],
        'expectedPeeledRef': spec.get('expectedPeeledRef'),
        'expectedResolvedRef': None,
        'expectedHeadSha': None,
        'observedHeadSha': None,
        'headShaMatchesExpected': None,
        'freshnessStatus': 'pending',
        'freshnessFailure': None,
        'requiredJobs': spec.get('requiredJobs', []),
        'requiredSteps': spec.get('requiredSteps', {}),
        'forbiddenJobs': spec.get('forbiddenJobs', []),
        'forbiddenSteps': spec.get('forbiddenSteps', {}),
        'status': 'pending',
    }

    resolved_ref, expected_head_sha, expected_ref_error = resolve_expected_ref(entry, spec, slug)
    if expected_ref_error:
        entry['status'] = 'failed'
        entry['failure'] = expected_ref_error
        entry['freshnessStatus'] = 'failed'
        entry['freshnessFailure'] = expected_ref_error
        entry['headShaMatchesExpected'] = False
        results.append(entry)
        errors.append(expected_ref_error)
        continue

    entry['expectedResolvedRef'] = resolved_ref
    entry['expectedHeadSha'] = expected_head_sha

    list_command = [
        gh_bin, 'run', 'list',
        '-R', repo,
        '--workflow', spec['workflowFile'],
        '--event', spec['requiredEvent'],
        '--branch', spec['requiredHeadBranch'],
        '--limit', '1',
        '--json', 'databaseId,workflowName,event,status,conclusion,headBranch,headSha,displayTitle,createdAt,url',
    ]
    list_result = run_json_command(list_command, slug, 'list')
    entry['listQuery'] = record_query_result(list_result)

    if list_result['exitCode'] != 0:
        stderr_text = list_result['stderr']
        if list_result['timedOut']:
            reason = f"{spec['workflowFile']} gh run list timed out"
        elif (
            'HTTP 404' in stderr_text
            or ('workflow' in stderr_text.lower() and 'not found' in stderr_text.lower())
            or 'could not find any workflows' in stderr_text.lower()
        ):
            reason = f"{spec['workflowFile']} is missing on the remote default branch"
        else:
            reason = f"{spec['workflowFile']} gh run list failed"
        entry['status'] = 'failed'
        entry['failure'] = reason
        entry['freshnessStatus'] = 'failed'
        entry['freshnessFailure'] = reason
        entry['headShaMatchesExpected'] = False
        results.append(entry)
        errors.append(reason)
        continue

    if list_result['parseError']:
        reason = f"{spec['workflowFile']} gh run list output was not valid JSON: {list_result['parseError']}"
        entry['status'] = 'failed'
        entry['failure'] = reason
        entry['freshnessStatus'] = 'failed'
        entry['freshnessFailure'] = reason
        entry['headShaMatchesExpected'] = False
        results.append(entry)
        errors.append(reason)
        continue

    runs = list_result['parsed']
    if not isinstance(runs, list) or not runs:
        fallback_command = [
            gh_bin, 'run', 'list',
            '-R', repo,
            '--workflow', spec['workflowFile'],
            '--limit', '1',
            '--json', 'databaseId,workflowName,event,status,conclusion,headBranch,headSha,displayTitle,createdAt,url',
        ]
        fallback_result = run_json_command(fallback_command, slug, 'latest-available')
        entry['latestAvailableQuery'] = record_query_result(fallback_result)
        latest_available = None
        if fallback_result['exitCode'] == 0 and not fallback_result['parseError'] and isinstance(fallback_result['parsed'], list) and fallback_result['parsed']:
            latest_available = fallback_result['parsed'][0]
            entry['latestAvailableRun'] = latest_available

        if latest_available:
            latest_branch = latest_available.get('headBranch')
            latest_sha = latest_available.get('headSha')
            reason = (
                f"{spec['workflowFile']} has no hosted run for event {spec['requiredEvent']!r} on "
                f"{spec['requiredHeadBranch']!r}; latest available was {latest_branch!r} @ {latest_sha!r}"
            )
        else:
            reason = (
                f"{spec['workflowFile']} has no hosted run for event {spec['requiredEvent']!r} "
                f"on {spec['requiredHeadBranch']!r}"
            )
        entry['status'] = 'failed'
        entry['failure'] = reason
        entry['freshnessStatus'] = 'failed'
        entry['freshnessFailure'] = reason
        entry['headShaMatchesExpected'] = False
        results.append(entry)
        errors.append(reason)
        continue

    run_summary = runs[0]
    entry['runSummary'] = run_summary
    entry['observedHeadSha'] = run_summary.get('headSha')

    if not isinstance(run_summary, dict) or 'databaseId' not in run_summary:
        reason = f"{spec['workflowFile']} gh run list output omitted databaseId"
        entry['status'] = 'failed'
        entry['failure'] = reason
        entry['freshnessStatus'] = 'failed'
        entry['freshnessFailure'] = reason
        entry['headShaMatchesExpected'] = False
        results.append(entry)
        errors.append(reason)
        continue

    view_command = [
        gh_bin, 'run', 'view', str(run_summary['databaseId']),
        '-R', repo,
        '--json', 'databaseId,workflowName,event,status,conclusion,headBranch,headSha,displayTitle,url,jobs',
    ]
    view_result = run_json_command(view_command, slug, 'view', timeout_seconds=60)
    entry['viewQuery'] = record_query_result(view_result)

    if view_result['exitCode'] != 0:
        if view_result['timedOut']:
            reason = f"{spec['workflowFile']} gh run view timed out for run {run_summary['databaseId']}"
        else:
            reason = f"{spec['workflowFile']} gh run view failed for run {run_summary['databaseId']}"
        entry['status'] = 'failed'
        entry['failure'] = reason
        entry['freshnessStatus'] = 'failed'
        entry['freshnessFailure'] = reason
        entry['headShaMatchesExpected'] = False
        results.append(entry)
        errors.append(reason)
        continue

    if view_result['parseError']:
        reason = f"{spec['workflowFile']} gh run view output was not valid JSON: {view_result['parseError']}"
        entry['status'] = 'failed'
        entry['failure'] = reason
        entry['freshnessStatus'] = 'failed'
        entry['freshnessFailure'] = reason
        entry['headShaMatchesExpected'] = False
        results.append(entry)
        errors.append(reason)
        continue

    run_view = view_result['parsed']
    jobs = run_view.get('jobs') if isinstance(run_view, dict) else None
    if not isinstance(run_view, dict) or not isinstance(jobs, list):
        reason = f"{spec['workflowFile']} gh run view did not include the jobs payload"
        entry['status'] = 'failed'
        entry['failure'] = reason
        entry['freshnessStatus'] = 'failed'
        entry['freshnessFailure'] = reason
        entry['headShaMatchesExpected'] = False
        results.append(entry)
        errors.append(reason)
        continue

    entry['runView'] = run_view
    observed_head_sha = run_view.get('headSha') or run_summary.get('headSha')
    entry['observedHeadSha'] = observed_head_sha
    if not observed_head_sha:
        reason = f"{spec['workflowFile']} hosted run omitted headSha for {spec['requiredHeadBranch']!r}"
        entry['status'] = 'failed'
        entry['failure'] = reason
        entry['freshnessStatus'] = 'failed'
        entry['freshnessFailure'] = reason
        entry['headShaMatchesExpected'] = False
        results.append(entry)
        errors.append(reason)
        continue

    if observed_head_sha != expected_head_sha:
        reason = (
            f"{spec['workflowFile']} hosted run headSha {observed_head_sha!r} "
            f"did not match expected {resolved_ref!r} sha {expected_head_sha!r}"
        )
        entry['status'] = 'failed'
        entry['failure'] = reason
        entry['freshnessStatus'] = 'failed'
        entry['freshnessFailure'] = reason
        entry['headShaMatchesExpected'] = False
        results.append(entry)
        errors.append(reason)
        continue

    entry['headShaMatchesExpected'] = True
    entry['freshnessStatus'] = 'ok'
    entry['freshnessFailure'] = None

    if run_summary.get('event') != spec['requiredEvent']:
        reason = (
            f"{spec['workflowFile']} latest hosted run event {run_summary.get('event')!r} "
            f"did not match required {spec['requiredEvent']!r}"
        )
        entry['status'] = 'failed'
        entry['failure'] = reason
        entry['freshnessStatus'] = 'failed'
        entry['freshnessFailure'] = reason
        entry['headShaMatchesExpected'] = False
        results.append(entry)
        errors.append(reason)
        continue

    if run_summary.get('headBranch') != spec['requiredHeadBranch']:
        reason = (
            f"{spec['workflowFile']} latest hosted run branch {run_summary.get('headBranch')!r} "
            f"did not match required {spec['requiredHeadBranch']!r}"
        )
        entry['status'] = 'failed'
        entry['failure'] = reason
        entry['freshnessStatus'] = 'failed'
        entry['freshnessFailure'] = reason
        entry['headShaMatchesExpected'] = False
        results.append(entry)
        errors.append(reason)
        continue

    if run_summary.get('status') != 'completed' or run_summary.get('conclusion') != 'success':
        reason = (
            f"{spec['workflowFile']} latest hosted run concluded {run_summary.get('status')!r}/"
            f"{run_summary.get('conclusion')!r} instead of completed/success"
        )
        entry['status'] = 'failed'
        entry['failure'] = reason
        entry['freshnessStatus'] = 'failed'
        entry['freshnessFailure'] = reason
        entry['headShaMatchesExpected'] = False
        results.append(entry)
        errors.append(reason)
        continue

    matched_jobs = {}
    failing_jobs = []
    missing_jobs = []
    for required_label in spec.get('requiredJobs', []):
        aliases = spec.get('jobAliases', {}).get(required_label, [required_label])
        matched_job, matched_alias = find_matching_job(jobs, aliases)
        if matched_job is None:
            missing_jobs.append(required_label)
            continue
        matched_jobs[required_label] = {
            'matchedAlias': matched_alias,
            'actualName': matched_job.get('name'),
            'status': matched_job.get('status'),
            'conclusion': matched_job.get('conclusion'),
        }
        if matched_job.get('status') != 'completed' or matched_job.get('conclusion') != 'success':
            failing_jobs.append(
                f"{required_label}: {matched_job.get('status')!r}/{matched_job.get('conclusion')!r}"
            )

    entry['matchedJobs'] = matched_jobs

    if missing_jobs:
        reason = f"{spec['workflowFile']} hosted run is missing required jobs: {missing_jobs}"
        entry['status'] = 'failed'
        entry['failure'] = reason
        entry['freshnessStatus'] = 'failed'
        entry['freshnessFailure'] = reason
        entry['headShaMatchesExpected'] = False
        results.append(entry)
        errors.append(reason)
        continue

    if failing_jobs:
        reason = f"{spec['workflowFile']} hosted run has non-green required jobs: {failing_jobs}"
        entry['status'] = 'failed'
        entry['failure'] = reason
        entry['freshnessStatus'] = 'failed'
        entry['freshnessFailure'] = reason
        entry['headShaMatchesExpected'] = False
        results.append(entry)
        errors.append(reason)
        continue

    forbidden_jobs = []
    for forbidden_label in spec.get('forbiddenJobs', []):
        aliases = spec.get('jobAliases', {}).get(forbidden_label, [forbidden_label])
        matched_job, _ = find_matching_job(jobs, aliases)
        if matched_job is not None:
            forbidden_jobs.append(forbidden_label)

    if forbidden_jobs:
        reason = f"{spec['workflowFile']} hosted run still includes forbidden jobs: {forbidden_jobs}"
        entry['status'] = 'failed'
        entry['failure'] = reason
        entry['freshnessStatus'] = 'failed'
        entry['freshnessFailure'] = reason
        entry['headShaMatchesExpected'] = False
        results.append(entry)
        errors.append(reason)
        continue

    missing_steps = []
    forbidden_steps = []
    for required_job_label, required_steps in spec.get('requiredSteps', {}).items():
        aliases = spec.get('jobAliases', {}).get(required_job_label, [required_job_label])
        matched_job, _ = find_matching_job(jobs, aliases)
        if matched_job is None:
            missing_steps.extend(f'{required_job_label}: {step_name}' for step_name in required_steps)
            continue
        steps = matched_job.get('steps')
        if not isinstance(steps, list):
            reason = f"{spec['workflowFile']} gh run view omitted steps for job {required_job_label!r}"
            entry['status'] = 'failed'
            entry['failure'] = reason
            entry['freshnessStatus'] = 'failed'
            entry['freshnessFailure'] = reason
            entry['headShaMatchesExpected'] = False
            results.append(entry)
            errors.append(reason)
            break
        actual_step_names = [step.get('name') for step in steps if isinstance(step, dict)]
        for required_step in required_steps:
            if required_step not in actual_step_names:
                missing_steps.append(f'{required_job_label}: {required_step}')
        for forbidden_step in spec.get('forbiddenSteps', {}).get(required_job_label, []):
            if forbidden_step in actual_step_names:
                forbidden_steps.append(f'{required_job_label}: {forbidden_step}')
    else:
        if missing_steps:
            reason = f"{spec['workflowFile']} hosted run is missing required steps: {missing_steps}"
            entry['status'] = 'failed'
            entry['failure'] = reason
            entry['freshnessStatus'] = 'failed'
            entry['freshnessFailure'] = reason
            entry['headShaMatchesExpected'] = False
            results.append(entry)
            errors.append(reason)
            continue

        if forbidden_steps:
            reason = f"{spec['workflowFile']} hosted run still includes forbidden steps: {forbidden_steps}"
            entry['status'] = 'failed'
            entry['failure'] = reason
            entry['freshnessStatus'] = 'failed'
            entry['freshnessFailure'] = reason
            entry['headShaMatchesExpected'] = False
            results.append(entry)
            errors.append(reason)
            continue

        entry['status'] = 'ok'
        results.append(entry)
        continue

artifact = {
    'generatedAt': datetime.now(timezone.utc).isoformat(),
    'repository': repo,
    'repositorySource': repo_source,
    'binaryTag': candidate_refs.get('binaryTag'),
    'workflows': results,
}
output_path.write_text(json.dumps(artifact, indent=2) + '\n')

print(f'repository: {repo} (source={repo_source})')
print(f'binary tag: {candidate_refs.get("binaryTag")}')
print(f'artifact: {relative(output_path)}')
for entry in results:
    print(f"{entry['workflowFile']}: {entry['status']}")
    print(
        f"  freshness: {entry['freshnessStatus']} "
        f"expected={entry.get('expectedHeadSha')!r} observed={entry.get('observedHeadSha')!r}"
    )
    if entry.get('failure'):
        print(f"  reason: {entry['failure']}")

if errors:
    print()
    print('errors:')
    for error in errors:
        print(f'- {error}')
    raise SystemExit(1)
PY
  then
    LAST_LOG_PATH="$log_path"
    fail_phase "$phase_name" "hosted workflow evidence drifted" "$log_path" "$REMOTE_RUNS_PATH"
  fi

  LAST_LOG_PATH="$log_path"
  finish_phase "$phase_name"
}

run_artifact_contract() {
  local phase_name="artifact-contract"
  local log_path="$VERIFY_ROOT/${phase_name}.log"

  start_phase "$phase_name" "verify stable hosted-evidence artifacts and phase markers were emitted"
  if ! python3 - "$VERIFY_ROOT" >"$log_path" 2>&1 <<'PY'
from pathlib import Path
import json
import sys

verify_root = Path(sys.argv[1])
required_files = [
    'status.txt',
    'current-phase.txt',
    'phase-report.txt',
    'candidate-refs.json',
    'remote-runs.json',
    'full-contract.log',
]
errors = []
for relative in required_files:
    path = verify_root / relative
    if not path.is_file():
        errors.append(f'missing required artifact {relative}')

phase_report_path = verify_root / 'phase-report.txt'
if phase_report_path.is_file():
    phase_report = phase_report_path.read_text(errors='replace')
    for marker in [
        'gh-preflight\tpassed',
        'candidate-refs\tpassed',
        'remote-evidence\tpassed',
    ]:
        if marker not in phase_report:
            errors.append(f'phase-report.txt missing marker {marker!r}')

candidate_refs_path = verify_root / 'candidate-refs.json'
if candidate_refs_path.is_file():
    try:
        candidate_refs = json.loads(candidate_refs_path.read_text())
    except json.JSONDecodeError as exc:
        errors.append(f'candidate-refs.json is not valid JSON: {exc}')
    else:
        binary_tag = candidate_refs.get('binaryTag')
        repository_source = candidate_refs.get('repositorySource')
        if not isinstance(binary_tag, str) or not binary_tag:
            errors.append('candidate-refs.json must declare a non-empty binaryTag')
        if not isinstance(repository_source, str) or not repository_source:
            errors.append('candidate-refs.json must declare a non-empty repositorySource')
        workflow_files = [workflow.get('workflowFile') for workflow in candidate_refs.get('workflows', []) if isinstance(workflow, dict)]
        if workflow_files != ['authoritative-verification.yml', 'deploy-services.yml', 'release.yml']:
            errors.append(f'candidate-refs.json workflow order drifted: {workflow_files!r}')
        if isinstance(binary_tag, str) and binary_tag:
            release_entries = [workflow for workflow in candidate_refs.get('workflows', []) if isinstance(workflow, dict) and workflow.get('workflowFile') == 'release.yml']
            if len(release_entries) != 1 or release_entries[0].get('expectedRef') != f'refs/tags/{binary_tag}':
                errors.append('candidate-refs.json release expectedRef must stay aligned with binaryTag')

remote_runs_path = verify_root / 'remote-runs.json'
if remote_runs_path.is_file():
    try:
        remote_runs = json.loads(remote_runs_path.read_text())
    except json.JSONDecodeError as exc:
        errors.append(f'remote-runs.json is not valid JSON: {exc}')
    else:
        if not isinstance(remote_runs.get('repositorySource'), str) or not remote_runs.get('repositorySource'):
            errors.append('remote-runs.json must declare a non-empty repositorySource')
        workflows = remote_runs.get('workflows')
        if not isinstance(workflows, list) or len(workflows) != 3:
            errors.append('remote-runs.json must contain exactly three workflow entries')
        else:
            for workflow in workflows:
                if not isinstance(workflow, dict):
                    errors.append('remote-runs.json contains a non-object workflow entry')
                    continue
                for required_field in ['workflowFile', 'requiredJobs', 'requiredSteps', 'forbiddenJobs', 'forbiddenSteps', 'expectedHeadSha', 'observedHeadSha', 'freshnessStatus']:
                    if required_field not in workflow:
                        errors.append(f"{workflow.get('workflowFile', '<unknown>')} missing field {required_field!r} in remote-runs.json")

if errors:
    print('errors:')
    for error in errors:
        print(f'- {error}')
    raise SystemExit(1)

print('artifact contract ok')
PY
  then
    LAST_LOG_PATH="$log_path"
    fail_phase "$phase_name" "hosted verifier artifact contract drifted" "$log_path" "$VERIFY_ROOT"
  fi

  LAST_LOG_PATH="$log_path"
  finish_phase "$phase_name"
}

prepare_verify_root
run_preflight
run_candidate_refs
run_remote_evidence
run_artifact_contract

echo "verify-m053-s03: ok"
echo "repository: ${GH_REPO} (source=${GH_REPO_SOURCE})"
