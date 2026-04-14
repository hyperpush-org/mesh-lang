# S04: S04 — UAT

**Milestone:** M061
**Written:** 2026-04-12T20:04:09.257Z

# UAT — S04 Canonical maintainer handoff and closeout rail

## Preconditions
- Workspace root is `mesh-lang` with sibling product repo available at `../hyperpush-mono`.
- Postgres is reachable at the local Mesher verification DSN used by the proof scripts.
- No human secrets are required; the local proof rail uses the seeded/dev Mesher defaults already wired into the product repo.

## Test Case 1 — Maintainer handoff is visible from the canonical inventory
1. Open `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`.
2. Confirm the document contains `## Maintainer handoff`.
3. Confirm the handoff includes `### Backend expansion order` and `### Proof commands to rerun`.
4. Read the handoff and verify it explains both how to interpret support-status vocabulary and which commands must stay green when rows change.

**Expected outcome:** The canonical inventory itself is enough for a backend maintainer to understand what to expand next and what proof commands to rerun after edits.

## Test Case 2 — Product-root and client README surfaces point to the same closeout rail
1. Open `../hyperpush-mono/mesher/client/README.md`.
2. Confirm it references `bash scripts/verify-m061-s04.sh` and the canonical route inventory.
3. Open `../hyperpush-mono/README.md`.
4. Confirm it also references `bash scripts/verify-m061-s04.sh` and no longer describes `mesher/client` as a mock-only dashboard.

**Expected outcome:** Both maintainer entry points route readers toward the same canonical inventory and root closeout command.

## Test Case 3 — Structural contract catches handoff/wrapper drift
1. Run `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`.
2. Confirm the test suite passes.
3. If intentionally experimenting later, remove one handoff heading or wrapper marker and rerun the same command.

**Expected outcome:** The normal run passes; a drifted heading/marker fails closed with a source-aware message naming the missing section or command.

## Test Case 4 — Root closeout wrapper emits fail-closed observability artifacts
1. Run `bash ../hyperpush-mono/scripts/verify-m061-s04.sh`.
2. Inspect `../hyperpush-mono/.tmp/m061-s04/verify/`.
3. Confirm `status.txt`, `current-phase.txt`, `phase-report.txt`, and delegated `latest-proof-bundle.txt` behavior are all present on a good run, or that the wrapper clearly names the failing delegated phase on a bad run.

**Expected outcome:** The wrapper either completes with retained proof artifacts or fails with enough retained evidence to localize the broken phase without guesswork.

## Test Case 5 — Isolated seeding does not silently trust a stray backend
1. Ensure something is already answering on the requested seed port, or use the existing contract coverage in the node:test suite.
2. Run the isolated seeding scenario through `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`.
3. Confirm the default path reports that it is ignoring the existing backend and starting an isolated verification backend unless reuse is explicitly requested.
4. Confirm explicit reuse only happens when `MESHER_REUSE_RUNNING_BACKEND=true` is set.

**Expected outcome:** Wrong-runtime reuse is opt-in only; the default path isolates itself.

## Edge Cases
- If the combined route-inventory replay later fails with `ERR_CONNECTION_REFUSED`, inspect the first assertion failure earlier in `route-inventory-dev.log` or `route-inventory-prod.log`; later connection errors are usually fallout, not the root cause.
- If the root wrapper is green but the structural contract is red, treat that as documentation/marker drift rather than runtime drift.
- If the structural contract is green but the root wrapper is red, treat that as a runtime/seeding/browser-replay problem in the delegated closeout rail.

