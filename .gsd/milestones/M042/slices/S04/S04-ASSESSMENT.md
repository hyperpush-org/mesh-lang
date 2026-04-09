# M042 / S04 — closeout blocked

## Status

Slice S04 is **not complete**. The docs/proof-surface rail is green, but the full slice acceptance bundle is still red on the inherited remote `/work` path.

## What passed

- `cargo run -q -p meshc -- test cluster-proof/tests && cargo run -q -p meshc -- build cluster-proof`
- `bash scripts/verify-m042-s04-proof-surface.sh`
- `bash scripts/verify-m042-s04-fly.sh --help`
- `npm --prefix website run build`

## Blocking command

```bash
bash scripts/verify-m039-s04.sh && bash scripts/verify-m042-s03.sh && bash scripts/verify-m042-s04.sh && bash scripts/verify-m042-s04-fly.sh --help
```

This currently fails in the first phase:

```bash
bash scripts/verify-m039-s04.sh
```

which fails inside:

```bash
cargo test -p meshc --test e2e_m039_s02 e2e_m039_s02_routes_work_to_peer_and_logs_execution -- --nocapture
```

## Observed failure

Authoritative failure log:
- `.tmp/m039-s02/verify/04-s02-remote-route.log`

Key evidence from that run:

- ingress log shows the first request was dispatched remotely:
  - `[cluster-proof] work dispatched request_id=work-0 ingress=node-a@127.0.0.1:59178 target=node-b@[::1]:59178 routed_remotely=true`
- peer never logs a successful execution for that request
- peer stderr aborts with:
  - `thread '<unnamed>' ... panicked at compiler/mesh-rt/src/string.rs:104:21: null pointer dereference occurred`
- after that crash, ingress falls back locally on the next probe request:
  - `[cluster-proof] work dispatched request_id=work-1 ... routed_remotely=false`
- the e2e then fails because the returned `/work` response no longer reports remote routing

## Most likely root cause

The failure looks lower-level than cluster-proof placement.

The current remote spawn path in `compiler/mesh-rt/src/dist/node.rs` handles `DIST_SPAWN` by taking `args_data = &msg[...]` from the reader-thread message buffer and passing `args_data.as_ptr()` directly into `mesh_actor_spawn(...)`.

That pointer is then queued through the scheduler without an owned copy. Once the reader handler returns and the distribution message buffer is dropped, the spawned actor can be resumed with a dangling args pointer. The peer crash in `compiler/mesh-rt/src/string.rs` is consistent with the spawned actor later dereferencing invalid string argument data.

Relevant seams to resume from:

- `compiler/mesh-rt/src/dist/node.rs`
  - `DIST_SPAWN` handler
  - `mesh_node_spawn(...)`
- `compiler/mesh-rt/src/actor/scheduler.rs`
  - `Scheduler::spawn(...)` currently ignores `args_size` and stores only a raw pointer in `SpawnRequest`
- `cluster-proof/work_continuity.mpl`
  - remote execution still uses `Node.spawn(target_node, execute_work, request_key, attempt_id)`

## Recommended resume point

Resume with a fresh context at the runtime spawn-argument ownership seam, not in docs or verifier scripts.

Suggested order:

1. Make spawned actor args owned across the scheduler boundary (runtime fix) **or** replace the remote execution path with one that does not rely on borrowed remote arg buffers.
2. Re-run:
   - `cargo test -p meshc --test e2e_m039_s02 e2e_m039_s02_routes_work_to_peer_and_logs_execution -- --nocapture`
3. Then replay the slice acceptance bundle:
   - `bash scripts/verify-m039-s04.sh`
   - `bash scripts/verify-m042-s03.sh`
   - `bash scripts/verify-m042-s04.sh`
   - `bash scripts/verify-m042-s04-fly.sh --help`
4. Only if those pass, finish the slice summary/UAT and call `gsd_complete_slice`.

## Notes

Do **not** trust the green docs rail as slice completion evidence. The runtime/operator proof remains red until the inherited M039 remote route path is repaired and the packaged M042 rail is replayed end to end.
