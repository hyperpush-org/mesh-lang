# S01: Evidence-backed route inventory

**Goal:** Publish a maintainer-facing `mesher/client` route inventory, backed by structural parity checks and rerunnable proof rails, so every top-level dashboard route is truthfully classified from the canonical route map without overstating live coverage.
**Demo:** Maintainers can see every top-level `mesher/client` route classified as live, mixed, or mock-only with code/test evidence.

## Must-Haves

- Ship `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` as the canonical top-level maintainer inventory for all eight `DashboardRouteKey` entries from `../hyperpush-mono/mesher/client/components/dashboard/dashboard-route-map.ts`.
- Every inventory row must use the canonical pathname from the route map, classify the route as `mixed` or `mock-only`, cite at least one code anchor and one rerunnable proof rail, and include a short backend-seam or boundary note.
- Issues, Alerts, and Settings must be documented as `mixed` with honest boundary notes; Performance, Solana Programs, Releases, Bounties, and Treasury must be documented as `mock-only`; no row may claim fully `live` top-level parity.
- The maintainer workflow must expose one rerunnable verifier that fails on doc ↔ route-map drift and reuses the existing seeded Playwright rails that prove current live and mixed behavior in dev and prod.
- Advance `R167` and `R170`, support `R168` and `R171`, and stay bounded by `R173`, `R174`, and `R175` by avoiding backend implementation, shell redesign, or public-doc repositioning.

## Threat Surface

- **Abuse**: a hand-edited inventory could overstate live support, misclassify fallback as `mock-only`, or drift from `dashboard-route-map.ts` unless the verifier fails closed on row/key/path mismatches.
- **Data exposure**: retained verifier logs and docs must stay limited to route metadata, file references, and Playwright diagnostics; they must not print secrets or reveal seeded API key material from the existing admin-ops rails.
- **Input trust**: `dashboard-route-map.ts`, mixed-route component files, markdown inventory rows, test grep filters, and seeded proof scripts are all trusted only after structural parsing and explicit assertions.

## Requirement Impact

- **Requirements touched**: `R167`, `R168`, `R170`, `R171`, `R173`, `R174`, `R175`
- **Re-verify**: exact eight-route coverage, canonical pathname parity, honest mixed/mock-only labels, non-empty code/proof evidence cells, and seeded dev/prod proof execution for `dashboard route parity`, `issues live`, `admin and ops live`, and `seeded walkthrough`.
- **Decisions revisited**: `D523`, `D524`, `D525`, `D526`

## Proof Level

- This slice proves: integration
- Real runtime required: yes
- Human/UAT required: no

## Verification

- `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`
- `bash ../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh`

## Observability / Diagnostics

- Runtime signals: retained verifier phase/status files under `../hyperpush-mono/mesher/.tmp/m061-s01/verify-client-route-inventory/`, plus the existing same-origin runtime diagnostics already surfaced by `../hyperpush-mono/mesher/client/tests/e2e/live-runtime-helpers.ts`.
- Inspection surfaces: `../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh`, `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`, and `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`.
- Failure visibility: missing or extra route rows, pathname drift, blank evidence cells, zero-match Playwright greps, seed failures, and dev/prod proof regressions must all fail with a named phase and retained log path.
- Redaction constraints: retain route metadata and Playwright diagnostics only; do not print secrets, tokens, or revealed API key values from seeded settings flows.

## Integration Closure

- Upstream surfaces consumed: `../hyperpush-mono/mesher/client/components/dashboard/dashboard-route-map.ts`, `../hyperpush-mono/mesher/client/components/dashboard/issues-page.tsx`, `../hyperpush-mono/mesher/client/components/dashboard/issue-detail.tsx`, `../hyperpush-mono/mesher/client/components/dashboard/alerts-page.tsx`, `../hyperpush-mono/mesher/client/components/dashboard/alert-detail.tsx`, `../hyperpush-mono/mesher/client/components/dashboard/settings/settings-page.tsx`, `../hyperpush-mono/mesher/client/components/dashboard/settings/settings-live-state.tsx`, `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts`, `../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts`, `../hyperpush-mono/mesher/client/tests/e2e/issues-live-read.spec.ts`, `../hyperpush-mono/mesher/client/tests/e2e/issues-live-actions.spec.ts`, `../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts`, `../hyperpush-mono/mesher/scripts/seed-live-issue.sh`, `../hyperpush-mono/mesher/scripts/seed-live-admin-ops.sh`, and `../hyperpush-mono/mesher/client/README.md`.
- New wiring introduced in this slice: a canonical `ROUTE-INVENTORY.md` reference surface, a structural doc-parity parser/test, and a single retained verifier entrypoint exposed through the client maintainer workflow.
- What remains before the milestone is truly usable end-to-end: S02 still needs subsection/control-level mixed-route decomposition, and later slices still need the backend gap map plus any longer-term drift guards beyond the top-level inventory.

## Tasks

- [x] **T01: Author the canonical client route inventory document** `est:90m`
  - Why: The slice has no value until maintainers can open one document beside the client app and see the current top-level route truth without rereading route components and test suites.
  - Files: `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`, `../hyperpush-mono/mesher/client/components/dashboard/dashboard-route-map.ts`, `../hyperpush-mono/mesher/client/components/dashboard/issues-page.tsx`, `../hyperpush-mono/mesher/client/components/dashboard/alerts-page.tsx`, `../hyperpush-mono/mesher/client/components/dashboard/settings/settings-page.tsx`, `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts`, `../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts`
  - Do: Create `ROUTE-INVENTORY.md` as the canonical top-level inventory; add one row per route-map entry with canonical pathname, normalized classification, code evidence, proof evidence, backend seam summary, and boundary notes; add a short mixed-route breakdown for Issues, Alerts, and Settings; document the invariants that Issues lives at `/`, Settings' in-app `mixed live` label normalizes to `mixed`, and runtime fallback is not the same as canonical `mock-only`.
  - Verify: `python3 - <<'PY'
from pathlib import Path
import re
text = Path('../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md').read_text()
rows = re.findall(r'^\| `([^`]+)` \| `([^`]+)` \| `(mixed|mock-only)` \|', text, re.M)
assert len(rows) == 8, rows
mapping = {key: path for key, path, _ in rows}
assert mapping['issues'] == '/'
assert mapping['settings'] == '/settings'
assert set(mapping) == {'issues','performance','solana-programs','releases','alerts','bounties','treasury','settings'}
PY
rg -n "mixed-route breakdown|dashboard-route-parity.spec.ts|seeded-walkthrough.spec.ts|issues-page.tsx|alerts-page.tsx|settings-page.tsx" ../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`
  - Done when: the document contains exactly eight canonical rows, all three mixed routes are called out honestly, no route is overstated as fully live, and every row names both code and proof evidence.
- [ ] **T02: Add a fail-closed doc-parity parser and structural test** `est:75m`
  - Why: A human-maintained document is not trustworthy on its own; the slice needs an executable contract that fails when the route map, classifications, or evidence references drift.
  - Files: `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs`, `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`, `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`, `../hyperpush-mono/mesher/client/components/dashboard/dashboard-route-map.ts`, `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts`, `../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts`, `../hyperpush-mono/mesher/client/tests/e2e/issues-live-read.spec.ts`, `../hyperpush-mono/mesher/client/tests/e2e/issues-live-actions.spec.ts`, `../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts`
  - Do: Add a tiny helper that parses the canonical route map and the markdown inventory into stable row objects; add a `node:test` contract that asserts exact key/path parity, expected mixed/mock-only classifications, and non-empty code/proof evidence; reject missing rows, duplicate rows, `/issues` drift, `mixed live` classification cells, and rows that cite no recognized proof suite; keep the parser scoped to top-level inventory truth rather than introducing a second runtime registry.
  - Verify: `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`
  - Done when: the structural test fails closed on route or evidence drift and the helper provides one canonical parse path for the new inventory doc.
- [ ] **T03: Wrap the inventory proof in one retained maintainer verifier** `est:105m`
  - Why: R170 and D526 require rerunnable proof, not just a parser test, so maintainers need one obvious command that executes both the structural contract and the existing seeded runtime evidence rails.
  - Files: `../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh`, `../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh`, `../hyperpush-mono/mesher/scripts/seed-live-issue.sh`, `../hyperpush-mono/mesher/scripts/seed-live-admin-ops.sh`, `../hyperpush-mono/mesher/client/package.json`, `../hyperpush-mono/mesher/client/README.md`, `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`
  - Do: Add a retained-phase verifier script modeled on `verify-maintainer-surface.sh`; make it run the structural `node:test`, both seed helpers, and targeted dev/prod Playwright greps for `dashboard route parity|issues live|admin and ops live|seeded walkthrough`; fail closed on missing files, zero-match test greps, or phase timeouts; expose the command through `client/package.json`; update `client/README.md` so the README points to `ROUTE-INVENTORY.md` as the canonical map and documents the verifier command.
  - Verify: `bash ../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh`
  - Done when: one command reruns structural and runtime proof with retained phase logs, names any failing phase and artifact path, and the maintainer workflow clearly points to the canonical inventory plus verifier.

## Files Likely Touched

- `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`
- `../hyperpush-mono/mesher/client/components/dashboard/dashboard-route-map.ts`
- `../hyperpush-mono/mesher/client/components/dashboard/issues-page.tsx`
- `../hyperpush-mono/mesher/client/components/dashboard/issue-detail.tsx`
- `../hyperpush-mono/mesher/client/components/dashboard/alerts-page.tsx`
- `../hyperpush-mono/mesher/client/components/dashboard/alert-detail.tsx`
- `../hyperpush-mono/mesher/client/components/dashboard/settings/settings-page.tsx`
- `../hyperpush-mono/mesher/client/components/dashboard/settings/settings-live-state.tsx`
- `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts`
- `../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts`
- `../hyperpush-mono/mesher/client/tests/e2e/issues-live-read.spec.ts`
- `../hyperpush-mono/mesher/client/tests/e2e/issues-live-actions.spec.ts`
- `../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts`
- `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs`
- `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`
- `../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh`
- `../hyperpush-mono/mesher/client/package.json`
- `../hyperpush-mono/mesher/client/README.md`
