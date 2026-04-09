---
id: T06
parent: S04
milestone: M047
provides: []
requires: []
affects: []
key_files: ["scripts/verify-m047-s04.sh", "scripts/verify-m045-s04.sh", "scripts/verify-m045-s05.sh", "scripts/verify-m046-s04.sh", "scripts/verify-m046-s05.sh", "scripts/verify-m046-s06.sh", "compiler/meshc/tests/e2e_m047_s04.rs", "compiler/meshc/tests/e2e_m045_s04.rs", "compiler/meshc/tests/e2e_m045_s05.rs", "compiler/meshc/tests/e2e_m046_s04.rs", "compiler/meshc/tests/e2e_m046_s05.rs", "compiler/meshc/tests/e2e_m046_s06.rs", "README.md", "website/docs/docs/distributed-proof/index.md", "website/docs/docs/distributed/index.md", "website/docs/docs/tooling/index.md", "website/docs/docs/getting-started/clustered-example/index.md", "tiny-cluster/README.md", "cluster-proof/README.md"]
key_decisions: ["Made scripts/verify-m047-s04.sh the single authoritative clustered cutover rail and demoted the M045/M046 verifier names to replayable compatibility aliases that delegate to it."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran the task’s required verification surface successfully: cargo test -p meshc --test e2e_m047_s04 -- --nocapture; cargo test -p meshc --test e2e_m045_s04 -- --nocapture; cargo test -p meshc --test e2e_m045_s05 -- --nocapture; bash scripts/verify-m047-s04.sh. The assembled verifier completed and retained .tmp/m047-s04/verify/{status.txt,current-phase.txt,phase-report.txt,full-contract.log,latest-proof-bundle.txt}."
completed_at: 2026-04-01T10:37:13.876Z
blocker_discovered: false
---

# T06: Added the M047 cutover verifier, repointed M045/M046 wrapper scripts to it, and updated docs/tests so the source-first `@cluster` story has one authoritative rail.

> Added the M047 cutover verifier, repointed M045/M046 wrapper scripts to it, and updated docs/tests so the source-first `@cluster` story has one authoritative rail.

## What Happened
---
id: T06
parent: S04
milestone: M047
key_files:
  - scripts/verify-m047-s04.sh
  - scripts/verify-m045-s04.sh
  - scripts/verify-m045-s05.sh
  - scripts/verify-m046-s04.sh
  - scripts/verify-m046-s05.sh
  - scripts/verify-m046-s06.sh
  - compiler/meshc/tests/e2e_m047_s04.rs
  - compiler/meshc/tests/e2e_m045_s04.rs
  - compiler/meshc/tests/e2e_m045_s05.rs
  - compiler/meshc/tests/e2e_m046_s04.rs
  - compiler/meshc/tests/e2e_m046_s05.rs
  - compiler/meshc/tests/e2e_m046_s06.rs
  - README.md
  - website/docs/docs/distributed-proof/index.md
  - website/docs/docs/distributed/index.md
  - website/docs/docs/tooling/index.md
  - website/docs/docs/getting-started/clustered-example/index.md
  - tiny-cluster/README.md
  - cluster-proof/README.md
key_decisions:
  - Made scripts/verify-m047-s04.sh the single authoritative clustered cutover rail and demoted the M045/M046 verifier names to replayable compatibility aliases that delegate to it.
duration: ""
verification_result: passed
completed_at: 2026-04-01T10:37:13.877Z
blocker_discovered: false
---

# T06: Added the M047 cutover verifier, repointed M045/M046 wrapper scripts to it, and updated docs/tests so the source-first `@cluster` story has one authoritative rail.

**Added the M047 cutover verifier, repointed M045/M046 wrapper scripts to it, and updated docs/tests so the source-first `@cluster` story has one authoritative rail.**

## What Happened

Added scripts/verify-m047-s04.sh as the authoritative clustered cutover rail for S04. It replays the parser/pkg/compiler cutover proofs, scaffold proofs, package smoke proofs, docs build, and the new e2e_m047_s04 Rust contract target, then retains one coherent .tmp/m047-s04/verify/ bundle with explicit phase markers and a retained bundle pointer. Added compiler/meshc/tests/e2e_m047_s04.rs to snapshot the new verifier graph and public verifier story under .tmp/m047-s04/*. Rewrote scripts/verify-m045-s04.sh, scripts/verify-m045-s05.sh, scripts/verify-m046-s04.sh, scripts/verify-m046-s05.sh, and scripts/verify-m046-s06.sh into replayable compatibility aliases that delegate to scripts/verify-m047-s04.sh and retain the delegated verify directory. Updated the M045/M046 source-based wrapper contract tests and repointed README/VitePress/package runbooks to describe scripts/verify-m047-s04.sh as the authoritative cutover rail and the M046/M045 names as compatibility aliases.

## Verification

Ran the task’s required verification surface successfully: cargo test -p meshc --test e2e_m047_s04 -- --nocapture; cargo test -p meshc --test e2e_m045_s04 -- --nocapture; cargo test -p meshc --test e2e_m045_s05 -- --nocapture; bash scripts/verify-m047-s04.sh. The assembled verifier completed and retained .tmp/m047-s04/verify/{status.txt,current-phase.txt,phase-report.txt,full-contract.log,latest-proof-bundle.txt}.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m047_s04 -- --nocapture` | 0 | ✅ pass | 808ms |
| 2 | `cargo test -p meshc --test e2e_m045_s04 -- --nocapture` | 0 | ✅ pass | 2371ms |
| 3 | `cargo test -p meshc --test e2e_m045_s05 -- --nocapture` | 0 | ✅ pass | 1620ms |
| 4 | `bash scripts/verify-m047-s04.sh` | 0 | ✅ pass | 58688ms |


## Deviations

Updated the clustered docs/runbook surfaces and the M046 source-contract tests in the same task even though they were not listed in the expected-output file list. Leaving them untouched would have kept the new M047 verifier graph internally inconsistent and left the published verifier story pointing at the old M046 authority.

## Known Issues

None.

## Files Created/Modified

- `scripts/verify-m047-s04.sh`
- `scripts/verify-m045-s04.sh`
- `scripts/verify-m045-s05.sh`
- `scripts/verify-m046-s04.sh`
- `scripts/verify-m046-s05.sh`
- `scripts/verify-m046-s06.sh`
- `compiler/meshc/tests/e2e_m047_s04.rs`
- `compiler/meshc/tests/e2e_m045_s04.rs`
- `compiler/meshc/tests/e2e_m045_s05.rs`
- `compiler/meshc/tests/e2e_m046_s04.rs`
- `compiler/meshc/tests/e2e_m046_s05.rs`
- `compiler/meshc/tests/e2e_m046_s06.rs`
- `README.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `tiny-cluster/README.md`
- `cluster-proof/README.md`


## Deviations
Updated the clustered docs/runbook surfaces and the M046 source-contract tests in the same task even though they were not listed in the expected-output file list. Leaving them untouched would have kept the new M047 verifier graph internally inconsistent and left the published verifier story pointing at the old M046 authority.

## Known Issues
None.
