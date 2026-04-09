---
estimated_steps: 4
estimated_files: 7
skills_used:
  - rust-best-practices
  - test
---

# T04: Adopt the same bootstrap surface in `cluster-proof` and add the assembled S01 acceptance rail

**Slice:** S01 — Runtime-Owned Cluster Bootstrap
**Milestone:** M045

## Description

Prove the helper is real by consuming it in `cluster-proof`, trimming duplicate bootstrap/config/entrypoint logic, and finishing with one fail-closed verifier that replays the new M045 rails plus the protected M044 regression surfaces.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `cluster-proof` startup / Fly entrypoint contract | Preserve fail-closed startup on invalid local or Fly env; do not keep serving with half-valid bootstrap state. | Fail the verifier with retained startup logs and copied artifacts instead of hanging. | Archive raw HTTP/CLI output and treat malformed bootstrap/status payloads as proof failures. |
| Protected M044 operator/public-contract rails | Replay the existing rails in order and stop on the first drift; do not weaken filters to compensate for the new helper. | Preserve per-phase logs and copied artifact directories when a legacy rail stalls or times out. | Reject malformed JSON, bad pointer files, or zero-test runs instead of claiming the slice is green. |

## Load Profile

- **Shared resources**: `cluster-proof` build/test outputs, local ports, spawned proof-app processes, and `.tmp/m045-s01/...` artifact directories.
- **Per-operation cost**: one package build/test run plus the assembled local verifier replay.
- **10x breakpoint**: port conflicts, stale artifacts, and verifier replay churn fail before runtime throughput matters.

## Negative Tests

- **Malformed inputs**: old bootstrap env names, missing cookie with discovery hint, blank seed, invalid node name, and partial Fly identity.
- **Error paths**: runtime bootstrap returns `Err(String)`, entrypoint exits early, and protected operator/public-contract rails fail after the migration.
- **Boundary conditions**: standalone `cluster-proof`, explicit node-name cluster mode, and Fly identity cluster mode.

## Steps

1. Rewrite `cluster-proof/main.mpl` to consume the runtime bootstrap result and keep `config.mpl` only for the remaining continuity/durability/container concerns.
2. Trim `cluster-proof/config.mpl`, `cluster-proof/tests/config.test.mpl`, and `cluster-proof/docker-entrypoint.sh` so bootstrap validation lives in the runtime helper while packaged fail-fast behavior remains honest.
3. Extend `compiler/meshc/tests/e2e_m045_s01.rs` and protected public-contract coverage in `compiler/meshc/tests/e2e_m044_s05.rs` so the new helper is proven on both the tiny scaffold surface and the retained local proof app.
4. Add `scripts/verify-m045-s01.sh` as the slice stopping condition; it must replay the runtime/bootstrap/scaffold rails, `cluster-proof` build/tests, and the protected M044 operator/public-contract rails fail-closed.

## Must-Haves

- [ ] `cluster-proof` no longer owns cluster mode / identity bootstrap in Mesh code.
- [ ] Local and Fly fail-closed bootstrap behavior still works through the runtime helper.
- [ ] `scripts/verify-m045-s01.sh` is authoritative and fails closed on zero-test or stale-artifact drift.

## Verification

- `cargo run -q -p meshc -- build cluster-proof`
- `cargo run -q -p meshc -- test cluster-proof/tests`
- `bash scripts/verify-m045-s01.sh`

## Observability Impact

- Signals added/changed: one assembled verifier with explicit phase markers plus runtime-owned bootstrap errors visible in proof-app logs.
- How a future agent inspects this: retained `.tmp/m045-s01/...` phase logs, copied stdout/stderr, `meshc cluster ...` JSON output, and protected M044 replay logs.
- Failure state exposed: failing verifier phase, bootstrap error reason, and legacy-rail drift without secret leakage.

## Inputs

- `cluster-proof/main.mpl` — current proof-app startup orchestration.
- `cluster-proof/config.mpl` — mixed bootstrap and continuity-policy helpers to split cleanly.
- `cluster-proof/tests/config.test.mpl` — helper-level bootstrap contract coverage.
- `cluster-proof/docker-entrypoint.sh` — packaged startup preflight that must stay honest.
- `compiler/meshc/tests/e2e_m044_s05.rs` — protected public-contract rails for malformed env and Fly/local startup.
- `compiler/meshc/tests/e2e_m045_s01.rs` — new bootstrap API/scaffold target to extend.
- `scripts/verify-m044-s03.sh` — protected scaffold/operator replay rail.

## Expected Output

- `cluster-proof/main.mpl` — runtime-bootstrap-driven proof-app startup.
- `cluster-proof/config.mpl` — trimmed helper surface focused on continuity/durability/container concerns.
- `cluster-proof/tests/config.test.mpl` — updated helper-level coverage for the reduced config boundary.
- `cluster-proof/docker-entrypoint.sh` — packaged fail-fast behavior aligned with runtime-owned bootstrap.
- `compiler/meshc/tests/e2e_m044_s05.rs` — protected public-contract assertions updated for the new helper.
- `compiler/meshc/tests/e2e_m045_s01.rs` — proof-app bootstrap coverage alongside scaffold coverage.
- `scripts/verify-m045-s01.sh` — assembled fail-closed acceptance rail for S01.
