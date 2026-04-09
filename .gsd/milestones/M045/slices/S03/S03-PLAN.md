# S03: Tiny Example Failover Truth

**Goal:** Prove failover on the same tiny scaffold-first clustered example: two local nodes form the cluster, a request reaches runtime-confirmed primary-owned mirrored pending state, primary loss promotes the standby, runtime-owned recovery completes on the standby, and same-identity rejoin fences the stale primary without adding app-owned failover or status logic.
**Demo:** After this: After this: the same tiny example survives primary loss and reports failover/status truth from the runtime without app-owned authority or failover choreography.

## Tasks
- [x] **T01: Added the M045/S03 scaffold failover e2e harness with retained runtime diagnostics, but the generated scaffold still fails the primary-owned pending-window verification.** — Finish the core R078 proof on the scaffold-first example by creating a dedicated destructive failover e2e that keeps cluster truth runtime-owned. The harness should use the scaffolded binary, not `cluster-proof`, and it should confirm actual placement from `meshc cluster continuity --json` before taking destructive action so local placement prediction cannot make the rail go green for the wrong reason.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Two-node scaffold runtime plus `meshc cluster status|continuity|diagnostics` surfaces | Stop with retained HTTP, CLI, and node-log artifacts; do not infer failover success from startup logs alone. | Bound membership/pending-state/promotion/rejoin polling and fail with the last observed payload. | Treat malformed CLI JSON as a hard proof failure, not a retryable warning. |
| Pending mirrored owner-loss timing window in the scaffold work path | Keep the app surface unchanged and fail with retained pre-kill artifacts showing whether records completed too early. | Use bounded candidate search and, if needed, bounded batching before declaring the harness nondeterministic. | Treat missing or contradictory pre-kill record fields as a proof failure rather than assuming the right placement. |

## Load Profile

- **Shared resources**: temporary scaffold dirs, local ports, spawned node processes, CLI subprocesses, and `.tmp/m045-s03` artifact roots.
- **Per-operation cost**: one scaffold init/build, two node boots, repeated CLI polls, one destructive kill/rejoin cycle, and one retained artifact bundle.
- **10x breakpoint**: process cleanup, pending-window timing, and artifact churn fail before runtime throughput; the harness must capture enough state to explain which stage drifted.

## Negative Tests

- **Malformed inputs**: request keys that never produce the required pre-kill owner/replica shape and malformed JSON from runtime CLI surfaces.
- **Error paths**: primary dies after the record already completed, standby never promotes, automatic recovery never rolls the attempt, or the stale primary executes after rejoin.
- **Boundary conditions**: `replica_status=preparing` vs `mirrored` before kill, single-submit vs bounded candidate batching, and same-identity rejoin after standby promotion.

## Steps

1. Create `compiler/meshc/tests/e2e_m045_s03.rs` with scaffold init/build/spawn helpers adapted from `compiler/meshc/tests/e2e_m045_s02.rs`, artifact retention under `.tmp/m045-s03/...`, and a two-node cluster setup that runs one scaffold binary as primary and one as standby.
2. Drive request-key candidates until runtime CLI truth shows a pre-kill record with `owner_node=primary`, `replica_node=standby`, `phase=submitted`, `result=pending`, and `replica_status` still `mirrored` or `preparing`; if a single submit races completion, use bounded candidate batching while keeping the app surface unchanged.
3. Kill the primary, prove standby promotion/recovery and same-identity rejoin fencing through `meshc cluster status|continuity|diagnostics --json`, and retain pre-kill/post-kill/post-rejoin JSON plus node logs and `scenario-meta.json`.

## Must-Haves

- [ ] The new scaffold failover rail proves the owner-loss shape before the destructive step instead of trusting a local heuristic.
- [ ] Promotion, recovery, and fenced rejoin are all asserted from runtime-owned CLI truth plus node logs.
- [ ] The retained bundle is sufficient to debug whether drift happened before kill, during promotion/recovery, or during rejoin.

## Verification

- `cargo test -p meshc --test e2e_m045_s03 m045_s03_failover_ -- --nocapture`
  - Estimate: 4h
  - Files: compiler/meshc/tests/e2e_m045_s03.rs, compiler/meshc/tests/e2e_m045_s02.rs, compiler/meshc/src/cluster.rs, compiler/mesh-pkg/src/scaffold.rs
  - Verify: cargo test -p meshc --test e2e_m045_s03 m045_s03_failover_ -- --nocapture
- [x] **T02: Captured the current M045/S03 scaffold failover drift and left retained runtime evidence for the next fix attempt.** — Keep the scaffold docs-grade and runtime-owned while S03 adds destructive failover proof. This task hardens the generated app and README rails against regression and reserves any timing stabilization to the smallest runtime or codegen seam if T01 proves harness-only observation is not deterministic.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Clustered scaffold generator and README contract in `compiler/mesh-pkg/src/scaffold.rs` | Fail source-contract tests loudly if the generated app grows app-owned failover, promotion, or status logic. | N/A — generation is synchronous and bounded. | Reject stale README/source assertions that would let proof-app-specific or manual failover text slip back in. |
| Optional runtime/codegen timing seam near declared-work completion | Keep the change below the app surface and fail tests if it leaks a scaffold-specific env knob or manual control path. | Bound any new wait/poll expectations in tests so timing drift produces evidence instead of hangs. | Treat mismatched attempt/phase truth as a runtime contract failure, not as a scaffold excuse. |

## Load Profile

- **Shared resources**: scaffold templates, temp-project init/build rails, and any runtime completion path touched for timing stabilization.
- **Per-operation cost**: one scaffold generation plus focused contract/e2e replays.
- **10x breakpoint**: stale source assertions and accidental app-surface growth fail before performance does; the task must pin the public surface tightly.

## Negative Tests

- **Malformed inputs**: generated `main.mpl` or `README.md` reintroducing `/promote`, `Continuity.promote`, `Continuity.mark_completed`, app-owned status helpers, `owner_node`/`replica_node` surfacing, or a scaffold-specific delay knob.
- **Error paths**: T01 shows the pending window is nondeterministic and the fallback stabilization leaks into the scaffold source shape or CLI contract.
- **Boundary conditions**: the scaffold still stays tiny after adding failover proof, and the README keeps `meshc cluster diagnostics` alongside the existing status/continuity contract.

## Steps

1. Tighten `compiler/mesh-pkg/src/scaffold.rs`, `compiler/meshc/tests/tooling_e2e.rs`, and scaffold contract assertions so generated clustered apps still expose only `Node.start_from_env()`, `/health`, the submit route, and runtime CLI surfaces including `meshc cluster diagnostics`.
2. Add explicit failover-absence assertions in the scaffold contract rails (no `/promote`, no `Continuity.promote`, no app-owned status/authority helpers, no `CLUSTER_PROOF_*`, and no app-specific delay knob), preferably with a named `m045_s03_scaffold_` assertion surface in `compiler/meshc/tests/e2e_m045_s03.rs`.
3. Only if T01's retained evidence shows harness-only timing is not deterministic, add the smallest runtime-owned pending-window seam beneath the app surface and extend the contract tests to prove the scaffold source shape does not grow.

## Must-Haves

- [ ] The generated scaffold remains visibly small and runtime-owned while S03 lands.
- [ ] README and contract rails explicitly protect `meshc cluster diagnostics` and the absence of manual failover surfaces.
- [ ] Any timing stabilization happens below the app surface and is proven not to leak example-owned distributed logic back into the scaffold.

## Verification

- `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`
- `cargo test -p meshc --test e2e_m045_s02 m045_s02_scaffold_runtime_completion_ -- --nocapture`
- `cargo test -p meshc --test e2e_m045_s03 m045_s03_scaffold_ -- --nocapture`
  - Estimate: 3h
  - Files: compiler/mesh-pkg/src/scaffold.rs, compiler/meshc/tests/tooling_e2e.rs, compiler/meshc/tests/e2e_m045_s02.rs, compiler/meshc/tests/e2e_m045_s03.rs, compiler/mesh-codegen/src/codegen/expr.rs, compiler/mesh-rt/src/dist/node.rs
  - Verify: cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture
cargo test -p meshc --test e2e_m045_s02 m045_s02_scaffold_runtime_completion_ -- --nocapture
cargo test -p meshc --test e2e_m045_s03 m045_s03_scaffold_ -- --nocapture
- [x] **T03: Narrowed the S03 scaffold failover harness and captured the remaining red runtime drift.** — Finish slice closeout with an authoritative, flattened verifier that replays the prerequisite runtime rails, the scaffold contract rails, and the new S03 e2e, then copies a fresh proof bundle into `.tmp/m045-s03/verify/` and fail-closes on zero-test, malformed pointer, or missing artifact shape.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Upstream runtime/scaffold regression commands replayed by `scripts/verify-m045-s03.sh` | Stop on the first red prerequisite and keep per-phase logs; do not continue to the S03 rail with stale assumptions. | Bound every command and fail with the captured log instead of hanging the verifier. | Treat missing `running N test` lines or malformed command output as verifier drift, not success. |
| Fresh `.tmp/m045-s03` artifact discovery and copy logic | Fail closed if no fresh bundle is produced or if the pointer/manifest targets the wrong directory. | N/A — copy/manifest checks are local and synchronous. | Reject missing `scenario-meta.json`, required JSON evidence files, or missing node logs rather than claiming the failover proof was retained. |

## Load Profile

- **Shared resources**: cargo test output, `.tmp/m045-s03` artifact roots, copied verifier bundles, and per-phase logs under `.tmp/m045-s03/verify/`.
- **Per-operation cost**: replay of focused runtime rails plus one full `e2e_m045_s03` run and artifact-copy validation.
- **10x breakpoint**: stale artifact roots and long-running test replays fail before throughput does; the verifier must make freshness and bundle shape explicit.

## Negative Tests

- **Malformed inputs**: zero-test filters, malformed `latest-proof-bundle.txt`, missing `scenario-meta.json`, and missing pre-kill/post-kill/post-rejoin evidence files.
- **Error paths**: prerequisite green rail goes red, the S03 e2e never emits a fresh artifact directory, or the copied bundle points at the wrong source.
- **Boundary conditions**: multiple old `.tmp/m045-s03` directories exist, the verifier still selects only the fresh replay output, and phase files stay truthful on early failure.

## Steps

1. Add `scripts/verify-m045-s03.sh` with phase/state files and direct replays of the focused prerequisites: runtime auto-promotion/recovery rails, the protected M044 S04 failover rail, the scaffold init contract, the S02 runtime-completion rail, and the new S03 e2e.
2. Snapshot and copy the fresh `.tmp/m045-s03` artifact directories into `.tmp/m045-s03/verify/retained-m045-s03-artifacts/`, then assert the retained bundle contains `scenario-meta.json`, pre-kill continuity/status JSON, post-promotion diagnostics/continuity JSON, post-rejoin status JSON, and node stdout/stderr logs.
3. Fail closed on zero-test drift, missing bundle freshness, malformed pointer files, or incomplete retained evidence so later slices can trust the verifier output without rerunning the cluster immediately.

## Must-Haves

- [ ] `scripts/verify-m045-s03.sh` is the authoritative local stopping condition for S03.
- [ ] The verifier retains one fresh failover bundle and checks its shape before returning green.
- [ ] Early failures leave enough per-phase output behind to distinguish prerequisite regressions from S03-only proof drift.

## Verification

- `bash scripts/verify-m045-s03.sh`
  - Estimate: 2h
  - Files: scripts/verify-m045-s03.sh, scripts/verify-m045-s02.sh, compiler/meshc/tests/e2e_m045_s03.rs, compiler/meshc/tests/e2e_m044_s04.rs
  - Verify: bash scripts/verify-m045-s03.sh
