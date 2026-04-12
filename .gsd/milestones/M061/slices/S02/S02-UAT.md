# S02: Mixed-surface audit — UAT

**Milestone:** M061
**Written:** 2026-04-12T08:10:27.172Z

# UAT — S02 Mixed-surface audit

## Preconditions

- Worktree is at the completed S02 state.
- Local Postgres for Mesher is reachable with the default dev connection expected by the Playwright harness.
- Commands are run from `mesh-lang/`.

## Test Case 1 — Maintainer inventory shows row-level truth for mixed routes
1. Open `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`.
2. Go to `## mixed-route breakdown`.
3. Confirm there are separate `### Issues`, `### Alerts`, and `### Settings` tables.
4. In **Issues**, verify rows exist for `overview`, `list`, `detail`, `live-actions`, `shell-controls`, and `proof-harness`.
5. In **Alerts**, verify rows exist for overview/list/detail plus supported actions and shell-only controls.
6. In **Settings**, verify rows exist for `general`, `team`, `api-keys`, `alert-rules`, `alert-channels`, and the remaining mock-only tabs.

**Expected outcome:** Every mixed route is decomposed into stable surface rows with normalized `live` / `mixed` / `shell-only` / `mock-only` classifications plus code and proof evidence.

## Test Case 2 — Structural contract fails closed on mixed-surface drift
1. Run:
   `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`
2. Observe the passing baseline.
3. (Edge check) Temporarily change one mixed-surface row classification or surface key in `ROUTE-INVENTORY.md` and rerun the command.
4. Restore the file afterward.

**Expected outcome:** The baseline run passes. The drifted run fails with a message naming the exact section/surface row instead of a generic markdown mismatch.

## Test Case 3 — Runtime proof shows truthful live vs shell-only behavior
1. Run:
   `env PLAYWRIGHT_PROJECT=dev npm --prefix ../hyperpush-mono/mesher/client exec -- playwright test --config ../hyperpush-mono/mesher/client/playwright.config.ts ../hyperpush-mono/mesher/client/tests/e2e/issues-live-read.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/issues-live-actions.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts`
2. Review the passing output.
3. Confirm the suites exercised:
   - Issues live read detail hydration and fallback behavior
   - Issues supported live actions, proof-harness diagnostics, and mutation failure handling
   - Alerts live acknowledge/resolve plus shell-only controls
   - Settings live general/team/api-key/alert-rule flows plus shell-only/mock-only markers

**Expected outcome:** All 21 tests pass. Proof uses same-origin `/api/v1` requests, explicit `data-*` markers, and visible failure diagnostics/toasts instead of screenshots alone.

## Test Case 4 — Fresh-backend issue proof is self-contained
1. Start from a clean Playwright run without manually invoking `bash mesher/scripts/seed-live-issue.sh`.
2. Run only the Issues + walkthrough suites from Test Case 3.
3. Confirm the suites still pass.

**Expected outcome:** The deterministic read/action issue rows are created by the suites themselves through same-origin seed events, so a fresh temporary backend still satisfies the proof rail without any external seed step.

