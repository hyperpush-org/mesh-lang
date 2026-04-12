---
estimated_steps: 5
estimated_files: 6
skills_used:
  - bash-scripting
  - playwright-best-practices
---

# T03: Wrap the inventory proof in one retained maintainer verifier

**Slice:** S01 — Evidence-backed route inventory
**Milestone:** M061

## Description

Close the slice by making the inventory easy to trust and rerun. Add one retained verifier that runs the structural contract plus the existing seeded dev/prod Playwright rails, then expose that command from the client maintainer workflow so later slices inherit one obvious proof entrypoint.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` | Stop immediately and report the structural drift phase before running runtime proof. | Fail the verifier with a named phase and retained log path. | Treat missing or malformed parser output as a hard failure; do not continue to Playwright. |
| `../hyperpush-mono/mesher/scripts/seed-live-issue.sh` and `../hyperpush-mono/mesher/scripts/seed-live-admin-ops.sh` | Fail closed and keep the failing seed phase log so runtime proof is not trusted on stale fixtures. | Abort the verifier, mark the seed phase timed out, and preserve the log. | Treat unexpected seed output as a hard failure when the command exits non-zero. |
| `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev -- --grep "dashboard route parity|issues live|admin and ops live|seeded walkthrough"` and the matching `test:e2e:prod` command | Fail the named phase and preserve the Playwright log/artifact path. | Fail the named phase and preserve the timed-out command log. | Reject runs that match zero tests or skip the intended suites so the verifier cannot pass vacuously. |

## Load Profile

- **Shared resources**: local dev/prod ports, the seeded Mesher backend, Playwright workers, and retained logs under `../hyperpush-mono/mesher/.tmp/m061-s01/verify-client-route-inventory/`.
- **Per-operation cost**: one structural `node:test` run, two seed helpers, and one targeted Playwright run in each of dev and prod.
- **10x breakpoint**: Playwright/runtime startup time and retained artifact size will break before parsing cost; the wrapper should keep grep filters narrow and phase logs bounded.

## Negative Tests

- **Malformed inputs**: missing `ROUTE-INVENTORY.md`, stale grep filters that match zero tests, or absent retained artifact directories.
- **Error paths**: seed helper failure, dev proof failure, prod proof failure, or a structural test failure before runtime execution.
- **Boundary conditions**: the wrapper should succeed only when both dev and prod rails run at least one named test and the structural test passes first.

## Steps

1. Add `../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh`, modeled after the retained-phase style already used by `verify-maintainer-surface.sh`.
2. Make the verifier run the structural `node:test`, both seed helpers, and the targeted dev/prod Playwright greps while retaining phase/status logs under `../hyperpush-mono/mesher/.tmp/m061-s01/verify-client-route-inventory/`.
3. Fail closed when required files are missing, when a grep matches zero tests, or when any phase times out.
4. Add a `verify:route-inventory` maintainer script entry in `../hyperpush-mono/mesher/client/package.json`.
5. Update `../hyperpush-mono/mesher/client/README.md` so maintainers know the README is not the canonical inventory and can rerun the dedicated verifier from the documented workflow.

## Must-Haves

- [ ] One command reruns the structural contract and the seeded dev/prod proof rails with retained phase logs.
- [ ] The verifier names the failing phase and artifact path instead of failing ambiguously.
- [ ] The client maintainer workflow points to `ROUTE-INVENTORY.md` as the canonical map and exposes the verifier command directly.

## Verification

- `bash ../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh`

## Observability Impact

- Signals added/changed: retained phase report, current phase, status file, and per-phase logs under `../hyperpush-mono/mesher/.tmp/m061-s01/verify-client-route-inventory/`.
- How a future agent inspects this: rerun `bash ../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh` and inspect the named log path or phase report.
- Failure state exposed: structural drift, seed failures, zero-match Playwright greps, and dev/prod proof regressions.

## Inputs

- `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` — structural contract from T02.
- `../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh` — retained-phase verifier pattern to copy.
- `../hyperpush-mono/mesher/scripts/seed-live-issue.sh` — seeded Issues proof setup.
- `../hyperpush-mono/mesher/scripts/seed-live-admin-ops.sh` — seeded Alerts/Settings proof setup.
- `../hyperpush-mono/mesher/client/package.json` — maintainer script entrypoint.
- `../hyperpush-mono/mesher/client/README.md` — maintainer workflow documentation.
- `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` — canonical inventory surface the verifier proves.

## Expected Output

- `../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh` — retained verifier for structural plus runtime proof.
- `../hyperpush-mono/mesher/client/package.json` — documented `verify:route-inventory` script entry.
- `../hyperpush-mono/mesher/client/README.md` — maintainer workflow updated to point at the canonical inventory and verifier.
