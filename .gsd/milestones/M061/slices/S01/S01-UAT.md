# S01: S01 — UAT

**Milestone:** M061
**Written:** 2026-04-12T06:09:56.912Z

# UAT — S01 Evidence-backed route inventory

## Preconditions
- Workspace uses the split sibling layout described by `AGENTS.md`: `mesh-lang/` plus `../hyperpush-mono/`.
- `../hyperpush-mono/mesher/client/` is present and uses the current TanStack Start dashboard shell.
- Local Postgres is available for Mesher seeding at `postgres://postgres:postgres@127.0.0.1:5432/mesher`.
- `target/debug/meshc` exists in `mesh-lang` or the sibling-workspace toolchain resolution path is otherwise healthy.

## Test case 1 — Canonical inventory exists beside the client package
1. Open `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`.
   - Expected: the document is clearly maintainer-facing and lives beside `mesher/client`, not only inside `.gsd/`.
2. Confirm the inventory table contains exactly eight top-level rows.
   - Expected: keys are `issues`, `performance`, `solana-programs`, `releases`, `alerts`, `bounties`, `treasury`, and `settings`.
3. Confirm the pathname column mirrors the canonical route map.
   - Expected: `issues` uses `/`; `settings` uses `/settings`; no extra or missing top-level route rows exist.
4. Confirm classifications are honest.
   - Expected: `issues`, `alerts`, and `settings` are `mixed`; `performance`, `solana-programs`, `releases`, `bounties`, and `treasury` are `mock-only`; no row claims fully `live` top-level parity.

## Test case 2 — Every row carries code evidence, proof evidence, and boundary notes
1. For each table row in `ROUTE-INVENTORY.md`, inspect the code-evidence and proof-evidence cells.
   - Expected: each row cites at least one concrete code anchor and one rerunnable proof surface; no evidence cell is blank.
2. Inspect the route notes and backend/boundary wording.
   - Expected: each row includes a short backend seam or boundary note rather than only a label.
3. Inspect the mixed-route breakdown section.
   - Expected: Issues, Alerts, and Settings each have a more specific mixed-surface breakdown instead of a route-level label only.

## Test case 3 — Structural doc-parity contract fails closed on drift
1. Run `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`.
   - Expected: the test passes in the current tree.
2. Review what the contract covers.
   - Expected: it enforces route-key/path parity against `components/dashboard/dashboard-route-map.ts`, the allowed mixed/mock-only top-level classifications, non-empty code/proof evidence, recognized proof-suite references, and verifier wiring references.
3. Edge check: inspect the test/helper sources.
   - Expected: the verifier reads the route map and markdown inventory directly instead of introducing a second runtime classification registry.

## Test case 4 — Maintainer workflow exposes one retained verifier
1. Open `../hyperpush-mono/mesher/client/package.json` and `../hyperpush-mono/mesher/client/README.md`.
   - Expected: the package exposes `verify:route-inventory`, and the README points maintainers to both `ROUTE-INVENTORY.md` and the dedicated verifier command.
2. Run `bash ../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh`.
   - Expected: the script runs the structural contract, seed helpers, and targeted dev/prod Playwright greps with retained phase/status files.
3. Inspect `../hyperpush-mono/mesher/.tmp/m061-s01/verify-client-route-inventory/` after the run.
   - Expected: `phase-report.txt`, `status.txt`, and `current-phase.txt` exist and identify the current/failing phase.

## Test case 5 — Failure visibility is truthful when a retained phase regresses
1. If the retained verifier exits non-zero, inspect the phase files and the named failing log.
   - Expected: the failing phase is explicit (for example `route-inventory-prod`) and the corresponding retained log path is printed.
2. Inspect the linked Playwright artifacts under `../hyperpush-mono/mesher/client/test-results/`.
   - Expected: artifacts exist for the failing browser proof so a maintainer can resume debugging without re-running the entire slice blindly.
3. Edge check for zero-live alerts:
   - Expected: a truthful `ready` alerts state may render `alerts-empty-state` instead of `alerts-list`; this is valid and should not be misclassified as bootstrap failure.

## Test case 6 — Browser-history route parity stays truthful after the S01 proof fix
1. Run `env PLAYWRIGHT_PROJECT=dev npm --prefix ../hyperpush-mono/mesher/client exec -- playwright test --config ../hyperpush-mono/mesher/client/playwright.config.ts --project=dev --grep "issues interactions persist across shell re-renders and browser history"`.
   - Expected: the route-parity history proof passes.
2. Review the tested behavior conceptually.
   - Expected: after back/forward navigation, the test only re-clicks the issue row if the issue is no longer selected; it does not close the panel accidentally by toggling an already-selected row.

## Known current limitation to record during UAT
- The retained verifier may still fail in the prod admin/ops alert lifecycle path because the newly created seeded alert is not observed quickly enough in `/api/v1/projects/default/alerts`. This does **not** invalidate the shipped inventory, parser contract, or failure-visibility rails, but it should be recorded as an operational gap for follow-on hardening.
