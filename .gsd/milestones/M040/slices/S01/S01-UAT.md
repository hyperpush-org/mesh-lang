# S01: Keyed Submit/Status Contract on the Existing Proof Rail — UAT

**Milestone:** M040
**Written:** 2026-03-28T18:20:47.858Z

# S01 UAT — Keyed Submit/Status Contract on the Existing Proof Rail

## Preconditions
- Run from the repository root.
- `cluster-proof` builds successfully: `cargo run -q -p meshc -- build cluster-proof`
- No other local process is bound to the verifier-selected HTTP port.
- Use the generated verifier artifacts in `.tmp/m040-s01/verify/` as the canonical evidence bundle.

## Test Case 1 — Initial keyed submit returns truthful pending status
1. Start the app in standalone mode with the verifier: `bash scripts/verify-m040-s01.sh`
2. Inspect `.tmp/m040-s01/verify/01-create.json`.
3. Confirm the response shows:
   - `request_key == "m040-s01-verify-key"`
   - `attempt_id` present and stable
   - `phase == "submitted"`
   - `result == "pending"`
   - `ingress_node == "standalone@local"`
   - `owner_node == "standalone@local"`
   - `replica_node == ""`
   - `replica_status == "unassigned"`
   - `routed_remotely == false`
   - `fell_back_locally == true`
4. Expected outcome: the keyed contract reports one logical request with truthful standalone owner/replica placeholder data and no empty ingress/owner fields.

## Test Case 2 — Keyed status converges to completion with the same attempt identity
1. After Test Case 1, inspect `.tmp/m040-s01/verify/02-completed.json`.
2. Confirm the completed status keeps the same `attempt_id` from `01-create.json`.
3. Confirm the response shows:
   - `phase == "completed"`
   - `result == "succeeded"`
   - `execution_node == "standalone@local"`
   - `owner_node == "standalone@local"`
4. Expected outcome: the keyed status endpoint converges from submitted to completed without changing logical request identity or inventing a second attempt.

## Test Case 3 — Same-key same-payload retry is idempotent
1. Inspect `.tmp/m040-s01/verify/03-duplicate.json`.
2. Confirm the response returned HTTP 200 in the verifier flow.
3. Confirm:
   - `attempt_id` matches both `01-create.json` and `02-completed.json`
   - `phase == "completed"`
   - `result == "succeeded"`
   - `conflict_reason == ""`
4. Expected outcome: retrying the same request key with the same payload reuses the original completed attempt instead of creating duplicate completion state.

## Test Case 4 — Same-key conflicting retry fails closed
1. Inspect `.tmp/m040-s01/verify/04-conflict.json`.
2. Confirm the verifier observed HTTP 409 for the conflicting retry.
3. Confirm:
   - `attempt_id` still matches the original attempt
   - `phase == "completed"`
   - `result == "succeeded"`
   - `conflict_reason == "request_key_conflict"`
   - `ok == false`
4. Expected outcome: reusing the same request key for a different payload is rejected without mutating the original completion record.

## Test Case 5 — Missing keyed status fails closed
1. Inspect `.tmp/m040-s01/verify/05-missing.json`.
2. Confirm the verifier observed HTTP 404.
3. Confirm:
   - `request_key == "missing-key"`
   - `phase == "missing"`
   - `result == "unknown"`
   - `error == "request_key_not_found"`
   - `ok == false`
4. Expected outcome: missing status lookups do not fabricate a pending/completed record and instead return a truthful not-found payload.

## Edge Cases
- **Blank request key:** `validate_request_key("   ")` must reject with `request_key is required` (covered by `cluster-proof/tests/work.test.mpl`).
- **Oversized request key:** a 129-character key must reject with `request_key must be 1..128 characters` (covered by `cluster-proof/tests/work.test.mpl`).
- **Blank payload:** `parse_submit_body` must reject keyed JSON whose payload is only whitespace with `payload is required` (covered by `cluster-proof/tests/work.test.mpl`).
- **Operational regression check:** if standalone `POST /work` ever returns `invalid_target_selection`, treat that as a regression in the `standalone@local` identity fallback rather than as acceptable behavior.

