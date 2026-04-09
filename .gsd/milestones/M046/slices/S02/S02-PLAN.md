# S02: Runtime-owned startup trigger and route-free status contract

**Goal:** Move clustered-work startup triggering out of Mesh app code and into compiler/runtime/tooling ownership so a route-free clustered app can auto-run declared work on boot and be inspected entirely through built-in `meshc cluster ...` surfaces.
**Demo:** After this: After this: a route-free clustered app can auto-run its clustered work on startup and be inspected entirely through built-in `meshc cluster ...` surfaces, with no app-owned submit/status routes.

## Tasks
- [x] **T01: Threaded work-only startup registrations through meshc/codegen and added LLVM rails for ordered startup hooks.** — Carry S01's clustered execution plan into a dedicated startup-work registration surface so codegen can emit runtime-owned startup hooks for `kind == Work` declarations and keep declared service handlers off the startup path.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `compiler/meshc/src/main.rs` clustered plan threading | fail the build with an explicit missing-startup-metadata error instead of silently dropping startup work | N/A | never synthesize startup registrations from service call/cast entries |
| `compiler/mesh-codegen/src/codegen/mod.rs` main-wrapper emission | fail the focused codegen rail on missing or misordered runtime hook calls | N/A | keep emitted IR free of startup registration calls when no work declarations exist |
| `compiler/mesh-codegen/src/codegen/intrinsics.rs` runtime declarations | fail fast on IR or link drift rather than emitting undeclared symbol names | N/A | reject signature mismatch between codegen declarations and runtime exports |

## Load Profile

- **Shared resources**: merged MIR function table plus the generated startup-registration vector.
- **Per-operation cost**: one linear pass over declared handlers plus one emitted registration call per startup work item.
- **10x breakpoint**: duplicate or missing startup registrations in generated IR will fail long before compile-time throughput becomes interesting.

## Negative Tests

- **Malformed inputs**: clustered service call/cast declarations and binaries with no clustered work.
- **Error paths**: missing runtime hook declarations or missing lowered wrapper symbols fail the compiler rail instead of compiling a partial startup path.
- **Boundary conditions**: source-declared and manifest-declared work both emit the same startup registration name, while service call/cast handlers remain declared-handler-only.

## Steps

1. Extend build/codegen planning with an explicit startup-work registration list derived only from `ClusteredDeclarationKind::Work`.
2. Thread that list through `compile_mir_to_binary(...)`, `compile_mir_to_llvm_ir(...)`, and `CodeGen`, leaving existing declared-handler registration behavior untouched.
3. Emit runtime startup registration and post-`mesh_main` trigger calls in `generate_main_wrapper(...)`, ordered after declared-handler registration and before scheduler handoff.
4. Add focused compiler/codegen proof rails that assert emitted LLVM contains the startup hook for work handlers and omits it for service call/cast handlers.

## Must-Haves

- [ ] Only clustered work declarations reach the startup-work runtime hook.
- [ ] Source and manifest declarations converge on the same startup registration identity.
- [ ] Declared service call/cast handlers remain available for their existing runtime path but never auto-trigger at startup.
- [ ] Emitted LLVM/main-wrapper ordering proves registration happens before the startup trigger runs.
  - Estimate: 2h
  - Files: compiler/meshc/src/main.rs, compiler/mesh-codegen/src/declared.rs, compiler/mesh-codegen/src/lib.rs, compiler/mesh-codegen/src/codegen/mod.rs, compiler/mesh-codegen/src/codegen/intrinsics.rs, compiler/meshc/tests/e2e_m046_s02.rs
  - Verify: cargo test -p meshc --test e2e_m046_s02 m046_s02_codegen_ -- --nocapture && cargo test -p meshc --test e2e_m044_s02 m044_s02_metadata_ -- --nocapture
- [x] **T02: Added runtime-owned startup submission, bounded convergence waiting, and clustered route-free keepalive for declared startup work.** — Make the runtime consume startup-work registrations autonomously: register stable startup identities, wait boundedly for peer convergence, submit through `submit_declared_work(...)`, and keep cluster-mode route-free binaries alive long enough to inspect without any app glue.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `compiler/mesh-rt/src/dist/node.rs` startup registry and trigger path | reject the startup work with an explicit diagnostic instead of panicking or silently skipping it | record a convergence-timeout diagnostic and fail closed if the replica requirement is still unmet | never submit startup work with a blank runtime name, blank request key, or missing handler metadata |
| `compiler/mesh-rt/src/dist/continuity.rs` submit/state machine | keep the continuity record rejected with an explicit reason instead of inventing success | preserve pending-or-rejected truth in continuity/diagnostics instead of dropping the record | reject duplicate or stale startup identities explicitly |
| `compiler/mesh-rt/src/actor/mod.rs` scheduler lifetime | keep cluster-mode route-free apps alive via runtime-owned work/keepalive actors instead of exiting immediately after `mesh_main` returns | N/A | do not strand the scheduler with zero active runtime-owned actors before tooling can inspect the node |

## Load Profile

- **Shared resources**: continuity registry, node session map, operator diagnostics buffer, and scheduler active-process count.
- **Per-operation cost**: one bounded membership polling loop and one continuity submit per startup work item per process boot.
- **10x breakpoint**: slow peer convergence or many startup work items will show up first as pending actor/diagnostic churn, not raw CPU saturation.

## Negative Tests

- **Malformed inputs**: blank runtime names, duplicate startup registrations, missing registered handlers, and standalone boot with no cluster cookie.
- **Error paths**: convergence timeout, `replica_required_unavailable`, and remote spawn rejection after reconnect.
- **Boundary conditions**: standalone mode, single-node cluster mode, two-node cluster mode with late peer arrival, and simultaneous boot on both nodes using the same startup identity.

## Steps

1. Add a runtime registry for startup work keyed by declared runtime registration name and derive a deterministic startup request key from that runtime name.
2. Spawn runtime-owned startup actors after registration that wait boundedly for peer convergence, derive required replicas from observed membership, and submit through the existing declared-work continuity path.
3. Add runtime-owned diagnostics for startup registration, trigger, timeout, rejection, and completion/fencing, and keep cluster-mode route-free binaries alive without any app `HTTP.serve(...)` or `Continuity.submit_declared_work(...)` glue.
4. Add focused runtime tests for registration dedupe, deterministic request identity, bounded timeout behavior, and keepalive-trigger interaction.

## Must-Haves

- [ ] Startup work uses a deterministic runtime-owned identity so multiple boots converge on one logical startup run.
- [ ] Startup submission waits boundedly for peers and fails closed with diagnostics when replication cannot be satisfied.
- [ ] Route-free cluster-mode processes stay alive long enough for `meshc cluster ...` inspection without app keepalive glue.
- [ ] No app-owned `Continuity.submit_declared_work(...)` or `Continuity.mark_completed(...)` call is required for startup execution.
  - Estimate: 3h
  - Files: compiler/mesh-rt/src/dist/node.rs, compiler/mesh-rt/src/dist/continuity.rs, compiler/mesh-rt/src/dist/operator.rs, compiler/mesh-rt/src/actor/mod.rs, compiler/mesh-rt/src/lib.rs
  - Verify: cargo test -p mesh-rt startup_work_ -- --nocapture
- [x] **T03: Surfaced declared runtime names on cluster continuity output and proved route-free startup discovery through the CLI.** — Expose enough continuity identity on `meshc cluster continuity` for a route-free proof to locate startup work by runtime name instead of relying on an app-owned status route or guessing an opaque internal key.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `compiler/meshc/src/cluster.rs` JSON and human-readable rendering | fail the CLI rail on missing identity fields instead of silently dropping startup-work metadata | N/A | reject malformed continuity JSON/rendering in tests instead of printing partial records |
| `compiler/mesh-rt/src/dist/operator.rs` continuity query transport | preserve existing `target_not_connected` and decode errors without adding the CLI as a visible cluster peer | return the existing operator timeout error | never strip the declared runtime name from list or single-record payloads |

## Load Profile

- **Shared resources**: operator query payload size and continuity list truncation.
- **Per-operation cost**: one additional string field per continuity record in list and single-record output.
- **10x breakpoint**: list truncation and payload size growth show up before transport or JSON serialization cost matters.

## Negative Tests

- **Malformed inputs**: empty runtime names in continuity records and malformed query replies.
- **Error paths**: disconnected-target and decode failures remain explicit on CLI output instead of encouraging app-owned fallbacks.
- **Boundary conditions**: list mode with multiple records, single-record mode by request key, and records whose runtime name is present even when owner/replica routing changes.

## Steps

1. Add `declared_handler_runtime_name` to `meshc cluster continuity` JSON output and human-readable record/list rendering.
2. Keep list and single-record paths aligned so a route-free proof can discover startup work in list mode and then inspect the exact record.
3. Add CLI-focused proof rails that assert runtime-name visibility without regressing the transient operator query contract.

## Must-Haves

- [ ] Route-free startup work is discoverable from `meshc cluster continuity` alone.
- [ ] JSON and human-readable continuity output stay aligned on runtime-name visibility.
- [ ] Operator/CLI failures remain explicit instead of pushing the proof back toward app-owned routes.
  - Estimate: 90m
  - Files: compiler/meshc/src/cluster.rs, compiler/mesh-rt/src/dist/continuity.rs, compiler/meshc/tests/e2e_m046_s02.rs, compiler/meshc/tests/e2e_m044_s03.rs
  - Verify: cargo test -p meshc --test e2e_m046_s02 m046_s02_cli_ -- --nocapture && cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture
- [x] **T04: Added a dual-node source-level startup proof that boots route-free nodes, auto-runs trivial clustered work, and verifies deduped completion plus diagnostics entirely through `meshc cluster ...`.** — Add the tiny route-free S02 proof fixture that boots two nodes, auto-runs startup work, and proves completion/diagnostics entirely through `meshc cluster ...` with no `/work`, `/status`, or explicit continuity-submit code in the fixture.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `compiler/meshc/tests/e2e_m046_s02.rs` temp-project harness | archive build/stdout/stderr and fail with the retained last observation instead of collapsing the proof to one panic line | bound each wait and record which CLI surface failed to converge | treat malformed JSON or missing fields as proof failures with archived raw output |
| runtime startup trigger path | fail the proof on missing continuity records, duplicate startup execution, or missing diagnostics instead of falling back to app routes | archive the last `status`, `continuity`, and `diagnostics` observations before failing | assert explicit reject reasons instead of swallowing startup-trigger failures |
| `meshc cluster ...` surfaces | fail on `target_not_connected` or missing runtime name rather than probing an app route | archive the last CLI output and logs for the failed node | reject malformed CLI JSON as a tooling proof failure |

## Load Profile

- **Shared resources**: dual-node temp binaries, continuity registry, operator query polling, and retained artifact directories.
- **Per-operation cost**: two long-running processes plus bounded polling of `meshc cluster status`, `meshc cluster continuity`, and `meshc cluster diagnostics`.
- **10x breakpoint**: slow convergence, port contention, or diagnostics-buffer truncation will fail the proof before test-runner CPU becomes relevant.

## Negative Tests

- **Malformed inputs**: fixture sources that reintroduce `HTTP.serve(...)`, `/work`, `/status`, or explicit `Continuity.submit_declared_work(...)`.
- **Error paths**: startup rejection or convergence timeout must be visible through CLI diagnostics with archived evidence.
- **Boundary conditions**: simultaneous two-node boot dedupes to one logical startup record, and the declared work body stays trivial (`1 + 1`) so orchestration ownership is obvious.

## Steps

1. Build a temp-project fixture in `compiler/meshc/tests/e2e_m046_s02.rs` that uses source-level `clustered(work)` plus `Node.start_from_env()` only, with trivial `1 + 1` work and no app routes.
2. Run two nodes, wait for runtime-owned membership/authority truth, and discover the startup record entirely through `meshc cluster status|continuity|diagnostics`.
3. Assert the fixture source never calls `Continuity.submit_declared_work(...)`, never adds `/work` or `/status` routes, and never teaches app-owned owner/replica shaping.
4. Close the slice by replaying the retained M044 declared-handler and operator rails alongside the new route-free proof.

## Must-Haves

- [ ] The S02 proof fixture is route-free and contains no app-owned startup/status control flow.
- [ ] The clustered work body stays trivial enough that the remaining complexity is visibly Mesh-owned.
- [ ] The proof uses only `meshc cluster status`, `meshc cluster continuity`, and `meshc cluster diagnostics` to inspect startup truth.
- [ ] The new proof rail and retained M044 rails together cover R086, R087, R091, R092, and R093.
  - Estimate: 2h
  - Files: compiler/meshc/tests/e2e_m046_s02.rs, compiler/meshc/tests/e2e_m045_s02.rs, compiler/meshc/tests/e2e_m044_s03.rs
  - Verify: cargo test -p meshc --test e2e_m046_s02 m046_s02_ -- --nocapture && cargo test -p meshc --test e2e_m044_s01 m044_s01_ -- --nocapture && cargo test -p meshc --test e2e_m044_s02 m044_s02_ -- --nocapture && cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture
