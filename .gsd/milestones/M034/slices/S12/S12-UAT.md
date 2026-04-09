# S12: Windows release-smoke remediation and final green closeout — UAT

**Milestone:** M034
**Written:** 2026-03-28T00:49:49.582Z

# S12: Windows release-smoke remediation and final green closeout — UAT

**Milestone:** M034
**Written:** 2026-03-27

## UAT Type

- UAT mode: mixed
- Why this mode is sufficient: This slice spans compiler/linker regressions, PowerShell verifier behavior, hosted GitHub Actions evidence, and reserved archive semantics. Honest acceptance is local proof plus fresh hosted blocker confirmation, not a fake green release claim.

## Preconditions

- Run from the repo root with Rust, Node, and PowerShell available.
- `.env` exists and contains the secrets needed by `bash scripts/verify-m034-s05.sh`.
- GitHub CLI read access is available for `snowdamiz/mesh-lang` so remote-evidence can inspect hosted runs.
- Expect the current truthful hosted state to still be red on `release.yml` run `23669185030` for `refs/tags/v0.1.0` / SHA `1e83ea930fdfd346b9e56659dc50d2f759ec5da2`.

## Smoke Test

Run `cargo test -p meshc --test e2e_m034_s12 -- --nocapture`.

**Expected:** 3 tests pass: native build tracing works, missing runtime lookup is reported before object emission, and a bad Windows LLVM prefix is reported before object emission.

## Test Cases

### 1. Local compiler trace and preflight regressions stay green

1. Run `cargo test -p meshc --test e2e_m034_s12 -- --nocapture`.
2. Run `cargo test -p mesh-codegen link -- --nocapture`.
3. **Expected:** both commands pass; the linker tests prove target-aware runtime discovery, and the e2e suite proves the new trace/preflight contract.

### 2. PowerShell staged-verifier regressions stay green

1. Run `pwsh -NoProfile -File scripts/tests/verify-m034-s03-last-exitcode.ps1`.
2. Run `pwsh -NoProfile -File scripts/tests/verify-m034-s03-installed-build.ps1`.
3. **Expected:** both commands pass; the installed-build regression accepts the hosted-log/trace fixtures and the verifier environment-shaping logic.

### 3. Workflow contract remains green after the Windows verifier changes

1. Run `bash scripts/verify-m034-s02-workflows.sh`.
2. **Expected:** the workflow contract passes unchanged; the Windows verifier changes did not drift the release/workflow semantics.

### 4. Hosted remote-evidence replay still records the live blocker truthfully

1. Run `VERIFY_M034_S05_STOP_AFTER=remote-evidence bash scripts/verify-m034-s05.sh`.
2. Read `.tmp/m034-s05/verify/status.txt`, `.tmp/m034-s05/verify/current-phase.txt`, `.tmp/m034-s05/verify/phase-report.txt`, and `.tmp/m034-s05/verify/remote-runs.json`.
3. **Expected:** the command exits 1; `status.txt` is `failed`; `current-phase.txt` is `remote-evidence`; earlier phases are `passed`; `remote-evidence` is `failed`; and `remote-runs.json` still points at hosted `release.yml` run `23669185030` on SHA `1e83ea930fdfd346b9e56659dc50d2f759ec5da2` with conclusion `failure`.

### 5. The reserved `first-green` label refuses red bundles

1. Run `bash scripts/verify-m034-s06-remote-evidence.sh first-green`.
2. Check whether `.tmp/m034-s06/evidence/first-green` exists.
3. **Expected:** the helper exits 1 with the reserved-label refusal message, and `.tmp/m034-s06/evidence/first-green` is still absent.

### 6. Full closeout replay still fails before downstream public phases

1. Run:
   ```bash
   set -euo pipefail
   test -f .env
   set -a
   source .env
   set +a
   bash scripts/verify-m034-s05.sh
   ```
2. Read `.tmp/m034-s05/verify/status.txt`, `.tmp/m034-s05/verify/current-phase.txt`, and `.tmp/m034-s05/verify/phase-report.txt`.
3. **Expected:** the replay exits 1; `status.txt` is `failed`; `current-phase.txt` is `remote-evidence`; `public-http\tpassed` is absent; and `s01-live-proof\tpassed` is absent because the verifier still stops at hosted rollout truth.

## Edge Cases

### Missing runtime or bad Windows LLVM prefix is reported before object emission

1. Run `cargo test -p meshc --test e2e_m034_s12 -- --nocapture`.
2. **Expected:** the dedicated tests prove both failure modes surface as preflight errors rather than as post-object or crash-style failures.

### Repeated `first-green` attempts do not create a reserved archive while the hosted lane is red

1. Run `bash scripts/verify-m034-s06-remote-evidence.sh first-green` more than once.
2. **Expected:** every attempt refuses the reserved label and `.tmp/m034-s06/evidence/first-green` remains absent.

## Failure Signals

- Any of the local Rust or PowerShell regressions fail.
- `VERIFY_M034_S05_STOP_AFTER=remote-evidence bash scripts/verify-m034-s05.sh` points at a different ref/SHA than the approved hosted run without an intentional reroll.
- `.tmp/m034-s06/evidence/first-green` appears while hosted `release.yml` is still red.
- The staged verifier stops emitting actionable trace/preflight context and falls back to an opaque Windows crash boundary.

## Requirements Proved By This UAT

- None — this UAT preserves blocker truth and milestone-evidence integrity rather than transitioning a new requirement status.

## Not Proven By This UAT

- A green hosted `release.yml` run on `v0.1.0`.
- A captured `first-green` archive.
- A successful full `bash scripts/verify-m034-s05.sh` replay through `public-http` and `s01-live-proof`.

## Notes for Tester

This slice is complete as a truthful blocker-capture and contract-hardening slice, not as a green milestone closeout. If the hosted Windows release-smoke path is repaired later, rerun the stop-after replay first, capture `first-green` exactly once, and only then rerun the full `.env`-backed closeout replay.
