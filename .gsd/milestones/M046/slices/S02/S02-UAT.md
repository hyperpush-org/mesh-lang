# S02: Runtime-owned startup trigger and route-free status contract — UAT

**Milestone:** M046
**Written:** 2026-03-31T19:23:37.983Z

# S02: Runtime-owned startup trigger and route-free status contract — UAT

**Milestone:** M046
**Written:** 2026-03-31

## UAT Type

- UAT mode: mixed runtime + artifact-driven
- Why this mode is sufficient: S02 changes the compiler/codegen startup hook path, `mesh-rt` startup behavior, CLI inspection output, and a dual-node route-free startup proof. The truthful acceptance surface is the exact Cargo rails that exercise those boundaries end to end and retain artifacts when the runtime path drifts.

## Preconditions

- Run from the repository root with Cargo available.
- Loopback ports on `127.0.0.1` and `::1` must be available for temporary dual-node tests.
- No external services are required.
- `target/` must be writable so the tests can build temporary binaries and `mesh-rt` artifacts.

## Smoke Test

Run `cargo test -p mesh-rt startup_work_ -- --nocapture`.

**Expected:** 5 tests run and pass, proving the runtime-owned startup registry keeps deterministic identities, does not falsely time out single-node clustered boots, spawns keepalive only when needed, and skips startup submission on standby authority.

## Test Cases

### 1. Codegen emits startup hooks only for clustered work

1. Run `cargo test -p meshc --test e2e_m046_s02 m046_s02_codegen_ -- --nocapture`.
2. Confirm the rail reports `running 4 tests` and all pass.
3. **Expected:**
   - Work builds emit `mesh_register_startup_work` and `mesh_trigger_startup_work` in the main wrapper.
   - Service-only builds keep declared-handler registration but omit both startup hooks.
   - Source-declared and manifest-declared work share the same startup runtime identity.
   - Missing startup declared-handler metadata fails explicitly instead of silently dropping the startup path.

### 2. Runtime startup registry and keepalive behavior stay fail-closed

1. Run `cargo test -p mesh-rt startup_work_ -- --nocapture`.
2. Confirm the rail reports `running 5 tests` and all pass.
3. **Expected:**
   - Duplicate startup registrations dedupe on runtime name and produce one stable deterministic identity.
   - Single-node clustered startup succeeds without a false convergence timeout.
   - Peer flap/convergence timeout stays explicit.
   - Cluster-mode keepalive spawns only once when startup registrations exist.
   - Standby authority keeps the route-free keepalive but skips auto-submitting startup work.

### 3. Single-node route-free startup work is discoverable entirely through CLI surfaces

1. Run `cargo test -p meshc --test e2e_m046_s02 m046_s02_cli_route_free_startup_work_is_discoverable_from_list_and_single_record_output -- --nocapture`.
2. Let the test boot the temporary route-free runtime binary.
3. **Expected:**
   - `meshc cluster status --json` reports the local node and healthy standalone authority truth.
   - `meshc cluster continuity --json` list output shows startup records with `declared_handler_runtime_name` populated.
   - Single-record continuity output returns the deterministic `request_key`, `phase=completed`, `result=succeeded`, and the matching runtime name.
   - Human-readable continuity output also prints `declared_handler_runtime_name` so the route-free workflow does not require JSON-only tooling.

### 4. Invalid request/auth paths stay explicit on route-free CLI inspection

1. Run `cargo test -p meshc --test e2e_m046_s02 m046_s02_cli_continuity_failures_stay_explicit_for_invalid_request_and_auth -- --nocapture`.
2. **Expected:**
   - Invalid request keys fail explicitly through `meshc cluster continuity`.
   - Missing or wrong auth stays an explicit CLI/runtime failure.
   - The rail never falls back to an app-owned HTTP route for status truth.

### 5. Simultaneous two-node route-free startup dedupes to one logical startup run

1. Run `cargo test -p meshc --test e2e_m046_s02 m046_s02_cli_tiny_route_free_startup_dedupes_on_two_nodes -- --nocapture`.
2. Let the test boot both nodes at once.
3. **Expected:**
   - The temporary fixture source contains only `clustered(work)`, `Node.start_from_env()`, and trivial arithmetic work (`1 + 1`); it contains no `/work`, `/status`, `HTTP.serve(...)`, `Continuity.submit_declared_work(...)`, or `Continuity.mark_completed(...)`.
   - `meshc cluster status --json` on both nodes reports the same two-node membership and the expected primary/standby authority roles.
   - `meshc cluster continuity --json` list output on both nodes shows exactly one record for `Work.execute_declared_work`.
   - The deterministic startup record reaches `phase=completed`, `result=succeeded`, mirrored replica status, and identical `request_key` on both nodes.
   - `meshc cluster diagnostics --json` surfaces `startup_trigger` and `startup_completed` with no `startup_rejected` or `startup_convergence_timeout` transition for that request key.

### 6. Retained M044 clustered-app rails stay green under the new startup/runtime surfaces

1. Run `cargo test -p meshc --test e2e_m044_s01 m044_s01_ -- --nocapture`.
2. Confirm the rail reports `running 15 tests` and all pass.
3. Run `cargo test -p meshc --test e2e_m044_s02 m044_s02_ -- --nocapture`.
4. Confirm the rail reports `running 9 tests` and all pass.
5. Run `cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture`.
6. Confirm the rail reports `running 2 tests` and all pass.
7. **Expected:** the older declared-handler metadata, runtime-owned submit path, and operator CLI rails remain green, proving S02 extended the clustered runtime boundary instead of forking it.

## Edge Cases

### Service declarations never auto-trigger at startup

1. Use the codegen rail above or run `cargo test -p meshc --test e2e_m046_s02 m046_s02_codegen_service_build_omits_startup_runtime_hooks -- --nocapture`.
2. **Expected:** service-only LLVM contains declared-handler registration but omits `mesh_register_startup_work` and `mesh_trigger_startup_work` entirely.

### Simultaneous connect races do not strand remote startup dispatch on dead half-connections

1. Use the two-node simultaneous-boot rail above.
2. **Expected:** the startup record completes instead of failing with `mesh node spawn failed ... write_error` or `declared_work_remote_spawn_failed:...`, proving duplicate session admission converges both nodes on one live transport.

## Failure Signals

- The codegen rail stops finding ordered `mesh_register_startup_work` / `mesh_trigger_startup_work` calls for clustered work, or starts finding them in service-only builds.
- `cargo test -p mesh-rt startup_work_ -- --nocapture` fails on standby skip, keepalive, or convergence behavior.
- Route-free CLI rails lose `declared_handler_runtime_name`, stop completing the deterministic startup record, or require an app-owned route to inspect status.
- The simultaneous two-node rail regresses to `mesh node spawn failed ... write_error`, `declared_work_remote_spawn_failed`, or any `startup_rejected` / `startup_convergence_timeout` transition for the selected startup request.
- Any retained M044 declared-handler/operator suite turns red, which would mean S02 drifted the established clustered-app surface.

## Requirements Proved By This UAT

- R087 — runtime/tooling can trigger clustered work on startup and expose proof entirely through route-free runtime/tooling surfaces.
- R086 / R091 / R092 / R093 are advanced by this UAT, but final milestone-scale validation still depends on the later `tiny-cluster/`, rebuilt `cluster-proof/`, and docs/alignment slices.

## Notes for Tester

If the simultaneous two-node rail regresses, inspect the retained `.tmp/m046-s02/...` bundle and both node logs before changing the proof fixture. The red path that mattered in S02 was transport admission inside `compiler/mesh-rt/src/dist/node.rs`, not the startup registration or CLI rendering layers.
