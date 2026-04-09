# S06 Research: S01 acceptance artifact backfill

## Summary

S06 is a **narrow artifact-repair slice**, not new compiler or Mesher work.

Current repo state already has the real S01 proof bundle:

- `.gsd/milestones/M032/slices/S01/S01-SUMMARY.md` exists and is the accepted audit matrix.
- `bash scripts/verify-m032-s01.sh` passed during this scout pass (`verify-m032-s01: ok`).
- `cargo test -q -p meshc --test e2e m032_ -- --nocapture` passed and ran **10 tests**.
- `cargo test -q -p meshc --test e2e_stdlib m032_ -- --nocapture` passed and ran **2 tests**.
- `.tmp/m032-s01/` fixtures and `.tmp/m032-s01/verify/` logs are present.

The only live gap is the acceptance artifact itself:

- `.gsd/milestones/M032/slices/S01/S01-UAT.md` is still a recovery stub (`# S01: Recovery placeholder UAT`).
- `.gsd/milestones/M032/M032-VALIDATION.md` marks that placeholder as the sole reason M032 still sits at `needs-remediation`.

So S06 should replace **one placeholder file** with a real artifact-driven UAT derived from the existing matrix. If execution starts editing compiler tests, Mesher code, or `scripts/verify-m032-s01.sh`, it is probably doing the wrong slice.

## Recommendation

Use the same UAT structure as S03/S04/S05:

- `## UAT Type`
- `## Preconditions`
- `## Smoke Test`
- `## Test Cases`
- `## Edge Cases`
- `## Failure Signals`
- `## Requirements Proved By This UAT`
- `## Not Proven By This UAT`
- `## Notes for Tester`

Recommended content and stance:

1. **Smoke test:** `bash scripts/verify-m032-s01.sh`
   - This is the fastest single proof that the S01 matrix still replays end-to-end on the real repo.
2. **Matrix/artifact checks:** inspect `S01-SUMMARY.md` for the stale / real blocker / real keep / mixed-truth / next-slice handoff sections.
3. **Broad named proof filters:** keep the original slice-level proof commands because they still work today and still run non-zero tests:
   - `cargo test -q -p meshc --test e2e m032_ -- --nocapture`
   - `cargo test -q -p meshc --test e2e_stdlib m032_ -- --nocapture`
4. **Keep the route-closure warning explicit:** the acceptance artifact should still say this family requires a live-request proof, not just `meshc build`.
5. **Treat `xmod_identity` carefully:** S01 handed that family to S02 as the real blocker, but the repo is now post-S02. The backfilled UAT should prove that the family remains visible in the audit and current proof surfaces; it should **not** demand the old pre-fix failure to still exist.

Important writing rule for this slice: keep the new UAT truthful to **current repo state**. Do not write a fictional “historical snapshot” that requires pre-S02 failures to remain broken just because S01 happened earlier.

## Requirements Targeted

- **R035** — primary target. S06 closes the evidence gap for the truthful stale-vs-real limitation classification by replacing the placeholder with a real acceptance script.
- **Supports R011** — the acceptance artifact should keep S01 anchored to real Mesher friction and named replayable proof, not prose-only claims.
- **Supports R013** — the UAT should keep the `xmod_identity` handoff family visible as S01’s real blocker identification, without pretending S06 itself re-fixes it.

No requirement status changes are expected from S06 itself.

## Skills Discovered

No additional skill is directly relevant.

This slice depends on repo-local GSD artifact conventions plus already-existing Mesh proof surfaces, not a new external framework or service. I checked the installed skill list; none match “backfill one missing slice UAT artifact” closely enough to justify loading or searching for a new skill.

## Implementation Landscape

### A. The current gap is exactly one file

Live problem files:

- `.gsd/milestones/M032/slices/S01/S01-UAT.md` — placeholder that must be replaced
- `.gsd/milestones/M032/M032-VALIDATION.md` — names that placeholder as the blocker to milestone closure

Stable supporting files that should be **consumed, not rewritten**:

- `.gsd/milestones/M032/slices/S01/S01-SUMMARY.md`
- `scripts/verify-m032-s01.sh`
- `compiler/meshc/tests/e2e.rs`
- `compiler/meshc/tests/e2e_stdlib.rs`
- `.tmp/m032-s01/*`
- `.tmp/m032-s01/verify/*`

Planning consequence:

- this can be a **single execution task** centered on rewriting `S01-UAT.md`
- do not reopen the audit matrix, test fixtures, or replay script unless fresh verification actually fails

### B. The proof surface is already alive and current

Observed during this scout pass:

```bash
bash scripts/verify-m032-s01.sh
cargo test -q -p meshc --test e2e m032_ -- --nocapture
cargo test -q -p meshc --test e2e_stdlib m032_ -- --nocapture
```

Observed result:

- replay script passed (`verify-m032-s01: ok`)
- e2e filter passed: `running 10 tests`
- stdlib filter passed: `running 2 tests`

Important current-truth detail from `scripts/verify-m032-s01.sh`:

- supported paths now include `request_query`, `xmod_from_json`, `service_call_case`, `cast_if_else`, and `xmod_identity`
- retained-limit checks still include `nested_and`, `timer_service_cast`, and the live route bare/closure split
- Mesher `fmt --check` and `build` are already part of the replay

Planning consequence:

The new UAT should cite the **current** replay bundle. It should not try to preserve the old S01 assumption that `xmod_identity` must still fail.

### C. The later-slice UATs already show the right pattern

Use these as shape references only:

- `.gsd/milestones/M032/slices/S03/S03-UAT.md`
- `.gsd/milestones/M032/slices/S04/S04-UAT.md`
- `.gsd/milestones/M032/slices/S05/S05-UAT.md`

Shared pattern worth copying:

- artifact-driven UAT, not human/manual product exploration
- one smoke command first
- then a few narrow test/grep checks tied directly to the slice claim
- explicit edge cases and failure signals
- clear `Requirements Proved` / `Not Proven` split

For S06, keep it narrower than S03-S05. It does not need a new proof matrix; it only needs to turn the existing S01 matrix into a real acceptance artifact.

### D. Recommended S01-UAT content

Suggested acceptance structure:

1. **Smoke test**
   - `bash scripts/verify-m032-s01.sh`
   - Expected: exit 0 and `verify-m032-s01: ok`

2. **Matrix is present and still names the full S01 classification/handoff**
   - grep `S01-SUMMARY.md` for stale, real blocker, real keep, mixed-truth, and next-slice handoff landmarks
   - include `xmod_identity`, route closures, nested `&&`, timer cast, and the mixed-truth `from_json` family

3. **Broad compiler proof filters still replay non-empty**
   - `cargo test -q -p meshc --test e2e m032_ -- --nocapture`
   - `cargo test -q -p meshc --test e2e_stdlib m032_ -- --nocapture`
   - Expected: `running 10 tests` and `running 2 tests`, not `running 0 tests`

4. **Debug surface remains inspectable**
   - mention `.tmp/m032-s01/verify/*.log` as the first place to inspect replay drift
   - keep the route-closure note explicit: live request is authoritative

5. **Placeholder is gone**
   - negative grep for `Recovery placeholder UAT` / `Doctor created this placeholder`

Recommended edge cases:

- zero-test false positives on filtered Cargo commands do not count as proof
- route-closure classification must stay tied to runtime/live request, not build-only evidence
- `xmod_identity` is still part of the S01 story, but now as a named handoff family with current supported-path proof rather than an expected failure

## Verification Plan

Use these commands as the S06 executor’s acceptance loop:

```bash
bash -lc 'test -s .gsd/milestones/M032/slices/S01/S01-SUMMARY.md && test -s .gsd/milestones/M032/slices/S01/S01-UAT.md'
rg -n "Recovery placeholder UAT|Doctor created this placeholder" .gsd/milestones/M032/slices/S01/S01-UAT.md
rg -n "Stale Folklore|Real Blockers|Real Keep-Sites|Mixed-Truth Comments|Next-Slice Handoff|xmod_identity|route closures|nested `&&`|Timer.send_after" .gsd/milestones/M032/slices/S01/S01-SUMMARY.md
cargo test -q -p meshc --test e2e m032_ -- --nocapture
cargo test -q -p meshc --test e2e_stdlib m032_ -- --nocapture
bash scripts/verify-m032-s01.sh
```

Notes:

- after the rewrite, the placeholder grep should return **no matches**
- keep the broad `m032_` filters because they are current, non-empty, and map cleanly to the slice demo
- use `scripts/verify-m032-s01.sh` as the final integrated proof because it already bundles Mesher fmt/build and the live route checks

Optional milestone-closeout step if S06 also includes sealing M032:

```bash
rg -n "Recovery placeholder UAT|needs-remediation|S01-UAT" .gsd/milestones/M032/M032-VALIDATION.md
```

But do not pre-edit validation text unless the milestone closeout flow explicitly includes rerunning validation.

## Current Baseline Observed During Research

- `test -s .gsd/milestones/M032/slices/S01/S01-SUMMARY.md` → present
- `rg -n "Recovery placeholder UAT|Doctor created this placeholder" .gsd/milestones/M032/slices/S01/S01-UAT.md` → matches still present
- `bash scripts/verify-m032-s01.sh` → passed (`verify-m032-s01: ok`)
- `cargo test -q -p meshc --test e2e m032_ -- --nocapture` → `running 10 tests`, all passed
- `cargo test -q -p meshc --test e2e_stdlib m032_ -- --nocapture` → `running 2 tests`, all passed

## Planner Notes

- This should plan as **one small execution task**, not a multi-task slice, unless the planner also wants milestone revalidation in the same pass.
- The highest-risk mistake is writing S01-UAT as if the repo still lives before S02. Don’t do that. Use the current replay surfaces, while keeping the S01 summary’s handoff story intact.
- If verification fails, inspect `.tmp/m032-s01/verify/*.log` first. If those logs are clean, then the failure is probably in the new UAT wording/grep logic rather than in compiler or Mesher behavior.
- Avoid touching `S01-SUMMARY.md`. Validation already accepts it as the authoritative audit matrix; S06 exists because the UAT artifact is missing, not because the summary is wrong.
