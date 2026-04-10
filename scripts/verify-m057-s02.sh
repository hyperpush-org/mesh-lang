#!/usr/bin/env bash
set -euo pipefail

python3 - "$@" <<'PY'
from __future__ import annotations

import json
import shutil
import subprocess
import sys
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

ROOT = Path.cwd()
RESULTS_JSON = ROOT / '.gsd' / 'milestones' / 'M057' / 'slices' / 'S02' / 'repo-mutation-results.json'
PLAN_JSON = ROOT / '.gsd' / 'milestones' / 'M057' / 'slices' / 'S02' / 'repo-mutation-plan.json'
LEDGER_JSON = ROOT / '.gsd' / 'milestones' / 'M057' / 'slices' / 'S01' / 'reconciliation-ledger.json'
HANDOFF_MD = ROOT / '.gsd' / 'milestones' / 'M057' / 'slices' / 'S02' / 'repo-mutation-results.md'
VERIFY_DIR = ROOT / '.tmp' / 'm057-s02' / 'verify'
COMMANDS_DIR = VERIFY_DIR / 'commands'
SUMMARY_JSON = VERIFY_DIR / 'verification-summary.json'
PHASE_REPORT = VERIFY_DIR / 'phase-report.txt'

EXPECTED = {
    'total_operations': 43,
    'close_count': 10,
    'rewrite_count': 31,
    'rewrite_scope_count': 21,
    'mock_follow_count': 7,
    'mesh_total': 17,
    'mesh_open': 7,
    'mesh_closed': 10,
    'hyperpush_total': 52,
    'hyperpush_open': 47,
    'hyperpush_closed': 5,
    'combined_total': 69,
    'naming_handles': ['hyperpush#54', 'hyperpush#55', 'hyperpush#56'],
    'closed_close_handles': [
        'mesh-lang#4',
        'mesh-lang#5',
        'mesh-lang#6',
        'mesh-lang#8',
        'mesh-lang#9',
        'mesh-lang#10',
        'mesh-lang#11',
        'mesh-lang#13',
        'mesh-lang#14',
    ],
    'reopened_close_handle': 'mesh-lang#3',
    'transfer_operation_id': 'transfer-hyperpush-8',
    'create_operation_id': 'create-pitch-retrospective-issue',
    'transfer_destination': {
        'issue_handle': 'mesh-lang#19',
        'issue_url': 'https://github.com/hyperpush-org/mesh-lang/issues/19',
        'repo_slug': 'hyperpush-org/mesh-lang',
        'number': 19,
        'state': 'CLOSED',
        'closed_at': '2026-04-10T17:09:24Z',
        'state_reason': 'completed',
    },
    'create_destination': {
        'issue_handle': 'hyperpush#58',
        'issue_url': 'https://github.com/hyperpush-org/hyperpush/issues/58',
        'repo_slug': 'hyperpush-org/hyperpush',
        'number': 58,
    },
}


class VerifyError(RuntimeError):
    def __init__(self, phase: str, target: str, message: str):
        super().__init__(message)
        self.phase = phase
        self.target = target


@dataclass
class CommandLog:
    index: int
    label: str
    command: list[str]
    exit_code: int
    timed_out: bool
    timeout_seconds: int
    stdout_path: str
    stderr_path: str
    meta_path: str


state: dict[str, Any] = {
    'started_at': datetime.now(timezone.utc).isoformat(),
    'last_target': None,
    'command_count': 0,
    'commands': [],
    'phases': [],
    'failed_phase': None,
}


def now_iso() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace('+00:00', 'Z')


def normalize_multiline(value: str | None) -> str:
    return (value or '').replace('\r\n', '\n').strip()


def normalize_state_reason(value: str | None) -> str | None:
    if not isinstance(value, str):
        return None
    normalized = value.strip().lower()
    return normalized or None


def require(condition: bool, phase: str, target: str, message: str) -> None:
    if not condition:
        raise VerifyError(phase, target, message)


def read_json(path: Path, label: str) -> dict[str, Any]:
    if not path.is_file():
        raise VerifyError('artifact-contract', label, f'missing required file: {path}')
    try:
        return json.loads(path.read_text())
    except json.JSONDecodeError as exc:
        raise VerifyError('artifact-contract', label, f'invalid JSON in {path}: {exc}') from exc


def ensure_verify_dir() -> None:
    if VERIFY_DIR.exists():
        shutil.rmtree(VERIFY_DIR)
    COMMANDS_DIR.mkdir(parents=True, exist_ok=True)


def write_text(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding='utf8')


def write_json(path: Path, payload: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2) + '\n', encoding='utf8')


def set_last_target(target: str) -> None:
    state['last_target'] = target
    write_text(VERIFY_DIR / 'last-target.txt', target + '\n')


def record_phase(name: str, status: str, detail: str, *, extra: dict[str, Any] | None = None) -> None:
    record = {
        'phase': name,
        'status': status,
        'detail': detail,
        'recorded_at': now_iso(),
    }
    if extra:
        record.update(extra)
    state['phases'].append(record)
    write_json(VERIFY_DIR / f'phase-{len(state["phases"]):02d}-{name}.json', record)
    write_json(SUMMARY_JSON, state)


def run_command(label: str, command: list[str], *, phase: str, target: str, timeout_seconds: int = 60, expect_success: bool = True) -> subprocess.CompletedProcess[str]:
    state['command_count'] += 1
    index = state['command_count']
    set_last_target(target)
    stdout_path = COMMANDS_DIR / f'{index:03d}-{label}.stdout.txt'
    stderr_path = COMMANDS_DIR / f'{index:03d}-{label}.stderr.txt'
    meta_path = COMMANDS_DIR / f'{index:03d}-{label}.json'
    timed_out = False
    try:
        completed = subprocess.run(
            command,
            cwd=ROOT,
            text=True,
            capture_output=True,
            timeout=timeout_seconds,
            check=False,
        )
    except subprocess.TimeoutExpired as exc:
        timed_out = True
        stdout = exc.stdout if isinstance(exc.stdout, str) else (exc.stdout.decode('utf8', errors='replace') if exc.stdout else '')
        stderr = exc.stderr if isinstance(exc.stderr, str) else (exc.stderr.decode('utf8', errors='replace') if exc.stderr else '')
        write_text(stdout_path, stdout)
        write_text(stderr_path, stderr)
        meta = {
            'label': label,
            'phase': phase,
            'target': target,
            'command': command,
            'exit_code': 124,
            'timed_out': True,
            'timeout_seconds': timeout_seconds,
            'stdout_path': str(stdout_path.relative_to(ROOT)),
            'stderr_path': str(stderr_path.relative_to(ROOT)),
            'recorded_at': now_iso(),
        }
        write_json(meta_path, meta)
        state['commands'].append(meta)
        raise VerifyError(phase, target, f'timed out after {timeout_seconds}s: {" ".join(command)}')

    write_text(stdout_path, completed.stdout)
    write_text(stderr_path, completed.stderr)
    meta = {
        'label': label,
        'phase': phase,
        'target': target,
        'command': command,
        'exit_code': completed.returncode,
        'timed_out': timed_out,
        'timeout_seconds': timeout_seconds,
        'stdout_path': str(stdout_path.relative_to(ROOT)),
        'stderr_path': str(stderr_path.relative_to(ROOT)),
        'recorded_at': now_iso(),
    }
    write_json(meta_path, meta)
    state['commands'].append(meta)

    if expect_success and completed.returncode != 0:
        raise VerifyError(
            phase,
            target,
            f'command failed with exit {completed.returncode}: {" ".join(command)}\n'
            f'see {stdout_path.relative_to(ROOT)} and {stderr_path.relative_to(ROOT)}',
        )

    return completed


def gh_json(label: str, args: list[str], *, phase: str, target: str, timeout_seconds: int = 60, allow_failure: bool = False) -> Any:
    completed = run_command(label, ['gh', *args], phase=phase, target=target, timeout_seconds=timeout_seconds, expect_success=not allow_failure)
    if allow_failure and completed.returncode != 0:
        return {
            'exit_code': completed.returncode,
            'stdout': completed.stdout,
            'stderr': completed.stderr,
        }
    try:
        return json.loads(completed.stdout)
    except json.JSONDecodeError as exc:
        raise VerifyError(phase, target, f'invalid JSON from gh command {label}: {exc}') from exc


def label_names(payload: dict[str, Any]) -> list[str]:
    labels = payload.get('labels') or []
    return sorted(label.get('name') for label in labels if isinstance(label, dict) and isinstance(label.get('name'), str))


def comments_list(payload: dict[str, Any]) -> list[dict[str, Any]]:
    comments = payload.get('comments') or []
    return [comment for comment in comments if isinstance(comment, dict)]


def derive_expected_sets(ledger: dict[str, Any]) -> tuple[list[str], list[str]]:
    rewrite_scope = []
    mock_follow = []
    for row in ledger['rows']:
        action = row['proposed_repo_action_kind']
        if action == 'rewrite_scope':
            rewrite_scope.append(row['canonical_issue_handle'])
            continue
        matched = row.get('matched_evidence_ids') or []
        if action == 'keep_open' and matched == ['frontend_exp_operator_surfaces_partial']:
            mock_follow.append(row['canonical_issue_handle'])
    return sorted(rewrite_scope), sorted(mock_follow)


def validate_artifacts(results: dict[str, Any], plan: dict[str, Any], ledger: dict[str, Any]) -> dict[str, Any]:
    phase = 'artifact-contract'
    require(results.get('version') == 'm057-s02-repo-mutation-results-v1', phase, 'results.version', 'unexpected results version')
    require(results.get('status') == 'ok', phase, 'results.status', 'results artifact is not marked ok')
    require(results.get('mode') == 'apply', phase, 'results.mode', 'results artifact must come from apply mode')
    require(results.get('rollup', {}).get('total') == EXPECTED['total_operations'], phase, 'results.rollup.total', 'unexpected total operation count')
    require(results.get('rollup', {}).get('already_satisfied') == EXPECTED['total_operations'], phase, 'results.rollup.already_satisfied', 'expected idempotence snapshot with all operations already satisfied')
    require(results.get('rollup', {}).get('applied') == 0, phase, 'results.rollup.applied', 'idempotence snapshot should not show newly applied operations')
    require(results.get('rollup', {}).get('failed') == 0, phase, 'results.rollup.failed', 'results artifact contains failed operations')

    operations = results.get('operations') or []
    require(len(operations) == EXPECTED['total_operations'], phase, 'results.operations', 'results artifact is missing operations')
    plan_ids = {operation['operation_id'] for operation in plan['operations']}
    result_ids = {operation['operation_id'] for operation in operations}
    require(plan_ids == result_ids, phase, 'results.operations', 'results artifact does not match the checked plan operation set')

    close_ops = [operation for operation in operations if operation['operation_kind'] == 'close']
    rewrite_ops = [operation for operation in operations if operation['operation_kind'] == 'rewrite']
    transfer = next((operation for operation in operations if operation['operation_id'] == EXPECTED['transfer_operation_id']), None)
    create = next((operation for operation in operations if operation['operation_id'] == EXPECTED['create_operation_id']), None)

    require(len(close_ops) == EXPECTED['close_count'], phase, 'close-ops', 'unexpected close operation count')
    require(len(rewrite_ops) == EXPECTED['rewrite_count'], phase, 'rewrite-ops', 'unexpected rewrite operation count')
    require(transfer is not None, phase, EXPECTED['transfer_operation_id'], 'transfer operation missing from results')
    require(create is not None, phase, EXPECTED['create_operation_id'], 'create operation missing from results')

    rewrite_scope_handles, mock_follow_handles = derive_expected_sets(ledger)
    require(len(rewrite_scope_handles) == EXPECTED['rewrite_scope_count'], phase, 'rewrite_scope', 'rewrite_scope ledger count drifted')
    require(len(mock_follow_handles) == EXPECTED['mock_follow_count'], phase, 'mock_follow', 'mock-backed follow-through count drifted')
    rewrite_handles = sorted(operation['canonical_issue_handle'] for operation in rewrite_ops)
    derived_naming_handles = sorted(
        handle
        for handle in rewrite_handles
        if handle not in rewrite_scope_handles and handle not in mock_follow_handles
    )
    require(derived_naming_handles == EXPECTED['naming_handles'], phase, 'naming-normalization', 'unexpected rewrite handles outside ledger rewrite_scope/mock-follow buckets')

    close_handles = sorted(operation['canonical_issue_handle'] for operation in close_ops)
    require(
        close_handles == sorted([*EXPECTED['closed_close_handles'], EXPECTED['reopened_close_handle']]),
        phase,
        'close-ops',
        'close operation handle set drifted',
    )

    for operation in close_ops:
        target = operation['canonical_issue_handle']
        require(operation['status'] == 'already_satisfied', phase, target, 'close operation should be already_satisfied in the rerun snapshot')
        comment = (operation.get('matching_comment') or {}).get('body')
        require(isinstance(comment, str) and 'Closing as shipped.' in comment, phase, target, 'close operation lost the shipped closeout comment')

        if target == EXPECTED['reopened_close_handle']:
            require(operation['final_state']['state'] == 'OPEN', phase, target, 'reopened close-bucket issue must currently be OPEN')
            require(operation['final_state']['closed_at'] is None, phase, target, 'reopened close-bucket issue should not retain a closed_at timestamp')
            require(operation['final_state']['state_reason'] == 'reopened', phase, target, 'reopened close-bucket issue should carry reopened state_reason')
            continue

        require(operation['final_state']['state'] == 'CLOSED', phase, target, 'close operation final state must be CLOSED')
        require(bool(operation['final_state']['closed_at']), phase, target, 'closed close-bucket issue lost its closed_at timestamp')
        require(operation['final_state']['state_reason'] == 'completed', phase, target, 'closed close-bucket issue lost its completed state_reason')

    for operation in rewrite_ops:
        target = operation['canonical_issue_handle']
        require(operation['status'] == 'already_satisfied', phase, target, 'rewrite operation should be already_satisfied in the rerun snapshot')
        require(operation['final_state']['state'] == 'OPEN', phase, target, 'rewrite operation must stay OPEN')
        require(operation['final_state']['issue_handle'] == target, phase, target, 'rewrite operation changed identity unexpectedly')
        require(operation['final_state']['title'] == operation['requested']['title_after'], phase, target, 'rewrite operation title drifted from the planned text')
        require(
            normalize_multiline(operation['final_state']['body']) == normalize_multiline(operation['requested']['body_after']),
            phase,
            target,
            'rewrite operation body drifted from the planned text',
        )

    for handle in EXPECTED['naming_handles']:
        operation = next(operation for operation in rewrite_ops if operation['canonical_issue_handle'] == handle)
        body = operation['final_state']['body']
        require('Public issue wording should refer to `hyperpush-org/hyperpush`' in body, phase, handle, 'naming-normalization row lost its public repo wording fix')
        require('Public issue wording should refer to `hyperpush-mono`' not in body, phase, handle, 'stale public hyperpush-mono wording returned')

    require(transfer['identity']['changes_identity'] is True, phase, EXPECTED['transfer_operation_id'], 'transfer result must preserve identity change')
    require(transfer['final_state']['repo_slug'] == EXPECTED['transfer_destination']['repo_slug'], phase, 'transfer.final_state.repo_slug', 'transfer repo slug drifted')
    require(transfer['final_state']['issue_handle'] == EXPECTED['transfer_destination']['issue_handle'], phase, 'transfer.final_state.issue_handle', 'transfer canonical handle drifted')
    require(transfer['final_state']['issue_url'] == EXPECTED['transfer_destination']['issue_url'], phase, 'transfer.final_state.issue_url', 'transfer canonical url drifted')
    require(transfer['final_state']['state'] == EXPECTED['transfer_destination']['state'], phase, 'transfer.final_state.state', 'transferred issue state drifted from the retained live truth snapshot')
    require(transfer['final_state']['closed_at'] == EXPECTED['transfer_destination']['closed_at'], phase, 'transfer.final_state.closed_at', 'transferred issue close timestamp drifted')
    require(transfer['final_state']['state_reason'] == EXPECTED['transfer_destination']['state_reason'], phase, 'transfer.final_state.state_reason', 'transferred issue state_reason drifted')

    require(create['identity']['changes_identity'] is True, phase, EXPECTED['create_operation_id'], 'create result must preserve identity change')
    require(create['final_state']['repo_slug'] == EXPECTED['create_destination']['repo_slug'], phase, 'create.final_state.repo_slug', 'create repo slug drifted')
    require(create['final_state']['issue_handle'] == EXPECTED['create_destination']['issue_handle'], phase, 'create.final_state.issue_handle', 'created issue handle drifted')
    require(create['final_state']['issue_url'] == EXPECTED['create_destination']['issue_url'], phase, 'create.final_state.issue_url', 'created issue url drifted')
    require(create['final_state']['state'] == 'CLOSED', phase, 'create.final_state.state', 'created retrospective issue must stay closed')
    require('already shipped during M056' in (create.get('matching_comment') or {}).get('body', ''), phase, EXPECTED['create_operation_id'], 'retrospective /pitch close comment missing')

    return {
        'close_handles': close_handles,
        'closed_close_handles': EXPECTED['closed_close_handles'],
        'reopened_close_handles': [EXPECTED['reopened_close_handle']],
        'rewrite_scope_handles': rewrite_scope_handles,
        'mock_follow_handles': mock_follow_handles,
        'naming_handles': EXPECTED['naming_handles'],
    }


def verify_repo_totals() -> dict[str, Any]:
    phase = 'repo-totals'
    mesh_items = gh_json(
        'mesh-lang-issue-list',
        ['issue', 'list', '-R', 'hyperpush-org/mesh-lang', '--state', 'all', '--limit', '200', '--json', 'number,title,state,url'],
        phase=phase,
        target='hyperpush-org/mesh-lang issue list',
    )
    hyperpush_items = gh_json(
        'hyperpush-issue-list',
        ['issue', 'list', '-R', 'hyperpush-org/hyperpush', '--state', 'all', '--limit', '200', '--json', 'number,title,state,url'],
        phase=phase,
        target='hyperpush-org/hyperpush issue list',
    )
    require(len(mesh_items) == EXPECTED['mesh_total'], phase, 'hyperpush-org/mesh-lang', f'expected {EXPECTED["mesh_total"]} mesh-lang issues after transfer, found {len(mesh_items)}')
    require(len(hyperpush_items) == EXPECTED['hyperpush_total'], phase, 'hyperpush-org/hyperpush', f'expected {EXPECTED["hyperpush_total"]} hyperpush issues after create, found {len(hyperpush_items)}')
    combined = len(mesh_items) + len(hyperpush_items)
    require(combined == EXPECTED['combined_total'], phase, 'combined-totals', f'expected combined total {EXPECTED["combined_total"]}, found {combined}')

    mesh_numbers = sorted(item['number'] for item in mesh_items)
    hyperpush_numbers = sorted(item['number'] for item in hyperpush_items)
    require(EXPECTED['transfer_destination']['number'] in mesh_numbers, phase, 'mesh-lang#19', 'mesh-lang repo totals do not include the transferred issue number')
    require(EXPECTED['create_destination']['number'] in hyperpush_numbers, phase, 'hyperpush#58', 'hyperpush repo totals do not include the retrospective /pitch issue number')

    mesh_open = sum(1 for item in mesh_items if item['state'] == 'OPEN')
    mesh_closed = sum(1 for item in mesh_items if item['state'] == 'CLOSED')
    hyperpush_open = sum(1 for item in hyperpush_items if item['state'] == 'OPEN')
    hyperpush_closed = sum(1 for item in hyperpush_items if item['state'] == 'CLOSED')
    require(mesh_open == EXPECTED['mesh_open'], phase, 'hyperpush-org/mesh-lang', f'expected {EXPECTED["mesh_open"]} open mesh-lang issues, found {mesh_open}')
    require(mesh_closed == EXPECTED['mesh_closed'], phase, 'hyperpush-org/mesh-lang', f'expected {EXPECTED["mesh_closed"]} closed mesh-lang issues, found {mesh_closed}')
    require(hyperpush_open == EXPECTED['hyperpush_open'], phase, 'hyperpush-org/hyperpush', f'expected {EXPECTED["hyperpush_open"]} open hyperpush issues, found {hyperpush_open}')
    require(hyperpush_closed == EXPECTED['hyperpush_closed'], phase, 'hyperpush-org/hyperpush', f'expected {EXPECTED["hyperpush_closed"]} closed hyperpush issues, found {hyperpush_closed}')

    return {
        'mesh_lang': {
            'total': len(mesh_items),
            'open': mesh_open,
            'closed': mesh_closed,
        },
        'hyperpush': {
            'total': len(hyperpush_items),
            'open': hyperpush_open,
            'closed': hyperpush_closed,
        },
        'combined_total': combined,
    }


def verify_issue_matches(operation: dict[str, Any]) -> dict[str, Any]:
    phase = 'issue-state-replay'
    final_state = operation['final_state']
    repo_slug = final_state['repo_slug']
    number = final_state['number']
    handle = final_state['issue_handle']
    fields = 'number,title,state,url,body,labels,comments,closedAt,stateReason'
    payload = gh_json(
        f'issue-view-{repo_slug.split("/")[-1]}-{number}',
        ['issue', 'view', str(number), '-R', repo_slug, '--json', fields],
        phase=phase,
        target=handle,
    )
    require(payload['number'] == number, phase, handle, 'issue number drifted')
    require(payload['url'] == final_state['issue_url'], phase, handle, 'canonical issue URL drifted')
    require(payload['state'] == final_state['state'], phase, handle, 'issue state drifted')
    require((payload.get('closedAt') or None) == final_state.get('closed_at'), phase, handle, 'issue closedAt drifted')
    require(normalize_state_reason(payload.get('stateReason')) == final_state.get('state_reason'), phase, handle, 'issue stateReason drifted')
    require(payload['title'] == final_state['title'], phase, handle, 'issue title drifted')
    require(normalize_multiline(payload.get('body')) == normalize_multiline(final_state.get('body')), phase, handle, 'issue body drifted')
    require(label_names(payload) == sorted(final_state.get('labels') or []), phase, handle, 'issue labels drifted')

    if operation['operation_kind'] in {'close', 'create'}:
        expected_comment = ((operation.get('matching_comment') or {}).get('body') or '').strip()
        actual_comments = [normalize_multiline(comment.get('body')) for comment in comments_list(payload)]
        require(normalize_multiline(expected_comment) in actual_comments, phase, handle, 'expected verification comment is missing from the live issue')

    return {
        'issue_handle': handle,
        'repo_slug': repo_slug,
        'number': number,
        'state': payload['state'],
        'url': payload['url'],
    }


def verify_transfer_source_absence() -> dict[str, Any]:
    phase = 'transfer-source-absence'
    payload = gh_json(
        'hyperpush-source-8',
        ['issue', 'view', '8', '-R', 'hyperpush-org/hyperpush', '--json', 'number,title,state,url'],
        phase=phase,
        target='hyperpush#8 source repo lookup',
        allow_failure=True,
    )
    require(payload['exit_code'] != 0, phase, 'hyperpush#8 source repo lookup', 'hyperpush#8 still resolves in the source repo after transfer')
    stderr_text = (payload.get('stderr') or '').strip()
    require('Could not resolve to an issue' in stderr_text, phase, 'hyperpush#8 source repo lookup', 'source-repo transfer lookup failed for an unexpected reason')
    return {
        'exit_code': payload['exit_code'],
        'stderr_excerpt': stderr_text,
    }


def replay_issue_states(results: dict[str, Any], bucket_sets: dict[str, Any]) -> dict[str, Any]:
    operations = results['operations']
    verified = [verify_issue_matches(operation) for operation in operations]
    transfer_absence = verify_transfer_source_absence()
    return {
        'verified_operations': len(verified),
        'verified_handles': [row['issue_handle'] for row in verified],
        'close_handles': bucket_sets['close_handles'],
        'closed_close_handles': bucket_sets['closed_close_handles'],
        'reopened_close_handles': bucket_sets['reopened_close_handles'],
        'rewrite_scope_handles': bucket_sets['rewrite_scope_handles'],
        'mock_follow_handles': bucket_sets['mock_follow_handles'],
        'naming_handles': bucket_sets['naming_handles'],
        'transfer_source_absence': transfer_absence,
    }


def render_handoff(results: dict[str, Any], repo_totals: dict[str, Any], bucket_sets: dict[str, Any]) -> str:
    transfer = next(operation for operation in results['operations'] if operation['operation_id'] == EXPECTED['transfer_operation_id'])
    create = next(operation for operation in results['operations'] if operation['operation_id'] == EXPECTED['create_operation_id'])
    return '\n'.join([
        '# M057 S02 Repo Mutation Results',
        '',
        f'- Verified at: `{now_iso()}`',
        f'- Results artifact: `{RESULTS_JSON.relative_to(ROOT)}`',
        f'- Retained verifier report: `{PHASE_REPORT.relative_to(ROOT)}`',
        f'- Retained verifier summary: `{SUMMARY_JSON.relative_to(ROOT)}`',
        '',
        '## Canonical identity changes for S03',
        '',
        '| Source | Destination | Final state | Notes |',
        '| --- | --- | --- | --- |',
        f"| `hyperpush#8` | [`{transfer['final_state']['issue_handle']}`]({transfer['final_state']['issue_url']}) | `{transfer['final_state']['state']}` | Preserve docs-bug history under the language repo. |",
        f"| `/pitch` derived gap | [`{create['final_state']['issue_handle']}`]({create['final_state']['issue_url']}) | `{create['final_state']['state']}` | Retrospective product-repo issue for the already-shipped evaluator route. |",
        '',
        '## Verified repo totals',
        '',
        '| Repo | Total | Open | Closed |',
        '| --- | --- | --- | --- |',
        f"| `hyperpush-org/mesh-lang` | `{repo_totals['mesh_lang']['total']}` | `{repo_totals['mesh_lang']['open']}` | `{repo_totals['mesh_lang']['closed']}` |",
        f"| `hyperpush-org/hyperpush` | `{repo_totals['hyperpush']['total']}` | `{repo_totals['hyperpush']['open']}` | `{repo_totals['hyperpush']['closed']}` |",
        f"| Combined | `{repo_totals['combined_total']}` | — | — |",
        '',
        '## Bucket outcomes',
        '',
        f"- Still-closed shipped `mesh-lang` rows: `{len(bucket_sets['closed_close_handles'])}` verified closed with their closeout comments intact.",
        f"- Reopened shipped `mesh-lang` row: `{', '.join(bucket_sets['reopened_close_handles'])}` is now `OPEN`, so S03 should treat it as active repo truth rather than preserved done state.",
        f"- `rewrite_scope` product rows: `{len(bucket_sets['rewrite_scope_handles'])}` verified open with rewritten title/body text matching the checked plan.",
        f"- Mock-backed follow-through rows: `{len(bucket_sets['mock_follow_handles'])}` verified open with truthful wording that keeps the operator-app/backend gaps explicit.",
        f"- Naming-normalization rows: `{', '.join(bucket_sets['naming_handles'])}` verified open with public `hyperpush-org/hyperpush` wording and only compatibility-path mentions of `hyperpush-mono`.",
        '',
        '## Notes for S03',
        '',
        '- The checked `repo-mutation-results.json` is an idempotence rerun snapshot from T02, so every operation is recorded as `already_satisfied` even though the canonical transfer/create mappings remain authoritative.',
        '- Live repo truth has moved since the original S02 handoff: the transferred docs issue `mesh-lang#19` is now closed, and `mesh-lang#3` has been reopened.',
        '- The org project still needs its item URLs/statuses realigned to the repo-truth state above; that board-only drift is intentionally deferred to S03.',
        f"- Re-run `bash {Path('scripts/verify-m057-s02.sh')}` before S03 mutates project state if you need a fresh live-read confirmation.",
        '',
    ])


def write_phase_report(success: bool, *, error: VerifyError | None = None) -> None:
    lines = [
        '# M057 S02 retained verifier',
        '',
        f'- Status: `{ "ok" if success else "failed" }`',
        f'- Started at: `{state["started_at"]}`',
        f'- Finished at: `{now_iso()}`',
        f'- Failed phase: `{state["failed_phase"] or "none"}`',
        f'- Last checked target: `{state["last_target"] or "none"}`',
        f'- Command count: `{state["command_count"]}`',
        '',
        '## Phase results',
        '',
    ]
    for phase in state['phases']:
        status_emoji = '✅' if phase['status'] == 'ok' else '❌'
        lines.append(f"- {status_emoji} `{phase['phase']}` — {phase['detail']}")
    if error is not None:
        lines.extend([
            '',
            '## Failure detail',
            '',
            f'- Phase: `{error.phase}`',
            f'- Target: `{error.target}`',
            f'- Message: {error}',
        ])
        if state['commands']:
            last_command = state['commands'][-1]
            lines.extend([
                f"- Last stdout: `{last_command['stdout_path']}`",
                f"- Last stderr: `{last_command['stderr_path']}`",
            ])
    write_text(PHASE_REPORT, '\n'.join(lines) + '\n')
    write_json(SUMMARY_JSON, state)


def main() -> int:
    ensure_verify_dir()
    results = read_json(RESULTS_JSON, 'results-json')
    plan = read_json(PLAN_JSON, 'plan-json')
    ledger = read_json(LEDGER_JSON, 'ledger-json')

    try:
        bucket_sets = validate_artifacts(results, plan, ledger)
        record_phase('artifact-contract', 'ok', 'validated results.json mappings, bucket coverage, and idempotence snapshot semantics', extra=bucket_sets)

        repo_totals = verify_repo_totals()
        record_phase('repo-totals', 'ok', f"verified repo totals mesh-lang={repo_totals['mesh_lang']['total']} hyperpush={repo_totals['hyperpush']['total']} combined={repo_totals['combined_total']}", extra=repo_totals)

        replay_summary = replay_issue_states(results, bucket_sets)
        record_phase('issue-state-replay', 'ok', f"replayed {replay_summary['verified_operations']} touched issue views against the persisted final_state snapshots", extra=replay_summary)

        handoff_markdown = render_handoff(results, repo_totals, bucket_sets)
        write_text(HANDOFF_MD, handoff_markdown)
        record_phase('handoff-render', 'ok', f'rendered {HANDOFF_MD.relative_to(ROOT)} for S03 handoff consumers')

        write_phase_report(True)
        print(json.dumps({
            'status': 'ok',
            'phase_report': str(PHASE_REPORT.relative_to(ROOT)),
            'summary_json': str(SUMMARY_JSON.relative_to(ROOT)),
            'handoff_markdown': str(HANDOFF_MD.relative_to(ROOT)),
            'repo_totals': repo_totals,
        }, indent=2))
        return 0
    except VerifyError as exc:
        state['failed_phase'] = exc.phase
        record_phase(exc.phase, 'failed', str(exc), extra={'target': exc.target})
        write_phase_report(False, error=exc)
        print(str(exc), file=sys.stderr)
        return 1


if __name__ == '__main__':
    raise SystemExit(main())
PY
