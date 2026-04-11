# S02: Core maintainer loop live — UAT

**Milestone:** M060
**Written:** 2026-04-11T22:50:33.395Z

# UAT — S02 Core maintainer loop live

## Preconditions

1. A local Mesher backend is available for the client's same-origin proxy target.
2. Run `bash mesher/scripts/seed-live-issue.sh` from `mesh-lang/`.
3. Start the dashboard in either dev or built-prod mode using the existing `mesher/client` harness.
4. Open the Issues route in the canonical `mesher/client` app.

## Test Case 1 — Seeded live Issues shell boots with truthful summary backing

1. Load the Issues route after running the seed helper.
   - **Expected:** The page boots without a mock-only empty state and selects seeded live issue data from the default project context.
2. Inspect the summary cards at the top of the Issues shell.
   - **Expected:** Cards expose truthful backing markers (`live`, `derived live`, or `fallback`) instead of one blanket “live” claim.
3. Select the seeded action issue.
   - **Expected:** The detail panel shows live-backed status and event data, and the detail shell remains mounted even where fallback-only fields are still required.

## Test Case 2 — Resolve moves an open issue through the live same-origin seam

1. With the seeded action issue selected in an open state, click `Resolve`.
   - **Expected:** Supported detail controls become busy/disabled while the write is in flight.
2. Wait for the mutation to finish.
   - **Expected:** The request goes through same-origin `/api/v1/issues/:id/resolve`.
3. Observe the issue list, detail status, and summary chrome after the write completes.
   - **Expected:** The issue status updates to resolved from refreshed provider state; the issue moves correctly under active filters; and the summary cards continue to show truthful source markers after refresh.

## Test Case 3 — Reopen returns the same issue to the open workflow

1. Starting from the resolved seeded action issue, click `Reopen`.
   - **Expected:** The control enters a busy/disabled state while the write is in flight.
2. Wait for the mutation to finish.
   - **Expected:** The request goes through same-origin `/api/v1/issues/:id/unresolve`.
3. Re-check the list and detail surfaces.
   - **Expected:** The issue returns to the open state from provider refetch, remains selected, and any active list filters reflect the reopened issue truthfully.

## Test Case 4 — Ignore archives the issue without pretending unsupported actions are live

1. With the seeded action issue selected, click `Ignore`.
   - **Expected:** The control enters a busy/disabled state; no unsupported action buttons masquerade as live alternatives.
2. Wait for the mutation to finish.
   - **Expected:** The request goes through same-origin `/api/v1/issues/:id/archive`.
3. Inspect the issue row and detail panel.
   - **Expected:** The issue shows the ignored/archived status mapping used by the shell, and summary/list/detail state refreshes from backend truth.

## Test Case 5 — Mutation failure stays visible and preserves operator context

1. Induce a backend mutation failure for the selected issue (for example, with the existing Playwright/mock harness used by the automated suite).
2. Trigger one of the supported actions.
   - **Expected:** A destructive toast becomes visible.
3. Inspect the selected issue after the failure.
   - **Expected:** The current issue remains selected; the prior visible state is preserved; and the shell exposes failure diagnostics instead of silently patching stale success.

## Test Case 6 — Post-mutation refresh failure is visible and does not silently trust `{"status":"ok"}`

1. Induce a failure on the overview refresh path after a nominally successful mutation response.
2. Trigger a supported action.
   - **Expected:** The shell does not silently accept the thin success body as final truth.
3. Observe UI feedback.
   - **Expected:** A destructive toast appears, issue state is not falsely presented as refreshed, and provider diagnostics show the refresh failure.

## Test Case 7 — Unsupported actions remain honestly unsupported

1. Inspect the issue detail shell for unsupported maintainer actions.
   - **Expected:** `assign`, `discard`, and `delete` are not presented as live wired actions in the S02 maintainer row.
2. If the retained proof rail for negative-path validation is available, trigger an unsupported action or unknown-issue path through it.
   - **Expected:** The provider rejects the request without issuing ad hoc browser fetches, and failure feedback is visible.

## Edge Cases

- Sparse live detail payloads still keep the fallback shell sections mounted instead of crashing or collapsing the detail view.
- Bootstrap or selected-detail read failures surface through the mounted toast region.
- Parallel dev/prod verification must use the deterministic two-fixture seed contract; do not reuse one mutable issue for both read and action proofs.
