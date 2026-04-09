---
estimated_steps: 4
estimated_files: 4
skills_used: []
---

# T02: Replace handler-side waiting with a coordinator service and scalar-only cluster return path

1. Refactor `cluster-proof/work.mpl` so the HTTP handler only captures request context and calls a local registered coordinator service; remove `self()` / `receive` from the HTTP handler path.
2. Add an ingress-owned coordinator/result-registry pattern using local `Process.register(...)` and distributed `Global.register(...)` / lookup where needed so remote work can report completion back without shipping handler pids or other local-only handles.
3. Keep all cross-node spawn/send inputs scalar-only (`request_token`, membership indexes, booleans, and other raw-safe correlation values) and reconstruct string-heavy JSON fields on the ingress node before returning the HTTP response.
4. Leave `/membership` untouched, repair `cluster-proof/tests/work.test.mpl` around pure selection/correlation helpers, and get `cluster-proof` building again.

## Inputs

- `.gsd/milestones/M039/slices/S02/tasks/T01-SUMMARY.md`
- `cluster-proof/main.mpl`
- `cluster-proof/cluster.mpl`
- `cluster-proof/work.mpl`
- `cluster-proof/tests/work.test.mpl`
- `mesher/api/helpers.mpl`
- `mesher/ingestion/pipeline.mpl`

## Expected Output

- `cluster-proof/work.mpl compiles with a coordinator-backed /work path`
- `cluster-proof/tests/work.test.mpl covers pure selection/correlation helpers and passes`
- `cluster-proof builds with /membership unchanged and /work restored on a runtime-supported return path`

## Verification

cargo run -q -p meshc -- test cluster-proof/tests
cargo run -q -p meshc -- build cluster-proof

## Observability Impact

Adds durable routing correlation surfaces (`request_id`, coordinator/result-registry logs, and explicit timeout/malformed-result states) on a runtime-supported actor boundary so future failures are inspectable without touching /membership.
