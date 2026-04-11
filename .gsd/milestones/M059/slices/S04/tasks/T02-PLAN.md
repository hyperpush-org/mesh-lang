---
estimated_steps: 4
estimated_files: 7
skills_used:
  - bash-scripting
  - test
---

# T02: Rewrite the remaining direct operational guidance to `mesher/client` and close the slice

**Slice:** S04 — Equivalence proof and direct operational cleanup
**Milestone:** M059

## Description

Remove the last direct maintainer-facing `frontend-exp` guidance across the product repo and mesh-lang workspace instructions without reopening historical documents or product mock content. Update the selected AGENTS/CONTRIBUTING/SUPPORT/issue-template surfaces to name `mesher/client` as the dashboard package while preserving `mesher/landing` as the intentional Next.js app and leaving historical planning documents plus mock release copy untouched.

After the guidance edits, run the full closeout proof from `mesh-lang`: canonical package build, full dev/prod parity suite, the scoped stale-path/positive-path greps over only the files this task touched, and the root-harness `--list` check so both maintainers and automated callers have a truthful package-path contract.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `../hyperpush-mono/AGENTS.md`, `../hyperpush-mono/CONTRIBUTING.md`, `../hyperpush-mono/SUPPORT.md`, and `./AGENTS.md` maintainer guidance | Update the selected direct-operational files together so maintainers never see mixed old/new dashboard paths. | N/A for static text edits; move directly to targeted validation. | Reject wording that reintroduces `frontend-exp`, confuses `mesher/client` with `mesher/landing`, or widens the cleanup into historical notes. |
| `../hyperpush-mono/.github/ISSUE_TEMPLATE/*.yml` GitHub issue intake | Keep labels, placeholders, and descriptions aligned with the canonical package path in the same task. | N/A for static text edits; validate with targeted grep. | Reject templates that still point bug reports or docs requests at `mesher/frontend-exp/`. |
| `../hyperpush-mono/mesher/client/package.json` and `./playwright.config.ts` closeout rails | Re-run the canonical build/parity/list commands from `mesh-lang` so the guidance rewrite is backed by a live contract check. | Treat hangs as broken runtime or harness expectations, not documentation-only drift. | Reject any result where the root harness or package contract still targets the old directory. |

## Load Profile

- **Shared resources**: one canonical package build, one dev server, one built-production server, the shared Playwright parity suite, and the selected maintainer-facing docs/templates.
- **Per-operation cost**: seven text-surface edits, two scoped grep passes, a root-harness list check, and the full build plus dev/prod parity suite.
- **10x breakpoint**: build/parity/runtime readiness drift breaks first; the text edits and greps are cheap.

## Negative Tests

- **Malformed inputs**: stale `frontend-exp` strings, incorrect path placeholders, or wording that incorrectly treats `mesher/landing` as the migrated dashboard package.
- **Error paths**: canonical build or parity fails after the guidance cleanup, or the root harness no longer lists against `mesher/client`.
- **Boundary conditions**: the selected direct-operational surfaces all move to `mesher/client` while historical planning docs and mock-data copy remain untouched.

## Steps

1. Update the selected direct-operational guidance surfaces in `../hyperpush-mono/` and `./AGENTS.md` so maintainers and issue reporters see `mesher/client` as the dashboard package and `mesher/landing` as the intentional Next.js app.
2. Keep the cleanup scoped to the exact files named in the slice plan; do not edit historical planning docs or product mock content just to chase string matches.
3. Run the canonical package `build`, full dev/prod parity suite, and the root-harness `--list` command from `mesh-lang`.
4. Run scoped stale-path and positive-path greps only over the files edited in this task so the cleanup is mechanically proven without false failures from legitimate history elsewhere.

## Must-Haves

- [ ] All selected direct-operational guidance files stop naming `frontend-exp` as the active dashboard package and instead point at `mesher/client`.
- [ ] The cleanup preserves `mesher/landing` as the legitimate Next.js app and leaves historical planning docs plus mock release copy untouched.
- [ ] The canonical `mesher/client` build, full dev/prod parity suite, scoped grep checks, and root-harness list check all pass from `mesh-lang`.

## Verification

- `npm --prefix ../hyperpush-mono/mesher/client run build`
- `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev`
- `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:prod`
- `! rg -n "mesher/frontend-exp|frontend-exp" ../hyperpush-mono/AGENTS.md ../hyperpush-mono/CONTRIBUTING.md ../hyperpush-mono/SUPPORT.md ../hyperpush-mono/.github/ISSUE_TEMPLATE/bug_report.yml ../hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml ../hyperpush-mono/.github/ISSUE_TEMPLATE/documentation.yml ./AGENTS.md`
- `rg -n "mesher/client" ../hyperpush-mono/AGENTS.md ../hyperpush-mono/CONTRIBUTING.md ../hyperpush-mono/SUPPORT.md ../hyperpush-mono/.github/ISSUE_TEMPLATE/bug_report.yml ../hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml ../hyperpush-mono/.github/ISSUE_TEMPLATE/documentation.yml ./AGENTS.md`
- `PLAYWRIGHT_PROJECT=dev npx --prefix ../hyperpush-mono/mesher/client playwright test --config ./playwright.config.ts --project=dev --list`

## Observability Impact

- Signals added/changed: scoped stale-path and positive-path greps become the direct maintainer-guidance drift signal, backed by full build/dev/prod parity and the root-harness list check.
- How a future agent inspects this: inspect the selected docs/template files, then rerun the six verification commands from `mesh-lang`.
- Failure state exposed: stale old-path references, incorrect landing/client ownership wording, broken canonical runtime rails, or a miswired root harness.

## Inputs

- `../hyperpush-mono/AGENTS.md` — product workspace ownership guidance that still mentions `mesher/frontend-exp/`.
- `../hyperpush-mono/CONTRIBUTING.md` — maintainer setup and verification guidance with stale dashboard package references.
- `../hyperpush-mono/SUPPORT.md` — support triage wording that still points users at `frontend-exp`.
- `../hyperpush-mono/.github/ISSUE_TEMPLATE/bug_report.yml` — bug intake labels/placeholders that still mention `frontend-exp`.
- `../hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml` — feature intake wording that still mentions `frontend-exp`.
- `../hyperpush-mono/.github/ISSUE_TEMPLATE/documentation.yml` — docs intake placeholder still pointing at the old package path.
- `./AGENTS.md` — mesh-lang workspace guidance that still lists `mesher/frontend-exp/` under the product repo.
- `./playwright.config.ts` — root-harness config used for the final `--list` verification from `mesh-lang`.
- `../hyperpush-mono/mesher/client/package.json` — canonical package command contract used for build and parity verification.
- `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts` — canonical browser proof rail whose full suite still must stay green after the guidance cleanup.

## Expected Output

- `../hyperpush-mono/AGENTS.md` — product workspace rules updated to point dashboard ownership at `mesher/client`.
- `../hyperpush-mono/CONTRIBUTING.md` — maintainer setup and verification steps updated to the canonical dashboard package path.
- `../hyperpush-mono/SUPPORT.md` — support routing guidance updated to the canonical dashboard package path.
- `../hyperpush-mono/.github/ISSUE_TEMPLATE/bug_report.yml` — bug intake wording and placeholders updated to `mesher/client`.
- `../hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml` — feature intake wording updated to `mesher/client`.
- `../hyperpush-mono/.github/ISSUE_TEMPLATE/documentation.yml` — documentation intake placeholder updated to `mesher/client`.
- `./AGENTS.md` — mesh-lang workspace guidance updated so future agents see the correct product surface path.
