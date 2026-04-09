# S10: Hosted verification blocker remediation — UAT

**Milestone:** M034
**Written:** 2026-03-27T21:33:13.230Z

# S10: Hosted verification blocker remediation — UAT

**Milestone:** M034
**Written:** 2026-03-28

## UAT Type

- UAT mode: mixed
- Why this mode is sufficient: S10 changed both code paths and hosted-proof attribution. The honest acceptance surface is a mix of focused local regressions plus refreshed hosted evidence on the current rollout target.

## Preconditions

- Run from the repo root.
- Local Docker Postgres for registry tests is reachable at `postgres://postgres:postgres@127.0.0.1:55433/postgres`.
- `gh` is authenticated for read-only workflow inspection.
- `.env` exists so `scripts/verify-m034-s05.sh` can load the secrets it expects for remote-evidence replay.

## Smoke Test

Run:

1. `DATABASE_URL='postgres://postgres:postgres@127.0.0.1:55433/postgres' cargo test --manifest-path registry/Cargo.toml latest -- --nocapture`
2. **Expected:** all `latest`-focused registry tests pass, including the out-of-order publish regression and the metadata/search malformed-latest failure tests.

## Test Cases

### 1. Registry latest-version stays monotonic under overlapping publishes

1. Run `DATABASE_URL='postgres://postgres:postgres@127.0.0.1:55433/postgres' cargo test --manifest-path registry/Cargo.toml latest -- --nocapture`.
2. Confirm the output includes:
   - `db::packages::tests::latest_version_stays_monotonic_for_out_of_order_publishes ... ok`
   - `routes::metadata::tests::package_document_uses_highest_latest_version ... ok`
   - `routes::search::tests::package_list_and_search_share_monotonic_latest_version ... ok`
3. **Expected:** package-level latest never regresses to an older version even when version inserts complete out of order.

### 2. Metadata/search fail closed when latest-version joins drift

1. Reuse the same registry test run from Test Case 1.
2. Confirm the output includes:
   - `routes::metadata::tests::package_document_fails_when_latest_version_row_is_missing ... ok`
   - `routes::search::tests::package_list_and_search_fail_when_latest_join_is_missing ... ok`
3. **Expected:** the handlers reject malformed latest-version state instead of fabricating empty or stale package metadata.

### 3. Windows MSVC runtime discovery is target-aware and verifier logs stay actionable

1. Run `cargo test -p mesh-codegen link -- --nocapture`.
2. Confirm the output includes the Windows-runtime-path regression tests:
   - `mesh_rt_candidates_should_use_windows_runtime_name_inside_target_subdir ... ok`
   - `find_mesh_rt_in_should_report_target_specific_runtime_name_when_missing ... ok`
3. Run `pwsh -NoProfile -File scripts/tests/verify-m034-s03-last-exitcode.ps1`.
4. **Expected:** both commands pass, proving that Windows MSVC now searches for `mesh_rt.lib` / `clang(.exe)` explicitly and that staged PowerShell logs retain command text, exit code, and artifact paths.

### 4. Release workflow contract still points at the real staged smoke steps

1. Run `bash scripts/verify-m034-s02-workflows.sh`.
2. **Expected:** the workflow-contract verifier passes without exemptions, confirming `release.yml` still runs the real staged installer/build smoke path instead of a weakened placeholder.

### 5. Hosted blocker evidence refreshes to the current rollout target and isolates only the release lane

1. Run:
   `set -a; source .env; set +a; VERIFY_M034_S05_STOP_AFTER=remote-evidence bash scripts/verify-m034-s05.sh`
2. Expect the command itself to exit non-zero because `release.yml` is still red.
3. Open `.tmp/m034-s05/verify/remote-runs.json` and confirm:
   - `authoritative-verification.yml.status == "ok"`
   - `authoritative-verification.yml.observedHeadSha == .tmp/m034-s09/rollout/target-sha.txt`
   - `release.yml.status == "failed"`
   - `release.yml.freshnessStatus == "ok"`
   - `release.yml.observedHeadSha == .tmp/m034-s09/rollout/target-sha.txt`
4. Run `python3 .tmp/m034-s09/rollout/monitor_workflows.py`.
5. Expect that command to exit non-zero for the same reason.
6. Open `.tmp/m034-s09/rollout/workflow-status.json` and confirm:
   - `authoritative-verification.yml.conclusion == "success"`
   - `release.yml.conclusion == "failure"`
   - both workflows point at the same rollout SHA from `.tmp/m034-s09/rollout/target-sha.txt`.
7. **Expected:** the hosted state is fresh and attributable, with `release.yml` as the only failing lane on the current target.

## Edge Cases

### Missing latest-version row for the recorded package latest

1. Use the registry regression suite from Test Case 1.
2. **Expected:** the metadata/search malformed-latest tests pass by returning explicit handler failure instead of an empty document or downgraded latest version.

### Workflow freshness is correct but health is not

1. Inspect `.tmp/m034-s05/verify/remote-runs.json` after Test Case 5.
2. **Expected:** `release.yml` shows `freshnessStatus: ok` and a matching `observedHeadSha`, while still remaining `status: failed`. This proves the blocker is a real hosted failure, not stale rollout evidence.

## Failure Signals

- `cargo test --manifest-path registry/Cargo.toml latest -- --nocapture` reports any `latest_*` test failure.
- `cargo test -p mesh-codegen link -- --nocapture` stops finding `mesh_rt.lib` / `clang(.exe)` on Windows-target cases or regresses Unix runtime-path selection.
- `pwsh -NoProfile -File scripts/tests/verify-m034-s03-last-exitcode.ps1` no longer sees exit-code/stdout/stderr metadata in the PowerShell command logs.
- `.tmp/m034-s05/verify/remote-runs.json` shows `authoritative-verification.yml` red again, or `release.yml` points at a different SHA than `.tmp/m034-s09/rollout/target-sha.txt`.
- `.tmp/m034-s09/rollout/workflow-status.json` shows more than one failing workflow, or shows stale `headSha` values.

## Requirements Proved By This UAT

- R007 — The real package workflow keeps a truthful package-level latest version across overlapping publishes, and the hosted authoritative proof lane now reflects that repaired registry behavior.

## Not Proven By This UAT

- This UAT does not prove that `release.yml` is green; it proves that the remaining blocker is isolated to that lane on the current rollout SHA.
- This UAT does not prove that the workflow-only synthetic commit contains the full local S10 code surface on the release tag lane.
- This UAT does not produce the first-green hosted archive for `.tmp/m034-s06/evidence/first-green/`.

## Notes for Tester

Treat the non-zero exits from the stop-after `remote-evidence` replay and `monitor_workflows.py` as expected only when the JSON outputs show the exact blocker shape above. A different failure mode means the hosted state widened again and S11 should not proceed.
