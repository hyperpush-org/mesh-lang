---
id: S01
parent: M040
milestone: M040
provides:
  - A keyed `/work` submit/status JSON contract with stable request-key identity, distinct attempt identity, idempotent retry behavior, and fail-closed conflicting reuse.
  - A truthful standalone identity fallback (`standalone@local`) that keeps ingress/owner/execution fields non-empty on the existing proof rail.
  - Named verifier surfaces (`e2e_m040_s01` and `scripts/verify-m040-s01.sh`) that downstream slices can replay before layering replica-backed continuity on top.
requires:
  []
affects:
  - S02
  - S03
key_files:
  - cluster-proof/work.mpl
  - cluster-proof/tests/work.test.mpl
  - compiler/meshc/tests/e2e_m040_s01.rs
  - scripts/verify-m040-s01.sh
  - .gsd/PROJECT.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Use stable `request_key` identity with distinct `attempt_id` values so keyed retries can report one logical request without duplicate completion leakage.
  - Keep the keyed submit runtime split into explicit decision and next-state helpers rather than the earlier failing tuple-extraction path.
  - Fall back to `standalone@local` whenever `Node.self()` is empty so the standalone proof rail keeps truthful non-empty ingress/owner/execution identity fields.
patterns_established:
  - Treat `/work` submit and `/work/:request_key` status as one keyed contract: same-key same-payload retries must reuse the original attempt, while same-key conflicting payloads must fail closed with `request_key_conflict`.
  - Use a synthetic local node identity in standalone mode instead of leaving ingress/owner blank; blank node identity should be treated as an implementation bug, not as acceptable proof output.
  - Archive verifier JSON and logs (`create`, `completed`, `duplicate`, `conflict`, `missing`, plus `phase-report.txt`) so later continuity slices can diff contract drift mechanically instead of re-deriving expected payloads from prose.
observability_surfaces:
  - `cluster-proof` runtime logs for work submit, execution, status transition, dedupe, conflict, and missing-status lookups.
  - `.tmp/m040-s01/verify/01-create.json`, `02-completed.json`, `03-duplicate.json`, `04-conflict.json`, `05-missing.json`, and `summary.json` as retained contract artifacts.
  - `.tmp/m040-s01/verify/phase-report.txt` plus `cluster-proof.stdout.log`/`cluster-proof.stderr.log` for operator replay and debugging.
  - `.tmp/m040-s01/e2e/run-*/cluster-proof.{stdout,stderr}.log` from the Rust runtime harness.
drill_down_paths:
  - .gsd/milestones/M040/slices/S01/tasks/T01-SUMMARY.md
  - .gsd/milestones/M040/slices/S01/tasks/T02-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-28T18:20:47.857Z
blocker_discovered: false
---

# S01: Keyed Submit/Status Contract on the Existing Proof Rail

**Added a keyed standalone submit/status contract on `cluster-proof` with stable `request_key` identity, distinct `attempt_id`, idempotent same-key retry, fail-closed conflicting reuse, and durable verifier evidence.**

## What Happened

S01 closed the first keyed continuity seam on top of the existing `cluster-proof` rail without widening the public namespace beyond `/work`. `cluster-proof/work.mpl` now maintains a keyed request model that separates stable logical identity (`request_key`) from execution identity (`attempt_id`), preserves truthful owner/replica fields in the response payload, and reuses the original attempt on same-key same-payload retries instead of leaking duplicate completion. To keep the standalone proof rail honest, the work path now falls back to a synthetic local node identity (`standalone@local`) whenever `Node.self()` is empty, which makes standalone submit/status requests produce truthful non-empty ingress, owner, and execution fields instead of the previous `invalid_target_selection` 500. The pure Mesh tests in `cluster-proof/tests/work.test.mpl` now cover the route-selection, validation, and parsing contract that later replica-backed work will reuse, and the new Rust harness `compiler/meshc/tests/e2e_m040_s01.rs` exercises the runtime contract end to end: submit keyed work, poll keyed status to completion, retry the same key without duplicate completion, reject conflicting same-key reuse, and return a truthful 404 missing-status payload. `scripts/verify-m040-s01.sh` adds the slice’s operator-facing proof surface by replaying the standalone flow and archiving `create`, `completed`, `duplicate`, `conflict`, and `missing` JSON plus the supporting cluster-proof logs under `.tmp/m040-s01/verify/` for downstream slices.

## Verification

Passed all planned S01 verification commands:
- `cargo run -q -p meshc -- test cluster-proof/tests`
- `cargo run -q -p meshc -- build cluster-proof`
- `cargo test -p meshc --test e2e_m040_s01 -- --nocapture`
- `bash scripts/verify-m040-s01.sh`

Verifier evidence confirms the keyed contract on the live standalone proof rail:
- initial submit returned `phase=submitted`, `result=pending`, `attempt_id=attempt-0`, `ingress_node=owner_node=standalone@local`, and `replica_status=unassigned`
- keyed status converged to `phase=completed`, `result=succeeded`, and `execution_node=standalone@local`
- same-key same-payload retry returned HTTP 200 with the original completed attempt
- same-key conflicting retry returned HTTP 409 with `conflict_reason=request_key_conflict`
- missing keyed status returned HTTP 404 with `error=request_key_not_found`

### Operational Readiness (Q8)
- **Health signal:** `GET /membership` returns HTTP 200, the process logs `work services ready`, and keyed status reaches `phase=completed` for an accepted request.
- **Failure signal:** `POST /work` returning `invalid_target_selection` indicates the standalone identity fallback regressed; clustered startup is still blocked if the process aborts with `compiler/mesh-rt/src/string.rs:171:14` (`misaligned pointer dereference`).
- **Recovery procedure:** rerun the four slice verification commands, inspect `.tmp/m040-s01/verify/*.json` plus `cluster-proof.stdout.log`, and start root-cause analysis in `effective_work_node_name()` / `current_target_selection()` if standalone ingress or owner fields go blank again.
- **Monitoring gaps:** there is still no live clustered keyed-flow proof or replica-backed telemetry in S01; owner-loss/replica safety signals remain future-slice work.

## Requirements Advanced

None.

## Requirements Validated

None.

## New Requirements Surfaced

- None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

The slice kept the authoritative proof surface on the standalone keyed contract plus pure route-selection tests instead of a live two-node keyed replay. That deviation was necessary because clustered startup still aborts before HTTP is ready with the existing `compiler/mesh-rt/src/string.rs:171:14` misaligned-pointer crash, so S01 closes the keyed API/status contract honestly without claiming clustered keyed continuity yet.

## Known Limitations

Live clustered keyed submit/status is still unproven because cluster-mode startup aborts before the keyed two-node flow can run. In the current standalone proof, replica fields remain placeholders (`replica_node=""`, `replica_status="unassigned"`), and `cluster-proof/tests/work.test.mpl` currently proves routing/validation/parser truth rather than the full mutation state machine because cross-module tuple/opaque-state assertions were not a stable unit-test surface in Mesh.

## Follow-ups

Retire the existing cluster-mode startup crash so S02 can run a real two-node keyed continuity proof, then extend the keyed runtime from standalone placeholders into replica-backed admission/status truth. If Mesh gains a more stable public state-test seam later, backfill deeper unit coverage for keyed mutation flows without depending on opaque cross-module tuple state.

## Files Created/Modified

- `cluster-proof/work.mpl` — Added the keyed runtime contract, standalone local-identity fallback, and the live submit/status behavior used by the new proof surfaces.
- `cluster-proof/tests/work.test.mpl` — Replaced the red keyed test draft with a stable route-selection, validation, and keyed-submit parsing contract that compiles cleanly under Mesh.
- `compiler/meshc/tests/e2e_m040_s01.rs` — Added the standalone runtime harness that submits keyed work, polls keyed status, proves idempotent retry, rejects conflicting same-key reuse, and checks missing-status behavior.
- `scripts/verify-m040-s01.sh` — Added the repo-root verifier that archives keyed submit/status JSON and supporting logs under `.tmp/m040-s01/verify/`.
- `.gsd/KNOWLEDGE.md` — Updated the M040/S01 closeout guidance to reflect the new standalone identity fallback and the remaining clustered runtime blocker.
- `.gsd/PROJECT.md` — Refreshed project state to mark S01 closed on the standalone keyed contract path and to document the remaining clustered startup gap.
