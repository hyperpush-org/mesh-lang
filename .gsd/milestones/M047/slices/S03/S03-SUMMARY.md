---
id: S03
parent: M047
milestone: M047
provides:
  - Truthful blocker mapping for the missing `HTTP.clustered(...)` compiler/lowering/runtime seam.
  - A repaired self-contained M032 route-limit control harness that future clustered-route work can lean on without `.tmp` fixture drift.
  - A recorded verifier rule that zero-test `m047_s03` filter runs are not implementation proof.
requires:
  - slice: S01
    provides: Source-first `@cluster` parsing, provenance, diagnostics, and LSP ranges that the planned route wrapper is supposed to reuse.
  - slice: S02
    provides: Runtime registration and continuity replication-count truth for ordinary clustered declarations, which the planned route wrapper is supposed to reuse.
affects:
  - S04
  - S05
  - S06
key_files:
  - compiler/meshc/tests/e2e_stdlib.rs
  - .gsd/milestones/M047/slices/S03/tasks/T01-SUMMARY.md
  - .gsd/milestones/M047/slices/S03/tasks/T02-SUMMARY.md
  - .gsd/milestones/M047/slices/S03/tasks/T03-SUMMARY.md
  - .gsd/milestones/M047/slices/S03/tasks/T04-SUMMARY.md
  - .gsd/milestones/M047/slices/S03/tasks/T05-SUMMARY.md
  - .gsd/KNOWLEDGE.md
  - .gsd/PROJECT.md
key_decisions:
  - Do not model `HTTP.clustered(...)` as a plain stdlib helper; it needs a compiler-known metadata seam from type inference into lowering.
  - If clustered HTTP routes land, they should reuse generated bare route shims, synthetic declared-handler registrations, and a service-style reply transport rather than generic route-closure ABI widening or submit/status polling.
  - Treat the `m047_s03` Cargo filters in mesh-typeck, mesh-codegen, and mesh-rt as presence checks only; the authoritative blocker signal is still the missing `e2e_m047_s03` target plus absent `HTTP.clustered(...)` symbols.
patterns_established:
  - When a slice-level proof target is still missing, keep the surrounding control rails truthful and fail closed on the absent target instead of inventing a placeholder verifier.
  - Retained integration controls in `compiler/meshc/tests/e2e_stdlib.rs` should embed stable Mesh source directly rather than depending on transient `.tmp/...` fixtures, so unrelated cleanup does not make the whole target stop compiling.
observability_surfaces:
  - `cargo test -p meshc --test e2e_m047_s02 -- --nocapture` remains the positive control for ordinary `@cluster` continuity/runtime-name/count truth.
  - `cargo test -p meshc --test e2e_stdlib e2e_m032_route_bare_handler_control -- --nocapture` and `cargo test -p meshc --test e2e_stdlib e2e_m032_route_closure_runtime_failure -- --nocapture` remain the guardrails that bare handler routes still work while generic closure routes still fail only at live request time.
  - `cargo test -p meshc --test e2e_m047_s03 -- --nocapture` is the authoritative negative signal right now: it fails with `no test target named e2e_m047_s03`.
  - `rg -n 'HTTP\.clustered|http_clustered' compiler/mesh-typeck/src compiler/mesh-codegen/src compiler/mesh-rt/src compiler/meshc/tests -g '*.rs'` returning no matches is the fast static proof that the wrapper surface still has not landed.
drill_down_paths:
  - .gsd/milestones/M047/slices/S03/tasks/T01-SUMMARY.md
  - .gsd/milestones/M047/slices/S03/tasks/T02-SUMMARY.md
  - .gsd/milestones/M047/slices/S03/tasks/T03-SUMMARY.md
  - .gsd/milestones/M047/slices/S03/tasks/T04-SUMMARY.md
  - .gsd/milestones/M047/slices/S03/tasks/T05-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-01T08:50:08.144Z
blocker_discovered: true
---

# S03: Clustered HTTP route wrapper

**S03 closed as a blocker-accounting slice: it did not ship `HTTP.clustered(...)`, but it preserved the compiler/runtime seam, repaired the retained M032 route-control harness, and left the missing wrapper surface fail-closed.**

## What Happened

S03 did not ship the planned clustered HTTP route-wrapper feature. T01 and T02 mapped the real compiler seam: `HTTP.clustered(...)` is still absent from the stdlib/typeck surfaces, so the clean implementation path is a compiler-known wrapper contract in `infer_call(...)` plus a structured metadata map threaded from inference through `TypeckResult` into lowering, with diagnostics kept in sync with mesh-lsp. T03 then verified that the lowerer and build planner still have nowhere to consume that metadata: there are no clustered-route symbols, no synthetic route-shim registrations, and `PreparedBuild.clustered_execution_plan` still only handles ordinary clustered declarations. T04 confirmed the runtime side of the same gap: there is still no clustered route execution path, no service-style reply seam on behalf of route handlers, and no `e2e_m047_s03` target proving the wrapper. The only actual code that shipped in the slice came from T05, which repaired the retained M032 route-limit controls by embedding stable Mesh sources directly in `compiler/meshc/tests/e2e_stdlib.rs` so those controls no longer depend on transient `.tmp` fixtures. The slice therefore closes as honest blocker documentation plus proof-surface maintenance: it preserved the implementation seam for the next slice, kept the surrounding control rails green, and explicitly fail-closed on the still-missing wrapper surface instead of fabricating a placeholder route-wrapper story.

## Verification

Replayed the current slice-level rails and recorded the actual state of the tree. `cargo test -p mesh-typeck m047_s03 -- --nocapture`, `cargo test -p mesh-codegen m047_s03 -- --nocapture`, and `cargo test -p mesh-rt m047_s03 -- --nocapture` all exited 0 while filtering to zero matching tests; they are presence checks only, not proof that the wrapper exists. `cargo test -p mesh-lsp -- --nocapture` passed (54 tests). `cargo test -p meshc --test e2e_m047_s02 -- --nocapture` passed (2 tests), confirming ordinary `@cluster` count/runtime-name continuity truth still holds. `cargo test -p meshc --test e2e_stdlib e2e_m032_route_bare_handler_control -- --nocapture` and `cargo test -p meshc --test e2e_stdlib e2e_m032_route_closure_runtime_failure -- --nocapture` both passed after the self-contained source repair in `e2e_stdlib.rs`. The authoritative blocker command, `cargo test -p meshc --test e2e_m047_s03 -- --nocapture`, still fails with `error: no test target named 'e2e_m047_s03' in 'meshc' package`. I also confirmed there are still no `HTTP.clustered(...)` / `http_clustered` hits in `compiler/mesh-typeck/src`, `compiler/mesh-codegen/src`, `compiler/mesh-rt/src`, or `compiler/meshc/tests`.

## Requirements Advanced

None.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Operational Readiness

- **Health signal**: the only positive runtime-adjacent controls for this slice are still the surrounding guardrails — `cargo test -p meshc --test e2e_m047_s02 -- --nocapture` plus the retained M032 bare-handler and closure-route controls. There is no positive `HTTP.clustered(...)` runtime health signal yet because the feature does not exist.
- **Failure signal**: `cargo test -p meshc --test e2e_m047_s03 -- --nocapture` fails with `no test target named e2e_m047_s03`, and `rg -n 'HTTP\.clustered|http_clustered' compiler/mesh-typeck/src compiler/mesh-codegen/src compiler/mesh-rt/src compiler/meshc/tests -g '*.rs'` returns no implementation hits.
- **Recovery**: implement the missing compiler-known wrapper surface, lowering metadata, runtime reply path, and live e2e target before treating clustered HTTP routes as shipped; until then, keep the repaired M032 controls and S02 ordinary-`@cluster` rail green.
- **Monitoring gaps**: there is no live clustered-route runtime, no continuity record for clustered HTTP handlers, and no request/reply diagnostics for the planned wrapper path yet.

## Deviations

The written plan assumed S03 would deliver a real `HTTP.clustered(...)` compiler/runtime path plus a live `e2e_m047_s03` rail. That implementation never landed in this checkout. The slice closed instead as a blocker-accounting unit: T01–T04 mapped and verified the missing seams without shipping the wrapper, and T05 only repaired the retained M032 route-limit controls so the surrounding proof surface stayed truthful.

## Known Limitations

`HTTP.clustered(...)` still does not exist in the compiler, lowerer, runtime, or CLI proof surface. There are no clustered route shim symbols, `PreparedBuild.clustered_execution_plan` still only reflects ordinary clustered declarations, `compiler/mesh-rt/src/http/server.rs` still executes matched handlers through the direct `call_handler(...)` path, and `cargo test -p meshc --test e2e_m047_s03 -- --nocapture` still fails with `no test target named e2e_m047_s03`. This slice therefore did not deliver the planned clustered HTTP route wrapper feature.

## Follow-ups

1. Implement `HTTP.clustered(handler)` / `HTTP.clustered(N, handler)` as a compiler-known surface in mesh-typeck, with dedicated metadata parallel to `overloaded_call_targets` and misuse diagnostics anchored through mesh-lsp.
2. Thread that metadata into MIR lowering so `PreparedBuild.clustered_execution_plan` can synthesize clustered route shims and declared-handler registrations instead of only ordinary `@cluster` declarations.
3. Add the runtime reply path for `Request -> Response` clustered route execution and a real `compiler/meshc/tests/e2e_m047_s03.rs` target that proves the route handler is the clustered boundary.
4. Keep the repaired M032 controls green while the wrapper work lands; they are the current guard against accidentally widening generic route-closure support while trying to add wrapper-local clustering.

## Files Created/Modified

- `compiler/meshc/tests/e2e_stdlib.rs` — Embedded self-contained Mesh sources for the retained M032 bare-handler and closure-route control rails so the test target no longer depends on transient `.tmp/m032-s01/...` fixtures.
- `.gsd/milestones/M047/slices/S03/tasks/T01-SUMMARY.md` — Recorded the initial typeck seam for `HTTP.clustered(...)` and the missing metadata handoff from inference into lowering.
- `.gsd/milestones/M047/slices/S03/tasks/T02-SUMMARY.md` — Recorded the narrowed T02 implementation seam: compiler-known wrapper typing plus post-inference validation and metadata threading.
- `.gsd/milestones/M047/slices/S03/tasks/T03-SUMMARY.md` — Recorded that lowering/registration work is still blocked because `PreparedBuild.clustered_execution_plan` has no clustered-route metadata and there are no `m047_s03` compiler rails.
- `.gsd/milestones/M047/slices/S03/tasks/T04-SUMMARY.md` — Recorded that runtime clustered-route execution did not ship because the compiler/lowering wrapper seam and `e2e_m047_s03` target are still absent.
- `.gsd/milestones/M047/slices/S03/tasks/T05-SUMMARY.md` — Recorded the repaired M032 controls and the fail-closed `e2e_m047_s03` blocker state.
- `.gsd/KNOWLEDGE.md` — Preserved the useful S03 verifier lesson that the `m047_s03` Cargo filters in typeck/codegen/runtime are zero-test presence checks, not implementation proof.
- `.gsd/PROJECT.md` — Refreshed the project-level current-state summary so M047 now explicitly notes that S03 did not land the wrapper and only repaired the retained route-control harness.
