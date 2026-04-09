---
id: S04
parent: M043
milestone: M043
provides:
  - Canonical M043 public proof surface and runbook wording tied mechanically to the shipped same-image failover rail.
  - A read-only Fly verifier/help path that checks config/payload authority fields without implying destructive cloud failover proof.
requires:
  - slice: S03
    provides: The destructive same-image two-cluster failover authority (`scripts/verify-m043-s03.sh`) that the public docs and Fly help path now defer to.
affects:
  []
key_files:
  - scripts/verify-m043-s04-proof-surface.sh
  - scripts/verify-m043-s04-fly.sh
  - cluster-proof/README.md
  - website/docs/docs/distributed-proof/index.md
  - website/docs/docs/distributed/index.md
  - README.md
  - .gsd/KNOWLEDGE.md
  - .gsd/PROJECT.md
key_decisions:
  - Keep `bash scripts/verify-m043-s03.sh` as the destructive local failover authority and make the Fly rail explicitly read-only.
  - Require `cluster_role`, `promotion_epoch`, and `replication_health` on live `/membership` and optional keyed-status payloads instead of reusing the older M042 status shape.
  - Treat the repo README and generic distributed guide as routing surfaces that defer failover/operator claims to the canonical proof page and runbook.
  - Parse `fly config show` as TOML in the Fly verifier so config drift checks do not depend on a JSON-only CLI shape.
patterns_established:
  - Pair a fail-closed docs-truth verifier with a separate read-only live-environment verifier so public claims and operator probes stay aligned without overclaiming destructive proof.
  - Keep one canonical destructive local authority (`scripts/verify-m043-s03.sh`) and make every cloud/environment helper explicitly subordinate to that authority.
  - Retain `status.txt`, `current-phase.txt`, `phase-report.txt`, and full logs for proof-surface and live-verifier rails so failures localize to one named phase instead of disappearing into generic shell output.
observability_surfaces:
  - `.tmp/m043-s04/proof-surface/{status.txt,current-phase.txt,phase-report.txt,full-contract.log}` for public-contract drift localization.
  - `.tmp/m043-s04/fly/{status.txt,current-phase.txt,phase-report.txt,full-contract.log,input-validation.log}` for read-only Fly help/live-path failures.
  - Runtime-owned `cluster_role`, `promotion_epoch`, and `replication_health` fields on live `/membership` and optional `/work/:request_key` responses as the operator-facing authority truth.
drill_down_paths:
  - .gsd/milestones/M043/slices/S04/tasks/T01-SUMMARY.md
  - .gsd/milestones/M043/slices/S04/tasks/T02-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-29T12:23:18.467Z
blocker_discovered: false
---

# S04: Public Proof Surface and Operator Contract Truth

**Mechanical docs/verifier rails now publish the shipped M043 failover contract and keep Fly evidence explicitly read-only.**

## What Happened

This slice closed the last public/operator gap in the M043 chain by turning the shipped failover behavior into an executable documentation contract instead of a prose-only promise. T01 forked the old proof-surface/Fly rails into M043-specific verifiers: `scripts/verify-m043-s04-proof-surface.sh` now fail-closes on stale script names or missing M043 failover wording, and `scripts/verify-m043-s04-fly.sh` keeps the Fly lane read-only while validating the authority-bearing payload shape (`cluster_role`, `promotion_epoch`, `replication_health`) and retaining per-phase artifacts under `.tmp/m043-s04/`. T02 then reconciled the public surfaces to those rails: `cluster-proof/README.md` became the deep runbook for the explicit `/promote` boundary, same-image local authority, stale-primary fencing, and bounded Fly scope; `website/docs/docs/distributed-proof/index.md` became the canonical public proof map; `website/docs/docs/distributed/index.md` and `README.md` were reduced to routing surfaces that point operator claims back to the canonical proof page and runbook instead of duplicating weaker wording. The main implementation gotcha was mechanical rather than conceptual: the proof-surface verifier compares the continued Fly live command block as exact markdown text, including doubled trailing backslashes, so the docs had to match the verifier’s literal escaped form. During closeout I re-ran the whole slice acceptance bundle, confirmed the retained artifact surfaces themselves, added one project-knowledge note about parsing `fly config show` as TOML instead of depending on a JSON-only CLI shape, and updated `.gsd/PROJECT.md` so the repo’s living state now reflects that the public M043 proof/docs rail is complete while live Fly evidence remains a separate environment-dependent gap.

## Verification

Re-ran the slice-level checks from the plan and confirmed the diagnostic surfaces. `bash -n scripts/verify-m043-s04-proof-surface.sh scripts/verify-m043-s04-fly.sh` passed, so both verifier scripts still parse. `bash scripts/verify-m043-s04-proof-surface.sh` passed and wrote an `ok` / `complete` proof bundle under `.tmp/m043-s04/proof-surface/` with per-phase records for proof-page, guide, README, runbook, sidebar, command-list, and stale-wording checks. `bash scripts/verify-m043-s04-fly.sh --help` passed and still names `bash scripts/verify-m043-s03.sh` as the destructive local authority while banning deploys, POST `/work`, and POST `/promote`. `npm --prefix website run build` passed, so the VitePress docs still render with the updated wording. `bash scripts/verify-m043-s03.sh` passed again, re-proving the destructive same-image failover authority that the docs now point to. For observability/negative-path confirmation, `bash scripts/verify-m043-s04-fly.sh` without env failed closed at `input-validation`, and the retained `.tmp/m043-s04/fly/` bundle truthfully recorded `status.txt=failed`, `current-phase.txt=input-validation`, and `input-validation.log` with the missing-env reason.

## Requirements Advanced

- R052 — Clarified and mechanically enforced the same-image + small-env operator contract on the public surfaces while keeping the Fly lane bounded and read-only; the remaining gap is live app evidence, not docs/verifier drift.
- R053 — Reconciled the runbook, proof page, distributed guide, README, and verifier pair to the shipped M043 failover contract so public distributed claims now track the current local authority instead of stale M042 wording.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

None.

## Known Limitations

The Fly verifier remains intentionally read-only and requires an already-deployed app to exercise live mode. This slice does not add destructive failover proof on Fly, does not create keyed work remotely, and does not validate live Fly payloads unless an operator supplies `CLUSTER_PROOF_FLY_APP`/`CLUSTER_PROOF_BASE_URL` (and optionally an existing `CLUSTER_PROOF_REQUEST_KEY`). Requirement R052 therefore remains an environment-backed follow-up rather than something this slice can close locally.

## Follow-ups

Capture real live Fly read-only evidence for the M043 contract against a deployed `cluster-proof` app so requirement R052 can move from the local/public operator rail to an environment-backed proof bundle.

## Files Created/Modified

- `scripts/verify-m043-s04-proof-surface.sh` — Added the fail-closed public proof-surface verifier that checks M043 command names, required failover wording, sidebar wiring, and retained artifacts.
- `scripts/verify-m043-s04-fly.sh` — Added the read-only Fly verifier/help rail with input validation, TOML config checks, authority-field validation, and retained phase artifacts.
- `cluster-proof/README.md` — Rewrote the operator runbook around the M043 failover contract: explicit `/promote`, runtime-owned authority fields, same-image local authority, stale-primary fencing, and bounded Fly scope.
- `website/docs/docs/distributed-proof/index.md` — Published the canonical public M043 proof page with the authoritative command list, failover contract summary, supported topology, non-goals, and failure map.
- `website/docs/docs/distributed/index.md` — Turned the generic distributed guide into a routing surface that sends operator/failover claims to the proof page and runbook instead of duplicating them.
- `README.md` — Updated the repo landing page to route distributed failover claims to the canonical proof page and cluster-proof runbook.
- `.gsd/KNOWLEDGE.md` — Recorded the M043/S04 Fly verifier gotcha that `fly config show` should be parsed as TOML and validated against the packaged config contract.
- `.gsd/PROJECT.md` — Refreshed the living project state so it now reflects M043/S04 as complete and scopes the remaining Fly gap to live environment evidence only.
