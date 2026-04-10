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
sys.path.insert(0, str(ROOT / 'scripts' / 'lib'))

from m057_project_mutation_apply import load_field_snapshot  # noqa: E402
from m057_tracker_inventory import (  # noqa: E402
    PROJECT_FIELD_PAGE_SIZE,
    PROJECT_NUMBER,
    PROJECT_OWNER,
    PROJECT_PAGE_SIZE,
    InventoryError,
    normalize_project_item,
    read_graphql_query,
    require_array,
    require_bool,
    require_int,
    require_object,
    require_string,
)

RESULTS_JSON = ROOT / '.gsd' / 'milestones' / 'M057' / 'slices' / 'S03' / 'project-mutation-results.json'
PLAN_JSON = ROOT / '.gsd' / 'milestones' / 'M057' / 'slices' / 'S03' / 'project-mutation-plan.json'
RESULTS_MD = ROOT / '.gsd' / 'milestones' / 'M057' / 'slices' / 'S03' / 'project-mutation-results.md'
FIELD_SNAPSHOT_JSON = ROOT / '.gsd' / 'milestones' / 'M057' / 'slices' / 'S01' / 'project-fields.snapshot.json'
VERIFY_DIR = ROOT / '.tmp' / 'm057-s03' / 'verify'
COMMANDS_DIR = VERIFY_DIR / 'commands'
SUMMARY_JSON = VERIFY_DIR / 'verification-summary.json'
PHASE_REPORT = VERIFY_DIR / 'phase-report.txt'
S02_VERIFY_SCRIPT = ROOT / 'scripts' / 'verify-m057-s02.sh'

EXPECTED = {
    'results_version': 'm057-s03-project-mutation-results-v1',
    'plan_version': 'm057-s03-project-mutation-plan-v1',
    'delete_handles': [
        'mesh-lang#3',
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
    'add_handles': ['hyperpush#58', 'mesh-lang#19'],
    'update_handles': [
        'hyperpush#29',
        'hyperpush#30',
        'hyperpush#31',
        'hyperpush#32',
        'hyperpush#33',
        'hyperpush#34',
        'hyperpush#35',
        'hyperpush#36',
        'hyperpush#37',
        'hyperpush#38',
        'hyperpush#39',
        'hyperpush#40',
        'hyperpush#41',
        'hyperpush#42',
        'hyperpush#43',
        'hyperpush#44',
        'hyperpush#45',
        'hyperpush#46',
        'hyperpush#47',
        'hyperpush#48',
        'hyperpush#49',
        'hyperpush#50',
        'hyperpush#57',
    ],
    'total_items': 55,
    'repo_counts': {
        'hyperpush-org/hyperpush': 48,
        'hyperpush-org/mesh-lang': 7,
    },
    'status_counts': {
        'Done': 2,
        'In Progress': 3,
        'Todo': 50,
    },
    'naming_titles': {
        'hyperpush#54': 'Hyperpush deploy topology: split marketing site from operator app routing and product runtime boundaries',
        'hyperpush#55': 'Hyperpush deployment: add a production Dockerfile and container startup path for the operator app',
        'hyperpush#56': 'Hyperpush deployment: create generic-VM compose stack and health verification for the marketing site, operator app, and product backend',
    },
    'inherited_rows': {
        'hyperpush#29': {
            'status': 'Todo',
            'domain': 'Hyperpush',
            'track': 'Core Parity',
            'commitment': 'Committed',
            'delivery_mode': 'Shared',
            'priority': 'P0',
            'start_date': '2026-04-10',
            'target_date': '2026-04-24',
            'hackathon_phase': 'Phase 2 — Parity',
        },
        'hyperpush#33': {
            'status': 'Todo',
            'domain': 'Hyperpush',
            'track': 'Operator App',
            'commitment': 'Committed',
            'delivery_mode': 'Shared',
            'priority': 'P0',
            'start_date': '2026-04-12',
            'target_date': '2026-04-30',
            'hackathon_phase': 'Phase 3 — Operator App',
        },
        'hyperpush#35': {
            'status': 'Todo',
            'domain': 'Hyperpush',
            'track': 'SaaS Growth',
            'commitment': 'Planned',
            'delivery_mode': 'SaaS-only',
            'priority': 'P1',
            'start_date': '2026-04-20',
            'target_date': '2026-05-06',
            'hackathon_phase': 'Phase 3 — Operator App',
        },
        'hyperpush#57': {
            'status': 'Todo',
            'domain': 'Hyperpush',
            'track': 'Operator App',
            'commitment': 'Committed',
            'delivery_mode': 'Shared',
            'priority': 'P0',
            'start_date': '2026-04-12',
            'target_date': '2026-04-30',
            'hackathon_phase': 'Phase 3 — Operator App',
        },
    },
}


class VerifyError(RuntimeError):
    def __init__(self, phase: str, target: str, drift_surface: str, message: str):
        super().__init__(message)
        self.phase = phase
        self.target = target
        self.drift_surface = drift_surface


state: dict[str, Any] = {
    'version': 'm057-s03-retained-verifier-v1',
    'started_at': datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace('+00:00', 'Z'),
    'status': 'running',
    'failed_phase': None,
    'drift_surface': None,
    'last_target': None,
    'command_count': 0,
    'commands': [],
    'phases': [],
    'paths': {
        'phase_report': str(PHASE_REPORT.relative_to(ROOT)),
        'summary_json': str(SUMMARY_JSON.relative_to(ROOT)),
        'handoff_markdown': str(RESULTS_MD.relative_to(ROOT)),
    },
}


def now_iso() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace('+00:00', 'Z')


def write_text(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding='utf8')


def write_json(path: Path, payload: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2) + '\n', encoding='utf8')


def persist_summary() -> None:
    write_json(SUMMARY_JSON, state)


def ensure_verify_dir() -> None:
    if VERIFY_DIR.exists():
        shutil.rmtree(VERIFY_DIR)
    COMMANDS_DIR.mkdir(parents=True, exist_ok=True)
    persist_summary()


def require(condition: bool, phase: str, target: str, drift_surface: str, message: str) -> None:
    if not condition:
        raise VerifyError(phase, target, drift_surface, message)


def set_last_target(target: str) -> None:
    state['last_target'] = target
    write_text(VERIFY_DIR / 'last-target.txt', target + '\n')
    persist_summary()


def record_phase(name: str, status: str, detail: str, *, drift_surface: str | None = None, target: str | None = None, extra: dict[str, Any] | None = None) -> None:
    record = {
        'phase': name,
        'status': status,
        'detail': detail,
        'recorded_at': now_iso(),
    }
    if drift_surface is not None:
        record['drift_surface'] = drift_surface
    if target is not None:
        record['target'] = target
    if extra:
        record.update(extra)
    state['phases'].append(record)
    write_json(VERIFY_DIR / f'phase-{len(state["phases"]):02d}-{name}.json', record)
    persist_summary()


def run_command(label: str, command: list[str], *, phase: str, target: str, timeout_seconds: int = 120, expect_success: bool = True) -> subprocess.CompletedProcess[str]:
    state['command_count'] += 1
    index = state['command_count']
    set_last_target(target)
    stdout_path = COMMANDS_DIR / f'{index:03d}-{label}.stdout.txt'
    stderr_path = COMMANDS_DIR / f'{index:03d}-{label}.stderr.txt'
    meta_path = COMMANDS_DIR / f'{index:03d}-{label}.json'

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
        state['commands'].append(meta)
        write_json(meta_path, meta)
        persist_summary()
        raise VerifyError(phase, target, 'project_membership', f'timed out after {timeout_seconds}s: {" ".join(command)}')

    write_text(stdout_path, completed.stdout)
    write_text(stderr_path, completed.stderr)
    meta = {
        'label': label,
        'phase': phase,
        'target': target,
        'command': command,
        'exit_code': completed.returncode,
        'timed_out': False,
        'timeout_seconds': timeout_seconds,
        'stdout_path': str(stdout_path.relative_to(ROOT)),
        'stderr_path': str(stderr_path.relative_to(ROOT)),
        'recorded_at': now_iso(),
    }
    state['commands'].append(meta)
    write_json(meta_path, meta)
    persist_summary()

    if expect_success and completed.returncode != 0:
        raise VerifyError(
            phase,
            target,
            'repo_truth' if phase == 'repo-precheck' else 'project_membership',
            f'command failed with exit {completed.returncode}: {" ".join(command)}\nsee {stdout_path.relative_to(ROOT)} and {stderr_path.relative_to(ROOT)}',
        )

    return completed


def read_json(path: Path, label: str) -> dict[str, Any]:
    if not path.is_file():
        raise VerifyError('artifact-contract', label, 'artifact_contract', f'missing required file: {path}')
    try:
        payload = json.loads(path.read_text())
    except json.JSONDecodeError as exc:
        raise VerifyError('artifact-contract', label, 'artifact_contract', f'invalid JSON in {path}: {exc}') from exc
    return require_object(payload, label)


def sorted_unique(values: list[str]) -> list[str]:
    return sorted(set(values))


def issue_handle_from_row(row: dict[str, Any]) -> str:
    issue = require_object(row.get('issue'), 'row.issue')
    repo = require_string(issue.get('repo'), 'row.issue.repo')
    number = issue.get('number')
    return f"{repo.split('/')[-1]}#{number}"


def field_value(row: dict[str, Any], field_key: str) -> Any:
    fields = require_object(row.get('field_values'), 'row.field_values')
    field = require_object(fields.get(field_key), f'row.field_values[{field_key}]')
    return field.get('value')


def assert_field_values(row: dict[str, Any], expected_fields: dict[str, Any], *, phase: str, target: str) -> None:
    for field_key, expected_value in expected_fields.items():
        actual_value = field_value(row, field_key)
        require(
            actual_value == expected_value,
            phase,
            target,
            'field_coherence',
            f'{target} field {field_key} expected {expected_value!r} but found {actual_value!r}',
        )


def flatten_plan_operation_ids(plan: dict[str, Any]) -> list[str]:
    operations_root = require_object(plan.get('operations'), 'plan.operations')
    ordered: list[str] = []
    for group in ('delete', 'add', 'update'):
        for operation in require_array(operations_root.get(group), f'plan.operations.{group}'):
            ordered.append(require_string(require_object(operation, f'plan.operations.{group}[]').get('operation_id'), 'operation.operation_id'))
    return ordered


def validate_artifacts(results: dict[str, Any], plan: dict[str, Any]) -> dict[str, Any]:
    phase = 'artifact-contract'
    require(results.get('version') == EXPECTED['results_version'], phase, 'results.version', 'artifact_contract', 'unexpected results version')
    require(results.get('status') == 'ok', phase, 'results.status', 'artifact_contract', 'results artifact is not marked ok')
    require(results.get('mode') == 'apply', phase, 'results.mode', 'artifact_contract', 'results artifact must come from apply mode')

    source_plan = require_object(results.get('source_plan'), 'results.source_plan')
    require(source_plan.get('version') == EXPECTED['plan_version'], phase, 'results.source_plan.version', 'artifact_contract', 'unexpected source plan version')
    repo_precheck = require_object(source_plan.get('repo_precheck'), 'results.source_plan.repo_precheck')
    require(repo_precheck.get('status') == 'ok', phase, 'results.source_plan.repo_precheck.status', 'artifact_contract', 'embedded S02 precheck is not green')
    require(repo_precheck.get('exit_code') == 0, phase, 'results.source_plan.repo_precheck.exit_code', 'artifact_contract', 'embedded S02 precheck exit code drifted')

    rollup = require_object(results.get('rollup'), 'results.rollup')
    planned = require_object(rollup.get('planned'), 'results.rollup.planned')
    require(planned.get('delete') == len(EXPECTED['delete_handles']), phase, 'results.rollup.planned.delete', 'artifact_contract', 'delete plan count drifted')
    require(planned.get('add') == len(EXPECTED['add_handles']), phase, 'results.rollup.planned.add', 'artifact_contract', 'add plan count drifted')
    require(planned.get('update') == len(EXPECTED['update_handles']), phase, 'results.rollup.planned.update', 'artifact_contract', 'update plan count drifted')
    require(rollup.get('total') == 35, phase, 'results.rollup.total', 'artifact_contract', 'total touched operation count drifted')
    require(rollup.get('applied') == 0, phase, 'results.rollup.applied', 'artifact_contract', 'rerun snapshot should not show newly applied operations')
    require(rollup.get('already_satisfied') == 35, phase, 'results.rollup.already_satisfied', 'artifact_contract', 'rerun snapshot should collapse to already_satisfied')
    require(rollup.get('failed') == 0, phase, 'results.rollup.failed', 'artifact_contract', 'results artifact contains failed operations')

    operations = [require_object(operation, 'results.operation') for operation in require_array(results.get('operations'), 'results.operations')]
    require(len(operations) == 35, phase, 'results.operations', 'artifact_contract', 'results artifact is missing touched operations')
    require(sorted_unique([operation['operation_id'] for operation in operations]) == sorted(flatten_plan_operation_ids(plan)), phase, 'results.operations', 'artifact_contract', 'results artifact does not match the checked plan operation set')

    delete_ops = [operation for operation in operations if operation.get('operation_kind') == 'delete']
    add_ops = [operation for operation in operations if operation.get('operation_kind') == 'add']
    update_ops = [operation for operation in operations if operation.get('operation_kind') == 'update']
    require(sorted_unique([operation['canonical_issue_handle'] for operation in delete_ops]) == sorted(EXPECTED['delete_handles']), phase, 'delete-touched-set', 'artifact_contract', 'delete touched set drifted')
    require(sorted_unique([operation['canonical_issue_handle'] for operation in add_ops]) == sorted(EXPECTED['add_handles']), phase, 'add-touched-set', 'artifact_contract', 'add touched set drifted')
    require(sorted_unique([operation['canonical_issue_handle'] for operation in update_ops]) == sorted(EXPECTED['update_handles']), phase, 'update-touched-set', 'artifact_contract', 'update touched set drifted')

    for operation in operations:
        target = require_string(operation.get('canonical_issue_handle'), 'operation.canonical_issue_handle')
        require(operation.get('status') == 'already_satisfied', phase, target, 'artifact_contract', f'{target} should be already_satisfied in the rerun snapshot')
        command_log = require_array(operation.get('command_log'), f'operation[{target}].command_log')
        require(len(command_log) == 0, phase, target, 'artifact_contract', f'{target} should not rerun live project mutations in the steady-state snapshot')

    mapping = require_object(results.get('canonical_mapping_results'), 'results.canonical_mapping_results')
    transfer = require_object(mapping.get('hyperpush_8_to_mesh_lang_19'), 'results.canonical_mapping_results.hyperpush_8_to_mesh_lang_19')
    pitch = require_object(mapping.get('pitch_gap_to_hyperpush_58'), 'results.canonical_mapping_results.pitch_gap_to_hyperpush_58')
    require(transfer.get('source_board_membership') == 'absent', phase, 'hyperpush#8 -> mesh-lang#19', 'artifact_contract', 'canonical transfer source membership drifted')
    require(transfer.get('destination_board_membership') == 'present', phase, 'hyperpush#8 -> mesh-lang#19', 'artifact_contract', 'canonical transfer destination membership drifted')
    require(pitch.get('destination_board_membership') == 'present', phase, '/pitch -> hyperpush#58', 'artifact_contract', 'pitch gap destination membership drifted')

    representative_rows = require_object(results.get('representative_rows'), 'results.representative_rows')
    require(require_object(representative_rows.get('done'), 'results.representative_rows.done').get('issue_handle') == 'mesh-lang#19', phase, 'representative done', 'artifact_contract', 'done representative drifted')
    require(require_object(representative_rows.get('in_progress'), 'results.representative_rows.in_progress').get('issue_handle') == 'hyperpush#54', phase, 'representative in_progress', 'artifact_contract', 'in-progress representative drifted')
    require(require_object(representative_rows.get('todo'), 'results.representative_rows.todo').get('issue_handle') == 'hyperpush#29', phase, 'representative todo', 'artifact_contract', 'todo representative drifted')

    naming_rows = [require_object(row, 'results.naming_preserved_rows[]') for row in require_array(results.get('naming_preserved_rows'), 'results.naming_preserved_rows')]
    require(sorted_unique([row['issue_handle'] for row in naming_rows]) == sorted(EXPECTED['naming_titles']), phase, 'naming-preserved-rows', 'artifact_contract', 'naming-preserved row set drifted')
    for handle, title in EXPECTED['naming_titles'].items():
        row = next(row for row in naming_rows if row['issue_handle'] == handle)
        actual_title = field_value(row, 'title')
        require(actual_title == title, phase, handle, 'artifact_contract', f'naming-preserved title drifted for {handle}')
        require('frontend-exp' not in actual_title and 'landing marketing' not in actual_title and 'mesher backend' not in actual_title, phase, handle, 'artifact_contract', f'{handle} title regressed to stale public naming')

    return {
        'delete_handles': EXPECTED['delete_handles'],
        'add_handles': EXPECTED['add_handles'],
        'update_handles': EXPECTED['update_handles'],
        'naming_handles': sorted(EXPECTED['naming_titles']),
    }


def capture_live_project_state(field_snapshot: dict[str, Any], tracked_fields: dict[str, dict[str, Any]]) -> dict[str, Any]:
    phase = 'live-project-capture'
    field_index = {field['field_id']: field for field in field_snapshot['fields']}
    query = read_graphql_query()
    all_items: list[dict[str, Any]] = []
    seen_cursors: set[str] = set()
    commands: list[dict[str, Any]] = []
    pages = 0
    after: str | None = None
    total_count: int | None = None
    project_meta: dict[str, Any] | None = None

    while True:
        pages += 1
        command = [
            'gh',
            'api',
            'graphql',
            '-f',
            f'query={query}',
            '-F',
            f'owner={PROJECT_OWNER}',
            '-F',
            f'number={PROJECT_NUMBER}',
            '-F',
            f'pageSize={PROJECT_PAGE_SIZE}',
            '-F',
            f'fieldPageSize={PROJECT_FIELD_PAGE_SIZE}',
        ]
        if after is not None:
            command.extend(['-F', f'after={after}'])
        completed = run_command(
            f'project-items-page-{pages}',
            command,
            phase=phase,
            target=f'project page {pages}',
            timeout_seconds=120,
            expect_success=True,
        )
        try:
            payload = json.loads(completed.stdout)
        except json.JSONDecodeError as exc:
            raise VerifyError(phase, f'project page {pages}', 'project_membership', f'invalid JSON from project capture page {pages}: {exc}') from exc

        root = require_object(payload, f'{phase}.page[{pages}]')
        data = require_object(root.get('data'), f'{phase}.page[{pages}].data')
        organization = require_object(data.get('organization'), f'{phase}.page[{pages}].data.organization')
        project = require_object(organization.get('projectV2'), f'{phase}.page[{pages}].data.organization.projectV2')
        page_meta = {
            'id': require_string(project.get('id'), f'{phase}.page[{pages}].project.id'),
            'title': require_string(project.get('title'), f'{phase}.page[{pages}].project.title'),
            'url': require_string(project.get('url'), f'{phase}.page[{pages}].project.url'),
            'owner': PROJECT_OWNER,
            'number': PROJECT_NUMBER,
        }
        if project_meta is None:
            project_meta = page_meta
        else:
            require(project_meta == page_meta, phase, f'project page {pages}', 'project_membership', 'project metadata changed between pages')

        items_connection = require_object(project.get('items'), f'{phase}.page[{pages}].project.items')
        page_total_count = require_int(items_connection.get('totalCount'), f'{phase}.page[{pages}].project.items.totalCount')
        if total_count is None:
            total_count = page_total_count
        else:
            require(total_count == page_total_count, phase, f'project page {pages}', 'project_membership', 'totalCount changed between pages')

        nodes = require_array(items_connection.get('nodes'), f'{phase}.page[{pages}].project.items.nodes')
        for item in nodes:
            try:
                normalized = normalize_project_item(
                    item,
                    index=len(all_items),
                    tracked_fields=tracked_fields,
                    field_index=field_index,
                )
            except InventoryError as exc:
                raise VerifyError(phase, f'project item {len(all_items)}', 'project_membership', str(exc)) from exc
            all_items.append(normalized)

        page_info = require_object(items_connection.get('pageInfo'), f'{phase}.page[{pages}].project.items.pageInfo')
        has_next_page = require_bool(page_info.get('hasNextPage'), f'{phase}.page[{pages}].project.items.pageInfo.hasNextPage')
        commands.append({'page': pages, 'after': after})
        if not has_next_page:
            break
        end_cursor = require_string(page_info.get('endCursor'), f'{phase}.page[{pages}].project.items.pageInfo.endCursor')
        require(end_cursor not in seen_cursors, phase, f'project page {pages}', 'project_membership', f'repeated pagination cursor {end_cursor!r}')
        seen_cursors.add(end_cursor)
        after = end_cursor

    require(project_meta is not None and total_count is not None, phase, 'project capture', 'project_membership', 'project capture returned no pages')

    repo_counts: dict[str, int] = {}
    status_counts: dict[str, int] = {}
    by_issue_handle: dict[str, dict[str, Any]] = {}
    by_issue_url: dict[str, dict[str, Any]] = {}
    for row in all_items:
        handle = issue_handle_from_row(row)
        require(handle not in by_issue_handle, phase, handle, 'project_membership', f'duplicate live board issue handle {handle}')
        issue_url = require_string(row.get('canonical_issue_url'), 'row.canonical_issue_url')
        require(issue_url not in by_issue_url, phase, handle, 'project_membership', f'duplicate live board canonical issue URL {issue_url}')
        by_issue_handle[handle] = row
        by_issue_url[issue_url] = row
        repo = require_string(require_object(row.get('issue'), 'row.issue').get('repo'), 'row.issue.repo')
        repo_counts[repo] = repo_counts.get(repo, 0) + 1
        status = field_value(row, 'status') or '<unset>'
        status_counts[status] = status_counts.get(status, 0) + 1

    require(len(all_items) == total_count, phase, 'project capture', 'project_membership', f'captured {len(all_items)} items but project reported totalCount {total_count}')

    return {
        'captured_at': now_iso(),
        'project': project_meta,
        'pages': pages,
        'items': sorted(all_items, key=lambda row: (require_object(row.get('issue'), 'row.issue').get('repo'), require_object(row.get('issue'), 'row.issue').get('number'))),
        'by_issue_handle': by_issue_handle,
        'by_issue_url': by_issue_url,
        'rollup': {
            'total_items': len(all_items),
            'repo_counts': dict(sorted(repo_counts.items())),
            'status_counts': dict(sorted(status_counts.items())),
        },
    }


def verify_project_membership(results: dict[str, Any], live_state: dict[str, Any], bucket_sets: dict[str, Any]) -> dict[str, Any]:
    phase = 'project-membership'
    rollup = require_object(live_state.get('rollup'), 'live_state.rollup')
    require(rollup.get('total_items') == EXPECTED['total_items'], phase, 'live board total', 'project_membership', f'expected {EXPECTED["total_items"]} board rows but found {rollup.get("total_items")}')
    require(rollup.get('repo_counts') == EXPECTED['repo_counts'], phase, 'live board repo counts', 'project_membership', f'live repo counts drifted: {rollup.get("repo_counts")}')
    require(rollup.get('status_counts') == EXPECTED['status_counts'], phase, 'live board status counts', 'project_membership', f'live status counts drifted: {rollup.get("status_counts")}')

    by_issue_handle = require_object(live_state.get('by_issue_handle'), 'live_state.by_issue_handle')
    for handle in bucket_sets['delete_handles']:
        require(handle not in by_issue_handle, phase, handle, 'project_membership', f'stale cleanup row {handle} still exists on the board')
    for handle in bucket_sets['add_handles'] + bucket_sets['update_handles'] + bucket_sets['naming_handles']:
        require(handle in by_issue_handle, phase, handle, 'project_membership', f'expected live board row {handle} is missing')

    require('hyperpush#8' not in by_issue_handle, phase, 'hyperpush#8', 'project_membership', 'source issue hyperpush#8 returned to the board after the canonical transfer')
    require('mesh-lang#19' in by_issue_handle, phase, 'mesh-lang#19', 'project_membership', 'canonical replacement row mesh-lang#19 is missing from the board')
    require('hyperpush#58' in by_issue_handle, phase, 'hyperpush#58', 'project_membership', 'canonical replacement row hyperpush#58 is missing from the board')

    mapping = require_object(results.get('canonical_mapping_results'), 'results.canonical_mapping_results')
    transfer = require_object(mapping.get('hyperpush_8_to_mesh_lang_19'), 'results.canonical_mapping_results.hyperpush_8_to_mesh_lang_19')
    pitch = require_object(mapping.get('pitch_gap_to_hyperpush_58'), 'results.canonical_mapping_results.pitch_gap_to_hyperpush_58')
    require(by_issue_handle['mesh-lang#19']['project_item_id'] == transfer.get('destination_project_item_id'), phase, 'mesh-lang#19', 'project_membership', 'mesh-lang#19 project item id drifted from the recorded canonical mapping')
    require(by_issue_handle['hyperpush#58']['project_item_id'] == pitch.get('destination_project_item_id'), phase, 'hyperpush#58', 'project_membership', 'hyperpush#58 project item id drifted from the recorded pitch-gap mapping')

    representative = require_object(results.get('representative_rows'), 'results.representative_rows')
    for slot in ('done', 'in_progress', 'todo'):
        expected_row = require_object(representative.get(slot), f'results.representative_rows.{slot}')
        handle = require_string(expected_row.get('issue_handle'), f'results.representative_rows.{slot}.issue_handle')
        live_row = by_issue_handle[handle]
        require(live_row['project_item_id'] == expected_row.get('project_item_id'), phase, handle, 'project_membership', f'representative {slot} project item id drifted')

    return {
        'total_items': rollup['total_items'],
        'repo_counts': rollup['repo_counts'],
        'status_counts': rollup['status_counts'],
        'stale_cleanup_absent': bucket_sets['delete_handles'],
    }


def verify_field_coherence(results: dict[str, Any], live_state: dict[str, Any]) -> dict[str, Any]:
    phase = 'field-coherence'
    by_issue_handle = require_object(live_state.get('by_issue_handle'), 'live_state.by_issue_handle')

    mesh19 = by_issue_handle['mesh-lang#19']
    require(require_object(mesh19.get('issue'), 'mesh19.issue').get('state') == 'CLOSED', phase, 'mesh-lang#19', 'field_coherence', 'mesh-lang#19 issue state drifted')
    assert_field_values(mesh19, {
        'status': 'Done',
        'domain': 'Mesh',
        'track': None,
        'commitment': None,
        'delivery_mode': None,
        'priority': None,
        'start_date': None,
        'target_date': None,
        'hackathon_phase': None,
    }, phase=phase, target='mesh-lang#19')

    hyperpush58 = by_issue_handle['hyperpush#58']
    require(require_object(hyperpush58.get('issue'), 'hyperpush58.issue').get('state') == 'CLOSED', phase, 'hyperpush#58', 'field_coherence', 'hyperpush#58 issue state drifted')
    assert_field_values(hyperpush58, {
        'status': 'Done',
        'domain': 'Hyperpush',
        'track': None,
        'commitment': None,
        'delivery_mode': None,
        'priority': None,
        'start_date': None,
        'target_date': None,
        'hackathon_phase': None,
    }, phase=phase, target='hyperpush#58')

    for handle, title in EXPECTED['naming_titles'].items():
        row = by_issue_handle[handle]
        actual_title = field_value(row, 'title')
        require(actual_title == title, phase, handle, 'field_coherence', f'{handle} live board title drifted from the normalized public wording')
        require('frontend-exp' not in actual_title and 'landing marketing' not in actual_title and 'mesher backend' not in actual_title, phase, handle, 'field_coherence', f'{handle} title regressed to stale public naming')
        assert_field_values(row, {
            'status': 'In Progress',
            'domain': 'Hyperpush',
            'track': 'Deployment',
            'commitment': 'Committed',
            'delivery_mode': 'Shared',
            'priority': 'P0',
            'start_date': '2026-04-10',
            'target_date': '2026-04-24',
            'hackathon_phase': 'Phase 1 — Foundation',
        }, phase=phase, target=handle)

    for handle, expected_fields in EXPECTED['inherited_rows'].items():
        assert_field_values(by_issue_handle[handle], expected_fields, phase=phase, target=handle)

    representative = require_object(results.get('representative_rows'), 'results.representative_rows')
    for slot, expected_handle in (('done', 'mesh-lang#19'), ('in_progress', 'hyperpush#54'), ('todo', 'hyperpush#29')):
        expected_row = require_object(representative.get(slot), f'results.representative_rows.{slot}')
        live_row = by_issue_handle[expected_handle]
        require(live_row['project_item_id'] == expected_row.get('project_item_id'), phase, expected_handle, 'field_coherence', f'representative {slot} project item id drifted')
        require(field_value(live_row, 'status') == field_value(expected_row, 'status'), phase, expected_handle, 'field_coherence', f'representative {slot} status drifted')
        require(field_value(live_row, 'domain') == field_value(expected_row, 'domain'), phase, expected_handle, 'field_coherence', f'representative {slot} domain drifted')
        require(field_value(live_row, 'track') == field_value(expected_row, 'track'), phase, expected_handle, 'field_coherence', f'representative {slot} track drifted')

    return {
        'representative_rows': {
            'done': 'mesh-lang#19',
            'in_progress': 'hyperpush#54',
            'todo': 'hyperpush#29',
        },
        'naming_handles': sorted(EXPECTED['naming_titles']),
        'inherited_handles': sorted(EXPECTED['inherited_rows']),
    }


def render_handoff_markdown(results: dict[str, Any], live_state: dict[str, Any], delegated_precheck: dict[str, Any]) -> str:
    by_issue_handle = require_object(live_state.get('by_issue_handle'), 'live_state.by_issue_handle')
    rollup = require_object(live_state.get('rollup'), 'live_state.rollup')
    representative = [
        ('Done', by_issue_handle['mesh-lang#19']),
        ('Active', by_issue_handle['hyperpush#54']),
        ('Next', by_issue_handle['hyperpush#29']),
    ]
    lines = [
        '# M057 S03 Board Truth Verification',
        '',
        f'- Verified at: `{now_iso()}`',
        f'- Results artifact: `{RESULTS_JSON.relative_to(ROOT)}`',
        f'- Retained verifier phase report: `{PHASE_REPORT.relative_to(ROOT)}`',
        f'- Retained verifier summary: `{SUMMARY_JSON.relative_to(ROOT)}`',
        f'- Delegated S02 verifier: `{delegated_precheck["phase_report"]}`',
        '',
        '## Final verified board truth',
        '',
        f"- Total board rows: `{rollup['total_items']}`",
        f"- Repo counts: `{json.dumps(rollup['repo_counts'], sort_keys=True)}`",
        f"- Status counts: `{json.dumps(rollup['status_counts'], sort_keys=True)}`",
        '',
        '| Slot | Issue | Project item | Repo | Status | Domain | Track | Title |',
        '| --- | --- | --- | --- | --- | --- | --- | --- |',
    ]
    for slot, row in representative:
        issue = require_object(row.get('issue'), 'row.issue')
        lines.append(
            f"| `{slot}` | `{issue_handle_from_row(row)}` | `{row['project_item_id']}` | `{issue['repo']}` | `{field_value(row, 'status')}` | `{field_value(row, 'domain')}` | `{field_value(row, 'track')}` | {field_value(row, 'title')} |"
        )
    lines.extend([
        '',
        '## Canonical mapping handling',
        '',
        '| Mapping | Source board membership | Destination board membership | Destination issue | Destination item |',
        '| --- | --- | --- | --- | --- |',
    ])
    transfer = require_object(require_object(results.get('canonical_mapping_results'), 'results.canonical_mapping_results').get('hyperpush_8_to_mesh_lang_19'), 'results.canonical_mapping_results.hyperpush_8_to_mesh_lang_19')
    pitch = require_object(require_object(results.get('canonical_mapping_results'), 'results.canonical_mapping_results').get('pitch_gap_to_hyperpush_58'), 'results.canonical_mapping_results.pitch_gap_to_hyperpush_58')
    lines.append(
        f"| `hyperpush#8 -> mesh-lang#19` | `{transfer['source_board_membership']}` | `{transfer['destination_board_membership']}` | `mesh-lang#19` | `{transfer['destination_project_item_id']}` |"
    )
    lines.append(
        f"| `/pitch -> hyperpush#58` | `n/a` | `{pitch['destination_board_membership']}` | `hyperpush#58` | `{pitch['destination_project_item_id']}` |"
    )
    lines.extend([
        '',
        '## Removed stale cleanup rows',
        '',
        'These previously stale cleanup items are now absent from org project #1, so the board no longer shows shipped Mesh cleanup rows as active roadmap work:',
        '',
    ])
    for handle in EXPECTED['delete_handles']:
        lines.append(f'- `{handle}`')
    lines.extend([
        '',
        '## Naming-normalized active rows',
        '',
        '| Issue | Project item | Status | Title |',
        '| --- | --- | --- | --- |',
    ])
    for handle in sorted(EXPECTED['naming_titles']):
        row = by_issue_handle[handle]
        lines.append(f"| `{handle}` | `{row['project_item_id']}` | `{field_value(row, 'status')}` | {field_value(row, 'title')} |")
    lines.extend([
        '',
        '## Inherited metadata spot checks',
        '',
        '| Issue | Status | Domain | Track | Commitment | Delivery | Priority | Start | Target | Phase |',
        '| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |',
    ])
    for handle in sorted(EXPECTED['inherited_rows']):
        row = by_issue_handle[handle]
        lines.append(
            f"| `{handle}` | `{field_value(row, 'status')}` | `{field_value(row, 'domain')}` | `{field_value(row, 'track')}` | `{field_value(row, 'commitment')}` | `{field_value(row, 'delivery_mode')}` | `{field_value(row, 'priority')}` | `{field_value(row, 'start_date')}` | `{field_value(row, 'target_date')}` | `{field_value(row, 'hackathon_phase')}` |"
        )
    lines.extend([
        '',
        '## Replay and failure surfaces',
        '',
        f'- Re-run `bash scripts/verify-m057-s03.sh` to replay the retained S03 verifier end to end.',
        f'- Start with `{PHASE_REPORT.relative_to(ROOT)}` to see the failed phase and drift surface, then inspect the last command logs under `{COMMANDS_DIR.relative_to(ROOT)}/`.',
        f'- The delegated repo-truth precheck still lives at `{delegated_precheck["summary_json"]}` and `{delegated_precheck["phase_report"]}`; if that phase goes red, treat it as repo-truth drift before touching project rows.',
        '',
    ])
    return '\n'.join(lines)


def write_phase_report(success: bool, *, error: VerifyError | None = None) -> None:
    lines = [
        '# M057 S03 retained verifier',
        '',
        f'- Status: `{ "ok" if success else "failed" }`',
        f'- Started at: `{state["started_at"]}`',
        f'- Finished at: `{now_iso()}`',
        f'- Failed phase: `{state["failed_phase"] or "none"}`',
        f'- Drift surface: `{state["drift_surface"] or "none"}`',
        f'- Last checked target: `{state["last_target"] or "none"}`',
        f'- Command count: `{state["command_count"]}`',
        '',
        'phase\tstatus\tdrift_surface\ttarget\tdetail',
    ]
    for phase in state['phases']:
        lines.append(
            f"{phase['phase']}\t{phase['status']}\t{phase.get('drift_surface', 'none')}\t{phase.get('target', 'none')}\t{phase['detail']}"
        )
    if error is not None:
        lines.extend([
            '',
            '## Failure detail',
            '',
            f'- Phase: `{error.phase}`',
            f'- Target: `{error.target}`',
            f'- Drift surface: `{error.drift_surface}`',
            f'- Message: {error}',
        ])
        if state['commands']:
            last_command = state['commands'][-1]
            lines.extend([
                f"- Last stdout: `{last_command['stdout_path']}`",
                f"- Last stderr: `{last_command['stderr_path']}`",
            ])
    write_text(PHASE_REPORT, '\n'.join(lines) + '\n')
    persist_summary()


def main() -> int:
    ensure_verify_dir()
    results = read_json(RESULTS_JSON, 'results-json')
    plan = read_json(PLAN_JSON, 'plan-json')
    field_snapshot, tracked_fields = load_field_snapshot(FIELD_SNAPSHOT_JSON)

    try:
        precheck_command = run_command(
            'retained-s02-verify',
            ['bash', str(S02_VERIFY_SCRIPT)],
            phase='repo-precheck',
            target='delegated S02 verifier',
            timeout_seconds=180,
            expect_success=True,
        )
        try:
            delegated_precheck = json.loads(precheck_command.stdout)
        except json.JSONDecodeError as exc:
            raise VerifyError('repo-precheck', 'delegated S02 verifier', 'repo_truth', f'invalid JSON from delegated S02 verifier: {exc}') from exc
        require(delegated_precheck.get('status') == 'ok', 'repo-precheck', 'delegated S02 verifier', 'repo_truth', 'delegated S02 verifier did not report ok status')
        state['delegated_repo_precheck'] = delegated_precheck
        record_phase('repo-precheck', 'ok', 'delegated retained S02 verifier passed', drift_surface='repo_truth', target='delegated S02 verifier', extra=delegated_precheck)

        bucket_sets = validate_artifacts(results, plan)
        state['artifact_contract'] = bucket_sets
        record_phase('artifact-contract', 'ok', 'validated touched-set coverage, canonical mapping summaries, and representative result rows', drift_surface='artifact_contract', extra=bucket_sets)

        live_state = capture_live_project_state(field_snapshot, tracked_fields)
        state['live_project_capture'] = {
            'captured_at': live_state['captured_at'],
            'rollup': live_state['rollup'],
            'pages': live_state['pages'],
        }
        record_phase('live-project-capture', 'ok', f"captured {live_state['rollup']['total_items']} live board rows across {live_state['pages']} page(s)", drift_surface='project_membership', extra=state['live_project_capture'])

        membership_summary = verify_project_membership(results, live_state, bucket_sets)
        state['project_membership'] = membership_summary
        record_phase('project-membership', 'ok', 'verified stale cleanup removals, canonical replacement presence, and representative row membership', drift_surface='project_membership', extra=membership_summary)

        coherence_summary = verify_field_coherence(results, live_state)
        state['field_coherence'] = coherence_summary
        record_phase('field-coherence', 'ok', 'verified naming normalization, canonical done rows, and inherited metadata spot checks', drift_surface='field_coherence', extra=coherence_summary)

        handoff_markdown = render_handoff_markdown(results, live_state, delegated_precheck)
        write_text(RESULTS_MD, handoff_markdown + '\n')
        state['handoff_markdown'] = str(RESULTS_MD.relative_to(ROOT))
        record_phase('handoff-render', 'ok', f'rendered {RESULTS_MD.relative_to(ROOT)}', drift_surface='field_coherence', target=str(RESULTS_MD.relative_to(ROOT)))

        state['status'] = 'ok'
        write_phase_report(True)
        print(json.dumps({
            'status': 'ok',
            'phase_report': str(PHASE_REPORT.relative_to(ROOT)),
            'summary_json': str(SUMMARY_JSON.relative_to(ROOT)),
            'handoff_markdown': str(RESULTS_MD.relative_to(ROOT)),
            'delegated_repo_precheck': {
                'phase_report': delegated_precheck['phase_report'],
                'summary_json': delegated_precheck['summary_json'],
            },
            'live_project_capture': state['live_project_capture'],
        }, indent=2))
        return 0
    except VerifyError as exc:
        state['status'] = 'failed'
        state['failed_phase'] = exc.phase
        state['drift_surface'] = exc.drift_surface
        record_phase(exc.phase, 'failed', str(exc), drift_surface=exc.drift_surface, target=exc.target)
        write_phase_report(False, error=exc)
        print(str(exc), file=sys.stderr)
        return 1


if __name__ == '__main__':
    raise SystemExit(main())
PY
