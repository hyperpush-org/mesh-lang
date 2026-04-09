---
id: T03
parent: S03
milestone: M046
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-rt/src/dist/node.rs", "compiler/meshc/tests/e2e_m046_s03.rs", "scripts/verify-m046-s03.sh", "tiny-cluster/work.mpl", "tiny-cluster/tests/work.test.mpl", "tiny-cluster/README.md", ".gsd/DECISIONS.md", ".gsd/KNOWLEDGE.md"]
key_decisions: ["D242: move the failover-only startup delay out of tiny-cluster package code and into mesh-rt as a runtime-owned `MESH_STARTUP_WORK_DELAY_MS` dispatch hook keyed to startup work.", "Choose a cluster port whose deterministic startup request hashes to the primary node before running the destructive failover proof, because owner placement depends on hashed node names."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the new runtime-owned startup delay hook with focused `mesh-rt` unit tests, rebuilt and reran the real `tiny-cluster/` package smoke rail, ran the focused S03 failover/rejoin e2e filter to green, and then closed the slice with `bash scripts/verify-m046-s03.sh`, which replayed the prerequisite startup/package commands and copied a fresh failover evidence bundle under `.tmp/m046-s03/verify/retained-m046-s03-artifacts`."
completed_at: 2026-03-31T20:58:28.090Z
blocker_discovered: false
---

# T03: Moved the tiny-cluster failover delay into mesh-rt, proved route-free promotion/recovery/fenced rejoin from Mesh CLI surfaces, and added `scripts/verify-m046-s03.sh` as the direct slice verifier.

> Moved the tiny-cluster failover delay into mesh-rt, proved route-free promotion/recovery/fenced rejoin from Mesh CLI surfaces, and added `scripts/verify-m046-s03.sh` as the direct slice verifier.

## What Happened
---
id: T03
parent: S03
milestone: M046
key_files:
  - compiler/mesh-rt/src/dist/node.rs
  - compiler/meshc/tests/e2e_m046_s03.rs
  - scripts/verify-m046-s03.sh
  - tiny-cluster/work.mpl
  - tiny-cluster/tests/work.test.mpl
  - tiny-cluster/README.md
  - .gsd/DECISIONS.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - D242: move the failover-only startup delay out of tiny-cluster package code and into mesh-rt as a runtime-owned `MESH_STARTUP_WORK_DELAY_MS` dispatch hook keyed to startup work.
  - Choose a cluster port whose deterministic startup request hashes to the primary node before running the destructive failover proof, because owner placement depends on hashed node names.
duration: ""
verification_result: passed
completed_at: 2026-03-31T20:58:28.092Z
blocker_discovered: false
---

# T03: Moved the tiny-cluster failover delay into mesh-rt, proved route-free promotion/recovery/fenced rejoin from Mesh CLI surfaces, and added `scripts/verify-m046-s03.sh` as the direct slice verifier.

**Moved the tiny-cluster failover delay into mesh-rt, proved route-free promotion/recovery/fenced rejoin from Mesh CLI surfaces, and added `scripts/verify-m046-s03.sh` as the direct slice verifier.**

## What Happened

Reworked the S03 proof seam to respect the user override that rejected package-owned failover timing. `tiny-cluster/work.mpl` is back to a plain source-first `clustered(work)` function that visibly returns `1 + 1`, and the package smoke/readme contract now fails if package code reintroduces `Env.get_int`, `Timer.sleep`, `TINY_CLUSTER_WORK_DELAY_MS`, or any package-owned control seam. The failover-only pending window moved into `compiler/mesh-rt/src/dist/node.rs` as a runtime-owned `MESH_STARTUP_WORK_DELAY_MS` dispatch hook that only applies to the runtime-owned startup request path, with pure normalization tests added alongside the existing startup runtime tests.

Extended `compiler/meshc/tests/e2e_m046_s03.rs` from the T02 startup rail into the destructive failover/rejoin proof. The new rail keeps the proof entirely on `meshc cluster status`, `meshc cluster continuity`, and `meshc cluster diagnostics`, adds bounded polling helpers plus negative helper tests, chooses a dual-stack cluster port whose deterministic startup request hashes to the primary owner, discovers the startup record from continuity list mode, kills the owner during the mirrored pending window, proves automatic promotion/recovery/completion on the standby, then restarts the stale primary and proves fenced rejoin plus post-rejoin continuity truth from the CLI surfaces. The rail retains scenario metadata, pre/post-kill JSON snapshots, and per-node stdout/stderr under `.tmp/m046-s03/tiny-cluster-failover-runtime-truth-*`.

Added `scripts/verify-m046-s03.sh` as the direct slice verifier. It replays `cargo build -q -p mesh-rt`, the focused M046/S02 startup rail, `meshc build/test` for `tiny-cluster/`, and the focused S03 failover rail without nested wrapper recursion; asserts named `cargo test` filters actually ran; snapshots fresh `.tmp/m046-s03` bundles after the failover replay; writes `phase-report.txt`, `status.txt`, `current-phase.txt`, and `latest-proof-bundle.txt`; and fails closed if the retained bundle shape or copied manifest drifts.

## Verification

Verified the new runtime-owned startup delay hook with focused `mesh-rt` unit tests, rebuilt and reran the real `tiny-cluster/` package smoke rail, ran the focused S03 failover/rejoin e2e filter to green, and then closed the slice with `bash scripts/verify-m046-s03.sh`, which replayed the prerequisite startup/package commands and copied a fresh failover evidence bundle under `.tmp/m046-s03/verify/retained-m046-s03-artifacts`.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-rt startup_work_delay_ -- --nocapture` | 0 | ✅ pass | 68500ms |
| 2 | `cargo run -q -p meshc -- build tiny-cluster && cargo run -q -p meshc -- test tiny-cluster/tests` | 0 | ✅ pass | 45800ms |
| 3 | `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_failover_ -- --nocapture` | 0 | ✅ pass | 25200ms |
| 4 | `bash scripts/verify-m046-s03.sh` | 0 | ✅ pass | 163500ms |


## Deviations

The task plan originally relied on a package-local delay hook in `tiny-cluster/work.mpl`. After the user override, I moved the pending-window control into `mesh-rt` as a runtime-owned `MESH_STARTUP_WORK_DELAY_MS` startup-dispatch hook and removed the package-owned delay code/docs/tests. I also made the failover harness choose a primary-owned startup placement explicitly because the deterministic startup request key is stable but its owner still depends on hashed node names.

## Known Issues

None.

## Files Created/Modified

- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/meshc/tests/e2e_m046_s03.rs`
- `scripts/verify-m046-s03.sh`
- `tiny-cluster/work.mpl`
- `tiny-cluster/tests/work.test.mpl`
- `tiny-cluster/README.md`
- `.gsd/DECISIONS.md`
- `.gsd/KNOWLEDGE.md`


## Deviations
The task plan originally relied on a package-local delay hook in `tiny-cluster/work.mpl`. After the user override, I moved the pending-window control into `mesh-rt` as a runtime-owned `MESH_STARTUP_WORK_DELAY_MS` startup-dispatch hook and removed the package-owned delay code/docs/tests. I also made the failover harness choose a primary-owned startup placement explicitly because the deterministic startup request key is stable but its owner still depends on hashed node names.

## Known Issues
None.
