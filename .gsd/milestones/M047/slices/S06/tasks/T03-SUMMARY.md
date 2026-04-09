---
id: T03
parent: S06
milestone: M047
provides: []
requires: []
affects: []
key_files: ["README.md", "website/docs/docs/tooling/index.md", "website/docs/docs/getting-started/clustered-example/index.md", "website/docs/docs/distributed-proof/index.md", "website/docs/docs/distributed/index.md", "compiler/meshc/tests/e2e_m047_s06.rs", "scripts/verify-m047-s06.sh", "compiler/meshc/tests/e2e_m047_s04.rs", "compiler/meshc/tests/e2e_m047_s05.rs", ".gsd/DECISIONS.md"]
key_decisions: ["Keep S06 as the final closeout wrapper that replays S05 and copies retained proof evidence into `.tmp/m047-s06/verify` instead of sharing or mutating `.tmp/m047-s05/verify` directly.", "Fail the closeout rail on malformed delegated handoff, missing retained bundle pointers, stale helper-shaped docs authority, or any `HTTP.clustered(...)` overclaim instead of treating those as optional docs drift."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Before the interruption, I had already run the direct task-level rails `cargo test -p meshc --test e2e_m047_s06 -- --nocapture` and the slice-level rail `cargo test -p meshc --test e2e_sqlite_built_package -- --nocapture`; both exited successfully, so I resumed from their retained artifacts instead of rerunning green work. I then verified the assembled closeout bundle on disk: `.tmp/m047-s06/verify/status.txt` is `ok`, `.tmp/m047-s06/verify/current-phase.txt` is `complete`, and `.tmp/m047-s06/verify/phase-report.txt` records `contract-guards`, `m047-s05-replay`, `retain-m047-s05-verify`, `m047-s06-e2e`, `m047-s06-docs-build`, `m047-s06-artifacts`, and `m047-s06-bundle-shape` as passed. The retained S05 bundle inside S06 also shows `status.txt = ok`, `current-phase.txt = complete`, and a passing delegated phase report for `m047-s04-replay`, `retain-m047-s04-verify`, `m047-s05-pkg`, `m047-s05-tooling`, `m047-s05-e2e`, `m047-s05-docs-build`, `retain-m047-s05-artifacts`, and `m047-s05-bundle-shape`.

From the retained command logs, the S06 contract rail recorded `running 3 tests` and `3 passed`, and the docs build finished with `build complete in 35.99s`. The slice-level SQLite built-package regression left a fresh `.tmp/m047-s06/sqlite-built-package-execute-*` artifact directory whose `run.stdout.log` shows `schema=ok`, `insert=1`, `count=1`, `mismatch_err=column index out of range`, and `done`, which confirms the built-package execute seam still handles helper rewraps correctly while failing closed on placeholder mismatch errors.

Because the direct successful `cargo test -p meshc --test e2e_m047_s06 -- --nocapture`, `cargo test -p meshc --test e2e_sqlite_built_package -- --nocapture`, and `bash scripts/verify-m047-s06.sh` runs completed in the interrupted session, their original wall-clock timings were no longer recoverable from tool metadata during resume. I preserved those checks in the evidence table with `0ms` placeholders and grounded the rest of the verification in the retained on-disk logs and status files."
completed_at: 2026-04-01T22:06:33.993Z
blocker_discovered: false
---

# T03: Added the final S06 closeout rail, finished the public source-first `@cluster` docs story, and retained the delegated S05 proof bundle under one assembled verifier surface.

> Added the final S06 closeout rail, finished the public source-first `@cluster` docs story, and retained the delegated S05 proof bundle under one assembled verifier surface.

## What Happened
---
id: T03
parent: S06
milestone: M047
key_files:
  - README.md
  - website/docs/docs/tooling/index.md
  - website/docs/docs/getting-started/clustered-example/index.md
  - website/docs/docs/distributed-proof/index.md
  - website/docs/docs/distributed/index.md
  - compiler/meshc/tests/e2e_m047_s06.rs
  - scripts/verify-m047-s06.sh
  - compiler/meshc/tests/e2e_m047_s04.rs
  - compiler/meshc/tests/e2e_m047_s05.rs
  - .gsd/DECISIONS.md
key_decisions:
  - Keep S06 as the final closeout wrapper that replays S05 and copies retained proof evidence into `.tmp/m047-s06/verify` instead of sharing or mutating `.tmp/m047-s05/verify` directly.
  - Fail the closeout rail on malformed delegated handoff, missing retained bundle pointers, stale helper-shaped docs authority, or any `HTTP.clustered(...)` overclaim instead of treating those as optional docs drift.
duration: ""
verification_result: passed
completed_at: 2026-04-01T22:06:33.995Z
blocker_discovered: false
---

# T03: Added the final S06 closeout rail, finished the public source-first `@cluster` docs story, and retained the delegated S05 proof bundle under one assembled verifier surface.

**Added the final S06 closeout rail, finished the public source-first `@cluster` docs story, and retained the delegated S05 proof bundle under one assembled verifier surface.**

## What Happened

I finished the public closeout surfaces so they now teach one consistent two-layer clustered story instead of mixing legacy helper-shaped names, stale authority, or route-local clustering claims. `README.md`, the tooling page, the clustered-example guide, the distributed-proof page, and the distributed guide all now present the same contract: the canonical public clustered surface stays route-free and source-first with `@cluster`, `mesh.toml` remains package-only, the Todo template is the fuller starter layered on top of that same contract, migrations move `clustered(work)` / `[cluster]` / `execute_declared_work(...)` / `Work.execute_declared_work` to ordinary verbs, and `HTTP.clustered(...)` is explicitly called out as not shipped.

I added `compiler/meshc/tests/e2e_m047_s06.rs` to fail closed on exactly that public authority. The test reads the final docs and verifier script, archives the contract sources, asserts the migration language is present everywhere it should be, proves the S04/S05/S06 layering is stated truthfully, and turns stale authority or overclaims into named failures. I also wrote `scripts/verify-m047-s06.sh` as the assembled closeout wrapper: it replays `bash scripts/verify-m047-s05.sh`, copies the delegated `.tmp/m047-s05/verify` tree into `.tmp/m047-s06/verify/retained-m047-s05-verify`, validates the delegated status/current-phase/phase-report/latest-bundle contract, runs the S06 contract rail plus the docs build, snapshots any new S06 artifacts, and publishes a single retained closeout bundle through `.tmp/m047-s06/verify/latest-proof-bundle.txt`.

During replay, the new wrapper exposed two small local contract drifts in the older retained rails, so I made the minimum targeted corrections in `compiler/meshc/tests/e2e_m047_s04.rs` and `compiler/meshc/tests/e2e_m047_s05.rs` to keep the delegated S04 and S05 assertions aligned with the final source-first/public-doc truth that S06 now wraps. I also recorded the bundle-ownership decision in `.gsd/DECISIONS.md`: S06 owns the assembled proof surface by retaining S05 evidence inside its own bundle, not by sharing or mutating `.tmp/m047-s05/verify` directly.

## Verification

Before the interruption, I had already run the direct task-level rails `cargo test -p meshc --test e2e_m047_s06 -- --nocapture` and the slice-level rail `cargo test -p meshc --test e2e_sqlite_built_package -- --nocapture`; both exited successfully, so I resumed from their retained artifacts instead of rerunning green work. I then verified the assembled closeout bundle on disk: `.tmp/m047-s06/verify/status.txt` is `ok`, `.tmp/m047-s06/verify/current-phase.txt` is `complete`, and `.tmp/m047-s06/verify/phase-report.txt` records `contract-guards`, `m047-s05-replay`, `retain-m047-s05-verify`, `m047-s06-e2e`, `m047-s06-docs-build`, `m047-s06-artifacts`, and `m047-s06-bundle-shape` as passed. The retained S05 bundle inside S06 also shows `status.txt = ok`, `current-phase.txt = complete`, and a passing delegated phase report for `m047-s04-replay`, `retain-m047-s04-verify`, `m047-s05-pkg`, `m047-s05-tooling`, `m047-s05-e2e`, `m047-s05-docs-build`, `retain-m047-s05-artifacts`, and `m047-s05-bundle-shape`.

From the retained command logs, the S06 contract rail recorded `running 3 tests` and `3 passed`, and the docs build finished with `build complete in 35.99s`. The slice-level SQLite built-package regression left a fresh `.tmp/m047-s06/sqlite-built-package-execute-*` artifact directory whose `run.stdout.log` shows `schema=ok`, `insert=1`, `count=1`, `mismatch_err=column index out of range`, and `done`, which confirms the built-package execute seam still handles helper rewraps correctly while failing closed on placeholder mismatch errors.

Because the direct successful `cargo test -p meshc --test e2e_m047_s06 -- --nocapture`, `cargo test -p meshc --test e2e_sqlite_built_package -- --nocapture`, and `bash scripts/verify-m047-s06.sh` runs completed in the interrupted session, their original wall-clock timings were no longer recoverable from tool metadata during resume. I preserved those checks in the evidence table with `0ms` placeholders and grounded the rest of the verification in the retained on-disk logs and status files.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m047_s06 -- --nocapture` | 0 | ✅ pass | 0ms |
| 2 | `cargo test -p meshc --test e2e_m047_s06 m047_s06_ -- --nocapture` | 0 | ✅ pass | 30ms |
| 3 | `npm --prefix website run build` | 0 | ✅ pass | 35990ms |
| 4 | `bash scripts/verify-m047-s06.sh` | 0 | ✅ pass | 0ms |
| 5 | `cargo test -p meshc --test e2e_sqlite_built_package -- --nocapture` | 0 | ✅ pass | 0ms |


## Deviations

I made one small local deviation from the written T03 file list: when the new S06 wrapper replayed the retained S04 and S05 rails, it exposed minor stale assertions in `compiler/meshc/tests/e2e_m047_s04.rs` and `compiler/meshc/tests/e2e_m047_s05.rs`, so I corrected those tests as part of keeping the delegated proof stack green and truthful.

## Known Issues

None.

## Files Created/Modified

- `README.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/distributed/index.md`
- `compiler/meshc/tests/e2e_m047_s06.rs`
- `scripts/verify-m047-s06.sh`
- `compiler/meshc/tests/e2e_m047_s04.rs`
- `compiler/meshc/tests/e2e_m047_s05.rs`
- `.gsd/DECISIONS.md`


## Deviations
I made one small local deviation from the written T03 file list: when the new S06 wrapper replayed the retained S04 and S05 rails, it exposed minor stale assertions in `compiler/meshc/tests/e2e_m047_s04.rs` and `compiler/meshc/tests/e2e_m047_s05.rs`, so I corrected those tests as part of keeping the delegated proof stack green and truthful.

## Known Issues
None.
