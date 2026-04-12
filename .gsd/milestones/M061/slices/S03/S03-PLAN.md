# S03: Backend gap map

**Goal:** Extend `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` with a fail-closed backend gap map that traces each current client promise to an existing Mesher seam, a partial seam, or a documented missing seam, so backend maintainers can plan follow-up work without re-auditing the client shell.
**Demo:** Backend maintainers can trace each client-side promise to an existing backend seam or a documented missing seam.

## Must-Haves

- Add a canonical `## Backend gap map` section to `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` that reuses stable top-level route keys and existing S02 surface keys wherever possible.
- For each backend-gap row, document the current client promise, the current backend seam, a stable support status (`covered`, `missing-payload`, `missing-controls`, or `no-route-family`), and the missing backend support that still blocks full truth.
- Cover both sides of the current product reality: mixed routes with partial or complete live support (`issues`, `alerts`, `settings`) and wholly mock-only route families (`performance`, `solana-programs`, `releases`, `bounties`, `treasury`).
- Extend `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs` and `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` so backend-gap rows fail closed on missing sections, duplicate keys, blank evidence/seam cells, unsupported status values, or drifted expected row sets.
- Keep `readRouteInventory()` / `parseRouteInventoryMarkdown()` stable for top-level callers; S03 must layer backend-gap parsing on top of the S01/S02 contract rather than replacing it.

## Threat Surface

- **Abuse**: a hand-edited backend gap map could overstate unsupported client promises as already backed, hide shell-only controls inside mixed panels, or collapse payload/control gaps into a false `covered` claim unless the parser/test contract rejects drift.
- **Data exposure**: the mapped surfaces include team membership metadata and one-time API key reveal UI, so the document must cite file paths, selectors, and route families only — never copied secrets, user ids, or raw payload samples.
- **Input trust**: issues, alerts, settings, team, API key, and alert-rule surfaces all accept user-entered values; the map must distinguish validated same-origin writes from visible chrome or fallback-populated fields.

## Requirement Impact

- **Requirements touched**: `R169`, `R170`, `R171`
- **Re-verify**: backend-gap row coverage for mixed and mock-only routes, exact expected row/status parity in the structural contract, and preserved stability of the existing top-level inventory readers.
- **Decisions revisited**: `D524`, `D526`, `D528`, `D529`, `D531`

## Proof Level

- This slice proves: contract
- Real runtime required: no
- Human/UAT required: no

## Verification

- `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`
- `python3 -c "from pathlib import Path; text = Path('../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md').read_text(); assert '## Backend gap map' in text; needles = ['`issues/overview`','`alerts/live-actions`','`settings/alert-channels`','`performance`','`treasury`','`covered`','`missing-payload`','`missing-controls`','`no-route-family`']; assert all(needle in text for needle in needles)"`

## Observability / Diagnostics

- Runtime signals: the structural contract should name the exact backend-gap row or section that drifted; no new runtime logs are required for this documentation slice.
- Inspection surfaces: `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` plus `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`.
- Failure visibility: missing/duplicate gap keys, unsupported status values, and stale seam summaries become explicit parser/test failures instead of silent markdown drift.
- Redaction constraints: keep the map at code-anchor and route-family level; do not copy secrets, API key values, or user identifiers into the document or tests.

## Integration Closure

- Upstream surfaces consumed: `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`, `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs`, `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`, `../hyperpush-mono/mesher/main.mpl`, `../hyperpush-mono/mesher/client/lib/mesher-api.ts`, `../hyperpush-mono/mesher/client/lib/issues-live-adapter.ts`, `../hyperpush-mono/mesher/client/lib/admin-ops-live-adapter.ts`, `../hyperpush-mono/mesher/client/components/dashboard/issue-detail.tsx`, `../hyperpush-mono/mesher/client/components/dashboard/alert-detail.tsx`, `../hyperpush-mono/mesher/client/components/dashboard/settings/settings-page.tsx`, and the mock-only route components under `../hyperpush-mono/mesher/client/components/dashboard/`.
- New wiring introduced in this slice: backend-gap tables keyed to stable route/surface ids plus parser/test helpers that lock those rows without changing the older top-level reader API.
- What remains before the milestone is truly usable end-to-end: S04 still needs the final milestone handoff and retained proof-rail hardening/validation, but backend maintainers should no longer need a fresh client audit to scope follow-up slices.

## Tasks

- [x] **T01: Map mixed-route client promises to existing and partial backend seams** `est:95m`
  - Why: R169 only becomes actionable when the current mixed routes stop reading like “some of it is live” and instead say exactly which promises are already covered versus which visible fields and controls still outrun the backend.
  - Files: `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`, `../hyperpush-mono/mesher/main.mpl`, `../hyperpush-mono/mesher/client/lib/mesher-api.ts`, `../hyperpush-mono/mesher/client/lib/issues-live-adapter.ts`, `../hyperpush-mono/mesher/client/lib/admin-ops-live-adapter.ts`, `../hyperpush-mono/mesher/client/components/dashboard/issue-detail.tsx`, `../hyperpush-mono/mesher/client/components/dashboard/alert-detail.tsx`, `../hyperpush-mono/mesher/client/components/dashboard/settings/settings-page.tsx`
  - Do: Audit the registered issue/alert/settings/team/api-key/alert-rule route families in `main.mpl` and `mesher-api.ts`, then use the live adapters plus mixed-route components to add backend-gap rows for the existing live families. Reuse stable S02 surface keys, keep supported lifecycle/member/key/rule flows in `covered`, and classify fallback-derived fields or shell-only controls as `missing-payload` or `missing-controls` instead of live-backed.
  - Verify: `python3 - <<'PY'
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
  - Done when: the backend gap map has stable mixed-route rows that backend maintainers can use to see which currently shipped issue/alert/settings promises are already covered and which still need payload or control support.
- [x] **T02: Document the missing backend route families behind mock-only dashboard routes** `est:75m`
  - Why: R171 is not met if maintainers still have to re-open every mock-only page to learn that Performance, Solana Programs, Releases, Bounties, and Treasury do not map to any backend route family yet.
  - Files: `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`, `../hyperpush-mono/mesher/main.mpl`, `../hyperpush-mono/mesher/client/components/dashboard/performance-page.tsx`, `../hyperpush-mono/mesher/client/components/dashboard/solana-programs-page.tsx`, `../hyperpush-mono/mesher/client/components/dashboard/releases-page.tsx`, `../hyperpush-mono/mesher/client/components/dashboard/bounties-page.tsx`, `../hyperpush-mono/mesher/client/components/dashboard/treasury-page.tsx`
  - Do: Confirm in `main.mpl` that no dedicated `/api/v1` route families exist for the remaining mock-only dashboard pages, then add route-scoped backend-gap rows that name the visible promise and the missing backend family each page would need. Keep these rows maintainable by grouping each wholly mock-backed page at route or major-subsection granularity instead of exploding every CTA into its own gap row.
  - Verify: `python3 - <<'PY'
from pathlib import Path
text = Path('../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md').read_text()
for needle in (
    '`performance`',
    '`solana-programs`',
    '`releases`',
    '`bounties`',
    '`treasury`',
    '`no-route-family`',
):
    assert needle in text, needle
PY`
  - Done when: the gap map names every remaining mock-only dashboard route and the missing backend family it still needs, without inventing unsupported route registrations.
- [ ] **T03: Lock the backend gap map with parser and contract coverage** `est:90m`
  - Why: R170 only advances if the new backend-gap rows fail closed on drift, rather than becoming another prose-only table that silently rots.
  - Files: `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs`, `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`, `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`
  - Do: Extend the document parser to read backend-gap sections on top of the existing S01/S02 model, preserve the older top-level reader wrappers, and lock exact expected mixed-route and mock-only gap rows plus allowed status values in `verify-client-route-inventory.test.mjs`. Add regression cases for missing sections, duplicate keys, blank seam/missing-support cells, and stale row sets so future edits fail with actionable messages.
  - Verify: `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`
  - Done when: the backend gap map is guarded by a passing structural contract that names the exact row or section when documentation or parser expectations drift.

## Files Likely Touched

- `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`
- `../hyperpush-mono/mesher/main.mpl`
- `../hyperpush-mono/mesher/client/lib/mesher-api.ts`
- `../hyperpush-mono/mesher/client/lib/issues-live-adapter.ts`
- `../hyperpush-mono/mesher/client/lib/admin-ops-live-adapter.ts`
- `../hyperpush-mono/mesher/client/components/dashboard/issue-detail.tsx`
- `../hyperpush-mono/mesher/client/components/dashboard/alert-detail.tsx`
- `../hyperpush-mono/mesher/client/components/dashboard/settings/settings-page.tsx`
- `../hyperpush-mono/mesher/client/components/dashboard/performance-page.tsx`
- `../hyperpush-mono/mesher/client/components/dashboard/solana-programs-page.tsx`
- `../hyperpush-mono/mesher/client/components/dashboard/releases-page.tsx`
- `../hyperpush-mono/mesher/client/components/dashboard/bounties-page.tsx`
- `../hyperpush-mono/mesher/client/components/dashboard/treasury-page.tsx`
- `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs`
- `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`
