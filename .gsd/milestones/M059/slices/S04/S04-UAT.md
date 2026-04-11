# S04: Equivalence proof and direct operational cleanup — UAT

**Milestone:** M059
**Written:** 2026-04-11T19:12:49.414Z

# S04 UAT — Equivalence proof and direct operational cleanup

## Preconditions

- Run all commands from `/Users/sn0w/Documents/dev/mesh-lang`.
- `../hyperpush-mono/mesher/client` dependencies are installed.
- Ports `3000` and `3001` are free before Playwright starts its dev/prod servers.
- Use the canonical sibling workspace layout described in `AGENTS.md`.

## Test Case 1 — Dev parity covers the remaining high-signal dashboard behaviors

1. Run `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev -- tests/e2e/dashboard-route-parity.spec.ts`.
   - **Expected:** Playwright starts only the dev project and reports `9 passed`.
2. Confirm the output includes the Solana Programs AI/sidebar parity test.
   - **Expected:** The listed/passed tests include `solana programs AI auto-collapses the sidebar and restores it on close`.
3. Confirm the output includes the Issues browser-history parity test.
   - **Expected:** The listed/passed tests include `browser back and forward preserve issues state after search, filters, and detail changes`.
4. Review the run for runtime-signal failures.
   - **Expected:** No console-error or failed-request assertion failures appear.

## Test Case 2 — Built production parity matches the dev proof surface

1. Run `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:prod -- tests/e2e/dashboard-route-parity.spec.ts`.
   - **Expected:** Playwright builds/starts the production app and reports `9 passed`.
2. Verify the same Solana Programs and Issues browser-history cases pass in production.
   - **Expected:** Both named tests pass under the `prod` project.
3. Verify direct-entry and unknown-path fallback coverage remains included.
   - **Expected:** The run also passes the direct-entry route checks and the unknown-path fallback-to-Issues check.

## Test Case 3 — Canonical package closeout rails stay green end to end

1. Run `npm --prefix ../hyperpush-mono/mesher/client run build`.
   - **Expected:** Vite produces `dist/client` and `dist/server` successfully with exit code 0.
2. Run `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev`.
   - **Expected:** The full dev parity suite reports `9 passed`.
3. Run `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:prod`.
   - **Expected:** The full prod parity suite reports `9 passed`.
4. Treat any console/request failure, route-key mismatch, or state-restoration mismatch as a blocker.
   - **Expected:** None occur.

## Test Case 4 — Maintainer-facing operational references point only at `mesher/client`

1. Run `! rg -n "mesher/frontend-exp|frontend-exp" ../hyperpush-mono/AGENTS.md ../hyperpush-mono/CONTRIBUTING.md ../hyperpush-mono/SUPPORT.md ../hyperpush-mono/.github/ISSUE_TEMPLATE/bug_report.yml ../hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml ../hyperpush-mono/.github/ISSUE_TEMPLATE/documentation.yml ./AGENTS.md`.
   - **Expected:** Exit code 0 with no matches.
2. Run `rg -n "mesher/client" ../hyperpush-mono/AGENTS.md ../hyperpush-mono/CONTRIBUTING.md ../hyperpush-mono/SUPPORT.md ../hyperpush-mono/.github/ISSUE_TEMPLATE/bug_report.yml ../hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml ../hyperpush-mono/.github/ISSUE_TEMPLATE/documentation.yml ./AGENTS.md`.
   - **Expected:** Positive matches appear in each intended maintainer-facing file.
3. Spot-check that `mesher/landing` is still described as the Next.js app where applicable.
   - **Expected:** The cleanup repoints dashboard guidance without erasing the intentional landing-app boundary.

## Test Case 5 — The mesh-lang root harness still resolves the moved dashboard package

1. Run `PLAYWRIGHT_PROJECT=dev npx --prefix ../hyperpush-mono/mesher/client playwright test --config ./playwright.config.ts --project=dev --list`.
   - **Expected:** The root harness lists the `mesher/client/tests/e2e/dashboard-route-parity.spec.ts` suite from `mesh-lang` without requiring a `cd` into the product repo.
2. Verify only the dev project is selected.
   - **Expected:** The output lists the 9 dev tests and does not boot/list the prod project.

## Edge Cases

- If a parity run fails immediately with port conflicts, free ports `3000`/`3001` and rerun the same command.
- If a runtime regression reappears only in browser proof, start with the targeted parity spec before changing broader docs/workflow files.
- If a stale-path grep fails, limit cleanup to the selected direct-operational files; do not rewrite historical milestone artifacts or unrelated landing copy as part of this slice.
