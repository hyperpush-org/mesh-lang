#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import os
import re
import sys
from pathlib import Path
from typing import Any

from m057_tracker_inventory import (
    HYPERPUSH_ALIAS_REPO,
    HYPERPUSH_REPO,
    MESH_LANG_REPO,
    InventoryError,
    iso_now,
    load_snapshots,
    require_array,
    require_object,
    require_string,
    validate_snapshots,
    write_json_atomic,
)

ROOT = Path(__file__).resolve().parents[2]
DEFAULT_OUTPUT_SUBDIR = Path(".gsd") / "milestones" / "M057" / "slices" / "S01"
SCRIPT_RELATIVE_PATH = "scripts/lib/m057_evidence_index.py"
EVIDENCE_JSON_FILENAME = "reconciliation-evidence.json"
EVIDENCE_MD_FILENAME = "reconciliation-evidence.md"
NAMING_MAP_FILENAME = "naming-ownership-map.json"
EVIDENCE_VERSION = "m057-s01-reconciliation-evidence-v1"
NAMING_MAP_VERSION = "m057-s01-naming-ownership-map-v1"


class EvidenceError(RuntimeError):
    pass


class SourceCache:
    def __init__(self, source_root: Path):
        self.source_root = source_root
        self._text_cache: dict[str, str] = {}
        self._json_cache: dict[str, Any] = {}

    def resolve(self, relative_path: str) -> Path:
        path = self.source_root / relative_path
        return path

    def read_text(self, relative_path: str) -> str:
        if relative_path in self._text_cache:
            return self._text_cache[relative_path]
        path = self.resolve(relative_path)
        if not path.is_file():
            raise EvidenceError(f"missing required source file: {relative_path}")
        text = path.read_text(encoding="utf8")
        self._text_cache[relative_path] = text
        return text

    def read_json(self, relative_path: str) -> Any:
        if relative_path in self._json_cache:
            return self._json_cache[relative_path]
        text = self.read_text(relative_path)
        try:
            payload = json.loads(text)
        except json.JSONDecodeError as exc:
            raise EvidenceError(f"{relative_path} is not valid JSON: {exc}") from exc
        self._json_cache[relative_path] = payload
        return payload


def issue_handle(issue: dict[str, Any]) -> str:
    repo_slug = require_string(issue.get("repo"), "issue.repo")
    return f"{repo_slug.split('/')[-1]}#{require_string(str(issue.get('number')), 'issue.number')}"


def issue_handle_from_parts(repo: str, number: int) -> str:
    return f"{repo.split('/')[-1]}#{number}"


# require_string from tracker inventory only accepts strings, so keep a local integer helper.
def require_int(value: Any, label: str) -> int:
    if not isinstance(value, int):
        raise EvidenceError(f"{label} must be an integer")
    return value


def load_issue_index(snapshot: dict[str, Any], label: str) -> dict[int, dict[str, Any]]:
    issues = require_array(snapshot.get("issues"), f"{label}.issues")
    index: dict[int, dict[str, Any]] = {}
    for idx, issue in enumerate(issues):
        issue_object = require_object(issue, f"{label}.issues[{idx}]")
        number = require_int(issue_object.get("number"), f"{label}.issues[{idx}].number")
        if number in index:
            raise EvidenceError(f"{label} contains duplicate issue number {number}")
        index[number] = issue_object
    return index


def require_issue(issue_index: dict[int, dict[str, Any]], *, repo: str, number: int) -> dict[str, Any]:
    issue = issue_index.get(number)
    if issue is None:
        raise EvidenceError(f"missing required issue {issue_handle_from_parts(repo, number)} in snapshot")
    issue_repo = require_string(issue.get("repo"), f"{repo}#{number}.repo")
    if issue_repo != repo:
        raise EvidenceError(
            f"snapshot repo drift for {issue_handle_from_parts(repo, number)}: expected {repo!r}, found {issue_repo!r}"
        )
    return issue


def find_line(relative_path: str, text: str, marker: str) -> int:
    for index, line in enumerate(text.splitlines(), start=1):
        if marker in line:
            return index
    raise EvidenceError(f"{relative_path} missing required marker {marker!r}")


def build_line_ref(cache: SourceCache, relative_path: str, marker: str, note: str) -> dict[str, Any]:
    text = cache.read_text(relative_path)
    line = find_line(relative_path, text, marker)
    return {
        "path": relative_path,
        "line": line,
        "marker": marker,
        "note": note,
    }


# For snapshot-backed refs, validate by structure and keep a stable pointer instead of brittle line numbers.
def build_snapshot_ref(relative_path: str, *, json_pointer: str, note: str) -> dict[str, Any]:
    return {
        "path": relative_path,
        "json_pointer": json_pointer,
        "note": note,
    }


def build_decision_refs(cache: SourceCache) -> list[dict[str, Any]]:
    return [
        build_line_ref(
            cache,
            ".gsd/DECISIONS.md",
            "| D454 | M057/S01 planning | data-shape |",
            "D454 defines the three-layer artifact split for raw snapshots, derived evidence, and the final ledger.",
        ),
        build_line_ref(
            cache,
            ".gsd/DECISIONS.md",
            "| D455 | M057/S01/T01 | tracker-reconciliation |",
            "D455 defines the canonical issue URL inventory shape that this evidence index consumes.",
        ),
        build_line_ref(
            cache,
            ".gsd/DECISIONS.md",
            "| D456 | M057/S01/T02 | tracker-reconciliation |",
            "D456 fixes the downstream naming contract by keeping workspace-path truth separate from public repo truth and canonical tracker destination.",
        ),
    ]


def build_issue_ref(issue: dict[str, Any], *, note: str) -> dict[str, Any]:
    return {
        "path": issue.get("canonical_issue_url"),
        "note": note,
    }


def ensure_issue_body_contains(issue: dict[str, Any], needle: str, label: str) -> None:
    body = require_string(issue.get("body"), f"{label}.body")
    if needle not in body:
        raise EvidenceError(f"{label} missing required body marker {needle!r}")


def render_destination(destination: dict[str, Any]) -> str:
    parts: list[str] = []
    repo_slug = destination.get("repo_slug")
    if isinstance(repo_slug, str):
        parts.append(repo_slug)
    issue_repo = destination.get("issue_repo")
    if isinstance(issue_repo, str):
        parts.append(f"tracker:{issue_repo}")
    workspace_root = destination.get("workspace_root")
    if isinstance(workspace_root, str):
        parts.append(f"workspace:{workspace_root}")
    compatibility_path = destination.get("compatibility_path")
    if isinstance(compatibility_path, str):
        parts.append(f"compat:{compatibility_path}")
    if not parts:
        return json.dumps(destination, sort_keys=True)
    return " | ".join(parts)


def build_naming_map(
    cache: SourceCache,
    *,
    source_root: Path,
    captured_at: str,
    canonical_redirect: dict[str, Any],
) -> dict[str, Any]:
    repo_identity = require_object(cache.read_json("scripts/lib/repo-identity.json"), "repo-identity")
    language_repo = require_object(repo_identity.get("languageRepo"), "repo-identity.languageRepo")
    product_repo = require_object(repo_identity.get("productRepo"), "repo-identity.productRepo")

    language_workspace_dir = require_string(language_repo.get("workspaceDir"), "repo-identity.languageRepo.workspaceDir")
    language_slug = require_string(language_repo.get("slug"), "repo-identity.languageRepo.slug")
    product_workspace_dir = require_string(product_repo.get("workspaceDir"), "repo-identity.productRepo.workspaceDir")
    product_slug = require_string(product_repo.get("slug"), "repo-identity.productRepo.slug")
    product_repo_url = require_string(product_repo.get("repoUrl"), "repo-identity.productRepo.repoUrl")
    product_issues_url = require_string(product_repo.get("issuesUrl"), "repo-identity.productRepo.issuesUrl")

    canonical_requested_repo = require_string(canonical_redirect.get("requested_repo"), "canonical_redirect.requested_repo")
    canonical_slug = require_string(canonical_redirect.get("canonical_slug"), "canonical_redirect.canonical_slug")
    canonical_url = require_string(canonical_redirect.get("canonical_url"), "canonical_redirect.canonical_url")

    if canonical_requested_repo != HYPERPUSH_ALIAS_REPO:
        raise EvidenceError(
            f"canonical redirect drift: expected requested repo {HYPERPUSH_ALIAS_REPO!r} but found {canonical_requested_repo!r}"
        )
    if canonical_slug != HYPERPUSH_REPO or canonical_url != f"https://github.com/{HYPERPUSH_REPO}":
        raise EvidenceError(
            f"canonical redirect drift: expected {HYPERPUSH_REPO!r} / https://github.com/{HYPERPUSH_REPO} but found {canonical_slug!r} / {canonical_url!r}"
        )

    allowed_product_slugs = {HYPERPUSH_ALIAS_REPO, HYPERPUSH_REPO}
    if product_slug not in allowed_product_slugs:
        raise EvidenceError(
            f"repo identity productRepo.slug must be one of {sorted(allowed_product_slugs)!r}, found {product_slug!r}"
        )
    allowed_product_repo_urls = {
        f"https://github.com/{HYPERPUSH_ALIAS_REPO}",
        f"https://github.com/{HYPERPUSH_REPO}",
    }
    if product_repo_url not in allowed_product_repo_urls:
        raise EvidenceError(
            f"repo identity productRepo.repoUrl must be one of {sorted(allowed_product_repo_urls)!r}, found {product_repo_url!r}"
        )
    allowed_product_issue_urls = {
        f"https://github.com/{HYPERPUSH_ALIAS_REPO}/issues",
        f"https://github.com/{HYPERPUSH_REPO}/issues",
    }
    if product_issues_url not in allowed_product_issue_urls:
        raise EvidenceError(
            f"repo identity productRepo.issuesUrl must be one of {sorted(allowed_product_issue_urls)!r}, found {product_issues_url!r}"
        )

    mesher_symlink = source_root / "mesher"
    if not mesher_symlink.exists():
        raise EvidenceError("missing compatibility path mesher/")
    if not mesher_symlink.is_symlink():
        raise EvidenceError("mesher/ must remain a compatibility symlink into the sibling product repo")
    mesher_target = os.readlink(mesher_symlink)
    if "hyperpush-mono" not in mesher_target:
        raise EvidenceError(f"mesher symlink drifted: expected hyperpush-mono target, found {mesher_target!r}")

    project_product_ref = build_line_ref(
        cache,
        ".gsd/PROJECT.md",
        "still surfaced locally through the `hyperpush-mono` workspace path",
        "Project contract says the public product repo is hyperpush while the local workspace still uses the hyperpush-mono path.",
    )
    project_compat_ref = build_line_ref(
        cache,
        ".gsd/PROJECT.md",
        "any local `mesh-lang/mesher` path is compatibility-only",
        "Project contract marks mesh-lang/mesher as a compatibility path, not the authoritative tracked tree.",
    )
    workspace_alias_ref = build_line_ref(
        cache,
        "scripts/workspace-git.sh",
        "if [[ \"$normalized_expected\" == 'https://github.com/hyperpush-org/hyperpush-mono' && \"$normalized_actual\" == 'https://github.com/hyperpush-org/hyperpush' ]]; then",
        "workspace-git explicitly accepts the hyperpush-mono -> hyperpush remote alias drift.",
    )
    workspace_name_ref = build_line_ref(
        cache,
        "scripts/workspace-git.sh",
        "hyperpush-mono|hyperpush) printf 'hyperpush-mono\\n' ;;",
        "workspace-git canonicalizes both hyperpush and hyperpush-mono CLI names to the local hyperpush-mono workspace root.",
    )
    repo_identity_ref = build_line_ref(
        cache,
        "scripts/lib/repo-identity.json",
        '"workspaceDir": "hyperpush-mono"',
        "repo identity still records the local product workspace alias as hyperpush-mono.",
    )

    surfaces = [
        {
            "surface_id": "mesh_lang_repo",
            "surface_label": "Language / docs / packages surfaces",
            "ownership_truth": "language-owned in mesh-lang",
            "delivery_truth": "authoritative tracked sources still live in this checkout",
            "workspace_path_truth": f"{language_workspace_dir}/... tracked directly under this repo root",
            "public_repo_truth": language_slug,
            "normalized_canonical_destination": {
                "repo_slug": language_slug,
                "issue_repo": "mesh-lang",
                "workspace_root": language_workspace_dir,
            },
            "evidence_refs": [
                build_line_ref(
                    cache,
                    ".gsd/PROJECT.md",
                    "mesh-lang` keeps the language/toolchain/docs/installers/registry/packages/public-site surfaces",
                    "Project contract names mesh-lang as the owner of language, docs, installer, registry, and public-site code.",
                )
            ],
        },
        {
            "surface_id": "hyperpush_product_repo",
            "surface_label": "Product repo surfaces exposed through mesher/",
            "ownership_truth": "product-owned in the Hyperpush product repo",
            "delivery_truth": "repo-boundary split shipped; tracker wording must normalize to the public product slug",
            "workspace_path_truth": (
                f"local compatibility path mesher -> {mesher_target}; workspace helper canonicalizes both hyperpush and "
                f"hyperpush-mono to workspace root {product_workspace_dir}"
            ),
            "public_repo_truth": canonical_slug,
            "normalized_canonical_destination": {
                "repo_slug": canonical_slug,
                "issue_repo": "hyperpush",
                "workspace_root": product_workspace_dir,
                "compatibility_path": f"mesher -> {mesher_target}",
            },
            "evidence_refs": [
                project_product_ref,
                project_compat_ref,
                workspace_alias_ref,
                workspace_name_ref,
                repo_identity_ref,
                build_snapshot_ref(
                    "hyperpush-issues.snapshot.json",
                    json_pointer="/canonical_redirect",
                    note="Live GitHub repo view canonicalized requested hyperpush-mono inventory capture to public repo hyperpush.",
                ),
            ],
        },
        {
            "surface_id": "product_pitch_route",
            "surface_label": "Evaluator /pitch route",
            "ownership_truth": "product-owned landing surface",
            "delivery_truth": "shipped in the sibling product repo during M056",
            "workspace_path_truth": "mesher/landing/app/pitch/page.tsx via compatibility symlink into the sibling product repo",
            "public_repo_truth": canonical_slug,
            "normalized_canonical_destination": {
                "repo_slug": canonical_slug,
                "issue_repo": "hyperpush",
                "workspace_root": product_workspace_dir,
                "surface_path": "mesher/landing/app/pitch/page.tsx",
            },
            "evidence_refs": [
                build_line_ref(
                    cache,
                    "mesher/landing/app/pitch/page.tsx",
                    "return <PitchDeck />",
                    "The route file exists and renders the shipped PitchDeck surface.",
                ),
                build_line_ref(
                    cache,
                    ".gsd/milestones/M056/M056-SUMMARY.md",
                    "M056 delivered `/pitch` as a real App Router route inside `mesher/landing/`",
                    "M056 milestone summary records /pitch as shipped in the product repo.",
                ),
            ],
        },
        {
            "surface_id": "mesh_lang_docs_packages_nav",
            "surface_label": "Website docs navbar / packages link",
            "ownership_truth": "language-owned docs surface",
            "delivery_truth": "active bug lives in mesh-lang docs, not the product repo",
            "workspace_path_truth": "website/docs/.vitepress/* tracked directly in mesh-lang",
            "public_repo_truth": language_slug,
            "normalized_canonical_destination": {
                "repo_slug": language_slug,
                "issue_repo": "mesh-lang",
                "workspace_root": language_workspace_dir,
                "surface_paths": [
                    "website/docs/.vitepress/config.mts",
                    "website/docs/.vitepress/theme/components/NavBar.vue",
                ],
            },
            "evidence_refs": [
                build_line_ref(
                    cache,
                    "website/docs/.vitepress/config.mts",
                    "{ text: 'Packages', link: '/packages/' }",
                    "VitePress nav config keeps the Packages link on the docs site.",
                ),
                build_line_ref(
                    cache,
                    "website/docs/.vitepress/theme/components/NavBar.vue",
                    "{ text: 'Packages', href: '/packages/' }",
                    "Custom NavBar repeats the same local Packages route.",
                ),
            ],
        },
        {
            "surface_id": "frontend_exp_operator_app",
            "surface_label": "frontend-exp operator UI",
            "ownership_truth": "product-owned operator app surface",
            "delivery_truth": "partial only; checked-in UI still consumes mock data instead of live operator services",
            "workspace_path_truth": "mesher/frontend-exp/lib/mock-data.ts via the compatibility symlink into the sibling product repo",
            "public_repo_truth": canonical_slug,
            "normalized_canonical_destination": {
                "repo_slug": canonical_slug,
                "issue_repo": "hyperpush",
                "workspace_root": product_workspace_dir,
                "surface_path": "mesher/frontend-exp/lib/mock-data.ts",
            },
            "evidence_refs": [
                build_line_ref(
                    cache,
                    "mesher/frontend-exp/lib/mock-data.ts",
                    "export const MOCK_ISSUES: Issue[] = [",
                    "frontend-exp still exports hard-coded mock issue rows.",
                ),
                build_line_ref(
                    cache,
                    "mesher/frontend-exp/lib/mock-data.ts",
                    "export const MOCK_STATS = {",
                    "frontend-exp still exports hard-coded dashboard statistics.",
                ),
            ],
        },
    ]

    drift_findings = [
        {
            "finding_id": "product_repo_public_slug_alias",
            "summary": "The local workspace still uses hyperpush-mono while live GitHub canonicalizes the public product repo to hyperpush.",
            "workspace_path_truth": f"product workspace root remains {product_workspace_dir} and mesher resolves through {mesher_target}",
            "public_repo_truth": canonical_slug,
            "normalized_canonical_destination": {
                "repo_slug": canonical_slug,
                "issue_repo": "hyperpush",
                "workspace_root": product_workspace_dir,
            },
            "evidence_refs": [project_product_ref, workspace_alias_ref, build_snapshot_ref(
                "hyperpush-issues.snapshot.json",
                json_pointer="/canonical_redirect",
                note="Inventory capture proved GitHub redirects hyperpush-mono to hyperpush at query time.",
            )],
        },
        {
            "finding_id": "compatibility_mesher_path_not_authoritative",
            "summary": "mesh-lang/mesher is only a compatibility symlink and must not be treated as a language-owned authoritative tree.",
            "workspace_path_truth": f"mesher -> {mesher_target}",
            "public_repo_truth": canonical_slug,
            "normalized_canonical_destination": {
                "repo_slug": canonical_slug,
                "issue_repo": "hyperpush",
                "workspace_root": product_workspace_dir,
                "compatibility_path": f"mesher -> {mesher_target}",
            },
            "evidence_refs": [project_compat_ref],
        },
    ]

    return {
        "version": NAMING_MAP_VERSION,
        "generated_at": iso_now(),
        "inventory_captured_at": captured_at,
        "source_script": SCRIPT_RELATIVE_PATH,
        "surfaces": surfaces,
        "drift_findings": drift_findings,
    }


def build_evidence_entries(
    cache: SourceCache,
    *,
    captured_at: str,
    naming_map: dict[str, Any],
    mesh_issue_index: dict[int, dict[str, Any]],
    hyperpush_issue_index: dict[int, dict[str, Any]],
) -> dict[str, Any]:
    issue8 = require_issue(hyperpush_issue_index, repo=HYPERPUSH_REPO, number=8)
    ensure_issue_body_contains(issue8, "website/docs/.vitepress/config.mts", "hyperpush#8")
    ensure_issue_body_contains(issue8, "website/docs/.vitepress/theme/components/NavBar.vue", "hyperpush#8")
    ensure_issue_body_contains(issue8, "packages.meshlang.dev", "hyperpush#8")

    shipped_mesh_numbers = [3, 4, 5, 6, 8, 9, 10, 11, 13, 14]
    shipped_hyperpush_numbers = [3, 4, 5]
    partial_hyperpush_numbers = [15, 33, 34, 51, 52, 53, 57]

    shipped_mesh_issues = [require_issue(mesh_issue_index, repo=MESH_LANG_REPO, number=number) for number in shipped_mesh_numbers]
    shipped_hyperpush_issues = [
        require_issue(hyperpush_issue_index, repo=HYPERPUSH_REPO, number=number) for number in shipped_hyperpush_numbers
    ]
    partial_hyperpush_issues = [
        require_issue(hyperpush_issue_index, repo=HYPERPUSH_REPO, number=number) for number in partial_hyperpush_numbers
    ]

    naming_product_surface = require_object(
        next(surface for surface in require_array(naming_map.get("surfaces"), "naming_map.surfaces") if surface.get("surface_id") == "hyperpush_product_repo"),
        "naming_map.surfaces[hyperpush_product_repo]",
    )
    naming_docs_surface = require_object(
        next(surface for surface in require_array(naming_map.get("surfaces"), "naming_map.surfaces") if surface.get("surface_id") == "mesh_lang_docs_packages_nav"),
        "naming_map.surfaces[mesh_lang_docs_packages_nav]",
    )
    naming_pitch_surface = require_object(
        next(surface for surface in require_array(naming_map.get("surfaces"), "naming_map.surfaces") if surface.get("surface_id") == "product_pitch_route"),
        "naming_map.surfaces[product_pitch_route]",
    )
    naming_operator_surface = require_object(
        next(surface for surface in require_array(naming_map.get("surfaces"), "naming_map.surfaces") if surface.get("surface_id") == "frontend_exp_operator_app"),
        "naming_map.surfaces[frontend_exp_operator_app]",
    )

    entries = [
        {
            "evidence_id": "mesh_launch_foundations_shipped",
            "classification": "shipped",
            "summary": (
                "Deploy, failover, diagnostics, release-verification, and public-surface truth for the Mesh launch path were "
                "already shipped in M053 and M054 even though multiple mesh-lang tracker rows remain open."
            ),
            "ownership_truth": "language-owned delivery work in mesh-lang",
            "delivery_truth": "shipped across M053 and M054; the tracker rows are stale shipped-but-open language work",
            "workspace_path_truth": "mesh-lang compiler/scripts/docs surfaces remain tracked directly in this repo",
            "public_repo_truth": MESH_LANG_REPO,
            "normalized_canonical_destination": {
                "repo_slug": MESH_LANG_REPO,
                "issue_repo": "mesh-lang",
                "workspace_root": "mesh-lang",
            },
            "issue_matchers": [
                {"repo": MESH_LANG_REPO, "number": number} for number in shipped_mesh_numbers
            ] + [{"repo": HYPERPUSH_REPO, "number": number} for number in shipped_hyperpush_numbers],
            "matched_issue_urls": [
                require_string(issue.get("canonical_issue_url"), f"mesh_launch_foundations_shipped.issue_url[{index}]")
                for index, issue in enumerate([*shipped_mesh_issues, *shipped_hyperpush_issues])
            ],
            "matched_issue_handles": [issue_handle(issue) for issue in [*shipped_mesh_issues, *shipped_hyperpush_issues]],
            "proposed_tracker_action": (
                "Close or rewrite the shipped-but-open tracker rows so they point at the delivered M053/M054 contract instead of describing those launch foundations as pending work."
            ),
            "evidence_refs": [
                build_line_ref(
                    cache,
                    ".gsd/milestones/M053/M053-SUMMARY.md",
                    "M053 started by making the generated Postgres Todo starter own a real staged deploy handoff",
                    "M053 summary records the staged deploy bundle and serious deploy contract as shipped.",
                ),
                build_line_ref(
                    cache,
                    ".gsd/milestones/M053/M053-SUMMARY.md",
                    "the generated Postgres starter is the serious deployable/shared path",
                    "M053 summary records docs/public-surface truth for the serious deployable starter.",
                ),
                build_line_ref(
                    cache,
                    ".gsd/milestones/M054/M054-SUMMARY.md",
                    "one public app URL may choose ingress",
                    "M054 summary records the bounded one-public-URL runtime contract as shipped.",
                ),
                build_line_ref(
                    cache,
                    ".gsd/milestones/M054/M054-SUMMARY.md",
                    "X-Mesh-Continuity-Request-Key",
                    "M054 summary records shipped direct request-key follow-through and diagnostics truth.",
                ),
                build_issue_ref(
                    require_issue(mesh_issue_index, repo=MESH_LANG_REPO, number=3),
                    note="Representative shipped-but-open mesh-lang issue in the deploy/runtime family.",
                ),
            ],
        },
        {
            "evidence_id": "frontend_exp_operator_surfaces_partial",
            "classification": "partial",
            "summary": (
                "frontend-exp exists, but the checked-in operator UI is still backed by MOCK_ISSUES and MOCK_STATS, so the app/backend replacement work remains partially delivered rather than shipped."
            ),
            "ownership_truth": "product-owned operator app work",
            "delivery_truth": "partial only; real backend/data workflows for the operator app remain open",
            "workspace_path_truth": require_string(naming_operator_surface.get("workspace_path_truth"), "frontend_exp_operator_surfaces_partial.workspace_path_truth"),
            "public_repo_truth": require_string(naming_operator_surface.get("public_repo_truth"), "frontend_exp_operator_surfaces_partial.public_repo_truth"),
            "normalized_canonical_destination": require_object(
                naming_operator_surface.get("normalized_canonical_destination"),
                "frontend_exp_operator_surfaces_partial.normalized_canonical_destination",
            ),
            "issue_matchers": [{"repo": HYPERPUSH_REPO, "number": number} for number in partial_hyperpush_numbers],
            "matched_issue_urls": [
                require_string(issue.get("canonical_issue_url"), f"frontend_exp_operator_surfaces_partial.issue_url[{index}]")
                for index, issue in enumerate(partial_hyperpush_issues)
            ],
            "matched_issue_handles": [issue_handle(issue) for issue in partial_hyperpush_issues],
            "proposed_tracker_action": (
                "Keep the product app/backend issues open, and describe them as mock-backed frontend-exp follow-through instead of implying the real operator app already shipped."
            ),
            "evidence_refs": [
                build_line_ref(
                    cache,
                    "mesher/frontend-exp/lib/mock-data.ts",
                    "export const MOCK_ISSUES: Issue[] = [",
                    "frontend-exp still exports hard-coded issue data.",
                ),
                build_line_ref(
                    cache,
                    "mesher/frontend-exp/lib/mock-data.ts",
                    "export const MOCK_STATS = {",
                    "frontend-exp still exports hard-coded dashboard stats.",
                ),
                build_line_ref(
                    cache,
                    ".gsd/PROJECT.md",
                    "the sibling product repo (`hyperpush-org/hyperpush`, still surfaced locally through the `hyperpush-mono` workspace path) owns `mesher/`, `mesher/landing/`, and `mesher/frontend-exp/`",
                    "Project contract names frontend-exp as a product-owned surface.",
                ),
            ],
        },
        {
            "evidence_id": "hyperpush_8_docs_bug_misfiled",
            "classification": "misfiled",
            "summary": (
                "hyperpush#8 is a real website/docs bug, but the issue body points only at mesh-lang-owned VitePress files, so the tracker row belongs in mesh-lang rather than the product repo."
            ),
            "ownership_truth": require_string(naming_docs_surface.get("ownership_truth"), "hyperpush_8_docs_bug_misfiled.ownership_truth"),
            "delivery_truth": "active docs bug on mesh-lang-owned files; current repo placement is wrong",
            "workspace_path_truth": require_string(naming_docs_surface.get("workspace_path_truth"), "hyperpush_8_docs_bug_misfiled.workspace_path_truth"),
            "public_repo_truth": require_string(naming_docs_surface.get("public_repo_truth"), "hyperpush_8_docs_bug_misfiled.public_repo_truth"),
            "normalized_canonical_destination": require_object(
                naming_docs_surface.get("normalized_canonical_destination"),
                "hyperpush_8_docs_bug_misfiled.normalized_canonical_destination",
            ),
            "issue_matchers": [{"repo": HYPERPUSH_REPO, "number": 8}],
            "matched_issue_urls": [require_string(issue8.get("canonical_issue_url"), "hyperpush_8_docs_bug_misfiled.issue_url")],
            "matched_issue_handles": [issue_handle(issue8)],
            "proposed_tracker_action": (
                "Move or recreate hyperpush#8 under mesh-lang and relabel it as a language-repo docs/packages-nav bug."
            ),
            "evidence_refs": [
                build_issue_ref(
                    issue8,
                    note="Issue body explicitly cites mesh-lang docs files and asks for packages.meshlang.dev routing.",
                ),
                build_line_ref(
                    cache,
                    "website/docs/.vitepress/config.mts",
                    "{ text: 'Packages', link: '/packages/' }",
                    "The nav config cited by hyperpush#8 is a mesh-lang docs file.",
                ),
                build_line_ref(
                    cache,
                    "website/docs/.vitepress/theme/components/NavBar.vue",
                    "{ text: 'Packages', href: '/packages/' }",
                    "The custom NavBar cited by hyperpush#8 is also a mesh-lang docs file.",
                ),
            ],
        },
        {
            "evidence_id": "pitch_route_missing_tracker_coverage",
            "classification": "missing_coverage",
            "summary": (
                "The evaluator-facing /pitch route already shipped in the sibling product repo during M056, but there is still no dedicated repo issue or project row that records that delivered surface."
            ),
            "ownership_truth": require_string(naming_pitch_surface.get("ownership_truth"), "pitch_route_missing_tracker_coverage.ownership_truth"),
            "delivery_truth": "shipped in M056 without dedicated tracker coverage",
            "workspace_path_truth": require_string(naming_pitch_surface.get("workspace_path_truth"), "pitch_route_missing_tracker_coverage.workspace_path_truth"),
            "public_repo_truth": require_string(naming_pitch_surface.get("public_repo_truth"), "pitch_route_missing_tracker_coverage.public_repo_truth"),
            "normalized_canonical_destination": require_object(
                naming_pitch_surface.get("normalized_canonical_destination"),
                "pitch_route_missing_tracker_coverage.normalized_canonical_destination",
            ),
            "issue_matchers": [],
            "matched_issue_urls": [],
            "matched_issue_handles": [],
            "proposed_tracker_action": (
                "Create or repoint one hyperpush tracker row so the shipped /pitch route is represented explicitly instead of being implied only by milestone history."
            ),
            "evidence_refs": [
                build_line_ref(
                    cache,
                    ".gsd/milestones/M056/M056-SUMMARY.md",
                    "Shipped a landing-native `/pitch` route inside `mesher/landing/`",
                    "M056 summary records /pitch as shipped.",
                ),
                build_line_ref(
                    cache,
                    "mesher/landing/app/pitch/page.tsx",
                    "return <PitchDeck />",
                    "The shipped /pitch route still exists in the product surface.",
                ),
                build_line_ref(
                    cache,
                    ".gsd/PROJECT.md",
                    "The evaluator-facing `/pitch` route remains the maintained landing artifact there",
                    "Project contract says /pitch lives in the sibling product repo after the split.",
                ),
            ],
            "derived_gap": {
                "gap_id": "product_pitch_route_shipped_without_tracker_row",
                "surface": "/pitch",
            },
        },
        {
            "evidence_id": "product_repo_naming_normalization",
            "classification": "active",
            "summary": (
                "Tracker wording still has to normalize stale hyperpush-mono references to the public hyperpush repo identity whenever a product-owned row points at mesher, landing, or frontend-exp work."
            ),
            "ownership_truth": require_string(naming_product_surface.get("ownership_truth"), "product_repo_naming_normalization.ownership_truth"),
            "delivery_truth": "naming normalization remains active reconciliation work, even though the repo split itself already shipped in M055",
            "workspace_path_truth": require_string(naming_product_surface.get("workspace_path_truth"), "product_repo_naming_normalization.workspace_path_truth"),
            "public_repo_truth": require_string(naming_product_surface.get("public_repo_truth"), "product_repo_naming_normalization.public_repo_truth"),
            "normalized_canonical_destination": require_object(
                naming_product_surface.get("normalized_canonical_destination"),
                "product_repo_naming_normalization.normalized_canonical_destination",
            ),
            "issue_matchers": [],
            "matched_issue_urls": [],
            "matched_issue_handles": [],
            "proposed_tracker_action": (
                "Rewrite stale hyperpush-mono repo mentions on repo issues and project items to the canonical public hyperpush destination while preserving the local hyperpush-mono workspace path only as implementation detail."
            ),
            "evidence_refs": [
                build_line_ref(
                    cache,
                    ".gsd/PROJECT.md",
                    "still surfaced locally through the `hyperpush-mono` workspace path",
                    "Project contract distinguishes the local workspace alias from the public product repo identity.",
                ),
                build_line_ref(
                    cache,
                    "scripts/workspace-git.sh",
                    "if [[ \"$normalized_expected\" == 'https://github.com/hyperpush-org/hyperpush-mono' && \"$normalized_actual\" == 'https://github.com/hyperpush-org/hyperpush' ]]; then",
                    "workspace-git already accepts the public hyperpush remote as the canonical repo identity.",
                ),
                build_snapshot_ref(
                    "hyperpush-issues.snapshot.json",
                    json_pointer="/canonical_redirect",
                    note="Live GitHub inventory capture canonicalized hyperpush-mono to hyperpush.",
                ),
                build_line_ref(
                    cache,
                    ".gsd/milestones/M055/M055-SUMMARY.md",
                    "M055 delivered the repo-boundary split as a contract-and-proof milestone",
                    "M055 summary records the split and naming/ownership reset as shipped.",
                ),
            ],
        },
    ]

    classification_counts: dict[str, int] = {}
    matched_issue_urls: set[str] = set()
    for entry in entries:
        classification = require_string(entry.get("classification"), f"{entry['evidence_id']}.classification")
        classification_counts[classification] = classification_counts.get(classification, 0) + 1
        for url in require_array(entry.get("matched_issue_urls"), f"{entry['evidence_id']}.matched_issue_urls"):
            issue_url = require_string(url, f"{entry['evidence_id']}.matched_issue_url")
            matched_issue_urls.add(issue_url)

    return {
        "version": EVIDENCE_VERSION,
        "generated_at": iso_now(),
        "inventory_captured_at": captured_at,
        "source_script": SCRIPT_RELATIVE_PATH,
        "rollup": {
            "entry_count": len(entries),
            "classification_counts": classification_counts,
            "matched_issue_count": len(matched_issue_urls),
        },
        "entries": entries,
    }


def render_refs(refs: list[dict[str, Any]]) -> list[str]:
    rendered: list[str] = []
    for ref in refs:
        path = ref.get("path")
        note = ref.get("note")
        if not isinstance(path, str) or not isinstance(note, str):
            raise EvidenceError("evidence ref must include string path and note")
        line = ref.get("line")
        json_pointer = ref.get("json_pointer")
        location = path
        if isinstance(line, int):
            location = f"{location}:{line}"
        elif isinstance(json_pointer, str):
            location = f"{location}{json_pointer}"
        rendered.append(f"- `{location}` — {note}")
    return rendered


def render_evidence_markdown(evidence: dict[str, Any]) -> str:
    rollup = require_object(evidence.get("rollup"), "evidence.rollup")
    entries = require_array(evidence.get("entries"), "evidence.entries")

    lines = [
        "# M057 S01 Reconciliation Evidence",
        "",
        f"- Version: `{require_string(evidence.get('version'), 'evidence.version')}`",
        f"- Inventory captured_at: `{require_string(evidence.get('inventory_captured_at'), 'evidence.inventory_captured_at')}`",
        f"- Generated at: `{require_string(evidence.get('generated_at'), 'evidence.generated_at')}`",
        f"- Evidence entries: `{require_int(rollup.get('entry_count'), 'evidence.rollup.entry_count')}`",
        f"- Matched issue URLs: `{require_int(rollup.get('matched_issue_count'), 'evidence.rollup.matched_issue_count')}`",
        "",
        "## Decision anchors",
        "",
    ]

    decision_refs = require_array(evidence.get("decision_refs"), "evidence.decision_refs")
    lines.extend(render_refs([require_object(ref, "evidence.decision_ref") for ref in decision_refs]))
    lines.extend([
        "",
        "## Rollup",
        "",
        "| Evidence ID | Classification | Matched issues | Canonical destination |",
        "| --- | --- | --- | --- |",
    ])

    for entry in entries:
        entry_object = require_object(entry, "evidence.entry")
        matched_handles = require_array(entry_object.get("matched_issue_handles"), f"{entry_object['evidence_id']}.matched_issue_handles")
        matched_label = ", ".join(require_string(item, "matched_issue_handle") for item in matched_handles) or "_none_"
        lines.append(
            "| `{}` | `{}` | {} | `{}` |".format(
                require_string(entry_object.get("evidence_id"), "evidence.entry.evidence_id"),
                require_string(entry_object.get("classification"), "evidence.entry.classification"),
                matched_label,
                render_destination(require_object(entry_object.get("normalized_canonical_destination"), "destination")),
            )
        )

    for entry in entries:
        entry_object = require_object(entry, "evidence.entry")
        evidence_id = require_string(entry_object.get("evidence_id"), "evidence.entry.evidence_id")
        lines.extend(
            [
                "",
                f"## {evidence_id}",
                "",
                require_string(entry_object.get("summary"), f"{evidence_id}.summary"),
                "",
                f"- classification: `{require_string(entry_object.get('classification'), f'{evidence_id}.classification')}`",
                f"- ownership_truth: {require_string(entry_object.get('ownership_truth'), f'{evidence_id}.ownership_truth')}",
                f"- delivery_truth: {require_string(entry_object.get('delivery_truth'), f'{evidence_id}.delivery_truth')}",
                f"- workspace_path_truth: {require_string(entry_object.get('workspace_path_truth'), f'{evidence_id}.workspace_path_truth')}",
                f"- public_repo_truth: {require_string(entry_object.get('public_repo_truth'), f'{evidence_id}.public_repo_truth')}",
                f"- normalized_canonical_destination: `{json.dumps(require_object(entry_object.get('normalized_canonical_destination'), f'{evidence_id}.normalized_canonical_destination'), sort_keys=True)}`",
            ]
        )

        matched_issue_handles = require_array(entry_object.get("matched_issue_handles"), f"{evidence_id}.matched_issue_handles")
        if matched_issue_handles:
            lines.append(
                f"- matched_issues: {', '.join(require_string(item, f'{evidence_id}.matched_issue_handle') for item in matched_issue_handles)}"
            )
        else:
            lines.append("- matched_issues: _none_")

        lines.append(f"- proposed_tracker_action: {require_string(entry_object.get('proposed_tracker_action'), f'{evidence_id}.proposed_tracker_action')}")
        lines.append("- evidence_refs:")
        lines.extend(render_refs(require_array(entry_object.get("evidence_refs"), f"{evidence_id}.evidence_refs")))

    return "\n".join(lines) + "\n"


def validate_naming_map(naming_map: dict[str, Any]) -> dict[str, Any]:
    if require_string(naming_map.get("version"), "naming_map.version") != NAMING_MAP_VERSION:
        raise EvidenceError("naming map version drifted")
    surfaces = require_array(naming_map.get("surfaces"), "naming_map.surfaces")
    if len(surfaces) < 4:
        raise EvidenceError(f"expected at least 4 naming surfaces, found {len(surfaces)}")
    surface_ids: set[str] = set()
    for index, surface in enumerate(surfaces):
        surface_object = require_object(surface, f"naming_map.surfaces[{index}]")
        surface_id = require_string(surface_object.get("surface_id"), f"naming_map.surfaces[{index}].surface_id")
        if surface_id in surface_ids:
            raise EvidenceError(f"duplicate naming_map surface_id {surface_id!r}")
        surface_ids.add(surface_id)
        require_string(surface_object.get("ownership_truth"), f"naming_map.surfaces[{index}].ownership_truth")
        require_string(surface_object.get("delivery_truth"), f"naming_map.surfaces[{index}].delivery_truth")
        require_string(surface_object.get("workspace_path_truth"), f"naming_map.surfaces[{index}].workspace_path_truth")
        require_string(surface_object.get("public_repo_truth"), f"naming_map.surfaces[{index}].public_repo_truth")
        destination = require_object(
            surface_object.get("normalized_canonical_destination"),
            f"naming_map.surfaces[{index}].normalized_canonical_destination",
        )
        require_string(destination.get("repo_slug"), f"naming_map.surfaces[{index}].normalized_canonical_destination.repo_slug")
        refs = require_array(surface_object.get("evidence_refs"), f"naming_map.surfaces[{index}].evidence_refs")
        if not refs:
            raise EvidenceError(f"naming_map.surfaces[{index}] must include at least one evidence_ref")
    if "hyperpush_product_repo" not in surface_ids:
        raise EvidenceError("naming map missing hyperpush_product_repo surface")
    if "mesh_lang_docs_packages_nav" not in surface_ids:
        raise EvidenceError("naming map missing mesh_lang_docs_packages_nav surface")
    return {"surface_count": len(surfaces)}


def validate_evidence_bundle(evidence: dict[str, Any]) -> dict[str, Any]:
    if require_string(evidence.get("version"), "evidence.version") != EVIDENCE_VERSION:
        raise EvidenceError("evidence version drifted")
    entries = require_array(evidence.get("entries"), "evidence.entries")
    if len(entries) < 5:
        raise EvidenceError(f"expected at least 5 evidence entries, found {len(entries)}")
    required_ids = {
        "mesh_launch_foundations_shipped",
        "frontend_exp_operator_surfaces_partial",
        "hyperpush_8_docs_bug_misfiled",
        "pitch_route_missing_tracker_coverage",
        "product_repo_naming_normalization",
    }
    seen_ids: set[str] = set()
    classification_counts: dict[str, int] = {}
    hyperpush_8_found = False
    pitch_gap_found = False
    for index, entry in enumerate(entries):
        entry_object = require_object(entry, f"evidence.entries[{index}]")
        evidence_id = require_string(entry_object.get("evidence_id"), f"evidence.entries[{index}].evidence_id")
        if evidence_id in seen_ids:
            raise EvidenceError(f"duplicate evidence_id {evidence_id!r}")
        seen_ids.add(evidence_id)
        classification = require_string(entry_object.get("classification"), f"{evidence_id}.classification")
        classification_counts[classification] = classification_counts.get(classification, 0) + 1
        require_string(entry_object.get("summary"), f"{evidence_id}.summary")
        require_string(entry_object.get("ownership_truth"), f"{evidence_id}.ownership_truth")
        require_string(entry_object.get("delivery_truth"), f"{evidence_id}.delivery_truth")
        require_string(entry_object.get("workspace_path_truth"), f"{evidence_id}.workspace_path_truth")
        require_string(entry_object.get("public_repo_truth"), f"{evidence_id}.public_repo_truth")
        require_object(entry_object.get("normalized_canonical_destination"), f"{evidence_id}.normalized_canonical_destination")
        evidence_refs = require_array(entry_object.get("evidence_refs"), f"{evidence_id}.evidence_refs")
        if not evidence_refs:
            raise EvidenceError(f"{evidence_id} must include evidence_refs")
        proposed_action = require_string(entry_object.get("proposed_tracker_action"), f"{evidence_id}.proposed_tracker_action")
        if proposed_action.strip() == "":
            raise EvidenceError(f"{evidence_id}.proposed_tracker_action cannot be blank")

        matched_handles = [
            require_string(handle, f"{evidence_id}.matched_issue_handles[]")
            for handle in require_array(entry_object.get("matched_issue_handles"), f"{evidence_id}.matched_issue_handles")
        ]
        if evidence_id == "hyperpush_8_docs_bug_misfiled":
            hyperpush_8_found = "hyperpush#8" in matched_handles
            if not hyperpush_8_found:
                raise EvidenceError("hyperpush_8_docs_bug_misfiled must match hyperpush#8")
        if evidence_id == "pitch_route_missing_tracker_coverage":
            summary = require_string(entry_object.get("summary"), f"{evidence_id}.summary")
            if "/pitch" not in summary:
                raise EvidenceError("pitch_route_missing_tracker_coverage summary must mention /pitch")
            pitch_gap = require_object(entry_object.get("derived_gap"), f"{evidence_id}.derived_gap")
            if require_string(pitch_gap.get("surface"), f"{evidence_id}.derived_gap.surface") != "/pitch":
                raise EvidenceError("pitch_route_missing_tracker_coverage derived_gap.surface must be /pitch")
            pitch_gap_found = True

    missing_ids = sorted(required_ids - seen_ids)
    if missing_ids:
        raise EvidenceError(f"missing required evidence ids: {', '.join(missing_ids)}")
    if not hyperpush_8_found:
        raise EvidenceError("expected hyperpush#8 misfiled evidence row")
    if not pitch_gap_found:
        raise EvidenceError("expected /pitch missing coverage evidence row")
    return {
        "entry_count": len(entries),
        "classification_counts": classification_counts,
    }


def build_outputs(*, source_root: Path, output_dir: Path) -> tuple[dict[str, Any], str, dict[str, Any], dict[str, Any]]:
    cache = SourceCache(source_root)
    snapshots = load_snapshots(output_dir)
    inventory_summary = validate_snapshots(snapshots)
    captured_at = require_string(inventory_summary.get("captured_at"), "inventory_summary.captured_at")
    hyperpush_snapshot = require_object(snapshots.get("hyperpush_issues"), "snapshots.hyperpush_issues")
    canonical_redirect = require_object(hyperpush_snapshot.get("canonical_redirect"), "hyperpush_issues.canonical_redirect")

    mesh_issue_index = load_issue_index(require_object(snapshots.get("mesh_lang_issues"), "snapshots.mesh_lang_issues"), "mesh_lang_issues")
    hyperpush_issue_index = load_issue_index(hyperpush_snapshot, "hyperpush_issues")

    naming_map = build_naming_map(
        cache,
        source_root=source_root,
        captured_at=captured_at,
        canonical_redirect=canonical_redirect,
    )
    decision_refs = build_decision_refs(cache)
    naming_map["decision_refs"] = decision_refs
    evidence = build_evidence_entries(
        cache,
        captured_at=captured_at,
        naming_map=naming_map,
        mesh_issue_index=mesh_issue_index,
        hyperpush_issue_index=hyperpush_issue_index,
    )
    evidence["decision_refs"] = decision_refs
    markdown = render_evidence_markdown(evidence)
    return evidence, markdown, naming_map, inventory_summary


def write_outputs(
    *,
    output_dir: Path,
    evidence: dict[str, Any],
    evidence_markdown: str,
    naming_map: dict[str, Any],
) -> list[Path]:
    output_dir.mkdir(parents=True, exist_ok=True)
    evidence_json_path = output_dir / EVIDENCE_JSON_FILENAME
    evidence_md_path = output_dir / EVIDENCE_MD_FILENAME
    naming_map_path = output_dir / NAMING_MAP_FILENAME
    write_json_atomic(evidence_json_path, evidence)
    evidence_md_path.write_text(evidence_markdown, encoding="utf8")
    write_json_atomic(naming_map_path, naming_map)
    return [evidence_json_path, evidence_md_path, naming_map_path]


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Build the M057 S01 code-evidence index and naming/ownership map.")
    parser.add_argument("--source-root", type=Path, default=ROOT, help="Alternate source root for isolated contract tests.")
    parser.add_argument("--output-dir", type=Path, help="Directory containing the T01 snapshots and receiving generated outputs.")
    parser.add_argument("--check", action="store_true", help="Validate invariants after generating outputs.")
    args = parser.parse_args(argv)
    args.source_root = args.source_root.resolve()
    if args.output_dir is None:
        args.output_dir = (args.source_root / DEFAULT_OUTPUT_SUBDIR).resolve()
    else:
        args.output_dir = args.output_dir.resolve()
    return args


def main(argv: list[str]) -> int:
    args = parse_args(argv)
    evidence, evidence_markdown, naming_map, inventory_summary = build_outputs(
        source_root=args.source_root,
        output_dir=args.output_dir,
    )
    written_paths = write_outputs(
        output_dir=args.output_dir,
        evidence=evidence,
        evidence_markdown=evidence_markdown,
        naming_map=naming_map,
    )

    check_summary: dict[str, Any] | None = None
    if args.check:
        check_summary = {
            "naming_map": validate_naming_map(naming_map),
            "evidence": validate_evidence_bundle(evidence),
        }

    print(
        json.dumps(
            {
                "status": "ok",
                "output_dir": str(args.output_dir),
                "written_files": [str(path) for path in written_paths],
                "inventory": inventory_summary,
                "evidence": require_object(evidence.get("rollup"), "evidence.rollup"),
                "check": check_summary,
            },
            indent=2,
        )
    )
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main(sys.argv[1:]))
    except (EvidenceError, InventoryError) as exc:
        print(str(exc), file=sys.stderr)
        raise SystemExit(1)
