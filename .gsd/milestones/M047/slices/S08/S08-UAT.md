# S08: Clustered route adoption in scaffold, docs, and closeout proof — UAT

**Milestone:** M047
**Written:** 2026-04-02T04:07:22.222Z

# S08: Clustered route adoption in scaffold, docs, and closeout proof — UAT

**Milestone:** M047
**Written:** 2026-04-02

## UAT Type

- UAT mode: mixed (scaffold-generation contract + native clustered runtime proof + Docker clustered runtime proof + docs/closeout replay)
- Why this mode is sufficient: S08 only counts as done if the public starter, docs, and retained closeout rails all describe the shipped wrapper truthfully and the selected Todo read routes prove real clustered continuity natively and in Docker.

## Preconditions

- Run from the repo root.
- Rust workspace dependencies are installed and the repo builds locally.
- Docker is installed and the daemon is running.
- Website dependencies are already installed (`npm --prefix website run build` must be runnable).
- Local loopback ports are available for one native Todo app, one containerized Todo app, and the container-published cluster port.
- Do **not** run `bash scripts/verify-m047-s05.sh`, `bash scripts/verify-m047-s06.sh`, or `npm --prefix website run build` in parallel with another VitePress build or another local cluster harness.

## Smoke Test

1. Run `cargo test -p mesh-pkg m047_s05 -- --nocapture`.
2. Run `cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture`.
3. Run `cargo test -p meshc --test e2e_m047_s05 -- --nocapture`.
4. Run `cargo test -p meshc --test e2e_m047_s07 -- --nocapture`.
5. Run `bash scripts/verify-m047-s05.sh`.
6. Run `cargo test -p meshc --test e2e_m047_s06 m047_s06_ -- --nocapture`.
7. Run `bash scripts/verify-m047-s06.sh`.
8. Run `npm --prefix website run build`.
9. **Expected:** all commands pass, `.tmp/m047-s05/verify/status.txt` and `.tmp/m047-s06/verify/status.txt` both read `ok`, and the docs build completes without reintroducing stale `HTTP.clustered(...)` non-goal language.

## Test Cases

### 1. The generated Todo starter adopts the wrapper narrowly and truthfully

1. Run `cargo test -p mesh-pkg m047_s05 -- --nocapture`.
2. Run `cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture`.
3. **Expected:** both rails pass.
4. Inspect the generated scaffold contract indirectly through the retained assertions or by generating a fresh starter and opening `work.mpl`, `api/router.mpl`, `api/todos.mpl`, and `README.md`.
5. **Expected:**
   - `work.mpl` keeps `@cluster pub fn sync_todos()` route-free.
   - `api/router.mpl` uses `HTTP.clustered(1, handle_list_todos)` only on `GET /todos` and `HTTP.clustered(1, handle_get_todo)` only on `GET /todos/:id`.
   - `GET /health`, `POST /todos`, `PUT /todos/:id`, and `DELETE /todos/:id` do **not** use `HTTP.clustered(...)`.
   - `handle_list_todos` and `handle_get_todo` are explicitly typed as `Request -> Response`.
   - The generated README explains that the Todo starter dogfoods the shipped wrapper narrowly while the route-free `@cluster` surfaces stay canonical.

### 2. Native single-node clustered read-route continuity is real

1. Run `cargo test -p meshc --test e2e_m047_s05 m047_s05_todo_scaffold_clustered_list_route_truth_is_real_natively_and_in_container -- --nocapture`.
2. Open `.tmp/m047-s05/verify/retained-m047-s05-artifacts.manifest.txt` and locate the latest `todo-scaffold-clustered-route-truth-*` directory.
3. In that directory, open `native-cluster-status.json`, `native-cluster-continuity-before.json`, `native-cluster-continuity-after.json`, `native-cluster-continuity-record.json`, and `native-cluster-todos.json`.
4. **Expected:**
   - `native-cluster-status.json` reports one node only, `cluster_role = primary`, `promotion_epoch = 0`, and `replication_health = local_only`.
   - `native-cluster-todos.json` returns the seeded Todo via `GET /todos`.
   - `native-cluster-continuity-record.json` reports `declared_handler_runtime_name = Api.Todos.handle_list_todos`, `replication_count = 1`, `phase = completed`, `result = succeeded`, `routed_remotely = false`, and `fell_back_locally = true`.
   - The `request_key` in the continuity record matches the new key introduced between the `before` and `after` continuity snapshots.

### 3. Docker single-node clustered read-route continuity is real

1. Run `cargo test -p meshc --test e2e_m047_s05 -- --nocapture` or `bash scripts/verify-m047-s05.sh`.
2. In the latest retained `todo-scaffold-clustered-route-truth-*` directory, open:
   - `container/clustered-container-http-port.inspect.json`
   - `container/clustered-container-cluster-port.inspect.json`
   - `container/clustered-container-status.json`
   - `container/clustered-container-continuity-record.json`
   - `container/clustered-container-todos.json`
   - `container/clustered-container.combined.log`
3. **Expected:**
   - The container publishes both the HTTP port and the requested cluster port.
   - `clustered-container-status.json` shows one-node membership with `replication_health = local_only`.
   - `clustered-container-continuity-record.json` reports `declared_handler_runtime_name = Api.Todos.handle_list_todos`, `replication_count = 1`, `phase = completed`, `result = succeeded`, and `fell_back_locally = true`.
   - The container log shows clustered bootstrap (`runtime bootstrap mode=cluster`) and a healthy HTTP runtime startup.
   - The JSON returned from `GET /todos` includes the Todo created inside the container run.

### 4. Docker proof fails closed when cluster publication is missing

1. In the same retained `todo-scaffold-clustered-route-truth-*` directory, open:
   - `container/missing-cluster-port-health.json`
   - `container/missing-cluster-port.timeout.txt`
   - `container/missing-cluster-port.ports.txt`
   - `container/missing-cluster-port.inspect.json`
   - `container/missing-cluster-port.combined.log`
2. **Expected:**
   - The container itself is healthy enough to answer `/health`.
   - The cluster-port proof still fails hard because the cluster port was not published.
   - The timeout artifact exists and there is no silent downgrade to an unproven operator path.

### 5. Docs and assembled closeout rails tell the same story as the starter

1. Run `cargo test -p meshc --test e2e_m047_s06 m047_s06_ -- --nocapture`.
2. Run `bash scripts/verify-m047-s06.sh`.
3. Run `npm --prefix website run build`.
4. Inspect:
   - `.tmp/m047-s06/verify/status.txt`
   - `.tmp/m047-s06/verify/phase-report.txt`
   - `.tmp/m047-s06/verify/retained-proof-bundle/`
5. **Expected:**
   - The docs-authority contract test passes.
   - `verify-m047-s06.sh` reports `verify-m047-s06: ok`.
   - The retained proof bundle contains the delegated S05 proof plus the S06 docs/build phases.
   - README and VitePress docs do **not** claim that `HTTP.clustered(...)` is still unshipped.
   - README and VitePress docs do **not** imply that the Todo starter proves default-count or broader two-node wrapper behavior; they point that authority to `cargo test -p meshc --test e2e_m047_s07 -- --nocapture`.

## Edge Cases

### Intentional panic text appears in passing `--nocapture` runs

1. Re-run `cargo test -p meshc --test e2e_m047_s05 -- --nocapture`.
2. **Expected:** panic text may appear for negative-path checks (malformed JSON/status helpers, unpublished Docker cluster ports), but the overall test result still ends with `ok` because those panics are caught and asserted as fail-closed behavior.

### Host-side operator queries against a Docker loopback node are not authoritative

1. When debugging Docker proof drift, inspect the retained container artifacts first.
2. **Expected:** the authoritative operator proof uses the same-netns helper-container seam; do **not** replace it with a host-side `meshc cluster status|continuity` call against the published cluster port if the container advertises `name@127.0.0.1:port`.

### Broader wrapper semantics remain outside the starter contract

1. Run `cargo test -p meshc --test e2e_m047_s07 -- --nocapture`.
2. **Expected:** S07 remains the proof surface for default-count and two-node `HTTP.clustered(...)` behavior. A passing S08 rail does not imply the Todo starter or public docs already cover that broader semantic space.

## Notes for Tester

- The fastest drill-down path after `bash scripts/verify-m047-s05.sh` is:
  1. open `.tmp/m047-s05/verify/status.txt`
  2. open `.tmp/m047-s05/verify/retained-m047-s05-artifacts.manifest.txt`
  3. inspect the latest `todo-scaffold-clustered-route-truth-*` directory named there
- The fastest drill-down path after `bash scripts/verify-m047-s06.sh` is:
  1. open `.tmp/m047-s06/verify/status.txt`
  2. open `.tmp/m047-s06/verify/phase-report.txt`
  3. inspect `.tmp/m047-s06/verify/retained-proof-bundle/`
- Keep website builds serial. Running `npm --prefix website run build` concurrently with the verify scripts can create false VitePress temp-dir failures.

