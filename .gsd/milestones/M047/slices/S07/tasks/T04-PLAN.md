---
estimated_steps: 4
estimated_files: 2
skills_used:
  - rust-best-practices
  - debug-like-expert
  - test
---

# T04: Prove clustered HTTP routes end to end on a two-node app and preserve the M032 route guardrails

**Slice:** S07 — Clustered HTTP route wrapper completion
**Milestone:** M047

## Description

Close the slice with a dedicated two-node HTTP proof rail instead of adopting the wrapper in scaffold/docs yet. The e2e should build a temp multi-module package, hit both success and unsupported-count routes, inspect continuity by before/after diff rather than list order, and keep the bare-function and closure-function M032 controls green.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| temp-project build and two-node bootstrap | fail with retained build/runtime artifacts; do not reuse scaffold/docs rails or manual repros as substitutes | bound process readiness and HTTP waits, then fail the named e2e with retained logs | malformed runtime bootstrap output is failure evidence, not acceptable best effort |
| HTTP request plus continuity inspection harness | capture before/after continuity snapshots and fail closed on missing request keys or runtime-name drift | keep bounded poll loops for HTTP readiness and continuity completion | malformed HTTP or CLI JSON should fail the rail rather than being treated as empty success |
| retained M032 route controls | rerun the existing bare-handler and closure-handler controls unchanged; do not widen generic route closure support while landing `HTTP.clustered(...)` | use the existing e2e timeouts | any unexpected closure success is a regression, not a flaky pass |

## Load Profile

- **Shared resources**: dual-stack cluster ports, temp project directories, HTTP ports, continuity query artifacts, and retained `.tmp/m047-s07` bundles.
- **Per-operation cost**: one temp-project build, two runtime processes, a small number of HTTP requests, and continuity/diagnostic queries.
- **10x breakpoint**: port collisions, readiness waits, and continuity diff heuristics fail before throughput matters.

## Negative Tests

- **Malformed inputs**: missing or unstable request keys, malformed continuity list JSON, and imported bare handlers that lose their defining-module runtime name.
- **Error paths**: the explicit-count route must surface the chosen HTTP failure contract plus durable rejection, and the success route must fail if continuity never reaches `completed/succeeded`.
- **Boundary conditions**: repeated requests against the same runtime name, default-count success on two nodes, and the unchanged M032 closure failure control all stay truthful together.

## Steps

1. Add `compiler/meshc/tests/e2e_m047_s07.rs` that builds a temp app with imported bare handlers and both `HTTP.clustered(handler)` / `HTTP.clustered(3, handler)` route forms, then boots a two-node cluster and sends live HTTP requests.
2. Reuse or extend `compiler/meshc/tests/support/m046_route_free.rs` helpers so the harness records before/after continuity snapshots keyed by request key/runtime name instead of assuming list order, and archive build/runtime/CLI artifacts under `.tmp/m047-s07/...`.
3. Assert the success route returns HTTP 200 with continuity truth `declared_handler_runtime_name=<actual handler>`, `replication_count=2`, `phase=completed`, `result=succeeded`; assert the explicit-count route returns the chosen HTTP 503/rejection contract with durable `unsupported_replication_count:3`; rerun `e2e_m032_route_*` unchanged.
4. Leave scaffold/docs adoption untouched in this slice; S08 owns migration of public surfaces to the shipped wrapper.

## Must-Haves

- [ ] `e2e_m047_s07` proves both the default-count success path and the unsupported explicit-count failure path against a live clustered HTTP app.
- [ ] Continuity inspection does not depend on list order and preserves the actual imported handler runtime name.
- [ ] The existing M032 bare-handler success and closure-handler failure controls remain green.

## Verification

- `cargo test -p meshc --test e2e_m047_s07 -- --nocapture`
- `cargo test -p meshc --test e2e_stdlib e2e_m032_route_ -- --nocapture`

## Observability Impact

- Signals added/changed: retained `.tmp/m047-s07/...` bundles with build logs, runtime stdout/stderr, HTTP responses, and continuity JSON snapshots.
- How a future agent inspects this: rerun the named e2e and compare the archived before/after continuity snapshots plus HTTP results.
- Failure state exposed: runtime-name drift, missing request keys, readiness timeouts, and unsupported-count rejection all surface as named e2e failures.

## Inputs

- `compiler/meshc/tests/support/m046_route_free.rs` — cluster bootstrap, continuity, and retained-artifact helpers.
- `compiler/meshc/tests/support/m047_todo_scaffold.rs` — reusable HTTP request and artifact utilities.
- `compiler/meshc/tests/e2e_stdlib.rs` — existing M032 bare-route and closure-route controls that must stay green.
- `reference-backend/api/router.mpl` — imported bare-handler router shape to mirror in the temp project.

## Expected Output

- `compiler/meshc/tests/e2e_m047_s07.rs` — dedicated two-node clustered HTTP route proof rail with retained artifacts.
- `compiler/meshc/tests/support/m046_route_free.rs` — helper additions for continuity snapshot diffing and runtime-name-aware record discovery.
