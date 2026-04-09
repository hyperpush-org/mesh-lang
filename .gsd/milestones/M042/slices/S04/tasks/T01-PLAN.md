---
estimated_steps: 26
estimated_files: 5
skills_used: []
---

# T01: Split `cluster-proof` into shared placement, legacy probe, and runtime continuity modules without changing the keyed submit/status contract.

Make the proof app visibly thin before touching operator scripts or docs.

## Why

`cluster-proof/work.mpl` already consumes the runtime-native continuity API, but the file still hides that fact by mixing legacy route proof, placement helpers, keyed submit/status HTTP translation, and work-execution plumbing in one module. S04 should leave a reader with an obvious seam: Mesh owns placement and HTTP adaptation, the runtime owns continuity state and recovery truth.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `cluster-proof` handler split across `main.mpl` and `work.mpl` | Fail closed at compile/test time; do not leave routes pointing at stale helpers. | Not applicable beyond ordinary test/build timeout; the refactor should not add polling. | Keep invalid runtime JSON on the existing parse-failure path instead of inventing fallback continuity state. |
| Runtime-owned `Continuity.submit/status/mark_completed` contract | Preserve the current API and log/status mapping exactly; do not widen or shadow it in Mesh code. | Keep the existing runtime timeout/error payload behavior. | Treat malformed continuity JSON as an explicit failure payload, not as a legacy-probe success path. |

## Load Profile

- **Shared resources**: `cluster-proof` route handlers, runtime continuity JSON parsing, and keyed-work log volume.
- **Per-operation cost**: one local module dispatch plus the existing runtime continuity calls; this task should stay structurally neutral at runtime.
- **10x breakpoint**: confusion and regressions show up first in handler wiring and log/status mismatches, not in raw performance.

## Negative Tests

- **Malformed inputs**: invalid submit JSON, blank payloads, malformed request keys, and invalid continuity JSON from the runtime parser seam.
- **Error paths**: rejected continuity records, duplicate same-key submits, conflict responses, and invalid target selection still map to the right HTTP payloads after the split.
- **Boundary conditions**: single-node fallback, deterministic multi-node placement, legacy `GET /work` probe behavior, and owner-loss submit downgrading still behave exactly as before.

## Steps

1. Split the legacy `GET /work` probe helpers from the keyed continuity submit/status helpers so the exported route handlers and worker helpers read as two distinct concerns.
2. Keep placement and runtime `Continuity.*` calls in Mesh, but do not move continuity semantics, dedupe, or recovery logic out of `mesh-rt`.
3. Update `cluster-proof/main.mpl` and `cluster-proof/tests/work.test.mpl` so legacy probe and keyed continuity seams are exercised separately.
4. Preserve the current log/status surfaces and fail closed on any parser or handler mismatch.

## Must-Haves

- [ ] `cluster-proof` code reads as placement plus `Continuity.*` adaptation, not as app-authored continuity orchestration.
- [ ] Legacy `GET /work` remains available but clearly isolated from keyed submit/status code.
- [ ] No new Mesh-side continuity state machine or recovery shim is introduced.
- [ ] `cluster-proof` tests and build stay green after the refactor.

## Inputs

- ``cluster-proof/work.mpl``
- ``cluster-proof/main.mpl``
- ``cluster-proof/tests/work.test.mpl``
- ``cluster-proof/Cluster.mpl``
- ``cluster-proof/Config.mpl``

## Expected Output

- ``cluster-proof/work.mpl``
- ``cluster-proof/main.mpl``
- ``cluster-proof/tests/work.test.mpl``
- ``cluster-proof/WorkLegacy.mpl``
- ``cluster-proof/WorkContinuity.mpl``

## Verification

cargo run -q -p meshc -- test cluster-proof/tests && cargo run -q -p meshc -- build cluster-proof

## Observability Impact

Keeps `[cluster-proof]` legacy-probe logs distinct from keyed continuity submit/status/dispatch logs so future failures localize to the thin consumer seam instead of one mixed module.
