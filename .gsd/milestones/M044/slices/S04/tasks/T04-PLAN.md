---
estimated_steps: 4
estimated_files: 8
skills_used:
  - test
---

# T04: Replace the manual proof rail with the S04 verifier and auto-only docs

**Slice:** S04 — Bounded Automatic Promotion
**Milestone:** M044

## Description

Once the runtime behaves correctly, the proof app and docs need to stop teaching the old contract. This task removes the `/promote` route from `cluster-proof`, updates the package tests and public docs to the new auto-only story, and adds one fail-closed S04 verifier that archives the destructive proof bundle and refuses stale manual-surface wording.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `cluster-proof` runtime/package tests | Fail the package rail and retain the broken app/log snapshot. | Bound server waits and surface which readiness or status transition stalled. | Reject malformed JSON/status artifacts instead of passing on partial truth. |
| Assembled verifier in `scripts/verify-m044-s04.sh` | Stop at the first failing phase with retained logs, status, and test-count artifacts. | Mark the exact verifier phase as timed out and keep the copied bundle. | Fail closed on missing `running N test`, malformed JSON, or stale manual-surface strings. |
| Public docs/proof pages | Fail the docs/build checks rather than letting stale `/promote` wording ship. | N/A — local file/build path. | Reject stale command blocks, missing markers, or mixed manual/auto wording. |

## Load Profile

- **Shared resources**: same-image Docker containers, retained verifier artifact directories, docs build output, and proof-app package tests.
- **Per-operation cost**: one destructive two-node replay plus package build/test and docs verification.
- **10x breakpoint**: Docker/container startup and retained artifact churn dominate before app logic; the verifier must keep bounded copies and deterministic phase cleanup.

## Negative Tests

- **Malformed inputs**: stale `/promote` or `Continuity.promote` references in docs/verifiers, missing retained artifact files, and missing `running N test` lines.
- **Error paths**: `cluster-proof` still depends on manual promotion, the same-image proof still needs a retry to finish, or stale-primary rejoin resumes execution.
- **Boundary conditions**: promoted standby finishes automatically, ambiguous cases retain refusal artifacts, and operator docs remain read-only even after auto-promotion ships.

## Steps

1. Remove the `/promote` HTTP route and manual-promotion helpers from `cluster-proof`, updating package tests to the new auto-promotion and auto-resume truth.
2. Add `scripts/verify-m044-s04.sh` that replays S03, runs the named `m044_s04_` rails, enforces non-zero test execution, and validates retained same-image artifacts for auto-promotion, auto-resume, and stale-primary fencing.
3. Update `README.md`, `cluster-proof/README.md`, and the distributed proof/docs pages so the public story is bounded automatic promotion, not manual promotion or operator mutation.
4. Rebuild docs and rerun the assembled verifier so the shipped proof app, verifier, and docs all describe the same contract.

## Must-Haves

- [ ] `cluster-proof` no longer exposes `/promote` or depends on a manual authority-change helper to finish the destructive failover rail.
- [ ] `scripts/verify-m044-s04.sh` is the authoritative fail-closed local acceptance command with retained artifacts and non-zero test-count guards.
- [ ] Public docs and runbooks explicitly describe bounded automatic promotion, stale-primary fencing, ambiguous fail-closed behavior, and read-only operator inspection.
- [ ] The final proof bundle shows no manual promote call or same-key retry in the healthy S04 path.

## Verification

- `cargo run -q -p meshc -- build cluster-proof`
- `cargo run -q -p meshc -- test cluster-proof/tests`
- `bash scripts/verify-m044-s04.sh`
- `npm --prefix website run build`

## Observability Impact

- Signals added/changed: `.tmp/m044-s04/verify/` becomes the authoritative retained proof bundle with phase/status files, same-image JSON/log artifacts, and stale-manual-surface drift checks.
- How a future agent inspects this: start with `status.txt`, `current-phase.txt`, `phase-report.txt`, and the copied same-image artifact manifest before replaying the destructive failover rail.
- Failure state exposed: whether the break is in package behavior, verifier orchestration, retained artifact truth, or public docs drift.

## Inputs

- `cluster-proof/main.mpl` — current HTTP route registration that still wires `/promote`.
- `cluster-proof/work_continuity.mpl` — current manual-promotion helper and keyed work payload shaping.
- `cluster-proof/tests/work.test.mpl` — current package-level status/authority assertions.
- `scripts/verify-m044-s03.sh` — the current assembled M044 verifier pattern and prerequisite replay.
- `README.md` — current top-level public routing for clustered/failover claims.
- `cluster-proof/README.md` — current deep runbook that still teaches `/promote`.
- `website/docs/docs/distributed/index.md` — generic distributed guide that links to the proof surface.
- `website/docs/docs/distributed-proof/index.md` — current public proof page that still describes manual promotion.

## Expected Output

- `cluster-proof/main.mpl` — auto-only proof-app routes with no `/promote` endpoint.
- `cluster-proof/work_continuity.mpl` — no manual-promotion helper or logs in the healthy path.
- `cluster-proof/tests/work.test.mpl` — package tests aligned to auto-promotion/auto-resume truth.
- `scripts/verify-m044-s04.sh` — fail-closed assembled verifier with retained artifact checks.
- `README.md` — top-level public routing updated to the auto-only failover story.
- `cluster-proof/README.md` — deep runbook updated to bounded automatic promotion.
- `website/docs/docs/distributed/index.md` — generic distributed page routed cleanly to the new proof surface.
- `website/docs/docs/distributed-proof/index.md` — public proof page updated to the S04 contract.
