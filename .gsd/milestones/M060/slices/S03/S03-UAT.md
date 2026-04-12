# S03: Admin and ops surfaces live — UAT

**Milestone:** M060
**Written:** 2026-04-12T00:39:26.787Z

# UAT — S03 Admin and ops surfaces live

## Preconditions
- Postgres is reachable at the canonical local Mesher database URL.
- Run `bash mesher/scripts/seed-live-admin-ops.sh` from the repo root to establish deterministic admin/ops state.
- Start the dashboard in either verified mode (`npm --prefix mesher/client run test:e2e:dev -- --grep "admin and ops live"` / `...prod...`) or launch the app and backend locally with the same seeded database.
- Use the seeded default project/org context (`project: default`, `org: default`).

## Test Case 1 — Alerts route reads live data and supports real lifecycle actions
1. Open `/alerts`.
   - Expected: the Alerts shell reaches a ready state, shows live/mixed source badges, and lists at least the seeded alert plus any recent alert-proof rows.
2. Select a live alert row.
   - Expected: the detail panel shows a live banner/source note, exposes enabled `Acknowledge` and `Resolve` actions when applicable, and keeps unsupported silence controls visibly disabled.
3. Click `Acknowledge` on an active alert.
   - Expected: the request goes through same-origin `/api/v1`, the selected alert remains mounted, and its status updates to `acknowledged` after refresh.
4. Click `Resolve` on the same alert.
   - Expected: the request goes through same-origin `/api/v1`, the selected alert remains mounted, and its status updates to `resolved` after refresh.

## Test Case 2 — Alerts failure feedback stays visible and truthful
1. Force or simulate an alerts bootstrap failure (for example, via the Playwright failure-path proof or a temporary same-origin route failure).
   - Expected: the Alerts shell stays mounted, shows explicit fallback/failure state markers, and does not silently pretend the live read succeeded.
2. Force or simulate an acknowledge/resolve failure.
   - Expected: the selected alert remains visible, the last write failure is surfaced, and a destructive toast appears in the mounted notification region.

## Test Case 3 — Settings General and storage are live without fake page-wide save
1. Open `/settings` and stay on the General tab.
   - Expected: General shows live retention/sample-rate values and real storage metrics with subsection-level source/state markers.
2. Change retention days and sample rate to new valid values and save through the General subsection control.
   - Expected: values persist after refresh, the request uses same-origin `/api/v1/projects/default/settings`, and there is no page-wide fake Save for unrelated unsupported tabs.
3. Inspect still-unsupported tabs/controls.
   - Expected: they remain present and visually stable, but are explicitly marked non-live rather than implying a real backend write.

## Test Case 4 — API keys are truthful and only reveal the new secret at creation time
1. Open the API Keys tab.
   - Expected: the seeded API key row is listed with truthful active/revoked state and no full secret echoed in the list.
2. Create a new API key with a distinct label.
   - Expected: the create request uses same-origin `/api/v1/projects/default/api-keys`, the new key value is revealed once at creation time, and subsequent list views do not expose the full secret again.
3. Revoke the created key.
   - Expected: revoke uses same-origin `/api/v1/api-keys/:id/revoke`, the row refreshes to revoked state, and a forced revoke failure surfaces a destructive toast plus subsection error marker.

## Test Case 5 — Alert rules are live while notification-channel affordances remain honest
1. Open the Alerts tab inside Settings.
   - Expected: existing alert rules load from the backend and show truthful enabled/disabled state.
2. Create a rule with valid JSON condition/action payloads.
   - Expected: the rule appears after refresh with the provided name and real backend-backed state.
3. Toggle the rule enabled/disabled, then delete it.
   - Expected: both actions use same-origin `/api/v1` routes and the list refreshes authoritatively after each mutation.
4. Inspect notification-channel controls.
   - Expected: they remain visible but are explicitly marked mock-only/non-live.

## Test Case 6 — Team uses org-slug resolution and real member mutations
1. Open the Team tab.
   - Expected: it resolves `/api/v1/orgs/default/members`, lists the seeded owner/admin members, and shows subsection source/state markers.
2. Add the seeded candidate user with raw `user_id` `33333333-3333-4333-8333-333333333333`.
   - Expected: the member is added through the real backend route and appears after authoritative refresh.
3. Change that member's role.
   - Expected: the role update persists after refresh and uses the same-origin Team route.
4. Remove the same member.
   - Expected: the member disappears after refresh, owner-lock semantics remain intact, and there is no fake email-invite flow.
5. Enter malformed Team input.
   - Expected: local validation blocks the write, keeps the current Team panel truthful, and does not send a same-origin mutation request.

## Edge Cases
- Zero-storage and empty-live-list states must remain truthful; the shell must not invent fallback rows for API keys, alerts, or rules when the backend returns empty collections.
- Malformed live payloads must surface as contract/failure states rather than being guessed into shell-ready data.
- Failure to read or write any admin/ops subsection must surface through both explicit section markers and the mounted destructive toast region.

