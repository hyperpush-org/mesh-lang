# S01: Runtime-Owned Cluster Bootstrap — UAT

**Milestone:** M045
**Written:** 2026-03-30T19:22:01.870Z

# S01: Runtime-Owned Cluster Bootstrap — UAT

**Milestone:** M045
**Written:** 2026-03-30

## UAT Type

- UAT mode: mixed
- Why this mode is sufficient: This slice shipped a public runtime API, generated source-contract changes, and packaged proof-rail changes. The honest acceptance surface is therefore a mix of unit/e2e verification plus one assembled replay that proves the scaffold and retained proof app both consume the same runtime-owned bootstrap boundary.

## Preconditions

- Run from the repo root: `/Users/sn0w/Documents/dev/mesh-lang`.
- Rust/Cargo toolchain must be available.
- No other process should be holding the scaffold/proof ports used by the e2e rails.
- Use the checked-in scripts/tests exactly; they already fail closed on zero-test drift where that matters.

## Smoke Test

Run:

```bash
cargo test -p meshc --test e2e_m045_s01 m045_s01_bootstrap_api_scaffold_contract_uses_runtime_owned_bootstrap -- --nocapture
```

**Expected:** the test passes and proves the generated clustered scaffold now contains `Node.start_from_env()` and `BootstrapStatus` instead of direct bootstrap env parsing or `Node.start(...)` orchestration.

## Test Cases

### 1. Runtime bootstrap matrix stays fail-closed and typed

1. Run `cargo test -p mesh-rt bootstrap_ -- --nocapture`.
2. Confirm the suite covers standalone, explicit `MESH_NODE_NAME`, Fly identity fallback, missing-cookie rejection, blank discovery seed, malformed node name, invalid cluster port, partial Fly identity, and bind-failure surfacing.
3. **Expected:** all bootstrap unit tests pass, and malformed env is rejected before partial node startup occurs.

### 2. Mesh code can call `Node.start_from_env()` and misuse fails loudly

1. Run `cargo test -p meshc --test e2e_m045_s01 m045_s01_bootstrap_api_ -- --nocapture`.
2. Confirm the target passes typed happy-path cases for standalone, explicit node identity, Fly identity fallback, and runtime bind failure.
3. Confirm the same target also covers wrong-arity, missing-field, and invalid-`Int` use compile-fail cases.
4. **Expected:** happy-path cases return typed `BootstrapStatus` data, runtime failures surface `Err(String)`, and compile-time misuse fails loudly instead of being coerced into strings or untyped values.

### 3. `meshc init --clustered` generates the smaller runtime-owned startup shape and still runs truthfully

1. Run `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`.
2. Run `cargo test -p meshc --test e2e_m044_s03 m044_s03_scaffold_ -- --nocapture`.
3. In the e2e output, confirm the generated app exposes `/health`, `meshc cluster status <node> --json` works, and the startup logs include the runtime-owned bootstrap message rather than the old app-owned bootstrap log.
4. **Expected:** the generated clustered app builds/runs, `/health` becomes available, `meshc cluster status` reports truth without the CLI joining as a visible peer, and the source/runtime contract stays on `Node.start_from_env()` + `BootstrapStatus`.

### 4. `cluster-proof` adopts the same bootstrap boundary and the assembled slice verifier stays green

1. Run `cargo run -q -p meshc -- build cluster-proof`.
2. Run `cargo run -q -p meshc -- test cluster-proof/tests`.
3. Run `bash scripts/verify-m045-s01.sh`.
4. **Expected:** `cluster-proof` builds, its package tests pass, and the assembled verifier prints `verify-m045-s01: ok` after replaying the M045 bootstrap rails plus the protected M044 scaffold/public-contract rails without zero-test or stale-artifact failures.

## Edge Cases

### Missing cluster cookie with other cluster hints

1. Run `cargo test -p meshc --test e2e_m045_s01 m045_s01_bootstrap_api_fail_closed_without_cookie -- --nocapture`.
2. **Expected:** the test passes and the runtime returns an explicit bootstrap error instead of silently coercing the app into standalone mode.

### Bind failure keeps node identity in the surfaced runtime error

1. Run `cargo test -p meshc --test e2e_m045_s01 m045_s01_bootstrap_api_bind_failure_surfaces_runtime_error -- --nocapture`.
2. **Expected:** the test passes and the reported error includes the resolved node identity/bind context, so a startup failure is diagnosable from the public runtime path.

### Read-only operator query still does not make the CLI appear as a cluster peer

1. Run `cargo test -p meshc --test e2e_m044_s03 m044_s03_scaffold_ -- --nocapture`.
2. Inspect the assertions around `meshc cluster status` in the test output if needed.
3. **Expected:** the query succeeds, but the CLI is not introduced as a visible peer in the reported cluster membership.

## Failure Signals

- `running 0 tests` or a missing named target where the verifier expects real rails.
- Generated `main.mpl` or `cluster-proof/main.mpl` still contains raw bootstrap env reads or direct `Node.start(...)` orchestration.
- Scaffold e2e cannot reach `/health` or the startup log lacks the runtime-owned bootstrap line.
- `meshc cluster status` fails, mutates visible peer membership, or stops matching the scaffolded app’s runtime truth.
- `verify-m045-s01.sh` exits non-zero or stops before replaying the protected M044 rails.

## Requirements Proved By This UAT

- R077 — The primary clustered example surfaces are materially smaller because bootstrap is now runtime-owned and typed instead of app-owned.
- R079 — Example-owned bootstrap/mode/identity logic was removed from the scaffold and retained proof app startup paths.
- R080 — `meshc init --clustered` now demonstrates the public runtime bootstrap boundary instead of teaching proof-app-style startup mechanics.

## Not Proven By This UAT

- R078 end-to-end proof of one tiny example showing local remote execution plus failover on the same app; that belongs to S02/S03.
- R081 docs-first rewrite of the public clustered docs story; S05 still has to move the teaching surface onto the tiny example.
- Full runtime ownership of continuity-topology validation; `cluster-proof/docker-entrypoint.sh` still preflights that env because the bootstrap API does not yet validate it.

## Notes for Tester

Use `bash scripts/verify-m045-s01.sh` as the terminal acceptance rail. The direct unit/e2e commands are useful for localizing failures, but the assembled script is the authoritative slice contract because it protects both the new M045 surfaces and the retained M044 public/scaffold rails in one replay.

