---
id: S04
parent: M042
milestone: M042
provides:
  - A visibly thin `cluster-proof` consumer over runtime-owned continuity
  - A green packaged one-image Docker verifier for keyed continuity (`bash scripts/verify-m042-s04.sh`)
  - A read-only Fly helper/help contract aligned with the same authority boundary
  - Mechanically checked README/docs/distributed-proof wording for the runtime-owned continuity story
requires:
  - slice: S03
    provides: Runtime-owned owner-loss recovery, same-key retry, stale-completion fencing, and the authoritative local verifier `bash scripts/verify-m042-s03.sh`.
affects:
  - M043
key_files:
  - cluster-proof/work.mpl
  - cluster-proof/work_legacy.mpl
  - cluster-proof/work_continuity.mpl
  - cluster-proof/main.mpl
  - cluster-proof/tests/work.test.mpl
  - compiler/mesh-rt/src/dist/node.rs
  - scripts/lib/m039_cluster_proof.sh
  - scripts/lib/m042_cluster_proof.sh
  - scripts/verify-m039-s04.sh
  - scripts/verify-m042-s03.sh
  - scripts/verify-m042-s04.sh
  - scripts/verify-m042-s04-fly.sh
  - scripts/verify-m042-s04-proof-surface.sh
  - cluster-proof/README.md
  - website/docs/docs/distributed-proof/index.md
  - website/docs/docs/distributed/index.md
  - website/docs/.vitepress/config.mts
  - README.md
key_decisions:
  - Keep `cluster-proof` as a thin proof consumer by isolating legacy `/work` behavior from keyed continuity adaptation instead of reintroducing app-authored continuity logic.
  - Preserve `scripts/verify-m039-s04.sh` as the historical baseline and layer M042 packaged/read-only wrappers on top rather than rewriting M039 history.
  - Repair remote execution through a runtime-safe remote spawn path so the one-image operator rails prove the real distributed surface again.
  - Treat `bash scripts/verify-m042-s03.sh` as the destructive local continuity authority and keep the Fly lane explicitly read-only in code, docs, and help text.
patterns_established:
  - Thin proof apps should adapt runtime-owned capabilities through small modules with explicit seams (`work.mpl` + `work_legacy.mpl` + `work_continuity.mpl`) instead of hiding core semantics in one monolith.
  - When a new operator rail supersedes an older proof surface, preserve the older verifier as historical baseline and add wrapper verifiers for the new contract instead of mutating validated history.
  - Operator/docs closeout should be fail-closed and mechanical: one proof-surface verifier, one help contract, and one authoritative command list shared by README and public docs.
  - Packaged distributed verifiers should archive per-phase JSON/log artifacts so the first failing phase is obvious from the artifact root alone.
observability_surfaces:
  - `.tmp/m042-s04/` packaged operator artifact root with keyed submit/status proof outputs
  - `scripts/verify-m042-s04-proof-surface.sh` for README/docs/help contract drift
  - `bash scripts/verify-m042-s04-fly.sh --help` as the explicit read-only Fly contract surface
  - Green replay of `.tmp/m039-s04/` and `.tmp/m042-s03/verify/` as prerequisite operator/continuity evidence
drill_down_paths:
  - .gsd/milestones/M042/slices/S04/tasks/T01-SUMMARY.md
  - .gsd/milestones/M042/slices/S04/tasks/T02-SUMMARY.md
  - .gsd/milestones/M042/slices/S04/tasks/T03-SUMMARY.md
  - .gsd/milestones/M042/slices/S04/tasks/T04-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-29T05:32:30.418Z
blocker_discovered: false
---

# S04: Thin cluster-proof consumer and truthful operator/docs rail

**S04 finished M042 by making `cluster-proof` a visibly thin consumer over the runtime-native continuity API, restoring the green one-image operator rails, and aligning docs/help/verifiers with that runtime-owned contract.**

## What Happened

S04 closed the milestone at the consumer and operator layer instead of adding new continuity semantics. The `cluster-proof` surface now reads as three concerns with explicit seams: shared placement/HTTP glue in `work.mpl`, legacy `GET /work` probe behavior in `work_legacy.mpl`, and keyed `POST /work` / `GET /work/:request_key` adaptation over the runtime `Continuity.*` API in `work_continuity.mpl`. That keeps the proof app visibly thin and avoids reintroducing Mesh-side continuity state.

The slice also repaired the remaining distributed execution seam that kept the operator wrappers red. Remote work now reaches the peer through a runtime-safe path instead of depending on the old raw-string / wrong-context spawn behavior, which brought the historical M039 one-image wrapper back to green and let the packaged M042 one-image wrapper prove runtime-owned keyed continuity end to end.

On top of that repaired runtime/app seam, the slice added and stabilized the M042 operator and documentation rail: the packaged local wrapper proves keyed submit/status truth through the repo-root Docker image, the Fly helper stays explicitly read-only, and the proof page / distributed guide / repo README are mechanically checked against the same command set and authority boundary.

## Operational Readiness (Q8)
- **Health signal:** `bash scripts/verify-m042-s03.sh` remains the destructive local authority for owner-loss/rejoin continuity; `bash scripts/verify-m042-s04.sh` proves the packaged one-image keyed rail; successful runs emit phase reports plus copied JSON/log artifacts under `.tmp/m042-s03/verify/` and `.tmp/m042-s04/`.
- **Failure signal:** the wrappers fail closed on the first drifting phase, and the packaged keyed path surfaces mismatches directly in submit/status payload checks (`packaged keyed submit response`, `packaged pending keyed status`, `packaged completed keyed status`). The docs/help rail likewise fails closed through `scripts/verify-m042-s04-proof-surface.sh` and the read-only Fly help contract.
- **Recovery procedure:** rerun the historical baseline first (`bash scripts/verify-m039-s04.sh`), then the local continuity authority (`bash scripts/verify-m042-s03.sh`), then the packaged wrapper (`bash scripts/verify-m042-s04.sh`). For live operator sanity only, use `bash scripts/verify-m042-s04-fly.sh --help` and then the read-only Fly mode with an existing deployment.
- **Monitoring gaps:** the Fly rail is still read-only and requires an existing app/request key for live inspection, so it does not provide destructive recovery evidence. Cross-cluster disaster continuity is still unproven here and remains follow-on M043 work.

## Verification

Passed the full assembled slice verification bundle:

- `cargo run -q -p meshc -- test cluster-proof/tests && cargo run -q -p meshc -- build cluster-proof`
- `bash scripts/verify-m039-s04.sh`
- `bash scripts/verify-m042-s03.sh`
- `bash scripts/verify-m042-s04.sh`
- `bash scripts/verify-m042-s04-fly.sh --help`
- `bash scripts/verify-m042-s04-proof-surface.sh`
- `npm --prefix website run build`

Observed success signals included:
- `verify-m039-s04: ok`
- `verify-m042-s03: ok`
- `verify-m042-s04: ok`
- packaged keyed submit/status checks reporting `keyed payload ok` on node-a and node-b
- VitePress build completing successfully after the proof-surface verifier passed

## Requirements Advanced

None.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

None. The slice finished by retiring the earlier T02/T03 blockers and then rerunning the full planned acceptance bundle successfully.

## Known Limitations

The Fly helper remains intentionally read-only and depends on an already-deployed app (plus an existing request key for keyed-status inspection). This slice does not add exactly-once claims, process-state migration, or cross-cluster disaster continuity; those remain outside the verified S04 contract.

## Follow-ups

M043 should extend the same runtime-owned continuity model across primary/standby clusters. Separately, live Fly evidence for the packaged operator rail can be captured when an app exists, but it is not required for the completed local S04 proof surface.

## Files Created/Modified

- `cluster-proof/work.mpl` — Reduced the shared work surface to placement and HTTP glue so continuity adaptation and legacy probing are visibly separate concerns.
- `cluster-proof/work_legacy.mpl` — Isolated the legacy `GET /work` proof behavior from the keyed continuity submit/status path.
- `cluster-proof/work_continuity.mpl` — Kept keyed submit/status as a thin adapter over `Continuity.*` without reintroducing Mesh-side continuity state.
- `cluster-proof/main.mpl` — Rewired the app entrypoint around the split work modules and the repaired remote execution path.
- `cluster-proof/tests/work.test.mpl` — Kept legacy probe and keyed continuity contract checks green after the module split.
- `compiler/mesh-rt/src/dist/node.rs` — Repaired the cross-node remote spawn seam so packaged and historical cluster-proof operator rails could execute remote work truthfully again.
- `scripts/lib/m039_cluster_proof.sh` — Preserved the historical helper baseline while keeping the operator artifact and timeout behavior fail-closed.
- `scripts/lib/m042_cluster_proof.sh` — Added the packaged keyed continuity helper rail used by the M042 wrapper verifier.
- `scripts/verify-m039-s04.sh` — Kept the historical one-image operator wrapper replayable as the validated M039 baseline.
- `scripts/verify-m042-s03.sh` — Served as the destructive local continuity authority and prerequisite replay for the packaged S04 wrapper.
- `scripts/verify-m042-s04.sh` — Proved packaged runtime-owned keyed continuity through the repo-root one-image Docker path.
- `scripts/verify-m042-s04-fly.sh` — Defined the read-only Fly sanity/config/log/probe rail and its help contract.
- `scripts/verify-m042-s04-proof-surface.sh` — Mechanically verified README/docs/sidebar/help wording against the actual S04 proof contract.
- `cluster-proof/README.md` — Documented the thin-consumer runbook, local authority boundary, and read-only Fly scope.
- `website/docs/docs/distributed-proof/index.md` — Published the runtime-owned distributed proof page around the verified M042 command set.
- `website/docs/docs/distributed/index.md` — Aligned the distributed guide with the runtime-owned `Continuity` story and proof-page routing.
- `website/docs/.vitepress/config.mts` — Kept the public docs navigation wired to the distributed proof surface.
- `README.md` — Aligned the repo entrypoint with the same distributed proof and operator authority wording.
