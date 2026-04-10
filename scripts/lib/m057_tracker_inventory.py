#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import re
import shlex
import shutil
import subprocess
import sys
import tempfile
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[2]
SCRIPT_RELATIVE_PATH = "scripts/lib/m057_tracker_inventory.py"
QUERY_RELATIVE_PATH = "scripts/lib/m057_project_items.graphql"
DEFAULT_OUTPUT_DIR = ROOT / ".gsd" / "milestones" / "M057" / "slices" / "S01"

MESH_LANG_REPO = "hyperpush-org/mesh-lang"
HYPERPUSH_REPO = "hyperpush-org/hyperpush"
HYPERPUSH_ALIAS_REPO = "hyperpush-org/hyperpush-mono"
PROJECT_OWNER = "hyperpush-org"
PROJECT_NUMBER = 1
ISSUE_LIMIT = 200
PROJECT_PAGE_SIZE = 50
PROJECT_FIELD_PAGE_SIZE = 100

REPO_ISSUE_FIELDS = [
    "number",
    "title",
    "state",
    "labels",
    "body",
    "url",
    "createdAt",
    "updatedAt",
    "closedAt",
]

TRACKED_PROJECT_FIELDS = (
    "Title",
    "Status",
    "Domain",
    "Track",
    "Commitment",
    "Delivery Mode",
    "Priority",
    "Start date",
    "Target date",
    "Hackathon Phase",
)

IGNORED_PROJECT_VALUE_TYPES = {
    "ProjectV2ItemFieldLabelValue",
    "ProjectV2ItemFieldMilestoneValue",
    "ProjectV2ItemFieldPullRequestValue",
    "ProjectV2ItemFieldRepositoryValue",
    "ProjectV2ItemFieldReviewerValue",
    "ProjectV2ItemFieldUserValue",
}

EXPECTED_COUNTS = {
    "mesh_lang_total": 16,
    "hyperpush_total": 52,
    "combined_total": 68,
    "project_items_total": 63,
    "project_fields_total": 18,
    "project_mesh_lang_total": 16,
    "project_hyperpush_total": 47,
}

EXPECTED_NON_PROJECT_HYPERPUSH_ISSUES = {2, 3, 4, 5, 8}
SNAPSHOT_VERSIONS = {
    "mesh_lang_issues": "m057-s01-mesh-lang-issues-v1",
    "hyperpush_issues": "m057-s01-hyperpush-issues-v1",
    "project_fields": "m057-s01-project-fields-v1",
    "project_items": "m057-s01-project-items-v1",
}
SNAPSHOT_FILES = {
    "mesh_lang_issues": "mesh-lang-issues.snapshot.json",
    "hyperpush_issues": "hyperpush-issues.snapshot.json",
    "project_fields": "project-fields.snapshot.json",
    "project_items": "project-items.snapshot.json",
}


class InventoryError(RuntimeError):
    pass


def iso_now() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def normalize_field_key(name: str) -> str:
    return re.sub(r"[^a-z0-9]+", "_", name.strip().lower()).strip("_")


def command_display(args: list[str]) -> str:
    return " ".join(shlex.quote(part) for part in args)


def require_object(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise InventoryError(f"{label} must be a JSON object")
    return value


def require_array(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise InventoryError(f"{label} must be a JSON array")
    return value


def require_string(value: Any, label: str) -> str:
    if not isinstance(value, str) or value.strip() == "":
        raise InventoryError(f"{label} must be a non-empty string")
    return value


def require_nullable_string(value: Any, label: str) -> str | None:
    if value is None:
        return None
    return require_string(value, label)


def require_int(value: Any, label: str) -> int:
    if not isinstance(value, int):
        raise InventoryError(f"{label} must be an integer")
    return value


def require_bool(value: Any, label: str) -> bool:
    if not isinstance(value, bool):
        raise InventoryError(f"{label} must be a boolean")
    return value


def query_path() -> Path:
    return ROOT / QUERY_RELATIVE_PATH


def run_gh_json(args: list[str], *, phase: str, timeout_seconds: int = 60) -> tuple[Any, str]:
    gh_path = shutil.which("gh")
    if gh_path is None:
        raise InventoryError(f"{phase}: gh CLI not found on PATH")

    command = [gh_path, *args]
    display = command_display(command)
    try:
        result = subprocess.run(
            command,
            cwd=ROOT,
            capture_output=True,
            encoding="utf8",
            timeout=timeout_seconds,
            check=False,
        )
    except subprocess.TimeoutExpired as exc:
        raise InventoryError(f"{phase}: command timed out after {timeout_seconds}s: {display}") from exc

    if result.returncode != 0:
        stderr = result.stderr.strip()
        stdout = result.stdout.strip()
        detail = stderr or stdout or f"exit {result.returncode}"
        raise InventoryError(f"{phase}: command failed: {display}\n{detail}")

    try:
        payload = json.loads(result.stdout)
    except json.JSONDecodeError as exc:
        raise InventoryError(f"{phase}: command returned invalid JSON: {display}\n{exc}") from exc

    if isinstance(payload, dict) and payload.get("errors"):
        errors = require_array(payload["errors"], f"{phase}.errors")
        messages: list[str] = []
        for index, error in enumerate(errors):
            error_object = require_object(error, f"{phase}.errors[{index}]")
            messages.append(require_string(error_object.get("message"), f"{phase}.errors[{index}].message"))
        raise InventoryError(f"{phase}: GitHub GraphQL returned errors: {', '.join(messages)}")

    return payload, display


def fetch_repo_identity(requested_slug: str, *, phase: str) -> tuple[dict[str, Any], str]:
    payload, command = run_gh_json(
        ["repo", "view", requested_slug, "--json", "nameWithOwner,url"],
        phase=phase,
        timeout_seconds=30,
    )
    repo = require_object(payload, phase)
    canonical_slug = require_string(repo.get("nameWithOwner"), f"{phase}.nameWithOwner")
    canonical_url = require_string(repo.get("url"), f"{phase}.url")
    identity = {
        "requested_slug": requested_slug,
        "canonical_slug": canonical_slug,
        "canonical_url": canonical_url.rstrip("/"),
        "issue_base_url": f"{canonical_url.rstrip('/')}/issues",
    }
    return identity, command


def require_canonical_slug(identity: dict[str, Any], expected_slug: str, *, phase: str) -> None:
    actual_slug = require_string(identity.get("canonical_slug"), f"{phase}.canonical_slug")
    actual_url = require_string(identity.get("canonical_url"), f"{phase}.canonical_url")
    expected_url = f"https://github.com/{expected_slug}"
    if actual_slug != expected_slug or actual_url != expected_url:
        raise InventoryError(
            f"{phase}: expected canonical repo {expected_slug} ({expected_url}) but got {actual_slug} ({actual_url})"
        )


def normalize_issue_label(label: Any, *, phase: str, issue_number: int, index: int) -> dict[str, Any]:
    label_object = require_object(label, f"{phase}.issue[{issue_number}].labels[{index}]")
    label_id = require_string(label_object.get("id"), f"{phase}.issue[{issue_number}].labels[{index}].id")
    name = require_string(label_object.get("name"), f"{phase}.issue[{issue_number}].labels[{index}].name")
    description = label_object.get("description")
    if description is not None and not isinstance(description, str):
        raise InventoryError(f"{phase}.issue[{issue_number}].labels[{index}].description must be a string or null")
    color = require_string(label_object.get("color"), f"{phase}.issue[{issue_number}].labels[{index}].color")
    return {
        "id": label_id,
        "name": name,
        "description": description,
        "color": color,
    }


def normalize_issue(issue: Any, *, repo_identity: dict[str, Any], phase: str, row_index: int) -> dict[str, Any]:
    issue_object = require_object(issue, f"{phase}.issue[{row_index}]")
    number = require_int(issue_object.get("number"), f"{phase}.issue[{row_index}].number")
    title = require_string(issue_object.get("title"), f"{phase}.issue[{row_index}].title")
    state = require_string(issue_object.get("state"), f"{phase}.issue[{row_index}].state")
    if "labels" not in issue_object:
        raise InventoryError(f"{phase}.issue[{row_index}].labels is required")
    labels = require_array(issue_object.get("labels"), f"{phase}.issue[{row_index}].labels")
    if "closedAt" not in issue_object:
        raise InventoryError(f"{phase}.issue[{row_index}].closedAt is required")
    body = issue_object.get("body")
    if not isinstance(body, str):
        raise InventoryError(f"{phase}.issue[{row_index}].body must be a string")
    url = require_string(issue_object.get("url"), f"{phase}.issue[{row_index}].url")
    created_at = require_string(issue_object.get("createdAt"), f"{phase}.issue[{row_index}].createdAt")
    updated_at = require_string(issue_object.get("updatedAt"), f"{phase}.issue[{row_index}].updatedAt")
    closed_at = require_nullable_string(issue_object.get("closedAt"), f"{phase}.issue[{row_index}].closedAt")
    expected_issue_url = f"{repo_identity['issue_base_url']}/{number}"
    if url.rstrip("/") != expected_issue_url:
        raise InventoryError(
            f"{phase}.issue[{row_index}].url expected {expected_issue_url!r} but found {url!r}"
        )

    normalized_labels = [
        normalize_issue_label(label, phase=phase, issue_number=number, index=index) for index, label in enumerate(labels)
    ]
    normalized_labels.sort(key=lambda item: item["name"].lower())

    return {
        "canonical_issue_url": expected_issue_url,
        "repo": require_string(repo_identity.get("canonical_slug"), f"{phase}.repo_identity.canonical_slug"),
        "number": number,
        "title": title,
        "state": state,
        "created_at": created_at,
        "updated_at": updated_at,
        "closed_at": closed_at,
        "url": expected_issue_url,
        "body": body,
        "labels": normalized_labels,
    }


def build_issue_snapshot(
    *,
    version: str,
    captured_at: str,
    repo_identity: dict[str, Any],
    source_command: str,
    issues: list[dict[str, Any]],
    alias_identity: dict[str, Any] | None = None,
    alias_command: str | None = None,
) -> dict[str, Any]:
    issues_sorted = sorted(issues, key=lambda item: item["number"])
    total = len(issues_sorted)
    open_count = sum(1 for issue in issues_sorted if issue["state"] == "OPEN")
    closed_count = sum(1 for issue in issues_sorted if issue["state"] == "CLOSED")
    snapshot = {
        "version": version,
        "snapshot_kind": "repo-issues",
        "captured_at": captured_at,
        "source": {
            "command": source_command,
            "requested_repo": repo_identity["requested_slug"],
        },
        "repo": repo_identity,
        "rollup": {
            "total": total,
            "open": open_count,
            "closed": closed_count,
        },
        "issues": issues_sorted,
    }
    if alias_identity is not None:
        snapshot["canonical_redirect"] = {
            "command": alias_command,
            "requested_repo": alias_identity["requested_slug"],
            "canonical_slug": alias_identity["canonical_slug"],
            "canonical_url": alias_identity["canonical_url"],
        }
    return snapshot


def capture_repo_issues(*, captured_at: str) -> tuple[dict[str, Any], dict[str, Any], dict[str, Any]]:
    mesh_identity, mesh_repo_command = fetch_repo_identity(MESH_LANG_REPO, phase="mesh-lang-repo-view")
    require_canonical_slug(mesh_identity, MESH_LANG_REPO, phase="mesh-lang-repo-view")

    product_identity, product_repo_command = fetch_repo_identity(HYPERPUSH_REPO, phase="hyperpush-repo-view")
    require_canonical_slug(product_identity, HYPERPUSH_REPO, phase="hyperpush-repo-view")

    product_alias_identity, product_alias_command = fetch_repo_identity(
        HYPERPUSH_ALIAS_REPO,
        phase="hyperpush-alias-repo-view",
    )
    require_canonical_slug(product_alias_identity, HYPERPUSH_REPO, phase="hyperpush-alias-repo-view")

    mesh_payload, mesh_issue_command = run_gh_json(
        [
            "issue",
            "list",
            "-R",
            MESH_LANG_REPO,
            "--state",
            "all",
            "--limit",
            str(ISSUE_LIMIT),
            "--json",
            ",".join(REPO_ISSUE_FIELDS),
        ],
        phase="mesh-lang-issues",
        timeout_seconds=60,
    )
    hyperpush_payload, hyperpush_issue_command = run_gh_json(
        [
            "issue",
            "list",
            "-R",
            HYPERPUSH_REPO,
            "--state",
            "all",
            "--limit",
            str(ISSUE_LIMIT),
            "--json",
            ",".join(REPO_ISSUE_FIELDS),
        ],
        phase="hyperpush-issues",
        timeout_seconds=60,
    )

    mesh_issues = [
        normalize_issue(issue, repo_identity=mesh_identity, phase="mesh-lang-issues", row_index=index)
        for index, issue in enumerate(require_array(mesh_payload, "mesh-lang-issues"))
    ]
    hyperpush_issues = [
        normalize_issue(issue, repo_identity=product_identity, phase="hyperpush-issues", row_index=index)
        for index, issue in enumerate(require_array(hyperpush_payload, "hyperpush-issues"))
    ]

    mesh_snapshot = build_issue_snapshot(
        version=SNAPSHOT_VERSIONS["mesh_lang_issues"],
        captured_at=captured_at,
        repo_identity=mesh_identity,
        source_command=mesh_issue_command,
        issues=mesh_issues,
    )
    hyperpush_snapshot = build_issue_snapshot(
        version=SNAPSHOT_VERSIONS["hyperpush_issues"],
        captured_at=captured_at,
        repo_identity=product_identity,
        source_command=hyperpush_issue_command,
        issues=hyperpush_issues,
        alias_identity=product_alias_identity,
        alias_command=product_alias_command,
    )
    canonical_repos = {
        "mesh_lang": {
            "command": mesh_repo_command,
            **mesh_identity,
        },
        "hyperpush": {
            "command": product_repo_command,
            **product_identity,
        },
        "hyperpush_alias": {
            "command": product_alias_command,
            **product_alias_identity,
        },
    }
    return mesh_snapshot, hyperpush_snapshot, canonical_repos


def normalize_project_field(field: Any, *, index: int) -> dict[str, Any]:
    field_object = require_object(field, f"project-fields.fields[{index}]")
    field_id = require_string(field_object.get("id"), f"project-fields.fields[{index}].id")
    name = require_string(field_object.get("name"), f"project-fields.fields[{index}].name")
    field_type = require_string(field_object.get("type"), f"project-fields.fields[{index}].type")
    options_payload = field_object.get("options", [])
    if options_payload is None:
        options_payload = []
    options = []
    for option_index, option in enumerate(require_array(options_payload, f"project-fields.fields[{index}].options")):
        option_object = require_object(option, f"project-fields.fields[{index}].options[{option_index}]")
        option_id = require_string(option_object.get("id"), f"project-fields.fields[{index}].options[{option_index}].id")
        option_name = require_string(option_object.get("name"), f"project-fields.fields[{index}].options[{option_index}].name")
        options.append(
            {
                "id": option_id,
                "name": option_name,
                "option_key": normalize_field_key(option_name),
            }
        )
    return {
        "field_id": field_id,
        "field_name": name,
        "field_key": normalize_field_key(name),
        "field_type": field_type,
        "options": options,
    }


def capture_project_fields(*, captured_at: str, canonical_repos: dict[str, Any]) -> tuple[dict[str, Any], dict[str, dict[str, Any]]]:
    payload, command = run_gh_json(
        ["project", "field-list", str(PROJECT_NUMBER), "--owner", PROJECT_OWNER, "--format", "json"],
        phase="project-fields",
        timeout_seconds=30,
    )
    root = require_object(payload, "project-fields")
    total_count = require_int(root.get("totalCount"), "project-fields.totalCount")
    fields = [normalize_project_field(field, index=index) for index, field in enumerate(require_array(root.get("fields"), "project-fields.fields"))]
    field_index: dict[str, dict[str, Any]] = {}
    tracked_fields: dict[str, dict[str, Any]] = {}
    for field in fields:
        field_id = field["field_id"]
        field_name = field["field_name"]
        if field_id in field_index:
            raise InventoryError(f"project-fields: duplicate field id {field_id}")
        field_index[field_id] = field
        if field_name in TRACKED_PROJECT_FIELDS:
            tracked_fields[field["field_key"]] = field

    missing_tracked = [name for name in TRACKED_PROJECT_FIELDS if normalize_field_key(name) not in tracked_fields]
    if missing_tracked:
        raise InventoryError(f"project-fields: missing tracked fields: {', '.join(missing_tracked)}")

    snapshot = {
        "version": SNAPSHOT_VERSIONS["project_fields"],
        "snapshot_kind": "project-fields",
        "captured_at": captured_at,
        "source": {
            "command": command,
            "owner": PROJECT_OWNER,
            "project_number": PROJECT_NUMBER,
        },
        "project": {
            "owner": PROJECT_OWNER,
            "number": PROJECT_NUMBER,
        },
        "canonical_repos": canonical_repos,
        "field_count": total_count,
        "tracked_field_keys": sorted(tracked_fields),
        "fields": fields,
    }
    return snapshot, tracked_fields


def read_graphql_query() -> str:
    path = query_path()
    if not path.is_file():
        raise InventoryError(f"missing {QUERY_RELATIVE_PATH}")
    query = path.read_text()
    if query.strip() == "":
        raise InventoryError(f"{QUERY_RELATIVE_PATH} is empty")
    return query


def normalize_field_value(
    *,
    node: dict[str, Any],
    field_definition: dict[str, Any],
    phase: str,
    item_label: str,
) -> tuple[Any, Any, str]:
    node_type = require_string(node.get("__typename"), f"{item_label}.{phase}.__typename")
    if node_type == "ProjectV2ItemFieldTextValue":
        value = node.get("text")
        if value is not None and not isinstance(value, str):
            raise InventoryError(f"{item_label}.{phase}.text must be a string or null")
        return (value.strip() or None) if isinstance(value, str) else None, None, node_type
    if node_type == "ProjectV2ItemFieldNumberValue":
        value = node.get("number")
        if value is not None and not isinstance(value, (int, float)):
            raise InventoryError(f"{item_label}.{phase}.number must be numeric or null")
        return value, None, node_type
    if node_type == "ProjectV2ItemFieldDateValue":
        value = node.get("date")
        return require_nullable_string(value, f"{item_label}.{phase}.date"), None, node_type
    if node_type == "ProjectV2ItemFieldSingleSelectValue":
        value = node.get("name")
        option_id = node.get("optionId")
        normalized_value = None
        if value is not None:
            if not isinstance(value, str):
                raise InventoryError(f"{item_label}.{phase}.name must be a string or null")
            normalized_value = value.strip() or None
        normalized_option_id = None
        if option_id is not None:
            if not isinstance(option_id, str):
                raise InventoryError(f"{item_label}.{phase}.optionId must be a string or null")
            normalized_option_id = option_id.strip() or None
        return normalized_value, normalized_option_id, node_type
    if node_type in IGNORED_PROJECT_VALUE_TYPES:
        raise InventoryError(
            f"{item_label}.{phase}: unsupported field value type {node_type} reached tracked field {field_definition['field_name']}"
        )
    raise InventoryError(f"{item_label}.{phase}: unsupported field value type {node_type}")


def build_default_field_values(tracked_fields: dict[str, dict[str, Any]], *, issue_title: str) -> dict[str, dict[str, Any]]:
    values: dict[str, dict[str, Any]] = {}
    for field_key, definition in tracked_fields.items():
        value = None
        value_type = None
        if definition["field_name"] == "Title":
            value = issue_title
            value_type = "issue.title"
        values[field_key] = {
            "field_id": definition["field_id"],
            "field_name": definition["field_name"],
            "field_key": definition["field_key"],
            "field_type": definition["field_type"],
            "value": value,
            "option_id": None,
            "value_type": value_type,
        }
    return values


def normalize_project_item(
    item: Any,
    *,
    index: int,
    tracked_fields: dict[str, dict[str, Any]],
    field_index: dict[str, dict[str, Any]],
) -> dict[str, Any]:
    item_object = require_object(item, f"project-items.items[{index}]")
    project_item_id = require_string(item_object.get("id"), f"project-items.items[{index}].id")
    content = require_object(item_object.get("content"), f"project-items.items[{index}].content")
    content_type = require_string(content.get("__typename"), f"project-items.items[{index}].content.__typename")
    if content_type != "Issue":
        raise InventoryError(
            f"project-items.items[{index}].content.__typename expected 'Issue' but found {content_type!r}"
        )
    issue_number = require_int(content.get("number"), f"project-items.items[{index}].content.number")
    issue_title = require_string(content.get("title"), f"project-items.items[{index}].content.title")
    issue_state = require_string(content.get("state"), f"project-items.items[{index}].content.state")
    issue_url = require_string(content.get("url"), f"project-items.items[{index}].content.url")
    repository = require_object(content.get("repository"), f"project-items.items[{index}].content.repository")
    repository_slug = require_string(
        repository.get("nameWithOwner"),
        f"project-items.items[{index}].content.repository.nameWithOwner",
    )
    repository_url = require_string(
        repository.get("url"),
        f"project-items.items[{index}].content.repository.url",
    )
    expected_issue_url = f"{repository_url.rstrip('/')}/issues/{issue_number}"
    if issue_url.rstrip("/") != expected_issue_url:
        raise InventoryError(
            f"project-items.items[{index}].content.url expected {expected_issue_url!r} but found {issue_url!r}"
        )

    field_values_connection = require_object(item_object.get("fieldValues"), f"project-items.items[{index}].fieldValues")
    page_info = require_object(field_values_connection.get("pageInfo"), f"project-items.items[{index}].fieldValues.pageInfo")
    if require_bool(page_info.get("hasNextPage"), f"project-items.items[{index}].fieldValues.pageInfo.hasNextPage"):
        raise InventoryError(
            f"project-items.items[{index}].fieldValues exceeds page size {PROJECT_FIELD_PAGE_SIZE}; increase field pagination"
        )

    field_values = build_default_field_values(tracked_fields, issue_title=issue_title)
    value_nodes = require_array(field_values_connection.get("nodes"), f"project-items.items[{index}].fieldValues.nodes")
    for value_index, node in enumerate(value_nodes):
        node_object = require_object(node, f"project-items.items[{index}].fieldValues.nodes[{value_index}]")
        node_type = require_string(
            node_object.get("__typename"),
            f"project-items.items[{index}].fieldValues.nodes[{value_index}].__typename",
        )
        if node_type in IGNORED_PROJECT_VALUE_TYPES:
            continue
        field = require_object(node_object.get("field"), f"project-items.items[{index}].fieldValues.nodes[{value_index}].field")
        field_id = require_string(field.get("id"), f"project-items.items[{index}].fieldValues.nodes[{value_index}].field.id")
        field_name = require_string(field.get("name"), f"project-items.items[{index}].fieldValues.nodes[{value_index}].field.name")
        if field_id not in field_index:
            raise InventoryError(
                f"project-items.items[{index}].fieldValues.nodes[{value_index}].field.id {field_id!r} missing from field schema"
            )
        field_definition = field_index[field_id]
        if field_definition["field_name"] != field_name:
            raise InventoryError(
                f"project-items.items[{index}].fieldValues.nodes[{value_index}].field.name expected {field_definition['field_name']!r} but found {field_name!r}"
            )
        field_key = field_definition["field_key"]
        if field_key not in field_values:
            continue
        value, option_id, value_type = normalize_field_value(
            node=node_object,
            field_definition=field_definition,
            phase=f"fieldValues.nodes[{value_index}]",
            item_label=f"project-items.items[{index}]",
        )
        field_values[field_key] = {
            "field_id": field_definition["field_id"],
            "field_name": field_definition["field_name"],
            "field_key": field_definition["field_key"],
            "field_type": field_definition["field_type"],
            "value": value,
            "option_id": option_id,
            "value_type": value_type,
        }

    return {
        "project_item_id": project_item_id,
        "canonical_issue_url": expected_issue_url,
        "issue": {
            "repo": repository_slug,
            "number": issue_number,
            "title": issue_title,
            "state": issue_state,
            "url": expected_issue_url,
        },
        "field_values": field_values,
    }


def capture_project_items(
    *,
    captured_at: str,
    tracked_fields: dict[str, dict[str, Any]],
    project_fields_snapshot: dict[str, Any],
    canonical_repos: dict[str, Any],
) -> dict[str, Any]:
    query = read_graphql_query()
    field_index = {field["field_id"]: field for field in project_fields_snapshot["fields"]}
    all_items: list[dict[str, Any]] = []
    seen_cursors: set[str] = set()
    pages = 0
    after: str | None = None
    project_meta: dict[str, Any] | None = None
    total_count: int | None = None
    commands: list[dict[str, Any]] = []

    while True:
        pages += 1
        command = [
            "api",
            "graphql",
            "-f",
            f"query={query}",
            "-F",
            f"owner={PROJECT_OWNER}",
            "-F",
            f"number={PROJECT_NUMBER}",
            "-F",
            f"pageSize={PROJECT_PAGE_SIZE}",
            "-F",
            f"fieldPageSize={PROJECT_FIELD_PAGE_SIZE}",
        ]
        if after is not None:
            command.extend(["-F", f"after={after}"])
        payload, command_display_value = run_gh_json(command, phase=f"project-items-page-{pages}", timeout_seconds=60)
        root = require_object(payload, f"project-items-page-{pages}")
        organization = require_object(root.get("data"), f"project-items-page-{pages}.data")
        organization = require_object(organization.get("organization"), f"project-items-page-{pages}.data.organization")
        project = require_object(organization.get("projectV2"), f"project-items-page-{pages}.data.organization.projectV2")
        page_project_meta = {
            "id": require_string(project.get("id"), f"project-items-page-{pages}.project.id"),
            "title": require_string(project.get("title"), f"project-items-page-{pages}.project.title"),
            "url": require_string(project.get("url"), f"project-items-page-{pages}.project.url"),
            "owner": PROJECT_OWNER,
            "number": PROJECT_NUMBER,
        }
        if project_meta is None:
            project_meta = page_project_meta
        elif project_meta != page_project_meta:
            raise InventoryError("project-items: project metadata changed between pages")

        items_connection = require_object(project.get("items"), f"project-items-page-{pages}.project.items")
        page_total_count = require_int(items_connection.get("totalCount"), f"project-items-page-{pages}.project.items.totalCount")
        if total_count is None:
            total_count = page_total_count
        elif total_count != page_total_count:
            raise InventoryError("project-items: totalCount changed between pages")
        nodes = require_array(items_connection.get("nodes"), f"project-items-page-{pages}.project.items.nodes")
        for index, item in enumerate(nodes):
            all_items.append(
                normalize_project_item(
                    item,
                    index=len(all_items),
                    tracked_fields=tracked_fields,
                    field_index=field_index,
                )
            )
        page_info = require_object(items_connection.get("pageInfo"), f"project-items-page-{pages}.project.items.pageInfo")
        has_next_page = require_bool(
            page_info.get("hasNextPage"),
            f"project-items-page-{pages}.project.items.pageInfo.hasNextPage",
        )
        command_marker = {
            "page": pages,
            "command": command_display_value,
            "after": after,
        }
        if pages == 1:
            commands = [command_marker]
        else:
            commands.append(command_marker)
        if not has_next_page:
            break
        end_cursor = require_string(page_info.get("endCursor"), f"project-items-page-{pages}.project.items.pageInfo.endCursor")
        if end_cursor in seen_cursors:
            raise InventoryError(f"project-items: repeated pagination cursor {end_cursor!r}")
        seen_cursors.add(end_cursor)
        after = end_cursor

    if project_meta is None or total_count is None:
        raise InventoryError("project-items: no project pages were returned")

    repo_counts: dict[str, int] = {}
    field_presence = {field_key: 0 for field_key in sorted(tracked_fields)}
    for item in all_items:
        repo = item["issue"]["repo"]
        repo_counts[repo] = repo_counts.get(repo, 0) + 1
        for field_key, field_value in item["field_values"].items():
            if field_value["value"] not in (None, ""):
                field_presence[field_key] += 1

    return {
        "version": SNAPSHOT_VERSIONS["project_items"],
        "snapshot_kind": "project-items",
        "captured_at": captured_at,
        "source": {
            "query_file": QUERY_RELATIVE_PATH,
            "commands": commands,
            "owner": PROJECT_OWNER,
            "project_number": PROJECT_NUMBER,
            "page_size": PROJECT_PAGE_SIZE,
            "field_page_size": PROJECT_FIELD_PAGE_SIZE,
            "pages": pages,
        },
        "project": project_meta,
        "canonical_repos": canonical_repos,
        "tracked_field_keys": sorted(tracked_fields),
        "rollup": {
            "total_items": len(all_items),
            "repo_counts": repo_counts,
            "field_presence": field_presence,
        },
        "items": sorted(all_items, key=lambda item: (item["issue"]["repo"], item["issue"]["number"])),
    }


def capture_inventory() -> dict[str, dict[str, Any]]:
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
    return {
        "mesh_lang_issues": mesh_snapshot,
        "hyperpush_issues": hyperpush_snapshot,
        "project_fields": project_fields_snapshot,
        "project_items": project_items_snapshot,
    }


def read_snapshot_file(output_dir: Path, key: str) -> dict[str, Any]:
    path = output_dir / SNAPSHOT_FILES[key]
    if not path.is_file():
        raise InventoryError(f"missing snapshot {path}")
    try:
        payload = json.loads(path.read_text())
    except json.JSONDecodeError as exc:
        raise InventoryError(f"{path} is not valid JSON: {exc}") from exc
    return require_object(payload, str(path))


def write_json_atomic(path: Path, payload: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.NamedTemporaryFile("w", dir=path.parent, delete=False, encoding="utf8") as handle:
        json.dump(payload, handle, indent=2)
        handle.write("\n")
        tmp_path = Path(handle.name)
    tmp_path.replace(path)


def write_snapshots(output_dir: Path, snapshots: dict[str, dict[str, Any]]) -> list[Path]:
    written_paths: list[Path] = []
    for key, filename in SNAPSHOT_FILES.items():
        path = output_dir / filename
        write_json_atomic(path, snapshots[key])
        written_paths.append(path)
    return written_paths


def require_version(snapshot: dict[str, Any], key: str) -> None:
    expected = SNAPSHOT_VERSIONS[key]
    actual = require_string(snapshot.get("version"), f"{key}.version")
    if actual != expected:
        raise InventoryError(f"{key}.version expected {expected!r} but found {actual!r}")


def require_snapshot_captured_at(snapshot: dict[str, Any], key: str) -> None:
    require_string(snapshot.get("captured_at"), f"{key}.captured_at")


def validate_snapshots(snapshots: dict[str, dict[str, Any]]) -> dict[str, Any]:
    mesh_snapshot = snapshots["mesh_lang_issues"]
    hyperpush_snapshot = snapshots["hyperpush_issues"]
    project_fields_snapshot = snapshots["project_fields"]
    project_items_snapshot = snapshots["project_items"]

    for key, snapshot in snapshots.items():
        require_version(snapshot, key)
        require_snapshot_captured_at(snapshot, key)

    mesh_repo = require_object(mesh_snapshot.get("repo"), "mesh_lang_issues.repo")
    require_canonical_slug(mesh_repo, MESH_LANG_REPO, phase="mesh_lang_issues.repo")
    hyperpush_repo = require_object(hyperpush_snapshot.get("repo"), "hyperpush_issues.repo")
    require_canonical_slug(hyperpush_repo, HYPERPUSH_REPO, phase="hyperpush_issues.repo")
    redirect = require_object(hyperpush_snapshot.get("canonical_redirect"), "hyperpush_issues.canonical_redirect")
    if require_string(redirect.get("requested_repo"), "hyperpush_issues.canonical_redirect.requested_repo") != HYPERPUSH_ALIAS_REPO:
        raise InventoryError("hyperpush_issues.canonical_redirect.requested_repo drifted")
    if require_string(redirect.get("canonical_slug"), "hyperpush_issues.canonical_redirect.canonical_slug") != HYPERPUSH_REPO:
        raise InventoryError("hyperpush_issues.canonical_redirect.canonical_slug drifted")

    mesh_issues = require_array(mesh_snapshot.get("issues"), "mesh_lang_issues.issues")
    hyperpush_issues = require_array(hyperpush_snapshot.get("issues"), "hyperpush_issues.issues")
    project_items = require_array(project_items_snapshot.get("items"), "project_items.items")
    project_fields = require_array(project_fields_snapshot.get("fields"), "project_fields.fields")

    if len(mesh_issues) != EXPECTED_COUNTS["mesh_lang_total"]:
        raise InventoryError(
            f"mesh_lang_issues expected {EXPECTED_COUNTS['mesh_lang_total']} issues but found {len(mesh_issues)}"
        )
    if len(hyperpush_issues) != EXPECTED_COUNTS["hyperpush_total"]:
        raise InventoryError(
            f"hyperpush_issues expected {EXPECTED_COUNTS['hyperpush_total']} issues but found {len(hyperpush_issues)}"
        )
    if len(mesh_issues) + len(hyperpush_issues) != EXPECTED_COUNTS["combined_total"]:
        raise InventoryError(
            f"combined repo issues expected {EXPECTED_COUNTS['combined_total']} rows but found {len(mesh_issues) + len(hyperpush_issues)}"
        )
    if len(project_fields) != EXPECTED_COUNTS["project_fields_total"]:
        raise InventoryError(
            f"project_fields expected {EXPECTED_COUNTS['project_fields_total']} rows but found {len(project_fields)}"
        )
    if len(project_items) != EXPECTED_COUNTS["project_items_total"]:
        raise InventoryError(
            f"project_items expected {EXPECTED_COUNTS['project_items_total']} rows but found {len(project_items)}"
        )

    tracked_field_keys = set(require_array(project_items_snapshot.get("tracked_field_keys"), "project_items.tracked_field_keys"))
    expected_tracked_keys = {normalize_field_key(name) for name in TRACKED_PROJECT_FIELDS}
    if tracked_field_keys != expected_tracked_keys:
        raise InventoryError(
            f"project_items.tracked_field_keys expected {sorted(expected_tracked_keys)!r} but found {sorted(tracked_field_keys)!r}"
        )

    issue_urls: set[str] = set()
    project_item_urls: set[str] = set()
    project_item_ids: set[str] = set()
    for snapshot_key, issues in (("mesh_lang_issues", mesh_issues), ("hyperpush_issues", hyperpush_issues)):
        for index, issue in enumerate(issues):
            issue_object = require_object(issue, f"{snapshot_key}.issues[{index}]")
            issue_url = require_string(issue_object.get("canonical_issue_url"), f"{snapshot_key}.issues[{index}].canonical_issue_url")
            if issue_url in issue_urls:
                raise InventoryError(f"duplicate canonical issue URL across repo snapshots: {issue_url}")
            issue_urls.add(issue_url)

    for index, item in enumerate(project_items):
        item_object = require_object(item, f"project_items.items[{index}]")
        project_item_id = require_string(item_object.get("project_item_id"), f"project_items.items[{index}].project_item_id")
        if project_item_id in project_item_ids:
            raise InventoryError(f"duplicate project item id {project_item_id}")
        project_item_ids.add(project_item_id)
        canonical_issue_url = require_string(
            item_object.get("canonical_issue_url"),
            f"project_items.items[{index}].canonical_issue_url",
        )
        if canonical_issue_url in project_item_urls:
            raise InventoryError(f"duplicate project item canonical issue URL {canonical_issue_url}")
        project_item_urls.add(canonical_issue_url)
        if canonical_issue_url not in issue_urls:
            raise InventoryError(f"orphan project item without repo issue row: {canonical_issue_url}")
        field_values = require_object(item_object.get("field_values"), f"project_items.items[{index}].field_values")
        for expected_field_key in expected_tracked_keys:
            if expected_field_key not in field_values:
                raise InventoryError(
                    f"project_items.items[{index}].field_values missing tracked field {expected_field_key!r}"
                )

    repo_counts = require_object(project_items_snapshot.get("rollup"), "project_items.rollup").get("repo_counts")
    repo_counts = require_object(repo_counts, "project_items.rollup.repo_counts")
    mesh_project_count = require_int(repo_counts.get(MESH_LANG_REPO), f"project_items.rollup.repo_counts[{MESH_LANG_REPO!r}]")
    hyperpush_project_count = require_int(repo_counts.get(HYPERPUSH_REPO), f"project_items.rollup.repo_counts[{HYPERPUSH_REPO!r}]")
    if mesh_project_count != EXPECTED_COUNTS["project_mesh_lang_total"]:
        raise InventoryError(
            f"project_items mesh-lang count expected {EXPECTED_COUNTS['project_mesh_lang_total']} but found {mesh_project_count}"
        )
    if hyperpush_project_count != EXPECTED_COUNTS["project_hyperpush_total"]:
        raise InventoryError(
            f"project_items hyperpush count expected {EXPECTED_COUNTS['project_hyperpush_total']} but found {hyperpush_project_count}"
        )

    non_project_hyperpush_numbers: set[int] = set()
    for issue in hyperpush_issues:
        issue_object = require_object(issue, "hyperpush_issues.issue")
        number = require_int(issue_object.get("number"), "hyperpush_issues.issue.number")
        issue_url = require_string(issue_object.get("canonical_issue_url"), "hyperpush_issues.issue.canonical_issue_url")
        if issue_url not in project_item_urls:
            non_project_hyperpush_numbers.add(number)
    if non_project_hyperpush_numbers != EXPECTED_NON_PROJECT_HYPERPUSH_ISSUES:
        raise InventoryError(
            f"non-project hyperpush issue set expected {sorted(EXPECTED_NON_PROJECT_HYPERPUSH_ISSUES)!r} but found {sorted(non_project_hyperpush_numbers)!r}"
        )

    field_names = {require_string(field.get("field_name"), f"project_fields.fields[{index}].field_name") for index, field in enumerate(project_fields)}
    missing_field_names = [name for name in TRACKED_PROJECT_FIELDS if name not in field_names]
    if missing_field_names:
        raise InventoryError(f"project_fields missing tracked names: {', '.join(missing_field_names)}")

    return {
        "captured_at": require_string(mesh_snapshot.get("captured_at"), "mesh_lang_issues.captured_at"),
        "mesh_lang_issues": len(mesh_issues),
        "hyperpush_issues": len(hyperpush_issues),
        "combined_issues": len(mesh_issues) + len(hyperpush_issues),
        "project_items": len(project_items),
        "project_fields": len(project_fields),
        "non_project_hyperpush_numbers": sorted(non_project_hyperpush_numbers),
    }


def load_snapshots(output_dir: Path) -> dict[str, dict[str, Any]]:
    return {key: read_snapshot_file(output_dir, key) for key in SNAPSHOT_FILES}


def summarize_snapshots(snapshots: dict[str, dict[str, Any]]) -> dict[str, Any]:
    mesh_issues = require_array(snapshots["mesh_lang_issues"].get("issues"), "mesh_lang_issues.issues")
    hyperpush_issues = require_array(snapshots["hyperpush_issues"].get("issues"), "hyperpush_issues.issues")
    project_items = require_array(snapshots["project_items"].get("items"), "project_items.items")
    project_fields = require_array(snapshots["project_fields"].get("fields"), "project_fields.fields")
    return {
        "captured_at": require_string(snapshots["mesh_lang_issues"].get("captured_at"), "mesh_lang_issues.captured_at"),
        "mesh_lang_issues": len(mesh_issues),
        "hyperpush_issues": len(hyperpush_issues),
        "combined_issues": len(mesh_issues) + len(hyperpush_issues),
        "project_items": len(project_items),
        "project_fields": len(project_fields),
    }


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Capture M057 S01 tracker inventory snapshots.")
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT_DIR)
    parser.add_argument("--refresh", action="store_true", help="Query GitHub live and rewrite the snapshot files.")
    parser.add_argument("--check", action="store_true", help="Validate the current snapshot files and expected counts.")
    args = parser.parse_args(argv)
    if not args.refresh and not args.check:
        parser.error("expected at least one of --refresh or --check")
    return args


def main(argv: list[str]) -> int:
    args = parse_args(argv)
    output_dir = args.output_dir.resolve()
    snapshots: dict[str, dict[str, Any]] | None = None

    if args.refresh:
        snapshots = capture_inventory()
        refresh_summary = summarize_snapshots(snapshots)
        if args.check:
            check_summary = validate_snapshots(snapshots)
            refresh_summary.update({"check": check_summary})
        write_snapshots(output_dir, snapshots)
        print(
            json.dumps(
                {
                    "status": "ok",
                    "mode": "refresh",
                    "output_dir": str(output_dir),
                    **refresh_summary,
                },
                indent=2,
            )
        )

    if args.check and not args.refresh:
        snapshots = load_snapshots(output_dir)
        summary = validate_snapshots(snapshots)
        print(
            json.dumps(
                {
                    "status": "ok",
                    "mode": "check",
                    "output_dir": str(output_dir),
                    **summary,
                },
                indent=2,
            )
        )

    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main(sys.argv[1:]))
    except InventoryError as exc:
        print(str(exc), file=sys.stderr)
        raise SystemExit(1)
