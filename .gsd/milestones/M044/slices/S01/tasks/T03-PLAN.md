---
estimated_steps: 30
estimated_files: 6
skills_used: []
---

# T03: Stopped T03 after confirming the typed continuity builtin seam from T02 never landed; wrote precise resume notes instead of shipping a fake cluster-proof rewrite.

---
estimated_steps: 4
estimated_files: 6
skills_used:
  - test
---

Prove the new public surface is real by rewriting the live proof app onto it. S01 is not done if the compiler/runtime change lands but `cluster-proof` still parses continuity JSON or lacks a real clustered-app manifest.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `cluster-proof` manifest opt-in and declaration contract | Fail the build with the exact manifest/declaration error instead of silently falling back to the old proof-app path. | N/A — static config. | Reject stale or misspelled declarations rather than broadening the clustered boundary implicitly. |
| Typed `Continuity.*` surface | Stop on compile/runtime failure and keep the proof-app logs; do not reintroduce `from_json` wrappers. | Fail the app tests and preserve logs/artifacts. | Treat any need to decode runtime continuity JSON as a regression in the public API contract. |
| Existing `cluster-proof` HTTP JSON responses | Keep HTTP shaping local to the app payload structs while avoiding builtin continuity JSON traits as a hidden dependency. | Fail the proof-app tests/build if response shaping drifts. | Reject malformed payloads or missing fields instead of hiding the typed/runtime boundary behind another opaque shim. |

## Load Profile

- **Shared resources**: `cluster-proof` build/test compilation, continuity runtime calls, and local HTTP JSON shaping.
- **Per-operation cost**: one package build, one package test run, and targeted grep checks over the continuity adapter layer.
- **10x breakpoint**: build/test churn and stale shims break before runtime load matters; the main risk is drift back to JSON decoding.

## Negative Tests

- **Malformed inputs**: missing clustered opt-in, undeclared handler targets, and stale helper names that still expect raw JSON strings.
- **Error paths**: runtime continuity rejection, authority unavailability, and status-not-found must still surface through the typed API without decode adapters.
- **Boundary conditions**: typed nested record reads, local HTTP payload encoding, and the line between clustered handlers and ordinary local code inside `cluster-proof`.

## Steps

1. Add `cluster-proof/mesh.toml` with the clustered opt-in and declared handler boundary defined in T01, keeping the package metadata valid for ordinary builds.
2. Rewrite `cluster-proof/work_continuity.mpl` to consume typed `Continuity` values directly, deleting the runtime continuity `parse_*_json` helpers and app-level `from_json` calls on authority/record/submit payloads.
3. Update `cluster-proof/work.mpl`, `cluster-proof/main.mpl`, and any package tests that still assume stringly continuity responses so the app boundary stays honest.
4. Extend the M044 proof rail with a `cluster_proof_`-prefixed build/test case or equivalent package assertions so the verifier can prove the rewritten consumer contract directly.

## Must-Haves

- [ ] `cluster-proof` has a real `mesh.toml` clustered-app opt-in and declared handler surface.
- [ ] Runtime continuity payloads are consumed as typed Mesh values; only HTTP response payloads remain locally JSON-shaped.
- [ ] No runtime continuity `from_json` helper survives in `cluster-proof/work_continuity.mpl`.

## Inputs

- ``cluster-proof/work_continuity.mpl` — current app-owned continuity JSON adapter layer.`
- ``cluster-proof/work.mpl` — current request/record payload structs and clustered-work helpers.`
- ``cluster-proof/main.mpl` — live proof-app entrypoint and HTTP wiring.`
- ``cluster-proof/tests/work.test.mpl` — package tests that guard the proof-app contract.`
- ``compiler/meshc/tests/e2e_m044_s01.rs` — typed compiler/runtime rail from T02 to extend with consumer proof coverage.`

## Expected Output

- ``cluster-proof/mesh.toml` — clustered-app opt-in and declaration contract for the proof app.`
- ``cluster-proof/work_continuity.mpl` — typed continuity consumer with runtime JSON decode shims removed.`
- ``cluster-proof/work.mpl` — proof-app clustered-work types/callers aligned with the typed public surface.`
- ``cluster-proof/main.mpl` — entrypoint/wiring aligned with the manifest-backed clustered contract.`
- ``cluster-proof/tests/work.test.mpl` — package tests updated for the typed surface.`
- ``compiler/meshc/tests/e2e_m044_s01.rs` — consumer-oriented `cluster_proof_` coverage or equivalent assertions.`

## Verification

`cargo run -q -p meshc -- build cluster-proof`
`cargo run -q -p meshc -- test cluster-proof/tests`
`! rg -n 'ContinuityAuthorityStatus\.from_json|ContinuitySubmitDecision\.from_json|WorkRequestRecord\.from_json|parse_authority_status_json|parse_continuity_submit_response|parse_continuity_record' cluster-proof/work_continuity.mpl`

## Observability Impact

- Signals added/changed: `cluster-proof` build/test failures should now point at typed continuity usage or declaration drift, not JSON decode helpers.
- How a future agent inspects this: rerun `meshc build/test cluster-proof` and inspect any retained proof-app logs plus the targeted grep result.
- Failure state exposed: whether the regression is manifest/declaration drift, typed continuity usage, or stale proof-app adapter code.
