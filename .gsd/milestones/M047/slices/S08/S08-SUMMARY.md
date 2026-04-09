---
id: S08
parent: M047
milestone: M047
provides:
  - A truthful `meshc init --template todo-api` starter that keeps `@cluster` route-free in `work.mpl` while dogfooding explicit-count `HTTP.clustered(1, ...)` on the selected read routes.
  - Native and Docker single-node clustered-route proof rails that record continuity/operator truth for `Api.Todos.handle_list_todos` instead of merely generating wrapper syntax.
  - Public docs and README copy that describe the Todo starter's wrapper adoption honestly while preserving the canonical route-free clustered examples and pointing broader wrapper behavior at S07.
  - Retained `.tmp/m047-s05` and `.tmp/m047-s06` proof bundles plus fail-closed contract guards that future regressions can inspect before reopening scaffold/docs work.
requires:
  - slice: S05
    provides: The SQLite Todo scaffold, Docker packaging path, and fast package/tooling rails that S08 rebased onto truthful clustered read-route adoption.
  - slice: S06
    provides: The assembled docs/migration/closeout verifier structure and retained-bundle convention that S08 updated to replay the recovered Todo proof.
  - slice: S07
    provides: The shipped `HTTP.clustered(...)` compiler/runtime seam, continuity/runtime-name truth, and dedicated broader wrapper rail that S08 could adopt without reopening compiler/runtime work.
affects:
  []
key_files:
  - compiler/mesh-pkg/src/scaffold.rs
  - compiler/meshc/tests/tooling_e2e.rs
  - compiler/meshc/tests/support/m047_todo_scaffold.rs
  - compiler/meshc/tests/support/m046_route_free.rs
  - compiler/meshc/tests/e2e_m047_s05.rs
  - README.md
  - website/docs/docs/tooling/index.md
  - website/docs/docs/getting-started/clustered-example/index.md
  - website/docs/docs/distributed/index.md
  - website/docs/docs/distributed-proof/index.md
  - compiler/meshc/tests/e2e_m047_s06.rs
  - scripts/verify-m047-s05.sh
  - scripts/verify-m047-s06.sh
  - .gsd/PROJECT.md
key_decisions:
  - Dogfood the shipped wrapper narrowly in the public starter: keep `work.mpl` route-free and local mutating routes intact, and adopt `HTTP.clustered(1, handler)` only on `GET /todos` and `GET /todos/:id`.
  - Treat Docker cluster-port publication as evidence only and use a same-netns helper container running `/app/target/debug/meshc` from the cached builder image as the authoritative operator seam for `meshc cluster status|continuity`.
  - Keep public docs and closeout rails honest by claiming only the Todo starter's explicit-count single-node wrapper adoption and deferring default-count/two-node wrapper semantics to the existing S07 rail.
patterns_established:
  - When a new clustered surface is real but narrower than the general feature space, keep the route-free canonical examples first and dogfood the new syntax only on the fuller starter paths that the repo can prove end to end.
  - For single-node clustered-route proofs, diff continuity by `request_key` plus runtime name and assert `declared_handler_runtime_name`, `replication_count=1`, `replication_health=local_only`, and `fell_back_locally=true` instead of expecting replica activity.
  - For Docker operator proofs against loopback-advertised Mesh node names, run `meshc cluster` from a helper container that shares the target container's network namespace; keep host-published port artifacts, but do not treat them as the authoritative truth surface.
  - Make closeout scripts mechanically guard docs truth by replaying lower-level verifiers, retaining their proof bundles, and exact-checking stale wording such as `HTTP.clustered(...) is still not shipped` so adoption drift fails hard.
observability_surfaces:
  - cargo test -p mesh-pkg m047_s05 -- --nocapture
  - cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture
  - cargo test -p meshc --test e2e_m047_s05 m047_s05_todo_scaffold_clustered_list_route_truth_is_real_natively_and_in_container -- --nocapture
  - cargo test -p meshc --test e2e_m047_s05 -- --nocapture
  - cargo test -p meshc --test e2e_m047_s07 -- --nocapture
  - bash scripts/verify-m047-s05.sh
  - cargo test -p meshc --test e2e_m047_s06 m047_s06_ -- --nocapture
  - bash scripts/verify-m047-s06.sh
  - npm --prefix website run build
  - .tmp/m047-s05/verify/status.txt
  - .tmp/m047-s05/verify/phase-report.txt
  - .tmp/m047-s05/verify/retained-m047-s05-artifacts.manifest.txt
  - .tmp/m047-s05/todo-scaffold-clustered-route-truth-1775102250401339000/native-cluster-status.json
  - .tmp/m047-s05/todo-scaffold-clustered-route-truth-1775102250401339000/native-cluster-continuity-record.json
  - .tmp/m047-s05/todo-scaffold-clustered-route-truth-1775102250401339000/container/clustered-container-status.json
  - .tmp/m047-s05/todo-scaffold-clustered-route-truth-1775102250401339000/container/clustered-container-continuity-record.json
  - .tmp/m047-s05/todo-scaffold-clustered-route-truth-1775102250401339000/container/missing-cluster-port.timeout.txt
  - .tmp/m047-s06/verify/status.txt
  - .tmp/m047-s06/verify/phase-report.txt
  - .tmp/m047-s06/verify/retained-proof-bundle
drill_down_paths:
  - .gsd/milestones/M047/slices/S08/tasks/T01-SUMMARY.md
  - .gsd/milestones/M047/slices/S08/tasks/T02-SUMMARY.md
  - .gsd/milestones/M047/slices/S08/tasks/T03-SUMMARY.md
  - .gsd/milestones/M047/slices/S08/tasks/T04-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-02T04:07:22.221Z
blocker_discovered: false
---

# S08: Clustered route adoption in scaffold, docs, and closeout proof

**S08 adopted the shipped `HTTP.clustered(1, ...)` wrapper into the Todo starter, public docs, and assembled closeout rails, and proved the selected read routes natively and in Docker without displacing the canonical route-free `@cluster` story.**

## What Happened

S08 completed the public adoption layer that S07 intentionally deferred. The generated `meshc init --template todo-api` starter now keeps `work.mpl` on the route-free `@cluster pub fn sync_todos()` contract while wrapping only `GET /todos` and `GET /todos/:id` with explicit-count `HTTP.clustered(1, ...)`. The wrapped handlers are typed as `Request -> Response`, the mutating routes and `GET /health` stay local, and the generated README now explains that this is the truthful narrower dogfood surface for the shipped wrapper rather than a replacement for the canonical route-free clustered examples.

The slice also finished the proof story that S05/S06 still lacked. `compiler/meshc/tests/support/m047_todo_scaffold.rs` now provides single-node continuity/status helpers that can drive the generated Todo app both natively and in Docker, diff continuity snapshots by `request_key` plus runtime name, and assert the real wrapper boundary through `declared_handler_runtime_name=Api.Todos.handle_list_todos`, `replication_count=1`, `replication_health=local_only`, and `fell_back_locally=true`. The Docker proof no longer depends on a host-side operator handshake against a published cluster port that can EOF when the container advertises `name@127.0.0.1:port`; instead, the authoritative operator seam is a helper container sharing the target container's network namespace and running `/app/target/debug/meshc` from the cached `cluster-proof/Dockerfile` builder image. Published port artifacts are still retained as publication evidence, but the proof truth now comes from the same-netns operator query path.

Public authority surfaces were then rebased around that narrower, truthful adoption. `README.md`, `website/docs/docs/tooling/index.md`, `website/docs/docs/getting-started/clustered-example/index.md`, `website/docs/docs/distributed/index.md`, and `website/docs/docs/distributed-proof/index.md` now remove stale `HTTP.clustered(...)` non-goal language, keep the route-free `@cluster` surfaces first, describe the Todo starter's selected explicit-count clustered read routes honestly, and point default-count/two-node wrapper behavior back to the dedicated S07 rail (`cargo test -p meshc --test e2e_m047_s07 -- --nocapture`). `compiler/meshc/tests/e2e_m047_s06.rs` plus `scripts/verify-m047-s06.sh` now fail closed on any drift between those docs, the Todo starter contract, and the retained closeout bundle.

The assembled closeout rails are green again. `bash scripts/verify-m047-s05.sh` now replays S04, package/tooling/e2e/docs checks, retains the latest Todo proof directories and bundle pointers under `.tmp/m047-s05/verify/`, and exposes the timestamped `todo-scaffold-clustered-route-truth-*` bundle for drill-down. `bash scripts/verify-m047-s06.sh` wraps that recovered S05 proof, reruns the doc-authority contracts, rebuilds the docs site, and retains the assembled `.tmp/m047-s06/verify/retained-proof-bundle` that downstream milestone closeout work can trust.

### Operational Readiness (Q8)
- **Health signal:** `.tmp/m047-s05/verify/status.txt` is `ok`; the retained proof manifest points at `.tmp/m047-s05/todo-scaffold-clustered-route-truth-1775102250401339000/`, whose `native-cluster-status.json` and `container/clustered-container-status.json` both show `cluster_role=primary`, `promotion_epoch=0`, `replication_health=local_only`, and membership reduced to the single local node. The paired `native-cluster-continuity-record.json` and `container/clustered-container-continuity-record.json` both report `declared_handler_runtime_name=Api.Todos.handle_list_todos`, `replication_count=1`, `phase=completed`, `result=succeeded`, and `fell_back_locally=true`.
- **Failure signal:** stale public wording now fails `cargo test -p meshc --test e2e_m047_s06 m047_s06_ -- --nocapture` or `bash scripts/verify-m047-s06.sh`; broken starter generation fails the mesh-pkg/tooling rails; and Docker operator regressions surface through `.tmp/m047-s05/todo-scaffold-clustered-route-truth-1775102250401339000/container/missing-cluster-port.timeout.txt`, the published-port `*.ports.txt` / `*.inspect.json` artifacts, or a non-`ok` verify status.
- **Recovery procedure:** rerun `bash scripts/verify-m047-s05.sh` first, inspect `.tmp/m047-s05/verify/retained-m047-s05-artifacts.manifest.txt` to locate the latest `todo-scaffold-clustered-route-truth-*` bundle, then inspect the native/container status + continuity JSON and the container logs before changing docs or scaffold wording. If the proof artifacts show the wrapper seam itself is wrong rather than the adoption layer, rerun `cargo test -p meshc --test e2e_m047_s07 -- --nocapture` before touching S08-owned files.
- **Monitoring gaps:** the public starter still proves only the explicit-count single-node adoption path (`HTTP.clustered(1, ...)` on selected read routes with `local_only` continuity) and intentionally does not claim default-count or broader two-node wrapper behavior; Docker proof still relies on the same-netns helper-container seam instead of a runtime-owned in-container operator entrypoint.


## Verification

Ran every slice-level verification command from the plan and all passed.

- `cargo test -p mesh-pkg m047_s05 -- --nocapture` — passed (`2 passed`) in 1.9s.
- `cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture` — passed (`2 passed`) in 9.8s.
- `cargo test -p meshc --test e2e_m047_s05 m047_s05_todo_scaffold_clustered_list_route_truth_is_real_natively_and_in_container -- --nocapture` — passed (`1 passed`) in 69.2s.
- `cargo test -p meshc --test e2e_m047_s05 -- --nocapture` — passed (`8 passed`) in 76.7s.
- `cargo test -p meshc --test e2e_m047_s07 -- --nocapture` — passed (`3 passed`) in 23.9s.
- `bash scripts/verify-m047-s05.sh` — passed in 347.7s and wrote `.tmp/m047-s05/verify/status.txt=ok`, a complete phase report, and the retained S05 proof bundle pointer.
- `cargo test -p meshc --test e2e_m047_s06 m047_s06_ -- --nocapture` — passed (`3 passed`) in 3.3s.
- `bash scripts/verify-m047-s06.sh` — passed in 351.3s and reported `verify-m047-s06: ok` while retaining `.tmp/m047-s06/verify/retained-proof-bundle`.
- `npm --prefix website run build` — passed in 55.4s.

The retained observability artifacts confirm the slice-specific runtime truth: both the native and Docker single-node proofs recorded `Api.Todos.handle_list_todos` as the continuity runtime name with `replication_count=1`, `replication_health=local_only`, and `fell_back_locally=true`, while the missing-cluster-port negative path retained timeout artifacts instead of silently degrading into an unproven operator story.

## Requirements Advanced

- R008 — S08 reworked the Todo scaffold, README, VitePress docs, and assembled closeout rails so Mesh's production-shaped launchability story now includes truthful native + Docker proof for selected clustered read routes instead of stale non-goal language.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

None to slice scope. The only noteworthy harness behavior is deliberate: several negative-path checks panic inside `catch_unwind` to prove fail-closed behavior (for malformed JSON/status helpers and unpublished Docker cluster ports), so panic text appears in `--nocapture` output even though the tests pass.

## Known Limitations

The Todo starter intentionally adopts only the explicit-count single-node wrapper path: `GET /todos` and `GET /todos/:id` use `HTTP.clustered(1, ...)`, while `GET /health` and the mutating routes stay local. Default-count and two-node wrapper behavior still belongs to the dedicated S07 rail.

Docker clustered-route operator proof remains same-netns-helper based. Because a container advertising `name@127.0.0.1:port` can EOF on host-side `meshc cluster status|continuity` despite being healthy, the retained S08 proof queries the authoritative listener from a helper container that shares the target container's network namespace.

## Follow-ups

If the starter ever needs broader clustered-route adoption, land that proof first on the dedicated S07-style runtime rails and only then widen the public starter/docs contract; do not let the Todo starter or public docs imply default-count or multi-node behavior they do not prove.

Consider extracting a purpose-built operator helper image or runtime-owned container operator entrypoint if the current reuse of the cached `cluster-proof/Dockerfile` builder image becomes a maintenance burden.

## Files Created/Modified

- `compiler/mesh-pkg/src/scaffold.rs` — Rebased the Todo scaffold generator and README text onto explicit-count clustered read routes, typed wrapped handlers, and updated source-contract assertions.
- `compiler/meshc/tests/tooling_e2e.rs` — Updated the fast scaffold-init rails to assert the new Todo wrapper contract and truthful generated README/source surfaces.
- `compiler/meshc/tests/support/m047_todo_scaffold.rs` — Added native/Docker single-node cluster helpers, same-netns helper-container operator queries, continuity waiters, Docker output packaging support, and retained artifact helpers for the Todo scaffold proof.
- `compiler/meshc/tests/support/m046_route_free.rs` — Extended shared continuity diff helpers so S08 can compare before/after records by `request_key` and runtime name without depending on list order.
- `compiler/meshc/tests/e2e_m047_s05.rs` — Rebased the Todo scaffold end-to-end rail onto real clustered read-route adoption, same-netns Docker operator proof, and fail-closed negative publication checks.
- `README.md` — Removed stale wrapper non-goal wording and documented the Todo starter's explicit-count clustered read-route adoption while keeping route-free examples canonical.
- `website/docs/docs/tooling/index.md` — Updated tooling guidance to describe the truthful Todo wrapper surface and point broader wrapper semantics at S07.
- `website/docs/docs/getting-started/clustered-example/index.md` — Kept route-free `@cluster` examples first while layering in the Todo starter as the fuller example with selected explicit-count clustered read routes.
- `website/docs/docs/distributed/index.md` — Rebased the distributed guide and verifier map around the updated S04/S05/S06/S07 authority boundaries.
- `website/docs/docs/distributed-proof/index.md` — Updated the proof guide to remove stale non-goal language and describe the Todo starter's clustered-route adoption plus verifier layering truthfully.
- `compiler/meshc/tests/e2e_m047_s06.rs` — Strengthened the docs-authority contract tests so stale `HTTP.clustered(...)` non-goal language and overclaims now fail closed.
- `scripts/verify-m047-s05.sh` — Recovered the Todo assembled verifier so it replays S04, package/tooling/e2e/docs rails, retains fresh proof artifacts, and emits an authoritative bundle pointer.
- `scripts/verify-m047-s06.sh` — Wrapped the recovered S05 proof, replayed doc-authority checks, rebuilt docs, and retained an assembled M047 closeout bundle for milestone-level proof.
- `.gsd/PROJECT.md` — Updated project state to reflect that S08 completed Todo/docs/closeout adoption of the shipped clustered route wrapper.
