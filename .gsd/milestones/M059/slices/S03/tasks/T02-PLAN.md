---
estimated_steps: 3
estimated_files: 8
skills_used:
  - github-workflows
  - bash-scripting
  - test
---

# T02: Update the machine-checked `mesher/client` contract and prove dev/prod parity from the new path

**Slice:** S03 — Finalize move to `mesher/client` and remove Next.js runtime path
**Milestone:** M059

## Description

The rename is not done until every machine-checked caller launches or verifies the dashboard from `mesher/client` instead of preserving a stale `frontend-exp` contract. This task updates the product-root CI/verifier/docs surfaces and the `mesh-lang` root Playwright entrypoint so all automated callers agree on the new canonical path.

Keep the scope narrow while proving the real runtime: `mesher/landing` is still a legitimate Next.js app and should remain untouched, while the moved dashboard package must keep the same mock-data/client-state parity that S02 already established. The closeout proof is the real `build` / `start` / Playwright parity rail from `../hyperpush-mono/mesher/client/`, not a docs-only grep sweep.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `../hyperpush-mono/.github/workflows/ci.yml` product CI contract | Update cache/install/build paths atomically so CI cannot point at a half-migrated package name. | Treat long-running or hanging build steps as broken path/wiring, not flaky infra. | Reject stale labels, cache paths, or build commands that still name `frontend-exp`. |
| `../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh` maintainer verifier | Keep the verifier aligned with README/CI in the same task so it does not self-contradict. | Fail fast on verifier contract drift instead of shipping a script that certifies the wrong path. | Reject marker checks that mix `frontend-exp` and `client` expectations. |
| `playwright.config.ts` plus `../hyperpush-mono/mesher/client/playwright.config.ts` | Make the root cross-repo harness target the moved package and the same parity suite. | Bound dev/prod readiness so failures surface as the wrong server/path rather than an endless wait. | Reject miswired base paths or commands that accidentally keep testing the old directory. |

## Load Profile

- **Shared resources**: one package build, one dev server, one built-production server, the shared Playwright parity suite, and product-root verification scripts.
- **Per-operation cost**: full app build plus two browser-parity passes and a targeted stale-path sweep across the machine-checked caller surfaces.
- **10x breakpoint**: build/start readiness drift and stale verifier/CI path assumptions break first; the grep/doc updates themselves are cheap.

## Negative Tests

- **Malformed inputs**: stale `frontend-exp` strings in CI/verifier/docs/root Playwright config, wrong cache dependency paths, or old labels that imply the wrong canonical package.
- **Error paths**: `npm run start` boots from the wrong directory, prod parity fails on non-root direct entry, or the verifier still certifies the old path.
- **Boundary conditions**: dev and prod both run from `../hyperpush-mono/mesher/client/`, the landing app remains the only intentional Next.js runtime surface, and the moved dashboard keeps the existing mock-data-only contract.

## Steps

1. Update the machine-checked external surfaces that still hardcode `frontend-exp`: `../hyperpush-mono/.github/workflows/ci.yml`, `../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh`, `../hyperpush-mono/README.md`, `../hyperpush-mono/.github/dependabot.yml`, and `./playwright.config.ts`.
2. Keep the product/runtime boundary truthful while editing: the dashboard path becomes `mesher/client`, but `mesher/landing` remains the in-scope Next.js app and no backend/mock-data boundaries are widened.
3. Re-run `build`, `test:e2e:dev`, and `test:e2e:prod` from `../hyperpush-mono/mesher/client/`, then do targeted stale-path checks so the rename is mechanically proven instead of asserted.

## Must-Haves

- [ ] Every machine-checked external caller that launches or verifies the dashboard points at `../hyperpush-mono/mesher/client/`.
- [ ] Product CI, the product maintainer verifier, and `mesh-lang` root Playwright verification stay mutually consistent after the rename.
- [ ] The moved package still passes build plus isolated dev/prod parity from the new path.
- [ ] The slice keeps `mesher/landing` as the only legitimate Next.js runtime surface and does not add loaders, server functions, or backend calls to the client app.

## Verification

- `npm --prefix ../hyperpush-mono/mesher/client run build`
- `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev`
- `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:prod`
- `! rg -n "mesher/frontend-exp|frontend-exp" ../hyperpush-mono/.github/workflows/ci.yml ../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh ../hyperpush-mono/README.md ../hyperpush-mono/.github/dependabot.yml ./playwright.config.ts`
- `rg -n "mesher/client|client" ../hyperpush-mono/.github/workflows/ci.yml ../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh ../hyperpush-mono/README.md ../hyperpush-mono/.github/dependabot.yml ./playwright.config.ts`

## Observability Impact

- Signals added/changed: CI/cache/build failures, verifier marker drift, and Playwright console/request failures all now point at the `mesher/client` contract.
- How a future agent inspects this: inspect `../hyperpush-mono/.github/workflows/ci.yml`, `../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh`, `playwright.config.ts`, and the package-local Playwright config/test output.
- Failure state exposed: stale old-path references, miswired webServer commands, prod boot regressions, or dev/prod parity failures after the rename.

## Inputs

- `../hyperpush-mono/mesher/client/package.json` — moved package contract from T01 that all external callers must now target.
- `../hyperpush-mono/mesher/client/playwright.config.ts` — package-local parity harness that the root verifier must continue to drive truthfully.
- `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts` — real parity assertions that must stay green after the rename.
- `../hyperpush-mono/.github/workflows/ci.yml` — product CI path contract that still hardcodes `frontend-exp`.
- `../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh` — machine-checked maintainer verifier that must stop asserting the old package path.
- `../hyperpush-mono/README.md` — product root surface that names the canonical dashboard path.
- `../hyperpush-mono/.github/dependabot.yml` — product dependency-update scope that must follow the renamed package.
- `playwright.config.ts` — mesh-lang cross-repo parity config that still launches `frontend-exp`.

## Expected Output

- `../hyperpush-mono/.github/workflows/ci.yml` — CI/cache/build steps updated to the canonical `mesher/client` path.
- `../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh` — product verifier updated to require the new path and truthful commands.
- `../hyperpush-mono/README.md` — product root documentation aligned with the renamed client package.
- `../hyperpush-mono/.github/dependabot.yml` — npm update scope and labels renamed to `mesher/client`.
- `playwright.config.ts` — mesh-lang root cross-repo verification pointed at `../hyperpush-mono/mesher/client/`.
- `../hyperpush-mono/mesher/client/package.json` — if needed, package scripts or metadata adjusted only to keep the renamed runtime/test contract passing.
