---
estimated_steps: 4
estimated_files: 2
skills_used:
  - rust-best-practices
  - test
---

# T01: Rewrite `meshc init --clustered` to emit the route-free equal-surface contract

Delete the last routeful scaffold shape so the generated clustered app matches the same source-first, route-free contract already proven by `tiny-cluster/` and `cluster-proof/`.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `compiler/mesh-pkg/src/scaffold.rs` clustered templates | Fail the unit and CLI scaffold rails if `[cluster]`, `/health`, `/work`, or `Continuity.submit_declared_work(...)` survive. | N/A | Treat mixed manifest/source declaration state or drifted runtime names as contract errors instead of silently preferring one path. |
| `compiler/meshc/tests/tooling_e2e.rs` init smoke | Fail fast if `meshc init --clustered` stops producing the route-free file set or README contract. | N/A | Treat malformed generated contents as scaffold drift rather than relaxing the assertions. |

## Negative Tests

- **Malformed inputs**: duplicate manifest/source declaration hints, missing `declared_work_runtime_name()`, or changed runtime handler strings.
- **Error paths**: any surviving `[cluster]`, `HTTP.serve(...)`, `/health`, `/work`, `Continuity.submit_declared_work(...)`, `Timer.sleep(...)`, or request-key-only continuity guidance must fail the scaffold contract.
- **Boundary conditions**: the generated scaffold may keep scaffold-specific naming/runbook text, but the emitted `main.mpl`/`work.mpl` control flow must stay structurally aligned with the proof packages.

## Steps

1. Rewrite the clustered scaffold templates in `compiler/mesh-pkg/src/scaffold.rs` so `mesh.toml` is package-only, `main.mpl` only logs `Node.start_from_env()` bootstrap success/failure, and `work.mpl` matches the proof packages around `declared_work_runtime_name()`, `clustered(work)`, and `1 + 1`.
2. Rewrite the generated clustered README contract so it explains source-owned `clustered(work)`, package-only `mesh.toml`, automatic startup work, and CLI-only inspection instead of HTTP submit/status routes.
3. Update the embedded scaffold unit test and the `tooling_e2e` init smoke test to assert the new route-free contract and forbid the deleted routeful strings.
4. Keep the scaffold README scaffold-specific where needed (local run env guidance), but do not let it diverge from the shared runtime-owned clustered story.

## Must-Haves

- [ ] `meshc init --clustered` no longer emits `[cluster]`, `HTTP.serve(...)`, `/health`, `/work`, or `Continuity.submit_declared_work(...)`.
- [ ] Generated `work.mpl` contains `declared_work_runtime_name()`, one `clustered(work)` declaration, runtime name `Work.execute_declared_work`, and visible `1 + 1` work.
- [ ] Generated `main.mpl` has one `Node.start_from_env()` bootstrap path and only logs success/failure.
- [ ] The fast scaffold unit/CLI rails assert the route-free contract directly.

## Done When

- [ ] `compiler/mesh-pkg/src/scaffold.rs` emits the same clustered-work story as the proof packages.
- [ ] `compiler/meshc/tests/tooling_e2e.rs` passes against the new scaffold output without retaining routeful expectations.

## Inputs

- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/meshc/tests/tooling_e2e.rs`
- `tiny-cluster/main.mpl`
- `tiny-cluster/work.mpl`
- `cluster-proof/main.mpl`
- `cluster-proof/work.mpl`
- `tiny-cluster/README.md`
- `cluster-proof/README.md`

## Expected Output

- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/meshc/tests/tooling_e2e.rs`

## Verification

cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture && cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture
