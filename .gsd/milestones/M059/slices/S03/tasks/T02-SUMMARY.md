---
id: T02
parent: S03
milestone: M059
key_files:
  - ../hyperpush-mono/.github/workflows/ci.yml
  - ../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh
  - ../hyperpush-mono/README.md
  - ../hyperpush-mono/.github/dependabot.yml
  - playwright.config.ts
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Mirror the package-local `PLAYWRIGHT_PROJECT`/`npm_config_project` selection logic in `mesh-lang/playwright.config.ts` so cross-repo callers boot only the requested dev or prod server from `mesher/client`.
  - Keep repo-root CI, verifier markers, docs, Dependabot scope, and the cross-repo Playwright config synchronized in the same rename task so the machine-checked contract cannot drift.
duration: 
verification_result: passed
completed_at: 2026-04-11T18:11:58.510Z
blocker_discovered: false
---

# T02: Aligned CI, verifier/docs, Dependabot, and the root Playwright harness with `mesher/client`, then re-proved dev/prod dashboard parity from the moved package.

**Aligned CI, verifier/docs, Dependabot, and the root Playwright harness with `mesher/client`, then re-proved dev/prod dashboard parity from the moved package.**

## What Happened

Updated every task-owned machine-checked external surface that still referenced `frontend-exp`: product CI now caches, installs, and builds `mesher/client`; the maintainer verifier now requires `mesher/client` markers in the README and CI workflow; the product root README now names `mesher/client` as the canonical TanStack dashboard while keeping `mesher/landing` as the intentional Next.js runtime surface; and Dependabot now scopes npm updates to `/mesher/client`. Rewrote `mesh-lang/playwright.config.ts` to target `../hyperpush-mono/mesher/client`, preserve the existing dev/prod route-parity suite, and honor `PLAYWRIGHT_PROJECT` / `npm_config_project` so root-level callers boot only the requested environment. Also recorded the cross-repo Playwright validation pattern in `.gsd/KNOWLEDGE.md` and saved decision D504 for the mirrored root-harness selection logic.

## Verification

`bash -n ../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh` passed. The stale-path grep across the workflow, verifier, README, Dependabot, and root Playwright config returned zero matches, and the positive `mesher/client|client` grep returned the expected updated contract hits. The real runtime proof from `../hyperpush-mono/mesher/client/` passed via `npm --prefix ../hyperpush-mono/mesher/client run build`, `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev`, and `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:prod`, with both Playwright projects passing all 7 route/UI parity tests on mock data. The root cross-repo Playwright harness also loaded successfully from `mesh-lang` via `PLAYWRIGHT_PROJECT=dev npx --prefix ../hyperpush-mono/mesher/client playwright test --config ./playwright.config.ts --project=dev --list`.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash -n ../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh` | 0 | ✅ pass | 82ms |
| 2 | `! rg -n "mesher/frontend-exp|frontend-exp" ../hyperpush-mono/.github/workflows/ci.yml ../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh ../hyperpush-mono/README.md ../hyperpush-mono/.github/dependabot.yml ./playwright.config.ts` | 0 | ✅ pass | 130ms |
| 3 | `rg -n "mesher/client|client" ../hyperpush-mono/.github/workflows/ci.yml ../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh ../hyperpush-mono/README.md ../hyperpush-mono/.github/dependabot.yml ./playwright.config.ts` | 0 | ✅ pass | 87ms |
| 4 | `npm --prefix ../hyperpush-mono/mesher/client run build` | 0 | ✅ pass | 6567ms |
| 5 | `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev` | 0 | ✅ pass | 33549ms |
| 6 | `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:prod` | 0 | ✅ pass | 28333ms |
| 7 | `PLAYWRIGHT_PROJECT=dev npx --prefix ../hyperpush-mono/mesher/client playwright test --config ./playwright.config.ts --project=dev --list` | 0 | ✅ pass | 2452ms |

## Deviations

Added two lightweight checks beyond the written slice rail: `bash -n` on the edited verifier script and a root Playwright `--list` loadability check from `mesh-lang`.

## Known Issues

The required positive grep `rg -n "mesher/client|client" ...` is intentionally noisy because it also matches unrelated strings like `PostgreSQL client` in CI; the authoritative rename proof is the zero-match stale-path grep plus the explicit `mesher/client` hits.

## Files Created/Modified

- `../hyperpush-mono/.github/workflows/ci.yml`
- `../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh`
- `../hyperpush-mono/README.md`
- `../hyperpush-mono/.github/dependabot.yml`
- `playwright.config.ts`
- `.gsd/KNOWLEDGE.md`
