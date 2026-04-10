#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path
from typing import Any

import yaml

from m057_reconciliation_ledger import AUDIT_MD_FILENAME, LEDGER_JSON_FILENAME, LedgerError, validate_ledger_bundle
from m057_tracker_inventory import (
    HYPERPUSH_REPO,
    InventoryError,
    MESH_LANG_REPO,
    ROOT,
    iso_now,
    load_snapshots,
    require_array,
    require_int,
    require_object,
    require_string,
    validate_snapshots,
    write_json_atomic,
)

SCRIPT_RELATIVE_PATH = "scripts/lib/m057_repo_mutation_plan.py"
PLAN_JSON_FILENAME = "repo-mutation-plan.json"
PLAN_MD_FILENAME = "repo-mutation-plan.md"
PLAN_VERSION = "m057-s02-repo-mutation-plan-v1"
DEFAULT_SOURCE_DIR = ROOT / ".gsd" / "milestones" / "M057" / "slices" / "S01"
DEFAULT_OUTPUT_DIR = ROOT / ".gsd" / "milestones" / "M057" / "slices" / "S02"
DEFAULT_TEMPLATE_HEADINGS = [
    "Problem statement",
    "Proposed solution",
    "Alternatives considered",
    "Expected impact",
    "Additional context",
]
EXPECTED_SOURCE_PRIMARY_BUCKET_COUNTS = {
    "shipped-but-open": 13,
    "rewrite-split": 21,
    "keep-open": 33,
    "misfiled": 1,
}
EXPECTED_PLAN_COUNTS = {
    "close": 10,
    "rewrite": 31,
    "transfer": 1,
    "create": 1,
    "skipped": 3,
    "total_apply": 43,
}
EXPECTED_SKIPPED_CLOSE_HANDLES = {"hyperpush#3", "hyperpush#4", "hyperpush#5"}
RECOGNIZED_LEDGER_ACTIONS = {
    "close_as_shipped",
    "keep_open",
    "rewrite_scope",
    "move_to_mesh_lang",
    "create_missing_issue",
}


class PlanError(RuntimeError):
    pass


def read_json(path: Path, label: str) -> dict[str, Any]:
    if not path.is_file():
        raise PlanError(f"missing {label}: {path}")
    try:
        payload = json.loads(path.read_text())
    except json.JSONDecodeError as exc:
        raise PlanError(f"{label} is not valid JSON: {path}\n{exc}") from exc
    return require_object(payload, label)


def read_text(path: Path, label: str) -> str:
    if not path.is_file():
        raise PlanError(f"missing {label}: {path}")
    return path.read_text(encoding="utf8")


def repo_issue_handle(repo_slug: str, number: int) -> str:
    return f"{repo_slug.split('/')[-1]}#{number}"


def normalize_heading_label(label: str) -> str:
    return re.sub(r"\s+", " ", label).strip()


def normalize_title_spaces(value: str) -> str:
    return re.sub(r"\s+", " ", value).strip()


def slugify(value: str) -> str:
    return re.sub(r"[^a-z0-9]+", "-", value.lower()).strip("-")


def normalize_public_wording(value: str) -> str:
    return value.replace("hyperpush-mono", "hyperpush")


def parse_template(path: Path, *, repo_slug: str) -> dict[str, Any]:
    try:
        raw = path.read_text(encoding="utf8")
    except OSError as exc:
        return {
            "repo_slug": repo_slug,
            "path": str(path),
            "readable": False,
            "fallback_used": True,
            "fallback_reason": f"unreadable template: {exc}",
            "renderer": "fallback-heading",
            "title_prefix": "[Feature]: ",
            "headings": DEFAULT_TEMPLATE_HEADINGS,
        }

    try:
        payload = yaml.safe_load(raw)
    except yaml.YAMLError as exc:
        return {
            "repo_slug": repo_slug,
            "path": str(path),
            "readable": False,
            "fallback_used": True,
            "fallback_reason": f"template parse error: {exc}",
            "renderer": "fallback-heading",
            "title_prefix": "[Feature]: ",
            "headings": DEFAULT_TEMPLATE_HEADINGS,
        }

    template = require_object(payload, f"template[{repo_slug}]")
    title_prefix = require_string(template.get("title"), f"template[{repo_slug}].title")
    body = require_array(template.get("body"), f"template[{repo_slug}].body")
    headings: list[str] = []
    for index, item in enumerate(body):
        body_item = require_object(item, f"template[{repo_slug}].body[{index}]")
        item_type = require_string(body_item.get("type"), f"template[{repo_slug}].body[{index}].type")
        if item_type not in {"textarea", "dropdown"}:
            continue
        attributes = require_object(body_item.get("attributes"), f"template[{repo_slug}].body[{index}].attributes")
        label = normalize_heading_label(require_string(attributes.get("label"), f"template[{repo_slug}].body[{index}].attributes.label"))
        if label not in headings:
            headings.append(label)

    if not headings:
        raise PlanError(
            f"template shape cannot provide stable section headings for {repo_slug}: {path}"
        )

    return {
        "repo_slug": repo_slug,
        "path": str(path),
        "readable": True,
        "fallback_used": False,
        "fallback_reason": None,
        "renderer": "template-heading",
        "title_prefix": title_prefix,
        "headings": headings,
    }


def parse_markdown_sections(markdown: str) -> tuple[str, list[dict[str, str]]]:
    heading_matches = list(re.finditer(r"^##\s+(.+?)\s*$", markdown, flags=re.MULTILINE))
    if not heading_matches:
        return markdown.strip(), []
    preamble = markdown[: heading_matches[0].start()].strip()
    sections: list[dict[str, str]] = []
    for index, match in enumerate(heading_matches):
        start = match.end()
        end = heading_matches[index + 1].start() if index + 1 < len(heading_matches) else len(markdown)
        sections.append(
            {
                "heading": normalize_heading_label(match.group(1)),
                "content": markdown[start:end].strip(),
            }
        )
    return preamble, sections


def section_map(sections: list[dict[str, str]]) -> dict[str, str]:
    return {section["heading"]: section["content"] for section in sections}


def extract_markdown_bullets(text: str) -> list[str]:
    bullets: list[str] = []
    for line in text.splitlines():
        stripped = line.strip()
        if stripped.startswith("- "):
            bullets.append(stripped[2:].strip())
    return bullets


def render_markdown(sections: list[tuple[str, str]]) -> str:
    rendered: list[str] = []
    for heading, content in sections:
        cleaned = content.strip()
        if cleaned == "":
            continue
        rendered.extend([f"## {heading}", cleaned, ""])
    return "\n".join(rendered).strip() + "\n"


def build_issue_indexes(snapshots: dict[str, dict[str, Any]]) -> tuple[dict[str, dict[str, Any]], dict[str, dict[str, Any]]]:
    by_handle: dict[str, dict[str, Any]] = {}
    by_url: dict[str, dict[str, Any]] = {}
    for snapshot_key in ("mesh_lang_issues", "hyperpush_issues"):
        snapshot = require_object(snapshots.get(snapshot_key), snapshot_key)
        issues = require_array(snapshot.get("issues"), f"{snapshot_key}.issues")
        for issue in issues:
            issue_object = require_object(issue, f"{snapshot_key}.issue")
            repo_slug = require_string(issue_object.get("repo"), f"{snapshot_key}.issue.repo")
            number = require_int(issue_object.get("number"), f"{snapshot_key}.issue.number")
            handle = repo_issue_handle(repo_slug, number)
            canonical_issue_url = require_string(issue_object.get("canonical_issue_url"), f"{handle}.canonical_issue_url")
            if handle in by_handle:
                raise PlanError(f"duplicate canonical issue handle in snapshots: {handle}")
            if canonical_issue_url in by_url:
                raise PlanError(f"duplicate canonical issue URL in snapshots: {canonical_issue_url}")
            by_handle[handle] = issue_object
            by_url[canonical_issue_url] = issue_object
    return by_handle, by_url


def ensure_source_counts(ledger: dict[str, Any]) -> None:
    rollup = require_object(ledger.get("rollup"), "ledger.rollup")
    bucket_counts = require_object(rollup.get("primary_bucket_counts"), "ledger.rollup.primary_bucket_counts")
    for bucket, expected in EXPECTED_SOURCE_PRIMARY_BUCKET_COUNTS.items():
        actual = require_int(bucket_counts.get(bucket), f"ledger.rollup.primary_bucket_counts[{bucket!r}]")
        if actual != expected:
            raise PlanError(f"source ledger bucket count drifted for {bucket}: expected {expected}, found {actual}")

    rows = [require_object(row, "ledger.row") for row in require_array(ledger.get("rows"), "ledger.rows")]
    if len(rows) != 68:
        raise PlanError(f"source ledger row count drifted: expected 68, found {len(rows)}")

    seen_handles: set[str] = set()
    for row in rows:
        handle = require_string(row.get("canonical_issue_handle"), "ledger.row.canonical_issue_handle")
        if handle in seen_handles:
            raise PlanError(f"duplicate canonical issue handle in source ledger: {handle}")
        seen_handles.add(handle)
        action_kind = require_string(row.get("proposed_repo_action_kind"), f"{handle}.proposed_repo_action_kind")
        if action_kind not in RECOGNIZED_LEDGER_ACTIONS:
            raise PlanError(f"unknown proposed_repo_action_kind {action_kind!r} for {handle}")
        project_backed = bool(row.get("project_backed"))
        project_item_id = row.get("project_item_id")
        if project_backed and not isinstance(project_item_id, str):
            raise PlanError(f"project-backed row missing project_item_id: {handle}")

    derived_gaps = [require_object(gap, "ledger.derived_gap") for gap in require_array(ledger.get("derived_gaps"), "ledger.derived_gaps")]
    if not derived_gaps:
        raise PlanError("source ledger must include derived_gaps")
    if not any(require_string(gap.get("surface"), "gap.surface") == "/pitch" for gap in derived_gaps):
        raise PlanError("source ledger missing /pitch derived_gaps entry")


def normalize_open_rewrite_title(row: dict[str, Any], current_title: str) -> str:
    handle = require_string(row.get("canonical_issue_handle"), "row.canonical_issue_handle")
    if handle == "hyperpush#24":
        return "Hyperpush launch roadmap (product-repo execution umbrella)"
    if handle == "hyperpush#54":
        return "Hyperpush deploy topology: split marketing site from operator app routing and product runtime boundaries"
    if handle == "hyperpush#55":
        return "Hyperpush deployment: add a production Dockerfile and container startup path for the operator app"
    if handle == "hyperpush#56":
        return "Hyperpush deployment: create generic-VM compose stack and health verification for the marketing site, operator app, and product backend"
    return normalize_title_spaces(current_title)


def build_close_comment(*, row: dict[str, Any], residual_children: list[str]) -> str:
    handle = require_string(row.get("canonical_issue_handle"), "row.canonical_issue_handle")
    lines = [
        "Closing as shipped. The S01 reconciliation ledger marked this row as stale shipped-but-open scope, and the delivered work is already covered by the M053/M054 Mesh launch foundations proof.",
        "",
        "Evidence: `.gsd/milestones/M053/M053-SUMMARY.md`, `.gsd/milestones/M054/M054-SUMMARY.md`.",
    ]
    if residual_children:
        lines.extend(
            [
                "",
                f"Residual follow-up remains open in {', '.join(residual_children)}. This closeout preserves the shipped parent history without hiding the remaining child work.",
            ]
        )
    lines.extend(["", f"Canonical issue: `{handle}`."])
    return "\n".join(lines).strip() + "\n"


def build_create_comment() -> str:
    return (
        "Closing as completed retroactively. The `/pitch` evaluator surface already shipped during M056, and this issue exists to make the repo issue history truthful and explicit.\n\n"
        "Evidence: `mesher/landing/app/pitch/page.tsx`, `.gsd/milestones/M056/M056-SUMMARY.md`.\n"
    )


def build_transfer_comment() -> str:
    return (
        "This issue transfers to `hyperpush-org/mesh-lang` so the existing bug history stays attached to the docs surface that actually owns the code.\n"
    )


def build_pitch_issue_title(template: dict[str, Any]) -> str:
    prefix = template.get("title_prefix") or ""
    return f"{prefix}record shipped /pitch evaluator route explicitly".strip()


def build_pitch_issue_body(template: dict[str, Any]) -> str:
    headings = [normalize_heading_label(require_string(item, "template.heading")) for item in template.get("headings", [])]
    section_values = {
        "Area": "landing app",
        "Problem statement": (
            "The evaluator-facing `/pitch` route already shipped in the product repo, but there is still no dedicated Hyperpush issue that records that surface directly."
        ),
        "Proposed solution": (
            "Create one retrospective product-repo issue for the shipped `/pitch` surface, point it at `mesher/landing/app/pitch/page.tsx`, and then close it as completed so repo issue history matches code truth."
        ),
        "Alternatives considered": (
            "Leaving `/pitch` implicit in milestone history keeps repo drift hidden, and repurposing an unrelated issue would destroy the audit trail for the shipped route."
        ),
        "Expected impact": (
            "S02 and S03 get an explicit canonical issue URL for `/pitch`, and maintainers can trust that the product repo issue list covers shipped evaluator-facing surfaces."
        ),
        "Additional context": (
            "- Code surface: `mesher/landing/app/pitch/page.tsx`\n"
            "- Shipped proof: `.gsd/milestones/M056/M056-SUMMARY.md`\n"
            "- This is a retrospective repo-truth issue, not new implementation work."
        ),
    }
    sections: list[tuple[str, str]] = []
    for heading in headings:
        sections.append((heading, section_values.get(heading, "See reconciliation evidence and M056 proof.")))
    return render_markdown(sections)


def build_special_issue_24_body(ledger_rows: list[dict[str, Any]]) -> str:
    product_epics = [
        row
        for row in ledger_rows
        if row.get("repo") == HYPERPUSH_REPO and row.get("number") in {11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23}
    ]
    product_epics.sort(key=lambda row: require_int(row.get("number"), "product_epic.number"))
    mesh_dependencies = [
        "hyperpush-org/mesh-lang#3 — Mesh clustered Docker-on-VM deploy baseline for Hyperpush",
        "hyperpush-org/mesh-lang#4 — Mesh runtime hardening for Hyperpush production load and clustering",
        "hyperpush-org/mesh-lang#5 — Mesh deploy and release primitives needed for Hyperpush autonomous remediation",
        "hyperpush-org/mesh-lang#6 — Mesh docs, packages, installers, and public surfaces must stay truthful for Hyperpush launch",
        "hyperpush-org/mesh-lang#15 — Mesh built-in runtime observability canvas for nodes, actors, and message flow",
    ]
    product_lines = [
        f"- [ ] hyperpush-org/hyperpush#{row['number']} — {row['title']}" for row in product_epics
    ]
    sections = [
        (
            "Scope",
            "This umbrella tracks the truthful product-repo launch work that still belongs in `hyperpush-org/hyperpush`. Cross-repo Mesh work is linked as an external dependency instead of being treated as product-repo scope.",
        ),
        (
            "Current state",
            "The original umbrella mixed product work, mesh-lang work, and stale repo-ownership assumptions. The reconciliation plan keeps the umbrella open, but narrows it to product-repo execution and explicit dependency links.",
        ),
        ("Product epics", "\n".join(product_lines)),
        ("External dependencies", "\n".join(f"- {line}" for line in mesh_dependencies)),
        (
            "Acceptance criteria",
            "- The umbrella only tracks work that still belongs in `hyperpush-org/hyperpush`.\n"
            "- Mesh-language/runtime/docs dependencies stay linked, but they are not flattened into product-repo scope.\n"
            "- Public repo wording stays truthful even though the local workspace still carries the historical `hyperpush-mono` path alias.",
        ),
    ]
    return render_markdown(sections)


def sentenceize(value: str) -> str:
    cleaned = re.sub(r"\s+", " ", value).strip()
    if cleaned == "":
        return cleaned
    cleaned = cleaned[0].upper() + cleaned[1:]
    if cleaned[-1] not in ".!?":
        cleaned += "."
    return cleaned


def build_topology_normalization_body(*, row: dict[str, Any], current_issue: dict[str, Any]) -> str:
    handle = require_string(row.get("canonical_issue_handle"), "row.canonical_issue_handle")
    _, sections = parse_markdown_sections(require_string(current_issue.get("body"), f"{handle}.body"))
    by_heading = section_map(sections)
    parent_heading = "Parent issue" if "Parent issue" in by_heading else "Parent epic" if "Parent epic" in by_heading else None
    rendered_sections: list[tuple[str, str]] = []
    if parent_heading is not None:
        rendered_sections.append((parent_heading, by_heading[parent_heading]))

    outcome_by_handle = {
        "hyperpush#54": "Define the concrete runtime boundary between the marketing site and the real operator app.",
        "hyperpush#55": "Give the operator app a real production container path instead of relying on local-only dev assumptions.",
        "hyperpush#56": "Create the reference generic-VM deployment stack and a health verification flow for the marketing site, operator app, and product backend together.",
    }
    rendered_sections.append(("Outcome", outcome_by_handle[handle]))
    rendered_sections.append(
        (
            "Current state",
            " ".join(
                [
                    sentenceize(require_string(row.get("delivery_truth"), f"{handle}.delivery_truth")),
                    "Public issue wording should refer to `hyperpush-org/hyperpush`; local `hyperpush-mono` compatibility paths remain supporting workspace context only.",
                ]
            ),
        )
    )

    acceptance_bullets = extract_markdown_bullets(by_heading.get("Acceptance criteria", ""))
    acceptance_bullets.append(
        "The wording stays aligned with the current repo/code truth instead of relying on stale cross-repo or local-path assumptions."
    )
    rendered_sections.append(("Acceptance criteria", "\n".join(f"- {bullet}" for bullet in acceptance_bullets)))
    rendered_sections.append(("Tracker context", "- Public repo: `hyperpush-org/hyperpush`\n- Workspace compatibility path: `mesher -> ../hyperpush-mono/mesher`"))
    return render_markdown(rendered_sections)


def build_open_rewrite_body(*, row: dict[str, Any], current_issue: dict[str, Any], ledger_rows: list[dict[str, Any]]) -> str:
    handle = require_string(row.get("canonical_issue_handle"), "row.canonical_issue_handle")
    if handle == "hyperpush#24":
        return build_special_issue_24_body(ledger_rows)
    if handle in {"hyperpush#54", "hyperpush#55", "hyperpush#56"}:
        return build_topology_normalization_body(row=row, current_issue=current_issue)

    _, sections = parse_markdown_sections(require_string(current_issue.get("body"), f"{handle}.body"))
    by_heading = section_map(sections)
    labels = [normalize_public_wording(label) for label in row.get("labels", [])]
    parent_heading = "Parent epic" if "Parent epic" in by_heading else "Parent issue" if "Parent issue" in by_heading else None

    rendered_sections: list[tuple[str, str]] = []
    if parent_heading is not None:
        rendered_sections.append((parent_heading, by_heading[parent_heading]))

    current_state_bits = [sentenceize(require_string(row.get("delivery_truth"), f"{handle}.delivery_truth"))]
    if "naming-drift" in require_array(row.get("secondary_audit_buckets"), f"{handle}.secondary_audit_buckets"):
        current_state_bits.append(
            f"Public repo truth should read `{require_string(row.get('public_repo_truth'), f'{handle}.public_repo_truth')}` even though local workspace evidence still references `{require_string(row.get('workspace_path_truth'), f'{handle}.workspace_path_truth')}`."
        )
    if require_array(row.get("matched_evidence_ids"), f"{handle}.matched_evidence_ids") == ["frontend_exp_operator_surfaces_partial"]:
        current_state_bits.append(
            "This issue remains active follow-through because the current operator surfaces are still partially mock-backed rather than fully shipped."
        )

    outcome = by_heading.get("Outcome")
    if outcome is None:
        outcome = require_string(row.get("proposed_repo_action"), f"{handle}.proposed_repo_action")
    rendered_sections.append(("Outcome", normalize_public_wording(outcome)))
    rendered_sections.append(("Current state", " ".join(current_state_bits)))

    if "Scope" in by_heading:
        rendered_sections.append(("Scope", normalize_public_wording(by_heading["Scope"])))
    if "Critical path" in by_heading:
        rendered_sections.append(("Critical path", normalize_public_wording(by_heading["Critical path"])))

    acceptance_bullets = extract_markdown_bullets(by_heading.get("Acceptance criteria", ""))
    if acceptance_bullets:
        normalized_bullets = [normalize_public_wording(bullet) for bullet in acceptance_bullets]
    else:
        normalized_bullets = [
            "The issue scope is concrete enough to execute from the product repo without guessing.",
            "The row stays open until shipped proof exists.",
        ]

    normalized_bullets.append(
        "The wording stays aligned with the current repo/code truth instead of relying on stale cross-repo or local-path assumptions."
    )
    if require_array(row.get("matched_evidence_ids"), f"{handle}.matched_evidence_ids") == ["frontend_exp_operator_surfaces_partial"]:
        normalized_bullets.append(
            "The issue does not treat the current mock-backed operator surfaces as already shipped."
        )
    rendered_sections.append(("Acceptance criteria", "\n".join(f"- {bullet}" for bullet in normalized_bullets)))

    for heading in ("Detailed acceptance criteria", "Initial sub-issues", "Notes"):
        if heading in by_heading and handle != "hyperpush#24":
            rendered_sections.append((heading, normalize_public_wording(by_heading[heading])))

    if labels:
        rendered_sections.append(("Tracker context", "\n".join([f"- Labels: {', '.join(labels)}", f"- Project-backed: {'yes' if row.get('project_backed') else 'no'}"])))

    return render_markdown(rendered_sections)


def build_transfer_body(current_issue: dict[str, Any]) -> str:
    body = require_string(current_issue.get("body"), "transfer_issue.body")
    return body if body.endswith("\n") else body + "\n"


def build_transfer_title(current_title: str) -> str:
    return "[Bug]: docs Packages nav link points to /packages instead of opening packages.meshlang.dev in a new tab"


def build_plan_operation(
    *,
    row: dict[str, Any],
    current_issue: dict[str, Any],
    op_kind: str,
    templates: dict[str, dict[str, Any]],
    ledger_rows: list[dict[str, Any]],
) -> dict[str, Any]:
    handle = require_string(row.get("canonical_issue_handle"), "row.canonical_issue_handle")
    current_title = require_string(current_issue.get("title"), f"{handle}.title")
    current_body = require_string(current_issue.get("body"), f"{handle}.body")
    repo_slug = require_string(current_issue.get("repo"), f"{handle}.repo")
    number = require_int(current_issue.get("number"), f"{handle}.number")
    current_url = require_string(current_issue.get("canonical_issue_url"), f"{handle}.canonical_issue_url")
    preamble, sections = parse_markdown_sections(current_body)
    current_headings = [section["heading"] for section in sections]

    planned_title = current_title
    planned_body = current_body if current_body.endswith("\n") else current_body + "\n"
    comment = None
    identity_changes = False
    target_repo_slug = repo_slug
    target_handle = handle
    target_url = current_url
    body_renderer = "preserve-current-body"

    if op_kind == "close":
        body_renderer = "preserve-current-body"
    elif op_kind == "rewrite":
        planned_title = normalize_open_rewrite_title(row, current_title)
        planned_body = build_open_rewrite_body(row=row, current_issue=current_issue, ledger_rows=ledger_rows)
        body_renderer = "existing-heading-rewrite"
    elif op_kind == "transfer":
        planned_title = build_transfer_title(current_title)
        planned_body = build_transfer_body(current_issue)
        comment = build_transfer_comment()
        target_repo_slug = MESH_LANG_REPO
        target_handle = "mesh-lang#TBD"
        target_url = None
        identity_changes = True
        body_renderer = "preserve-current-body"
    else:
        raise PlanError(f"unexpected operation kind {op_kind!r} for {handle}")

    return {
        "operation_id": f"{op_kind}-{slugify(handle)}",
        "apply": True,
        "operation_kind": op_kind,
        "source_bucket": require_string(row.get("primary_audit_bucket"), f"{handle}.primary_audit_bucket"),
        "repo_action_kind": require_string(row.get("proposed_repo_action_kind"), f"{handle}.proposed_repo_action_kind"),
        "canonical_issue_handle": handle,
        "canonical_issue_url": current_url,
        "issue_number": number,
        "project_backed": bool(row.get("project_backed")),
        "project_item_id": row.get("project_item_id"),
        "labels": row.get("labels", []),
        "matched_evidence_ids": row.get("matched_evidence_ids", []),
        "secondary_audit_buckets": row.get("secondary_audit_buckets", []),
        "ownership_truth": require_string(row.get("ownership_truth"), f"{handle}.ownership_truth"),
        "delivery_truth": require_string(row.get("delivery_truth"), f"{handle}.delivery_truth"),
        "workspace_path_truth": require_string(row.get("workspace_path_truth"), f"{handle}.workspace_path_truth"),
        "public_repo_truth": require_string(row.get("public_repo_truth"), f"{handle}.public_repo_truth"),
        "evidence_refs": row.get("evidence_refs", []),
        "title": {
            "before": current_title,
            "after": planned_title,
            "changed": planned_title != current_title,
        },
        "body": {
            "before": current_body,
            "after": planned_body,
            "changed": planned_body != (current_body if current_body.endswith("\n") else current_body + "\n"),
            "before_preamble": preamble,
            "before_headings": current_headings,
            "renderer": body_renderer,
        },
        "comment": {
            "required": False,
            "body": comment,
        },
        "identity": {
            "before": {
                "repo_slug": repo_slug,
                "issue_handle": handle,
                "issue_url": current_url,
            },
            "after": {
                "repo_slug": target_repo_slug,
                "issue_handle": target_handle,
                "issue_url": target_url,
            },
            "changes_identity": identity_changes,
        },
        "template_context": templates[target_repo_slug],
    }


def build_create_operation(gap: dict[str, Any], templates: dict[str, dict[str, Any]]) -> dict[str, Any]:
    template = templates[HYPERPUSH_REPO]
    title = build_pitch_issue_title(template)
    body = build_pitch_issue_body(template)
    return {
        "operation_id": "create-pitch-retrospective-issue",
        "apply": True,
        "operation_kind": "create",
        "source_bucket": require_string(gap.get("bucket"), "gap.bucket"),
        "repo_action_kind": require_string(gap.get("proposed_repo_action_kind"), "gap.proposed_repo_action_kind"),
        "canonical_issue_handle": None,
        "canonical_issue_url": None,
        "issue_number": None,
        "project_backed": False,
        "project_item_id": None,
        "labels": ["enhancement"],
        "matched_evidence_ids": ["pitch_route_missing_tracker_coverage"],
        "secondary_audit_buckets": [],
        "ownership_truth": require_string(gap.get("ownership_truth"), "gap.ownership_truth"),
        "delivery_truth": require_string(gap.get("delivery_truth"), "gap.delivery_truth"),
        "workspace_path_truth": require_string(gap.get("workspace_path_truth"), "gap.workspace_path_truth"),
        "public_repo_truth": require_string(gap.get("public_repo_truth"), "gap.public_repo_truth"),
        "evidence_refs": gap.get("evidence_refs", []),
        "title": {
            "before": None,
            "after": title,
            "changed": True,
        },
        "body": {
            "before": None,
            "after": body,
            "changed": True,
            "before_preamble": "",
            "before_headings": [],
            "renderer": template.get("renderer"),
        },
        "comment": {
            "required": True,
            "body": build_create_comment(),
        },
        "identity": {
            "before": None,
            "after": {
                "repo_slug": HYPERPUSH_REPO,
                "issue_handle": "hyperpush#TBD",
                "issue_url": None,
            },
            "changes_identity": True,
        },
        "template_context": template,
        "gap_id": require_string(gap.get("gap_id"), "gap.gap_id"),
        "surface": require_string(gap.get("surface"), "gap.surface"),
    }


def build_skipped_row(row: dict[str, Any]) -> dict[str, Any]:
    handle = require_string(row.get("canonical_issue_handle"), "row.canonical_issue_handle")
    reason = ""
    if handle in EXPECTED_SKIPPED_CLOSE_HANDLES:
        reason = "already_closed_shipped_row"
    elif require_string(row.get("proposed_repo_action_kind"), f"{handle}.proposed_repo_action_kind") == "keep_open":
        reason = "repo_issue_already_truthful_for_s02"
    else:
        reason = "not_in_live_repo_mutation_set"
    return {
        "canonical_issue_handle": handle,
        "canonical_issue_url": require_string(row.get("canonical_issue_url"), f"{handle}.canonical_issue_url"),
        "repo_action_kind": require_string(row.get("proposed_repo_action_kind"), f"{handle}.proposed_repo_action_kind"),
        "primary_audit_bucket": require_string(row.get("primary_audit_bucket"), f"{handle}.primary_audit_bucket"),
        "reason": reason,
        "state": require_string(row.get("state"), f"{handle}.state"),
    }


def assign_close_comments(operations: list[dict[str, Any]], issues_by_url: dict[str, dict[str, Any]]) -> None:
    close_handles = {
        require_string(operation.get("canonical_issue_handle"), "operation.canonical_issue_handle")
        for operation in operations
        if operation.get("operation_kind") == "close"
    }
    for operation in operations:
        if operation.get("operation_kind") != "close":
            continue
        current_body = require_string(require_object(operation.get("body"), "operation.body").get("before"), "operation.body.before")
        linked_urls = sorted(set(re.findall(r"https://github\.com/[^\s)]+/issues/\d+", current_body)))
        residual_children: list[str] = []
        for issue_url in linked_urls:
            linked_issue = issues_by_url.get(issue_url)
            if linked_issue is None:
                continue
            linked_handle = repo_issue_handle(
                require_string(linked_issue.get("repo"), "linked_issue.repo"),
                require_int(linked_issue.get("number"), "linked_issue.number"),
            )
            if linked_handle == operation.get("canonical_issue_handle"):
                continue
            linked_state = require_string(linked_issue.get("state"), f"{linked_handle}.state")
            if linked_state == "OPEN" and linked_handle not in close_handles:
                residual_children.append(linked_handle)
        residual_children.sort()
        operation["comment"] = {
            "required": True,
            "body": build_close_comment(row=operation, residual_children=residual_children),
        }


def validate_plan(plan: dict[str, Any]) -> dict[str, Any]:
    operations = [require_object(operation, "plan.operation") for operation in require_array(plan.get("operations"), "plan.operations")]
    skipped = [require_object(row, "plan.skipped_row") for row in require_array(plan.get("skipped_rows"), "plan.skipped_rows")]

    counts: dict[str, int] = {"close": 0, "rewrite": 0, "transfer": 0, "create": 0}
    seen_handles: set[str] = set()
    seen_ids: set[str] = set()
    transfer_verified = False
    create_verified = False
    for operation in operations:
        operation_id = require_string(operation.get("operation_id"), "operation.operation_id")
        if operation_id in seen_ids:
            raise PlanError(f"duplicate operation_id in plan: {operation_id}")
        seen_ids.add(operation_id)

        operation_kind = require_string(operation.get("operation_kind"), f"{operation_id}.operation_kind")
        if operation_kind not in counts:
            raise PlanError(f"unknown operation kind in plan: {operation_kind}")
        counts[operation_kind] += 1

        handle = operation.get("canonical_issue_handle")
        if handle is not None:
            canonical_handle = require_string(handle, f"{operation_id}.canonical_issue_handle")
            if canonical_handle in seen_handles:
                raise PlanError(f"duplicate planned issue handle in plan: {canonical_handle}")
            seen_handles.add(canonical_handle)

        title = require_object(operation.get("title"), f"{operation_id}.title")
        require_string(title.get("after"), f"{operation_id}.title.after")
        body = require_object(operation.get("body"), f"{operation_id}.body")
        require_string(body.get("after"), f"{operation_id}.body.after")
        identity = require_object(operation.get("identity"), f"{operation_id}.identity")
        if operation_kind in {"close", "create"}:
            comment = require_object(operation.get("comment"), f"{operation_id}.comment")
            if not comment.get("required"):
                raise PlanError(f"{operation_id} must require a comment")
            require_string(comment.get("body"), f"{operation_id}.comment.body")

        if operation_kind == "transfer":
            after = require_object(identity.get("after"), f"{operation_id}.identity.after")
            if require_string(after.get("repo_slug"), f"{operation_id}.identity.after.repo_slug") != MESH_LANG_REPO:
                raise PlanError("hyperpush#8 transfer must target mesh-lang")
            if require_string(operation.get("canonical_issue_handle"), f"{operation_id}.canonical_issue_handle") != "hyperpush#8":
                raise PlanError("transfer operation must stay bound to hyperpush#8")
            transfer_verified = True

        if operation_kind == "create":
            after = require_object(identity.get("after"), f"{operation_id}.identity.after")
            if require_string(after.get("repo_slug"), f"{operation_id}.identity.after.repo_slug") != HYPERPUSH_REPO:
                raise PlanError("/pitch retrospective issue must target hyperpush")
            if require_string(operation.get("surface"), f"{operation_id}.surface") != "/pitch":
                raise PlanError("create operation must expand /pitch")
            create_verified = True

    skipped_handles = {
        require_string(row.get("canonical_issue_handle"), "skipped_row.canonical_issue_handle")
        for row in skipped
    }
    if EXPECTED_SKIPPED_CLOSE_HANDLES - skipped_handles:
        missing = ", ".join(sorted(EXPECTED_SKIPPED_CLOSE_HANDLES - skipped_handles))
        raise PlanError(f"skipped rows missing already-closed hyperpush rows: {missing}")

    if counts != {k: EXPECTED_PLAN_COUNTS[k] for k in ("close", "rewrite", "transfer", "create")}:
        raise PlanError(f"plan operation counts drifted: {counts}")
    expected_skipped_total = EXPECTED_PLAN_COUNTS["skipped"] + 23
    if len(skipped) != expected_skipped_total:
        raise PlanError(
            f"skipped row count drifted: expected {expected_skipped_total} total skipped rows (23 untouched keep-open + 3 already-closed closeouts), found {len(skipped)}"
        )
    if len(operations) != EXPECTED_PLAN_COUNTS["total_apply"]:
        raise PlanError(f"plan apply row count drifted: expected 43, found {len(operations)}")
    if not transfer_verified:
        raise PlanError("plan missing hyperpush#8 transfer operation")
    if not create_verified:
        raise PlanError("plan missing /pitch create operation")

    return {
        "operation_counts": counts,
        "skipped_rows": len(skipped),
        "transfer_verified": transfer_verified,
        "create_verified": create_verified,
    }


def render_operation_row(operation: dict[str, Any]) -> str:
    before = require_object(require_object(operation.get("identity"), "operation.identity").get("before"), "operation.identity.before") if operation.get("identity") and require_object(operation.get("identity"), "operation.identity").get("before") else None
    after = require_object(require_object(operation.get("identity"), "operation.identity").get("after"), "operation.identity.after")
    before_handle = before.get("issue_handle") if before else "_new_"
    after_handle = after.get("issue_handle") or "_tbd_"
    comment = require_object(operation.get("comment"), "operation.comment")
    return "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |".format(
        require_string(operation.get("operation_kind"), "operation.operation_kind"),
        before_handle,
        after_handle,
        "yes" if require_object(operation.get("title"), "operation.title").get("changed") else "no",
        "yes" if require_object(operation.get("body"), "operation.body").get("changed") else "no",
        "yes" if comment.get("required") else "no",
    )


def render_plan_markdown(plan: dict[str, Any]) -> str:
    operations = [require_object(operation, "plan.operation") for operation in require_array(plan.get("operations"), "plan.operations")]
    skipped_rows = [require_object(row, "plan.skipped_row") for row in require_array(plan.get("skipped_rows"), "plan.skipped_rows")]
    grouped: dict[str, list[dict[str, Any]]] = {"close": [], "rewrite": [], "transfer": [], "create": []}
    for operation in operations:
        grouped[require_string(operation.get("operation_kind"), "operation.operation_kind")].append(operation)

    lines = [
        "# M057 S02 Repo Mutation Plan",
        "",
        f"- Version: `{require_string(plan.get('version'), 'plan.version')}`",
        f"- Generated at: `{require_string(plan.get('generated_at'), 'plan.generated_at')}`",
        f"- Source ledger: `{require_string(require_object(plan.get('source'), 'plan.source').get('ledger_path'), 'plan.source.ledger_path')}`",
        f"- Apply operations: `{len(operations)}`",
        f"- Skipped rows: `{len(skipped_rows)}`",
        "",
        "## Rollup",
        "",
        "| Kind | Count |",
        "| --- | --- |",
        f"| `close` | `{len(grouped['close'])}` |",
        f"| `rewrite` | `{len(grouped['rewrite'])}` |",
        f"| `transfer` | `{len(grouped['transfer'])}` |",
        f"| `create` | `{len(grouped['create'])}` |",
        f"| `skipped` | `{len(skipped_rows)}` |",
        "",
        "## Template rendering",
        "",
        "| Repo | Renderer | Fallback | Headings |",
        "| --- | --- | --- | --- |",
    ]
    for repo_slug, template in require_object(plan.get("template_context"), "plan.template_context").items():
        template_object = require_object(template, f"template_context[{repo_slug}]")
        lines.append(
            "| `{}` | `{}` | `{}` | `{}` |".format(
                repo_slug,
                require_string(template_object.get("renderer"), f"template_context[{repo_slug}].renderer"),
                "yes" if template_object.get("fallback_used") else "no",
                ", ".join(require_array(template_object.get("headings"), f"template_context[{repo_slug}].headings")),
            )
        )

    for heading in ("close", "rewrite", "transfer", "create"):
        lines.extend(["", f"## {heading}", "", "| Kind | Before | After | Title change | Body change | Comment |", "| --- | --- | --- | --- | --- | --- |"])
        for operation in grouped[heading]:
            lines.append(render_operation_row(operation))

    lines.extend(["", "## skipped", "", "| Issue | Repo action | Reason |", "| --- | --- | --- |"])
    for row in skipped_rows:
        lines.append(
            "| `{}` | `{}` | `{}` |".format(
                require_string(row.get("canonical_issue_handle"), "skipped_row.canonical_issue_handle"),
                require_string(row.get("repo_action_kind"), "skipped_row.repo_action_kind"),
                require_string(row.get("reason"), "skipped_row.reason"),
            )
        )

    lines.extend(
        [
            "",
            "## Explicit identity-changing operations",
            "",
            "- `hyperpush#8` transfers into `hyperpush-org/mesh-lang` and therefore changes canonical issue identity during apply.",
            "- `/pitch` expands into one dedicated retrospective `hyperpush` issue that is created-and-closed from the plan.",
        ]
    )
    return "\n".join(lines) + "\n"


def build_plan(*, source_root: Path, source_dir: Path, output_dir: Path) -> tuple[dict[str, Any], str, dict[str, Any]]:
    ledger_path = source_dir / LEDGER_JSON_FILENAME
    audit_path = source_dir / AUDIT_MD_FILENAME
    ledger = read_json(ledger_path, "source ledger")
    audit_markdown = read_text(audit_path, "source audit")

    snapshots = load_snapshots(source_dir)
    validate_snapshots(snapshots)
    validate_ledger_bundle(ledger, audit_markdown, snapshots)
    ensure_source_counts(ledger)

    issues_by_handle, issues_by_url = build_issue_indexes(snapshots)

    templates = {
        MESH_LANG_REPO: parse_template(source_root / ".github" / "ISSUE_TEMPLATE" / "feature_request.yml", repo_slug=MESH_LANG_REPO),
        HYPERPUSH_REPO: parse_template(source_root.parent / "hyperpush-mono" / ".github" / "ISSUE_TEMPLATE" / "feature_request.yml", repo_slug=HYPERPUSH_REPO),
    }

    rows = [require_object(row, "ledger.row") for row in require_array(ledger.get("rows"), "ledger.rows")]
    rows_by_handle = {require_string(row.get("canonical_issue_handle"), "row.canonical_issue_handle"): row for row in rows}
    operations: list[dict[str, Any]] = []
    skipped_rows: list[dict[str, Any]] = []

    for row in rows:
        handle = require_string(row.get("canonical_issue_handle"), "row.canonical_issue_handle")
        current_issue = issues_by_handle.get(handle)
        if current_issue is None:
            raise PlanError(f"source snapshots missing current issue row for {handle}")
        action_kind = require_string(row.get("proposed_repo_action_kind"), f"{handle}.proposed_repo_action_kind")
        state = require_string(row.get("state"), f"{handle}.state")
        project_action_kind = require_string(row.get("proposed_project_action_kind"), f"{handle}.proposed_project_action_kind")

        if action_kind == "close_as_shipped":
            if state == "OPEN":
                operations.append(
                    build_plan_operation(
                        row=row,
                        current_issue=current_issue,
                        op_kind="close",
                        templates=templates,
                        ledger_rows=rows,
                    )
                )
            elif state == "CLOSED":
                skipped_rows.append(build_skipped_row(row))
            else:
                raise PlanError(f"unexpected state for close_as_shipped row {handle}: {state}")
            continue

        if action_kind == "rewrite_scope":
            operations.append(
                build_plan_operation(
                    row=row,
                    current_issue=current_issue,
                    op_kind="rewrite",
                    templates=templates,
                    ledger_rows=rows,
                )
            )
            continue

        if action_kind == "keep_open" and project_action_kind == "update_project_item":
            operations.append(
                build_plan_operation(
                    row=row,
                    current_issue=current_issue,
                    op_kind="rewrite",
                    templates=templates,
                    ledger_rows=rows,
                )
            )
            continue

        if action_kind == "move_to_mesh_lang":
            operations.append(
                build_plan_operation(
                    row=row,
                    current_issue=current_issue,
                    op_kind="transfer",
                    templates=templates,
                    ledger_rows=rows,
                )
            )
            continue

        skipped_rows.append(build_skipped_row(row))

    assign_close_comments(operations, issues_by_url)

    pitch_gap_candidates = [
        require_object(gap, "ledger.derived_gap")
        for gap in require_array(ledger.get("derived_gaps"), "ledger.derived_gaps")
        if require_string(require_object(gap, "ledger.derived_gap").get("surface"), "gap.surface") == "/pitch"
    ]
    if len(pitch_gap_candidates) != 1:
        raise PlanError(f"expected exactly one /pitch derived gap, found {len(pitch_gap_candidates)}")
    operations.append(build_create_operation(pitch_gap_candidates[0], templates))

    close_handles_in_plan = {
        require_string(operation.get("canonical_issue_handle"), "operation.canonical_issue_handle")
        for operation in operations
        if operation.get("operation_kind") == "close"
    }
    closed_handles_in_plan = {
        require_string(operation.get("canonical_issue_handle"), "operation.canonical_issue_handle")
        for operation in operations
        if operation.get("canonical_issue_handle") is not None
    }
    for handle in EXPECTED_SKIPPED_CLOSE_HANDLES:
        if handle in closed_handles_in_plan:
            raise PlanError(f"attempted to plan live mutation for already-closed row {handle}")

    rollup = {
        "close": sum(1 for operation in operations if operation.get("operation_kind") == "close"),
        "rewrite": sum(1 for operation in operations if operation.get("operation_kind") == "rewrite"),
        "transfer": sum(1 for operation in operations if operation.get("operation_kind") == "transfer"),
        "create": sum(1 for operation in operations if operation.get("operation_kind") == "create"),
        "skipped": len(skipped_rows),
        "total_apply": len(operations),
        "close_handles": sorted(close_handles_in_plan),
        "rewrite_handles": sorted(
            require_string(operation.get("canonical_issue_handle"), "operation.canonical_issue_handle")
            for operation in operations
            if operation.get("operation_kind") == "rewrite"
        ),
    }

    plan = {
        "version": PLAN_VERSION,
        "generated_at": iso_now(),
        "source_script": SCRIPT_RELATIVE_PATH,
        "source": {
            "ledger_path": str(ledger_path.relative_to(source_root)),
            "audit_path": str(audit_path.relative_to(source_root)),
            "snapshot_dir": str(source_dir.relative_to(source_root)),
            "output_dir": str(output_dir.relative_to(source_root)),
            "ledger_version": require_string(ledger.get("version"), "ledger.version"),
        },
        "template_context": templates,
        "rollup": rollup,
        "operations": operations,
        "skipped_rows": sorted(skipped_rows, key=lambda row: require_string(row.get("canonical_issue_handle"), "skipped_row.canonical_issue_handle")),
        "invariants": {
            "plan_only_input": True,
            "later_apply_must_use_manifest": True,
            "explicit_transfer_handle": "hyperpush#8",
            "explicit_create_surface": "/pitch",
        },
    }

    markdown = render_plan_markdown(plan)
    check_summary = validate_plan(plan)
    return plan, markdown, check_summary


def write_outputs(*, output_dir: Path, plan: dict[str, Any], markdown: str) -> list[Path]:
    output_dir.mkdir(parents=True, exist_ok=True)
    plan_path = output_dir / PLAN_JSON_FILENAME
    markdown_path = output_dir / PLAN_MD_FILENAME
    write_json_atomic(plan_path, plan)
    markdown_path.write_text(markdown, encoding="utf8")
    return [plan_path, markdown_path]


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Build the M057 S02 dry-run repo mutation plan.")
    parser.add_argument("--source-root", type=Path, default=ROOT, help="Alternate source root for isolated contract tests.")
    parser.add_argument("--source-dir", type=Path, help="Directory containing the immutable S01 ledger and snapshots.")
    parser.add_argument("--output-dir", type=Path, help="Directory receiving the dry-run plan artifacts.")
    parser.add_argument("--check", action="store_true", help="Validate the generated plan contract after writing outputs.")
    args = parser.parse_args(argv)
    args.source_root = args.source_root.resolve()
    args.source_dir = (args.source_root / DEFAULT_SOURCE_DIR.relative_to(ROOT)).resolve() if args.source_dir is None else args.source_dir.resolve()
    args.output_dir = (args.source_root / DEFAULT_OUTPUT_DIR.relative_to(ROOT)).resolve() if args.output_dir is None else args.output_dir.resolve()
    return args


def main(argv: list[str]) -> int:
    args = parse_args(argv)
    plan, markdown, check_summary = build_plan(
        source_root=args.source_root,
        source_dir=args.source_dir,
        output_dir=args.output_dir,
    )
    written_paths = write_outputs(output_dir=args.output_dir, plan=plan, markdown=markdown)
    print(
        json.dumps(
            {
                "status": "ok",
                "output_dir": str(args.output_dir),
                "written_files": [str(path) for path in written_paths],
                "rollup": require_object(plan.get("rollup"), "plan.rollup"),
                "check": check_summary if args.check else None,
            },
            indent=2,
        )
    )
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main(sys.argv[1:]))
    except (PlanError, LedgerError, InventoryError) as exc:
        print(str(exc), file=sys.stderr)
        raise SystemExit(1)
