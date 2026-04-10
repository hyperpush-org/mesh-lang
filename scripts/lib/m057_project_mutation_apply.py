#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import shlex
import shutil
import subprocess
import sys
import time
from dataclasses import dataclass
from pathlib import Path
from typing import Any

from m057_project_mutation_plan import (
    DEFAULT_S01_DIR,
    DEFAULT_OUTPUT_DIR,
    PLAN_JSON_FILENAME,
    TRACKED_FIELD_KEYS,
    PlanError,
    validate_plan,
)
from m057_tracker_inventory import (
    InventoryError,
    PROJECT_NUMBER,
    PROJECT_OWNER,
    ROOT,
    capture_project_items,
    iso_now,
    require_array,
    require_object,
    require_string,
    write_json_atomic,
)

SCRIPT_RELATIVE_PATH = "scripts/lib/m057_project_mutation_apply.py"
RESULTS_JSON_FILENAME = "project-mutation-results.json"
RESULTS_MD_FILENAME = "project-mutation-results.md"
RESULTS_VERSION = "m057-s03-project-mutation-results-v1"
LAST_OPERATION_RELATIVE_PATH = Path(".tmp") / "m057-s03" / "apply" / "last-operation.txt"


class ApplyError(RuntimeError):
    pass


@dataclass
class CommandResult:
    command: list[str]
    display: str
    exit_code: int
    stdout: str
    stderr: str
    started_at: str
    completed_at: str
    timed_out: bool = False
    timeout_seconds: int | None = None

    def summary(self, *, include_output: bool = False) -> dict[str, Any]:
        payload = {
            "command": self.display,
            "exit_code": self.exit_code,
            "started_at": self.started_at,
            "completed_at": self.completed_at,
            "timed_out": self.timed_out,
        }
        if self.timeout_seconds is not None:
            payload["timeout_seconds"] = self.timeout_seconds
        if include_output:
            payload["stdout"] = self.stdout
            payload["stderr"] = self.stderr
        return payload


class CommandFailure(ApplyError):
    def __init__(self, phase: str, result: CommandResult):
        self.phase = phase
        self.result = result
        detail = result.stderr.strip() or result.stdout.strip() or f"exit {result.exit_code}"
        super().__init__(f"{phase}: command failed: {result.display}\n{detail}")


class InspectableOperationFailure(ApplyError):
    def __init__(self, message: str, *, command_log: list[dict[str, Any]] | None = None):
        super().__init__(message)
        self.command_log = command_log or []


@dataclass
class LiveState:
    captured_at: str
    snapshot: dict[str, Any]
    by_issue_url: dict[str, dict[str, Any]]
    by_item_id: dict[str, dict[str, Any]]


@dataclass
class ApplyContext:
    project_id: str
    project_owner: str
    project_number: int
    output_dir: Path
    last_operation_path: Path
    field_snapshot: dict[str, Any]
    tracked_fields: dict[str, dict[str, Any]]
    plan: dict[str, Any]
    live_state: LiveState


def last_operation_path_for_root(source_root: Path) -> Path:
    return source_root / LAST_OPERATION_RELATIVE_PATH


def repo_relpath(path: Path) -> str:
    try:
        return str(path.relative_to(ROOT))
    except ValueError:
        return str(path)


def command_display(command: list[str]) -> str:
    return " ".join(shlex.quote(part) for part in command)


def run_command(command: list[str], *, timeout_seconds: int = 120) -> CommandResult:
    started_at = iso_now()
    try:
        completed = subprocess.run(
            command,
            cwd=ROOT,
            capture_output=True,
            text=True,
            timeout=timeout_seconds,
            check=False,
        )
    except subprocess.TimeoutExpired as exc:
        stdout = exc.stdout if isinstance(exc.stdout, str) else (exc.stdout.decode("utf8", errors="replace") if exc.stdout else "")
        stderr = exc.stderr if isinstance(exc.stderr, str) else (exc.stderr.decode("utf8", errors="replace") if exc.stderr else "")
        return CommandResult(
            command=command,
            display=command_display(command),
            exit_code=124,
            stdout=stdout,
            stderr=stderr,
            started_at=started_at,
            completed_at=iso_now(),
            timed_out=True,
            timeout_seconds=timeout_seconds,
        )
    return CommandResult(
        command=command,
        display=command_display(command),
        exit_code=completed.returncode,
        stdout=completed.stdout,
        stderr=completed.stderr,
        started_at=started_at,
        completed_at=iso_now(),
    )


def ensure_gh_available() -> str:
    gh_path = shutil.which("gh")
    if gh_path is None:
        raise ApplyError("gh CLI not found on PATH")
    return gh_path


def read_json(path: Path, label: str) -> dict[str, Any]:
    if not path.is_file():
        raise ApplyError(f"missing {label}: {path}")
    try:
        payload = json.loads(path.read_text())
    except json.JSONDecodeError as exc:
        raise ApplyError(f"{label} is not valid JSON: {path}\n{exc}") from exc
    return require_object(payload, label)


def load_plan(plan_path: Path) -> dict[str, Any]:
    plan = read_json(plan_path, "plan")
    validate_plan(plan)
    return plan


def load_field_snapshot(path: Path) -> tuple[dict[str, Any], dict[str, dict[str, Any]]]:
    snapshot = read_json(path, "project fields snapshot")
    tracked_field_keys = {
        require_string(field_key, "project_fields.tracked_field_keys[]")
        for field_key in require_array(snapshot.get("tracked_field_keys"), "project_fields.tracked_field_keys")
    }
    tracked_fields: dict[str, dict[str, Any]] = {}
    for field in require_array(snapshot.get("fields"), "project_fields.fields"):
        field_object = require_object(field, "project_fields.field")
        field_key = require_string(field_object.get("field_key"), "project_fields.field.field_key")
        if field_key in tracked_field_keys:
            tracked_fields[field_key] = field_object
    missing = [field_key for field_key in TRACKED_FIELD_KEYS if field_key not in tracked_fields]
    if missing:
        raise ApplyError(f"project fields snapshot missing tracked keys: {', '.join(missing)}")
    project = require_object(snapshot.get("project"), "project_fields.project")
    require_string(project.get("id"), "project_fields.project.id")
    return snapshot, tracked_fields


def ordered_operations(plan: dict[str, Any]) -> list[dict[str, Any]]:
    operations_root = require_object(plan.get("operations"), "plan.operations")
    ordered: list[dict[str, Any]] = []
    for group in ("delete", "add", "update"):
        ordered.extend(require_object(op, f"plan.operations.{group}[]") for op in require_array(operations_root.get(group), f"plan.operations.{group}"))
    return ordered


def is_retryable_live_capture_error(exc: InventoryError) -> bool:
    message = str(exc)
    return "totalCount changed between pages" in message or "repeated pagination cursor" in message


def capture_live_state(*, field_snapshot: dict[str, Any], tracked_fields: dict[str, dict[str, Any]]) -> LiveState:
    captured_at = iso_now()
    snapshot = capture_project_items(
        captured_at=captured_at,
        tracked_fields=tracked_fields,
        project_fields_snapshot=field_snapshot,
        canonical_repos=require_object(field_snapshot.get("canonical_repos"), "project_fields.canonical_repos"),
    )
    items = [require_object(item, "project_items.item") for item in require_array(snapshot.get("items"), "project_items.items")]
    by_issue_url: dict[str, dict[str, Any]] = {}
    by_item_id: dict[str, dict[str, Any]] = {}
    for item in items:
        issue_url = require_string(item.get("canonical_issue_url"), "project_items.item.canonical_issue_url")
        item_id = require_string(item.get("project_item_id"), "project_items.item.project_item_id")
        if issue_url in by_issue_url:
            raise ApplyError(f"duplicate live project issue URL {issue_url}")
        if item_id in by_item_id:
            raise ApplyError(f"duplicate live project item id {item_id}")
        by_issue_url[issue_url] = item
        by_item_id[item_id] = item
    return LiveState(captured_at=captured_at, snapshot=snapshot, by_issue_url=by_issue_url, by_item_id=by_item_id)


def capture_live_state_with_retries(
    *,
    field_snapshot: dict[str, Any],
    tracked_fields: dict[str, dict[str, Any]],
    attempts: int = 3,
    delay_seconds: float = 2.0,
) -> LiveState:
    last_error: InventoryError | None = None
    for attempt in range(1, attempts + 1):
        try:
            return capture_live_state(field_snapshot=field_snapshot, tracked_fields=tracked_fields)
        except InventoryError as exc:
            if not is_retryable_live_capture_error(exc) or attempt == attempts:
                last_error = exc
                break
            last_error = exc
            time.sleep(delay_seconds)
    assert last_error is not None
    raise ApplyError(f"live project capture failed after {attempts} attempts: {last_error}") from last_error


def set_live_state(context: ApplyContext, live_state: LiveState) -> None:
    context.live_state = live_state


def refresh_live_state(context: ApplyContext) -> LiveState:
    live_state = capture_live_state_with_retries(field_snapshot=context.field_snapshot, tracked_fields=context.tracked_fields)
    set_live_state(context, live_state)
    return live_state


def project_rollup(snapshot: dict[str, Any]) -> dict[str, Any]:
    return require_object(snapshot.get("rollup"), "project_items.rollup")


def normalize_scalar(value: Any) -> Any:
    if isinstance(value, str):
        stripped = value.strip()
        return stripped or None
    return value


def current_row_for_operation(context: ApplyContext, operation: dict[str, Any]) -> dict[str, Any] | None:
    canonical_issue_url = require_string(operation.get("canonical_issue_url"), "operation.canonical_issue_url")
    planned_item_id = operation.get("project_item_id")
    if isinstance(planned_item_id, str) and planned_item_id in context.live_state.by_item_id:
        return context.live_state.by_item_id[planned_item_id]
    return context.live_state.by_issue_url.get(canonical_issue_url)


def field_value_for_key(row: dict[str, Any], field_key: str) -> dict[str, Any]:
    field_values = require_object(row.get("field_values"), "row.field_values")
    return require_object(field_values.get(field_key), f"row.field_values[{field_key!r}]")


def value_matches(*, current_field: dict[str, Any], desired_value: Any, desired_option_id: Any) -> bool:
    current_value = normalize_scalar(current_field.get("value"))
    current_option_id = normalize_scalar(current_field.get("option_id"))
    return current_value == normalize_scalar(desired_value) and current_option_id == normalize_scalar(desired_option_id)


def project_item_field_command(
    *,
    item_id: str,
    project_id: str,
    field_definition: dict[str, Any],
    desired_value: Any,
    desired_option_id: Any,
) -> list[str]:
    gh_path = ensure_gh_available()
    field_type = require_string(field_definition.get("field_type"), "field_definition.field_type")
    field_id = require_string(field_definition.get("field_id"), "field_definition.field_id")
    command = [
        gh_path,
        "project",
        "item-edit",
        "--id",
        item_id,
        "--project-id",
        project_id,
        "--field-id",
        field_id,
    ]
    if desired_value is None and desired_option_id is None:
        command.append("--clear")
        return command
    if field_type == "ProjectV2SingleSelectField":
        option_id = require_string(desired_option_id, f"{field_definition['field_key']}.desired_option_id")
        command.extend(["--single-select-option-id", option_id])
        return command
    value = require_string(desired_value, f"{field_definition['field_key']}.desired_value")
    if field_definition["field_key"] in {"start_date", "target_date"}:
        command.extend(["--date", value])
        return command
    command.extend(["--text", value])
    return command


def apply_local_field_value(row: dict[str, Any], *, field_key: str, desired_value: Any, desired_option_id: Any) -> None:
    field = field_value_for_key(row, field_key)
    field["value"] = normalize_scalar(desired_value)
    field["option_id"] = normalize_scalar(desired_option_id)
    if desired_option_id is not None:
        field["value_type"] = "ProjectV2ItemFieldSingleSelectValue"
    elif field_key in {"start_date", "target_date"} and desired_value is not None:
        field["value_type"] = "ProjectV2ItemFieldDateValue"
    elif desired_value is not None:
        field["value_type"] = "ProjectV2ItemFieldTextValue"
    else:
        field["value_type"] = None


def write_last_operation(*, operation: dict[str, Any], live_row: dict[str, Any] | None, last_operation_path: Path) -> None:
    last_operation_path.parent.mkdir(parents=True, exist_ok=True)
    lines = [
        f"timestamp={iso_now()}",
        f"operation_id={require_string(operation.get('operation_id'), 'operation.operation_id')}",
        f"operation_kind={require_string(operation.get('operation_kind'), 'operation.operation_kind')}",
        f"canonical_issue_handle={require_string(operation.get('canonical_issue_handle'), 'operation.canonical_issue_handle')}",
        f"canonical_issue_url={require_string(operation.get('canonical_issue_url'), 'operation.canonical_issue_url')}",
    ]
    if live_row is not None:
        lines.append(f"project_item_id={require_string(live_row.get('project_item_id'), 'live_row.project_item_id')}")
    elif isinstance(operation.get("project_item_id"), str):
        lines.append(f"project_item_id={operation['project_item_id']}")
    last_operation_path.write_text("\n".join(lines) + "\n", encoding="utf8")


def maybe_parse_add_item_id(stdout: str) -> str | None:
    stripped = stdout.strip()
    if not stripped:
        return None
    try:
        payload = json.loads(stripped)
    except json.JSONDecodeError:
        return None
    if isinstance(payload, dict):
        for key in ("id", "itemId", "projectItemId"):
            value = payload.get(key)
            if isinstance(value, str) and value.strip():
                return value.strip()
        for key in ("item", "projectItem"):
            nested = payload.get(key)
            if isinstance(nested, dict):
                value = nested.get("id")
                if isinstance(value, str) and value.strip():
                    return value.strip()
    return None


def command_log_with_failure(existing: list[dict[str, Any]], exc: Exception) -> list[dict[str, Any]]:
    if isinstance(exc, CommandFailure):
        return [*existing, exc.result.summary(include_output=True)]
    return getattr(exc, "command_log", existing)


def locate_live_row_after_add(
    *,
    context: ApplyContext,
    canonical_issue_url: str,
    project_item_id: str | None,
    attempts: int = 4,
    delay_seconds: float = 2.0,
) -> dict[str, Any] | None:
    for attempt in range(1, attempts + 1):
        refreshed = refresh_live_state(context)
        live_row = None
        if project_item_id is not None:
            live_row = refreshed.by_item_id.get(project_item_id)
        if live_row is None:
            live_row = refreshed.by_issue_url.get(canonical_issue_url)
        if live_row is not None:
            return live_row
        if attempt < attempts:
            time.sleep(delay_seconds)
    return None


def operation_request_snapshot(operation: dict[str, Any]) -> dict[str, Any]:
    snapshot = {
        "operation_id": require_string(operation.get("operation_id"), "operation.operation_id"),
        "operation_kind": require_string(operation.get("operation_kind"), "operation.operation_kind"),
        "canonical_issue_handle": require_string(operation.get("canonical_issue_handle"), "operation.canonical_issue_handle"),
        "canonical_issue_url": require_string(operation.get("canonical_issue_url"), "operation.canonical_issue_url"),
        "project_item_id": operation.get("project_item_id"),
        "current_row": operation.get("current_row"),
        "final_row_state": operation.get("final_row_state"),
    }
    for optional_key in ("field_changes", "change_count", "touch_reason", "project_action_kind", "note", "ledger_reason"):
        if optional_key in operation:
            snapshot[optional_key] = operation[optional_key]
    return snapshot


def initialize_results(*, plan: dict[str, Any], output_dir: Path, mode: str) -> dict[str, Any]:
    return {
        "version": RESULTS_VERSION,
        "generated_at": iso_now(),
        "source_script": SCRIPT_RELATIVE_PATH,
        "mode": mode,
        "output_dir": repo_relpath(output_dir),
        "status": "running",
        "started_at": iso_now(),
        "completed_at": None,
        "source_plan": {
            "path": repo_relpath(output_dir / PLAN_JSON_FILENAME),
            "version": require_string(plan.get("version"), "plan.version"),
            "generated_at": require_string(plan.get("generated_at"), "plan.generated_at"),
            "rollup": require_object(plan.get("rollup"), "plan.rollup"),
            "repo_precheck": require_object(plan.get("preflight"), "plan.preflight"),
            "canonical_mapping_handling": require_object(plan.get("canonical_mapping_handling"), "plan.canonical_mapping_handling"),
            "inheritance_rollup": require_object(plan.get("inheritance_rollup"), "plan.inheritance_rollup"),
        },
        "last_attempted_operation_id": None,
        "last_attempted_issue_handle": None,
        "last_attempted_project_item_id": None,
        "initial_live_capture": None,
        "final_live_capture": None,
        "canonical_mapping_results": None,
        "representative_rows": None,
        "naming_preserved_rows": None,
        "operations": [],
        "rollup": {
            "planned": {
                "delete": len(require_array(require_object(plan.get("operations"), "plan.operations").get("delete"), "plan.operations.delete")),
                "add": len(require_array(require_object(plan.get("operations"), "plan.operations").get("add"), "plan.operations.add")),
                "update": len(require_array(require_object(plan.get("operations"), "plan.operations").get("update"), "plan.operations.update")),
            },
            "total": 0,
            "applied": 0,
            "already_satisfied": 0,
            "failed": 0,
        },
    }


def update_rollup(results: dict[str, Any]) -> None:
    operations = [require_object(operation, "results.operation") for operation in require_array(results.get("operations"), "results.operations")]
    counts = {"applied": 0, "already_satisfied": 0, "failed": 0}
    by_kind = {"delete": 0, "add": 0, "update": 0}
    for operation in operations:
        status = require_string(operation.get("status"), "results.operation.status")
        operation_kind = require_string(operation.get("operation_kind"), "results.operation.operation_kind")
        if status in counts:
            counts[status] += 1
        if operation_kind in by_kind:
            by_kind[operation_kind] += 1
    rollup = require_object(results.get("rollup"), "results.rollup")
    rollup.update({"total": len(operations), **counts, "completed_by_kind": by_kind})


def render_row_summary(row: dict[str, Any] | None) -> dict[str, Any] | None:
    if row is None:
        return None
    issue = require_object(row.get("issue"), "row.issue")
    field_values = require_object(row.get("field_values"), "row.field_values")
    return {
        "project_item_id": require_string(row.get("project_item_id"), "row.project_item_id"),
        "issue_handle": f"{require_string(issue.get('repo'), 'row.issue.repo').split('/')[-1]}#{issue.get('number')}",
        "issue_url": require_string(row.get("canonical_issue_url"), "row.canonical_issue_url"),
        "repo": require_string(issue.get("repo"), "row.issue.repo"),
        "number": issue.get("number"),
        "issue_state": require_string(issue.get("state"), "row.issue.state"),
        "field_values": field_values,
    }


def render_results_markdown(results: dict[str, Any]) -> str:
    lines = [
        "# M057 S03 Project Mutation Results",
        "",
        f"- Version: `{require_string(results.get('version'), 'results.version')}`",
        f"- Generated at: `{require_string(results.get('generated_at'), 'results.generated_at')}`",
        f"- Mode: `{require_string(results.get('mode'), 'results.mode')}`",
        f"- Status: `{require_string(results.get('status'), 'results.status')}`",
        f"- Started at: `{require_string(results.get('started_at'), 'results.started_at')}`",
        f"- Completed at: `{results.get('completed_at')}`",
        "",
    ]

    source_plan = require_object(results.get("source_plan"), "results.source_plan")
    repo_precheck = require_object(source_plan.get("repo_precheck"), "results.source_plan.repo_precheck")
    lines.extend(
        [
            "## Source plan",
            "",
            f"- Plan path: `{require_string(source_plan.get('path'), 'results.source_plan.path')}`",
            f"- Plan version: `{require_string(source_plan.get('version'), 'results.source_plan.version')}`",
            f"- Embedded repo precheck status: `{require_string(repo_precheck.get('status'), 'results.source_plan.repo_precheck.status')}`",
            f"- Embedded repo precheck command: `{require_string(repo_precheck.get('command'), 'results.source_plan.repo_precheck.command')}`",
            f"- Embedded repo precheck exit code: `{repo_precheck.get('exit_code')}`",
            "",
        ]
    )

    rollup = require_object(results.get("rollup"), "results.rollup")
    planned = require_object(rollup.get("planned"), "results.rollup.planned")
    lines.extend(
        [
            "## Operation rollup",
            "",
            "| Kind | Planned | Completed |",
            "| --- | --- | --- |",
            f"| `delete` | `{planned.get('delete')}` | `{require_object(rollup.get('completed_by_kind'), 'results.rollup.completed_by_kind').get('delete')}` |",
            f"| `add` | `{planned.get('add')}` | `{require_object(rollup.get('completed_by_kind'), 'results.rollup.completed_by_kind').get('add')}` |",
            f"| `update` | `{planned.get('update')}` | `{require_object(rollup.get('completed_by_kind'), 'results.rollup.completed_by_kind').get('update')}` |",
            "",
            f"- Applied now: `{rollup.get('applied')}`",
            f"- Already satisfied: `{rollup.get('already_satisfied')}`",
            f"- Failed: `{rollup.get('failed')}`",
            "",
        ]
    )

    final_capture = results.get("final_live_capture")
    if isinstance(final_capture, dict):
        lines.extend(
            [
                "## Final board state",
                "",
                f"- Captured at: `{final_capture.get('captured_at')}`",
                f"- Total items: `{require_object(final_capture.get('rollup'), 'results.final_live_capture.rollup').get('total_items')}`",
                f"- Repo counts: `{json.dumps(require_object(final_capture.get('rollup'), 'results.final_live_capture.rollup').get('repo_counts'), sort_keys=True)}`",
                f"- Status counts: `{json.dumps(final_capture.get('status_counts'), sort_keys=True)}`",
                "",
            ]
        )

    mapping_results = results.get("canonical_mapping_results")
    if isinstance(mapping_results, dict):
        lines.extend(
            [
                "## Canonical mapping handling",
                "",
                "| Mapping | Source board membership | Destination board membership | Destination item |",
                "| --- | --- | --- | --- |",
            ]
        )
        transfer = require_object(mapping_results.get("hyperpush_8_to_mesh_lang_19"), "canonical_mapping_results.transfer")
        pitch = require_object(mapping_results.get("pitch_gap_to_hyperpush_58"), "canonical_mapping_results.pitch")
        lines.append(
            f"| `hyperpush#8 -> mesh-lang#19` | `{transfer.get('source_board_membership')}` | `{transfer.get('destination_board_membership')}` | `{transfer.get('destination_project_item_id')}` |"
        )
        lines.append(
            f"| `/pitch -> hyperpush#58` | `n/a` | `{pitch.get('destination_board_membership')}` | `{pitch.get('destination_project_item_id')}` |"
        )
        lines.append("")

    representative_rows = results.get("representative_rows")
    if isinstance(representative_rows, dict):
        lines.extend(
            [
                "## Representative done / active / next rows",
                "",
                "| Slot | Issue | Project item | Status | Domain | Track |",
                "| --- | --- | --- | --- | --- | --- |",
            ]
        )
        for slot in ("done", "in_progress", "todo"):
            row = representative_rows.get(slot)
            if not isinstance(row, dict):
                continue
            field_values = require_object(row.get("field_values"), f"representative_rows.{slot}.field_values")
            lines.append(
                f"| `{slot}` | `{row.get('issue_handle')}` | `{row.get('project_item_id')}` | `{require_object(field_values.get('status'), f'representative_rows.{slot}.status').get('value')}` | `{require_object(field_values.get('domain'), f'representative_rows.{slot}.domain').get('value')}` | `{require_object(field_values.get('track'), f'representative_rows.{slot}.track').get('value')}` |"
            )
        lines.append("")

    operations = [require_object(operation, "results.operation") for operation in require_array(results.get("operations"), "results.operations")]
    lines.extend(
        [
            "## Touched rows",
            "",
            "| Operation | Kind | Outcome | Issue | Project item | Status | Domain | Track |",
            "| --- | --- | --- | --- | --- | --- | --- | --- |",
        ]
    )
    for operation in operations:
        final_state = operation.get("final_state")
        if isinstance(final_state, dict):
            field_values = require_object(final_state.get("field_values"), "operation.final_state.field_values")
            project_item_id = final_state.get("project_item_id")
            status_value = require_object(field_values.get("status"), "operation.final_state.field_values.status").get("value")
            domain_value = require_object(field_values.get("domain"), "operation.final_state.field_values.domain").get("value")
            track_value = require_object(field_values.get("track"), "operation.final_state.field_values.track").get("value")
        else:
            project_item_id = "—"
            status_value = "—"
            domain_value = "—"
            track_value = "—"
        lines.append(
            f"| `{operation.get('operation_id')}` | `{operation.get('operation_kind')}` | `{operation.get('status')}` | `{operation.get('canonical_issue_handle')}` | `{project_item_id}` | `{status_value}` | `{domain_value}` | `{track_value}` |"
        )
    lines.append("")

    naming_rows = results.get("naming_preserved_rows")
    if isinstance(naming_rows, list) and naming_rows:
        lines.extend(
            [
                "## Naming-preserved reference rows",
                "",
                "| Issue | Project item | Status | Title |",
                "| --- | --- | --- | --- |",
            ]
        )
        for row in naming_rows:
            row_object = require_object(row, "results.naming_preserved_rows[]")
            field_values = require_object(row_object.get("field_values"), "results.naming_preserved_rows[].field_values")
            title = require_object(field_values.get("title"), "results.naming_preserved_rows[].field_values.title").get("value")
            status_value = require_object(field_values.get("status"), "results.naming_preserved_rows[].field_values.status").get("value")
            lines.append(
                f"| `{row_object.get('issue_handle')}` | `{row_object.get('project_item_id')}` | `{status_value}` | {title} |"
            )
        lines.append("")

    return "\n".join(lines)


def persist_results(results_path: Path, markdown_path: Path, results: dict[str, Any]) -> None:
    update_rollup(results)
    write_json_atomic(results_path, results)
    markdown_path.write_text(render_results_markdown(results) + "\n", encoding="utf8")


def initial_live_capture_summary(live_state: LiveState) -> dict[str, Any]:
    return {
        "captured_at": live_state.captured_at,
        "rollup": project_rollup(live_state.snapshot),
    }


def ensure_matching_final_state(*, live_row: dict[str, Any], final_row_state: dict[str, Any], planned_field_keys: list[str] | None = None) -> None:
    desired_fields = require_object(final_row_state.get("field_values"), "final_row_state.field_values")
    field_keys = planned_field_keys or TRACKED_FIELD_KEYS
    for field_key in field_keys:
        if field_key == "title":
            continue
        desired_field = require_object(desired_fields.get(field_key), f"final_row_state.field_values[{field_key!r}]")
        desired_value = desired_field.get("value")
        desired_option_id = desired_field.get("option_id")
        if desired_value is None and desired_option_id is None:
            continue
        live_field = field_value_for_key(live_row, field_key)
        if not value_matches(current_field=live_field, desired_value=desired_value, desired_option_id=desired_option_id):
            raise ApplyError(
                f"final board state mismatch for {require_string(final_row_state.get('issue_handle'), 'final_row_state.issue_handle')} field {field_key}: expected {desired_value!r}/{desired_option_id!r} but found {live_field.get('value')!r}/{live_field.get('option_id')!r}"
            )


def build_canonical_mapping_results(*, plan: dict[str, Any], live_state: LiveState) -> dict[str, Any]:
    mapping = require_object(plan.get("canonical_mapping_handling"), "plan.canonical_mapping_handling")
    transfer = require_object(mapping.get("hyperpush_8_to_mesh_lang_19"), "plan.canonical_mapping_handling.hyperpush_8_to_mesh_lang_19")
    pitch = require_object(mapping.get("pitch_gap_to_hyperpush_58"), "plan.canonical_mapping_handling.pitch_gap_to_hyperpush_58")

    transfer_destination_url = require_string(transfer.get("destination_issue_url"), "transfer.destination_issue_url")
    transfer_destination_row = live_state.by_issue_url.get(transfer_destination_url)
    source_hyperpush_8_url = "https://github.com/hyperpush-org/hyperpush/issues/8"
    pitch_destination_url = require_string(pitch.get("destination_issue_url"), "pitch.destination_issue_url")
    pitch_destination_row = live_state.by_issue_url.get(pitch_destination_url)

    return {
        "hyperpush_8_to_mesh_lang_19": {
            "source_issue_handle": require_string(transfer.get("source_issue_handle"), "transfer.source_issue_handle"),
            "destination_issue_handle": require_string(transfer.get("destination_issue_handle"), "transfer.destination_issue_handle"),
            "source_board_membership": "present" if source_hyperpush_8_url in live_state.by_issue_url else "absent",
            "destination_board_membership": "present" if transfer_destination_row is not None else "absent",
            "destination_project_item_id": transfer_destination_row.get("project_item_id") if transfer_destination_row else None,
        },
        "pitch_gap_to_hyperpush_58": {
            "gap_id": require_string(pitch.get("gap_id"), "pitch.gap_id"),
            "destination_issue_handle": require_string(pitch.get("destination_issue_handle"), "pitch.destination_issue_handle"),
            "destination_board_membership": "present" if pitch_destination_row is not None else "absent",
            "destination_project_item_id": pitch_destination_row.get("project_item_id") if pitch_destination_row else None,
        },
    }


def build_representative_rows(live_state: LiveState) -> dict[str, Any]:
    preferred = {
        "done": "https://github.com/hyperpush-org/mesh-lang/issues/19",
        "in_progress": "https://github.com/hyperpush-org/hyperpush/issues/54",
        "todo": "https://github.com/hyperpush-org/hyperpush/issues/29",
    }
    status_map = {"done": "Done", "in_progress": "In Progress", "todo": "Todo"}
    rows: dict[str, Any] = {}
    for slot, issue_url in preferred.items():
        row = live_state.by_issue_url.get(issue_url)
        if row is None:
            target_status = status_map[slot]
            for candidate in live_state.by_issue_url.values():
                status_value = field_value_for_key(candidate, "status").get("value")
                if status_value == target_status:
                    row = candidate
                    break
        if row is None:
            raise ApplyError(f"missing representative {slot} row from final live project state")
        rows[slot] = render_row_summary(row)
    return rows


def build_naming_preserved_rows(live_state: LiveState) -> list[dict[str, Any]]:
    issue_urls = [
        "https://github.com/hyperpush-org/hyperpush/issues/54",
        "https://github.com/hyperpush-org/hyperpush/issues/55",
        "https://github.com/hyperpush-org/hyperpush/issues/56",
    ]
    rows: list[dict[str, Any]] = []
    for issue_url in issue_urls:
        row = live_state.by_issue_url.get(issue_url)
        if row is None:
            raise ApplyError(f"missing naming-preserved reference row {issue_url}")
        rows.append(render_row_summary(row))
    return rows


def final_status_counts(live_state: LiveState) -> dict[str, int]:
    counts: dict[str, int] = {}
    for row in live_state.by_issue_url.values():
        value = field_value_for_key(row, "status").get("value") or "<unset>"
        counts[value] = counts.get(value, 0) + 1
    return dict(sorted(counts.items()))


def finalize_results(*, context: ApplyContext, results: dict[str, Any]) -> None:
    live_state = refresh_live_state(context)
    expected_total = require_object(context.plan.get("rollup"), "plan.rollup").get("desired_project_items")
    actual_total = project_rollup(live_state.snapshot).get("total_items")
    if actual_total != expected_total:
        raise ApplyError(f"final board item total drifted: expected {expected_total}, found {actual_total}")

    operations_lookup = {
        require_string(operation.get("operation_id"), "results.operation.operation_id"): operation
        for operation in require_array(results.get("operations"), "results.operations")
    }
    for operation in ordered_operations(context.plan):
        operation_id = require_string(operation.get("operation_id"), "operation.operation_id")
        result_operation = require_object(operations_lookup.get(operation_id), f"results.operations[{operation_id}]")
        operation_kind = require_string(operation.get("operation_kind"), "operation.operation_kind")
        issue_url = require_string(operation.get("canonical_issue_url"), "operation.canonical_issue_url")
        final_row = live_state.by_issue_url.get(issue_url)
        if operation_kind == "delete":
            if final_row is not None:
                raise ApplyError(f"delete operation {operation_id} left {issue_url} on the board")
            result_operation["final_state"] = None
            result_operation["project_item_id_after"] = None
            continue

        if final_row is None:
            raise ApplyError(f"operation {operation_id} expected {issue_url} to exist on the board after apply")
        final_row_summary = render_row_summary(final_row)
        result_operation["final_state"] = final_row_summary
        result_operation["project_item_id_after"] = final_row_summary["project_item_id"]
        result_operation["project_item_id"] = final_row_summary["project_item_id"]
        planned_final_state = require_object(operation.get("final_row_state"), f"operation[{operation_id}].final_row_state")
        if operation_kind == "add":
            ensure_matching_final_state(live_row=final_row, final_row_state=planned_final_state)
        else:
            field_changes = [require_object(change, f"operation[{operation_id}].field_changes[]") for change in require_array(operation.get("field_changes"), f"operation[{operation_id}].field_changes")]
            ensure_matching_final_state(
                live_row=final_row,
                final_row_state=planned_final_state,
                planned_field_keys=[require_string(change.get("field_key"), "field_change.field_key") for change in field_changes],
            )

    results["final_live_capture"] = {
        "captured_at": live_state.captured_at,
        "rollup": project_rollup(live_state.snapshot),
        "status_counts": final_status_counts(live_state),
    }
    results["canonical_mapping_results"] = build_canonical_mapping_results(plan=context.plan, live_state=live_state)
    results["representative_rows"] = build_representative_rows(live_state)
    results["naming_preserved_rows"] = build_naming_preserved_rows(live_state)


def execute_delete(operation: dict[str, Any], *, context: ApplyContext) -> dict[str, Any]:
    live_row = current_row_for_operation(context, operation)
    write_last_operation(operation=operation, live_row=live_row, last_operation_path=context.last_operation_path)
    command_log: list[dict[str, Any]] = []
    if live_row is None:
        return {
            "status": "already_satisfied",
            "skipped_reason": "project_row_already_absent",
            "command_log": command_log,
            "final_state": None,
            "project_item_id_after": None,
        }

    gh_path = ensure_gh_available()
    item_id = require_string(live_row.get("project_item_id"), "live_row.project_item_id")
    command = [gh_path, "project", "item-delete", str(context.project_number), "--owner", context.project_owner, "--id", item_id, "--format", "json"]
    result = run_command(command, timeout_seconds=120)
    if result.exit_code != 0:
        raise CommandFailure(f"{require_string(operation.get('operation_id'), 'operation.operation_id')}-delete", result)
    command_log.append(result.summary())
    issue_url = require_string(operation.get("canonical_issue_url"), "operation.canonical_issue_url")
    context.live_state.by_issue_url.pop(issue_url, None)
    context.live_state.by_item_id.pop(item_id, None)
    return {
        "status": "applied",
        "skipped_reason": None,
        "command_log": command_log,
        "final_state": None,
        "project_item_id_after": None,
    }


def desired_add_mutations(current_row: dict[str, Any], final_row_state: dict[str, Any]) -> list[dict[str, Any]]:
    desired_fields = require_object(final_row_state.get("field_values"), "final_row_state.field_values")
    mutations: list[dict[str, Any]] = []
    for field_key in TRACKED_FIELD_KEYS:
        if field_key == "title":
            continue
        desired_field = require_object(desired_fields.get(field_key), f"final_row_state.field_values[{field_key!r}]")
        desired_value = desired_field.get("value")
        desired_option_id = desired_field.get("option_id")
        if desired_value is None and desired_option_id is None:
            continue
        current_field = field_value_for_key(current_row, field_key)
        if value_matches(current_field=current_field, desired_value=desired_value, desired_option_id=desired_option_id):
            continue
        mutations.append(
            {
                "field_key": field_key,
                "desired_value": desired_value,
                "desired_option_id": desired_option_id,
            }
        )
    return mutations


def execute_add(operation: dict[str, Any], *, context: ApplyContext) -> dict[str, Any]:
    live_row = current_row_for_operation(context, operation)
    write_last_operation(operation=operation, live_row=live_row, last_operation_path=context.last_operation_path)
    command_log: list[dict[str, Any]] = []
    operation_id = require_string(operation.get("operation_id"), "operation.operation_id")
    issue_url = require_string(operation.get("canonical_issue_url"), "operation.canonical_issue_url")
    added = False

    if live_row is None:
        gh_path = ensure_gh_available()
        command = [gh_path, "project", "item-add", str(context.project_number), "--owner", context.project_owner, "--url", issue_url, "--format", "json"]
        result = run_command(command, timeout_seconds=120)
        if result.exit_code != 0:
            raise CommandFailure(f"{operation_id}-add", result)
        command_log.append(result.summary())
        added = True
        maybe_item_id = maybe_parse_add_item_id(result.stdout)
        live_row = locate_live_row_after_add(
            context=context,
            canonical_issue_url=issue_url,
            project_item_id=maybe_item_id,
        )
        if live_row is None:
            raise InspectableOperationFailure(
                f"{operation_id}: project item add succeeded but the canonical issue row did not appear in the refreshed board capture after retrying",
                command_log=command_log,
            )

    final_row_state = require_object(operation.get("final_row_state"), "operation.final_row_state")
    mutations = desired_add_mutations(live_row, final_row_state)
    for mutation in mutations:
        field_key = require_string(mutation.get("field_key"), "mutation.field_key")
        field_definition = require_object(context.tracked_fields.get(field_key), f"tracked_fields[{field_key!r}]")
        item_id = require_string(live_row.get("project_item_id"), "live_row.project_item_id")
        edit_command = project_item_field_command(
            item_id=item_id,
            project_id=context.project_id,
            field_definition=field_definition,
            desired_value=mutation.get("desired_value"),
            desired_option_id=mutation.get("desired_option_id"),
        )
        edit_result = run_command(edit_command, timeout_seconds=120)
        if edit_result.exit_code != 0:
            raise CommandFailure(f"{operation_id}-{field_key}", edit_result)
        command_log.append(edit_result.summary())
        apply_local_field_value(
            live_row,
            field_key=field_key,
            desired_value=mutation.get("desired_value"),
            desired_option_id=mutation.get("desired_option_id"),
        )

    if not added and not mutations:
        status = "already_satisfied"
        skipped_reason = "project_row_already_present_with_matching_fields"
    else:
        status = "applied"
        skipped_reason = None

    final_row_summary = render_row_summary(live_row)
    return {
        "status": status,
        "skipped_reason": skipped_reason,
        "command_log": command_log,
        "final_state": final_row_summary,
        "project_item_id_after": final_row_summary["project_item_id"],
    }


def execute_update(operation: dict[str, Any], *, context: ApplyContext) -> dict[str, Any]:
    live_row = current_row_for_operation(context, operation)
    write_last_operation(operation=operation, live_row=live_row, last_operation_path=context.last_operation_path)
    command_log: list[dict[str, Any]] = []
    operation_id = require_string(operation.get("operation_id"), "operation.operation_id")
    if live_row is None:
        raise InspectableOperationFailure(f"{operation_id}: missing live project row for update")

    field_changes = [require_object(change, "operation.field_change") for change in require_array(operation.get("field_changes"), "operation.field_changes")]
    applied_changes = 0
    for change in field_changes:
        field_key = require_string(change.get("field_key"), "field_change.field_key")
        current_field = field_value_for_key(live_row, field_key)
        after = require_object(change.get("after"), "field_change.after")
        desired_value = after.get("value")
        desired_option_id = after.get("option_id")
        if value_matches(current_field=current_field, desired_value=desired_value, desired_option_id=desired_option_id):
            continue
        field_definition = require_object(context.tracked_fields.get(field_key), f"tracked_fields[{field_key!r}]")
        command = project_item_field_command(
            item_id=require_string(live_row.get("project_item_id"), "live_row.project_item_id"),
            project_id=context.project_id,
            field_definition=field_definition,
            desired_value=desired_value,
            desired_option_id=desired_option_id,
        )
        result = run_command(command, timeout_seconds=120)
        if result.exit_code != 0:
            raise CommandFailure(f"{operation_id}-{field_key}", result)
        command_log.append(result.summary())
        apply_local_field_value(live_row, field_key=field_key, desired_value=desired_value, desired_option_id=desired_option_id)
        applied_changes += 1

    final_row_summary = render_row_summary(live_row)
    if applied_changes == 0:
        return {
            "status": "already_satisfied",
            "skipped_reason": "project_fields_already_match_plan",
            "command_log": command_log,
            "final_state": final_row_summary,
            "project_item_id_after": final_row_summary["project_item_id"],
        }
    return {
        "status": "applied",
        "skipped_reason": None,
        "command_log": command_log,
        "final_state": final_row_summary,
        "project_item_id_after": final_row_summary["project_item_id"],
    }


def execute_operation(operation: dict[str, Any], *, context: ApplyContext) -> dict[str, Any]:
    operation_kind = require_string(operation.get("operation_kind"), "operation.operation_kind")
    if operation_kind == "delete":
        return execute_delete(operation, context=context)
    if operation_kind == "add":
        return execute_add(operation, context=context)
    if operation_kind == "update":
        return execute_update(operation, context=context)
    raise ApplyError(f"unsupported operation kind {operation_kind!r}")


def build_operation_result(operation: dict[str, Any], *, index: int, outcome: dict[str, Any], error: str | None = None) -> dict[str, Any]:
    result = {
        "index": index,
        "operation_id": require_string(operation.get("operation_id"), "operation.operation_id"),
        "operation_kind": require_string(operation.get("operation_kind"), "operation.operation_kind"),
        "canonical_issue_handle": require_string(operation.get("canonical_issue_handle"), "operation.canonical_issue_handle"),
        "canonical_issue_url": require_string(operation.get("canonical_issue_url"), "operation.canonical_issue_url"),
        "status": outcome.get("status", "failed"),
        "requested": operation_request_snapshot(operation),
        "final_state": outcome.get("final_state"),
        "project_item_id_after": outcome.get("project_item_id_after"),
        "skipped_reason": outcome.get("skipped_reason"),
        "started_at": outcome.get("started_at"),
        "completed_at": outcome.get("completed_at"),
        "command_log": outcome.get("command_log", []),
    }
    if error is not None:
        result["error"] = error
    return result


def apply_plan(*, context: ApplyContext, results: dict[str, Any], results_path: Path, markdown_path: Path) -> dict[str, Any]:
    results["initial_live_capture"] = initial_live_capture_summary(context.live_state)
    persist_results(results_path, markdown_path, results)

    for index, operation in enumerate(ordered_operations(context.plan), start=1):
        operation_id = require_string(operation.get("operation_id"), "operation.operation_id")
        live_row = current_row_for_operation(context, operation)
        results["last_attempted_operation_id"] = operation_id
        results["last_attempted_issue_handle"] = require_string(operation.get("canonical_issue_handle"), "operation.canonical_issue_handle")
        results["last_attempted_project_item_id"] = live_row.get("project_item_id") if isinstance(live_row, dict) else operation.get("project_item_id")
        started_at = iso_now()
        try:
            outcome = execute_operation(operation, context=context)
            outcome["started_at"] = started_at
            outcome["completed_at"] = iso_now()
            results["operations"].append(build_operation_result(operation, index=index, outcome=outcome))
            persist_results(results_path, markdown_path, results)
        except (ApplyError, CommandFailure, InspectableOperationFailure, PlanError, InventoryError) as exc:
            failure_result = {
                "status": "failed",
                "final_state": render_row_summary(live_row) if isinstance(live_row, dict) else None,
                "project_item_id_after": live_row.get("project_item_id") if isinstance(live_row, dict) else None,
                "skipped_reason": None,
                "started_at": started_at,
                "completed_at": iso_now(),
                "command_log": command_log_with_failure([], exc),
            }
            results["operations"].append(build_operation_result(operation, index=index, outcome=failure_result, error=str(exc)))
            results["status"] = "failed"
            results["completed_at"] = iso_now()
            persist_results(results_path, markdown_path, results)
            raise

    finalize_results(context=context, results=results)
    results["status"] = "ok"
    results["completed_at"] = iso_now()
    persist_results(results_path, markdown_path, results)
    return results


def dry_run_summary(*, context: ApplyContext) -> dict[str, Any]:
    pending = {"delete": 0, "add": 0, "update": 0}
    already_satisfied = {"delete": 0, "add": 0, "update": 0}
    would_fail: list[str] = []
    for operation in ordered_operations(context.plan):
        operation_kind = require_string(operation.get("operation_kind"), "operation.operation_kind")
        live_row = current_row_for_operation(context, operation)
        if operation_kind == "delete":
            if live_row is None:
                already_satisfied[operation_kind] += 1
            else:
                pending[operation_kind] += 1
            continue
        if operation_kind == "add":
            if live_row is None:
                pending[operation_kind] += 1
                continue
            final_row_state = require_object(operation.get("final_row_state"), "operation.final_row_state")
            if desired_add_mutations(live_row, final_row_state):
                pending[operation_kind] += 1
            else:
                already_satisfied[operation_kind] += 1
            continue
        if live_row is None:
            would_fail.append(require_string(operation.get("operation_id"), "operation.operation_id"))
            continue
        field_changes = [require_object(change, "operation.field_change") for change in require_array(operation.get("field_changes"), "operation.field_changes")]
        missing = 0
        for change in field_changes:
            field_key = require_string(change.get("field_key"), "field_change.field_key")
            after = require_object(change.get("after"), "field_change.after")
            if not value_matches(
                current_field=field_value_for_key(live_row, field_key),
                desired_value=after.get("value"),
                desired_option_id=after.get("option_id"),
            ):
                missing += 1
        if missing == 0:
            already_satisfied[operation_kind] += 1
        else:
            pending[operation_kind] += 1

    return {
        "status": "ok",
        "mode": "check",
        "output_dir": str(context.output_dir),
        "plan_path": str(context.output_dir / PLAN_JSON_FILENAME),
        "plan_version": require_string(context.plan.get("version"), "plan.version"),
        "repo_precheck": require_object(context.plan.get("preflight"), "plan.preflight"),
        "planned_rollup": require_object(context.plan.get("rollup"), "plan.rollup"),
        "current_live_capture": initial_live_capture_summary(context.live_state),
        "pending": pending,
        "already_satisfied": already_satisfied,
        "would_fail": would_fail,
        "ordered_operation_ids": [require_string(operation.get("operation_id"), "operation.operation_id") for operation in ordered_operations(context.plan)],
    }


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Apply the M057 S03 checked project mutation manifest against org project #1.")
    parser.add_argument("--source-root", type=Path, default=ROOT, help="Alternate source root for isolated contract tests.")
    parser.add_argument("--s01-dir", type=Path, help="Directory containing the S01 project field snapshot.")
    parser.add_argument("--output-dir", type=Path, help="Directory containing the checked plan and receiving results artifacts.")
    parser.add_argument("--check", action="store_true", help="Validate the checked manifest and print a dry-run summary.")
    parser.add_argument("--apply", action="store_true", help="Apply the checked manifest to the live org project and persist results.")
    args = parser.parse_args(argv)
    if args.apply and args.check:
        parser.error("choose only one of --check or --apply")
    if not args.apply and not args.check:
        args.check = True
    args.source_root = args.source_root.resolve()
    args.s01_dir = (args.source_root / DEFAULT_S01_DIR.relative_to(ROOT)).resolve() if args.s01_dir is None else args.s01_dir.resolve()
    args.output_dir = (args.source_root / DEFAULT_OUTPUT_DIR.relative_to(ROOT)).resolve() if args.output_dir is None else args.output_dir.resolve()
    return args


def build_context(args: argparse.Namespace) -> ApplyContext:
    field_snapshot_path = args.s01_dir / "project-fields.snapshot.json"
    field_snapshot, tracked_fields = load_field_snapshot(field_snapshot_path)
    plan = load_plan(args.output_dir / PLAN_JSON_FILENAME)
    live_state = capture_live_state_with_retries(field_snapshot=field_snapshot, tracked_fields=tracked_fields)
    project = require_object(field_snapshot.get("project"), "project_fields.project")
    return ApplyContext(
        project_id=require_string(project.get("id"), "project_fields.project.id"),
        project_owner=PROJECT_OWNER,
        project_number=PROJECT_NUMBER,
        output_dir=args.output_dir,
        last_operation_path=last_operation_path_for_root(args.source_root),
        field_snapshot=field_snapshot,
        tracked_fields=tracked_fields,
        plan=plan,
        live_state=live_state,
    )


def main(argv: list[str]) -> int:
    args = parse_args(argv)
    context = build_context(args)

    if args.check:
        print(json.dumps(dry_run_summary(context=context), indent=2))
        return 0

    results_path = context.output_dir / RESULTS_JSON_FILENAME
    markdown_path = context.output_dir / RESULTS_MD_FILENAME
    results = initialize_results(plan=context.plan, output_dir=context.output_dir, mode="apply")
    results = apply_plan(context=context, results=results, results_path=results_path, markdown_path=markdown_path)
    print(
        json.dumps(
            {
                "status": results["status"],
                "mode": "apply",
                "output_dir": str(context.output_dir),
                "results_path": str(results_path),
                "markdown_path": str(markdown_path),
                "last_operation_path": str(context.last_operation_path),
                "rollup": results["rollup"],
                "last_attempted_operation_id": results["last_attempted_operation_id"],
            },
            indent=2,
        )
    )
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main(sys.argv[1:]))
    except (ApplyError, CommandFailure, InspectableOperationFailure, PlanError, InventoryError) as exc:
        print(str(exc), file=sys.stderr)
        raise SystemExit(1)
