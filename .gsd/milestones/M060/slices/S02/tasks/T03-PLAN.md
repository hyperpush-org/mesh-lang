---
estimated_steps: 4
estimated_files: 4
skills_used:
  - playwright-best-practices
  - bash-scripting
---

# T03: Finish seeded dev/prod maintainer-loop proof and document the supported live seam

**Slice:** S02 — Core maintainer loop live
**Milestone:** M060

## Description

Close the slice with deterministic seeded proof in both runtimes and maintainer-facing documentation. Treat this as the final integration pass: make the seed helper replay-safe for action cases, finish failure-path assertions, and document the supported live action set plus the verification commands.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `bash mesher/scripts/seed-live-issue.sh` deterministic action fixture | Fail loudly before browser verification and leave the failing seed/readback step inspectable. | Stop the proof and preserve the shell/backend state for diagnosis. | Reject malformed seeded ids/statuses so browser tests do not assert against unknown state. |
| Dev and prod Playwright runtimes | Keep the failing runtime isolated and visible instead of weakening assertions to pass both. | Fail on the timed-out action/reload step rather than adding sleeps. | Reject missing request, toast, or source signals as runtime contract drift. |

## Load Profile

- **Shared resources**: seeded default issue state in Mesher, dev and built-prod browser runtimes, and the combined live read/action spec set.
- **Per-operation cost**: one deterministic seed/reset plus dev and prod live browser runs.
- **10x breakpoint**: repeated full-suite replays will stress deterministic seed/reset and stale cached browser assumptions before they stress the app itself.

## Negative Tests

- **Malformed inputs**: seed helper leaves the issue already resolved or archived, or the browser receives malformed status after mutation.
- **Error paths**: induced 500 on mutation or refresh shows no destructive toast, direct backend port requests appear, or one runtime diverges from the other.
- **Boundary conditions**: repeated seed plus resolve/unresolve/archive replay stays deterministic across both runtimes.

## Steps

1. Update `mesher/scripts/seed-live-issue.sh` so the seeded issue starts in a replay-safe open state and remains deterministic for resolve/unresolve/archive browser assertions.
2. Finish `mesher/client/tests/e2e/issues-live-actions.spec.ts` and extend `mesher/client/tests/e2e/issues-live-read.spec.ts` only if needed for regression coverage so the combined `issues live` grep proves action happy paths, failure toasts, summary truth, and same-origin routing in dev and prod.
3. Update `mesher/client/README.md` with the supported S02 action set, the still-mocked surfaces, and the exact seeded verification commands maintainers should run.
4. Re-run the full slice verification commands exactly as written and treat any read or action regression as a blocker for completion.

## Must-Haves

- [ ] The seed helper is replay-safe for repeated resolve/unresolve/archive verification.
- [ ] Dev and prod Playwright runs both prove same-origin issue actions, visible failure toasts, and truthful post-mutation list/summary state.
- [ ] `mesher/client/README.md` documents the supported S02 live action set and verification steps without claiming unsupported actions are ready.
- [ ] The slice closes with the exact verification commands from the slice plan, not a weaker one-off smoke check.

## Verification

- `bash mesher/scripts/seed-live-issue.sh`
- `npm --prefix mesher/client run test:e2e:dev -- --grep "issues live"`
- `npm --prefix mesher/client run test:e2e:prod -- --grep "issues live"`

## Observability Impact

- Signals added/changed: deterministic seed/readback checks, explicit action-failure toast assertions, and combined `issues live` dev/prod runtime coverage.
- How a future agent inspects this: run the seed helper, replay the `issues live` grep in dev and prod, and inspect captured same-origin request lists plus toast text in the browser report.
- Failure state exposed: non-deterministic seed state, runtime-specific action regressions, or summary/read seams that drift after mutation.

## Inputs

- `mesher/scripts/seed-live-issue.sh` — deterministic seeded issue helper that must become replay-safe for action assertions.
- `mesher/client/tests/e2e/issues-live-actions.spec.ts` — action proof rail from T01/T02.
- `mesher/client/tests/e2e/issues-live-read.spec.ts` — existing read seam proof that must stay green when the combined `issues live` grep runs.
- `mesher/client/README.md` — maintainer guidance that must explain the supported live seam truthfully.
- `mesher/client/playwright.config.ts` — current dev/prod harness for the live Mesher client package.

## Expected Output

- `mesher/scripts/seed-live-issue.sh` — replay-safe seed/reset behavior for action verification.
- `mesher/client/tests/e2e/issues-live-actions.spec.ts` — completed dev/prod action happy-path and failure-path coverage.
- `mesher/client/tests/e2e/issues-live-read.spec.ts` — retained or extended read seam coverage as needed for the combined `issues live` suite.
- `mesher/client/README.md` — supported S02 live action set and exact verification commands.
