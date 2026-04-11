# T01 wrap-up draft

## Current state
- Began implementing the shared admin/ops client seam in `mesher/client/lib/mesher-api.ts`.
- Added typed request/response models and same-origin helpers for alerts, alert rules, project settings/storage, org members, and API keys.
- No Alerts UI/state/test refactor is implemented yet.

## Files changed
- `mesher/client/lib/mesher-api.ts`

## What was added in `mesher-api.ts`
- `MesherAlertStatus`, `MesherProjectAlert`, `MesherAlertRule`, `MesherProjectSettings`, `MesherProjectStorage`, `MesherOrgMember`, `MesherApiKeyRecord`
- Parsers for alerts, alert rules, settings, storage, members, API keys, mutation status, created ids, and created API key responses
- Same-origin helpers:
  - `fetchDefaultProjectAlerts`
  - `acknowledgeAlert`
  - `resolveAlert`
  - `fetchDefaultProjectAlertRules`
  - `createDefaultProjectAlertRule`
  - `toggleAlertRule`
  - `deleteAlertRule`
  - `fetchDefaultProjectSettings`
  - `updateDefaultProjectSettings`
  - `fetchDefaultProjectStorage`
  - `fetchOrgMembers`
  - `addOrgMember`
  - `updateOrgMemberRole`
  - `removeOrgMember`
  - `fetchDefaultProjectApiKeys`
  - `createDefaultProjectApiKey`
  - `revokeApiKey`

## Verification state
- Tried LSP diagnostics on `mesher/client/lib/mesher-api.ts`, but no language server was available in this environment.
- No TypeScript/build/e2e verification was run after the edit.

## Resume plan
1. Re-open `mesher/client/lib/mesher-api.ts` and run a real TypeScript/build verification first.
2. Implement `mesher/client/lib/admin-ops-live-adapter.ts` for alert payload → shell mapping.
3. Implement `mesher/client/components/dashboard/alerts-live-state.tsx` using the same provider/toast/action-refresh pattern as `dashboard-issues-state.tsx`.
4. Refactor:
   - `mesher/client/components/dashboard/alerts-page.tsx`
   - `mesher/client/components/dashboard/alert-detail.tsx`
   - `mesher/client/components/dashboard/alert-stats.tsx`
   - alert types in `mesher/client/lib/mock-data.ts`
5. Add `mesher/client/tests/e2e/admin-ops-live.spec.ts` with:
   - same-origin request tracking
   - live alerts happy path
   - destructive toast failure path
   - malformed/empty/acknowledged/resolved coverage from the task plan
6. Run at least:
   - `npm --prefix mesher/client run test:e2e:dev -- --grep "admin and ops live alerts"`

## Important implementation notes
- Backend alert lifecycle is `active -> acknowledged -> resolved`; there is no live silence action.
- `GET /api/v1/projects/:project_id/alerts` returns a plain array, not an object wrapper.
- `condition_snapshot` must be treated as a real object; malformed payloads should fail parsing instead of being guessed.
- The backend only exposes alert list + acknowledge + resolve for fired alerts in this task scope.
