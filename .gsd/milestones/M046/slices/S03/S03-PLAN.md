# S03: `tiny-cluster/` local no-HTTP proof

**Goal:** Create a real repo-owned `tiny-cluster/` package as the smallest route-free clustered proof surface, then prove local startup, failover, and status truth entirely through Mesh-owned runtime/CLI surfaces.
**Demo:** After this: After this: `tiny-cluster/` proves the local clustered story with no HTTP routes, trivial `1 + 1` work, and live runtime-owned placement/failover/status truth.

## Tasks
- [x] **T01: Created the real `tiny-cluster` package with a bounded local delay hook and file-backed smoke tests.** — Create the actual repo-owned `tiny-cluster/` package by promoting the S02 temp fixture into durable source files, keeping the public surface strictly route-free, and adding a tiny package test/readme contract so the package proves itself before any Rust e2e harness touches it.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `tiny-cluster/work.mpl` delay + declared work contract | Fail the package test or fall back to the default `1 + 1` path instead of silently changing the workload. | N/A | Treat malformed or negative delay input as zero or explicit failure; do not let the proof hang behind arbitrary sleeps. |
| `tiny-cluster/mesh.toml` package manifest | Fail `meshc build` if manifest drift reintroduces `[cluster]` declarations or any second declaration path. | N/A | Keep manifest parsing fail-closed; never auto-synthesize declarations that would hide source drift. |
| `tiny-cluster/tests/work.test.mpl` package smoke rail | Fail fast on missing imports or missing contract helpers instead of punting all proof back to the Rust e2e rail. | N/A | Treat unexpected route or submit/status strings as contract failures. |

## Load Profile

- **Shared resources**: One local declared-work execution plus an optional env-controlled delay hook used only by later failover rails.
- **Per-operation cost**: One trivial declared work call and, when the local env hook is enabled, one bounded `Timer.sleep(...)` before returning `2`.
- **10x breakpoint**: Unbounded or misleading delay values would make the proof dishonest long before compute or memory costs matter.

## Negative Tests

- **Malformed inputs**: Missing, invalid, negative, or oversized delay values if the helper normalizes them, plus manifest drift toward `[cluster]` declarations.
- **Error paths**: Build/test failure when package code reintroduces `HTTP.serve(...)`, `/work`, `/status`, `/health`, or explicit continuity calls.
- **Boundary conditions**: Default no-delay execution still returns `2`, and the tiny work body stays visibly trivial even when the failover harness later opts into a bounded delay window.

## Steps

1. Create `tiny-cluster/mesh.toml`, `tiny-cluster/main.mpl`, and `tiny-cluster/work.mpl` by promoting the S02 temp fixture into real package files with a package-only manifest and a single source `clustered(work)` declaration.
2. Keep `tiny-cluster/main.mpl` route-free (`Node.start_from_env()` only) and keep `tiny-cluster/work.mpl` visibly trivial (`1 + 1`) while adding the smallest package-local, opt-in delay hook the later failover rail can use without becoming a public control surface.
3. Add `tiny-cluster/tests/work.test.mpl` so the package itself proves the declared work contract before any Rust e2e harness runs.
4. Write `tiny-cluster/README.md` as a local-only runbook that points operators to `meshc cluster status|continuity|diagnostics` rather than app routes and explicitly avoids premature scaffold/public-doc alignment work.

## Must-Haves

- [ ] `tiny-cluster/` exists as a real repo-owned package with buildable source and at least one package test file.
- [ ] The package stays source-first: no `[cluster]` declarations in `mesh.toml`, no `/work`, `/status`, `/health`, and no explicit continuity submit/status calls in source.
- [ ] The declared work still resolves to trivial arithmetic (`1 + 1`) by default; any delay hook is local-only and opt-in for the failover harness.
- [ ] The README teaches the runtime-owned CLI inspection story instead of inventing a package-owned control plane.

## Verification

- `cargo run -q -p meshc -- build tiny-cluster`
- `cargo run -q -p meshc -- test tiny-cluster/tests`
  - Estimate: 2h
  - Files: tiny-cluster/mesh.toml, tiny-cluster/main.mpl, tiny-cluster/work.mpl, tiny-cluster/tests/work.test.mpl, tiny-cluster/README.md
  - Verify: cargo run -q -p meshc -- build tiny-cluster && cargo run -q -p meshc -- test tiny-cluster/tests
- [x] **T02: Added a repo-backed `tiny-cluster` e2e rail that proves startup/status truth through Mesh CLI surfaces.** — Build the real `tiny-cluster/` package through a new Rust e2e rail that proves the source/manifest contract, boots two nodes without HTTP routes, and discovers the deterministic startup record entirely through runtime-owned CLI surfaces.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Real `tiny-cluster/` package build path | Fail before launching nodes and archive the build output instead of silently swapping back to a temp fixture. | N/A | Treat missing package files or malformed source assertions as a contract failure. |
| `compiler/meshc/tests/e2e_m046_s02.rs` helper port | Archive build/stdout/stderr and fail with the retained last observation instead of collapsing the proof to one panic line. | Bound each wait and record which CLI surface failed to converge. | Treat malformed JSON or missing fields as proof failures with archived raw output. |
| `meshc cluster status|continuity|diagnostics` queries | Fail on `target_not_connected` or missing runtime name rather than probing an app route. | Archive the last CLI output and logs for the failed node. | Reject malformed CLI JSON as a tooling proof failure. |

## Load Profile

- **Shared resources**: Two long-running local node processes, one continuity registry, bounded CLI polling, and retained artifact directories under `.tmp/m046-s03/...`.
- **Per-operation cost**: Two process boots plus repeated `meshc cluster status`, `meshc cluster continuity`, and `meshc cluster diagnostics` queries until one logical startup record completes.
- **10x breakpoint**: Slow convergence, duplicate startup records, or artifact churn will fail the rail before test-runner CPU becomes interesting.

## Negative Tests

- **Malformed inputs**: Package drift that reintroduces `[cluster]`, `HTTP.serve(...)`, `/work`, `/status`, `/health`, or explicit continuity calls.
- **Error paths**: Missing runtime-name discovery, duplicate startup execution, or missing diagnostics must fail with retained CLI evidence.
- **Boundary conditions**: Two-node simultaneous boot still dedupes to one logical startup record, and the work body remains trivial (`1 + 1`) even though the runtime owns the orchestration.

## Steps

1. Add `compiler/meshc/tests/e2e_m046_s03.rs` that builds the real repo package at `tiny-cluster/` (not temp source strings) and copies the core package files into `.tmp/m046-s03/...` artifacts for contract debugging.
2. Add file-content assertions that fail if `tiny-cluster/` reintroduces `[cluster]`, `HTTP.serve(...)`, `/work`, `/status`, `/health`, or explicit continuity calls, and confirm the single `Node.start_from_env()` and visible `1 + 1` work body.
3. Reuse the S02 dual-node route-free harness pattern to boot two nodes from the built `tiny-cluster` binary and discover the startup record by `declared_handler_runtime_name == "Work.execute_declared_work"` via list mode on both nodes.
4. Assert one logical record completes on both nodes, completion stays Mesh-owned, and diagnostics show startup trigger/completion without any app-owned control flow.

## Must-Haves

- [ ] The rail builds and runs the real repo package, not a copied temp fixture.
- [ ] `meshc cluster continuity` list/single-record output is sufficient to discover and inspect the startup work for `tiny-cluster/`.
- [ ] Two nodes converge on one logical startup record and complete without app-owned submit/status routes.
- [ ] Failures retain build logs, CLI JSON, and node stdout/stderr under `.tmp/m046-s03/...`.

## Verification

- `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_package_ -- --nocapture`
- `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_startup_ -- --nocapture`

## Observability Impact

- Signals added/changed: `.tmp/m046-s03/...` build/status/continuity/diagnostics snapshots for the real `tiny-cluster` package.
- How a future agent inspects this: rerun the focused test filters and inspect the archived JSON/logs in the failing bundle.
- Failure state exposed: the last membership, continuity, and diagnostics observation plus per-node stdout/stderr stay attached to the failure.
  - Estimate: 3h
  - Files: compiler/meshc/tests/e2e_m046_s03.rs, tiny-cluster/mesh.toml, tiny-cluster/main.mpl, tiny-cluster/work.mpl, compiler/meshc/tests/e2e_m046_s02.rs
  - Verify: cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_package_ -- --nocapture && cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_startup_ -- --nocapture
- [x] **T03: Moved the tiny-cluster failover delay into mesh-rt, proved route-free promotion/recovery/fenced rejoin from Mesh CLI surfaces, and added `scripts/verify-m046-s03.sh` as the direct slice verifier.** — Extend the S03 e2e rail into the destructive failover/rejoin proof and close the slice with a direct verifier that replays consumed prerequisites, copies fresh `.tmp/m046-s03` bundles, and fails closed on missing tests or missing evidence.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `tiny-cluster/work.mpl` failover delay window | Fail with archived pre-kill snapshots if the startup record never becomes observable instead of inventing a route or second submit path. | Bound the wait for a mirrored pending window and archive the last continuity/status snapshot before failing. | Treat malformed delay input as zero/invalid and fail the pre-kill discovery rail honestly. |
| `compiler/meshc/tests/e2e_m045_s03.rs` helper port | Reuse only CLI/process helpers and fail with archived last observations instead of falling back to HTTP helpers. | Bound promotion/recovery/rejoin polling windows and record which CLI surface failed. | Reject malformed JSON and missing diagnostic transitions as proof failures. |
| `scripts/verify-m046-s03.sh` direct wrapper | Fail on timeout, zero tests, or missing fresh artifact bundles instead of reporting a false green wrapper. | Mark the current phase as failed and keep the last command log plus artifact hint. | Treat malformed phase reports, pointers, or bundle manifests as verifier failures. |

## Load Profile

- **Shared resources**: Two node processes, one forced owner kill, one rejoin, repeated CLI polling, and copied proof bundles under `.tmp/m046-s03/verify`.
- **Per-operation cost**: Startup discovery, destructive failover, promotion/recovery/rejoin observation, plus one direct verifier replay of the prerequisite commands.
- **10x breakpoint**: Pending-window timing drift and artifact bundle size will break first; raw CPU is not the bottleneck.

## Negative Tests

- **Malformed inputs**: Missing or invalid work-delay env values, missing runtime-name discovery, or a continuity record that never reaches mirrored pending state before kill.
- **Error paths**: Missing `automatic_promotion`, `automatic_recovery`, `recovery_rollover`, or `fenced_rejoin` transitions must fail with retained evidence instead of retrying indefinitely.
- **Boundary conditions**: The stale primary must rejoin fenced as standby after promotion, and the package’s default no-delay behavior must remain fast/trivial outside the failover harness.

## Steps

1. Use the package-local delay env only inside the failover harness to keep the startup record pending long enough to discover through `meshc cluster continuity` list mode, then treat the returned record as the single source of truth.
2. Port only the CLI/process wait/assert helpers needed from `compiler/meshc/tests/e2e_m045_s03.rs` into `compiler/meshc/tests/e2e_m046_s03.rs` to kill the owner, prove standby promotion/recovery/completion, and assert fenced rejoin with no HTTP routes or app submit/status contract.
3. Retain scenario metadata, pre/post-kill CLI snapshots, and node logs under `.tmp/m046-s03/...` so promotion/recovery drift is diagnosable from one bundle.
4. Add `scripts/verify-m046-s03.sh` with direct prerequisite commands (`cargo build -q -p mesh-rt`, the focused S02 startup rail, `meshc build/test` on `tiny-cluster/`, and the S03 e2e rail), named-test-count assertions, artifact snapshot/copy logic, and bundle-shape checks without nested wrapper recursion.

## Must-Haves

- [ ] The failover/rejoin proof uses only runtime-owned CLI surfaces; no route or app-owned control seam is added.
- [ ] The default package behavior stays fast and trivial; the delay env is test-only and does not change the public workload story.
- [ ] Promotion, recovery, completion, and fenced rejoin are all asserted from retained `meshc cluster status|continuity|diagnostics` evidence.
- [ ] `scripts/verify-m046-s03.sh` replays direct prerequisite commands, proves named test filters ran, and retains a fresh copied failover bundle fail-closed.

## Verification

- `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_failover_ -- --nocapture`
- `bash scripts/verify-m046-s03.sh`

## Observability Impact

- Signals added/changed: failover bundles with `scenario-meta.json`, pre/post-kill status/continuity/diagnostics snapshots, and verifier `phase-report.txt`, `status.txt`, `current-phase.txt`, plus `latest-proof-bundle.txt`.
- How a future agent inspects this: open `.tmp/m046-s03/verify/phase-report.txt`, follow `.tmp/m046-s03/verify/latest-proof-bundle.txt`, and inspect the copied node logs/JSON snapshots.
- Failure state exposed: the exact last phase, failing command log, and copied failover evidence directory remain on disk after verifier failure.
  - Estimate: 3h
  - Files: compiler/meshc/tests/e2e_m046_s03.rs, scripts/verify-m046-s03.sh, tiny-cluster/work.mpl, compiler/meshc/tests/e2e_m045_s03.rs, compiler/meshc/tests/e2e_m046_s02.rs
  - Verify: cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_failover_ -- --nocapture && bash scripts/verify-m046-s03.sh
- [x] **T04: Replaced the last tiny-cluster failover timing knob with a language-owned startup dispatch window and fail-closed contract guards.** — 1. Audit `tiny-cluster/`, the S03 e2e rail, the local verifier, and any slice-owned runbook text for surviving `Env.get_int(...)`, `Timer.sleep(...)`, `TINY_CLUSTER_*DELAY*`, or user-directed `MESH_STARTUP_WORK_DELAY_MS` guidance.
2. Replace any remaining app/package code or user-facing setup requirement with a Mesh-owned runtime/proof seam that stays invisible to example code while still giving the failover rail an observable pending window.
3. Extend negative assertions so the package smoke rail, e2e contract checks, and verifier fail closed if tiny-cluster source or slice-owned guidance reintroduce package-owned timing helpers.
4. Re-run the focused runtime, package, failover, and direct verifier commands and retain a fresh `.tmp/m046-s03/...` evidence bundle proving the route-free failover story still works without app/user-owned timing seams.
  - Estimate: 2h
  - Files: compiler/mesh-rt/src/dist/node.rs, compiler/meshc/tests/e2e_m046_s03.rs, scripts/verify-m046-s03.sh, tiny-cluster/work.mpl, tiny-cluster/tests/work.test.mpl, tiny-cluster/README.md
  - Verify: cargo test -p mesh-rt startup_work_ -- --nocapture && cargo run -q -p meshc -- build tiny-cluster && cargo run -q -p meshc -- test tiny-cluster/tests && cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_failover_ -- --nocapture && bash scripts/verify-m046-s03.sh
