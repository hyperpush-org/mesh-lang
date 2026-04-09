---
id: S07
parent: M047
milestone: M047
provides:
  - A shipped compiler-known `HTTP.clustered(handler)` / `HTTP.clustered(N, handler)` surface with wrapper-local diagnostics and imported-handler runtime-name preservation.
  - Deterministic `__declared_route_<runtime_name>` lowering and declared-handler registration reuse for clustered HTTP routes without startup-work leakage.
  - Runtime clustered HTTP dispatch that treats the route handler as the clustered boundary and reports continuity truth through the shared runtime-name/replication-count fields.
  - A dedicated two-node `e2e_m047_s07` proof rail plus retained M032 route controls that downstream adoption work can reuse.
requires:
  - slice: S02
    provides: The shared declared-handler runtime-name/replication-count seam and continuity truth that S07 reuses for clustered HTTP routes instead of inventing route-local shadow metadata.
  - slice: S04
    provides: The hard source-first cutover plus explicit public non-goal boundary that let S07 ship the real wrapper implementation first while leaving scaffold/docs adoption to S08.
affects:
  - S08
key_files:
  - compiler/mesh-typeck/src/infer.rs
  - compiler/mesh-typeck/src/lib.rs
  - compiler/mesh-typeck/src/error.rs
  - compiler/mesh-typeck/src/diagnostics.rs
  - compiler/mesh-lsp/src/analysis.rs
  - compiler/mesh-typeck/tests/http_clustered_routes.rs
  - compiler/mesh-codegen/src/mir/lower.rs
  - compiler/mesh-codegen/src/declared.rs
  - compiler/mesh-codegen/src/codegen/mod.rs
  - compiler/mesh-rt/src/http/router.rs
  - compiler/mesh-rt/src/http/server.rs
  - compiler/mesh-rt/src/dist/node.rs
  - compiler/meshc/tests/e2e_m047_s07.rs
  - compiler/meshc/tests/support/m046_route_free.rs
  - .gsd/DECISIONS.md
  - .gsd/KNOWLEDGE.md
  - .gsd/PROJECT.md
key_decisions:
  - Keep `HTTP.clustered(handler)` typed as the underlying handler while recording wrapper metadata keyed by wrapper span and validating route-slot consumption in a final post-inference sweep.
  - Lower clustered route wrappers directly to deterministic `__declared_route_<runtime_name>` bare shims and reuse the shared declared-handler runtime-name/replication-count registry through a dedicated route kind rather than startup work.
  - Reverse-map route shim function pointers back to declared-handler runtime metadata at router registration time and execute clustered HTTP routes through a dedicated transient request/response transport that reuses continuity submission/completion truth.
  - Use `cargo test -p meshc --test e2e_m047_s07 -- --nocapture` plus retained `.tmp/m047-s07/clustered-http-routes-two-node/` artifacts as the authoritative live proof surface, and keep `cargo test -p meshc --test e2e_stdlib e2e_m032_route_ -- --nocapture` as the unchanged route-closure guardrail.
patterns_established:
  - Record `HTTP.clustered(...)` metadata at the wrapper span during type checking, keep the wrapper typed as the underlying handler, and let lowering consume that metadata later instead of inventing a fake helper type.
  - Implement route-local clustering as sugar over the existing declared-handler runtime-name/replication-count seam by generating deterministic bare route shims rather than widening generic route-closure ABI surfaces.
  - Treat unsupported clustered-route fanout as an explicit failure contract: reject durably through continuity and return a 503 response instead of degrading silently into local direct invocation.
  - For repeated requests against one clustered route runtime name, diff before/after continuity snapshots by `request_key` plus runtime name rather than trusting list order.
observability_surfaces:
  - cargo test -p mesh-typeck m047_s07 -- --nocapture
  - cargo test -p mesh-lsp m047_s07 -- --nocapture
  - cargo test -p mesh-codegen m047_s07 -- --nocapture
  - cargo test -p mesh-rt m047_s07 -- --nocapture
  - cargo test -p meshc --test e2e_m047_s07 -- --nocapture
  - cargo test -p meshc --test e2e_stdlib e2e_m032_route_ -- --nocapture
  - .tmp/m047-s07/clustered-http-routes-two-node/cluster-status-primary.json
  - .tmp/m047-s07/clustered-http-routes-two-node/cluster-status-standby.json
  - .tmp/m047-s07/clustered-http-routes-two-node/route-success-first.json
  - .tmp/m047-s07/clustered-http-routes-two-node/route-success-second.json
  - .tmp/m047-s07/clustered-http-routes-two-node/route-unsupported-count.json
  - .tmp/m047-s07/clustered-http-routes-two-node/continuity-after-first-primary.json
  - .tmp/m047-s07/clustered-http-routes-two-node/continuity-after-second-primary.json
  - .tmp/m047-s07/clustered-http-routes-two-node/continuity-rejected-primary.json
  - .tmp/m047-s07/clustered-http-routes-two-node/primary.combined.log
  - .tmp/m047-s07/clustered-http-routes-two-node/standby.combined.log
drill_down_paths:
  - .gsd/milestones/M047/slices/S07/tasks/T01-SUMMARY.md
  - .gsd/milestones/M047/slices/S07/tasks/T02-SUMMARY.md
  - .gsd/milestones/M047/slices/S07/tasks/T03-SUMMARY.md
  - .gsd/milestones/M047/slices/S07/tasks/T04-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-02T00:52:42.945Z
blocker_discovered: false
---

# S07: Clustered HTTP route wrapper completion

**S07 shipped the real `HTTP.clustered(...)` route-wrapper path end to end: compiler-known typing/diagnostics, deterministic route-shim lowering onto the shared declared-handler seam, clustered HTTP dispatch/continuity truth, and a live two-node proof rail that keeps the retained M032 route guards green.**

## What Happened

S07 turned the earlier blocker map into the shipped clustered HTTP route path. On the compiler front, `HTTP.clustered(handler)` and `HTTP.clustered(N, handler)` are now compiler-known wrapper surfaces rather than unresolved HTTP-field lookups. Type checking keeps the wrapped handler type, records wrapper metadata keyed by the wrapper call span, preserves imported bare-handler origin so runtime names stay rooted in the defining module, and emits focused wrapper-local diagnostics for non-route-position use, closure/call-expression misuse, private handlers, conflicting replication counts, and missing imported origin metadata. `mesh-lsp` projects the same wrapper ranges so editors point at the `HTTP.clustered(...)` call instead of unrelated route tokens.

The lowering/codegen seam now consumes that metadata instead of reparsing route wrappers later. `HTTP.clustered(...)` lowers directly to deterministic bare shims such as `__declared_route_api_todos_handle_list_todos`, preserving the public `fn(Request) -> Response` signature while calling the real handler. Direct and pipe-form route registrations dedupe onto one shim, imported bare handlers keep their defining-module runtime identity, and route registrations reuse the ordinary declared-handler runtime-name/replication-count registry through a dedicated route kind so clustered HTTP handlers never leak into startup-work registration.

At runtime, router registration reverse-maps route shim function pointers back onto declared-handler runtime metadata, route entries now carry `declared_handler_runtime_name` and `replication_count`, and the HTTP server executes clustered routes through a dedicated transient request/response transport instead of falling back to local direct invocation. Default-count routes submit and complete the same continuity records used by ordinary clustered work, keyed to the actual handler runtime name. Unsupported explicit fanout rejects durably with HTTP 503 plus `unsupported_replication_count:3`, and continuity records keep the rejection reason truthful without leaking request bodies.

### Operational Readiness (Q8)
- **Health signal:** `cargo test -p meshc --test e2e_m047_s07 -- --nocapture` passes and retains `.tmp/m047-s07/clustered-http-routes-two-node/` with `cluster-status-*.json`, `route-success-*.json`, `continuity-*-completed-*.json`, and runtime logs showing both nodes bootstrapped and served HTTP; successful route records on both nodes report `declared_handler_runtime_name=Api.Todos.handle_list_todos`, `replication_count=2`, `phase=completed`, `result=succeeded`, `replica_status=mirrored`, and `execution_node == owner_node`.
- **Failure signal:** wrapper misuse now surfaces as named compiler/LSP diagnostics at the wrapper span; live unsupported-count routes return HTTP 503 with `{"error":"unsupported_replication_count:3"}` and continuity records enter `phase=rejected`, `result=rejected`, `error=unsupported_replication_count:3`; any silent local success on `/todos/retry`, missing request keys, or continuity drift between nodes is a real regression.
- **Recovery procedure:** rerun the slice rails in dependency order (`mesh-typeck`, `mesh-lsp`, `mesh-codegen`, `mesh-rt`, then `meshc --test e2e_m047_s07`), inspect `.tmp/m047-s07/clustered-http-routes-two-node/` starting with `scenario-meta.json`, `route-unsupported-count.json`, and the latest `continuity-after-*.json`, then check `compiler/mesh-rt/src/http/server.rs`, `compiler/mesh-rt/src/dist/node.rs`, and `compiler/mesh-codegen/src/mir/lower.rs` for request identity, route-shim registration, or dispatch drift before touching scaffold/docs adoption work.
- **Monitoring gaps:** clustered HTTP request keys are still node-local rather than cluster-global, so repeated same-runtime success traffic should stay on one ingress node during proof; higher fanout remains unimplemented beyond the durable rejection contract; and public docs/scaffold surfaces have not yet adopted the shipped wrapper, so the assembled closeout rail still stops at S06.

## Verification

Ran all slice-level rails and each passed.

- `cargo test -p mesh-typeck m047_s07 -- --nocapture` — passed (`7 passed, 248 filtered out`).
- `cargo test -p mesh-lsp m047_s07 -- --nocapture` — passed (`2 passed, 53 filtered out`).
- `cargo test -p mesh-codegen m047_s07 -- --nocapture` — passed (`8 passed, 194 filtered out`).
- `cargo test -p mesh-rt m047_s07 -- --nocapture` — passed (`8 passed, 643 filtered out`).
- `cargo test -p meshc --test e2e_m047_s07 -- --nocapture` — passed (`3 passed`) and retained the full `.tmp/m047-s07/clustered-http-routes-two-node/` evidence bundle.
- `cargo test -p meshc --test e2e_stdlib e2e_m032_route_ -- --nocapture` — passed (`2 passed, 104 filtered out`).

The runtime/e2e rails also confirmed the slice-specific observability contract: continuity inspection uses before/after request-key diffs instead of list order, successful `/todos` requests record the real imported handler runtime name on both nodes, and `/todos/retry` returns the chosen 503 contract with a durable `unsupported_replication_count:3` rejection.

## Requirements Advanced

- R099 — S07 routes `HTTP.clustered(...)` through the same declared-handler runtime-name/replication-count registry and continuity model already used by ordinary `@cluster` functions, via deterministic `__declared_route_<runtime_name>` shims and shared registration/dispatch paths.

## Requirements Validated

- R100 — `cargo test -p mesh-typeck m047_s07 -- --nocapture`, `cargo test -p mesh-lsp m047_s07 -- --nocapture`, `cargo test -p mesh-codegen m047_s07 -- --nocapture`, `cargo test -p mesh-rt m047_s07 -- --nocapture`, and `cargo test -p meshc --test e2e_m047_s07 -- --nocapture` now prove `HTTP.clustered(handler)` / `HTTP.clustered(N, handler)` typecheck, lower, execute, and surface continuity truth as real route-local clustering.
- R101 — The lowering/runtime/e2e rails preserve `Api.Todos.handle_list_todos` and `Api.Todos.handle_retry_todos` as the declared handler runtime names, register deterministic route shims around those handlers, and show live continuity records plus HTTP results keyed to the route handler itself rather than a downstream helper boundary.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

None to slice scope. The only notable harness constraint is deliberate: repeated successful clustered-route requests stay on one ingress node because route request keys are currently node-local, but the e2e still verifies that the same request key and continuity truth replicate across both cluster nodes.

## Known Limitations

`HTTP.clustered(3, handler)` and other explicit counts that require more than one replica still reject durably with HTTP 503 and continuity error `unsupported_replication_count:3`; S07 only ships the default two-node success path.

Clustered HTTP request keys are generated per process (`http-route::<runtime>::<seq>`), so repeated same-runtime traffic through different ingress nodes can collide until route identity becomes cluster-global.

Public docs, scaffold output, and the assembled S06 closeout verifier still intentionally teach route-free `@cluster` surfaces until S08 adopts the shipped wrapper truthfully.

## Follow-ups

Adopt the shipped wrapper into the Todo scaffold, README/VitePress docs, and the assembled closeout verifier in S08 so public surfaces match the compiler/runtime truthfully.

Decide whether clustered HTTP should eventually support higher fanout or continue to fail closed above replication count 2, then extend the runtime/e2e contract accordingly.

Consider promoting clustered HTTP request keys from node-local sequences to a cluster-global identity so repeated same-runtime traffic can be proven safely across multiple ingress nodes.

## Files Created/Modified

- `compiler/mesh-typeck/src/infer.rs` — Recognizes `HTTP.clustered(...)` as a compiler-known wrapper, validates wrapper argument shapes and route-slot use, and records clustered-route metadata keyed by wrapper span.
- `compiler/mesh-typeck/src/lib.rs` — Publishes clustered-route wrapper metadata on `TypeckResult` so lowering/codegen can reuse imported runtime names and replication counts.
- `compiler/mesh-typeck/src/error.rs` — Adds named `HTTP.clustered(...)` error variants for invalid wrapper usage, private handlers, non-route-position use, conflicting counts, and missing imported origin metadata.
- `compiler/mesh-typeck/src/diagnostics.rs` — Renders wrapper-local diagnostics and actionable help text instead of generic undefined-symbol fallout.
- `compiler/mesh-lsp/src/analysis.rs` — Projects the new wrapper diagnostics into LSP with range-accurate spans anchored on the wrapper expression and preserves imported-origin metadata in analysis results.
- `compiler/mesh-typeck/tests/http_clustered_routes.rs` — Adds focused source-level tests for direct/piped wrappers, imported bare handlers, default vs explicit counts, and closed failure cases.
- `compiler/mesh-codegen/src/mir/lower.rs` — Rewrites `HTTP.clustered(...)` directly to deterministic `__declared_route_<runtime_name>` bare shims and keeps those shims reachable through MIR merge.
- `compiler/mesh-codegen/src/declared.rs` — Builds route-specific declared-handler plan entries, dedupes matching wrappers, rejects conflicting counts, and keeps clustered route handlers out of startup-work registration.
- `compiler/mesh-codegen/src/codegen/mod.rs` — Emits route declared-handler registration markers so runtime registration preserves the real handler runtime name and replication count.
- `compiler/mesh-rt/src/http/router.rs` — Stores declared-handler runtime metadata on route entries at registration time so matched routes know whether they are clustered and what runtime/count they represent.
- `compiler/mesh-rt/src/http/server.rs` — Adds clustered HTTP request/response transport, request-key and payload-hash generation, truthful 503 rejection responses, and live clustered-route dispatch from the HTTP server.
- `compiler/mesh-rt/src/dist/node.rs` — Reverse-maps route shim function pointers to declared-handler metadata and executes clustered HTTP routes locally or remotely through the shared continuity/declared-handler seam.
- `compiler/meshc/tests/e2e_m047_s07.rs` — Adds the authoritative two-node clustered HTTP route proof rail with retained artifacts for success, rejection, continuity diffs, and runtime logs.
- `compiler/meshc/tests/support/m046_route_free.rs` — Extends the shared cluster-test support helpers so continuity diffs compare request keys per runtime name instead of depending on list order.
- `.gsd/DECISIONS.md` — Records the S07 verification-surface decision plus requirement-validation decisions for the shipped clustered HTTP wrapper.
- `.gsd/KNOWLEDGE.md` — Records the retained S07 artifact bundle and M032 companion rail as the first resume point for future clustered-route regressions.
- `.gsd/PROJECT.md` — Refreshes project state so the repo context reflects that `HTTP.clustered(...)` is shipped in compiler/runtime/e2e while public adoption is still deferred to S08.
