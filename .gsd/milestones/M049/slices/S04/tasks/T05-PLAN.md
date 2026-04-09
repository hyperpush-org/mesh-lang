---
estimated_steps: 4
estimated_files: 8
skills_used:
  - test
---

# T05: Sweep older direct bash verifiers onto a shared clustered fixture helper

**Slice:** S04 — Retire top-level proof-app onboarding surfaces
**Milestone:** M049

## Description

Split the broad bash path churn into one bounded task: add a shared shell helper for clustered fixture roots and retarget the older direct verifier family that still shells out to `tiny-cluster` / `cluster-proof` repo-root paths.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Older direct bash verifiers (`m039`–`m045`) | Fail before any root-directory deletion and name the remaining stale script. | Keep the phase log and command label so the stale consumer is obvious. | Treat stale path constants or regex checks as contract drift. |
| Shared shell helper/fixture constants | Fail closed if a script cannot resolve the moved fixture location. | N/A — local path helper only. | Reject missing fixture roots rather than auto-falling back to repo-root directories. |
| Representative older retained rails | Stop the task if migrated scripts no longer run on the moved fixtures. | Preserve `.tmp/...` verifier logs for diagnosis. | Reject malformed output or missing `running N test` markers where those rails already require them. |

## Load Profile

- **Shared resources**: the older direct bash verifier family, shared `.tmp` verifier trees, and the moved internal fixtures.
- **Per-operation cost**: grep/build/test phases with retained logs rather than live services.
- **10x breakpoint**: execution time and review churn dominate; the task should centralize fixture-path constants instead of open-coding another round of replacements.

## Negative Tests

- **Malformed inputs**: stale `tiny-cluster/` or `cluster-proof/` path literals in older direct scripts, missing helper sourcing, or helper constants that still point at repo-root directories.
- **Error paths**: older direct rails no longer finding retained bundles or continuing to shell out to deleted root-package paths.
- **Boundary conditions**: internal scripts may still consume the relocated fixtures, but they must do it through the shared helper instead of duplicated literals.

## Steps

1. Add `scripts/lib/clustered_fixture_paths.sh` with shared constants/helpers for `scripts/fixtures/clustered/tiny-cluster` and `scripts/fixtures/clustered/cluster-proof`.
2. Retarget the older direct verifier family (`scripts/verify-m039-s01.sh`, `scripts/verify-m039-s02.sh`, `scripts/verify-m039-s03.sh`, `scripts/verify-m040-s01.sh`, `scripts/verify-m042-s01.sh`, `scripts/verify-m043-s01.sh`, `scripts/verify-m045-s02.sh`, plus remaining same-pattern direct consumers found by the task's initial sweep) to source or use that helper.
3. Keep their retained phase/artifact behavior intact while replacing repo-root build/test commands with fixture-backed commands.
4. Prove the helper-backed older rails on representative direct consumers before moving to wrapper/closeout deletion work.

## Must-Haves

- [ ] Older direct bash verifiers stop open-coding repo-root `tiny-cluster` / `cluster-proof` paths.
- [ ] One shared helper owns the clustered fixture roots for bash consumers.
- [ ] Representative older retained rails still pass against the moved fixtures.

## Verification

- `bash scripts/verify-m039-s01.sh`
- `bash scripts/verify-m045-s02.sh`

## Observability Impact

- Signals added/changed: older direct verifier phase logs should now name the shared helper-backed fixture paths.
- How a future agent inspects this: rerun the representative scripts above and inspect their `.tmp/.../verify` logs if a stale consumer remains.
- Failure state exposed: missing helper sourcing or stale root literals fail as named script-phase errors instead of as ambiguous missing-file crashes.

## Inputs

- `scripts/verify-m039-s01.sh` — representative older direct verifier that still builds `cluster-proof` from the repo root.
- `scripts/verify-m039-s02.sh` — older direct verifier that still shells out to the root proof fixtures.
- `scripts/verify-m039-s03.sh` — older direct verifier that still shells out to the root proof fixtures.
- `scripts/verify-m040-s01.sh` — older direct verifier that still shells out to the root proof fixtures.
- `scripts/verify-m042-s01.sh` — older direct verifier that still shells out to the root proof fixtures.
- `scripts/verify-m043-s01.sh` — older direct verifier that still shells out to the root proof fixtures.
- `scripts/verify-m045-s02.sh` — retained scaffold/runtime verifier that still carries root-package expectations.

## Expected Output

- `scripts/lib/clustered_fixture_paths.sh` — shared shell helper exposing stable clustered fixture roots.
- `scripts/verify-m039-s01.sh` — representative older direct verifier retargeted to the moved fixtures.
- `scripts/verify-m039-s02.sh` — older direct verifier retargeted to the moved fixtures.
- `scripts/verify-m039-s03.sh` — older direct verifier retargeted to the moved fixtures.
- `scripts/verify-m040-s01.sh` — older direct verifier retargeted to the moved fixtures.
- `scripts/verify-m042-s01.sh` — older direct verifier retargeted to the moved fixtures.
- `scripts/verify-m043-s01.sh` — older direct verifier retargeted to the moved fixtures.
- `scripts/verify-m045-s02.sh` — retained runtime verifier aligned with the moved fixtures.
