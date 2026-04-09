# S04: Hard cutover and dogfood migration — UAT

**Milestone:** M047
**Written:** 2026-04-01T10:47:51.821Z

# S04: Hard cutover and dogfood migration — UAT

**Milestone:** M047
**Written:** 2026-04-01

## UAT Type

- UAT mode: mixed
- Why this mode is sufficient: This slice changed parser/pkg/compiler behavior, generated scaffold output, repo-owned proof packages, docs/runbooks, historical verifier delegation, and one runtime recovery seam. The honest acceptance surface is a mix of artifact checks, named cargo rails, and the assembled verifier bundle.

## Preconditions

- Run from the repo root with the Rust toolchain installed.
- Website dependencies are already installed so `npm --prefix website run build` can run.
- No separate service deployment is required; all checks are local repo rails.

## Smoke Test

1. Run `bash scripts/verify-m047-s04.sh`.
2. Open `.tmp/m047-s04/verify/status.txt`.
3. Open `.tmp/m047-s04/verify/current-phase.txt`.
4. **Expected:** the script exits 0, `status.txt` contains `ok`, and `current-phase.txt` contains `complete`.

## Test Cases

### 1. Legacy clustered source and manifest inputs now fail closed with migration guidance

1. Run `cargo test -p mesh-parser m047_s04 -- --nocapture`.
2. Run `cargo test -p mesh-pkg m047_s04 -- --nocapture`.
3. Run `cargo test -p meshc --test e2e_m047_s01 -- --nocapture`.
4. **Expected:** parser/package/compiler rails all pass; the legacy-source and legacy-manifest cases are rejected with migration-oriented guidance instead of being accepted as live clustered declarations.

### 2. `meshc init --clustered` generates the new public source-first contract

1. Run `cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture`.
2. Run `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`.
3. Inspect the generated fixture output if needed.
4. **Expected:** generated `work.mpl` contains `@cluster pub fn execute_declared_work(...)`, generated `mesh.toml` stays package-only, `main.mpl` stays route-free with `Node.start_from_env()`, and legacy `clustered(work)` / `[cluster]` markers are absent.

### 3. Repo-owned dogfood packages teach the same source-first route-free story

1. Run `cargo run -q -p meshc -- test tiny-cluster/tests`.
2. Run `cargo run -q -p meshc -- build tiny-cluster`.
3. Run `cargo run -q -p meshc -- test cluster-proof/tests`.
4. Run `cargo run -q -p meshc -- build cluster-proof`.
5. **Expected:** both package smoke suites and builds pass; `tiny-cluster/` and `cluster-proof/` both require `@cluster pub fn execute_declared_work(...)`, reject `clustered(work)`, reject manifest clustering, and keep operators on `meshc cluster status|continuity|diagnostics`.

### 4. Historical route-free rails stay green on the new contract

1. Run `cargo test -p meshc --test e2e_m045_s01 m045_s01_ -- --nocapture`.
2. Run `cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture`.
3. Run `cargo test -p meshc --test e2e_m045_s03 m045_s03_ -- --nocapture`.
4. Run `cargo test -p meshc --test e2e_m046_s03 m046_s03_ -- --nocapture`.
5. Run `cargo test -p meshc --test e2e_m046_s04 m046_s04_ -- --nocapture`.
6. Run `cargo test -p meshc --test e2e_m046_s05 m046_s05_ -- --nocapture`.
7. Run `cargo test -p meshc --test e2e_m046_s06 m046_s06_ -- --nocapture`.
8. **Expected:** all historical route-free rails pass while asserting the new `@cluster` wording, runtime-name continuity through `Work.execute_declared_work`, and the compatibility-alias verifier story.

### 5. Public docs and verifier ownership now point at the M047 cutover rail

1. Run `npm --prefix website run build`.
2. Run `cargo test -p meshc --test e2e_m047_s04 -- --nocapture`.
3. Run `cargo test -p meshc --test e2e_m045_s04 -- --nocapture`.
4. Run `cargo test -p meshc --test e2e_m045_s05 -- --nocapture`.
5. **Expected:** the site builds, `e2e_m047_s04` passes, and the historical wrapper tests prove that `scripts/verify-m047-s04.sh` is the authoritative rail while the older M045/M046 wrapper names are only compatibility aliases.

## Edge Cases

### Startup recovery after promotion still works on the route-free proof path

1. Run `cargo test -p mesh-rt startup_automatic_recovery_relaxes_single_node_required_replica_count -- --nocapture`.
2. Run `cargo test -p meshc --test e2e_m046_s03 m046_s03_ -- --nocapture`.
3. **Expected:** the focused runtime regression test passes and the historical tiny-cluster failover rail no longer regresses with `replica_required_unavailable` after standby promotion.

### The authoritative verifier retains a debuggable bundle shape

1. Run `bash scripts/verify-m047-s04.sh`.
2. Inspect `.tmp/m047-s04/verify/phase-report.txt`.
3. Inspect `.tmp/m047-s04/verify/latest-proof-bundle.txt` and confirm the pointed directory exists.
4. **Expected:** every phase is marked `passed`, the pointer file resolves to a retained artifact directory, and failures would be localizable from this bundle without rerunning the whole matrix.

## Failure Signals

- `clustered(work)` or `[cluster]` is accepted as a live clustered declaration anywhere in parser/pkg/compiler flows.
- Generated scaffold/package `work.mpl` files contain the deleted helper or legacy markers.
- README/docs text claims `HTTP.clustered(...)` already exists.
- Historical M045/M046 wrapper tests still treat their own verifier names as authoritative instead of delegating to `scripts/verify-m047-s04.sh`.
- Route-free failover rails regress with `replica_required_unavailable` after promotion.
- `.tmp/m047-s04/verify/status.txt` is not `ok`, `current-phase.txt` is not `complete`, or `latest-proof-bundle.txt` is malformed.

## Requirements Proved By This UAT

- R097 — `@cluster` / `@cluster(N)` are now the supported public clustered syntax and the legacy source form is rejected with migration guidance.
- R102 — the old public clustered surface is removed instead of kept as a coequal taught model.
- R103 — repo-owned clustered scaffold/example/proof/verifier surfaces now dogfood the source-first contract.

## Not Proven By This UAT

- R100 / R101 — `HTTP.clustered(...)` route-wrapper behavior remains intentionally unproven because that feature is still unshipped.
- The future SQLite Todo scaffold from S05 is not covered here.
- Automatic requirement-status projection in the GSD DB is not covered; the current DB still rejects M047 requirement IDs even though the checked-in requirements file renders them.

## Notes for Tester

This slice is a hard public contract cutover, so expect most proofs to be exact-string or retained-artifact sensitive. The authoritative place to start if anything fails is `.tmp/m047-s04/verify/phase-report.txt` and the retained bundle pointed to by `.tmp/m047-s04/verify/latest-proof-bundle.txt`. If the failure is specifically a post-promotion route-free regression, inspect `compiler/mesh-rt/src/dist/node.rs` and the `startup_automatic_recovery_relaxes_single_node_required_replica_count` rail before touching docs or scaffold text.
