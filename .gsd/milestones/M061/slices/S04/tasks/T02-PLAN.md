---
estimated_steps: 4
estimated_files: 4
skills_used:
  - bash-scripting
  - test
---

# T02: Add the product-root closeout wrapper and lock the closeout contract markers

**Slice:** S04 — Canonical maintainer handoff
**Milestone:** M061

## Description

Turn the handoff into a fail-closed contract before touching the runtime seeding hazard. Model the new root wrapper on `../hyperpush-mono/scripts/verify-m051-s01.sh`, but keep the delegated client verifier route-inventory-focused. Extend the client verifier so it emits a `latest-proof-bundle.txt` pointer to retained logs and proof inputs, then extend the node:test contract so README/root README/wrapper/CI drift fails immediately with actionable messages.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh` | Stop the wrapper contract at the package-verifier layer and do not let the root wrapper report success without delegated artifacts. | Keep explicit phase timeouts and surface the failing per-phase log path in the wrapper artifact directory. | Treat missing or empty `latest-proof-bundle.txt`, `phase-report.txt`, `status.txt`, or `current-phase.txt` as fail-closed drift. |
| `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` | Reject marker drift instead of relying on README prose or shell output. | N/A — local structural tests should finish quickly; if they stall, fail the task rather than weakening coverage. | Make missing headings, stale commands, or wrapper-marker mismatches name the exact source file and marker. |
| `../hyperpush-mono/.github/workflows/ci.yml` | Keep CI acknowledgement minimal and structural if full runtime replay would be too expensive. | N/A — file edit only. | Do not claim CI runs the full local wrapper if the workflow only executes the structural contract. |

## Load Profile

- **Shared resources**: local script/test/doc files plus the product CI workflow.
- **Per-operation cost**: one root wrapper shell script, one package-verifier retained bundle path, one structural node:test rail, and one lightweight CI acknowledgement step.
- **10x breakpoint**: if more proof surfaces are added later, keep the root wrapper delegating and keep the node:test contract marker-based instead of hardcoding every future log filename twice.

## Negative Tests

- **Malformed inputs**: empty or missing `latest-proof-bundle.txt`, stale root-wrapper command strings, and missing handoff headings must all fail the structural contract.
- **Error paths**: the root wrapper must fail when delegated status/current-phase/phase-report artifacts are absent or when the delegated proof-bundle pointer does not resolve to a directory.
- **Boundary conditions**: CI may acknowledge only the structural contract, but README/root README/wrapper markers must still all agree on the same root-level closeout command.

## Steps

1. Add `../hyperpush-mono/scripts/verify-m061-s04.sh` as the root closeout wrapper that delegates to `../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh` and then asserts delegated status/current-phase/phase-report/latest-proof-bundle outputs.
2. Extend `../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh` to retain a proof-bundle directory and write `latest-proof-bundle.txt` without changing its route-inventory ownership boundaries.
3. Extend `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` to lock the new handoff headings, root/client README markers, root-wrapper markers, CI markers, and retained proof-bundle contract.
4. Update `../hyperpush-mono/.github/workflows/ci.yml` so the client job acknowledges the structural route-inventory contract instead of only building the package.

## Must-Haves

- [ ] A product-root `verify-m061-s04.sh` wrapper exists and fails closed on delegated artifact drift.
- [ ] The package verifier writes a retained proof-bundle pointer that the root wrapper and node:test contract can validate.
- [ ] The structural node:test rail locks the canonical handoff headings plus README/root README/wrapper/CI markers.
- [ ] Product CI acknowledges the closeout rail truthfully, without claiming to run a heavier local-only replay it does not actually execute.

## Verification

- `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`

## Observability Impact

- Signals added/changed: root-wrapper `status.txt`, `current-phase.txt`, `phase-report.txt`, `latest-proof-bundle.txt`, plus delegated retained proof-bundle metadata.
- How a future agent inspects this: run `bash ../hyperpush-mono/scripts/verify-m061-s04.sh` and inspect `../hyperpush-mono/.tmp/m061-s04/verify/` plus the delegated bundle path.
- Failure state exposed: missing wrapper markers, bundle-pointer drift, or CI/README contract drift becomes an explicit structural-test or wrapper-phase failure.

## Inputs

- `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` — canonical handoff headings added in T01.
- `../hyperpush-mono/mesher/client/README.md` — package-level workflow markers that must match the structural contract.
- `../hyperpush-mono/README.md` — root-level workflow markers that must match the structural contract.
- `../hyperpush-mono/scripts/verify-m051-s01.sh` — root-wrapper reference pattern for delegated fail-closed verification.
- `../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh` — retained proof-bundle reference pattern.
- `../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh` — delegated package verifier that needs a proof-bundle pointer.
- `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` — structural contract rail that must lock the new markers.
- `../hyperpush-mono/.github/workflows/ci.yml` — product CI surface that should acknowledge the structural contract.

## Expected Output

- `../hyperpush-mono/scripts/verify-m061-s04.sh` — product-root closeout wrapper.
- `../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh` — package verifier with retained proof-bundle pointer.
- `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` — fail-closed contract for handoff, wrapper, and CI markers.
- `../hyperpush-mono/.github/workflows/ci.yml` — client job acknowledges the structural route-inventory contract.
