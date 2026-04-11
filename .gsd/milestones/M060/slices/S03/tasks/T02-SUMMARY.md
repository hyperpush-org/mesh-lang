---
id: T02
parent: S03
milestone: M060
key_files:
  - mesher/client/lib/admin-ops-live-adapter.ts
  - mesher/client/components/dashboard/settings/settings-live-state.tsx
  - mesher/client/components/dashboard/settings/settings-page.tsx
  - mesher/client/tests/e2e/admin-ops-live.spec.ts
key_decisions:
  - D518 — Use subsection-scoped live state for General, API Keys, and Alert Rules, and remove the page-wide save affordance for unsupported tabs.
duration: 
verification_result: mixed
completed_at: 2026-04-11T23:53:42.940Z
blocker_discovered: false
---

# T02: Made Settings truthful for live general, API-key, and alert-rule flows with same-origin proof coverage.

**Made Settings truthful for live general, API-key, and alert-rule flows with same-origin proof coverage.**

## What Happened

I expanded `mesher/client/lib/admin-ops-live-adapter.ts` beyond Alerts so the Settings shell can render normalized live project settings/storage, API keys, and alert-rule data with source metadata instead of backend records directly. I then added `mesher/client/components/dashboard/settings/settings-live-state.tsx` as the subsection-scoped owner for parallel bootstrap reads, targeted refetch-after-write mutations, one-time API-key reveal handling, destructive toast failures, and explicit `data-*` diagnostics for General, API Keys, and Alert Rules.

With that state seam in place, I rewrote `mesher/client/components/dashboard/settings/settings-page.tsx` to remove the dishonest page-wide fake save loop and replace it with mixed live/mock rendering: General now truthfully edits live `retention_days` and `sample_rate` while showing real storage metrics; API Keys now lists, creates, reveals once, and revokes real keys; and Alerts now lists, creates, toggles, and deletes real alert rules while keeping notification-channel controls visibly mock-only. Unsupported tabs and controls remain present inside the existing shell, but they are explicitly marked non-live instead of implying backend writes that do not exist.

I also grew `mesher/client/tests/e2e/admin-ops-live.spec.ts` from the prior Alerts-only proof into a combined admin/ops live rail for Settings. The new coverage exercises same-origin General/API-key/alert-rule happy paths, malformed-input local validation, destructive-toast revoke failure handling, empty-list/zero-storage boundary cases, and same-origin request tracking in both dev and prod harnesses. During verification I found one combined-rail flake where the Settings bootstrap could overwrite the first invalid General input if the test typed before the section reached `data-state="ready"`; I fixed that by waiting for the panel’s ready marker before triggering the local-validation assertions.

## Verification

Verified the new mixed live Settings seam with both focused and slice-level Playwright rails. `npm --prefix mesher/client run build` passed after the Settings rewrite. `npm --prefix mesher/client run test:e2e:dev -- --grep "admin and ops live settings"` passed with four Settings-specific cases: same-origin General/API-key/alert-rule happy path, malformed-input local validation, destructive-toast revoke failure, and zero-storage/empty-list boundary behavior. `npm --prefix mesher/client run test:e2e:prod -- --grep "admin and ops live settings"` passed with the same four cases. I then ran the broader slice rail: `npm --prefix mesher/client run test:e2e:dev -- --grep "admin and ops live"` and `npm --prefix mesher/client run test:e2e:prod -- --grep "admin and ops live"` both passed with all nine Alerts+Settings proofs. The only slice-level command that still fails is `bash mesher/scripts/seed-live-admin-ops.sh`, because that helper does not exist yet in this slice.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix mesher/client run build` | 0 | ✅ pass | 38400ms |
| 2 | `npm --prefix mesher/client run test:e2e:dev -- --grep "admin and ops live settings"` | 0 | ✅ pass | 18900ms |
| 3 | `npm --prefix mesher/client run test:e2e:prod -- --grep "admin and ops live settings"` | 0 | ✅ pass | 18300ms |
| 4 | `bash mesher/scripts/seed-live-admin-ops.sh` | 127 | ❌ fail | 0ms |
| 5 | `npm --prefix mesher/client run test:e2e:dev -- --grep "admin and ops live"` | 0 | ✅ pass | 33700ms |
| 6 | `npm --prefix mesher/client run test:e2e:prod -- --grep "admin and ops live"` | 0 | ✅ pass | 36500ms |

## Deviations

Used the existing Playwright dev/prod harness as the authoritative real-browser verification path for the Settings shell instead of adding a separate ad-hoc browser-tools server spin-up step; the direct bg_shell attempt failed immediately because the ad-hoc shell session did not inherit `DATABASE_URL`, while the Playwright harness already exercised the real same-origin UI flows successfully in both environments. Also, the slice-level seed helper remains absent, so the broader verification status is partial only on that missing script.

## Known Issues

`mesher/scripts/seed-live-admin-ops.sh` is still missing, so that slice-level helper command remains a known red check for this intermediate task. The rest of the combined Alerts+Settings dev/prod rail is green. Unsupported Team/Integrations/Billing/Security/Notifications/Profile controls remain intentionally mock-only in the Settings shell until later slice work wires their backend-backed routes.

## Files Created/Modified

- `mesher/client/lib/admin-ops-live-adapter.ts`
- `mesher/client/components/dashboard/settings/settings-live-state.tsx`
- `mesher/client/components/dashboard/settings/settings-page.tsx`
- `mesher/client/tests/e2e/admin-ops-live.spec.ts`
