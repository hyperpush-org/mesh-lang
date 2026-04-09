---
id: T02
parent: S08
milestone: M047
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/tests/support/m046_route_free.rs", "compiler/meshc/tests/support/m047_todo_scaffold.rs", "compiler/meshc/tests/e2e_m047_s05.rs", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Require a new continuity request key plus matching runtime name and replication_count=1 before accepting wrapped GET /todos proof evidence.", "Validate clustered Todo helper env locally and pass Docker container cluster settings through explicit `docker create -e KEY=value` args instead of `Command::env(...)` on the Docker CLI process.", "Treat the Docker host-published cluster-port handshake failure as a blocker-worthy retained artifact instead of downgrading it to a skipped proof."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Pre-change baseline `cargo test -p meshc --test e2e_m047_s05 -- --nocapture` passed. After the edits, the helper-level clustered env/diff rails compiled and passed, but the isolated clustered native+Docker proof still failed on the Docker operator handshake (`meshc cluster status ... unavailable: handshake with 127.0.0.1:<port> failed: send_name failed: unexpected end of file`). Because that blocker remained unresolved, the full post-change S05/S07/script verification gate was not rerun."
completed_at: 2026-04-02T02:34:00.163Z
blocker_discovered: true
---

# T02: Added single-node clustered-route proof helpers for the Todo scaffold and captured the Docker host-handshake blocker that still prevents S05 closeout.

> Added single-node clustered-route proof helpers for the Todo scaffold and captured the Docker host-handshake blocker that still prevents S05 closeout.

## What Happened
---
id: T02
parent: S08
milestone: M047
key_files:
  - compiler/meshc/tests/support/m046_route_free.rs
  - compiler/meshc/tests/support/m047_todo_scaffold.rs
  - compiler/meshc/tests/e2e_m047_s05.rs
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Require a new continuity request key plus matching runtime name and replication_count=1 before accepting wrapped GET /todos proof evidence.
  - Validate clustered Todo helper env locally and pass Docker container cluster settings through explicit `docker create -e KEY=value` args instead of `Command::env(...)` on the Docker CLI process.
  - Treat the Docker host-published cluster-port handshake failure as a blocker-worthy retained artifact instead of downgrading it to a skipped proof.
duration: ""
verification_result: mixed
completed_at: 2026-04-02T02:34:00.174Z
blocker_discovered: true
---

# T02: Added single-node clustered-route proof helpers for the Todo scaffold and captured the Docker host-handshake blocker that still prevents S05 closeout.

**Added single-node clustered-route proof helpers for the Todo scaffold and captured the Docker host-handshake blocker that still prevents S05 closeout.**

## What Happened

Extended the shared route-free support with runtime/count-aware continuity diff helpers, extended the Todo scaffold harness with clustered runtime config validation plus native/Docker cluster-mode helper surfaces, and added focused clustered-route proof coverage in e2e_m047_s05. The native one-node clustered GET /todos proof now retains truthful continuity metadata for Api.Todos.handle_list_todos with replication_count=1, local_only health, and fell_back_locally=true. The Docker half remains blocked: after fixing container env injection, the generated image boots in cluster mode and publishes both HTTP and cluster ports, but host-side `meshc cluster status` against the published cluster port still fails `send_name failed: unexpected end of file`, so the task concluded as blocker discovery rather than full closeout.

## Verification

Pre-change baseline `cargo test -p meshc --test e2e_m047_s05 -- --nocapture` passed. After the edits, the helper-level clustered env/diff rails compiled and passed, but the isolated clustered native+Docker proof still failed on the Docker operator handshake (`meshc cluster status ... unavailable: handshake with 127.0.0.1:<port> failed: send_name failed: unexpected end of file`). Because that blocker remained unresolved, the full post-change S05/S07/script verification gate was not rerun.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m047_s05 -- --nocapture` | 0 | ✅ pass | 89000ms |
| 2 | `cargo test -p meshc --test e2e_m047_s05 m047_s05_cluster_runtime_helpers_reject_missing_mesh_env_values -- --nocapture` | 0 | ✅ pass | 12500ms |
| 3 | `cargo test -p meshc --test e2e_m047_s05 m047_s05_continuity_diff_helpers_ -- --nocapture && cargo test -p meshc --test e2e_m047_s05 m047_s05_http_snapshot_helpers_fail_closed_on_bad_json_and_status -- --nocapture` | 0 | ✅ pass | 8300ms |
| 4 | `cargo test -p meshc --test e2e_m047_s05 m047_s05_todo_scaffold_clustered_list_route_truth_is_real_natively_and_in_container -- --nocapture` | 101 | ❌ fail | 486560ms |
| 5 | `cargo test -p meshc --test e2e_m047_s05 m047_s05_todo_scaffold_clustered_list_route_truth_is_real_natively_and_in_container -- --nocapture` | 101 | ❌ fail | 624550ms |


## Deviations

Added a focused clustered-route proof test beside the existing broad CRUD/rate-limit/restart rail instead of folding all assertions into the large original S05 runtime-truth test. `scripts/verify-m047-s05.sh` was left structurally unchanged because it already delegates to the full `e2e_m047_s05` target, and the Docker blocker surfaced before any bundle-shape rebaseline was justified.

## Known Issues

Docker single-node cluster proof is still blocked. The generated container now boots in cluster mode and publishes the requested host cluster port, but `meshc cluster status <node@127.0.0.1:port> --json` fails with `send_name failed: unexpected end of file`. The retained bundle `.tmp/m047-s05/todo-scaffold-clustered-route-truth-1775096433580666000/container/` contains `clustered-container-status.timeout.txt`, `clustered-container-cluster-port.ports.txt`, `clustered-container-cluster-port.host-port.log`, `clustered-container-cluster-port.inspect.json`, and `clustered-container.combined.log` for recovery.

## Files Created/Modified

- `compiler/meshc/tests/support/m046_route_free.rs`
- `compiler/meshc/tests/support/m047_todo_scaffold.rs`
- `compiler/meshc/tests/e2e_m047_s05.rs`
- `.gsd/KNOWLEDGE.md`


## Deviations
Added a focused clustered-route proof test beside the existing broad CRUD/rate-limit/restart rail instead of folding all assertions into the large original S05 runtime-truth test. `scripts/verify-m047-s05.sh` was left structurally unchanged because it already delegates to the full `e2e_m047_s05` target, and the Docker blocker surfaced before any bundle-shape rebaseline was justified.

## Known Issues
Docker single-node cluster proof is still blocked. The generated container now boots in cluster mode and publishes the requested host cluster port, but `meshc cluster status <node@127.0.0.1:port> --json` fails with `send_name failed: unexpected end of file`. The retained bundle `.tmp/m047-s05/todo-scaffold-clustered-route-truth-1775096433580666000/container/` contains `clustered-container-status.timeout.txt`, `clustered-container-cluster-port.ports.txt`, `clustered-container-cluster-port.host-port.log`, `clustered-container-cluster-port.inspect.json`, and `clustered-container.combined.log` for recovery.
