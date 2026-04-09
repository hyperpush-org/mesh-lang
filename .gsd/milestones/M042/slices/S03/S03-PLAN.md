# S03: Owner-loss recovery, same-key retry, and stale-completion safety

**Goal:** Prove restart-by-key continuity on the runtime-owned substrate under real node loss and rejoin conditions without claiming arbitrary process-state migration or exactly-once semantics.
**Demo:** After this: After this slice, a two-node cluster can lose the active owner and still serve truthful keyed continuity status from surviving replicated state; same-key retry converges through a rolled attempt_id, stale completions are rejected, and rejoin is observable through the same runtime-owned status model.

## Tasks
- [x] **T01: Added recovery-aware attempt rollover, attempt-token-first merge fencing, and stale-completion rejection coverage in the runtime continuity registry.** — Close the correctness core inside `mesh-rt` before touching node lifecycle or live harnesses. Same-key retry after owner loss only becomes honest if the registry can roll a new attempt and then fence older completions everywhere.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Existing continuity records in `compiler/mesh-rt/src/dist/continuity.rs` | Keep the stored record authoritative when retry is not eligible; never mutate conflicting payloads into recovery state. | Do not block retry on missing liveness evidence; leave the record unchanged and fail closed to duplicate/conflict truth. | Reject impossible record combinations instead of merging them into active state. |
| Continuity merge and snapshot upserts | Prefer the newer attempt token before terminal/non-terminal phase so stale completed truth cannot overwrite an active retry. | Preserve the local newer attempt even if a rejoin snapshot arrives late. | Ignore payload-hash mismatches and attempt regressions instead of poisoning the registry. |

## Load Profile

- **Shared resources**: Continuity registry lock, attempt-token counter, and cluster-wide continuity upsert traffic.
- **Per-operation cost**: One registry mutation plus attempt-token parsing and merge-precedence checks.
- **10x breakpoint**: Hot-key retry storms hit lock contention and stale-upsert races first; precedence must remain monotonic under extra message volume.

## Negative Tests

- **Malformed inputs**: Missing request key / payload hash / owner node, invalid attempt IDs, and mismatched payload hashes on merge.
- **Error paths**: Same-key retry while the old owner is still authoritative, late `mark_completed(old_attempt)` after rollover, and stale completed upserts arriving after a newer retry was created.
- **Boundary conditions**: `attempt-0` to `attempt-1` rollover, repeated retry after a prior rollover, and snapshot/upsert merges where both sides carry different replica states for the same request key.

## Steps

1. Add an explicit recovery-eligible retry transition in `compiler/mesh-rt/src/dist/continuity.rs` for same-key same-payload pending records that have lost their active owner and should roll to a new `attempt_id`.
2. Reorder `preferred_record(...)` and any related helpers so parsed `attempt_id` tokens fence stale older records before terminal/non-terminal precedence is considered.
3. Keep `mark_completed(...)` fenced to the active attempt and add coverage for stale completion after rollover plus stale rejoin/snapshot merges.
4. Expand the continuity unit tests so recovery retry, stale completion rejection, and stale completed merge rejection are proven at the runtime level before node-lifecycle work starts.

## Must-Haves

- [ ] Same-key retry after owner loss can return `created` with a new active `attempt_id` instead of unconditional `duplicate`.
- [ ] Older completed or rejected records cannot overwrite a newer submitted retry through `merge_remote_record(...)` or `merge_snapshot(...)`.
- [ ] `mark_completed(old_attempt_id)` fails with `attempt_id_mismatch` after rollover.
- [ ] `next_attempt_token` stays monotonic across retry rollover and merged records.
  - Estimate: 2h
  - Files: `compiler/mesh-rt/src/dist/continuity.rs`
  - Verify: cargo test -p mesh-rt continuity -- --nocapture
- [x] **T02: Added explicit runtime owner-loss continuity state and ordinary-submit recovery rollover.** — Once the registry can roll attempts safely, wire real node-loss knowledge into that transition path. This task keeps owner-loss detection in `node.rs`, preserves the existing Mesh-facing `Continuity` API, and only touches `cluster-proof` if the runtime-owned status model needs a thin parsing or log-surface adjustment.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Node disconnect/reconnect hooks in `compiler/mesh-rt/src/dist/node.rs` | Mark owner loss explicitly and keep the surviving replica serving truthful status instead of leaving the lost owner authoritative forever. | A reconnecting node must not block the surviving node from keeping the newer attempt active. | Invalid sync/upsert payloads must be ignored in favor of the safer local record. |
| Thin `cluster-proof` continuity consumer in `cluster-proof/work.mpl` | Preserve the runtime-owned contract and fail closed on invalid JSON rather than reintroducing app-authored repair logic. | Pending status should stay readable from the surviving node during retry and rejoin polling. | Completion/log parsing must reject impossible field shapes instead of inventing success. |

## Load Profile

- **Shared resources**: Node session map, disconnect/connect callbacks, continuity sync payloads, and per-request status polling.
- **Per-operation cost**: One owner-loss transition per affected record plus the existing sync-on-connect upsert flow.
- **10x breakpoint**: Churny reconnects and repeated same-key retries will stress sync ordering first; reconnect paths must preserve the newest attempt without widening the API.

## Negative Tests

- **Malformed inputs**: Disconnect for an unrelated node, reconnect snapshot carrying an older attempt, and status JSON missing expected continuity fields.
- **Error paths**: Owner disappears while the request is still pending, retry arrives on the surviving node, and the old owner rejoins with stale replicated state.
- **Boundary conditions**: Owner-loss when the surviving node was the mirrored replica, owner-loss while the request is already terminal, and repeated disconnect/reconnect cycles for the same node identity.

## Steps

1. Add owner-loss continuity handling to `handle_node_disconnect(...)` and related runtime helpers so records whose `owner_node` disappears become recovery-eligible on the surviving replica while replica-loss downgrade behavior still works.
2. Keep recovery on the ordinary `Continuity.submit(...)` path by threading the minimal liveness/eligibility seam needed for same-key retry to roll a new attempt without widening the Mesh-facing API.
3. Verify that connect-time `send_continuity_sync(...)` plus the new merge precedence keep the latest attempt authoritative after same-identity rejoin.
4. Update `cluster-proof/work.mpl` and `cluster-proof/tests/work.test.mpl` only if the runtime-owned status/log contract needs thin parsing or failure-surface adjustments for owner-loss recovery.

## Must-Haves

- [ ] The runtime notices owner loss, not just replica loss, and leaves the surviving node able to serve truthful continuity status.
- [ ] Same-key recovery still goes through `Continuity.submit(...)`; no new Mesh-facing owner-repair API is introduced.
- [ ] Same-identity rejoin cannot resurrect an older attempt or stale owner mapping over the newer retry.
- [ ] Any `cluster-proof` changes remain thin consumer work: parsing, status mapping, or log-surface updates only.
  - Estimate: 90m
  - Files: `compiler/mesh-rt/src/dist/node.rs`, `compiler/mesh-rt/src/dist/continuity.rs`, `cluster-proof/work.mpl`, `cluster-proof/tests/work.test.mpl`
  - Verify: cargo test -p mesh-rt continuity -- --nocapture && cargo run -q -p meshc -- test cluster-proof/tests
- [x] **T03: Drafted the S03 owner-loss harness, added the fail-closed verifier wrapper, and patched cluster-proof recovery submits, but the full S03 verification rail still needs one more rerun.** — Finish the slice with a destructive proof rail that exercises the real owner-loss contract. Reuse the stable local-owner placement search from S02 and the kill/restart artifact discipline from M039/S03 so the proof stays about continuity recovery, not the unrelated remote-spawn crash.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Spawned `cluster-proof` processes and membership convergence in the Rust harness | Capture stdout/stderr and fail with the named readiness or phase that did not converge. | Time out with preserved pre-loss/degraded/retry/rejoin artifacts instead of hanging or silently skipping the hard part. | Archive raw HTTP bodies and fail closed on non-JSON or invalid continuity payloads. |
| S03 verifier wrapper | Replay prerequisites before the slice-specific target and stop on the first drift. | Fail if any named test filter runs 0 tests or required artifact bundle is missing. | Reject malformed JSON manifests or copied logs instead of claiming the slice passed. |

## Load Profile

- **Shared resources**: Ephemeral ports, spawned `cluster-proof` processes, `.tmp/m042-s03/...` artifact directories, and the repo-local `cluster-proof` build output.
- **Per-operation cost**: Building `mesh-rt` / `cluster-proof` once, then running a small number of destructive two-node scenarios with HTTP polling and process restarts.
- **10x breakpoint**: Port reuse, process cleanup, and late stale-completion races will flake first; the harness must keep setup/teardown deterministic and archive enough evidence to debug the first failing phase.

## Negative Tests

- **Malformed inputs**: Non-JSON status responses, zero-test command filters, and malformed copied artifact manifests.
- **Error paths**: Owner loss while the request is pending, same-key retry on the surviving node, stale completion from the old attempt, and same-identity rejoin with stale replicated state.
- **Boundary conditions**: Stable local-owner selection before owner loss, rollover from one attempt to the next on the same request key, and rejoin after the retry has already completed.

## Steps

1. Create `compiler/meshc/tests/e2e_m042_s03.rs` by combining the stable HTTP/artifact helpers from `e2e_m042_s02.rs` with the kill/restart patterns from `compiler/meshc/tests/e2e_m039_s03.rs`.
2. Add named scenarios for surviving-node status after owner loss, same-key retry rolling a new `attempt_id`, stale-completion rejection, and same-identity rejoin preserving the newer attempt as authoritative.
3. Write `scripts/verify-m042-s03.sh` so it replays runtime continuity tests, `cluster-proof` tests, `bash scripts/verify-m042-s02.sh`, and the named `e2e_m042_s03` target while fail-closing on missing `running N test` evidence or missing artifact bundles.
4. Preserve copied proof bundles under `.tmp/m042-s03/verify/` for pre-loss, degraded, retry-rollover, stale-completion, and post-rejoin phases with per-node logs and raw HTTP captures.

## Must-Haves

- [ ] The repo contains a named `compiler/meshc/tests/e2e_m042_s03.rs` target proving owner-loss status serving, retry rollover, stale-completion safety, and rejoin truth.
- [ ] `scripts/verify-m042-s03.sh` replays the stable prerequisites, fails closed on zero-test filters, and archives copied evidence for each destructive phase.
- [ ] The proof stays on the stable local-owner rail instead of reopening the unrelated remote-owner execution crash.
- [ ] The preserved artifacts make the first failing phase obvious from logs and JSON alone.
  - Estimate: 2h
  - Files: `compiler/meshc/tests/e2e_m042_s03.rs`, `scripts/verify-m042-s03.sh`
  - Verify: cargo test -p meshc --test e2e_m042_s03 -- --nocapture && bash scripts/verify-m042-s03.sh
