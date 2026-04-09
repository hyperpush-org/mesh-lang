---
id: S02
parent: M047
milestone: M047
provides:
  - Runtime-owned replication-count semantics for ordinary `@cluster` functions, including default count `2`, explicit count preservation, durable unsupported-fanout rejection, and end-to-end CLI proof through `meshc cluster continuity`.
requires:
  - slice: S01
    provides: Source-first `@cluster` / `@cluster(N)` parsing plus declaration provenance/count metadata and meshc/LSP diagnostics for ordinary clustered functions.
affects:
  - S03
  - S04
  - S06
key_files:
  - compiler/mesh-codegen/src/declared.rs
  - compiler/mesh-codegen/src/codegen/mod.rs
  - compiler/mesh-rt/src/dist/node.rs
  - compiler/mesh-rt/src/dist/continuity.rs
  - compiler/meshc/src/cluster.rs
  - compiler/meshc/tests/e2e_m047_s02.rs
  - .gsd/PROJECT.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - D274: store replication_count on declared-handler registrations keyed by runtime name while leaving startup-work registration as a name-only list.
  - D275: continuity records store public replication_count, derive required_replica_count from declared-handler metadata, and reject unsupported fanout durably inside the continuity state machine while keeping the single-node startup carveout explicit.
  - D276: requirement R098 is now validated by the S02 proof rail rather than remaining a planned-only contract.
patterns_established:
  - Carry clustered execution metadata through the existing declared-handler runtime-name registry instead of inventing startup-only side tables or app-local heuristics.
  - Treat unsupported replication factors as durable continuity truth — visible through records, CLI, and diagnostics — rather than as silent fallback behavior.
  - Prove compiler/runtime seams with a route-free temp-project rail that checks emitted LLVM markers and runtime-owned continuity output together.
observability_surfaces:
  - `meshc cluster continuity --json` / human output now renders `runtime_name` and `replication_count` for ordinary source-declared clustered functions.
  - Runtime continuity diagnostics/logs preserve explicit rejection reasons like `unsupported_replication_count:3` instead of hiding higher-fanout failure behind local success.
  - Retained proof artifacts under `.tmp/m047-s02/cli-source-cluster-counts-*/` and `.tmp/m047-s02/llvm-registration-markers-*/` capture emitted LLVM, continuity JSON/human output, diagnostics, and runtime logs for replay/debugging.
drill_down_paths:
  - .gsd/milestones/M047/slices/S02/tasks/T01-SUMMARY.md
  - .gsd/milestones/M047/slices/S02/tasks/T02-SUMMARY.md
  - .gsd/milestones/M047/slices/S02/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-01T07:32:44.040Z
blocker_discovered: false
---

# S02: Replication-count semantics for clustered functions

**Replication counts now flow from source-declared `@cluster` functions into runtime registration and continuity truth, with bare `@cluster` defaulting to `2` and unsupported higher fanout staying durably rejected instead of being silently clipped.**

## What Happened

S02 turned the new source-first clustered declaration syntax into a truthful runtime contract instead of a parser-only surface. T01 carried the resolved `replication_count` from meshc planning through mesh-codegen and LLVM registration into the runtime declared-handler registry, keyed by the same generic runtime names the runtime already executes. That removed the old implicit-count guessing path and kept startup registrations as identity-only entries instead of creating a second metadata table. T02 then taught the continuity runtime and CLI to use that metadata honestly: continuity records now preserve public `replication_count`, startup/direct-submit/recovery derive `required_replica_count` from declared-handler metadata, and unsupported higher fanout is durably rejected inside the continuity state machine instead of being silently clipped down to the old one-replica behavior. The single-node route-free startup carveout stayed explicit, so bare `@cluster` remains usable on one node while still surfacing `replication_count=2` as public truth. T03 closed the user-facing proof seam with a new `compiler/meshc/tests/e2e_m047_s02.rs` rail. That rail builds an ordinary route-free Mesh app using `@cluster` and `@cluster(3)`, archives LLVM beside a temp output binary, and proves three things end to end: generic runtime names like `Work.handle_submit` and `Work.handle_retry` survive registration, bare `@cluster` surfaces `replication_count=2` through `meshc cluster continuity`, and explicit higher fanout stays visible as `replication_count=3` with an explicit `unsupported_replication_count:3` rejection instead of pretending the runtime honored it. The shared M046 route-free rail also stayed green, so the slice delivered replication-count semantics without regressing the existing startup-work contract.

## Verification

Replayed every slice-level rail from the plan. `cargo test -p mesh-codegen m047_s02 -- --nocapture` passed 6 tests proving declared-handler count threading and codegen marker emission. `cargo test -p mesh-rt m047_s02 -- --nocapture` passed 6 tests proving continuity records preserve `replication_count`, runtime lookup derives required replicas from declared-handler metadata, and unsupported higher fanout is durably rejected with explicit diagnostics. `cargo test -p meshc --test e2e_m047_s02 -- --nocapture` passed 2 tests proving emitted LLVM registration markers, generic runtime names, continuity JSON/human truth, single-record output, and startup diagnostics for ordinary `@cluster` functions. `cargo test -p meshc --test e2e_m046_s02 m046_s02_ -- --nocapture` passed 7 tests, confirming the shared route-free startup/continuity contract stayed green after the S02 count-bearing record changes.

## Requirements Advanced

- R099 — S02 proved that ordinary non-HTTP functions still cluster through the new source-first syntax and generic runtime-name registry, preserving the general clustered-function model that route wrappers will consume next.

## Requirements Validated

- R098 — Validated by `cargo test -p mesh-codegen m047_s02 -- --nocapture`, `cargo test -p mesh-rt m047_s02 -- --nocapture`, and `cargo test -p meshc --test e2e_m047_s02 -- --nocapture`, which prove bare `@cluster` registers and surfaces `replication_count=2`, explicit `@cluster(3)` preserves `replication_count=3`, and unsupported higher fanout remains durably queryable instead of being silently downgraded.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Operational Readiness

- **Health signal**: `meshc cluster continuity --json` and the human continuity output now surface generic `runtime_name` plus public `replication_count` for ordinary source-declared clustered functions, and the default `@cluster` record completes through the existing single-node startup carveout instead of disappearing into implicit runtime behavior.
- **Failure signal**: unsupported higher fanout leaves an explicit rejected continuity record plus diagnostics/log text such as `unsupported_replication_count:3`; missing `runtime_name` / `replication_count` fields in CLI output is also a direct contract failure.
- **Recovery**: supported bare `@cluster` continues through the existing single-node startup carveout when no peer is present. Unsupported higher fanout does not self-heal on the current runtime; the operator must lower the requested count or wait for a later runtime that can honestly satisfy more than one required replica.
- **Monitoring gaps**: this slice improves CLI/diagnostic truth, not metrics. There is still no dedicated metric surface for replication-count drift, and there is still no successful >1 required-replica execution path to monitor.

## Deviations

None in scope. During T03, the proof rail had to archive emitted LLVM from the explicit `--output` directory because `meshc build --emit-llvm --output <binary>` writes `output.ll` beside the requested binary rather than under the temp project root.

## Known Limitations

The runtime still honestly supports only one required replica today, so `@cluster(3)` remains durably rejected with `unsupported_replication_count:3` instead of succeeding as a true three-copy execution path. Route-local `HTTP.clustered(...)` wrappers are not landed yet, so S03 still needs to prove that HTTP routes lower onto the same general clustered-function capability. The GSD requirements DB also still rejects the M047 requirement family as unknown, so saved requirement decisions are authoritative even when `gsd_requirement_update` cannot mark `R098` directly.

## Follow-ups

S03 should lower `HTTP.clustered(handler)` / `HTTP.clustered(N, handler)` onto the same declared-handler registration and continuity record surfaces introduced here instead of inventing route-local count metadata. S04 should reuse the new runtime-name + `replication_count` truth surface when migrating examples, scaffolds, and docs away from `clustered(work)` and `.toml` clustering.

## Files Created/Modified

- `compiler/mesh-codegen/src/declared.rs` — Threaded resolved `replication_count` through declared-handler planning/runtime metadata and kept startup registration separate from count-bearing declared-handler registration.
- `compiler/mesh-codegen/src/codegen/mod.rs` — Emitted declared-handler registration markers with runtime name, executable symbol, and replication count; rejected missing lowered symbols instead of defaulting silently.
- `compiler/mesh-rt/src/dist/node.rs` — Stored declared-handler replication counts keyed by runtime name and derived required replica counts for startup/direct submit/recovery from that registry.
- `compiler/mesh-rt/src/dist/continuity.rs` — Preserved public `replication_count` in continuity records and durable rejection state, including unsupported higher fanout.
- `compiler/meshc/src/cluster.rs` — Surfaced runtime-owned continuity truth through `meshc cluster continuity` JSON/human output, including runtime name and replication count.
- `compiler/meshc/tests/e2e_m047_s02.rs` — Added the end-to-end M047 proof rail for ordinary `@cluster` functions and retained LLVM/runtime artifacts under `.tmp/m047-s02/...`.
- `.gsd/PROJECT.md` — Refreshed current-state project text to reflect that M047/S02 landed and the remaining work is the route-wrapper/cutover wave.
- `.gsd/KNOWLEDGE.md` — Recorded the LLVM output-path gotcha and the current M047 requirements-DB mismatch for future closeout work.

## Forward Intelligence

### What the next slice should know
- S03 can treat declared-handler `runtime_name` + `replication_count` as the single runtime source of truth for clustered execution semantics. Do not add route-local shadow metadata when `HTTP.clustered(...)` lands.
- The authoritative M047 proof rail is `compiler/meshc/tests/e2e_m047_s02.rs`, and its retained bundles under `.tmp/m047-s02/...` already preserve the exact LLVM, continuity, and diagnostics shapes that downstream route-wrapper work should keep stable.

### What's fragile
- `meshc build --emit-llvm --output <binary>` writes `output.ll` beside the explicit output target, not under the temp project root. A missing LLVM artifact can be a harness-path bug rather than real codegen drift.
- The current runtime still rejects any path that would need more than one required replica. That is honest current behavior, but it means route-wrapper work must preserve explicit rejection truth until the runtime capability expands.

### Authoritative diagnostics
- `.tmp/m047-s02/cli-source-cluster-counts-*/cluster-continuity-*.json` and `cluster-diagnostics-json.json` — these retain the user-facing continuity truth and rejection reason exactly as `meshc cluster continuity` / diagnostics expose them.
- `cargo test -p mesh-rt m047_s02 -- --nocapture` — this is the fastest authoritative seam for regressions in continuity-record encode/decode and declared-handler `required_replica_count` derivation.

### What assumptions changed
- The numeric argument on `@cluster(N)` is now public replication-count truth, not a vague execution-width hint or startup-only heuristic.
- Unsupported higher fanout is no longer silently downgraded to the old single-replica behavior; it remains durably queryable as an explicit rejection.
