---
id: S02
parent: M042
milestone: M042
provides:
  - Runtime-native replica-backed continuity admission that either mirrors pending work after replica prepare/ack or durably rejects the request when required replica safety is unavailable.
  - Truthful same-key replay for rejected and accepted continuity records through the ordinary `cluster-proof` `/work` and `/work/:request_key` surfaces without app-authored replica acknowledgement.
  - A dedicated S02 proof rail (`e2e_m042_s02` + `scripts/verify-m042-s02.sh`) with copied evidence bundles for rejected, mirrored, and degraded status transitions.
requires:
  - slice: S01
    provides: Runtime-owned continuity registry, Mesh-facing `Continuity` API, and the standalone keyed submit/status proof rail that S02 extends with replica-backed admission truth.
affects:
  - S03
  - S04
key_files:
  - compiler/mesh-rt/src/dist/continuity.rs
  - compiler/mesh-rt/src/dist/node.rs
  - compiler/mesh-rt/src/lib.rs
  - compiler/mesh-typeck/src/infer.rs
  - compiler/mesh-codegen/src/codegen/intrinsics.rs
  - compiler/mesh-codegen/src/mir/lower.rs
  - cluster-proof/work.mpl
  - cluster-proof/tests/work.test.mpl
  - compiler/meshc/tests/e2e_m042_s02.rs
  - scripts/verify-m042-s02.sh
key_decisions:
  - Extend `Continuity.submit(...)` with an explicit replica requirement so the runtime, not `cluster-proof`, owns replica-backed admission truth.
  - Keep `mesh_continuity_submit` as a compatibility wrapper around `mesh_continuity_submit_with_durability(...)` until the compiler/runtime seam is fully updated.
  - Treat `bash scripts/verify-m042-s02.sh` as the authoritative local acceptance surface instead of reusing the unrelatedly-broken full S01 two-node verifier.
patterns_established:
  - Use explicit `required_replica_count` data at the submit boundary instead of inferring durability policy from replica-node shape.
  - Prefer monotonic continuity merge precedence so later safer truth (`rejected`, `degraded_continuing`) cannot be overwritten by stale mirrored state.
  - Expose durable continuity truth through the ordinary status API and structured runtime transition logs rather than app-authored acknowledgement state.
  - When a broader legacy verifier is blocked by an unrelated failure, add a dedicated slice verifier that replays only the stable prerequisites, fails closed on named test counts, and archives copied evidence bundles.
observability_surfaces:
  - `GET /work/:request_key` status JSON with stable `phase`, `result`, `replica_status`, `owner_node`, `replica_node`, and `error` fields for rejected/mirrored/degraded truth.
  - Structured runtime stderr lines from `[mesh-rt continuity] transition=*` showing `request_key`, `attempt_id`, `replica_status`, and downgrade/rejection reasons.
  - Copied proof bundles under `.tmp/m042-s02/verify/` (`05-rejection-artifacts`, `06-mirrored-artifacts`, `07-degraded-artifacts`) plus `phase-report.txt` and test-count logs.
drill_down_paths:
  - .gsd/milestones/M042/slices/S02/tasks/T01-SUMMARY.md
  - .gsd/milestones/M042/slices/S02/tasks/T02-SUMMARY.md
  - .gsd/milestones/M042/slices/S02/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-28T23:40:43.867Z
blocker_discovered: false
---

# S02: Replica-backed admission and fail-closed durability truth

**Runtime-owned keyed continuity now fails closed on replica-backed durability: replica-required submits either persist a durable rejected record or enter mirrored pending state, and later replica loss downgrades status truthfully to `degraded_continuing` through the ordinary `cluster-proof` status rail.**

## What Happened

S02 moved replica-backed durability from proof-app convention into the runtime continuity boundary. In `mesh-rt`, keyed submit is now explicitly durability-aware instead of inferring policy from whether a replica node happened to be present. The runtime prepares and acknowledges replica state before admitting replica-required work, persists stable rejected records when that safety cannot be established, and keeps merge precedence monotonic so older mirrored data cannot overwrite later rejected or degraded truth. Disconnect handling also changed: if a record was already mirrored and the replica disappears while work is still pending, the surviving owner record is downgraded to `degraded_continuing` instead of pretending it is still mirrored.

The compiler/runtime seam and the proof app were then aligned to that runtime truth. `Continuity.submit(...)` now carries an explicit replica requirement, the codegen/typecheck/intrinsic path lowers that requirement into the runtime ABI, and `cluster-proof/work.mpl` passes `required_replica_count(current_durability_policy())` instead of manufacturing safety from placement shape alone. The live `/work` path stopped synthesizing replica acknowledgement in Mesh code and now maps runtime-owned outcomes directly: accepted created/duplicate records stay `ok=true`, durable rejected records return stored failure truth without dispatching work, and conflicting same-key reuse preserves the existing 409 contract.

Because the older full S01 two-node verifier is still polluted by the unrelated remote-owner completion crash, S02 also established its own honest proof rail. `compiler/meshc/tests/e2e_m042_s02.rs` covers malformed response archiving, single-node replica-required rejection replay, two-node local-owner mirrored status truth, and post-loss degradation. `scripts/verify-m042-s02.sh` replays only the stable prerequisites plus the named S02 cases, fail-closes on missing test-count evidence or artifact bundles, and preserves the copied JSON/log evidence under `.tmp/m042-s02/verify/` for downstream debugging.

## Verification

Passed the full slice verification surface:

- `cargo test -p mesh-rt continuity -- --nocapture` — passed (`13 passed` continuity-focused runtime tests).
- `cargo run -q -p meshc -- test cluster-proof/tests` — passed (config + keyed-work contract coverage, including rejected duplicate replay and mirrored/rejected response mapping).
- `cargo test -p meshc --test e2e_m042_s02 -- --nocapture` — passed (`4 passed` covering malformed response archiving, single-node rejection, two-node mirrored status, and degraded status after replica loss).
- `bash scripts/verify-m042-s02.sh` — passed (`verify-m042-s02: ok`). The verifier replayed the stable prerequisite rails, confirmed each named phase ran, and preserved copied artifact bundles under `.tmp/m042-s02/verify/05-rejection-artifacts/`, `06-mirrored-artifacts/`, and `07-degraded-artifacts/`.

Operational/diagnostic surfaces were also confirmed from the preserved evidence:
- Rejected submit truth in `.tmp/m042-s02/verify/05-rejection-artifacts/.../rejected-submit.json` (`ok=false`, `phase="rejected"`, `result="rejected"`, `replica_status="rejected"`, `error="replica_required_unavailable"`).
- Mirrored pending truth in `.tmp/m042-s02/verify/06-mirrored-artifacts/.../pending-owner-status.json` (`ok=true`, `result="pending"`, `replica_status="mirrored"`).
- Degraded pending truth in `.tmp/m042-s02/verify/07-degraded-artifacts/.../degraded-owner-status.json` (`ok=true`, `result="pending"`, `replica_status="degraded_continuing"`).
- Replica-loss reason in `.tmp/m042-s02/verify/07-degraded-artifacts/.../node-a.stderr.log` (`[mesh-rt continuity] transition=degraded ... reason=replica_lost:<node>`).

## Requirements Advanced

- R049 — Strengthened the runtime-owned keyed submit/status contract so same-key retries now replay durable rejected truth as well as accepted duplicate truth without redispatch, which is the fail-closed admission half of the at-least-once continuity story.
- R050 — Added replica-backed admission, mirrored pending replication truth, and post-loss `degraded_continuing` downgrade behavior on the live two-node status rail, advancing the default two-node safety proof from app-authored acknowledgement toward runtime-owned durability.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

None.

## Known Limitations

The unrelated two-node remote-owner completion path is still blocked by the existing remote `Node.spawn` string-argument/runtime crash after mirrored submission, so S02 truth is intentionally proven on the stable local-owner rail plus runtime-owned status surfaces rather than on end-to-end remote execution completion. Also, replica-loss reason remains a runtime stderr diagnostic today; the ordinary `/work/:request_key` JSON status surface reports `replica_status="degraded_continuing"` but does not include the loss reason text.

## Follow-ups

S03 should consume this degraded/rejected foundation to prove owner-loss recovery from surviving continuity state, same-key retry with rolled `attempt_id`, and stale-completion rejection. If S03 needs to widen proof beyond the stable local-owner rail, it should first retire the remote `Node.spawn` string-argument/runtime crash rather than treating that failure as a continuity regression.

## Files Created/Modified

- `compiler/mesh-rt/src/dist/continuity.rs` — Added durability-aware submit decisions, persistent rejected-record handling, and safer continuity merge precedence for mirrored vs rejected/degraded truth.
- `compiler/mesh-rt/src/dist/node.rs` — Implemented replica prepare/ack-or-reject coordination and disconnect-triggered downgrade from mirrored to `degraded_continuing`.
- `compiler/mesh-rt/src/lib.rs` — Exported the durability-aware continuity runtime surface to the compiler-generated ABI.
- `compiler/mesh-typeck/src/infer.rs` — Updated the Mesh-facing `Continuity.submit(...)` typing to carry the explicit replica requirement.
- `compiler/mesh-codegen/src/codegen/intrinsics.rs` — Adjusted intrinsic declarations so codegen targets the durability-aware continuity runtime entrypoint.
- `compiler/mesh-codegen/src/mir/lower.rs` — Lowered the new replica-requirement argument from Mesh code into the runtime intrinsic call.
- `cluster-proof/work.mpl` — Removed live app-authored replica acknowledgement, passed runtime durability requirements, and mapped rejected/duplicate/degraded continuity truth directly into HTTP responses.
- `cluster-proof/tests/work.test.mpl` — Expanded helper-level coverage for durable rejection replay, duplicate truth, and policy-derived replica counts.
- `compiler/meshc/tests/e2e_m042_s02.rs` — Added the slice-specific rejection/mirroring/degradation end-to-end harness and artifact archiving.
- `scripts/verify-m042-s02.sh` — Added the canonical S02 verifier wrapper with stable prerequisite replay, fail-closed test-count checks, and copied artifact validation.
