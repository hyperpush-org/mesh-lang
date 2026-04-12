# S02: Mixed-surface audit

**Goal:** Extend the canonical `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` from top-level route labels to a fail-closed mixed-surface inventory for `issues`, `alerts`, and `settings`, so maintainers can see which panels, subsections, and controls are truly live versus explicitly shell-backed without inventing a second registry.
**Demo:** Maintainers can answer exactly which Issues, Alerts, and Settings panels and controls are real versus shell-only.

## Must-Haves

- Expand `## mixed-route breakdown` into three structured markdown tables for `Issues`, `Alerts`, and `Settings`, each using stable surface keys plus level/classification/evidence/seam/boundary columns.
- Document panel/subsection/control granularity for every mixed route: Issues must cover overview, list/detail, supported actions, shell-only controls, and the proof harness; Alerts must cover overview, list/detail, supported actions, and shell-only controls; Settings must cover General, Team, API Keys, Alert Rules, Alert Channels, and every still-mock-only tab.
- Keep canonical vocabulary honest: use normalized row classifications such as `mixed`, `live`, `mock-only`, and `shell-only`; keep runtime `fallback` described in notes/diagnostics rather than as the durable contract classification.
- Extend `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs` and `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` so mixed-surface rows fail closed on missing/duplicate/drifted entries, blank evidence cells, unexpected classifications, and unrecognized proof references.
- Add only the minimum Playwright assertions needed in existing suites so every proof-cited fine-grained row is backed by real runtime evidence from `mesh-lang` using the explicit sibling config path.

## Threat Surface

- **Abuse**: a hand-edited mixed-surface table could overstate unsupported controls as live, collapse runtime `fallback` into canonical `mock-only`, or hide that the issue proof harness is diagnostic-only unless the parser/test rejects drift.
- **Data exposure**: Settings proof rails touch team membership metadata and one-time API key reveal UI; docs and assertions must cite selectors/files only and must not record revealed secret material in markdown or logs.
- **Input trust**: the audited surfaces send user-entered values through same-origin settings and lifecycle APIs, so the inventory must distinguish validated live writes from shell-only chrome instead of trusting labels alone.

## Requirement Impact

- **Requirements touched**: `R168`, `R170`, `R171`
- **Re-verify**: fine-grained row coverage for Issues/Alerts/Settings, exact mixed-surface row/classification parity in the parser test, and explicit runtime assertions for any newly claimed shell-only or live-backed controls.
- **Decisions revisited**: `D524`, `D525`, `D526`, `D527`, `D528`

## Proof Level

- This slice proves: integration
- Real runtime required: yes
- Human/UAT required: no

## Verification

- `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`
- `env PLAYWRIGHT_PROJECT=dev npm --prefix ../hyperpush-mono/mesher/client exec -- playwright test --config ../hyperpush-mono/mesher/client/playwright.config.ts ../hyperpush-mono/mesher/client/tests/e2e/issues-live-read.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/issues-live-actions.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts`

## Observability / Diagnostics

- Runtime signals: row-level `data-source` / `data-state` markers, action source notes, section error banners, and retained mutation diagnostics already exposed by the Issues, Alerts, and Settings shells.
- Inspection surfaces: `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`, `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`, and the existing Playwright suites under `../hyperpush-mono/mesher/client/tests/e2e/`.
- Failure visibility: the structural contract should name the offending section/surface row when docs drift, and the targeted browser suites should fail on explicit selectors such as `*-source`, `*-status-banner`, `*-mock-only-banner`, or `*-action-error` rather than on screenshots.
- Redaction constraints: do not persist team/member identifiers or revealed API key secrets in the document; retain selector names, file references, and test diagnostics only.

## Integration Closure

- Upstream surfaces consumed: `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`, `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs`, `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`, `../hyperpush-mono/mesher/client/components/dashboard/issues-page.tsx`, `../hyperpush-mono/mesher/client/components/dashboard/issue-detail.tsx`, `../hyperpush-mono/mesher/client/components/dashboard/issue-list.tsx`, `../hyperpush-mono/mesher/client/components/dashboard/stats-bar.tsx`, `../hyperpush-mono/mesher/client/components/dashboard/alert-detail.tsx`, `../hyperpush-mono/mesher/client/components/dashboard/alert-list.tsx`, `../hyperpush-mono/mesher/client/components/dashboard/alert-stats.tsx`, `../hyperpush-mono/mesher/client/components/dashboard/settings/settings-page.tsx`, `../hyperpush-mono/mesher/client/components/dashboard/settings/settings-live-state.tsx`, and the existing Issues/Alerts/Settings Playwright suites.
- New wiring introduced in this slice: canonical mixed-surface tables in the maintainer doc, parser/test coverage for exact section/surface pairs, and any minimal selector/assertion additions needed so proof cells remain truthful at fine granularity.
- What remains before the milestone is truly usable end-to-end: S03 still needs the backend gap map, and S04 still needs final proof-rail hardening/validation for the full milestone closeout.

## Tasks

- [x] **T01: Convert the mixed-route breakdown into structured Issues/Alerts/Settings tables** `est:75m`
  - Why: The slice demo is impossible until the canonical maintainer doc stops hand-waving mixed routes and names the exact panels and controls that are live versus shell-backed.
  - Files: `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`, `../hyperpush-mono/mesher/client/components/dashboard/issues-page.tsx`, `../hyperpush-mono/mesher/client/components/dashboard/issue-detail.tsx`, `../hyperpush-mono/mesher/client/components/dashboard/alert-detail.tsx`, `../hyperpush-mono/mesher/client/components/dashboard/settings/settings-page.tsx`, `../hyperpush-mono/mesher/client/components/dashboard/settings/settings-live-state.tsx`
  - Do: Replace the prose-only mixed-route bullets with three markdown tables under `### Issues`, `### Alerts`, and `### Settings`; give each row a stable backticked surface key, level, normalized classification, code evidence, proof evidence, live-seam summary, and boundary note; use the minimum row sets planned for Issues/Alerts/Settings; keep `fallback` documented as a runtime condition in notes and invariants rather than as the canonical row classification; and keep the issue proof harness explicitly labeled as diagnostic chrome rather than a supported maintainer action.
  - Verify: `python3 - <<'PY'
from pathlib import Path
text = Path('../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md').read_text()
for heading in ('### Issues', '### Alerts', '### Settings'):
    assert heading in text, heading
for needle in (
    '| `overview` | `panel` | `mixed` |',
    '| `live-actions` | `control` | `live` |',
    '| `shell-controls` | `control` | `shell-only` |',
    '| `proof-harness` | `control` | `shell-only` |',
    '| `general` | `panel` | `mixed` |',
    '| `team` | `panel` | `live` |',
    '| `alert-channels` | `subsection` | `shell-only` |',
    '| `profile` | `tab` | `mock-only` |',
):
    assert needle in text, needle
PY`
  - Done when: `ROUTE-INVENTORY.md` contains machine-readable Issues/Alerts/Settings tables with stable surface keys and honest row-level boundaries that cover the slice demo.
- [x] **T02: Extend the route-inventory parser and contract test for mixed-surface rows** `est:90m`
  - Why: R168 and R171 only stay useful if future edits to the maintainer doc fail closed instead of silently drifting back to prose or overstated live claims.
  - Files: `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs`, `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`, `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`
  - Do: Update the parser so it understands section-scoped mixed-surface tables in addition to the top-level inventory; lock the exact expected `(route section, surface key)` pairs for Issues, Alerts, and Settings; preserve S01's top-level route-map parity contract; reject duplicate rows, blank evidence cells, `fallback` as a canonical classification, and unrecognized proof references; and make failures name the offending section/surface row instead of only reporting generic markdown drift.
  - Verify: `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`
  - Done when: the structural contract fails on any mixed-surface row drift while still preserving the top-level route-map parity guarantees from S01.
- [ ] **T03: Add minimal runtime assertions for fine-grained mixed-surface proof** `est:105m`
  - Why: The doc and parser only become trustworthy proof rails when each cited fine-grained row is backed by an explicit runtime assertion rather than by inference from a broader route-level test.
  - Files: `../hyperpush-mono/mesher/client/tests/e2e/issues-live-read.spec.ts`, `../hyperpush-mono/mesher/client/tests/e2e/issues-live-actions.spec.ts`, `../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts`, `../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts`, `../hyperpush-mono/mesher/client/components/dashboard/issue-detail.tsx`, `../hyperpush-mono/mesher/client/components/dashboard/alert-detail.tsx`
  - Do: Audit the proof suites cited by the new mixed-surface rows and add only the missing assertions or test ids needed to make those citations honest; prefer existing `data-source`, `data-state`, banner, and action selectors; likely close proof gaps around grouped issue shell-only controls, alert shell-only controls such as `alert-detail-copy-link`, and any Settings mixed/mock-only row that is still only implied; keep fallback/error-path assertions intact; and always run Playwright from `mesh-lang` with `--config ../hyperpush-mono/mesher/client/playwright.config.ts`.
  - Verify: `env PLAYWRIGHT_PROJECT=dev npm --prefix ../hyperpush-mono/mesher/client exec -- playwright test --config ../hyperpush-mono/mesher/client/playwright.config.ts ../hyperpush-mono/mesher/client/tests/e2e/issues-live-read.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/issues-live-actions.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts`
  - Done when: the cited Issues/Alerts/Settings suites pass from `mesh-lang`, and every fine-grained proof row added in T01 has an explicit runtime assertion path in an existing suite.

## Files Likely Touched

- `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`
- `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs`
- `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`
- `../hyperpush-mono/mesher/client/components/dashboard/issues-page.tsx`
- `../hyperpush-mono/mesher/client/components/dashboard/issue-detail.tsx`
- `../hyperpush-mono/mesher/client/components/dashboard/issue-list.tsx`
- `../hyperpush-mono/mesher/client/components/dashboard/stats-bar.tsx`
- `../hyperpush-mono/mesher/client/components/dashboard/alert-detail.tsx`
- `../hyperpush-mono/mesher/client/components/dashboard/alert-list.tsx`
- `../hyperpush-mono/mesher/client/components/dashboard/alert-stats.tsx`
- `../hyperpush-mono/mesher/client/components/dashboard/settings/settings-page.tsx`
- `../hyperpush-mono/mesher/client/components/dashboard/settings/settings-live-state.tsx`
- `../hyperpush-mono/mesher/client/tests/e2e/issues-live-read.spec.ts`
- `../hyperpush-mono/mesher/client/tests/e2e/issues-live-actions.spec.ts`
- `../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts`
- `../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts`
