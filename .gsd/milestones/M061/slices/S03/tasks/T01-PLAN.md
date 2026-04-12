---
estimated_steps: 4
estimated_files: 8
skills_used:
  - test
---

# T01: Map mixed-route client promises to existing and partial backend seams

**Slice:** S03 — Backend gap map
**Milestone:** M061

## Description

Add the first half of the backend gap map to `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` for the routes that already talk to Mesher today. Reuse the S02 surface vocabulary where possible (`issues/overview`, `issues/detail`, `issues/live-actions`, `issues/shell-controls`, `alerts/overview`, `alerts/detail`, `alerts/live-actions`, `alerts/shell-controls`, `settings/general`, `settings/team`, `settings/api-keys`, `settings/alert-rules`, and `settings/alert-channels`). For each row, record the client promise, the concrete backend seam currently used, a stable support status (`covered`, `missing-payload`, `missing-controls`, or `no-route-family`), and the remaining backend work. Base every claim on `main.mpl`, `mesher-api.ts`, the live adapters, and the issue/alert/settings components so fallback-populated fields do not get mistaken for real backend coverage.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `../hyperpush-mono/mesher/main.mpl` | Treat route-registration ambiguity as a gap and stop short of claiming a live seam. | N/A — local file reads are synchronous and cheap. | Do not infer missing or renamed routes; document the missing seam explicitly and leave a code anchor note. |
| `../hyperpush-mono/mesher/client/lib/mesher-api.ts` | Fail the row back to an explicit gap instead of inventing a client/backend contract. | N/A — local file reads are synchronous and cheap. | Distinguish exported helper paths from visible UI text; only cite actual same-origin request helpers. |
| `../hyperpush-mono/mesher/client/lib/issues-live-adapter.ts`, `../hyperpush-mono/mesher/client/lib/admin-ops-live-adapter.ts`, and the mixed-route components | Keep fallback-derived fields and shell-only controls out of the `covered` bucket. | N/A — local file reads are synchronous and cheap. | If a field/control only overlays fallback content, classify it as `missing-payload` or `missing-controls` instead of live-backed. |

## Load Profile

- **Shared resources**: local markdown/code reads across the canonical doc, route registry, client API helpers, adapters, and route components.
- **Per-operation cost**: a fixed-size code audit over one route registry, two client adapter modules, and three mixed-route UI surfaces.
- **10x breakpoint**: if mixed surfaces multiply later, preserve stable route/surface keys and avoid exploding shell-only controls into per-button rows unless backend sequencing genuinely needs that detail.

## Negative Tests

- **Malformed inputs**: visible fallback metrics or shell-only buttons must not be promoted to `covered` just because they render inside a mixed panel.
- **Error paths**: missing route-registration evidence in `main.mpl`, missing request helpers in `mesher-api.ts`, or adapter overlays that only derive fallback summaries must all resolve to explicit gap rows rather than optimistic prose.
- **Boundary conditions**: `issues/live-actions`, `alerts/live-actions`, `settings/team`, `settings/api-keys`, and `settings/alert-rules` can be `covered`; overview/detail rows that still rely on fallback aggregates or shell-only chrome must stay partial; `settings/alert-channels` must stay a missing seam.

## Steps

1. Audit `main.mpl` and `mesher-api.ts` to list the existing issue, alert, settings, storage, team, API key, and alert-rule route families that already exist.
2. Use the live adapters plus `issue-detail.tsx`, `alert-detail.tsx`, and `settings-page.tsx` to distinguish fully covered actions from fallback-derived fields and shell-only controls that still outrun the backend.
3. Add mixed-route backend-gap rows to `ROUTE-INVENTORY.md` using stable route/surface keys, support status values, concrete seam summaries, and missing-support notes.
4. Keep the row wording actionable for backend planners: name the missing payload fields, aggregates, or unsupported controls instead of vague “not done yet” prose.

## Must-Haves

- [ ] Mixed-route backend-gap rows exist for the supported Issues, Alerts, and Settings surfaces that currently talk to Mesher or visibly outrun those seams.
- [ ] Each row cites both the client promise and the current backend seam, and uses one of the planned stable status values.
- [ ] Covered rows stay limited to actually supported lifecycle/member/key/rule flows; fallback overlays and shell-only chrome remain explicitly partial or missing.

## Inputs

- `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`
- `../hyperpush-mono/mesher/main.mpl`
- `../hyperpush-mono/mesher/client/lib/mesher-api.ts`
- `../hyperpush-mono/mesher/client/lib/issues-live-adapter.ts`
- `../hyperpush-mono/mesher/client/lib/admin-ops-live-adapter.ts`
- `../hyperpush-mono/mesher/client/components/dashboard/issue-detail.tsx`
- `../hyperpush-mono/mesher/client/components/dashboard/alert-detail.tsx`
- `../hyperpush-mono/mesher/client/components/dashboard/settings/settings-page.tsx`

## Expected Output

- `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`

## Verification

`python3 - <<'PY'
from pathlib import Path
text = Path('../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md').read_text()
for needle in (
    '`issues/overview`',
    '`issues/live-actions`',
    '`alerts/detail`',
    '`alerts/live-actions`',
    '`settings/general`',
    '`settings/team`',
    '`settings/api-keys`',
    '`settings/alert-rules`',
    '`settings/alert-channels`',
):
    assert needle in text, needle
PY`
