#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path
from typing import Any

from m057_evidence_index import (
    EVIDENCE_JSON_FILENAME,
    NAMING_MAP_FILENAME,
    DEFAULT_OUTPUT_SUBDIR,
    EvidenceError,
    validate_evidence_bundle,
    validate_naming_map,
)
from m057_tracker_inventory import (
    EXPECTED_COUNTS,
    EXPECTED_NON_PROJECT_HYPERPUSH_ISSUES,
    HYPERPUSH_REPO,
    MESH_LANG_REPO,
    InventoryError,
    ROOT,
    SNAPSHOT_FILES,
    iso_now,
    load_snapshots,
    require_array,
    require_object,
    require_string,
    validate_snapshots,
    write_json_atomic,
)

SCRIPT_RELATIVE_PATH = "scripts/lib/m057_reconciliation_ledger.py"
LEDGER_JSON_FILENAME = "reconciliation-ledger.json"
AUDIT_MD_FILENAME = "reconciliation-audit.md"
LEDGER_VERSION = "m057-s01-reconciliation-ledger-v1"

PRIMARY_BUCKETS = {"shipped-but-open", "rewrite-split", "keep-open", "misfiled"}
SECONDARY_BUCKETS = {"naming-drift"}
REPO_ACTION_KINDS = {
    "close_as_shipped",
    "keep_open",
    "rewrite_scope",
    "move_to_mesh_lang",
    "create_missing_issue",
}
PROJECT_ACTION_KINDS = {
    "remove_from_project",
    "keep_in_project",
    "update_project_item",
    "create_project_item",
    "leave_untracked",
}
SHIPPED_EVIDENCE_ID = "mesh_launch_foundations_shipped"
PARTIAL_EVIDENCE_ID = "frontend_exp_operator_surfaces_partial"
MISFILED_EVIDENCE_ID = "hyperpush_8_docs_bug_misfiled"
MISSING_COVERAGE_EVIDENCE_ID = "pitch_route_missing_tracker_coverage"
ACTIVE_NAMING_EVIDENCE_ID = "product_repo_naming_normalization"


class LedgerError(RuntimeError):
    pass


def require_int(value: Any, label: str) -> int:
    if not isinstance(value, int):
        raise LedgerError(f"{label} must be an integer")
    return value


def require_nullable_string(value: Any, label: str) -> str | None:
    if value is None:
        return None
    return require_string(value, label)


def issue_handle(repo: str, number: int) -> str:
    return f"{repo.split('/')[-1]}#{number}"


def read_generated_json(output_dir: Path, filename: str, label: str) -> dict[str, Any]:
    path = output_dir / filename
    if not path.is_file():
        raise LedgerError(f"missing {path}")
    try:
        payload = json.loads(path.read_text())
    except json.JSONDecodeError as exc:
        raise LedgerError(f"{path} is not valid JSON: {exc}") from exc
    return require_object(payload, label)


def json_pointer_ref(path: str, json_pointer: str, note: str) -> dict[str, Any]:
    return {
        "path": path,
        "json_pointer": json_pointer,
        "note": note,
    }


def ref_key(ref: dict[str, Any]) -> tuple[Any, ...]:
    return (
        ref.get("path"),
        ref.get("line"),
        ref.get("json_pointer"),
        ref.get("note"),
    )


def normalize_ref(ref: Any, label: str) -> dict[str, Any]:
    ref_object = require_object(ref, label)
    path = require_string(ref_object.get("path"), f"{label}.path")
    note = require_string(ref_object.get("note"), f"{label}.note")
    line = ref_object.get("line")
    if line is not None and not isinstance(line, int):
        raise LedgerError(f"{label}.line must be an integer or null")
    json_pointer = ref_object.get("json_pointer")
    if json_pointer is not None and not isinstance(json_pointer, str):
        raise LedgerError(f"{label}.json_pointer must be a string or null")
    normalized = {"path": path, "note": note}
    if line is not None:
        normalized["line"] = line
    if json_pointer is not None:
        normalized["json_pointer"] = json_pointer
    return normalized


def dedupe_refs(refs: list[dict[str, Any]]) -> list[dict[str, Any]]:
    seen: set[tuple[Any, ...]] = set()
    deduped: list[dict[str, Any]] = []
    for ref in refs:
        key = ref_key(ref)
        if key in seen:
            continue
        seen.add(key)
        deduped.append(ref)
    return deduped


def project_field_values(project_item: dict[str, Any]) -> dict[str, Any]:
    field_values = require_object(project_item.get("field_values"), "project_item.field_values")
    values: dict[str, Any] = {}
    for field_key, payload in field_values.items():
        field_payload = require_object(payload, f"project_item.field_values[{field_key!r}]")
        values[field_key] = field_payload.get("value")
    return values


def build_issue_locations(snapshots: dict[str, dict[str, Any]]) -> tuple[list[dict[str, Any]], dict[str, dict[str, Any]]]:
    rows: list[dict[str, Any]] = []
    by_url: dict[str, dict[str, Any]] = {}
    for snapshot_key in ("mesh_lang_issues", "hyperpush_issues"):
        snapshot = require_object(snapshots.get(snapshot_key), snapshot_key)
        issues = require_array(snapshot.get("issues"), f"{snapshot_key}.issues")
        snapshot_filename = SNAPSHOT_FILES[snapshot_key]
        for issue_index, issue in enumerate(issues):
            issue_object = require_object(issue, f"{snapshot_key}.issues[{issue_index}]")
            canonical_issue_url = require_string(
                issue_object.get("canonical_issue_url"),
                f"{snapshot_key}.issues[{issue_index}].canonical_issue_url",
            )
            if canonical_issue_url in by_url:
                raise LedgerError(f"duplicate canonical issue URL across repo snapshots: {canonical_issue_url}")
            row = {
                "snapshot_key": snapshot_key,
                "snapshot_filename": snapshot_filename,
                "issue_index": issue_index,
                "issue": issue_object,
            }
            by_url[canonical_issue_url] = row
            rows.append(row)
    rows.sort(
        key=lambda item: (
            require_string(item["issue"].get("repo"), "issue.repo"),
            require_int(item["issue"].get("number"), "issue.number"),
        )
    )
    return rows, by_url


def build_project_item_locations(project_items_snapshot: dict[str, Any]) -> dict[str, dict[str, Any]]:
    items = require_array(project_items_snapshot.get("items"), "project_items.items")
    by_url: dict[str, dict[str, Any]] = {}
    for item_index, item in enumerate(items):
        item_object = require_object(item, f"project_items.items[{item_index}]")
        canonical_issue_url = require_string(
            item_object.get("canonical_issue_url"),
            f"project_items.items[{item_index}].canonical_issue_url",
        )
        if canonical_issue_url in by_url:
            raise LedgerError(f"duplicate project item canonical issue URL {canonical_issue_url}")
        by_url[canonical_issue_url] = {
            "item_index": item_index,
            "item": item_object,
        }
    return by_url


def build_evidence_indexes(evidence: dict[str, Any]) -> tuple[dict[str, dict[str, Any]], dict[str, list[dict[str, Any]]], dict[str, int]]:
    entries = require_array(evidence.get("entries"), "evidence.entries")
    entry_by_id: dict[str, dict[str, Any]] = {}
    entries_by_issue_url: dict[str, list[dict[str, Any]]] = {}
    entry_locations: dict[str, int] = {}
    for entry_index, entry in enumerate(entries):
        entry_object = require_object(entry, f"evidence.entries[{entry_index}]")
        evidence_id = require_string(entry_object.get("evidence_id"), f"evidence.entries[{entry_index}].evidence_id")
        if evidence_id in entry_by_id:
            raise LedgerError(f"duplicate evidence_id {evidence_id!r}")
        entry_by_id[evidence_id] = entry_object
        entry_locations[evidence_id] = entry_index
        matched_issue_urls = require_array(entry_object.get("matched_issue_urls"), f"{evidence_id}.matched_issue_urls")
        for matched_index, issue_url in enumerate(matched_issue_urls):
            canonical_issue_url = require_string(issue_url, f"{evidence_id}.matched_issue_urls[{matched_index}]")
            entries_by_issue_url.setdefault(canonical_issue_url, []).append(entry_object)
    for canonical_issue_url, issue_entries in entries_by_issue_url.items():
        if len(issue_entries) > 1:
            evidence_ids = ", ".join(
                require_string(entry.get("evidence_id"), "evidence_id") for entry in issue_entries
            )
            raise LedgerError(
                f"ambiguous evidence match for {canonical_issue_url}: {evidence_ids}"
            )
    return entry_by_id, entries_by_issue_url, entry_locations


def build_naming_indexes(naming_map: dict[str, Any]) -> tuple[dict[str, dict[str, Any]], dict[str, int], dict[str, dict[str, Any]], dict[str, int]]:
    surfaces = require_array(naming_map.get("surfaces"), "naming_map.surfaces")
    surface_by_id: dict[str, dict[str, Any]] = {}
    surface_locations: dict[str, int] = {}
    for surface_index, surface in enumerate(surfaces):
        surface_object = require_object(surface, f"naming_map.surfaces[{surface_index}]")
        surface_id = require_string(surface_object.get("surface_id"), f"naming_map.surfaces[{surface_index}].surface_id")
        if surface_id in surface_by_id:
            raise LedgerError(f"duplicate naming surface_id {surface_id!r}")
        surface_by_id[surface_id] = surface_object
        surface_locations[surface_id] = surface_index

    findings = require_array(naming_map.get("drift_findings"), "naming_map.drift_findings")
    finding_by_id: dict[str, dict[str, Any]] = {}
    finding_locations: dict[str, int] = {}
    for finding_index, finding in enumerate(findings):
        finding_object = require_object(finding, f"naming_map.drift_findings[{finding_index}]")
        finding_id = require_string(
            finding_object.get("finding_id"),
            f"naming_map.drift_findings[{finding_index}].finding_id",
        )
        if finding_id in finding_by_id:
            raise LedgerError(f"duplicate naming drift finding_id {finding_id!r}")
        finding_by_id[finding_id] = finding_object
        finding_locations[finding_id] = finding_index
    return surface_by_id, surface_locations, finding_by_id, finding_locations


def resolve_surface_id(issue: dict[str, Any], matched_entry: dict[str, Any] | None) -> str:
    repo = require_string(issue.get("repo"), "issue.repo")
    if matched_entry is not None:
        evidence_id = require_string(matched_entry.get("evidence_id"), "matched_entry.evidence_id")
        if evidence_id == MISFILED_EVIDENCE_ID:
            return "mesh_lang_docs_packages_nav"
        if evidence_id == PARTIAL_EVIDENCE_ID:
            return "frontend_exp_operator_app"
    if repo == MESH_LANG_REPO:
        return "mesh_lang_repo"
    return "hyperpush_product_repo"


def row_needs_naming_drift(issue: dict[str, Any], matched_entry: dict[str, Any] | None) -> bool:
    repo = require_string(issue.get("repo"), "issue.repo")
    if matched_entry is not None and require_string(matched_entry.get("evidence_id"), "matched_entry.evidence_id") == MISFILED_EVIDENCE_ID:
        return True
    if repo != HYPERPUSH_REPO:
        return False
    title = require_string(issue.get("title"), "issue.title")
    body = require_string(issue.get("body"), "issue.body")
    combined = f"{title}\n{body}".lower()
    return any(token in combined for token in ("hyperpush-mono", "frontend-exp", "landing", "mesher"))


def is_rewrite_split_candidate(issue: dict[str, Any], project_item: dict[str, Any] | None) -> bool:
    title = require_string(issue.get("title"), "issue.title").lower()
    if "umbrella" in title:
        return True
    if project_item is None:
        return False
    values = project_field_values(project_item)
    return values.get("track") in (None, "") or values.get("domain") in (None, "")


def build_issue_snapshot_ref(location: dict[str, Any], issue: dict[str, Any]) -> dict[str, Any]:
    repo = require_string(issue.get("repo"), "issue.repo")
    number = require_int(issue.get("number"), "issue.number")
    return json_pointer_ref(
        location["snapshot_filename"],
        f"/issues/{location['issue_index']}",
        f"Normalized repo snapshot row for {issue_handle(repo, number)}.",
    )


def build_project_snapshot_ref(location: dict[str, Any], issue: dict[str, Any]) -> dict[str, Any]:
    repo = require_string(issue.get("repo"), "issue.repo")
    number = require_int(issue.get("number"), "issue.number")
    return json_pointer_ref(
        SNAPSHOT_FILES["project_items"],
        f"/items/{location['item_index']}",
        f"Project item row joined to {issue_handle(repo, number)}.",
    )


def build_surface_refs(
    *,
    surface_id: str,
    surface_by_id: dict[str, dict[str, Any]],
    surface_locations: dict[str, int],
) -> list[dict[str, Any]]:
    surface = require_object(surface_by_id.get(surface_id), f"surface[{surface_id}]")
    refs = [
        json_pointer_ref(
            NAMING_MAP_FILENAME,
            f"/surfaces/{surface_locations[surface_id]}",
            f"Naming/ownership surface {surface_id} used for canonical repo truth.",
        )
    ]
    refs.extend(
        normalize_ref(ref, f"surface[{surface_id}].evidence_refs[{index}]")
        for index, ref in enumerate(require_array(surface.get("evidence_refs"), f"surface[{surface_id}].evidence_refs"))
    )
    return refs


def build_drift_refs(
    *,
    finding_by_id: dict[str, dict[str, Any]],
    finding_locations: dict[str, int],
) -> list[dict[str, Any]]:
    refs: list[dict[str, Any]] = []
    for finding_id in ("product_repo_public_slug_alias", "compatibility_mesher_path_not_authoritative"):
        finding = finding_by_id.get(finding_id)
        if finding is None:
            continue
        refs.append(
            json_pointer_ref(
                NAMING_MAP_FILENAME,
                f"/drift_findings/{finding_locations[finding_id]}",
                f"Naming-drift finding {finding_id} applied to this row.",
            )
        )
        refs.extend(
            normalize_ref(ref, f"finding[{finding_id}].evidence_refs[{index}]")
            for index, ref in enumerate(require_array(finding.get("evidence_refs"), f"finding[{finding_id}].evidence_refs"))
        )
    return refs


def build_matched_entry_refs(matched_entry: dict[str, Any], entry_locations: dict[str, int]) -> list[dict[str, Any]]:
    evidence_id = require_string(matched_entry.get("evidence_id"), "matched_entry.evidence_id")
    refs = [
        json_pointer_ref(
            EVIDENCE_JSON_FILENAME,
            f"/entries/{entry_locations[evidence_id]}",
            f"Matched reconciliation evidence entry {evidence_id}.",
        )
    ]
    refs.extend(
        normalize_ref(ref, f"matched_entry[{evidence_id}].evidence_refs[{index}]")
        for index, ref in enumerate(require_array(matched_entry.get("evidence_refs"), f"matched_entry[{evidence_id}].evidence_refs"))
    )
    return refs


def derive_default_delivery_truth(issue: dict[str, Any], primary_bucket: str, naming_drift: bool) -> str:
    repo = require_string(issue.get("repo"), "issue.repo")
    repo_label = "language" if repo == MESH_LANG_REPO else "product"
    if primary_bucket == "rewrite-split":
        truth = (
            f"current reconciliation evidence does not show this {repo_label} row as shipped, but its project metadata is incomplete or too broad after the repo split, so the tracker scope needs rewrite/split before downstream execution"
        )
    else:
        truth = (
            f"current reconciliation evidence does not show this {repo_label} row as already shipped, so it should stay open until implementation proof exists"
        )
    if naming_drift:
        truth += "; repo/surface wording should also normalize to the canonical public hyperpush ownership truth"
    return truth


def build_action_plan(
    *,
    issue: dict[str, Any],
    project_item: dict[str, Any] | None,
    matched_entry: dict[str, Any] | None,
    primary_bucket: str,
    naming_drift: bool,
) -> tuple[str, str, str, str]:
    title = require_string(issue.get("title"), "issue.title")
    if matched_entry is not None:
        evidence_id = require_string(matched_entry.get("evidence_id"), "matched_entry.evidence_id")
        tracker_action = require_string(matched_entry.get("proposed_tracker_action"), f"{evidence_id}.proposed_tracker_action")
        if evidence_id == SHIPPED_EVIDENCE_ID:
            project_kind = "remove_from_project" if project_item is not None else "leave_untracked"
            project_action = (
                "Remove the linked project item after the closeout note points at the shipped M053/M054 proof."
                if project_item is not None
                else "Leave this cleanup issue off the project once the closeout note is added."
            )
            return "close_as_shipped", tracker_action, project_kind, project_action
        if evidence_id == MISFILED_EVIDENCE_ID:
            return (
                "move_to_mesh_lang",
                tracker_action,
                "create_project_item",
                "Create a replacement Mesh-domain project item once the issue is recreated under mesh-lang, and keep the stale hyperpush row off project #1.",
            )
        if evidence_id == PARTIAL_EVIDENCE_ID:
            project_kind = "update_project_item" if project_item is not None else "leave_untracked"
            project_action = (
                "Keep the project item active, but update its title/notes so it explicitly tracks mock-backed frontend-exp follow-through instead of implied shipped operator behavior."
                if project_item is not None
                else "Keep the repo issue open and unprojected until a truthful replacement project item is needed."
            )
            return "keep_open", tracker_action, project_kind, project_action

    if primary_bucket == "rewrite-split":
        project_kind = "update_project_item" if project_item is not None else "create_project_item"
        project_action = (
            "Update the project item title/fields to match the rewritten or split scope before execution continues."
            if project_item is not None
            else "Create a replacement project item only after the issue is rewritten into concrete executable scope."
        )
        return (
            "rewrite_scope",
            "Rewrite or split this tracker row into concrete executable scope that matches the post-split repo and board reality.",
            project_kind,
            project_action,
        )

    if naming_drift:
        project_kind = "update_project_item" if project_item is not None else "leave_untracked"
        project_action = (
            "Keep the project item active, but normalize its wording to the canonical public hyperpush repo and current ownership surface."
            if project_item is not None
            else "Keep this repo issue off the project unless it is later reintroduced with normalized naming and ownership wording."
        )
        return (
            "keep_open",
            "Keep this issue open, but normalize repo/surface wording to the canonical public hyperpush ownership truth before downstream execution.",
            project_kind,
            project_action,
        )

    project_kind = "keep_in_project" if project_item is not None else "leave_untracked"
    project_action = (
        "Keep the linked project item active; current code reality does not provide shipped proof that would justify removing it yet."
        if project_item is not None
        else "Leave this issue unprojected for now; it is one of the known repo-only rows outside the current board snapshot."
    )
    return (
        "keep_open",
        f"Keep {title!r} open; current code reality does not provide shipped proof that would justify closing it yet.",
        project_kind,
        project_action,
    )


def build_row(
    *,
    issue_location: dict[str, Any],
    project_location: dict[str, Any] | None,
    matched_entry: dict[str, Any] | None,
    entry_locations: dict[str, int],
    surface_by_id: dict[str, dict[str, Any]],
    surface_locations: dict[str, int],
    finding_by_id: dict[str, dict[str, Any]],
    finding_locations: dict[str, int],
) -> dict[str, Any]:
    issue = require_object(issue_location.get("issue"), "issue_location.issue")
    project_item = project_location["item"] if project_location is not None else None
    repo = require_string(issue.get("repo"), "issue.repo")
    number = require_int(issue.get("number"), "issue.number")
    canonical_issue_url = require_string(issue.get("canonical_issue_url"), "issue.canonical_issue_url")
    title = require_string(issue.get("title"), "issue.title")

    surface_id = resolve_surface_id(issue, matched_entry)
    surface = require_object(surface_by_id.get(surface_id), f"surface[{surface_id}]")
    naming_drift = row_needs_naming_drift(issue, matched_entry)

    if matched_entry is not None:
        evidence_id = require_string(matched_entry.get("evidence_id"), "matched_entry.evidence_id")
        if evidence_id == SHIPPED_EVIDENCE_ID:
            primary_bucket = "shipped-but-open"
        elif evidence_id == MISFILED_EVIDENCE_ID:
            primary_bucket = "misfiled"
        elif evidence_id == PARTIAL_EVIDENCE_ID:
            primary_bucket = "keep-open"
        else:
            raise LedgerError(f"unexpected issue-backed evidence entry {evidence_id!r}")
        ownership_truth = require_string(matched_entry.get("ownership_truth"), f"{evidence_id}.ownership_truth")
        delivery_truth = require_string(matched_entry.get("delivery_truth"), f"{evidence_id}.delivery_truth")
        workspace_path_truth = require_string(matched_entry.get("workspace_path_truth"), f"{evidence_id}.workspace_path_truth")
        public_repo_truth = require_string(matched_entry.get("public_repo_truth"), f"{evidence_id}.public_repo_truth")
        normalized_canonical_destination = require_object(
            matched_entry.get("normalized_canonical_destination"),
            f"{evidence_id}.normalized_canonical_destination",
        )
        matched_evidence_ids = [evidence_id]
    else:
        primary_bucket = "rewrite-split" if is_rewrite_split_candidate(issue, project_item) else "keep-open"
        ownership_truth = require_string(surface.get("ownership_truth"), f"surface[{surface_id}].ownership_truth")
        delivery_truth = derive_default_delivery_truth(issue, primary_bucket, naming_drift)
        workspace_path_truth = require_string(surface.get("workspace_path_truth"), f"surface[{surface_id}].workspace_path_truth")
        public_repo_truth = require_string(surface.get("public_repo_truth"), f"surface[{surface_id}].public_repo_truth")
        normalized_canonical_destination = require_object(
            surface.get("normalized_canonical_destination"),
            f"surface[{surface_id}].normalized_canonical_destination",
        )
        matched_evidence_ids = []

    repo_action_kind, repo_action, project_action_kind, project_action = build_action_plan(
        issue=issue,
        project_item=project_item,
        matched_entry=matched_entry,
        primary_bucket=primary_bucket,
        naming_drift=naming_drift,
    )

    refs = [build_issue_snapshot_ref(issue_location, issue)]
    if project_location is not None:
        refs.append(build_project_snapshot_ref(project_location, issue))
    refs.extend(
        build_surface_refs(
            surface_id=surface_id,
            surface_by_id=surface_by_id,
            surface_locations=surface_locations,
        )
    )
    if matched_entry is not None:
        refs.extend(build_matched_entry_refs(matched_entry, entry_locations))
    if naming_drift:
        refs.extend(
            build_drift_refs(
                finding_by_id=finding_by_id,
                finding_locations=finding_locations,
            )
        )
    refs = dedupe_refs(refs)

    row = {
        "canonical_issue_url": canonical_issue_url,
        "canonical_issue_handle": issue_handle(repo, number),
        "repo": repo,
        "number": number,
        "title": title,
        "state": require_string(issue.get("state"), "issue.state"),
        "project_item_id": require_nullable_string(
            project_item.get("project_item_id") if project_item is not None else None,
            f"{canonical_issue_url}.project_item_id",
        ),
        "project_backed": project_item is not None,
        "project_fields": project_field_values(project_item) if project_item is not None else {},
        "labels": [
            require_string(label.get("name"), f"{canonical_issue_url}.labels[{index}].name")
            for index, label in enumerate(require_array(issue.get("labels"), f"{canonical_issue_url}.labels"))
        ],
        "primary_audit_bucket": primary_bucket,
        "secondary_audit_buckets": ["naming-drift"] if naming_drift else [],
        "matched_evidence_ids": matched_evidence_ids,
        "naming_surface_id": surface_id,
        "ownership_truth": ownership_truth,
        "delivery_truth": delivery_truth,
        "workspace_path_truth": workspace_path_truth,
        "public_repo_truth": public_repo_truth,
        "normalized_canonical_destination": normalized_canonical_destination,
        "proposed_repo_action_kind": repo_action_kind,
        "proposed_repo_action": repo_action,
        "proposed_project_action_kind": project_action_kind,
        "proposed_project_action": project_action,
        "evidence_refs": refs,
    }
    return row


def build_derived_gap(
    *,
    entry: dict[str, Any],
    entry_locations: dict[str, int],
    surface_by_id: dict[str, dict[str, Any]],
    surface_locations: dict[str, int],
) -> dict[str, Any]:
    evidence_id = require_string(entry.get("evidence_id"), "gap_entry.evidence_id")
    if evidence_id != MISSING_COVERAGE_EVIDENCE_ID:
        raise LedgerError(f"unexpected derived gap evidence entry {evidence_id!r}")
    derived_gap = require_object(entry.get("derived_gap"), f"{evidence_id}.derived_gap")
    gap_id = require_string(derived_gap.get("gap_id"), f"{evidence_id}.derived_gap.gap_id")
    surface_id = "product_pitch_route"
    refs = build_surface_refs(
        surface_id=surface_id,
        surface_by_id=surface_by_id,
        surface_locations=surface_locations,
    )
    refs.extend(build_matched_entry_refs(entry, entry_locations))
    refs = dedupe_refs(refs)
    return {
        "gap_id": gap_id,
        "surface": require_string(derived_gap.get("surface"), f"{evidence_id}.derived_gap.surface"),
        "bucket": "missing-coverage",
        "summary": require_string(entry.get("summary"), f"{evidence_id}.summary"),
        "ownership_truth": require_string(entry.get("ownership_truth"), f"{evidence_id}.ownership_truth"),
        "delivery_truth": require_string(entry.get("delivery_truth"), f"{evidence_id}.delivery_truth"),
        "workspace_path_truth": require_string(entry.get("workspace_path_truth"), f"{evidence_id}.workspace_path_truth"),
        "public_repo_truth": require_string(entry.get("public_repo_truth"), f"{evidence_id}.public_repo_truth"),
        "normalized_canonical_destination": require_object(
            entry.get("normalized_canonical_destination"),
            f"{evidence_id}.normalized_canonical_destination",
        ),
        "proposed_repo_action_kind": "create_missing_issue",
        "proposed_repo_action": require_string(entry.get("proposed_tracker_action"), f"{evidence_id}.proposed_tracker_action"),
        "proposed_project_action_kind": "create_project_item",
        "proposed_project_action": "Create a project #1 item for the replacement /pitch tracking issue so the shipped surface is visible on the org board.",
        "evidence_refs": refs,
    }


def render_ref_location(ref: dict[str, Any]) -> str:
    path = require_string(ref.get("path"), "ref.path")
    line = ref.get("line")
    json_pointer = ref.get("json_pointer")
    location = path
    if isinstance(line, int):
        location = f"{location}:{line}"
    elif isinstance(json_pointer, str):
        location = f"{location}{json_pointer}"
    return location


def render_rows_table(rows: list[dict[str, Any]]) -> list[str]:
    lines = [
        "| Issue | Project item | Repo action | Project action | Evidence refs |",
        "| --- | --- | --- | --- | --- |",
    ]
    for row in rows:
        project_item_id = row.get("project_item_id") or "_none_"
        evidence_count = len(require_array(row.get("evidence_refs"), "row.evidence_refs"))
        lines.append(
            "| `{}` | `{}` | `{}` | `{}` | `{}` |".format(
                require_string(row.get("canonical_issue_handle"), "row.canonical_issue_handle"),
                project_item_id,
                require_string(row.get("proposed_repo_action_kind"), "row.proposed_repo_action_kind"),
                require_string(row.get("proposed_project_action_kind"), "row.proposed_project_action_kind"),
                evidence_count,
            )
        )
    return lines


def render_audit_markdown(ledger: dict[str, Any]) -> str:
    rollup = require_object(ledger.get("rollup"), "ledger.rollup")
    rows = [require_object(row, "ledger.row") for row in require_array(ledger.get("rows"), "ledger.rows")]
    derived_gaps = [require_object(gap, "ledger.derived_gap") for gap in require_array(ledger.get("derived_gaps"), "ledger.derived_gaps")]

    primary_groups = {bucket: [] for bucket in sorted(PRIMARY_BUCKETS)}
    naming_drift_rows: list[dict[str, Any]] = []
    for row in rows:
        bucket = require_string(row.get("primary_audit_bucket"), "row.primary_audit_bucket")
        primary_groups.setdefault(bucket, []).append(row)
        secondary = require_array(row.get("secondary_audit_buckets"), "row.secondary_audit_buckets")
        if "naming-drift" in secondary:
            naming_drift_rows.append(row)

    lines = [
        "# M057 S01 Reconciliation Audit",
        "",
        f"- Version: `{require_string(ledger.get('version'), 'ledger.version')}`",
        f"- Inventory captured_at: `{require_string(ledger.get('inventory_captured_at'), 'ledger.inventory_captured_at')}`",
        f"- Generated at: `{require_string(ledger.get('generated_at'), 'ledger.generated_at')}`",
        f"- Ledger rows: `{require_int(rollup.get('rows_total'), 'ledger.rollup.rows_total')}`",
        f"- Project-backed rows: `{require_int(rollup.get('project_backed_rows'), 'ledger.rollup.project_backed_rows')}`",
        f"- Non-project rows: `{require_int(rollup.get('non_project_rows'), 'ledger.rollup.non_project_rows')}`",
        f"- Derived gaps: `{require_int(rollup.get('derived_gap_count'), 'ledger.rollup.derived_gap_count')}`",
        "",
        "## Rollup",
        "",
        "| Bucket | Count |",
        "| --- | --- |",
        f"| `shipped-but-open` | `{require_int(require_object(rollup.get('primary_bucket_counts'), 'primary_bucket_counts').get('shipped-but-open'), 'primary_bucket_counts.shipped-but-open')}` |",
        f"| `rewrite/split` | `{require_int(require_object(rollup.get('primary_bucket_counts'), 'primary_bucket_counts').get('rewrite-split'), 'primary_bucket_counts.rewrite-split')}` |",
        f"| `keep-open` | `{require_int(require_object(rollup.get('primary_bucket_counts'), 'primary_bucket_counts').get('keep-open'), 'primary_bucket_counts.keep-open')}` |",
        f"| `misfiled` | `{require_int(require_object(rollup.get('primary_bucket_counts'), 'primary_bucket_counts').get('misfiled'), 'primary_bucket_counts.misfiled')}` |",
        f"| `missing-coverage` | `{len(derived_gaps)}` |",
        f"| `naming-drift` | `{len(naming_drift_rows)}` |",
    ]

    section_order = [
        ("shipped-but-open", "shipped-but-open"),
        ("rewrite/split", "rewrite-split"),
        ("keep-open", "keep-open"),
        ("misfiled", "misfiled"),
    ]
    for heading, bucket in section_order:
        bucket_rows = primary_groups.get(bucket, [])
        lines.extend(["", f"## {heading}", ""])
        if not bucket_rows:
            lines.append("_none_")
            continue
        lines.extend(render_rows_table(bucket_rows))

    lines.extend(["", "## missing-coverage", ""])
    if not derived_gaps:
        lines.append("_none_")
    else:
        lines.extend(
            [
                "| Gap | Repo action | Project action | Evidence refs |",
                "| --- | --- | --- | --- |",
            ]
        )
        for gap in derived_gaps:
            lines.append(
                "| `{}` | `{}` | `{}` | `{}` |".format(
                    require_string(gap.get("surface"), "gap.surface"),
                    require_string(gap.get("proposed_repo_action_kind"), "gap.proposed_repo_action_kind"),
                    require_string(gap.get("proposed_project_action_kind"), "gap.proposed_project_action_kind"),
                    len(require_array(gap.get("evidence_refs"), "gap.evidence_refs")),
                )
            )

    lines.extend(["", "## naming-drift", ""])
    if not naming_drift_rows:
        lines.append("_none_")
    else:
        lines.extend(
            [
                "| Issue | Primary bucket | Repo truth | Workspace truth |",
                "| --- | --- | --- | --- |",
            ]
        )
        for row in naming_drift_rows:
            lines.append(
                "| `{}` | `{}` | `{}` | `{}` |".format(
                    require_string(row.get("canonical_issue_handle"), "row.canonical_issue_handle"),
                    require_string(row.get("primary_audit_bucket"), "row.primary_audit_bucket"),
                    require_string(row.get("public_repo_truth"), "row.public_repo_truth"),
                    require_string(row.get("workspace_path_truth"), "row.workspace_path_truth"),
                )
            )

    lines.extend(["", "## Derived gap evidence", ""])
    for gap in derived_gaps:
        lines.extend(
            [
                f"### {require_string(gap.get('surface'), 'gap.surface')}",
                "",
                require_string(gap.get("summary"), "gap.summary"),
                "",
                f"- proposed_repo_action: {require_string(gap.get('proposed_repo_action'), 'gap.proposed_repo_action')}",
                f"- proposed_project_action: {require_string(gap.get('proposed_project_action'), 'gap.proposed_project_action')}",
                "- evidence_refs:",
            ]
        )
        for ref in require_array(gap.get("evidence_refs"), "gap.evidence_refs"):
            ref_object = normalize_ref(ref, "gap.evidence_ref")
            lines.append(f"  - `{render_ref_location(ref_object)}` — {require_string(ref_object.get('note'), 'gap.evidence_ref.note')}")

    return "\n".join(lines) + "\n"


def validate_ledger_bundle(ledger: dict[str, Any], audit_markdown: str, snapshots: dict[str, dict[str, Any]]) -> dict[str, Any]:
    if require_string(ledger.get("version"), "ledger.version") != LEDGER_VERSION:
        raise LedgerError("ledger version drifted")

    issue_locations, issue_by_url = build_issue_locations(snapshots)
    project_items_snapshot = require_object(snapshots.get("project_items"), "project_items")
    project_by_url = build_project_item_locations(project_items_snapshot)
    rows = [require_object(row, "ledger.row") for row in require_array(ledger.get("rows"), "ledger.rows")]
    if len(rows) != EXPECTED_COUNTS["combined_total"]:
        raise LedgerError(
            f"ledger expected {EXPECTED_COUNTS['combined_total']} rows but found {len(rows)}"
        )

    seen_urls: set[str] = set()
    project_backed_rows = 0
    non_project_urls: set[str] = set()
    primary_bucket_counts = {bucket: 0 for bucket in PRIMARY_BUCKETS}
    naming_drift_rows = 0
    for row_index, row in enumerate(rows):
        canonical_issue_url = require_string(row.get("canonical_issue_url"), f"ledger.rows[{row_index}].canonical_issue_url")
        if canonical_issue_url in seen_urls:
            raise LedgerError(f"duplicate ledger canonical_issue_url {canonical_issue_url}")
        seen_urls.add(canonical_issue_url)
        if canonical_issue_url not in issue_by_url:
            raise LedgerError(f"ledger row missing from repo issue snapshots: {canonical_issue_url}")

        require_string(row.get("canonical_issue_handle"), f"ledger.rows[{row_index}].canonical_issue_handle")
        require_string(row.get("ownership_truth"), f"ledger.rows[{row_index}].ownership_truth")
        require_string(row.get("delivery_truth"), f"ledger.rows[{row_index}].delivery_truth")
        require_string(row.get("workspace_path_truth"), f"ledger.rows[{row_index}].workspace_path_truth")
        require_string(row.get("public_repo_truth"), f"ledger.rows[{row_index}].public_repo_truth")
        require_object(row.get("normalized_canonical_destination"), f"ledger.rows[{row_index}].normalized_canonical_destination")
        repo_action_kind = require_string(row.get("proposed_repo_action_kind"), f"ledger.rows[{row_index}].proposed_repo_action_kind")
        if repo_action_kind not in REPO_ACTION_KINDS:
            raise LedgerError(f"unknown proposed_repo_action_kind {repo_action_kind!r}")
        require_string(row.get("proposed_repo_action"), f"ledger.rows[{row_index}].proposed_repo_action")
        project_action_kind = require_string(row.get("proposed_project_action_kind"), f"ledger.rows[{row_index}].proposed_project_action_kind")
        if project_action_kind not in PROJECT_ACTION_KINDS:
            raise LedgerError(f"unknown proposed_project_action_kind {project_action_kind!r}")
        require_string(row.get("proposed_project_action"), f"ledger.rows[{row_index}].proposed_project_action")

        primary_bucket = require_string(row.get("primary_audit_bucket"), f"ledger.rows[{row_index}].primary_audit_bucket")
        if primary_bucket not in PRIMARY_BUCKETS:
            raise LedgerError(f"unknown primary_audit_bucket {primary_bucket!r}")
        primary_bucket_counts[primary_bucket] += 1

        secondary_buckets = require_array(row.get("secondary_audit_buckets"), f"ledger.rows[{row_index}].secondary_audit_buckets")
        for secondary_index, secondary_bucket in enumerate(secondary_buckets):
            secondary_value = require_string(secondary_bucket, f"ledger.rows[{row_index}].secondary_audit_buckets[{secondary_index}]")
            if secondary_value not in SECONDARY_BUCKETS:
                raise LedgerError(f"unknown secondary_audit_bucket {secondary_value!r}")
            if secondary_value == "naming-drift":
                naming_drift_rows += 1

        evidence_refs = [
            normalize_ref(ref, f"ledger.rows[{row_index}].evidence_refs[{ref_index}]")
            for ref_index, ref in enumerate(require_array(row.get("evidence_refs"), f"ledger.rows[{row_index}].evidence_refs"))
        ]
        if not evidence_refs:
            raise LedgerError(f"ledger row {canonical_issue_url} must include evidence_refs")

        project_backed = bool(row.get("project_backed"))
        project_item_id = row.get("project_item_id")
        if project_backed:
            if canonical_issue_url not in project_by_url:
                raise LedgerError(f"project-backed row missing project snapshot entry: {canonical_issue_url}")
            project_item_id = require_string(project_item_id, f"ledger.rows[{row_index}].project_item_id")
            if project_item_id.strip() == "":
                raise LedgerError(f"project-backed row cannot have blank project_item_id: {canonical_issue_url}")
            project_backed_rows += 1
        else:
            if project_item_id is not None:
                raise LedgerError(f"non-project row must leave project_item_id null: {canonical_issue_url}")
            non_project_urls.add(canonical_issue_url)

    if seen_urls != set(issue_by_url):
        missing_urls = sorted(set(issue_by_url) - seen_urls)
        raise LedgerError(f"ledger missing repo issue rows: {', '.join(missing_urls)}")
    if project_backed_rows != EXPECTED_COUNTS["project_items_total"]:
        raise LedgerError(
            f"ledger expected {EXPECTED_COUNTS['project_items_total']} project-backed rows but found {project_backed_rows}"
        )
    if len(non_project_urls) != len(EXPECTED_NON_PROJECT_HYPERPUSH_ISSUES):
        raise LedgerError(
            f"ledger expected {len(EXPECTED_NON_PROJECT_HYPERPUSH_ISSUES)} non-project rows but found {len(non_project_urls)}"
        )

    expected_non_project_urls = {
        f"https://github.com/{HYPERPUSH_REPO}/issues/{number}" for number in EXPECTED_NON_PROJECT_HYPERPUSH_ISSUES
    }
    if non_project_urls != expected_non_project_urls:
        raise LedgerError(
            "non-project row set drifted: expected {} but found {}".format(
                sorted(expected_non_project_urls),
                sorted(non_project_urls),
            )
        )

    derived_gaps = [require_object(gap, "ledger.derived_gap") for gap in require_array(ledger.get("derived_gaps"), "ledger.derived_gaps")]
    if not derived_gaps:
        raise LedgerError("ledger must include at least one derived gap")
    gap_surface_found = False
    for gap_index, gap in enumerate(derived_gaps):
        gap_bucket = require_string(gap.get("bucket"), f"ledger.derived_gaps[{gap_index}].bucket")
        if gap_bucket != "missing-coverage":
            raise LedgerError(f"unexpected derived gap bucket {gap_bucket!r}")
        repo_action_kind = require_string(gap.get("proposed_repo_action_kind"), f"ledger.derived_gaps[{gap_index}].proposed_repo_action_kind")
        if repo_action_kind not in REPO_ACTION_KINDS:
            raise LedgerError(f"unknown derived gap proposed_repo_action_kind {repo_action_kind!r}")
        project_action_kind = require_string(gap.get("proposed_project_action_kind"), f"ledger.derived_gaps[{gap_index}].proposed_project_action_kind")
        if project_action_kind not in PROJECT_ACTION_KINDS:
            raise LedgerError(f"unknown derived gap proposed_project_action_kind {project_action_kind!r}")
        gap_refs = require_array(gap.get("evidence_refs"), f"ledger.derived_gaps[{gap_index}].evidence_refs")
        if not gap_refs:
            raise LedgerError(f"derived gap {gap_index} must include evidence_refs")
        if require_string(gap.get("surface"), f"ledger.derived_gaps[{gap_index}].surface") == "/pitch":
            gap_surface_found = True
    if not gap_surface_found:
        raise LedgerError("ledger missing /pitch missing-coverage gap")

    rollup = require_object(ledger.get("rollup"), "ledger.rollup")
    if require_int(rollup.get("rows_total"), "ledger.rollup.rows_total") != EXPECTED_COUNTS["combined_total"]:
        raise LedgerError("ledger.rollup.rows_total drifted")
    if require_int(rollup.get("project_backed_rows"), "ledger.rollup.project_backed_rows") != EXPECTED_COUNTS["project_items_total"]:
        raise LedgerError("ledger.rollup.project_backed_rows drifted")
    if require_int(rollup.get("non_project_rows"), "ledger.rollup.non_project_rows") != len(EXPECTED_NON_PROJECT_HYPERPUSH_ISSUES):
        raise LedgerError("ledger.rollup.non_project_rows drifted")
    if require_int(rollup.get("orphan_project_rows"), "ledger.rollup.orphan_project_rows") != 0:
        raise LedgerError("ledger.rollup.orphan_project_rows must remain 0")
    if require_int(rollup.get("derived_gap_count"), "ledger.rollup.derived_gap_count") != len(derived_gaps):
        raise LedgerError("ledger.rollup.derived_gap_count drifted")

    rollup_bucket_counts = require_object(rollup.get("primary_bucket_counts"), "ledger.rollup.primary_bucket_counts")
    for bucket, count in primary_bucket_counts.items():
        if require_int(rollup_bucket_counts.get(bucket), f"ledger.rollup.primary_bucket_counts[{bucket!r}]") != count:
            raise LedgerError(f"ledger primary bucket count drifted for {bucket}")
    if require_int(rollup.get("naming_drift_rows"), "ledger.rollup.naming_drift_rows") != naming_drift_rows:
        raise LedgerError("ledger.rollup.naming_drift_rows drifted")

    required_headings = [
        "## shipped-but-open",
        "## rewrite/split",
        "## keep-open",
        "## misfiled",
        "## missing-coverage",
        "## naming-drift",
    ]
    for heading in required_headings:
        if heading not in audit_markdown:
            raise LedgerError(f"audit markdown missing required section {heading!r}")
    if "/pitch" not in audit_markdown:
        raise LedgerError("audit markdown must mention /pitch missing coverage")

    return {
        "rows_total": len(rows),
        "project_backed_rows": project_backed_rows,
        "non_project_rows": len(non_project_urls),
        "naming_drift_rows": naming_drift_rows,
        "primary_bucket_counts": primary_bucket_counts,
        "derived_gap_count": len(derived_gaps),
    }


def build_outputs(*, source_root: Path, output_dir: Path) -> tuple[dict[str, Any], str, dict[str, Any]]:
    _ = source_root  # reserved for parity with sibling M057 builders and isolated tests
    snapshots = load_snapshots(output_dir)
    inventory_summary = validate_snapshots(snapshots)
    evidence = read_generated_json(output_dir, EVIDENCE_JSON_FILENAME, "evidence")
    naming_map = read_generated_json(output_dir, NAMING_MAP_FILENAME, "naming_map")
    validate_evidence_bundle(evidence)
    validate_naming_map(naming_map)

    issue_locations, issue_by_url = build_issue_locations(snapshots)
    project_items_snapshot = require_object(snapshots.get("project_items"), "project_items")
    project_by_url = build_project_item_locations(project_items_snapshot)
    evidence_by_id, entries_by_issue_url, entry_locations = build_evidence_indexes(evidence)
    surface_by_id, surface_locations, finding_by_id, finding_locations = build_naming_indexes(naming_map)

    rows: list[dict[str, Any]] = []
    for issue_location in issue_locations:
        issue = require_object(issue_location.get("issue"), "issue_location.issue")
        canonical_issue_url = require_string(issue.get("canonical_issue_url"), "issue.canonical_issue_url")
        project_location = project_by_url.get(canonical_issue_url)
        matched_entries = entries_by_issue_url.get(canonical_issue_url, [])
        if len(matched_entries) > 1:
            evidence_ids = ", ".join(
                require_string(entry.get("evidence_id"), "matched_entry.evidence_id") for entry in matched_entries
            )
            raise LedgerError(f"ambiguous issue classification for {canonical_issue_url}: {evidence_ids}")
        matched_entry = matched_entries[0] if matched_entries else None
        rows.append(
            build_row(
                issue_location=issue_location,
                project_location=project_location,
                matched_entry=matched_entry,
                entry_locations=entry_locations,
                surface_by_id=surface_by_id,
                surface_locations=surface_locations,
                finding_by_id=finding_by_id,
                finding_locations=finding_locations,
            )
        )

    issue_url_set = set(issue_by_url)
    project_url_set = set(project_by_url)
    orphan_project_rows = sorted(project_url_set - issue_url_set)
    if orphan_project_rows:
        raise LedgerError(f"orphan project rows detected: {', '.join(orphan_project_rows)}")

    derived_gaps: list[dict[str, Any]] = []
    if MISSING_COVERAGE_EVIDENCE_ID in evidence_by_id:
        derived_gaps.append(
            build_derived_gap(
                entry=require_object(evidence_by_id[MISSING_COVERAGE_EVIDENCE_ID], MISSING_COVERAGE_EVIDENCE_ID),
                entry_locations=entry_locations,
                surface_by_id=surface_by_id,
                surface_locations=surface_locations,
            )
        )

    primary_bucket_counts = {bucket: 0 for bucket in PRIMARY_BUCKETS}
    naming_drift_rows = 0
    for row in rows:
        primary_bucket_counts[require_string(row.get("primary_audit_bucket"), "row.primary_audit_bucket")] += 1
        if "naming-drift" in require_array(row.get("secondary_audit_buckets"), "row.secondary_audit_buckets"):
            naming_drift_rows += 1

    ledger = {
        "version": LEDGER_VERSION,
        "generated_at": iso_now(),
        "inventory_captured_at": require_string(inventory_summary.get("captured_at"), "inventory_summary.captured_at"),
        "source_script": SCRIPT_RELATIVE_PATH,
        "join_key": "canonical_issue_url",
        "rollup": {
            "rows_total": len(rows),
            "project_backed_rows": sum(1 for row in rows if row.get("project_backed")),
            "non_project_rows": sum(1 for row in rows if not row.get("project_backed")),
            "orphan_project_rows": len(orphan_project_rows),
            "naming_drift_rows": naming_drift_rows,
            "derived_gap_count": len(derived_gaps),
            "repo_counts": {
                MESH_LANG_REPO: sum(1 for row in rows if row.get("repo") == MESH_LANG_REPO),
                HYPERPUSH_REPO: sum(1 for row in rows if row.get("repo") == HYPERPUSH_REPO),
            },
            "primary_bucket_counts": primary_bucket_counts,
        },
        "rows": rows,
        "derived_gaps": derived_gaps,
        "decision_refs": require_array(evidence.get("decision_refs"), "evidence.decision_refs"),
    }
    audit_markdown = render_audit_markdown(ledger)
    check_summary = validate_ledger_bundle(ledger, audit_markdown, snapshots)
    return ledger, audit_markdown, check_summary


def write_outputs(*, output_dir: Path, ledger: dict[str, Any], audit_markdown: str) -> list[Path]:
    output_dir.mkdir(parents=True, exist_ok=True)
    ledger_path = output_dir / LEDGER_JSON_FILENAME
    audit_path = output_dir / AUDIT_MD_FILENAME
    write_json_atomic(ledger_path, ledger)
    audit_path.write_text(audit_markdown, encoding="utf8")
    return [ledger_path, audit_path]


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Build the M057 S01 reconciliation ledger and audit proof.")
    parser.add_argument("--source-root", type=Path, default=ROOT, help="Alternate source root for isolated contract tests.")
    parser.add_argument("--output-dir", type=Path, help="Directory containing the M057 snapshots/evidence and receiving ledger outputs.")
    parser.add_argument("--check", action="store_true", help="Validate the generated ledger and audit outputs.")
    args = parser.parse_args(argv)
    args.source_root = args.source_root.resolve()
    if args.output_dir is None:
        args.output_dir = (args.source_root / DEFAULT_OUTPUT_SUBDIR).resolve()
    else:
        args.output_dir = args.output_dir.resolve()
    return args


def main(argv: list[str]) -> int:
    args = parse_args(argv)
    ledger, audit_markdown, check_summary = build_outputs(
        source_root=args.source_root,
        output_dir=args.output_dir,
    )
    written_paths = write_outputs(output_dir=args.output_dir, ledger=ledger, audit_markdown=audit_markdown)

    print(
        json.dumps(
            {
                "status": "ok",
                "output_dir": str(args.output_dir),
                "written_files": [str(path) for path in written_paths],
                "rollup": require_object(ledger.get("rollup"), "ledger.rollup"),
                "check": check_summary if args.check else None,
            },
            indent=2,
        )
    )
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main(sys.argv[1:]))
    except (LedgerError, EvidenceError, InventoryError) as exc:
        print(str(exc), file=sys.stderr)
        raise SystemExit(1)
