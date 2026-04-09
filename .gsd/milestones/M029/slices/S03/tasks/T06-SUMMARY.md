---
id: T06
parent: S03
milestone: M029
provides:
  - S03 now closes with green formatter, build, grep, captured-log, and UAT proof across both `mesher/` and `reference-backend/`, with exact-output tests guarding the formatter regressions hit during the final wave
key_files:
  - compiler/mesh-fmt/src/walker.rs
  - compiler/meshc/src/main.rs
  - compiler/meshc/tests/e2e_fmt.rs
  - mesher/types/event.mpl
  - mesher/types/issue.mpl
  - reference-backend/api/jobs.mpl
  - reference-backend/api/router.mpl
  - reference-backend/jobs/worker.mpl
  - reference-backend/main.mpl
  - reference-backend/migrations/20260323010000_create_jobs.mpl
  - reference-backend/runtime/registry.mpl
  - reference-backend/storage/jobs.mpl
  - .gsd/milestones/M029/slices/S03/S03-UAT.md
  - .gsd/milestones/M029/slices/S03/tasks/T06-PLAN.md
  - .gsd/milestones/M029/slices/S03/S03-PLAN.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Fixed the formatter and `meshc fmt --check` success-path noise instead of weakening the slice proof once the final gate showed real regressions (`pubtype`, `table"..."`, and non-empty captured logs)
  - Restored `mesher/types/event.mpl` and `mesher/types/issue.mpl` from the pre-format source snapshot before rerunning the repaired formatter, because the first broken formatter pass had already truncated those declarations and a second pass could not recover lost CST structure
patterns_established:
  - If `meshc fmt` emits `pubtype` or `table"..."`, restore any already-damaged source from a pre-format copy before rerunning the repaired formatter; once the parser loses the declaration body, format idempotency cannot recover it
  - `meshc fmt --check` must stay silent on success; slice gates that capture formatter output to a log file use emptiness as part of the acceptance contract, not just exit code 0
observability_surfaces:
  - "cargo test -q -p mesh-fmt --lib"
  - "cargo test -q -p meshc --test e2e_fmt"
  - "cargo run -q -p meshc -- fmt --check mesher"
  - "cargo run -q -p meshc -- fmt --check reference-backend"
  - "cargo run -q -p meshc -- build mesher"
  - "cargo run -q -p meshc -- build reference-backend"
  - "! rg -n '^from .{121,}' mesher -g '*.mpl'"
  - "! rg -n '^from .*\\. ' mesher reference-backend -g '*.mpl'"
  - "/tmp/m029-s03-fmt-mesher.log"
  - ".gsd/milestones/M029/slices/S03/S03-UAT.md"
duration: 52m
verification_result: passed
completed_at: 2026-03-24T13:51:45-04:00
blocker_discovered: false
---

# T06: Canonicalize remaining Mesher files and record final slice proof

**Fixed the formatter/CLI spacing regressions, restored the damaged Mesher type modules, and closed S03 with green formatter/build/UAT proof on both dogfood apps.**

## What Happened

I fixed the pre-flight artifact gap first. `.gsd/milestones/M029/slices/S03/tasks/T06-PLAN.md` now includes the required `## Observability Impact` section before the source wave.

Then I read the two Mesher migration files, two Mesher test files, six Mesher type files, and the `reference-backend/api/health.mpl` import anchor. I moved `mesher/types`, `mesher/tests`, and `mesher/migrations` onto the current formatter output, then used the closeout gate to check the slice instead of assuming the plan snapshot was still accurate.

That gate surfaced a real formatter bug rather than ordinary cleanup: `mesher/types/event.mpl` and `mesher/types/issue.mpl` came back as broken `pubtype` output, and the final captured-log proof later showed that `meshc fmt --check` was still noisy on successful runs. The root causes were both in the formatter stack: `SUM_TYPE_DEF` visibility tokens were not spaced in `compiler/mesh-fmt/src/walker.rs`, `SCHEMA_OPTION` still fell through to generic inline spacing and collapsed `table "..."`, and `compiler/meshc/src/main.rs` still printed `N file(s) already formatted` on successful `fmt --check` runs.

I fixed the walker, added exact-output coverage in `compiler/meshc/tests/e2e_fmt.rs`, restored the two Mesher type files from the pre-format source I had already read earlier in the task, and reran the repaired formatter over `mesher/types`. I then reformatted `reference-backend/`, which was still red on the seven stale files already called out in project knowledge; the final pass touched eleven backend files total but stayed formatter-clean afterward.

With the formatter and CLI fixed, I wrote `.gsd/milestones/M029/slices/S03/S03-UAT.md` in the required artifact-driven form. It records the final source-shape proof, calls out `reference-backend/api/health.mpl` as the preserved canonical multiline smoke target, and documents the exact checks that prove the slice closed truthfully.

No runtime behavior or observability surface was added in product code. The durable inspection surfaces remain formatter/build commands, the two grep gates, `/tmp/m029-s03-fmt-mesher.log`, the CLI formatter tests, and the slice UAT artifact.

## Verification

I verified the formatter fix itself before trusting any more cleanup output:
- `cargo test -q -p mesh-fmt --lib`
- `cargo test -q -p meshc --test e2e_fmt`

Then I reran the task-level closeout command from the task plan after the UAT artifact existed. It passed end-to-end, including Mesher reformatting, both formatter checks, both builds, both grep gates, and the UAT file-existence check.

Finally I reran every slice-level verification check individually. All passed, including the captured-log gate that previously failed before `meshc fmt --check` was made silent on success:
- repo-wide long-import grep
- `meshc fmt --check mesher`
- `meshc fmt --check reference-backend`
- `meshc build mesher`
- `meshc build reference-backend`
- repo-wide spaced-dotted-path grep
- captured formatter-log emptiness gate
- `test -f .gsd/milestones/M029/slices/S03/S03-UAT.md`

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -q -p mesh-fmt --lib` | 0 | ✅ pass | 0.63s |
| 2 | `cargo test -q -p meshc --test e2e_fmt` | 0 | ✅ pass | 6.76s |
| 3 | `cargo run -q -p meshc -- fmt mesher/types && cargo run -q -p meshc -- fmt mesher/tests && cargo run -q -p meshc -- fmt mesher/migrations && cargo run -q -p meshc -- fmt --check mesher && cargo run -q -p meshc -- fmt --check reference-backend && cargo run -q -p meshc -- build mesher && cargo run -q -p meshc -- build reference-backend && ! rg -n '^from .{121,}' mesher -g '*.mpl' && ! rg -n '^from .*\. ' mesher reference-backend -g '*.mpl' && test -f .gsd/milestones/M029/slices/S03/S03-UAT.md` | 0 | ✅ pass | 56.62s |
| 4 | `! rg -n '^from .{121,}' mesher -g '*.mpl' && ! rg -n '^from .*\. ' mesher reference-backend -g '*.mpl'` | 0 | ✅ pass | 0.11s |
| 5 | `! rg -n '^from .{121,}' mesher -g '*.mpl'` | 0 | ✅ pass | 0.04s |
| 6 | `cargo run -q -p meshc -- fmt --check mesher` | 0 | ✅ pass | 7.44s |
| 7 | `cargo run -q -p meshc -- fmt --check reference-backend` | 0 | ✅ pass | 6.56s |
| 8 | `cargo run -q -p meshc -- build mesher` | 0 | ✅ pass | 13.44s |
| 9 | `cargo run -q -p meshc -- build reference-backend` | 0 | ✅ pass | 8.32s |
| 10 | `! rg -n '^from .*\. ' mesher reference-backend -g '*.mpl'` | 0 | ✅ pass | 0.07s |
| 11 | `(cargo run -q -p meshc -- fmt --check mesher > /tmp/m029-s03-fmt-mesher.log 2>&1 && test ! -s /tmp/m029-s03-fmt-mesher.log) \|\| (rg -n 'error\|panic\|from .*\. ' /tmp/m029-s03-fmt-mesher.log && false)` | 0 | ✅ pass | 7.11s |
| 12 | `test -f .gsd/milestones/M029/slices/S03/S03-UAT.md` | 0 | ✅ pass | 0.01s |

## Diagnostics

The durable inspection surfaces for this task are the formatter library suite, the `meshc` formatter CLI suite, `cargo run -q -p meshc -- fmt --check mesher`, `cargo run -q -p meshc -- fmt --check reference-backend`, both build commands, the two repo-wide import-shape greps, `/tmp/m029-s03-fmt-mesher.log`, and `.gsd/milestones/M029/slices/S03/S03-UAT.md`.

If this work appears to regress later, start with the two exact-output formatter surfaces:
1. `cargo test -q -p mesh-fmt --lib`
2. `cargo test -q -p meshc --test e2e_fmt`

If those fail with `pubtype` or `table"..."` symptoms, inspect `compiler/mesh-fmt/src/walker.rs` around `walk_block_def(...)` and `walk_schema_option(...)`, then restore any already-corrupted source from a pre-format copy before rerunning the fixed formatter. If the formatter tests are green but slice proof fails, inspect `/tmp/m029-s03-fmt-mesher.log`, `cargo run -q -p meshc -- fmt --check reference-backend`, and the two grep gates to separate CLI-noise regressions from actual source-shape drift.

## Deviations

- Expanded the task beyond the original Mesher-only formatter wave because the truthful slice gate still required reformatting the known stale `reference-backend/` files and because the final proof exposed a real formatter/CLI regression instead of simple dogfood cleanup.
- Restored `mesher/types/event.mpl` and `mesher/types/issue.mpl` from the pre-format source snapshot after the first broken formatter pass truncated them; this recovery step was not in the written plan but was required to preserve the actual Mesher declarations before rerunning the repaired formatter.

## Known Issues

- None in the S03 acceptance surface. The slice closes green.
- The accepted canonical formatter output still includes spaces around generic/result-type syntax and compact `do|state|` separators in some files; that output is now explicitly documented in `S03-UAT.md` as accepted, not a blocker.

## Files Created/Modified

- `compiler/mesh-fmt/src/walker.rs` — fixed spacing for public sum-type headers and schema `table "..."` options so the formatter no longer emits `pubtype` or `table"..."` corruption.
- `compiler/meshc/src/main.rs` — made `meshc fmt --check` silent on success so captured-log gates can use empty output as a truthful proof surface.
- `compiler/meshc/tests/e2e_fmt.rs` — added exact-output coverage for the spacing regressions and asserted that successful `fmt --check` runs stay silent.
- `mesher/types/event.mpl` — restored the damaged event type module from pre-format source and moved it onto the repaired canonical formatter output.
- `mesher/types/issue.mpl` — restored the damaged issue type module from pre-format source and moved it onto the repaired canonical formatter output.
- `mesher/types/alert.mpl` — moved the remaining Mesher type file onto canonical formatter output.
- `mesher/types/project.mpl` — moved the remaining Mesher type file onto canonical formatter output.
- `mesher/types/retention.mpl` — moved the remaining Mesher type file onto canonical formatter output.
- `mesher/types/user.mpl` — moved the remaining Mesher type file onto canonical formatter output.
- `mesher/tests/fingerprint.test.mpl` — moved the remaining Mesher test file onto canonical formatter output.
- `mesher/tests/validation.test.mpl` — moved the remaining Mesher test file onto canonical formatter output.
- `mesher/migrations/20260216120000_create_initial_schema.mpl` — moved the remaining Mesher migration file onto canonical formatter output.
- `mesher/migrations/20260226000000_seed_default_org.mpl` — moved the remaining Mesher migration file onto canonical formatter output.
- `reference-backend/api/jobs.mpl` — moved stale backend formatter output onto the repaired canonical form.
- `reference-backend/api/router.mpl` — moved stale backend formatter output onto the repaired canonical form.
- `reference-backend/jobs/worker.mpl` — moved stale backend formatter output onto the repaired canonical form.
- `reference-backend/main.mpl` — moved stale backend formatter output onto the repaired canonical form.
- `reference-backend/migrations/20260323010000_create_jobs.mpl` — moved stale backend formatter output onto the repaired canonical form.
- `reference-backend/runtime/registry.mpl` — moved stale backend formatter output onto the repaired canonical form.
- `reference-backend/storage/jobs.mpl` — moved stale backend formatter output onto the repaired canonical form.
- `.gsd/milestones/M029/slices/S03/S03-UAT.md` — recorded the final artifact-driven closeout proof for S03.
- `.gsd/milestones/M029/slices/S03/tasks/T06-PLAN.md` — added the missing observability section required by the pre-flight check.
- `.gsd/KNOWLEDGE.md` — recorded the formatter spacing/truncation gotcha and the silent-success requirement for `meshc fmt --check`.
