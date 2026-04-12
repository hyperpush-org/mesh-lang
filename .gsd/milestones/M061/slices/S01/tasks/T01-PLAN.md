---
estimated_steps: 4
estimated_files: 7
skills_used:
  - react-best-practices
  - tanstack-start-best-practices
---

# T01: Author the canonical client route inventory document

**Slice:** S01 — Evidence-backed route inventory
**Milestone:** M061

## Description

Create the maintainer-facing inventory surface before adding automation. The document should live beside the client app, derive its top-level rows from `dashboard-route-map.ts`, and make the current truth legible to future backend planners without pretending mixed routes are fully live.

## Steps

1. Create `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` as the canonical top-level inventory rather than overloading `.gsd/` artifacts or the package README.
2. Add one row per `DashboardRouteKey` with canonical pathname, normalized classification, concrete code evidence refs, proof evidence refs, backend seam summary, and a short boundary note.
3. Add a short mixed-route breakdown for Issues, Alerts, and Settings that points to the component files and proof suites future slices will expand.
4. Document the important invariants: Issues lives at `/`, Settings' in-app label `mixed live` normalizes to milestone language `mixed`, and runtime `fallback` is not the same as canonical `mock-only` classification.

## Must-Haves

- [ ] Exactly eight top-level rows mirror `dashboard-route-map.ts`, including Issues at `/`.
- [ ] No route is labeled fully `live`; Issues, Alerts, and Settings are `mixed`, and the other five routes are `mock-only`.
- [ ] Every row cites at least one code anchor and one existing proof rail that a maintainer can rerun.

## Verification

- `python3 - <<'PY'
from pathlib import Path
import re
text = Path('../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md').read_text()
rows = re.findall(r'^\| `([^`]+)` \| `([^`]+)` \| `(mixed|mock-only)` \|', text, re.M)
assert len(rows) == 8, rows
mapping = {key: path for key, path, _ in rows}
assert mapping['issues'] == '/'
assert mapping['settings'] == '/settings'
assert set(mapping) == {'issues','performance','solana-programs','releases','alerts','bounties','treasury','settings'}
PY`
- `rg -n "mixed-route breakdown|dashboard-route-parity.spec.ts|seeded-walkthrough.spec.ts|issues-page.tsx|alerts-page.tsx|settings-page.tsx" ../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`

## Inputs

- `../hyperpush-mono/mesher/client/components/dashboard/dashboard-route-map.ts` — canonical top-level route keys and pathnames.
- `../hyperpush-mono/mesher/client/components/dashboard/issues-page.tsx` — Issues top-level live/mixed evidence.
- `../hyperpush-mono/mesher/client/components/dashboard/issue-detail.tsx` — Issues shell-only boundary evidence.
- `../hyperpush-mono/mesher/client/components/dashboard/alerts-page.tsx` — Alerts top-level live/mixed evidence.
- `../hyperpush-mono/mesher/client/components/dashboard/alert-detail.tsx` — Alerts shell-only boundary evidence.
- `../hyperpush-mono/mesher/client/components/dashboard/settings/settings-page.tsx` — Settings support-map and shell-source evidence.
- `../hyperpush-mono/mesher/client/components/dashboard/settings/settings-live-state.tsx` — Settings section-source evidence.
- `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts` — direct-entry and route-shell parity proof rail.
- `../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts` — route-by-route seeded walkthrough proof rail.
- `../hyperpush-mono/mesher/client/tests/e2e/issues-live-read.spec.ts` — Issues live-read evidence.
- `../hyperpush-mono/mesher/client/tests/e2e/issues-live-actions.spec.ts` — Issues action evidence.
- `../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts` — Alerts and Settings live-action evidence.

## Expected Output

- `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` — canonical top-level route inventory with mixed-route notes and rerunnable evidence refs.
