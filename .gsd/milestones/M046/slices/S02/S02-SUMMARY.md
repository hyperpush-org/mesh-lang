---
id: S02
parent: M046
milestone: M046
provides:
  - Runtime-owned startup-work registration and trigger hooks threaded through `meshc`/codegen for clustered work only.
  - A `mesh-rt` startup path that derives deterministic startup identities, waits boundedly for peer stability, submits through declared-work continuity, and keeps route-free clustered binaries alive long enough for inspection.
  - A route-free CLI inspection contract where `meshc cluster continuity` exposes `declared_handler_runtime_name` in list and single-record modes and `meshc cluster diagnostics` captures startup lifecycle truth.
  - A dual-node route-free startup proof that boots both nodes simultaneously, dedupes one logical startup run, and verifies completion entirely through `meshc cluster status|continuity|diagnostics`.
requires:
  - slice: S01
    provides: The shared manifest+source clustered execution plan plus declared-handler runtime registration identity that S02 consumes for startup registration and deterministic runtime-name discovery.
affects:
  - S03
  - S04
  - S05
  - S06
key_files:
  - compiler/meshc/src/main.rs
  - compiler/mesh-codegen/src/codegen/mod.rs
  - compiler/mesh-codegen/src/codegen/intrinsics.rs
  - compiler/mesh-rt/src/dist/node.rs
  - compiler/mesh-rt/src/dist/continuity.rs
  - compiler/mesh-rt/src/dist/operator.rs
  - compiler/meshc/src/cluster.rs
  - compiler/meshc/tests/e2e_m046_s02.rs
  - .gsd/PROJECT.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Register and trigger startup work by declared handler runtime name so source-declared and manifest-declared work share one runtime identity surface.
  - Keep runtime-owned startup keepalive even for route-free clustered binaries, but auto-submit startup work only from primary authority so simultaneous boots converge on one logical startup run.
  - Expose `declared_handler_runtime_name` on `meshc cluster continuity` list and single-record output so route-free tooling can discover startup work without app routes.
  - Resolve simultaneous duplicate node connects deterministically by local/remote node name so both nodes keep the same live transport instead of stranded half-connections.
patterns_established:
  - Use declared handler runtime names as the stable join point between compiler planning, runtime startup identity, and CLI discovery surfaces.
  - Spawn runtime-owned keepalive before startup validation/dispatch so route-free clustered binaries remain inspectable even when startup work fails early.
  - Treat route-free clustered startup as a continuity-backed runtime concern: deterministic request identity, bounded peer convergence, explicit diagnostics, and no app-owned trigger/status shims.
  - When simultaneous discovery can create duplicate transports, make session admission converge deterministically on one connection instead of keeping the first local session by remote name alone.
observability_surfaces:
  - `meshc cluster status --json` for truthful membership and authority during route-free startup.
  - `meshc cluster continuity --json` and human output now expose `declared_handler_runtime_name`, deterministic `request_key`, owner/replica, phase/result, and explicit errors for startup work.
  - `meshc cluster diagnostics --json` records startup lifecycle transitions including registration, keepalive, trigger, completion, standby skip, rejection, and convergence timeout.
  - `compiler/meshc/tests/e2e_m046_s02.rs` retains per-node stdout/stderr plus CLI JSON/log artifacts under `.tmp/m046-s02/...` when the route-free startup proof fails.
drill_down_paths:
  - .gsd/milestones/M046/slices/S02/tasks/T01-SUMMARY.md
  - .gsd/milestones/M046/slices/S02/tasks/T02-SUMMARY.md
  - .gsd/milestones/M046/slices/S02/tasks/T03-SUMMARY.md
  - .gsd/milestones/M046/slices/S02/tasks/T04-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-31T19:23:37.969Z
blocker_discovered: false
---

# S02: Runtime-owned startup trigger and route-free status contract

**S02 moved clustered startup triggering and startup-state inspection onto runtime/tooling-owned surfaces, then proved a simultaneous two-node route-free startup run can be discovered and verified entirely through `meshc cluster ...` without app-owned submit or status routes.**

## What Happened

S02 completed the runtime-owned startup half of M046. T01 threaded clustered work-only startup registrations through `meshc` planning and codegen so `generate_main_wrapper(...)` now registers declared handlers, emits `mesh_register_startup_work(...)` only for clustered work declarations, runs `mesh_main`, then calls `mesh_trigger_startup_work(...)` before handing control to the scheduler. Service call/cast declarations stay on the declared-handler path but never reach the startup hook.

T02 moved the startup trigger itself into `mesh-rt`. The runtime now keeps an ordered startup-work registry keyed by declared handler runtime name, derives deterministic startup `request_key` / `payload_hash` values from that runtime name, waits boundedly for peer stability only after a peer has actually been observed, submits through the existing declared-work continuity path, records explicit startup diagnostics, and spawns a runtime-owned keepalive actor so route-free clustered binaries stay inspectable even when startup work is rejected. The primary/standby contract is now explicit: standby nodes keep the runtime-owned keepalive for inspection but skip startup submission, so simultaneous boots converge on one logical startup run instead of both authorities racing the same key.

T03 completed the tooling side by surfacing `declared_handler_runtime_name` on `meshc cluster continuity` JSON and human-readable output. That makes startup work discoverable from runtime-owned tooling alone: list mode finds the runtime name, single-record mode drills into the deterministic `request_key`, and invalid request/auth paths stay explicit instead of pushing the proof back toward app-owned routes.

T04 closed the slice with a tiny route-free startup proof. The temp-project fixture contains only `clustered(work)`, `Node.start_from_env()`, and trivial arithmetic work (`1 + 1`). It has no `/work`, no `/status`, no `HTTP.serve(...)`, and no explicit `Continuity.submit_declared_work(...)` or `Continuity.mark_completed(...)` calls. The dual-node proof boots both nodes at once, discovers the single deterministic startup record through `meshc cluster continuity`, verifies completion and dedupe on both nodes, and checks startup diagnostics purely through `meshc cluster status|continuity|diagnostics`.

Closeout exposed one real runtime bug rather than an artifact-only miss: simultaneous two-node boot could create duplicate node sessions where each side kept its own accepted half-connection and dropped the opposite client half. That stranded control traffic on dead transports and made the first runtime-owned remote spawn fail with `mesh node spawn failed ... write_error`. The fix stayed in `mesh-rt`: duplicate session admission now resolves deterministically by local/remote node name so the earlier-sorting node keeps the outgoing session and the later-sorting node keeps the incoming session, which makes both nodes converge on one live transport without weakening the simultaneous-boot proof.

### Operational Readiness

- **Health signal:** `meshc cluster status --json` reports truthful membership/authority, `meshc cluster continuity --json` lists the deterministic startup record with `declared_handler_runtime_name`, and `meshc cluster diagnostics --json` records `startup_registered`, `startup_keepalive`, `startup_trigger`, `startup_completed`, `startup_skipped`, and explicit failure transitions.
- **Failure signal:** startup convergence timeouts, missing handler metadata, auth failures, invalid request keys, and remote spawn transport failures all surface as explicit rejected continuity/diagnostic states rather than silent success or an app fallback route.
- **Recovery procedure:** if a route-free startup proof goes red, replay `cargo test -p mesh-rt startup_work_ -- --nocapture`, then `cargo test -p meshc --test e2e_m046_s02 m046_s02_cli_ -- --nocapture`, then the full `m046_s02_` rail and inspect the retained `.tmp/m046-s02/...` artifact bundle plus both node logs/CLI JSON. For simultaneous-boot failures, start in `compiler/mesh-rt/src/dist/node.rs` session admission before reopening startup registration or CLI rendering.
- **Monitoring gaps:** S02 proves startup trigger/status truth and route-free inspection, but it does not yet ship the repo-level `tiny-cluster/` package or the rebuilt packaged `cluster-proof/` surface, so route-free failover and public-doc alignment remain open to later slices.

## Verification

Ran the exact slice verification contract from the plan and all rails passed.

- `cargo test -p meshc --test e2e_m046_s02 m046_s02_codegen_ -- --nocapture` — 4 passed
- `cargo test -p meshc --test e2e_m044_s02 m044_s02_metadata_ -- --nocapture` — 4 passed
- `cargo test -p mesh-rt startup_work_ -- --nocapture` — 5 passed
- `cargo test -p meshc --test e2e_m046_s02 m046_s02_cli_ -- --nocapture` — 3 passed
- `cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture` — 2 passed
- `cargo test -p meshc --test e2e_m046_s02 m046_s02_ -- --nocapture` — 7 passed
- `cargo test -p meshc --test e2e_m044_s01 m044_s01_ -- --nocapture` — 15 passed
- `cargo test -p meshc --test e2e_m044_s02 m044_s02_ -- --nocapture` — 9 passed
- `cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture` — 2 passed

Focused closeout replay after the transport fix:
- `cargo test -p meshc --test e2e_m046_s02 m046_s02_cli_tiny_route_free_startup_dedupes_on_two_nodes -- --nocapture` — 1 passed

These rails confirm startup hook ordering, runtime-owned startup submission/keepalive behavior, explicit CLI/runtime failure surfaces, route-free startup discovery through `meshc cluster ...`, retained M044 manifest/operator behavior, and the repaired simultaneous-boot transport seam.

## Requirements Advanced

- R086 — Moved startup triggering and startup-state inspection onto runtime/tooling-owned surfaces so the route-free proof app only marks clustered work and calls `Node.start_from_env()` while the runtime owns deterministic startup submission, placement, replication setup, and startup status semantics.
- R091 — Extended the runtime-owned CLI surface so `meshc cluster status|continuity|diagnostics` is enough to discover and inspect startup work on a route-free clustered app without an app-owned status endpoint.
- R092 — Established a passing route-free startup proof that uses no `/work` or `/status` routes and no explicit continuity-submit call, pushing the public clustered story toward runtime/tooling-owned proof surfaces.
- R093 — Kept the S02 route-free dual-node proof on trivial arithmetic work (`1 + 1`) so the remaining orchestration and failure-handling complexity is visibly Mesh-owned.

## Requirements Validated

- R087 — Validated by `cargo test -p mesh-rt startup_work_ -- --nocapture`, `cargo test -p meshc --test e2e_m046_s02 m046_s02_cli_ -- --nocapture`, and `cargo test -p meshc --test e2e_m046_s02 m046_s02_ -- --nocapture`, which prove a route-free clustered app auto-runs startup work and is inspected entirely through runtime/tooling surfaces with no app-owned HTTP submit/status routes or explicit `Continuity.submit_declared_work(...)` call.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

Closeout was blocked by a real runtime transport bug that the original task summaries had not retired: the dual-node simultaneous-boot proof still failed in `mesh-rt` session admission with `mesh node spawn failed ... write_error`. The slice plan itself did not change, but S02 needed one extra runtime fix in `compiler/mesh-rt/src/dist/node.rs` so the intended two-node route-free proof could pass honestly.

## Known Limitations

- S02 proves runtime-owned startup triggering and startup-state inspection on test fixtures, but the repo still does not ship the final local `tiny-cluster/` proof package yet.
- The packaged `cluster-proof/` rebuild is still pending, so the route-free startup contract is not yet the repo’s packaged clustered proof surface.
- `meshc cluster continuity` now makes startup work discoverable by runtime name, but the public docs/scaffold/example alignment work is still open to S03–S05.
- The GSD requirements database still does not know about the M046 requirement IDs, so requirement status changes for this milestone need to be recorded as decisions until the DB mapping is repaired.

## Follow-ups

- S03 should build `tiny-cluster/` directly on the S02 startup contract, keep the workload trivial (`1 + 1`), and prove route-free failover/status truth on the same CLI surfaces.
- S04 should rebuild `cluster-proof/` on the same runtime-owned startup/inspection boundary instead of carrying forward any app-owned route or status shim.
- S05 should align `meshc init --clustered`, `tiny-cluster/`, `cluster-proof/`, and docs/verifiers so all clustered-example surfaces express the same contract.
- Future distributed-runtime work that touches connection admission should keep the simultaneous-connect rule under regression coverage; this slice proved that route-free startup truth depends on converging both nodes on the same live transport, not merely on avoiding duplicate names.

## Files Created/Modified

- `compiler/meshc/src/main.rs` — Threaded clustered startup-work registrations through build planning so work-only startup metadata reaches codegen.
- `compiler/mesh-codegen/src/codegen/mod.rs` — Emitted ordered startup registration and post-`mesh_main` trigger calls in the generated main wrapper while keeping service declarations off the startup path.
- `compiler/mesh-codegen/src/codegen/intrinsics.rs` — Declared the runtime startup registration/trigger intrinsics used by the main wrapper.
- `compiler/mesh-rt/src/dist/node.rs` — Added runtime-owned startup-work registration, deterministic startup identity, bounded convergence polling, route-free keepalive/diagnostics, primary-only startup submission, and deterministic duplicate-session resolution for simultaneous boot.
- `compiler/mesh-rt/src/dist/continuity.rs` — Carried declared runtime-name identity through continuity submission/record handling so startup work stays discoverable and deduped.
- `compiler/mesh-rt/src/dist/operator.rs` — Recorded startup diagnostics needed for route-free operator/CLI inspection.
- `compiler/meshc/src/cluster.rs` — Surfaced `declared_handler_runtime_name` in `meshc cluster continuity` JSON and human-readable output.
- `compiler/meshc/tests/e2e_m046_s02.rs` — Added codegen rails, CLI discovery rails, explicit failure-surface checks, and the dual-node simultaneous route-free startup proof for S02.
- `.gsd/PROJECT.md` — Updated current project state to reflect S02 completion and the remaining M046 alignment work.
- `.gsd/KNOWLEDGE.md` — Recorded the route-free startup keepalive/authority lesson and the simultaneous-connect transport rule for future agents.
