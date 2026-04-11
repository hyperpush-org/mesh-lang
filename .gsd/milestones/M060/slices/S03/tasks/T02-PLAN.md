---
estimated_steps: 4
estimated_files: 4
skills_used:
  - react-best-practices
  - playwright-best-practices
---

# T02: Make Settings general, API keys, and alert rules truthful without breaking the shell

**Slice:** S03 — Admin and ops surfaces live
**Milestone:** M060

## Description

This task lands the largest mixed live/mock surface: the existing Settings page must stop pretending the whole page is writable while still preserving the shell. Wire only the backend-backed subsections — retention/sample-rate plus storage in General, API key list/create/revoke, and alert-rule list/create/toggle/delete — through a settings-owned state layer, and make unsupported controls visibly present but non-live so the page is honest after S03.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Settings bootstrap reads (`GET /settings`, `GET /storage`, `GET /api-keys`, `GET /alert-rules`) | Keep the Settings shell mounted, mark the affected subsection failed, and show a destructive toast instead of falling back to fake global save state. | Keep unaffected tabs usable, expose timeout state per subsection, and avoid wedging the whole page in loading. | Reject the payload through typed parsing/adapters and leave unsupported sections explicitly mock-only rather than guessing. |
| Settings/API-key/alert-rule writes | Preserve the current visible values until a real refresh confirms the backend change, show a destructive toast on failure, and do not let the global Save button imply success for unsupported controls. | Clear pending state, show timeout diagnostics, and keep the current tab open. | Treat malformed success bodies as failures and do not surface partial optimistic updates. |

## Load Profile

- **Shared resources**: parallel settings bootstrap reads, API-key/rule mutation endpoints, clipboard reveal flow for newly created keys, and the shared toaster.
- **Per-operation cost**: four parallel bootstrap reads, one POST per supported write, and targeted refetches for the mutated subsection.
- **10x breakpoint**: repeated create/revoke/toggle flows will reveal stale subsection caches and fake-save drift first, so writes must stay subsection-scoped instead of page-global.

## Negative Tests

- **Malformed inputs**: empty key label, invalid retention/sample-rate values, malformed rule JSON, or unsupported tab attempts that should stay non-live.
- **Error paths**: settings update 400, API-key revoke 500, alert-rule toggle/delete failure, and a failed create-key response that must not leave a fake revealed secret on screen.
- **Boundary conditions**: empty rules/key lists, storage counts of zero, and one-time key reveal behavior after creating a new key.

## Steps

1. Add `mesher/client/components/dashboard/settings/settings-live-state.tsx` to own parallel bootstrap for General/API Keys/Alerts Rules, subsection-scoped pending/error state, and destructive-toast mutations inside the existing page shell.
2. Refactor `mesher/client/components/dashboard/settings/settings-page.tsx` so General shows live retention/sample-rate and storage metrics, API Keys uses real list/create/revoke flows with truthful one-time key reveal, and Alerts Rules uses real list/create/toggle/delete flows backed by the shared adapters.
3. Remove the dishonest page-wide fake save loop for unsupported controls; keep those tabs/controls visibly present, but mark them as mock-only/non-live instead of letting the header Save button imply backend writes that do not exist.
4. Expand `mesher/client/tests/e2e/admin-ops-live.spec.ts` to cover same-origin settings/api-key/rule happy paths plus at least one destructive-toast failure case in the Settings shell.

## Must-Haves

- [ ] General exposes live `retention_days`, `sample_rate`, and storage metrics through the existing Settings shell.
- [ ] API Keys lists real keys and supports truthful create/revoke flows without exposing a reusable secret after the initial reveal moment.
- [ ] Alert Rules lists real rules and supports create/toggle/delete through same-origin `/api/v1`, while unsupported notification-channel controls remain visibly non-live.
- [ ] Unsupported settings tabs and controls stay present and visually stable, but the page no longer performs a fake global save for surfaces the backend does not support.

## Verification

- `npm --prefix mesher/client run test:e2e:dev -- --grep "admin and ops live settings"`
- The Settings shell keeps unsupported areas present while proving live General/API key/alert-rule reads and writes through same-origin `/api/v1` calls.

## Observability Impact

- Signals added/changed: per-subsection `data-*` source/state markers for General, API Keys, and Alerts Rules plus last mutation/error indicators.
- How a future agent inspects this: switch through Settings tabs, inspect `settings-shell` state attributes, and replay `npm --prefix mesher/client run test:e2e:dev -- --grep "admin and ops live settings"`.
- Failure state exposed: fake-save drift is removed; failed subsection reads/writes now surface through visible toasts and explicit non-ready markers.

## Inputs

- `mesher/client/lib/mesher-api.ts` — admin/ops request helpers from T01 that now need to back the Settings page.
- `mesher/client/lib/admin-ops-live-adapter.ts` — shared adapters for alert-rule, API-key, and settings/storage payload normalization.
- `mesher/client/components/dashboard/settings/settings-page.tsx` — current monolithic Settings shell with fake global save drift.
- `mesher/client/tests/e2e/admin-ops-live.spec.ts` — the browser-proof file seeded in T01 that should grow into slice-level coverage.

## Expected Output

- `mesher/client/components/dashboard/settings/settings-live-state.tsx` — subsection-scoped settings bootstrap/mutation owner for General, API Keys, and Alerts Rules.
- `mesher/client/components/dashboard/settings/settings-page.tsx` — truthful mixed live/mock Settings UI that preserves the shell while backing supported controls with Mesher.
- `mesher/client/lib/admin-ops-live-adapter.ts` — expanded settings/API-key/rule adapters and source metadata used by the Settings shell.
- `mesher/client/tests/e2e/admin-ops-live.spec.ts` — browser proof covering same-origin settings/api-key/rule reads and writes plus a destructive-toast failure path.
