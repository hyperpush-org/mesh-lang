# S04: Remove Legacy Example-Side Cluster Logic

**Goal:** Remove the remaining cluster-proof-era example-owned clustering seams so the primary clustered story stays scaffold-first and runtime-owned: dead placement logic is gone, declared work no longer lives inside the continuity wrapper, and the repo’s current docs/verifier contract points at M045 rails instead of the older M044 cleanup story.
**Demo:** After this: After this: old `cluster-proof`-style placement/config/status glue is gone or deeply collapsed, and the repo no longer teaches example-owned distributed mechanics as the primary story.

## Tasks
- [x] **T01: Removed dead cluster-proof placement helpers and refocused package tests on live membership/config seams.** — Remove the dead deterministic placement engine from `cluster-proof/cluster.mpl` and make the package tests prove only the live membership/config seams that still matter for the proof package.

## Steps

1. Delete `CanonicalPlacement` and the unused placement/canonical-owner helpers from `cluster-proof/cluster.mpl` while keeping `canonical_membership(...)`, `membership_snapshot()`, and `membership_payload(...)` behavior stable.
2. Rewrite `cluster-proof/tests/work.test.mpl` and `cluster-proof/tests/config.test.mpl` so they assert current package truths (membership payload authority fields, durability/topology validation, request parsing/status payloads) instead of helper-shaped or dead-placement behavior.
3. Re-run the package build/tests and fail closed if membership payload shape or topology validation drifts.

## Must-Haves

- [ ] No deterministic owner/replica placement helpers remain in `cluster-proof/cluster.mpl`.
- [ ] Package tests still prove membership payload truth and continuity topology validation through live public seams.
- [ ] `cluster-proof` still builds and its package tests stay green after the delete.
  - Estimate: 2h
  - Files: cluster-proof/cluster.mpl, cluster-proof/tests/work.test.mpl, cluster-proof/tests/config.test.mpl
  - Verify: cargo run -q -p meshc -- build cluster-proof
cargo run -q -p meshc -- test cluster-proof/tests
- [x] **T02: Moved cluster-proof declared work into `Work` and dropped wrapper-side completion glue.** — Align `cluster-proof` with the scaffold-first runtime-owned execution shape: the manifest-declared handler should live in `Work`, while `work_continuity` stays a thin HTTP/status translator over runtime `Continuity`.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Runtime-owned declared-work completion on the `cluster-proof` manifest target | Stop with build/test or failover-rail evidence; do not reintroduce `Continuity.mark_completed(...)` as a fallback. | Bound the destructive failover replay and retain the last continuity/log outputs. | Treat contradictory continuity/status payloads as real runtime drift, not as a reason to keep the wrapper seam. |
| Manifest/codegen registration for `Work.execute_declared_work` | Fail closed on missing-handler build/runtime errors; do not leave the manifest pointing at `WorkContinuity.execute_declared_work`. | N/A — registration drift should surface during build or targeted runtime proof. | Treat malformed handler/record truth as a contract failure, not a retryable warning. |

## Load Profile

- **Shared resources**: `cluster-proof` build output, local ports, node processes, existing same-image failover rails, and retained `.tmp` evidence.
- **Per-operation cost**: one package build/test replay plus one destructive runtime failover replay.
- **10x breakpoint**: runtime promotion/recovery timing and stale handler registration fail before throughput; the task must keep enough logs/continuity output to explain which seam drifted.

## Negative Tests

- **Malformed inputs**: missing declared-handler registration, malformed continuity/status JSON, and stale manifest target strings.
- **Error paths**: declared work returns but the record never completes through the runtime path, or automatic promotion/recovery regresses after the target move.
- **Boundary conditions**: duplicate submit, owner-loss recovery, and stale-primary rejoin still behave the same with the slimmer target shape.

## Steps

1. Move the manifest target from `WorkContinuity.execute_declared_work` to `Work.execute_declared_work`, implement the declared handler in `cluster-proof/work.mpl`, and keep only package-local execution behavior there (including any retained proof-only delay/logging the destructive rail still needs).
2. Remove manual completion and wrapper-era leftovers from `cluster-proof/work_continuity.mpl` (`Continuity.mark_completed(...)`, completion-failure logging, dead actor execution path, and the old target helper) so that file only owns keyed submit/status HTTP translation.
3. Update package/runtime assertions so the slimmer target shape still satisfies the existing same-image failover proof.

## Must-Haves

- [ ] `cluster-proof/mesh.toml` declares `Work.execute_declared_work`, not `WorkContinuity.execute_declared_work`.
- [ ] `cluster-proof/work_continuity.mpl` no longer manually closes continuity records or owns the actual declared-work handler.
- [ ] The existing same-image failover rail still passes on the runtime-owned completion path.
  - Estimate: 4h
  - Files: cluster-proof/mesh.toml, cluster-proof/work.mpl, cluster-proof/work_continuity.mpl, cluster-proof/tests/work.test.mpl, compiler/meshc/tests/e2e_m044_s04.rs
  - Verify: cargo run -q -p meshc -- build cluster-proof
cargo run -q -p meshc -- test cluster-proof/tests
cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_promotion_ -- --nocapture
- [x] **T03: Promoted M045 S04 to the current distributed closeout rail and rewired docs/verifiers away from the stale M044 story.** — Make M045 the current public cleanup/closeout story so the repo teaches the scaffold-first clustered path first and treats `cluster-proof` as the deeper proof consumer instead of the primary clustered abstraction.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Source/docs contract tests and public pages | Fail with exact file-path assertions; do not leave the repo pointing at `verify-m044-s05.sh` as the current clustered closeout rail. | N/A — these are local source assertions. | Treat missing or contradictory marker text as a contract failure, not documentation drift to ignore. |
| Assembled verifier replay plus retained proof-bundle handoff | Stop on the first red prerequisite and keep per-phase logs plus copied bundle pointers. | Bound every replayed command and fail with the captured phase log instead of hanging. | Reject zero-test filters, malformed pointer files, or missing retained bundle shape as verifier drift. |

## Load Profile

- **Shared resources**: docs pages, focused Rust e2e files, `.tmp/m045-s03/verify`, and the new `.tmp/m045-s04/verify` artifact root.
- **Per-operation cost**: one source/docs contract test, one assembled verifier replay, and one docs build.
- **10x breakpoint**: stale references and artifact-copy drift fail before throughput; the verifier must make freshness and bundle shape explicit.

## Negative Tests

- **Malformed inputs**: stale `verify-m044-s05.sh` references in public docs, zero-test `m045_s04_` filters, and malformed bundle pointers or copied manifests.
- **Error paths**: M045 S02/S03 rails go red during replay, docs still mention old example-owned mechanics, or the copied S03 proof bundle is missing required files.
- **Boundary conditions**: the repo may contain multiple old `.tmp/m045-s03` directories, but the new verifier must retain only the fresh prerequisite bundle it actually replayed.

## Steps

1. Add `compiler/meshc/tests/e2e_m045_s04.rs` to assert the new source/docs/verifier contract: the cleaned `cluster-proof` target shape is real, public pages point at `scripts/verify-m045-s04.sh`, and `scripts/verify-m045-s03.sh` is the failover-specific subrail.
2. Add `scripts/verify-m045-s04.sh` as the assembled cleanup verifier: replay the M045 scaffold happy-path and failover rails, run the new source/docs contract test plus `cluster-proof` build/tests and the docs build, and retain the fresh upstream proof bundle it depends on.
3. Update `README.md`, `cluster-proof/README.md`, the distributed docs pages, and the old M044 closeout test so M045 now owns the current public story while M044 stops policing present-tense docs wording.

## Must-Haves

- [ ] Public clustered docs/readmes point at M045 rails, not `verify-m044-s05.sh`, as the current closeout story.
- [ ] There is a named M045 source/docs/verifier contract test and assembled verifier for S04.
- [ ] Historical M044 closeout assertions no longer fail only because the primary story moved to M045.
  - Estimate: 3h
  - Files: compiler/meshc/tests/e2e_m045_s04.rs, compiler/meshc/tests/e2e_m044_s05.rs, scripts/verify-m045-s04.sh, README.md, cluster-proof/README.md, website/docs/docs/distributed/index.md, website/docs/docs/distributed-proof/index.md, website/docs/docs/tooling/index.md
  - Verify: cargo test -p meshc --test e2e_m045_s04 m045_s04_ -- --nocapture
bash scripts/verify-m045-s04.sh
