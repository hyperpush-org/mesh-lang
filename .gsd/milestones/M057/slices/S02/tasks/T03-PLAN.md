---
estimated_steps: 3
estimated_files: 4
skills_used:
  - gh
  - test
---

# T03: Verify the live repo state and publish the S03 handoff artifacts

Close the loop with read-only verification and retained diagnostics. This task proves the live repo issue sets now match the S01 truth buckets, publishes a human-readable handoff for S03, and leaves a retained replay surface so future agents can localize any reconciliation drift quickly.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `gh issue view` / `gh issue list` read-only checks | Fail the verification and record which handle/query could not be inspected. | Abort the verifier, keep existing result artifacts, and expose the timed-out phase in `.tmp/m057-s02/verify/`. | Reject responses missing canonical URL/state/title fields or mismatching the expected repo after transfer/create. |
| `repo-mutation-results.json` | Refuse to render the handoff markdown or wrapper success marker until the results artifact exists and matches the expected schema. | N/A — local file validation should fail immediately. | Reject missing old→new mapping for transferred/created issues, blank final states, or incomplete bucket coverage. |

## Load Profile

- **Shared resources**: GitHub read-only issue APIs, retained verification logs under `.tmp/m057-s02/verify/`, and the S02 markdown handoff artifact.
- **Per-operation cost**: spot-check issue views for the touched buckets plus two repo-wide list queries to confirm final totals.
- **10x breakpoint**: read-only API latency and retained-log size would grow first; the verifier should use grouped checks instead of one-off manual inspection.

## Negative Tests

- **Malformed inputs**: results rows missing new canonical URL for `hyperpush#8` transfer or missing created issue URL for `/pitch`.
- **Error paths**: repo totals still match the pre-S02 snapshot, a rewritten issue closes unexpectedly, or `hyperpush#8` still appears in the product repo after transfer.
- **Boundary conditions**: `mesh-lang` total becomes 17, `hyperpush` total stays 52 after transfer+create, and the combined total becomes 69.

## Steps

1. Add a results contract test that validates the live-state artifact schema, required mapping fields, and the expected mutation bucket rollups.
2. Build a retained verifier that replays the read-only GH checks, records phase/status files under `.tmp/m057-s02/verify/`, and fails if counts, states, or canonical repo destinations drift.
3. Publish a compact handoff markdown file for S03 with the new canonical issue URLs/numbers, the bucket-by-bucket repo outcomes, and any known follow-up limits such as board drift still pending in S03.

## Must-Haves

- [ ] Verification confirms 10 shipped `mesh-lang` issues are closed, 21 `rewrite_scope` rows remain open with updated text, 7 mock-backed follow-through rows remain open with truthful wording, and `#54/#55/#56` no longer present stale public `hyperpush-mono` ownership in their rewritten text.
- [ ] Verification confirms the transferred `hyperpush#8` destination issue and the newly created `/pitch` issue by canonical URL/number.
- [ ] The retained verifier leaves enough phase/detail output that a future agent can tell whether a failure is a planner bug, an applicator bug, or live GitHub drift.

## Inputs

- `scripts/lib/m057_repo_mutation_apply.py` — live mutation results producer from T02.
- `.gsd/milestones/M057/slices/S02/repo-mutation-results.json` — per-operation outcomes and canonical mapping fields.
- `.gsd/milestones/M057/slices/S02/repo-mutation-plan.json` — expected touched set and bucket breakdown for read-only verification.
- `.gsd/milestones/M057/slices/S01/reconciliation-ledger.json` — original action truth used to compare final live state against S01 expectations.

## Expected Output

- `scripts/tests/verify-m057-s02-results.test.mjs` — results artifact contract test and live bucket expectations.
- `scripts/verify-m057-s02.sh` — retained read-only verifier with phase/status logs under `.tmp/m057-s02/verify/`.
- `.gsd/milestones/M057/slices/S02/repo-mutation-results.md` — human-readable S03 handoff with new canonical issue mappings and verified repo-state outcomes.

## Verification

- `node --test scripts/tests/verify-m057-s02-results.test.mjs`
- `bash scripts/verify-m057-s02.sh`

## Observability Impact

- Signals added/changed: retained verification phase markers, repo-total assertions, issue-bucket outcome summaries, and explicit transferred/created issue mapping fields in the human-readable handoff.
- How a future agent inspects this: run `bash scripts/verify-m057-s02.sh`, inspect `.tmp/m057-s02/verify/phase-report.txt`, or open `repo-mutation-results.md` for the S03-facing summary.
- Failure state exposed: failed phase name, last checked query or issue handle, final count mismatch, and missing canonical mapping fields for identity-changing operations.
