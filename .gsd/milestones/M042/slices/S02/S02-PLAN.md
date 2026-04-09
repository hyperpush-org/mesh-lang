# S02: Replica-backed admission and fail-closed durability truth

**Goal:** Make runtime-owned keyed submit fail closed on replica-backed durability: accept work only after runtime-proven replica preparation or reject it explicitly with a stored record, then surface mirrored, degraded, and rejected truth through the ordinary `cluster-proof` status API without depending on the still-broken remote-owner completion path.
**Demo:** After this: After this slice, the same cluster-proof keyed submit path either accepts work with mirrored replica truth or rejects it explicitly when replica safety is unavailable; operators can inspect that mirrored/degraded/rejected state through the ordinary status surface.

## Tasks
- [x] **T01: Added runtime-owned durable continuity admission, replica prepare/ack rejection, and disconnect downgrade truth.** — Introduce the real S02 runtime boundary inside `mesh-rt`: continuity submit must know whether replica safety is required, record rejected admissions durably, and downgrade mirrored work to `degraded_continuing` when replica safety is later lost.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Inter-node continuity message handling in `compiler/mesh-rt/src/dist/node.rs` | Keep the owner-side record truthful and reject or degrade instead of assuming replica safety. | Treat missing replica confirmation as non-admission or post-admission degradation; never silently keep `mirrored`. | Ignore invalid remote continuity payloads after validation and keep the safer local record. |
| Continuity registry merge rules in `compiler/mesh-rt/src/dist/continuity.rs` | Prefer terminal / safer replica truth and return explicit errors for invalid transitions. | Do not block the registry indefinitely on peer state; record the local durable decision and surface it. | Reject mismatched attempt/payload/owner data instead of merging it into the request record. |

## Load Profile

- **Shared resources**: Continuity registry lock, node session map, continuity upsert/sync traffic, and per-request log volume.
- **Per-operation cost**: One registry mutation plus replica prepare/ack or reject bookkeeping, followed by one or more continuity messages.
- **10x breakpoint**: Session write failures and lock contention will show up first; the task must keep merge precedence monotonic so extra message volume cannot regress truth.

## Negative Tests

- **Malformed inputs**: Missing request key / payload hash / owner node, replica equal to owner, invalid attempt ID, and empty replica on replica-required submit.
- **Error paths**: Replica prepare or ack unavailable, stale mirrored upsert arriving after degradation, and disconnect occurring while work is still pending.
- **Boundary conditions**: `required_replica_count = 0` vs `1`, rejected duplicate replay, conflict after rejection, and repeated degrade/ack transitions.

## Steps

1. Extend `SubmitRequest` / `SubmitDecision` and the continuity transition helpers so submit admission is durability-aware instead of inferring policy from `replica_node` shape alone.
2. Add the runtime-owned prepare/ack-or-reject path that persists rejected records with stable `phase`, `result`, `replica_status`, and `error` fields instead of relying on fire-and-forget upserts.
3. Update disconnect handling and merge precedence so accepted mirrored work downgrades to `degraded_continuing` when replica safety is lost, and stale mirrored data cannot overwrite later safer truth.
4. Expand the continuity unit tests around rejection replay, conflict preservation, mirrored acceptance, degrade-on-disconnect, and merge monotonicity.

## Must-Haves

- [ ] Submitting with replica safety required can return durable rejected truth, not just `created` / `duplicate` / `conflict`.
- [ ] Rejected records are stored and replayed on same-key same-payload retry.
- [ ] Disconnect handling can surface `degraded_continuing` for surviving records that previously had replica safety.
- [ ] `preferred_record(...)` / snapshot merge logic does not let older mirrored state overwrite newer degraded or rejected truth.
  - Estimate: 2h
  - Files: compiler/mesh-rt/src/dist/continuity.rs, compiler/mesh-rt/src/dist/node.rs, compiler/mesh-rt/src/lib.rs
  - Verify: cargo test -p mesh-rt continuity -- --nocapture
- [x] **T02: Plumbed durability-aware Continuity.submit through the compiler and made cluster-proof replay runtime-owned rejected and duplicate truth without app-authored replica acknowledgements.** — Once the runtime owns admission truth, the Mesh-facing API and `/work` handlers need to stop papering over it. This task keeps the compiler/runtime seam small, passes the durability requirement through `Continuity.submit(...)`, and makes `cluster-proof` replay rejected or mirrored state exactly as stored.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Continuity runtime ABI / compiler intrinsic mapping | Fail the build or tests loudly; do not leave partially updated arity or symbol mappings. | N/A for compile-time plumbing. | Treat unexpected JSON or field shape from runtime as a parse failure and return the stored failure payload instead of inventing success. |
| `cluster-proof` submit/status response mapping | Return the truthful HTTP failure (`503` or `409`) with stored status payload, not a synthetic success path. | Keep status reads on the existing runtime lookup path; do not add app-local fallback state. | Reject invalid runtime JSON with an explicit failure response instead of silently dropping fields. |

## Load Profile

- **Shared resources**: Mesh compiler intrinsic declarations, runtime ABI surface, and the `/work` HTTP entrypoint that may see repeated retries for the same key.
- **Per-operation cost**: Trivial compile-time changes plus one runtime submit/status JSON parse per HTTP request.
- **10x breakpoint**: Retry storms against the same key must still replay stored duplicate/rejected truth without dispatching extra work.

## Negative Tests

- **Malformed inputs**: Bad request key / payload, unexpected JSON from runtime, and invalid durability-policy-derived replica requirement.
- **Error paths**: Runtime submit returns rejected durable admission, duplicate replay of rejected record, and conflicting same-key reuse.
- **Boundary conditions**: Standalone/local-only submit still works, cluster-mode replica-backed submit with no replica returns `503`, and duplicate success vs duplicate rejection map to different `ok` values.

## Steps

1. Update the `Continuity` stdlib typing and LLVM intrinsic declaration/lowering so `Continuity.submit(...)` carries the new durability argument while keeping the API continuity-specific.
2. Export any adjusted runtime symbols from `compiler/mesh-rt/src/lib.rs` and make `cluster-proof/work.mpl` pass `required_replica_count(current_durability_policy())` into runtime submit.
3. Remove `acknowledged_replica_record(...)` from the live submit path so `cluster-proof` no longer manufactures `mirrored` state after the fact.
4. Add truthful submit/duplicate/status response mapping for rejected durable admission and preserve the existing same-key conflict contract.
5. Update `cluster-proof/tests/work.test.mpl` for helper-level response and policy behavior that can be exercised without the live cluster harness.

## Must-Haves

- [ ] The compiler/runtime seam compiles with the new `Continuity.submit(...)` arity and no stale acknowledge-based happy path.
- [ ] `cluster-proof` passes the runtime durability requirement instead of inferring safety from `replica_node` alone.
- [ ] `POST /work` returns stored rejected truth when durability admission fails and does not dispatch work in that branch.
- [ ] Duplicate replay is truthful for both successful and rejected stored records.
  - Estimate: 90m
  - Files: compiler/mesh-typeck/src/infer.rs, compiler/mesh-codegen/src/codegen/intrinsics.rs, compiler/mesh-codegen/src/mir/lower.rs, compiler/mesh-rt/src/lib.rs, cluster-proof/work.mpl, cluster-proof/tests/work.test.mpl
  - Verify: cargo run -q -p meshc -- test cluster-proof/tests
- [x] **T03: Added the S02 continuity e2e target and fail-closed verifier that prove rejected, mirrored, and degraded status truth on the stable local-owner rail with copied artifacts under `.tmp/m042-s02/`.** — This slice needs its own proof rail because `scripts/verify-m042-s01.sh` still depends on the unrelated remote-owner completion crash. The new harness must prove the S02 contract only on stable paths and preserve enough artifacts to debug regressions later.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `cluster-proof` process startup / membership convergence in the Rust harness | Capture stdout/stderr and fail with the named readiness condition that was not met. | Time out with preserved artifacts instead of hanging or silently skipping tests. | Treat non-JSON HTTP bodies as contract failures and archive the raw response. |
| Slice verifier wrapper | Fail closed on the first missing proof phase or zero-test run; do not reuse the stale S01 full verifier. | Surface which phase stalled and preserve the partial artifact directory. | Reject malformed archived JSON / logs rather than claiming the slice is green. |

## Load Profile

- **Shared resources**: Ephemeral ports, spawned `cluster-proof` processes, `.tmp/m042-s02/...` artifact directories, and the repo-local `cluster-proof` build output.
- **Per-operation cost**: Building `mesh-rt` / `cluster-proof` once, then a small number of live HTTP polls per e2e case.
- **10x breakpoint**: Port allocation and process cleanup flake first; the harness must keep setup/teardown deterministic and archive enough logs to diagnose races.

## Negative Tests

- **Malformed inputs**: Missing or invalid request key body, non-JSON HTTP response, and zero-test command filters.
- **Error paths**: Single-node cluster-mode replica-backed submit rejected with stored status, replica node killed after mirrored admission, and harness timeout before status transitions.
- **Boundary conditions**: Local-owner with non-empty replica selection, duplicate retry of rejected record, and pending mirrored work observed before completion.

## Steps

1. Create `compiler/meshc/tests/e2e_m042_s02.rs` by reusing only the stable process / HTTP / artifact helpers from S01, not the remote-owner completion proof itself.
2. Add named e2e cases for single-node cluster-mode rejection, two-node local-owner/remote-replica mirrored admission, and post-loss `degraded_continuing` status.
3. Write `scripts/verify-m042-s02.sh` so it replays runtime tests, `cluster-proof` tests, the S01 standalone regression, and the new S02 target without calling the known-failing full S01 verifier.
4. Make the verifier fail closed on missing `running N test` evidence or missing artifact files so a skipped test target cannot look green.

## Must-Haves

- [ ] The repo contains a named `compiler/meshc/tests/e2e_m042_s02.rs` target covering rejected, mirrored, and degraded status truth.
- [ ] The new verifier replays only stable prerequisite proofs and stops before the unrelated remote-owner completion blocker.
- [ ] The harness archives per-node logs and response JSON under `.tmp/m042-s02/...` for later diagnosis.
- [ ] The verification surface checks that the intended tests actually ran, not just that Cargo exited 0.
  - Estimate: 2h
  - Files: compiler/meshc/tests/e2e_m042_s02.rs, compiler/meshc/tests/e2e_m042_s01.rs, scripts/verify-m042-s02.sh
  - Verify: cargo test -p meshc --test e2e_m042_s02 -- --nocapture && bash scripts/verify-m042-s02.sh
