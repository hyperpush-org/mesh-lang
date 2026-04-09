# S05: Cluster-Proof Rewrite, Docs, and Final Closeout

**Goal:** Finish M044 by turning `cluster-proof` into a pure dogfood consumer of the public clustered-app model, removing its old explicit clustering path, and publishing a final closeout verifier/docs story that teaches scaffold-first clustered Mesh apps instead of proof-app folklore.
**Demo:** After this: After this: `cluster-proof` is a dogfood consumer of the new clustered-app model, the old explicit clustering path is gone from its code, and the docs/verifiers teach “build a clustered Mesh app” as the primary story.

## Tasks
- [x] **T01: Retargeted cluster-proof bootstrap to the public MESH_* contract and added live public-contract startup coverage; the S03 operator rail still needs one fresh rerun.** — Retarget `cluster-proof` startup, config, and live harnesses to the same clustered-app contract that `meshc init --clustered` already ships. This task should remove the proof-app-specific bootstrap/env dialect from the paths ordinary operators and the final docs teach, while keeping `CLUSTER_PROOF_WORK_DELAY_MS` as the only proof-only timing knob.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Config/bootstrap parsing in `cluster-proof/config.mpl` and `cluster-proof/main.mpl` | Fail startup with explicit config errors instead of silently falling back to standalone mode. | Bound live harness waits and surface which node never became ready. | Reject malformed `MESH_*` identity/cookie/topology values and keep the node down. |
| Container bootstrap in `cluster-proof/docker-entrypoint.sh` and `cluster-proof/fly.toml` | Stop the image before the binary starts and record the failing env contract. | N/A — startup validation path. | Reject partial identity or contradictory role/epoch env instead of synthesizing proof-app defaults. |
| Live harnesses in `compiler/meshc/tests/e2e_m044_s03.rs`, `compiler/meshc/tests/e2e_m044_s04.rs`, and `compiler/meshc/tests/e2e_m044_s05.rs` | Fail the affected rail with retained stdout/stderr artifacts. | Mark the exact startup/membership stage that stalled. | Reject malformed membership/authority payloads instead of letting the public-contract proof pass. |

## Load Profile

- **Shared resources**: live node ports, env-driven bootstrap parsing, and two-node startup/membership convergence in the M044 e2e harnesses.
- **Per-operation cost**: one process start per node plus membership polling and read-only operator inspection.
- **10x breakpoint**: local port contention and startup retries fail before app logic; the contract must stay small and deterministic.

## Negative Tests

- **Malformed inputs**: blank or missing `MESH_CLUSTER_COOKIE`, malformed `MESH_NODE_NAME`, blank `MESH_DISCOVERY_SEED`, and contradictory `MESH_CONTINUITY_ROLE` / `MESH_CONTINUITY_PROMOTION_EPOCH` values.
- **Error paths**: cluster identity hints without a cookie, Fly identity missing one required value, and old `CLUSTER_PROOF_*` bootstrap names no longer honored.
- **Boundary conditions**: standalone mode without a cookie, clustered mode with explicit `MESH_NODE_NAME`, and Fly-derived identity without `MESH_NODE_NAME`.

## Steps

1. Replace the proof-app-specific cookie and node-identity helpers in `cluster-proof/config.mpl` / `cluster-proof/main.mpl` with the public `MESH_CLUSTER_COOKIE` and `MESH_NODE_NAME` contract, while keeping Fly identity fallback and durability/topology validation honest.
2. Update `cluster-proof/docker-entrypoint.sh`, `cluster-proof/fly.toml`, and `cluster-proof/tests/config.test.mpl` so the local same-image and Fly startup paths use the same public contract and only `CLUSTER_PROOF_WORK_DELAY_MS` remains proof-specific.
3. Rewire `compiler/meshc/tests/e2e_m044_s03.rs` and `compiler/meshc/tests/e2e_m044_s04.rs` to launch `cluster-proof` with the public env surface, and add `compiler/meshc/tests/e2e_m044_s05.rs` coverage for live `cluster-proof` public-contract startup.
4. Keep the new proof rail fail-closed on bootstrap drift by asserting the proof app starts, joins, and reports membership/authority truth without any dependency on `CLUSTER_PROOF_COOKIE`, `CLUSTER_PROOF_NODE_BASENAME`, or `CLUSTER_PROOF_ADVERTISE_HOST`.

## Must-Haves

- [ ] `cluster-proof` bootstrap/config accepts the same public `MESH_*` contract the scaffolded app uses.
- [ ] The same-image local and Fly bootstrap paths stay truthful on the public contract, with `CLUSTER_PROOF_WORK_DELAY_MS` as the only remaining proof-only env knob.
- [ ] The M044 live harnesses and a new S05 e2e rail prove the contract with real cluster startup, not just file-content greps.
  - Estimate: 90m
  - Files: cluster-proof/config.mpl, cluster-proof/main.mpl, cluster-proof/docker-entrypoint.sh, cluster-proof/fly.toml, cluster-proof/tests/config.test.mpl, compiler/meshc/tests/e2e_m044_s03.rs, compiler/meshc/tests/e2e_m044_s04.rs, compiler/meshc/tests/e2e_m044_s05.rs
  - Verify: cargo run -q -p meshc -- test cluster-proof/tests/config.test.mpl
cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture
cargo test -p meshc --test e2e_m044_s05 m044_s05_public_contract_ -- --nocapture
- [x] **T02: Removed cluster-proof’s legacy /work probe path and proved the keyed runtime-owned route is the only remaining work surface.** — Now that the public bootstrap contract is stable, finish the dogfood rewrite by removing the old explicit clustering probe path and the dead manual-era helpers from `cluster-proof`. After this task, the app should expose only the keyed runtime-owned submit/status path, with no `WorkLegacy` route or app-owned placement/dispatch logic left in code.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Keyed continuity submit/status path in `cluster-proof/work_continuity.mpl` | Fail the package/e2e rail and keep the keyed route red rather than falling back to the deleted legacy path. | Bound status polling in tests and surface whether submit or completion stalled. | Reject malformed continuity records or request payloads instead of inventing partial JSON. |
| Remaining keyed helper module in `cluster-proof/work.mpl` | Keep request/status models and validation truthful even as legacy placement helpers are removed. | N/A — local helper path. | Reject malformed request keys and payloads explicitly. |
| Package tests and new live `e2e_m044_s05` coverage | Fail closed on missing 404/absence checks, stale helper literals, or regressions in duplicate/conflict status behavior. | Bound live server waits and surface the exact rail that still depended on the legacy probe. | Reject malformed responses instead of letting the cleanup proof pass on happy-path only behavior. |

## Load Profile

- **Shared resources**: runtime continuity records, async declared-work execution, and package/e2e status polling.
- **Per-operation cost**: one keyed submit plus duplicate/conflict/status lookups; the cleanup risk is correctness drift, not throughput.
- **10x breakpoint**: repeated same-key submits and pending-status polling will stress the keyed path first if the legacy cleanup accidentally drops the runtime-owned behavior.

## Negative Tests

- **Malformed inputs**: invalid request keys, malformed submit JSON, and status lookups for missing request keys.
- **Error paths**: duplicate submit with a rejected record, conflicting same-key submit, and keyed status while authority is unavailable.
- **Boundary conditions**: `GET /work` returns 404 / is not mounted, `POST /work` still creates and reuses keyed records, and `cluster-proof/work_legacy.mpl` is gone from the tree.

## Steps

1. Remove `WorkLegacy` route wiring from `cluster-proof/main.mpl` and delete `cluster-proof/work_legacy.mpl`, keeping only the keyed `POST /work` / `GET /work/:request_key` surfaces mounted.
2. Shrink `cluster-proof/work.mpl` to keyed request/status models and validation helpers only, or extract the surviving keyed helpers into a smaller module, so `TargetSelection`, canonical placement, and legacy probe utilities disappear from app code.
3. Delete dead manual/legacy helpers from `cluster-proof/work_continuity.mpl` (`promotion_response_status_code`, `log_promotion*`, `dispatch_work`, `run_legacy_probe_record`, `submit_from_selection`) and update `cluster-proof/tests/work.test.mpl` plus `compiler/meshc/tests/e2e_m044_s05.rs` to assert the keyed runtime-owned path still works and the legacy route is absent.
4. Add fail-closed absence checks so the rewrite cannot quietly reintroduce `WorkLegacy`, `handle_work_probe`, or app-owned placement/dispatch helpers after the cleanup lands.

## Must-Haves

- [ ] `cluster-proof` exposes only the keyed runtime-owned submit/status surface; the legacy `GET /work` probe is gone.
- [ ] `WorkLegacy`, `TargetSelection`, and the manual-era promotion/dispatch helpers are removed from the proof app code, not merely left dead.
- [ ] Package tests and the S05 live e2e rail prove keyed submit/status behavior and the absence of the old explicit path together.
  - Estimate: 90m
  - Files: cluster-proof/main.mpl, cluster-proof/work.mpl, cluster-proof/work_continuity.mpl, cluster-proof/work_legacy.mpl, cluster-proof/tests/work.test.mpl, compiler/meshc/tests/e2e_m044_s05.rs
  - Verify: cargo run -q -p meshc -- build cluster-proof
cargo run -q -p meshc -- test cluster-proof/tests
cargo test -p meshc --test e2e_m044_s05 m044_s05_legacy_cleanup_ -- --nocapture
test ! -e cluster-proof/work_legacy.mpl
- [x] **T03: Added `scripts/verify-m044-s05.sh`, rewrote the clustered docs around `meshc init --clustered` + `meshc cluster`, and surfaced the remaining S04 replay drift in the assembled closeout rail.** — Close the milestone by making the docs and proof surfaces say the same thing the code now proves. This task adds a dedicated `scripts/verify-m044-s05.sh` final-assembly rail that replays the earlier clustered-app and failover proofs, then rewrites the public runbooks/pages so `meshc init --clustered` + `meshc cluster` are the primary clustered-app story and `cluster-proof` is the deeper dogfood/runbook proof consumer.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Assembled verifier in `scripts/verify-m044-s05.sh` | Stop at the first failing phase with retained phase/status/source-truth artifacts. | Mark the exact replay phase that stalled and keep the copied artifact paths. | Fail closed on missing `running N test`, malformed JSON/artifacts, or stale source/docs literals. |
| Prior behavior rails `scripts/verify-m044-s03.sh` and `scripts/verify-m044-s04.sh` | Surface the upstream failing rail directly instead of masking it behind docs-only success. | Preserve the failing prior phase and copied bundle path for debugging. | Reject malformed retained bundle state instead of passing the closeout rail on incomplete evidence. |
| Public docs/build surfaces | Fail the docs truth/build step rather than letting stale M043/S04-only wording or legacy route/env text ship. | N/A — local file/build path. | Reject missing markers, stale command blocks, or mixed clustered-app vs proof-app stories. |

## Load Profile

- **Shared resources**: replayed verifier artifact directories, docs build output, and the final S05 closeout artifact bundle.
- **Per-operation cost**: one S03 replay, one S04 replay, one S05 e2e/package replay, and one docs build/truth sweep.
- **10x breakpoint**: repeated verifier reruns churn artifact directories before code paths saturate; the closeout script must stay deterministic and fail fast.

## Negative Tests

- **Malformed inputs**: missing retained bundle files, missing `running N test` evidence, and stale docs/source literals such as `GET /work`, `CLUSTER_PROOF_COOKIE`, `remain later distributed slices`, or `M043 failover contract`.
- **Error paths**: S03 scaffold/operator drift, S04 failover drift, docs that still present `cluster-proof` as the first abstraction, or source absence checks that miss a reintroduced legacy helper.
- **Boundary conditions**: scaffold-first docs still route readers to the deeper `cluster-proof` runbook for the bounded failover details, and the Fly story remains clearly read-only.

## Steps

1. Add `scripts/verify-m044-s05.sh` as the authoritative final-assembly rail: replay `scripts/verify-m044-s03.sh` and `scripts/verify-m044-s04.sh`, run the named `e2e_m044_s05` filter plus `cluster-proof` build/tests, fail closed on zero-test runs, and retain `.tmp/m044-s05/verify/` status/source/docs-truth artifacts.
2. Rewrite `cluster-proof/README.md`, `website/docs/docs/distributed-proof/index.md`, `website/docs/docs/distributed/index.md`, `website/docs/docs/tooling/index.md`, and repo `README.md` so the primary clustered story starts with the scaffold/operator surfaces and treats `cluster-proof` as the deeper dogfood proof consumer.
3. Keep the public contract exact: no legacy `GET /work` probe or proof-specific bootstrap envs in docs, no stale “later slices” or “M043 failover contract” wording, and the Fly proof remains clearly read-only.
4. Rebuild the docs and run the full S05 verifier so the shipped runbooks, website pages, and retained closeout artifacts all agree on the final M044 story.

## Must-Haves

- [ ] `scripts/verify-m044-s05.sh` is the authoritative local closeout command and replays the S03/S04 product rails instead of checking docs in isolation.
- [ ] Public docs teach `meshc init --clustered` + `meshc cluster` as the primary clustered-app story and route failover depth to `cluster-proof` without treating it as the first abstraction.
- [ ] Source/docs truth checks fail on legacy route/env wording and on stale “later slice” / “M043 contract” language.
  - Estimate: 75m
  - Files: scripts/verify-m044-s05.sh, README.md, cluster-proof/README.md, website/docs/docs/tooling/index.md, website/docs/docs/distributed/index.md, website/docs/docs/distributed-proof/index.md
  - Verify: cargo test -p meshc --test e2e_m044_s05 -- --nocapture
bash scripts/verify-m044-s05.sh
npm --prefix website run build
