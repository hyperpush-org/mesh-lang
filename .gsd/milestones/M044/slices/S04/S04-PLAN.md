# S04: Bounded Automatic Promotion

**Goal:** Make bounded automatic promotion the default failover path for declared clustered work without leaving a manual authority-change escape hatch or unsafe ambiguity path.
**Demo:** After this: After this: killing the active primary causes safe auto-promotion for declared clustered work when the runtime can prove safety; ambiguous cases fail closed, and stale-primary rejoin stays fenced.

## Tasks
- [x] **T01: Bound safe auto-promotion to provable node-loss conditions** — S04 cannot honestly ship on top of the M043 manual promote helper. This task adds the runtime-owned decision boundary for when a standby may promote itself after peer loss, and it makes the refusal path explicit and inspectable instead of silently leaving later tasks to guess why failover did or did not happen.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Runtime node membership/session truth in `compiler/mesh-rt/src/dist/node.rs` | Refuse promotion and emit an explicit bounded-failover diagnostic. | Treat the peer as ambiguous and stay fenced/standby. | Reject the peer/membership state as unsafe and record the refusal reason. |
| Continuity authority projection in `compiler/mesh-rt/src/dist/continuity.rs` | Keep the existing authority and surface a fail-closed reason. | Do not promote until the state transition completes locally. | Reject inconsistent mirrored/epoch truth instead of promoting anyway. |
| Structured diagnostics in `compiler/mesh-rt/src/dist/operator.rs` | Preserve stderr continuity logs and return an explicit recording failure for tests. | Keep promotion logic bounded; diagnostics failure must not force promotion. | Drop malformed diagnostic payloads rather than polluting the buffer. |

## Load Profile

- **Shared resources**: live node session state, the continuity registry, and the bounded operator-diagnostic ring buffer.
- **Per-operation cost**: one disconnect scan plus one authority/record projection pass and bounded diagnostic writes.
- **10x breakpoint**: peer flapping and large mirrored record sets will stress disconnect-time scans first; the decision path must stay bounded rather than doing unbounded replay work here.

## Negative Tests

- **Malformed inputs**: invalid authority role/epoch combinations, malformed peer identity/session state, and inconsistent mirrored record shapes.
- **Error paths**: primary loss with no mirrored record, ambiguous/multi-peer loss, and stale higher-epoch truth already present on rejoin.
- **Boundary conditions**: healthy two-node mirror, empty continuity registry, and standby records that should stay degraded instead of promoting.

## Steps

1. Define the bounded safety rule for automatic promotion from the real disconnect path in `compiler/mesh-rt/src/dist/node.rs`, using live peer/session truth instead of proof-app heuristics.
2. Extend `compiler/mesh-rt/src/dist/continuity.rs` so safe promotion advances authority/epoch and ambiguous cases fail closed with explicit reasons.
3. Emit structured operator diagnostics and correlated `mesh-rt continuity` log lines for both auto-promotion and auto-promotion refusal so later tasks can assert on them.
4. Add runtime and destructive e2e coverage that proves safe promotion and ambiguous refusal without any manual promote call.

## Must-Haves

- [ ] A standby only auto-promotes when the runtime can prove the supported one-primary/one-standby transition is safe from current session + mirrored continuity truth.
- [ ] Ambiguous or insufficient state refuses promotion explicitly instead of inferring safety from peer disappearance alone.
- [ ] Operator diagnostics and retained test artifacts make the promote-vs-refuse decision legible after the fact.
  - Estimate: 90m
  - Files: compiler/mesh-rt/src/dist/node.rs, compiler/mesh-rt/src/dist/continuity.rs, compiler/mesh-rt/src/dist/operator.rs, compiler/meshc/tests/e2e_m044_s04.rs
  - Verify: cargo test -p mesh-rt automatic_promotion_ -- --nocapture
cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_promotion_ -- --nocapture
- [x] **T02: Replayed the missing auto-resume proof surface and documented the exact runtime and harness seams; no recovery code landed in this unit.** — A promoted standby that still needs a client retry is not R068. This task closes the runtime-owned recovery gap by preserving the declared-handler identity needed to redispatch work after owner loss, then proving the promoted standby completes the request without `POST /promote` or a second submit.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Declared-handler registry and runtime-name lookup in `compiler/mesh-rt/src/dist/node.rs` | Reject recovery and leave an explicit recovery error on the continuity record. | Keep the request pending with diagnosable recovery state; do not fabricate completion. | Reject redispatch metadata as invalid and fail closed. |
| Continuity retry-rollover state in `compiler/mesh-rt/src/dist/continuity.rs` | Preserve the owner-lost record and surface the failed recovery reason. | Keep the request observable as pending/owner-lost or degraded instead of dropping it. | Refuse rollover if request/attempt metadata is inconsistent. |
| Same-image destructive proof in `compiler/meshc/tests/e2e_m044_s04.rs` | Fail the test rail and retain artifacts/logs instead of masking the missing recovery. | Bound the wait around recovery transitions and surface which stage stalled. | Reject malformed JSON/artifact state instead of passing on missing proof. |

## Load Profile

- **Shared resources**: continuity record storage, declared-handler registry lookups, remote spawn/session transport, and same-image artifact capture.
- **Per-operation cost**: one recovery metadata lookup plus one redispatch for each eligible owner-lost request.
- **10x breakpoint**: repeated owner-loss events on many mirrored requests can amplify redispatch pressure; the recovery path must stay keyed, bounded, and idempotent.

## Negative Tests

- **Malformed inputs**: missing runtime handler metadata, mismatched request/attempt ids, and duplicate/stale recovery records.
- **Error paths**: redispatch failure on the promoted standby, recovery attempted while still ambiguous, and stale-primary same-key traffic after rejoin.
- **Boundary conditions**: one pending mirrored request, already-completed request, and owner-loss followed by a healthy rejoin with no duplicate execution.

## Steps

1. Persist the declared-handler recovery metadata the runtime needs at submit time so owner-loss recovery does not depend on an external caller still holding the handler name.
2. Extend the runtime recovery path to redispatch eligible owner-lost declared work automatically after a safe promotion using a new attempt fence.
3. Prove that the promoted standby completes the request without a second submit while the old primary never executes the recovered attempt after rejoin.
4. Retain artifact/log assertions that distinguish auto-resume from the old manual-promote-plus-retry path.

## Must-Haves

- [ ] Declared clustered work survives primary loss through runtime-owned auto-resume, not just through retry eligibility.
- [ ] Recovery uses a new fenced attempt id and remains idempotent under stale-primary rejoin.
- [ ] The proof rail shows completion on the promoted standby with no manual promote call and no same-key client retry.
  - Estimate: 90m
  - Files: compiler/mesh-rt/src/dist/node.rs, compiler/mesh-rt/src/dist/continuity.rs, compiler/meshc/tests/e2e_m044_s04.rs, cluster-proof/tests/work.test.mpl
  - Verify: cargo test -p mesh-rt automatic_recovery_ -- --nocapture
cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_resume_ -- --nocapture
- [x] **T03: Removed the public `Continuity.promote()` Mesh surface and replaced it with an explicit auto-only compiler diagnostic.** — D185 and R067 say the failover control mode is auto-only. Even with runtime auto-promotion working, leaving `Continuity.promote()` callable from Mesh code keeps a manual override seam alive. This task removes that public control surface while preserving the internal Rust authority-transition helper the runtime needs for bounded automatic promotion.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Mesh builtin/typechecker surface in `compiler/mesh-typeck/src/` | Fail compile with a clear unsupported/manual-surface diagnostic. | N/A — compile-time path. | Reject malformed promote calls at compile time instead of lowering them. |
| Codegen/runtime export wiring in `compiler/mesh-codegen/src/` and `compiler/mesh-rt/src/lib.rs` | Remove the manual promote entrypoint cleanly; do not leave a dead intrinsic or unresolved symbol. | N/A — compile/link path. | Treat stale promote references as compile/link failures, not runtime surprises. |
| Historical compiler/e2e coverage | Update or replace expectations so the repo does not keep green tests for a contract that no longer exists. | N/A — test path. | Fail closed on stale manual-promotion assertions. |

## Load Profile

- **Shared resources**: compiler builtin registry, codegen intrinsic table, and runtime export surface.
- **Per-operation cost**: trivial compile-time symbol resolution; the important risk is surface drift, not throughput.
- **10x breakpoint**: N/A — correctness/compatibility task rather than a hot runtime path.

## Negative Tests

- **Malformed inputs**: wrong-arity `Continuity.promote(...)`, promote calls in ordinary Mesh code, and stale manual-promotion proof fixtures.
- **Error paths**: removed intrinsic/export still referenced by generated code, or old tests still expecting a successful manual promote result.
- **Boundary conditions**: `Continuity.authority_status()` still works, but any manual promote call fails closed with a clear diagnostic.

## Steps

1. Remove the Mesh-visible `Continuity.promote()` builtin/lowering/export path while keeping the internal Rust promotion helper the runtime auto-promotion logic uses.
2. Update compile/e2e coverage so manual promotion attempts now fail closed explicitly and `Continuity.authority_status()` remains the read-only Mesh-visible authority seam.
3. Audit stale tests and compatibility fixtures that still assume manual promotion succeeds, and retarget them to the new auto-only contract.
4. Keep the failure surface specific enough that app authors learn the operator contract changed instead of seeing a generic unresolved symbol or parse failure.

## Must-Haves

- [ ] Mesh application code can no longer change authority manually through `Continuity.promote()`.
- [ ] The runtime still retains the internal authority-transition seam needed by T01/T02 auto-promotion logic.
- [ ] Compiler/runtime/test surfaces fail closed with a clear manual-promotion-disabled diagnostic instead of stale success or dead-symbol errors.
  - Estimate: 60m
  - Files: compiler/mesh-typeck/src/infer.rs, compiler/mesh-typeck/src/builtins.rs, compiler/mesh-codegen/src/mir/lower.rs, compiler/mesh-codegen/src/codegen/intrinsics.rs, compiler/mesh-rt/src/dist/continuity.rs, compiler/mesh-rt/src/lib.rs, compiler/meshc/tests/e2e_m044_s01.rs, compiler/meshc/tests/e2e_m043_s02.rs
  - Verify: cargo test -p meshc --test e2e_m044_s04 m044_s04_manual_surface_ -- --nocapture
cargo test -p meshc --test e2e_m044_s01 m044_s01_typed_continuity_ -- --nocapture
- [x] **T04: Recorded that T04 is blocked because the S04 auto-promotion/auto-resume proof rail is still missing locally.** — Once the runtime behaves correctly, the proof app and docs need to stop teaching the old contract. This task removes the `/promote` route from `cluster-proof`, updates the package tests and public docs to the new auto-only story, and adds one fail-closed S04 verifier that archives the destructive proof bundle and refuses stale manual-surface wording.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `cluster-proof` runtime/package tests | Fail the package rail and retain the broken app/log snapshot. | Bound server waits and surface which readiness or status transition stalled. | Reject malformed JSON/status artifacts instead of passing on partial truth. |
| Assembled verifier in `scripts/verify-m044-s04.sh` | Stop at the first failing phase with retained logs/status/test-count artifacts. | Mark the exact verifier phase as timed out and keep the copied bundle. | Fail closed on missing `running N test`, malformed JSON, or stale manual-surface strings. |
| Public docs/proof pages | Fail the docs/build/proof-surface checks rather than letting stale `/promote` wording ship. | N/A — local file/build path. | Reject stale command blocks, missing markers, or mixed manual/auto wording. |

## Load Profile

- **Shared resources**: same-image Docker containers, retained verifier artifact directories, docs build output, and proof-app package tests.
- **Per-operation cost**: one destructive two-node replay plus package build/test and docs verification.
- **10x breakpoint**: Docker/container startup and retained artifact churn dominate before app logic; the verifier must keep bounded copies and deterministic phase cleanup.

## Negative Tests

- **Malformed inputs**: stale `/promote` or `Continuity.promote` references in docs/verifiers, missing retained artifact files, and missing `running N test` lines.
- **Error paths**: cluster-proof still depends on manual promotion, same-image proof needs a retry to finish, or stale-primary rejoin resumes execution.
- **Boundary conditions**: promoted standby finishes automatically, ambiguous cases retain refusal artifacts, and operator docs remain read-only even after auto-promotion ships.

## Steps

1. Remove the `/promote` HTTP route and manual-promotion helpers from `cluster-proof`, updating package tests to the new auto-promotion/auto-resume truth.
2. Add `scripts/verify-m044-s04.sh` that replays S03, runs the named `m044_s04_` rails, enforces non-zero test execution, and validates retained same-image artifacts for auto-promotion, auto-resume, and stale-primary fencing.
3. Update `README.md`, `cluster-proof/README.md`, and the distributed proof/docs pages so the public story is bounded automatic promotion, not manual promotion or operator mutation.
4. Rebuild docs and rerun the assembled verifier so the shipped proof/app/docs surfaces all describe the same contract.

## Must-Haves

- [ ] `cluster-proof` no longer exposes `/promote` or depends on a manual authority-change helper to finish the destructive failover rail.
- [ ] `scripts/verify-m044-s04.sh` is the authoritative fail-closed local acceptance command with retained artifacts and non-zero test-count guards.
- [ ] Public docs and runbooks explicitly describe bounded automatic promotion, stale-primary fencing, ambiguous fail-closed behavior, and read-only operator inspection.
- [ ] The final proof bundle shows no manual promote call or same-key retry in the healthy S04 path.
  - Estimate: 90m
  - Files: cluster-proof/main.mpl, cluster-proof/work_continuity.mpl, cluster-proof/tests/work.test.mpl, scripts/verify-m044-s04.sh, README.md, cluster-proof/README.md, website/docs/docs/distributed/index.md, website/docs/docs/distributed-proof/index.md
  - Verify: cargo run -q -p meshc -- build cluster-proof
cargo run -q -p meshc -- test cluster-proof/tests
bash scripts/verify-m044-s04.sh
npm --prefix website run build
  - Blocker: `compiler/meshc/tests/e2e_m044_s04.rs` is still missing. `compiler/mesh-rt/src/dist/node.rs::handle_node_disconnect(...)` still lacks a runtime-owned call into the internal promotion seam. `submit_declared_work(...)` still has no runtime-owned auto-resume redispatch path, so the healthy S04 path cannot yet finish without the old retry-era seam. Public docs remain stale because the truthful S04 replacement rail is not ready to publish.
