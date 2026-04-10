#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any

from m057_tracker_inventory import (
    HYPERPUSH_REPO,
    InventoryError,
    MESH_LANG_REPO,
    ROOT,
    capture_project_fields,
    capture_project_items,
    capture_repo_issues,
    iso_now,
    normalize_field_key,
    require_array,
    require_bool,
    require_int,
    require_object,
    require_string,
    write_json_atomic,
)

SCRIPT_RELATIVE_PATH = "scripts/lib/m057_project_mutation_plan.py"
PLAN_JSON_FILENAME = "project-mutation-plan.json"
PLAN_MD_FILENAME = "project-mutation-plan.md"
PLAN_VERSION = "m057-s03-project-mutation-plan-v1"
DEFAULT_S01_DIR = ROOT / ".gsd" / "milestones" / "M057" / "slices" / "S01"
DEFAULT_S02_DIR = ROOT / ".gsd" / "milestones" / "M057" / "slices" / "S02"
DEFAULT_OUTPUT_DIR = ROOT / ".gsd" / "milestones" / "M057" / "slices" / "S03"
PRECHECK_SCRIPT = ROOT / "scripts" / "verify-m057-s02.sh"

TRACKED_FIELD_KEYS = [
    "title",
    "status",
    "domain",
    "track",
    "commitment",
    "delivery_mode",
    "priority",
    "start_date",
    "target_date",
    "hackathon_phase",
]
INHERITED_FIELD_KEYS = [
    "domain",
    "track",
    "commitment",
    "delivery_mode",
    "priority",
    "start_date",
    "target_date",
    "hackathon_phase",
]
EXPECTED_CURRENT_PROJECT_TOTAL = 63
EXPECTED_CURRENT_PROJECT_REPO_COUNTS = {
    MESH_LANG_REPO: 16,
    HYPERPUSH_REPO: 47,
}
EXPECTED_REPO_TOTALS = {
    MESH_LANG_REPO: 17,
    HYPERPUSH_REPO: 52,
    "combined": 69,
}
EXPECTED_DELETE_HANDLES = {
    "mesh-lang#3",
    "mesh-lang#4",
    "mesh-lang#5",
    "mesh-lang#6",
    "mesh-lang#8",
    "mesh-lang#9",
    "mesh-lang#10",
    "mesh-lang#11",
    "mesh-lang#13",
    "mesh-lang#14",
}
EXPECTED_ADD_HANDLES = {"mesh-lang#19", "hyperpush#58"}
EXPECTED_NAMING_HANDLES = {"hyperpush#54", "hyperpush#55", "hyperpush#56"}
EXPECTED_ROLLUP = {
    "delete": 10,
    "add": 2,
    "update": 23,
    "unchanged": 30,
    "inherited_rows": 23,
    "desired_total": 55,
}
PRIORITY_LABEL_TO_OPTION = {
    "priority: low": "P2",
    "priority: medium": "P1",
    "priority: high": "P0",
}
PARENT_SECTION_RE = re.compile(
    r"^##\s+Parent\s+(?:issue|epic)\s*$\s*^-\s+[^#\n]+#(\d+)\s*$",
    flags=re.IGNORECASE | re.MULTILINE,
)


class PlanError(RuntimeError):
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

    def to_summary(self) -> dict[str, Any]:
        payload: dict[str, Any] = {
            "command": self.display,
            "exit_code": self.exit_code,
            "started_at": self.started_at,
            "completed_at": self.completed_at,
            "timed_out": self.timed_out,
            "stdout": self.stdout,
            "stderr": self.stderr,
        }
        if self.timeout_seconds is not None:
            payload["timeout_seconds"] = self.timeout_seconds
        return payload


@dataclass
class PreflightOutcome:
    ok: bool
    record: dict[str, Any]


def repo_issue_handle(repo_slug: str, number: int) -> str:
    return f"{repo_slug.split('/')[-1]}#{number}"


def path_for_artifact(value: Path, *, root: Path) -> str:
    try:
        return str(value.relative_to(root))
    except ValueError:
        return str(value)


def read_json(path: Path, label: str) -> dict[str, Any]:
    if not path.is_file():
        raise PlanError(f"missing {label}: {path}")
    try:
        payload = json.loads(path.read_text())
    except json.JSONDecodeError as exc:
        raise PlanError(f"{label} is not valid JSON: {path}\n{exc}") from exc
    return require_object(payload, label)


def command_display(command: list[str]) -> str:
    return " ".join(command)


def run_command(command: list[str], *, cwd: Path, timeout_seconds: int) -> CommandResult:
    started_at = iso_now()
    try:
        completed = subprocess.run(
            command,
            cwd=cwd,
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


def normalize_scalar(value: Any) -> Any:
    if isinstance(value, str):
        stripped = value.strip()
        return stripped or None
    return value


def field_value_summary(field_value: dict[str, Any]) -> dict[str, Any]:
    return {
        "field_id": require_string(field_value.get("field_id"), "field_value.field_id"),
        "field_name": require_string(field_value.get("field_name"), "field_value.field_name"),
        "field_key": require_string(field_value.get("field_key"), "field_value.field_key"),
        "field_type": require_string(field_value.get("field_type"), "field_value.field_type"),
        "value": field_value.get("value"),
        "option_id": field_value.get("option_id"),
        "value_type": field_value.get("value_type"),
    }


def normalize_live_issue(issue: dict[str, Any]) -> dict[str, Any]:
    repo_slug = require_string(issue.get("repo"), "live_issue.repo")
    number = require_int(issue.get("number"), "live_issue.number")
    title = require_string(issue.get("title"), "live_issue.title")
    issue_url = require_string(issue.get("canonical_issue_url"), "live_issue.canonical_issue_url")
    labels = []
    for label in require_array(issue.get("labels", []), "live_issue.labels"):
        label_object = require_object(label, "live_issue.label")
        labels.append(require_string(label_object.get("name"), "live_issue.label.name"))
    labels.sort()
    return {
        "repo": repo_slug,
        "number": number,
        "issue_handle": repo_issue_handle(repo_slug, number),
        "issue_url": issue_url,
        "title": title,
        "state": require_string(issue.get("state"), "live_issue.state"),
        "body": require_string(issue.get("body"), "live_issue.body"),
        "labels": labels,
        "created_at": issue.get("created_at"),
        "updated_at": issue.get("updated_at"),
        "closed_at": issue.get("closed_at"),
    }


def normalize_live_item(item: dict[str, Any]) -> dict[str, Any]:
    issue = require_object(item.get("issue"), "live_item.issue")
    repo_slug = require_string(issue.get("repo"), "live_item.issue.repo")
    number = require_int(issue.get("number"), "live_item.issue.number")
    field_values = require_object(item.get("field_values"), "live_item.field_values")
    normalized_fields = {
        field_key: field_value_summary(require_object(field_values.get(field_key), f"live_item.field_values[{field_key!r}]"))
        for field_key in TRACKED_FIELD_KEYS
    }
    return {
        "project_item_id": require_string(item.get("project_item_id"), "live_item.project_item_id"),
        "canonical_issue_url": require_string(item.get("canonical_issue_url"), "live_item.canonical_issue_url"),
        "issue": {
            "repo": repo_slug,
            "number": number,
            "issue_handle": repo_issue_handle(repo_slug, number),
            "title": require_string(issue.get("title"), "live_item.issue.title"),
            "state": require_string(issue.get("state"), "live_item.issue.state"),
            "url": require_string(issue.get("url"), "live_item.issue.url"),
        },
        "field_values": normalized_fields,
    }


def compact_live_capture(raw_bundle: dict[str, Any]) -> dict[str, Any]:
    mesh_issues = [normalize_live_issue(issue) for issue in require_array(raw_bundle["mesh_lang_issues"].get("issues"), "mesh_lang_issues.issues")]
    hyperpush_issues = [normalize_live_issue(issue) for issue in require_array(raw_bundle["hyperpush_issues"].get("issues"), "hyperpush_issues.issues")]
    project_fields_snapshot = require_object(raw_bundle["project_fields"], "project_fields")
    project_items_snapshot = require_object(raw_bundle["project_items"], "project_items")
    return {
        "captured_at": require_string(project_items_snapshot.get("captured_at"), "project_items.captured_at"),
        "mesh_lang_issues": {
            "repo": require_object(raw_bundle["mesh_lang_issues"].get("repo"), "mesh_lang_issues.repo"),
            "rollup": require_object(raw_bundle["mesh_lang_issues"].get("rollup"), "mesh_lang_issues.rollup"),
            "issues": mesh_issues,
        },
        "hyperpush_issues": {
            "repo": require_object(raw_bundle["hyperpush_issues"].get("repo"), "hyperpush_issues.repo"),
            "rollup": require_object(raw_bundle["hyperpush_issues"].get("rollup"), "hyperpush_issues.rollup"),
            "issues": hyperpush_issues,
        },
        "project_fields": {
            "project": require_object(project_fields_snapshot.get("project"), "project_fields.project"),
            "tracked_field_keys": require_array(project_fields_snapshot.get("tracked_field_keys"), "project_fields.tracked_field_keys"),
            "fields": require_array(project_fields_snapshot.get("fields"), "project_fields.fields"),
        },
        "project_items": {
            "project": require_object(project_items_snapshot.get("project"), "project_items.project"),
            "rollup": require_object(project_items_snapshot.get("rollup"), "project_items.rollup"),
            "items": [normalize_live_item(item) for item in require_array(project_items_snapshot.get("items"), "project_items.items")],
        },
    }


def capture_live_state() -> dict[str, Any]:
    captured_at = iso_now()
    mesh_snapshot, hyperpush_snapshot, canonical_repos = capture_repo_issues(captured_at=captured_at)
    project_fields_snapshot, tracked_fields = capture_project_fields(captured_at=captured_at, canonical_repos=canonical_repos)
    project_items_snapshot = capture_project_items(
        captured_at=captured_at,
        tracked_fields=tracked_fields,
        project_fields_snapshot=project_fields_snapshot,
        canonical_repos=canonical_repos,
    )
    project_fields_snapshot["project"].update(
        {
            "id": project_items_snapshot["project"]["id"],
            "title": project_items_snapshot["project"]["title"],
            "url": project_items_snapshot["project"]["url"],
        }
    )
    return compact_live_capture(
        {
            "mesh_lang_issues": mesh_snapshot,
            "hyperpush_issues": hyperpush_snapshot,
            "project_fields": project_fields_snapshot,
            "project_items": project_items_snapshot,
        }
    )


def load_live_state(path: Path) -> dict[str, Any]:
    bundle = read_json(path, "live state bundle")
    return compact_live_capture(bundle)


def run_preflight(*, source_root: Path, override_json: Path | None) -> PreflightOutcome:
    if override_json is not None:
        payload = read_json(override_json, "preflight json")
        status = require_string(payload.get("status"), "preflight.status")
        return PreflightOutcome(ok=status == "ok", record=payload)

    command = ["bash", str(source_root / PRECHECK_SCRIPT.relative_to(ROOT))]
    result = run_command(command, cwd=source_root, timeout_seconds=300)
    record = {
        "status": "ok" if result.exit_code == 0 else ("timeout" if result.timed_out else "error"),
        "command": result.display,
        "exit_code": result.exit_code,
        "timed_out": result.timed_out,
        "started_at": result.started_at,
        "completed_at": result.completed_at,
        "stdout": result.stdout,
        "stderr": result.stderr,
        "parsed": None,
    }
    if result.exit_code == 0:
        try:
            record["parsed"] = json.loads(result.stdout)
        except json.JSONDecodeError as exc:
            record["status"] = "malformed"
            record["stderr"] = (record["stderr"] + f"\ninvalid preflight json: {exc}").strip()
            return PreflightOutcome(ok=False, record=record)
        return PreflightOutcome(ok=True, record=record)
    return PreflightOutcome(ok=False, record=record)


def validate_field_schema(*, snapshot_fields: dict[str, Any], live_fields: dict[str, Any]) -> dict[str, dict[str, Any]]:
    snapshot_keys = set(require_array(snapshot_fields.get("tracked_field_keys"), "snapshot_fields.tracked_field_keys"))
    live_keys = set(require_array(live_fields.get("tracked_field_keys"), "live_fields.tracked_field_keys"))
    if snapshot_keys != live_keys:
        raise PlanError(
            f"tracked field key drift: snapshot={sorted(snapshot_keys)!r} live={sorted(live_keys)!r}"
        )

    snapshot_index = {
        require_string(field.get("field_key"), "snapshot field.field_key"): require_object(field, "snapshot field")
        for field in require_array(snapshot_fields.get("fields"), "snapshot_fields.fields")
        if require_string(require_object(field, "snapshot field").get("field_key"), "snapshot field.field_key") in snapshot_keys
    }
    live_index = {
        require_string(field.get("field_key"), "live field.field_key"): require_object(field, "live field")
        for field in require_array(live_fields.get("fields"), "live_fields.fields")
        if require_string(require_object(field, "live field").get("field_key"), "live field.field_key") in live_keys
    }
    for field_key in sorted(snapshot_keys):
        snapshot_field = snapshot_index[field_key]
        live_field = live_index.get(field_key)
        if live_field is None:
            raise PlanError(f"live project fields missing tracked key {field_key!r}")
        if require_string(snapshot_field.get("field_id"), f"snapshot_fields[{field_key}].field_id") != require_string(live_field.get("field_id"), f"live_fields[{field_key}].field_id"):
            raise PlanError(f"project field id drifted for {field_key}")
        if require_string(snapshot_field.get("field_name"), f"snapshot_fields[{field_key}].field_name") != require_string(live_field.get("field_name"), f"live_fields[{field_key}].field_name"):
            raise PlanError(f"project field name drifted for {field_key}")
        snapshot_options = {
            require_string(option.get("option_key"), f"snapshot_fields[{field_key}].option.option_key"): require_string(option.get("id"), f"snapshot_fields[{field_key}].option.id")
            for option in require_array(snapshot_field.get("options", []), f"snapshot_fields[{field_key}].options")
        }
        live_options = {
            require_string(option.get("option_key"), f"live_fields[{field_key}].option.option_key"): require_string(option.get("id"), f"live_fields[{field_key}].option.id")
            for option in require_array(live_field.get("options", []), f"live_fields[{field_key}].options")
        }
        if snapshot_options != live_options:
            raise PlanError(f"project field option drifted for {field_key}")
    return snapshot_index


def index_live_issues(live_capture: dict[str, Any]) -> tuple[dict[str, dict[str, Any]], dict[str, dict[str, Any]]]:
    by_handle: dict[str, dict[str, Any]] = {}
    by_url: dict[str, dict[str, Any]] = {}
    for key in ("mesh_lang_issues", "hyperpush_issues"):
        issues = require_array(require_object(live_capture[key], key).get("issues"), f"{key}.issues")
        for issue in issues:
            issue_object = require_object(issue, f"{key}.issue")
            handle = require_string(issue_object.get("issue_handle"), f"{key}.issue.issue_handle")
            url = require_string(issue_object.get("issue_url"), f"{key}.issue.issue_url")
            if handle in by_handle:
                raise PlanError(f"duplicate live issue handle {handle}")
            if url in by_url:
                raise PlanError(f"duplicate live issue url {url}")
            by_handle[handle] = issue_object
            by_url[url] = issue_object
    return by_handle, by_url


def index_live_project_items(live_capture: dict[str, Any]) -> tuple[dict[str, dict[str, Any]], dict[str, dict[str, Any]]]:
    items_payload = require_object(live_capture.get("project_items"), "live_capture.project_items")
    rollup = require_object(items_payload.get("rollup"), "project_items.rollup")
    total_items = require_int(rollup.get("total_items"), "project_items.rollup.total_items")
    if total_items != EXPECTED_CURRENT_PROJECT_TOTAL:
        raise PlanError(
            f"live project item total drifted: expected {EXPECTED_CURRENT_PROJECT_TOTAL}, found {total_items}"
        )
    repo_counts = require_object(rollup.get("repo_counts"), "project_items.rollup.repo_counts")
    for repo_slug, expected in EXPECTED_CURRENT_PROJECT_REPO_COUNTS.items():
        actual = require_int(repo_counts.get(repo_slug), f"project_items.rollup.repo_counts[{repo_slug!r}]")
        if actual != expected:
            raise PlanError(f"live project repo count drifted for {repo_slug}: expected {expected}, found {actual}")

    by_handle: dict[str, dict[str, Any]] = {}
    by_url: dict[str, dict[str, Any]] = {}
    seen_item_ids: set[str] = set()
    for item in require_array(items_payload.get("items"), "project_items.items"):
        item_object = require_object(item, "project_items.item")
        project_item_id = require_string(item_object.get("project_item_id"), "project_items.item.project_item_id")
        if project_item_id in seen_item_ids:
            raise PlanError(f"duplicate live project item id {project_item_id}")
        seen_item_ids.add(project_item_id)
        url = require_string(item_object.get("canonical_issue_url"), "project_items.item.canonical_issue_url")
        issue = require_object(item_object.get("issue"), "project_items.item.issue")
        handle = require_string(issue.get("issue_handle"), "project_items.item.issue.issue_handle")
        if url in by_url:
            raise PlanError(f"duplicate live project canonical issue url {url}")
        if handle in by_handle:
            raise PlanError(f"duplicate live project issue handle {handle}")
        by_url[url] = item_object
        by_handle[handle] = item_object
    return by_handle, by_url


def validate_live_repo_totals(*, live_capture: dict[str, Any], preflight_record: dict[str, Any]) -> dict[str, Any]:
    parsed = preflight_record.get("parsed")
    if not isinstance(parsed, dict):
        raise PlanError("successful preflight did not expose parsed repo_totals")
    repo_totals = require_object(parsed.get("repo_totals"), "preflight.parsed.repo_totals")
    mesh_total = require_int(require_object(repo_totals.get("mesh_lang"), "preflight.repo_totals.mesh_lang").get("total"), "preflight.repo_totals.mesh_lang.total")
    hyperpush_total = require_int(require_object(repo_totals.get("hyperpush"), "preflight.repo_totals.hyperpush").get("total"), "preflight.repo_totals.hyperpush.total")
    combined_total = require_int(repo_totals.get("combined_total"), "preflight.repo_totals.combined_total")
    if mesh_total != EXPECTED_REPO_TOTALS[MESH_LANG_REPO] or hyperpush_total != EXPECTED_REPO_TOTALS[HYPERPUSH_REPO] or combined_total != EXPECTED_REPO_TOTALS["combined"]:
        raise PlanError(
            f"preflight repo totals drifted: mesh={mesh_total} hyperpush={hyperpush_total} combined={combined_total}"
        )

    issue_counts = {
        MESH_LANG_REPO: len(require_array(require_object(live_capture.get("mesh_lang_issues"), "mesh_lang_issues").get("issues"), "mesh_lang_issues.issues")),
        HYPERPUSH_REPO: len(require_array(require_object(live_capture.get("hyperpush_issues"), "hyperpush_issues").get("issues"), "hyperpush_issues.issues")),
    }
    if issue_counts[MESH_LANG_REPO] != mesh_total or issue_counts[HYPERPUSH_REPO] != hyperpush_total:
        raise PlanError(
            "contradictory repo totals between retained S02 verifier and live issue capture"
        )
    if issue_counts[MESH_LANG_REPO] + issue_counts[HYPERPUSH_REPO] != combined_total:
        raise PlanError("contradictory combined repo total between retained S02 verifier and live issue capture")
    return {
        "mesh_lang_total": mesh_total,
        "hyperpush_total": hyperpush_total,
        "combined_total": combined_total,
    }


def extract_canonical_mappings(results: dict[str, Any]) -> dict[str, dict[str, Any]]:
    operations = [require_object(operation, "s02.operation") for operation in require_array(results.get("operations"), "s02.operations")]
    by_id = {
        require_string(operation.get("operation_id"), "s02.operation.operation_id"): operation
        for operation in operations
    }
    transfer = by_id.get("transfer-hyperpush-8")
    create = by_id.get("create-pitch-retrospective-issue")
    if transfer is None:
        raise PlanError("missing canonical transfer mapping for hyperpush#8")
    if create is None:
        raise PlanError("missing canonical create mapping for /pitch")

    transfer_final = require_object(transfer.get("final_state"), "transfer.final_state")
    create_final = require_object(create.get("final_state"), "create.final_state")

    if require_string(transfer_final.get("issue_handle"), "transfer.final_state.issue_handle") != "mesh-lang#19":
        raise PlanError("missing canonical mapping for mesh-lang#19")
    if require_string(create_final.get("issue_handle"), "create.final_state.issue_handle") != "hyperpush#58":
        raise PlanError("missing canonical mapping for hyperpush#58")

    return {
        "transfer_hyperpush_8": {
            "source_handle": "hyperpush#8",
            "source_url": "https://github.com/hyperpush-org/hyperpush/issues/8",
            "destination": transfer_final,
            "plan_action": "add_replacement_project_item",
        },
        "create_pitch_route": {
            "gap_id": "product_pitch_route_shipped_without_tracker_row",
            "destination": create_final,
            "plan_action": "add_replacement_project_item",
        },
    }


def parse_parent_handle(issue: dict[str, Any]) -> str | None:
    body = require_string(issue.get("body"), f"{issue.get('issue_handle')}.body")
    match = PARENT_SECTION_RE.search(body)
    if match is None:
        return None
    return f"hyperpush#{match.group(1)}"


def build_parent_index(issue_by_handle: dict[str, dict[str, Any]]) -> dict[str, str]:
    parent_index: dict[str, str] = {}
    for handle, issue in issue_by_handle.items():
        parent_handle = parse_parent_handle(issue)
        if parent_handle is not None:
            parent_index[handle] = parent_handle
    return parent_index


def make_option_lookup(snapshot_field_index: dict[str, dict[str, Any]]) -> dict[str, dict[str, str]]:
    option_lookup: dict[str, dict[str, str]] = {}
    for field_key, definition in snapshot_field_index.items():
        options = {
            require_string(option.get("name"), f"{field_key}.option.name"): require_string(option.get("id"), f"{field_key}.option.id")
            for option in require_array(definition.get("options", []), f"{field_key}.options")
        }
        option_lookup[field_key] = options
    return option_lookup


def desired_field_value(
    *,
    snapshot_field: dict[str, Any],
    option_lookup: dict[str, dict[str, str]],
    field_key: str,
    value: Any,
    value_source: str,
) -> dict[str, Any]:
    normalized_value = normalize_scalar(value)
    option_id = None
    if normalized_value is not None:
        options = option_lookup.get(field_key, {})
        if options:
            option_id = options.get(str(normalized_value))
            if option_id is None:
                raise PlanError(f"unknown field option for {field_key}: {normalized_value!r}")
    return {
        "field_id": require_string(snapshot_field.get("field_id"), f"snapshot_field[{field_key}].field_id"),
        "field_name": require_string(snapshot_field.get("field_name"), f"snapshot_field[{field_key}].field_name"),
        "field_key": field_key,
        "field_type": require_string(snapshot_field.get("field_type"), f"snapshot_field[{field_key}].field_type"),
        "value": normalized_value,
        "option_id": option_id,
        "value_source": value_source,
    }


def infer_status_for_row(*, issue_state: str, current_status: str | None, existing_item: bool) -> tuple[str, str]:
    if issue_state == "CLOSED":
        return "Done", "live_issue_state"
    if current_status in {"Todo", "In Progress"}:
        return current_status, "live_project_status"
    if current_status == "Done":
        return "Todo", "open_issue_cannot_remain_done"
    if existing_item:
        return "Todo", "open_issue_default"
    return "Todo", "new_open_issue_default"


def priority_from_labels(labels: list[str]) -> tuple[str | None, str | None]:
    for label in labels:
        mapped = PRIORITY_LABEL_TO_OPTION.get(label)
        if mapped is not None:
            return mapped, f"live_issue_label:{label}"
    return None, None


class InheritanceResolver:
    def __init__(self, *, live_items_by_handle: dict[str, dict[str, Any]], parent_index: dict[str, str]):
        self.live_items_by_handle = live_items_by_handle
        self.parent_index = parent_index
        self.cache: dict[tuple[str, str], dict[str, Any]] = {}

    def resolve(self, handle: str, field_key: str, *, seen: set[str] | None = None) -> dict[str, Any]:
        cache_key = (handle, field_key)
        if cache_key in self.cache:
            return self.cache[cache_key]
        if seen is None:
            seen = set()
        if handle in seen:
            raise PlanError(f"cyclic inheritance detected while resolving {field_key} for {handle}")
        seen.add(handle)

        live_item = self.live_items_by_handle.get(handle)
        if live_item is None:
            raise PlanError(f"missing project item for inheritance source {handle}")
        live_field = require_object(live_item.get("field_values"), f"{handle}.field_values").get(field_key)
        if not isinstance(live_field, dict):
            raise PlanError(f"missing tracked field {field_key!r} on live project item {handle}")
        live_value = normalize_scalar(live_field.get("value"))
        if live_value is not None:
            resolved = {
                "value": live_value,
                "source_handle": handle,
                "chain": [handle],
                "source_value_type": live_field.get("value_type"),
            }
            self.cache[cache_key] = resolved
            return resolved

        parent_handle = self.parent_index.get(handle)
        if parent_handle is None:
            raise PlanError(f"broken inheritance source definition for {handle}: no parent issue section resolves {field_key}")
        parent_item = self.live_items_by_handle.get(parent_handle)
        if parent_item is None:
            raise PlanError(f"broken inheritance source definition for {handle}: parent {parent_handle} is not on the live project board")
        parent_result = self.resolve(parent_handle, field_key, seen=seen)
        resolved = {
            "value": parent_result["value"],
            "source_handle": parent_result["source_handle"],
            "chain": [handle, *parent_result["chain"]],
            "source_value_type": parent_result.get("source_value_type"),
        }
        self.cache[cache_key] = resolved
        return resolved


def build_desired_existing_row(
    *,
    ledger_row: dict[str, Any],
    live_issue: dict[str, Any],
    live_item: dict[str, Any],
    snapshot_field_index: dict[str, dict[str, Any]],
    option_lookup: dict[str, dict[str, str]],
    resolver: InheritanceResolver,
) -> dict[str, Any]:
    field_values = require_object(live_item.get("field_values"), "live_item.field_values")
    desired_fields: dict[str, dict[str, Any]] = {}
    inheritance_sources: dict[str, dict[str, Any]] = {}

    current_status = normalize_scalar(require_object(field_values.get("status"), "live_item.field_values.status").get("value"))
    desired_status, status_source = infer_status_for_row(
        issue_state=require_string(live_issue.get("state"), "live_issue.state"),
        current_status=current_status,
        existing_item=True,
    )

    for field_key in TRACKED_FIELD_KEYS:
        snapshot_field = snapshot_field_index[field_key]
        if field_key == "title":
            desired_fields[field_key] = desired_field_value(
                snapshot_field=snapshot_field,
                option_lookup=option_lookup,
                field_key=field_key,
                value=require_string(live_issue.get("title"), "live_issue.title"),
                value_source="live_issue_title",
            )
            continue
        if field_key == "status":
            desired_fields[field_key] = desired_field_value(
                snapshot_field=snapshot_field,
                option_lookup=option_lookup,
                field_key=field_key,
                value=desired_status,
                value_source=status_source,
            )
            continue

        current_field = require_object(field_values.get(field_key), f"live_item.field_values[{field_key!r}]")
        current_value = normalize_scalar(current_field.get("value"))
        if current_value is not None:
            desired_fields[field_key] = desired_field_value(
                snapshot_field=snapshot_field,
                option_lookup=option_lookup,
                field_key=field_key,
                value=current_value,
                value_source="live_project_field",
            )
            continue

        if field_key in INHERITED_FIELD_KEYS and repo_issue_handle(require_string(live_issue.get("repo"), "live_issue.repo"), require_int(live_issue.get("number"), "live_issue.number")) != "hyperpush#24":
            resolved = resolver.resolve(require_string(live_issue.get("issue_handle"), "live_issue.issue_handle"), field_key)
            desired_fields[field_key] = desired_field_value(
                snapshot_field=snapshot_field,
                option_lookup=option_lookup,
                field_key=field_key,
                value=resolved["value"],
                value_source="parent_chain_inheritance",
            )
            inheritance_sources[field_key] = resolved
            continue

        desired_fields[field_key] = desired_field_value(
            snapshot_field=snapshot_field,
            option_lookup=option_lookup,
            field_key=field_key,
            value=None,
            value_source="no_deterministic_source",
        )

    desired_row = {
        "issue_handle": require_string(live_issue.get("issue_handle"), "live_issue.issue_handle"),
        "issue_url": require_string(live_issue.get("issue_url"), "live_issue.issue_url"),
        "repo": require_string(live_issue.get("repo"), "live_issue.repo"),
        "number": require_int(live_issue.get("number"), "live_issue.number"),
        "issue_state": require_string(live_issue.get("state"), "live_issue.state"),
        "project_item_id": require_string(live_item.get("project_item_id"), "live_item.project_item_id"),
        "field_values": desired_fields,
        "source_kind": require_string(ledger_row.get("proposed_project_action_kind"), "ledger_row.proposed_project_action_kind"),
        "inheritance_sources": inheritance_sources,
    }
    return desired_row


def build_transfer_add_row(
    *,
    live_issue: dict[str, Any],
    snapshot_field_index: dict[str, dict[str, Any]],
    option_lookup: dict[str, dict[str, str]],
    ledger_row: dict[str, Any],
) -> dict[str, Any]:
    labels = require_array(live_issue.get("labels", []), "live_issue.labels")
    priority_value, priority_source = priority_from_labels([require_string(label, "live_issue.label") for label in labels])
    desired_status, status_source = infer_status_for_row(
        issue_state=require_string(live_issue.get("state"), "live_issue.state"),
        current_status=None,
        existing_item=False,
    )
    field_values = {
        "title": desired_field_value(
            snapshot_field=snapshot_field_index["title"],
            option_lookup=option_lookup,
            field_key="title",
            value=require_string(live_issue.get("title"), "live_issue.title"),
            value_source="canonical_transferred_issue_title",
        ),
        "status": desired_field_value(
            snapshot_field=snapshot_field_index["status"],
            option_lookup=option_lookup,
            field_key="status",
            value=desired_status,
            value_source=status_source,
        ),
        "domain": desired_field_value(
            snapshot_field=snapshot_field_index["domain"],
            option_lookup=option_lookup,
            field_key="domain",
            value="Mesh",
            value_source="ledger_replacement_mesh_domain",
        ),
        "track": desired_field_value(
            snapshot_field=snapshot_field_index["track"],
            option_lookup=option_lookup,
            field_key="track",
            value=None,
            value_source="no_deterministic_source",
        ),
        "commitment": desired_field_value(
            snapshot_field=snapshot_field_index["commitment"],
            option_lookup=option_lookup,
            field_key="commitment",
            value=None,
            value_source="no_deterministic_source",
        ),
        "delivery_mode": desired_field_value(
            snapshot_field=snapshot_field_index["delivery_mode"],
            option_lookup=option_lookup,
            field_key="delivery_mode",
            value=None,
            value_source="no_deterministic_source",
        ),
        "priority": desired_field_value(
            snapshot_field=snapshot_field_index["priority"],
            option_lookup=option_lookup,
            field_key="priority",
            value=priority_value,
            value_source=priority_source or "no_deterministic_source",
        ),
        "start_date": desired_field_value(
            snapshot_field=snapshot_field_index["start_date"],
            option_lookup=option_lookup,
            field_key="start_date",
            value=None,
            value_source="no_deterministic_source",
        ),
        "target_date": desired_field_value(
            snapshot_field=snapshot_field_index["target_date"],
            option_lookup=option_lookup,
            field_key="target_date",
            value=None,
            value_source="no_deterministic_source",
        ),
        "hackathon_phase": desired_field_value(
            snapshot_field=snapshot_field_index["hackathon_phase"],
            option_lookup=option_lookup,
            field_key="hackathon_phase",
            value=None,
            value_source="no_deterministic_source",
        ),
    }
    return {
        "issue_handle": require_string(live_issue.get("issue_handle"), "live_issue.issue_handle"),
        "issue_url": require_string(live_issue.get("issue_url"), "live_issue.issue_url"),
        "repo": require_string(live_issue.get("repo"), "live_issue.repo"),
        "number": require_int(live_issue.get("number"), "live_issue.number"),
        "issue_state": require_string(live_issue.get("state"), "live_issue.state"),
        "project_item_id": None,
        "field_values": field_values,
        "source_kind": require_string(ledger_row.get("proposed_project_action_kind"), "ledger_row.proposed_project_action_kind"),
        "inheritance_sources": {},
    }


def build_pitch_add_row(
    *,
    live_issue: dict[str, Any],
    snapshot_field_index: dict[str, dict[str, Any]],
    option_lookup: dict[str, dict[str, str]],
) -> dict[str, Any]:
    desired_status, status_source = infer_status_for_row(
        issue_state=require_string(live_issue.get("state"), "live_issue.state"),
        current_status=None,
        existing_item=False,
    )
    field_values = {
        "title": desired_field_value(
            snapshot_field=snapshot_field_index["title"],
            option_lookup=option_lookup,
            field_key="title",
            value=require_string(live_issue.get("title"), "live_issue.title"),
            value_source="canonical_created_issue_title",
        ),
        "status": desired_field_value(
            snapshot_field=snapshot_field_index["status"],
            option_lookup=option_lookup,
            field_key="status",
            value=desired_status,
            value_source=status_source,
        ),
        "domain": desired_field_value(
            snapshot_field=snapshot_field_index["domain"],
            option_lookup=option_lookup,
            field_key="domain",
            value="Hyperpush",
            value_source="derived_gap_public_repo_truth",
        ),
        "track": desired_field_value(
            snapshot_field=snapshot_field_index["track"],
            option_lookup=option_lookup,
            field_key="track",
            value=None,
            value_source="no_deterministic_source",
        ),
        "commitment": desired_field_value(
            snapshot_field=snapshot_field_index["commitment"],
            option_lookup=option_lookup,
            field_key="commitment",
            value=None,
            value_source="no_deterministic_source",
        ),
        "delivery_mode": desired_field_value(
            snapshot_field=snapshot_field_index["delivery_mode"],
            option_lookup=option_lookup,
            field_key="delivery_mode",
            value=None,
            value_source="no_deterministic_source",
        ),
        "priority": desired_field_value(
            snapshot_field=snapshot_field_index["priority"],
            option_lookup=option_lookup,
            field_key="priority",
            value=None,
            value_source="no_deterministic_source",
        ),
        "start_date": desired_field_value(
            snapshot_field=snapshot_field_index["start_date"],
            option_lookup=option_lookup,
            field_key="start_date",
            value=None,
            value_source="no_deterministic_source",
        ),
        "target_date": desired_field_value(
            snapshot_field=snapshot_field_index["target_date"],
            option_lookup=option_lookup,
            field_key="target_date",
            value=None,
            value_source="no_deterministic_source",
        ),
        "hackathon_phase": desired_field_value(
            snapshot_field=snapshot_field_index["hackathon_phase"],
            option_lookup=option_lookup,
            field_key="hackathon_phase",
            value=None,
            value_source="no_deterministic_source",
        ),
    }
    return {
        "issue_handle": require_string(live_issue.get("issue_handle"), "live_issue.issue_handle"),
        "issue_url": require_string(live_issue.get("issue_url"), "live_issue.issue_url"),
        "repo": require_string(live_issue.get("repo"), "live_issue.repo"),
        "number": require_int(live_issue.get("number"), "live_issue.number"),
        "issue_state": require_string(live_issue.get("state"), "live_issue.state"),
        "project_item_id": None,
        "field_values": field_values,
        "source_kind": "derived_gap_create_project_item",
        "inheritance_sources": {},
    }


def current_row_snapshot(live_item: dict[str, Any]) -> dict[str, Any]:
    field_values = require_object(live_item.get("field_values"), "live_item.field_values")
    return {
        "project_item_id": require_string(live_item.get("project_item_id"), "live_item.project_item_id"),
        "issue_handle": require_string(require_object(live_item.get("issue"), "live_item.issue").get("issue_handle"), "live_item.issue.issue_handle"),
        "issue_url": require_string(live_item.get("canonical_issue_url"), "live_item.canonical_issue_url"),
        "field_values": {
            field_key: field_value_summary(require_object(field_values.get(field_key), f"live_item.field_values[{field_key!r}]"))
            for field_key in TRACKED_FIELD_KEYS
        },
    }


def field_change(before: dict[str, Any], after: dict[str, Any], *, inheritance_sources: dict[str, dict[str, Any]]) -> dict[str, Any] | None:
    before_value = normalize_scalar(before.get("value"))
    after_value = normalize_scalar(after.get("value"))
    before_option_id = before.get("option_id")
    after_option_id = after.get("option_id")
    if before_value == after_value and before_option_id == after_option_id:
        return None
    field_key = require_string(after.get("field_key"), "after.field_key")
    payload = {
        "field_key": field_key,
        "field_name": require_string(after.get("field_name"), "after.field_name"),
        "before": {
            "value": before_value,
            "option_id": before_option_id,
            "value_type": before.get("value_type"),
        },
        "after": {
            "value": after_value,
            "option_id": after_option_id,
            "value_source": after.get("value_source"),
        },
        "change_reason": require_string(after.get("value_source"), "after.value_source"),
    }
    if field_key in inheritance_sources:
        payload["inheritance"] = inheritance_sources[field_key]
    return payload


def compute_update_operation(
    *,
    ledger_row: dict[str, Any],
    live_item: dict[str, Any],
    desired_row: dict[str, Any],
) -> dict[str, Any] | None:
    current_snapshot = current_row_snapshot(live_item)
    desired_fields = require_object(desired_row.get("field_values"), "desired_row.field_values")
    changes: list[dict[str, Any]] = []
    for field_key in TRACKED_FIELD_KEYS:
        current_field = require_object(current_snapshot["field_values"].get(field_key), f"current_snapshot.field_values[{field_key!r}]")
        desired_field = require_object(desired_fields.get(field_key), f"desired_row.field_values[{field_key!r}]")
        change = field_change(current_field, desired_field, inheritance_sources=require_object(desired_row.get("inheritance_sources"), "desired_row.inheritance_sources"))
        if change is not None:
            changes.append(change)
    if not changes:
        return None
    issue_handle = require_string(desired_row.get("issue_handle"), "desired_row.issue_handle")
    return {
        "operation_id": f"update-{issue_handle.replace('#', '-')}",
        "operation_kind": "update",
        "canonical_issue_handle": issue_handle,
        "canonical_issue_url": require_string(desired_row.get("issue_url"), "desired_row.issue_url"),
        "project_item_id": require_string(current_snapshot.get("project_item_id"), "current_snapshot.project_item_id"),
        "project_action_kind": require_string(ledger_row.get("proposed_project_action_kind"), "ledger_row.proposed_project_action_kind"),
        "current_row": current_snapshot,
        "final_row_state": desired_row,
        "field_changes": changes,
        "change_count": len(changes),
        "touch_reason": "inherit_missing_metadata" if any(change.get("change_reason") == "parent_chain_inheritance" for change in changes) else "align_live_project_row",
    }


def build_delete_operation(*, ledger_row: dict[str, Any], live_item: dict[str, Any]) -> dict[str, Any]:
    handle = require_string(ledger_row.get("canonical_issue_handle"), "ledger_row.canonical_issue_handle")
    return {
        "operation_id": f"delete-{handle.replace('#', '-')}",
        "operation_kind": "delete",
        "canonical_issue_handle": handle,
        "canonical_issue_url": require_string(ledger_row.get("canonical_issue_url"), "ledger_row.canonical_issue_url"),
        "project_item_id": require_string(live_item.get("project_item_id"), "live_item.project_item_id"),
        "current_row": current_row_snapshot(live_item),
        "final_row_state": None,
        "touch_reason": "remove_stale_cleanup_row",
        "ledger_reason": require_string(ledger_row.get("proposed_project_action"), "ledger_row.proposed_project_action"),
    }


def build_add_operation(*, handle: str, desired_row: dict[str, Any], touch_reason: str, note: str) -> dict[str, Any]:
    return {
        "operation_id": f"add-{handle.replace('#', '-')}",
        "operation_kind": "add",
        "canonical_issue_handle": handle,
        "canonical_issue_url": require_string(desired_row.get("issue_url"), "desired_row.issue_url"),
        "project_item_id": None,
        "current_row": None,
        "final_row_state": desired_row,
        "touch_reason": touch_reason,
        "note": note,
    }


def collect_verified_noops(
    *,
    ledger_rows_by_handle: dict[str, dict[str, Any]],
    live_items_by_handle: dict[str, dict[str, Any]],
    desired_rows_by_handle: dict[str, dict[str, Any]],
    snapshot_items_by_handle: dict[str, dict[str, Any]],
) -> list[dict[str, Any]]:
    verified: list[dict[str, Any]] = []
    for handle in sorted(desired_rows_by_handle):
        live_item = live_items_by_handle.get(handle)
        desired_row = desired_rows_by_handle[handle]
        if live_item is None:
            continue
        if compute_update_operation(
            ledger_row=ledger_rows_by_handle[handle],
            live_item=live_item,
            desired_row=desired_row,
        ) is not None:
            continue
        verification_kind = "already_satisfied"
        snapshot_item = snapshot_items_by_handle.get(handle)
        historical_snapshot = None
        if handle in EXPECTED_NAMING_HANDLES and snapshot_item is not None:
            verification_kind = "naming_normalization_preserved"
            historical_snapshot = {
                "title": require_object(snapshot_item.get("field_values"), "snapshot_item.field_values")["title"]["value"],
                "issue_url": require_string(snapshot_item.get("canonical_issue_url"), "snapshot_item.canonical_issue_url"),
            }
        verified.append(
            {
                "canonical_issue_handle": handle,
                "canonical_issue_url": require_string(desired_row.get("issue_url"), "desired_row.issue_url"),
                "project_item_id": require_string(live_item.get("project_item_id"), "live_item.project_item_id"),
                "verification_kind": verification_kind,
                "current_row": current_row_snapshot(live_item),
                "final_row_state": desired_row,
                "historical_snapshot": historical_snapshot,
            }
        )
    return verified


def build_blocked_plan(*, source_root: Path, output_dir: Path, preflight: dict[str, Any]) -> tuple[dict[str, Any], str]:
    plan = {
        "version": PLAN_VERSION,
        "generated_at": iso_now(),
        "source_script": SCRIPT_RELATIVE_PATH,
        "plan_status": "blocked_preflight",
        "source": {
            "output_dir": path_for_artifact(output_dir, root=source_root),
            "preflight_command": preflight.get("command"),
        },
        "preflight": preflight,
        "rollup": {
            "delete": 0,
            "add": 0,
            "update": 0,
            "unchanged": 0,
            "inherited_rows": 0,
            "current_project_items": None,
            "desired_project_items": None,
        },
        "operations": {
            "delete": [],
            "add": [],
            "update": [],
        },
        "verified_noops": [],
        "canonical_mapping_handling": {},
        "live_capture": None,
    }
    markdown = "\n".join(
        [
            "# M057 S03 Project Mutation Plan",
            "",
            "- Plan status: `blocked_preflight`",
            f"- Generated at: `{plan['generated_at']}`",
            f"- Preflight status: `{preflight.get('status')}`",
            f"- Preflight exit code: `{preflight.get('exit_code')}`",
            "",
            "## Why planning stopped",
            "",
            "The retained S02 verifier did not pass, so S03 refused to derive board mutations from stale or contradictory repo truth.",
            "",
            "## Preflight diagnostics",
            "",
            "```text",
            str(preflight.get("stdout") or ""),
            str(preflight.get("stderr") or ""),
            "```",
            "",
        ]
    )
    return plan, markdown


def build_plan(*, source_root: Path, s01_dir: Path, s02_dir: Path, output_dir: Path, live_capture: dict[str, Any], preflight: dict[str, Any]) -> tuple[dict[str, Any], str]:
    ledger = read_json(s01_dir / "reconciliation-ledger.json", "S01 reconciliation ledger")
    snapshot_fields = read_json(s01_dir / "project-fields.snapshot.json", "S01 project fields snapshot")
    snapshot_items = read_json(s01_dir / "project-items.snapshot.json", "S01 project items snapshot")
    s02_results = read_json(s02_dir / "repo-mutation-results.json", "S02 repo mutation results")

    snapshot_field_index = validate_field_schema(
        snapshot_fields=snapshot_fields,
        live_fields=require_object(live_capture.get("project_fields"), "live_capture.project_fields"),
    )
    option_lookup = make_option_lookup(snapshot_field_index)
    repo_totals = validate_live_repo_totals(live_capture=live_capture, preflight_record=preflight)
    issue_by_handle, issue_by_url = index_live_issues(live_capture)
    live_items_by_handle, live_items_by_url = index_live_project_items(live_capture)
    parent_index = build_parent_index(issue_by_handle)
    resolver = InheritanceResolver(live_items_by_handle=live_items_by_handle, parent_index=parent_index)
    canonical_mappings = extract_canonical_mappings(s02_results)

    ledger_rows = [require_object(row, "ledger.row") for row in require_array(ledger.get("rows"), "ledger.rows")]
    ledger_rows_by_handle = {
        require_string(row.get("canonical_issue_handle"), "ledger.row.canonical_issue_handle"): row
        for row in ledger_rows
    }
    snapshot_items_by_handle = {
        require_string(require_object(item.get("issue"), "snapshot_item.issue").get("repo"), "snapshot_item.issue.repo").split("/")[-1]
        + "#"
        + str(require_int(require_object(item.get("issue"), "snapshot_item.issue").get("number"), "snapshot_item.issue.number")): require_object(item, "snapshot_item")
        for item in require_array(snapshot_items.get("items"), "snapshot_items.items")
    }

    delete_ops: list[dict[str, Any]] = []
    update_ops: list[dict[str, Any]] = []
    desired_rows_by_handle: dict[str, dict[str, Any]] = {}

    for row in ledger_rows:
        handle = require_string(row.get("canonical_issue_handle"), "ledger.row.canonical_issue_handle")
        project_action_kind = require_string(row.get("proposed_project_action_kind"), f"{handle}.proposed_project_action_kind")
        project_backed = require_bool(row.get("project_backed"), f"{handle}.project_backed")
        live_item = live_items_by_handle.get(handle)
        live_issue = issue_by_handle.get(handle)
        if live_issue is None and handle != "hyperpush#8":
            raise PlanError(f"missing live repo issue row for {handle}")

        if project_backed and live_item is None:
            raise PlanError(f"missing live project item for board-backed row {handle}")
        if project_action_kind == "remove_from_project":
            if live_item is None:
                raise PlanError(f"expected stale cleanup row {handle} to still be present on the live board")
            delete_ops.append(build_delete_operation(ledger_row=row, live_item=live_item))
            continue
        if project_action_kind in {"keep_in_project", "update_project_item"}:
            if live_item is None or live_issue is None:
                raise PlanError(f"cannot build desired retained row for {handle}")
            desired_row = build_desired_existing_row(
                ledger_row=row,
                live_issue=live_issue,
                live_item=live_item,
                snapshot_field_index=snapshot_field_index,
                option_lookup=option_lookup,
                resolver=resolver,
            )
            desired_rows_by_handle[handle] = desired_row
            update_operation = compute_update_operation(ledger_row=row, live_item=live_item, desired_row=desired_row)
            if update_operation is not None:
                update_ops.append(update_operation)
            continue
        if project_action_kind == "create_project_item":
            continue
        raise PlanError(f"unexpected project action kind for {handle}: {project_action_kind}")

    transfer_mapping = canonical_mappings["transfer_hyperpush_8"]
    transfer_destination = require_object(transfer_mapping.get("destination"), "transfer_mapping.destination")
    transfer_handle = require_string(transfer_destination.get("issue_handle"), "transfer_mapping.destination.issue_handle")
    transfer_issue = issue_by_handle.get(transfer_handle)
    if transfer_issue is None:
        raise PlanError(f"canonical transferred issue {transfer_handle} is missing from the live repo capture")
    if live_items_by_handle.get("hyperpush#8") is not None:
        raise PlanError("stale hyperpush#8 project row unexpectedly exists on the live board")
    transfer_add_row = build_transfer_add_row(
        live_issue=transfer_issue,
        snapshot_field_index=snapshot_field_index,
        option_lookup=option_lookup,
        ledger_row=require_object(ledger_rows_by_handle.get("hyperpush#8"), "ledger_rows_by_handle['hyperpush#8']"),
    )

    pitch_destination = require_object(canonical_mappings["create_pitch_route"].get("destination"), "pitch_mapping.destination")
    pitch_handle = require_string(pitch_destination.get("issue_handle"), "pitch_mapping.destination.issue_handle")
    pitch_issue = issue_by_handle.get(pitch_handle)
    if pitch_issue is None:
        raise PlanError(f"canonical created issue {pitch_handle} is missing from the live repo capture")
    pitch_add_row = build_pitch_add_row(
        live_issue=pitch_issue,
        snapshot_field_index=snapshot_field_index,
        option_lookup=option_lookup,
    )

    add_ops = [
        build_add_operation(
            handle=transfer_handle,
            desired_row=transfer_add_row,
            touch_reason="add_canonical_replacement_row",
            note="S02 transferred hyperpush#8 into mesh-lang#19; S03 adds the replacement board row under the canonical mesh issue identity.",
        ),
        build_add_operation(
            handle=pitch_handle,
            desired_row=pitch_add_row,
            touch_reason="add_missing_tracker_coverage_row",
            note="S02 created hyperpush#58 for the shipped /pitch route; S03 adds the missing board row for the canonical retrospective issue.",
        ),
    ]

    verified_noops = collect_verified_noops(
        ledger_rows_by_handle=ledger_rows_by_handle,
        live_items_by_handle=live_items_by_handle,
        desired_rows_by_handle=desired_rows_by_handle,
        snapshot_items_by_handle=snapshot_items_by_handle,
    )

    inheritance_rows = [operation for operation in update_ops if operation.get("touch_reason") == "inherit_missing_metadata"]
    inheritance_rollup = {
        "rows": len(inheritance_rows),
        "field_change_count": sum(require_int(operation.get("change_count"), "operation.change_count") for operation in inheritance_rows),
        "deepest_chain_length": max(
            (
                len(require_object(change.get("inheritance"), "change.inheritance").get("chain"))
                for operation in inheritance_rows
                for change in require_array(operation.get("field_changes"), "operation.field_changes")
                if isinstance(change, dict) and change.get("inheritance") is not None
            ),
            default=0,
        ),
    }

    live_final_status_counts = {"Todo": 0, "In Progress": 0, "Done": 0}
    for desired_row in [*desired_rows_by_handle.values(), transfer_add_row, pitch_add_row]:
        status = normalize_scalar(require_object(require_object(desired_row.get("field_values"), "desired_row.field_values").get("status"), "desired_row.field_values.status").get("value"))
        if status in live_final_status_counts:
            live_final_status_counts[status] += 1

    rollup = {
        "delete": len(delete_ops),
        "add": len(add_ops),
        "update": len(update_ops),
        "unchanged": len(verified_noops),
        "inherited_rows": len(inheritance_rows),
        "current_project_items": EXPECTED_CURRENT_PROJECT_TOTAL,
        "desired_project_items": len(desired_rows_by_handle) + len(add_ops),
        "final_status_counts": live_final_status_counts,
        "repo_totals": repo_totals,
    }

    plan = {
        "version": PLAN_VERSION,
        "generated_at": iso_now(),
        "source_script": SCRIPT_RELATIVE_PATH,
        "plan_status": "ready",
        "source": {
            "s01_dir": path_for_artifact(s01_dir, root=source_root),
            "s02_dir": path_for_artifact(s02_dir, root=source_root),
            "output_dir": path_for_artifact(output_dir, root=source_root),
            "s01_ledger": path_for_artifact(s01_dir / "reconciliation-ledger.json", root=source_root),
            "s01_project_fields_snapshot": path_for_artifact(s01_dir / "project-fields.snapshot.json", root=source_root),
            "s01_project_items_snapshot": path_for_artifact(s01_dir / "project-items.snapshot.json", root=source_root),
            "s02_results": path_for_artifact(s02_dir / "repo-mutation-results.json", root=source_root),
        },
        "preflight": preflight,
        "field_schema": {
            "project": require_object(require_object(live_capture.get("project_fields"), "live_capture.project_fields").get("project"), "live_capture.project_fields.project"),
            "tracked_fields": snapshot_field_index,
        },
        "live_capture": live_capture,
        "canonical_mapping_handling": {
            "hyperpush_8_to_mesh_lang_19": {
                "source_issue_handle": "hyperpush#8",
                "destination_issue_handle": transfer_handle,
                "destination_issue_url": require_string(transfer_add_row.get("issue_url"), "transfer_add_row.issue_url"),
                "source_board_membership": "absent",
                "destination_board_membership": "missing_add_required",
                "planned_operation_id": f"add-{transfer_handle.replace('#', '-')}",
                "board_policy": "replacement_mesh_row_must_exist",
            },
            "pitch_gap_to_hyperpush_58": {
                "gap_id": "product_pitch_route_shipped_without_tracker_row",
                "destination_issue_handle": pitch_handle,
                "destination_issue_url": require_string(pitch_add_row.get("issue_url"), "pitch_add_row.issue_url"),
                "destination_board_membership": "missing_add_required",
                "planned_operation_id": f"add-{pitch_handle.replace('#', '-')}",
                "board_policy": "replacement_hyperpush_row_must_exist",
            },
        },
        "rollup": rollup,
        "inheritance_rollup": inheritance_rollup,
        "operations": {
            "delete": sorted(delete_ops, key=lambda operation: require_string(operation.get("canonical_issue_handle"), "delete.canonical_issue_handle")),
            "add": sorted(add_ops, key=lambda operation: require_string(operation.get("canonical_issue_handle"), "add.canonical_issue_handle")),
            "update": sorted(update_ops, key=lambda operation: require_string(operation.get("canonical_issue_handle"), "update.canonical_issue_handle")),
        },
        "verified_noops": sorted(verified_noops, key=lambda row: require_string(row.get("canonical_issue_handle"), "verified_noop.canonical_issue_handle")),
    }

    markdown = render_plan_markdown(plan)
    return plan, markdown


def validate_plan(plan: dict[str, Any]) -> dict[str, Any]:
    status = require_string(plan.get("plan_status"), "plan.plan_status")
    if status == "blocked_preflight":
        preflight = require_object(plan.get("preflight"), "plan.preflight")
        if require_string(preflight.get("status"), "plan.preflight.status") == "ok":
            raise PlanError("blocked_preflight artifact cannot report a green preflight")
        return {"plan_status": status}

    if status != "ready":
        raise PlanError(f"unknown S03 plan_status {status!r}")

    rollup = require_object(plan.get("rollup"), "plan.rollup")
    for key, expected in EXPECTED_ROLLUP.items():
        actual = require_int(rollup.get(key), f"plan.rollup[{key!r}]")
        if actual != expected:
            raise PlanError(f"plan rollup drifted for {key}: expected {expected}, found {actual}")
    if require_int(rollup.get("current_project_items"), "plan.rollup.current_project_items") != EXPECTED_CURRENT_PROJECT_TOTAL:
        raise PlanError("current_project_items drifted")

    operations = require_object(plan.get("operations"), "plan.operations")
    delete_ops = [require_object(operation, "plan.operations.delete[]") for operation in require_array(operations.get("delete"), "plan.operations.delete")]
    add_ops = [require_object(operation, "plan.operations.add[]") for operation in require_array(operations.get("add"), "plan.operations.add")]
    update_ops = [require_object(operation, "plan.operations.update[]") for operation in require_array(operations.get("update"), "plan.operations.update")]

    delete_handles = {require_string(operation.get("canonical_issue_handle"), "delete_op.canonical_issue_handle") for operation in delete_ops}
    add_handles = {require_string(operation.get("canonical_issue_handle"), "add_op.canonical_issue_handle") for operation in add_ops}
    if delete_handles != EXPECTED_DELETE_HANDLES:
        raise PlanError(f"delete handle set drifted: {sorted(delete_handles)!r}")
    if add_handles != EXPECTED_ADD_HANDLES:
        raise PlanError(f"add handle set drifted: {sorted(add_handles)!r}")

    inheritance_handles = {
        require_string(operation.get("canonical_issue_handle"), "update_op.canonical_issue_handle")
        for operation in update_ops
        if require_string(operation.get("touch_reason"), "update_op.touch_reason") == "inherit_missing_metadata"
    }
    if len(inheritance_handles) != EXPECTED_ROLLUP["inherited_rows"]:
        raise PlanError("inheritance row count drifted")

    canonical_mapping = require_object(plan.get("canonical_mapping_handling"), "plan.canonical_mapping_handling")
    transfer_mapping = require_object(canonical_mapping.get("hyperpush_8_to_mesh_lang_19"), "canonical_mapping.transfer")
    pitch_mapping = require_object(canonical_mapping.get("pitch_gap_to_hyperpush_58"), "canonical_mapping.pitch")
    if require_string(transfer_mapping.get("destination_issue_handle"), "canonical_mapping.transfer.destination_issue_handle") != "mesh-lang#19":
        raise PlanError("missing canonical mesh-lang#19 mapping")
    if require_string(pitch_mapping.get("destination_issue_handle"), "canonical_mapping.pitch.destination_issue_handle") != "hyperpush#58":
        raise PlanError("missing canonical hyperpush#58 mapping")

    verified_noops = [require_object(row, "plan.verified_noops[]") for row in require_array(plan.get("verified_noops"), "plan.verified_noops")]
    noop_lookup = {
        require_string(row.get("canonical_issue_handle"), "verified_noop.canonical_issue_handle"): row
        for row in verified_noops
    }
    for handle in EXPECTED_NAMING_HANDLES:
        row = noop_lookup.get(handle)
        if row is None:
            raise PlanError(f"missing verified noop for naming-normalized row {handle}")
        if require_string(row.get("verification_kind"), f"{handle}.verification_kind") != "naming_normalization_preserved":
            raise PlanError(f"expected naming_normalization_preserved verification for {handle}")

    update_57 = next(
        (operation for operation in update_ops if require_string(operation.get("canonical_issue_handle"), "update_op.canonical_issue_handle") == "hyperpush#57"),
        None,
    )
    if update_57 is None:
        raise PlanError("missing update operation for hyperpush#57")
    change_lookup = {
        require_string(change.get("field_key"), "update_57.change.field_key"): change
        for change in require_array(update_57.get("field_changes"), "update_57.field_changes")
    }
    chain = require_object(change_lookup["track"].get("inheritance"), "update_57.track.inheritance").get("chain")
    if chain != ["hyperpush#57", "hyperpush#34", "hyperpush#15"]:
        raise PlanError(f"hyperpush#57 inheritance chain drifted: {chain!r}")

    return {
        "plan_status": status,
        "delete": len(delete_ops),
        "add": len(add_ops),
        "update": len(update_ops),
        "verified_noops": len(verified_noops),
    }


def render_field_change_summary(change: dict[str, Any]) -> str:
    field_key = require_string(change.get("field_key"), "change.field_key")
    before = require_object(change.get("before"), "change.before").get("value")
    after = require_object(change.get("after"), "change.after").get("value")
    reason = require_string(change.get("change_reason"), "change.change_reason")
    suffix = ""
    if reason == "parent_chain_inheritance":
        inheritance = require_object(change.get("inheritance"), "change.inheritance")
        suffix = f" (from {' -> '.join(require_array(inheritance.get('chain'), 'change.inheritance.chain'))})"
    return f"`{field_key}`: {before!r} -> {after!r}{suffix}"


def render_plan_markdown(plan: dict[str, Any]) -> str:
    if require_string(plan.get("plan_status"), "plan.plan_status") == "blocked_preflight":
        return "\n".join(
            [
                "# M057 S03 Project Mutation Plan",
                "",
                "- Plan status: `blocked_preflight`",
                f"- Generated at: `{require_string(plan.get('generated_at'), 'plan.generated_at')}`",
                f"- Preflight status: `{require_string(require_object(plan.get('preflight'), 'plan.preflight').get('status'), 'plan.preflight.status')}`",
                "",
                "Planning stopped because the retained S02 verifier did not pass. See the JSON artifact for stdout/stderr capture.",
                "",
            ]
        )

    operations = require_object(plan.get("operations"), "plan.operations")
    delete_ops = [require_object(operation, "delete_op") for operation in require_array(operations.get("delete"), "plan.operations.delete")]
    add_ops = [require_object(operation, "add_op") for operation in require_array(operations.get("add"), "plan.operations.add")]
    update_ops = [require_object(operation, "update_op") for operation in require_array(operations.get("update"), "plan.operations.update")]
    verified_noops = [require_object(row, "verified_noop") for row in require_array(plan.get("verified_noops"), "plan.verified_noops")]
    rollup = require_object(plan.get("rollup"), "plan.rollup")
    preflight = require_object(plan.get("preflight"), "plan.preflight")
    inheritance_rollup = require_object(plan.get("inheritance_rollup"), "plan.inheritance_rollup")
    canonical_mapping = require_object(plan.get("canonical_mapping_handling"), "plan.canonical_mapping_handling")

    lines = [
        "# M057 S03 Project Mutation Plan",
        "",
        f"- Version: `{require_string(plan.get('version'), 'plan.version')}`",
        f"- Generated at: `{require_string(plan.get('generated_at'), 'plan.generated_at')}`",
        f"- Plan status: `{require_string(plan.get('plan_status'), 'plan.plan_status')}`",
        f"- Preflight status: `{require_string(preflight.get('status'), 'plan.preflight.status')}`",
        f"- Current board rows: `{require_int(rollup.get('current_project_items'), 'plan.rollup.current_project_items')}`",
        f"- Desired board rows after apply: `{require_int(rollup.get('desired_project_items'), 'plan.rollup.desired_project_items')}`",
        "",
        "## Rollup",
        "",
        "| Kind | Count |",
        "| --- | --- |",
        f"| `delete` | `{len(delete_ops)}` |",
        f"| `add` | `{len(add_ops)}` |",
        f"| `update` | `{len(update_ops)}` |",
        f"| `unchanged` | `{len(verified_noops)}` |",
        f"| `inherited_rows` | `{require_int(inheritance_rollup.get('rows'), 'inheritance_rollup.rows')}` |",
        "",
        "## Preflight evidence",
        "",
        f"- Command: `{preflight.get('command')}`",
        f"- Exit code: `{preflight.get('exit_code')}`",
        f"- Timed out: `{preflight.get('timed_out')}`",
        "",
        "## Canonical mapping handling",
        "",
        "| Source | Destination | Board policy | Planned op |",
        "| --- | --- | --- | --- |",
    ]
    transfer_mapping = require_object(canonical_mapping.get("hyperpush_8_to_mesh_lang_19"), "canonical.transfer")
    pitch_mapping = require_object(canonical_mapping.get("pitch_gap_to_hyperpush_58"), "canonical.pitch")
    lines.append(
        f"| `hyperpush#8` | `{require_string(transfer_mapping.get('destination_issue_handle'), 'canonical.transfer.destination_issue_handle')}` | `{require_string(transfer_mapping.get('board_policy'), 'canonical.transfer.board_policy')}` | `{require_string(transfer_mapping.get('planned_operation_id'), 'canonical.transfer.planned_operation_id')}` |"
    )
    lines.append(
        f"| `/pitch` gap | `{require_string(pitch_mapping.get('destination_issue_handle'), 'canonical.pitch.destination_issue_handle')}` | `{require_string(pitch_mapping.get('board_policy'), 'canonical.pitch.board_policy')}` | `{require_string(pitch_mapping.get('planned_operation_id'), 'canonical.pitch.planned_operation_id')}` |"
    )

    lines.extend([
        "",
        "## Delete operations",
        "",
        "| Issue | Project item | Reason |",
        "| --- | --- | --- |",
    ])
    for operation in delete_ops:
        lines.append(
            f"| `{require_string(operation.get('canonical_issue_handle'), 'delete_op.canonical_issue_handle')}` | `{require_string(operation.get('project_item_id'), 'delete_op.project_item_id')}` | `{require_string(operation.get('touch_reason'), 'delete_op.touch_reason')}` |"
        )

    lines.extend([
        "",
        "## Add operations",
        "",
        "| Issue | Repo | Status | Domain | Note |",
        "| --- | --- | --- | --- | --- |",
    ])
    for operation in add_ops:
        final_row_state = require_object(operation.get("final_row_state"), "add_op.final_row_state")
        field_values = require_object(final_row_state.get("field_values"), "add_op.final_row_state.field_values")
        lines.append(
            f"| `{require_string(operation.get('canonical_issue_handle'), 'add_op.canonical_issue_handle')}` | `{require_string(final_row_state.get('repo'), 'add_op.final_row_state.repo')}` | `{require_object(field_values.get('status'), 'add.status').get('value')}` | `{require_object(field_values.get('domain'), 'add.domain').get('value')}` | {require_string(operation.get('note'), 'add_op.note')} |"
        )

    lines.extend([
        "",
        "## Update operations",
        "",
        "| Issue | Change count | Summary |",
        "| --- | --- | --- |",
    ])
    for operation in update_ops:
        changes = [render_field_change_summary(change) for change in require_array(operation.get("field_changes"), "update_op.field_changes")]
        lines.append(
            f"| `{require_string(operation.get('canonical_issue_handle'), 'update_op.canonical_issue_handle')}` | `{require_int(operation.get('change_count'), 'update_op.change_count')}` | {'; '.join(changes)} |"
        )

    lines.extend([
        "",
        "## Verified no-op rows",
        "",
        "| Issue | Verification | Snapshot note |",
        "| --- | --- | --- |",
    ])
    for row in verified_noops:
        historical_snapshot = row.get("historical_snapshot")
        snapshot_note = "—"
        if isinstance(historical_snapshot, dict):
            snapshot_note = f"snapshot title was {historical_snapshot.get('title')!r}"
        lines.append(
            f"| `{require_string(row.get('canonical_issue_handle'), 'verified_noop.canonical_issue_handle')}` | `{require_string(row.get('verification_kind'), 'verified_noop.verification_kind')}` | {snapshot_note} |"
        )

    lines.extend([
        "",
        "## Inheritance coverage",
        "",
        f"- Rows requiring inheritance: `{require_int(inheritance_rollup.get('rows'), 'inheritance_rollup.rows')}`",
        f"- Field changes applied through inheritance: `{require_int(inheritance_rollup.get('field_change_count'), 'inheritance_rollup.field_change_count')}`",
        f"- Deepest parent chain: `{require_int(inheritance_rollup.get('deepest_chain_length'), 'inheritance_rollup.deepest_chain_length')}` handles",
        "",
    ])
    return "\n".join(lines)


def write_outputs(*, output_dir: Path, plan: dict[str, Any], markdown: str) -> list[Path]:
    output_dir.mkdir(parents=True, exist_ok=True)
    plan_path = output_dir / PLAN_JSON_FILENAME
    markdown_path = output_dir / PLAN_MD_FILENAME
    write_json_atomic(plan_path, plan)
    markdown_path.write_text(markdown + ("" if markdown.endswith("\n") else "\n"), encoding="utf8")
    return [plan_path, markdown_path]


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Build the M057 S03 project mutation manifest from live repo truth.")
    parser.add_argument("--source-root", type=Path, default=ROOT, help="Alternate source root for isolated contract tests.")
    parser.add_argument("--s01-dir", type=Path, help="Directory containing the S01 ledger and snapshots.")
    parser.add_argument("--s02-dir", type=Path, help="Directory containing the S02 results artifact.")
    parser.add_argument("--output-dir", type=Path, help="Directory receiving the S03 plan artifacts.")
    parser.add_argument("--live-state-file", type=Path, help="Optional captured live-state bundle for offline contract tests.")
    parser.add_argument("--preflight-json", type=Path, help="Optional retained preflight result JSON for offline contract tests.")
    parser.add_argument("--check", action="store_true", help="Validate the generated plan contract after writing outputs.")
    args = parser.parse_args(argv)
    args.source_root = args.source_root.resolve()
    args.s01_dir = (args.source_root / DEFAULT_S01_DIR.relative_to(ROOT)).resolve() if args.s01_dir is None else args.s01_dir.resolve()
    args.s02_dir = (args.source_root / DEFAULT_S02_DIR.relative_to(ROOT)).resolve() if args.s02_dir is None else args.s02_dir.resolve()
    args.output_dir = (args.source_root / DEFAULT_OUTPUT_DIR.relative_to(ROOT)).resolve() if args.output_dir is None else args.output_dir.resolve()
    if args.live_state_file is not None:
        args.live_state_file = args.live_state_file.resolve()
    if args.preflight_json is not None:
        args.preflight_json = args.preflight_json.resolve()
    return args


def main(argv: list[str]) -> int:
    args = parse_args(argv)
    preflight_outcome = run_preflight(source_root=args.source_root, override_json=args.preflight_json)
    if not preflight_outcome.ok:
        blocked_plan, blocked_markdown = build_blocked_plan(
            source_root=args.source_root,
            output_dir=args.output_dir,
            preflight=preflight_outcome.record,
        )
        written_paths = write_outputs(output_dir=args.output_dir, plan=blocked_plan, markdown=blocked_markdown)
        print(
            json.dumps(
                {
                    "status": "blocked_preflight",
                    "output_dir": str(args.output_dir),
                    "written_files": [str(path) for path in written_paths],
                    "preflight": preflight_outcome.record,
                },
                indent=2,
            )
        )
        return 1

    live_capture = load_live_state(args.live_state_file) if args.live_state_file is not None else capture_live_state()
    plan, markdown = build_plan(
        source_root=args.source_root,
        s01_dir=args.s01_dir,
        s02_dir=args.s02_dir,
        output_dir=args.output_dir,
        live_capture=live_capture,
        preflight=preflight_outcome.record,
    )
    written_paths = write_outputs(output_dir=args.output_dir, plan=plan, markdown=markdown)
    check_summary = validate_plan(plan) if args.check else None
    print(
        json.dumps(
            {
                "status": "ok",
                "output_dir": str(args.output_dir),
                "written_files": [str(path) for path in written_paths],
                "rollup": require_object(plan.get("rollup"), "plan.rollup"),
                "check": check_summary,
            },
            indent=2,
        )
    )
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main(sys.argv[1:]))
    except (PlanError, InventoryError) as exc:
        print(str(exc), file=sys.stderr)
        raise SystemExit(1)
