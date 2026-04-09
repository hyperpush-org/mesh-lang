# S03: `tiny-cluster/` local no-HTTP proof — UAT

**Milestone:** M046
**Written:** 2026-03-31T21:41:37.207Z

## UAT Type

- UAT mode: mixed runtime + artifact-driven
- Why this mode is sufficient: S03 changes a real package surface (`tiny-cluster/`), runtime failover observability in `mesh-rt`, CLI-only startup/failover inspection, and the retained verification bundle shape. The truthful acceptance surface is the exact build/test/e2e/verifier commands that exercise those boundaries end to end and keep the resulting artifacts on disk.

## Preconditions

- Run from the repository root with Cargo available.
- Loopback ports on `127.0.0.1` and `::1` must be available for temporary dual-node tests.
- No stale local `tiny-cluster` node processes should be holding prior test ports.
- `target/` and `.tmp/m046-s03/` must be writable so the tests can build binaries and retain proof bundles.

## Smoke Test

Run `cargo run -q -p meshc -- build tiny-cluster && cargo run -q -p meshc -- test tiny-cluster/tests`.

**Expected:** the package builds successfully and the package smoke rail reports `3 passed`, proving the real repo package exists, the declared work still returns `1 + 1`, and the smoke contract fails closed on routes, manifest drift, or app-owned timing hooks.

## Test Cases

### 1. Package contract stays source-first, route-free, and timing-seam-free

1. Run `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_package_ -- --nocapture`.
2. Confirm the rail reports `running 2 tests` and both pass.
3. **Expected:**
   - `tiny-cluster/mesh.toml` contains `[package]` only and omits `[cluster]` declarations.
   - `tiny-cluster/main.mpl` contains exactly one `Node.start_from_env()` call and no `HTTP.serve(...)`, `/work`, `/status`, `/health`, or explicit continuity calls.
   - `tiny-cluster/work.mpl` contains exactly one `clustered(work)` marker, a visible `1 + 1` body, and no `Env.get_int`, `Timer.sleep`, `TINY_CLUSTER_*DELAY`, or `MESH_STARTUP_WORK_DELAY_MS` seams.
   - `tiny-cluster/tests/work.test.mpl` and `README.md` keep the same contract and point operators to `meshc cluster status|continuity|diagnostics`.

### 2. Two-node startup dedupes to one logical runtime-owned record

1. Run `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_startup_ -- --nocapture`.
2. Let the test boot both nodes from the real repo-owned `tiny-cluster` binary.
3. **Expected:**
   - The rail reports `running 1 test` and passes.
   - `meshc cluster continuity --json` list output on both nodes discovers the startup record by `declared_handler_runtime_name == "Work.execute_declared_work"`.
   - Both nodes agree on one deterministic `request_key` and the record reaches mirrored `completed/succeeded` state with no duplicate startup execution.
   - `meshc cluster diagnostics --json` shows runtime-owned startup transitions rather than any app-owned submit/status path.

### 3. Route-free failover, recovery, completion, and fenced rejoin are visible only through Mesh CLI surfaces

1. Run `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_failover_ -- --nocapture`.
2. Confirm the rail reports `running 3 tests` and all pass.
3. **Expected:**
   - Helper rails accept mirrored pending/preparing truth and reject malformed cluster JSON.
   - The destructive rail observes a runtime-owned pending window, kills the owner, then proves `automatic_promotion`, `automatic_recovery`, `recovery_rollover`, and final completion on the standby.
   - After restarting the stale primary, the rail proves `fenced_rejoin` and post-rejoin continuity/status truth without any HTTP route or app-owned control seam.
   - The proof retains a `.tmp/m046-s03/tiny-cluster-failover-runtime-truth-*` bundle with scenario metadata, pre/post-kill JSON snapshots, and per-node stdout/stderr logs.

### 4. The direct slice verifier replays the whole proof and retains a fresh evidence bundle

1. Run `bash scripts/verify-m046-s03.sh`.
2. **Expected:**
   - The script prints `verify-m046-s03: ok`.
   - `.tmp/m046-s03/verify/status.txt` contains `ok` and `.tmp/m046-s03/verify/current-phase.txt` contains `complete`.
   - `.tmp/m046-s03/verify/phase-report.txt` shows `contract-guards`, `mesh-rt-build`, `m046-s02-startup`, `tiny-cluster-build`, `tiny-cluster-tests`, `m046-s03-e2e`, `m046-s03-artifacts`, and `m046-s03-bundle-shape` all marked `passed`.
   - `.tmp/m046-s03/verify/latest-proof-bundle.txt` points at `.tmp/m046-s03/verify/retained-m046-s03-artifacts`, and the copied bundle manifest includes at least one `tiny-cluster-failover-runtime-truth-*` directory with `scenario-meta.json`, pre/post-kill status/continuity/diagnostics files, and node stdout/stderr logs.

### 5. Runtime-owned startup dispatch behavior stays bounded and internal to Mesh

1. Run `cargo test -p mesh-rt startup_work_ -- --nocapture`.
2. Confirm the rail reports `running 6 tests` and all pass.
3. **Expected:**
   - The runtime proves `startup_work_dispatch_window_only_applies_to_runtime_owned_clustered_startup_requests`.
   - Startup registration, convergence, keepalive, and standby-skip behavior stay green.
   - The acceptance surface does not require any package/user timing guidance to keep failover observable.

## Edge Cases

### Historical task text can remain in the enduring plan as long as the current-state override is present

1. Open `.gsd/milestones/M046/slices/S03/S03-PLAN.md`.
2. **Expected:** historical T01–T03 text may still mention earlier timing-seam steps, but the file must also contain `**Follow-up override task (T04):** Retire the remaining runtime-env timing seam.`, the line that the pending window must stay Mesh-owned and invisible to `tiny-cluster/` source, and the line that the slice must not require `MESH_STARTUP_WORK_DELAY_MS`.

### Contract guards fail closed on route/timing drift before runtime replay starts

1. Temporarily imagine reintroducing `Env.get_int`, `Timer.sleep`, `TINY_CLUSTER_*DELAY`, `MESH_STARTUP_WORK_DELAY_MS`, `/work`, `/status`, or `/health` into `tiny-cluster/` or its README.
2. **Expected:** `scripts/verify-m046-s03.sh` stops in the `contract-guards` phase before claiming any green runtime proof, leaving the failing content-check log under `.tmp/m046-s03/verify/`.

## Failure Signals

- `tiny-cluster` build or smoke tests fail, or the smoke rail stops reporting `3 passed`.
- The package contract rail starts finding `[cluster]`, routes, explicit continuity calls, or timing helpers in package source, tests, README, or the enduring plan contract.
- The startup rail loses deterministic `declared_handler_runtime_name` discovery, duplicates the startup record, or stops completing through CLI-only surfaces.
- The failover rail no longer emits `automatic_promotion`, `automatic_recovery`, `recovery_rollover`, `startup_dispatch_window`, or `fenced_rejoin`, or it stops retaining the expected `.tmp/m046-s03/...` bundle.
- The direct verifier does not leave `.tmp/m046-s03/verify/status.txt=ok`, a fully passed `phase-report.txt`, and a retained proof-bundle manifest.

## Requirements Proved By This UAT

- R088 — `tiny-cluster/` exists as a local-first, route-free clustered proof using trivial `1 + 1` work.
- R093 — the canonical clustered proof workload stays intentionally trivial and keeps any failover-observability seam out of app code.
- R086 and R091 are advanced by this UAT because `tiny-cluster/` now proves runtime-owned startup/failover/status semantics and CLI-only failover truth on a real package surface.

## Notes for Tester

If the verifier or failover rail goes red, inspect `.tmp/m046-s03/verify/phase-report.txt`, `.tmp/m046-s03/verify/04-m046-s03-artifacts.txt`, and the referenced `tiny-cluster-failover-runtime-truth-*` bundle before editing package source. The intended debugging order is package contract drift first, then CLI/runtime evidence, then node logs — not a fallback to app-owned HTTP routes or timing helpers.
