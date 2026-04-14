---
estimated_steps: 4
estimated_files: 4
skills_used:
  - bash-scripting
  - playwright-best-practices
---

# T03: Harden live-issue seeding and prove the closeout rail end to end

**Slice:** S04 — Canonical maintainer handoff
**Milestone:** M061

## Description

Retire the last reproduced repeatability hazard before calling the handoff done. `../hyperpush-mono/mesher/scripts/seed-live-issue.sh` currently reuses any Mesher already listening on its chosen port, which can point the verifier at the wrong runtime or hang the readback phase. Mirror the isolated-port behavior already used by `seed-live-admin-ops.sh`, keep backend reuse opt-in only, then rerun the new root wrapper and fix only the setup/runtime drift that prevents the closeout rail from passing.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `../hyperpush-mono/mesher/scripts/seed-live-issue.sh` | Default to isolated startup and fail fast if a clean backend cannot be booted or reached. | Surface the exact readiness log or last-response artifact instead of silently looping on the wrong port. | Reject malformed settings responses or missing seeded issue state as verifier failure, not as evidence of success. |
| `../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh` | Keep the failing phase and per-phase log path visible so reruns fix the specific broken seam instead of weakening the rail. | Preserve explicit phase timeouts for structure, seeding, dev, and prod proof runs. | Treat a missing retained proof-bundle pointer or missing Playwright artifacts as a closeout failure. |
| The dev/prod Playwright proof suites cited by the route inventory | Rerun only the delegated failing phase with retained logs; do not broaden waits or weaken assertions without a reproduced root cause. | Keep the route-inventory grep targeted and workers/settings unchanged unless the failure proves a real isolation mismatch. | Treat same-origin read/write mismatches as real runtime regressions rather than compensating in the seed script. |

## Load Profile

- **Shared resources**: one temporary Mesher build/runtime, seeded same-origin APIs, delegated dev/prod Playwright runs, and retained artifact directories.
- **Per-operation cost**: one isolated seed-live-issue backend boot plus the existing route-inventory structure/dev/prod proof replay.
- **10x breakpoint**: stray local processes and reused ports fail first; isolated-by-default port selection is the cheap protection against cross-run contamination.

## Negative Tests

- **Malformed inputs**: a pre-existing backend on the default issue-seed port must not be treated as authoritative unless reuse is explicitly requested.
- **Error paths**: wrong-runtime settings responses, missing seeded issue rows, failing dev/prod suite filters, or missing proof-bundle directories must all remain visible in wrapper artifacts.
- **Boundary conditions**: after the wrapper passes, `latest-proof-bundle.txt` must resolve to a directory and the delegated phase report must include all expected passed markers.

## Steps

1. Add isolated-by-default backend-endpoint selection to `../hyperpush-mono/mesher/scripts/seed-live-issue.sh`, borrowing the safe behavior from `../hyperpush-mono/mesher/scripts/seed-live-admin-ops.sh` while preserving an explicit opt-in reuse path if maintainers really want it.
2. Adjust the package verifier only as needed so the hardened issue seed path, retained proof bundle, and root wrapper all agree on artifact locations and failure messages.
3. Run `bash ../hyperpush-mono/scripts/verify-m061-s04.sh`; if it fails, inspect the named failing phase/log or proof-bundle artifact and fix only the reproduced setup/runtime problem needed to make the closeout rail pass.
4. Leave the final passing proof surfaces inspectable through the root wrapper output, delegated phase report, and `latest-proof-bundle.txt` pointer.

## Must-Haves

- [ ] `seed-live-issue.sh` no longer silently reuses an arbitrary already-running backend by default.
- [ ] The delegated route-inventory verifier and the new root wrapper agree on artifact paths and proof-bundle expectations.
- [ ] `bash ../hyperpush-mono/scripts/verify-m061-s04.sh` passes and leaves an inspectable proof-bundle pointer for future reruns.

## Verification

- `bash ../hyperpush-mono/scripts/verify-m061-s04.sh`

## Observability Impact

- Signals added/changed: isolated-seed startup messages, retained readiness or last-response artifacts, delegated phase logs, and root-wrapper proof-bundle pointers.
- How a future agent inspects this: rerun `bash ../hyperpush-mono/scripts/verify-m061-s04.sh`, then open the named failing log or the directory referenced by `latest-proof-bundle.txt`.
- Failure state exposed: stale-backend reuse, readiness failure, suite-filter drift, or missing artifact bundles all become explicit wrapper or verifier failures.

## Inputs

- `../hyperpush-mono/mesher/scripts/seed-live-issue.sh` — current issue seed path that still trusts any already-running backend on its chosen port.
- `../hyperpush-mono/mesher/scripts/seed-live-admin-ops.sh` — isolated-port reference implementation.
- `../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh` — delegated verifier whose artifact paths must align with the hardened seed script.
- `../hyperpush-mono/scripts/verify-m061-s04.sh` — root wrapper added in T02.
- `../hyperpush-mono/mesher/client/tests/e2e/issues-live-read.spec.ts` — live issue read proof cited by the route inventory.
- `../hyperpush-mono/mesher/client/tests/e2e/issues-live-actions.spec.ts` — live issue action proof cited by the route inventory.
- `../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts` — admin/ops proof suite replayed by the delegated verifier.
- `../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts` — assembled walkthrough proof replayed by the delegated verifier.

## Expected Output

- `../hyperpush-mono/mesher/scripts/seed-live-issue.sh` — isolated-by-default issue seeding.
- `../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh` — delegated verifier aligned with the hardened seed path and retained proof bundle.
- `../hyperpush-mono/scripts/verify-m061-s04.sh` — passing closeout wrapper that points to inspectable evidence.
