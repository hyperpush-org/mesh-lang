---
id: M029
provides:
  - Repaired `meshc fmt` so canonical output preserves dotted module paths, parenthesized multiline imports, top-level spacing, schema-option spacing, and silent `fmt --check` success.
  - Finished the Mesher/reference-backend dogfood cleanup, leaving both apps formatter-clean and build-clean under the repaired formatter.
  - Closed R024, R026, and R027 with milestone-level verification on current `main` plus exact-output regressions that guard this formatter bug class.
key_decisions:
  - Keep exact-output formatter tests at the walker, library, and CLI layers because `fmt --check` can pass on already-corrupted text.
  - Repair real dogfood source and rerun formatter waves instead of accepting idempotent bad output as proof.
  - Do the human-authored multiline-import rewrites first, then prove the result with formatter/build/test gates against the real codebases.
patterns_established:
  - Formatter bug fixes that can corrupt parseable text need text-sensitive regressions and real repo smoke targets.
  - Dogfood cleanup proofs should pair content greps (`<>`, wrapping `List.map`, long imports, dotted-path corruption) with formatter/build/test gates.
  - If a broken formatter emits parse-invalid text like `pubtype` or `table"..."`, restore from a pre-format copy before rerunning the fixed formatter.
observability_surfaces:
  - `git log --stat --oneline --grep='M029' -- ':!.gsd/'`
  - `cargo test -q -p mesh-fmt --lib`
  - `cargo test -q -p meshc --test e2e_fmt`
  - `cargo run -q -p meshc -- fmt --check reference-backend`
  - `cargo run -q -p meshc -- fmt --check mesher`
  - `cargo run -q -p meshc -- build reference-backend`
  - `cargo run -q -p meshc -- build mesher`
  - `cargo test -q -p meshc --test e2e -- --nocapture`
  - `rg -n '^from .{121,}' mesher -g '*.mpl'`
  - `rg -n '^from .*\. ' mesher reference-backend -g '*.mpl'`
  - `rg -n 'List\.map\(rows,|Ok\(List\.map\(' mesher -g '*.mpl'`
  - `rg -n '<>' mesher -g '*.mpl' | cut -d: -f1-2 | sort`
requirement_outcomes:
  - id: R024
    from_status: active
    to_status: validated
    proof: Current closeout reran `meshc fmt --check mesher` and `meshc build mesher`, the long-import and dotted-path greps are clean, the pipe-style gate returns 0, and only five accepted SQL/DDL `<>` sites remain in `mesher/storage/{queries,schema}.mpl`.
  - id: R026
    from_status: active
    to_status: validated
    proof: `cargo test -q -p mesh-fmt --lib` now passes 129/129, `cargo test -q -p meshc --test e2e_fmt` passes 9/9, and the repaired formatter leaves both dogfood apps clean under `fmt --check`.
  - id: R027
    from_status: active
    to_status: validated
    proof: `cargo run -q -p meshc -- fmt --check reference-backend` now exits cleanly with a 0-byte success log, and `rg -n '^from .*\. ' reference-backend -g '*.mpl'` returns no matches.
duration: 6h27m + milestone closeout
verification_result: passed
completed_at: 2026-03-24
---

# M029: Mesher & Reference-Backend Dogfood Completion

**Closed the formatter corruption gap and finished the Mesher/reference-backend dogfood cleanup, leaving both apps formatter-clean, build-clean, and using the canonical multiline-import path.**

## What Happened

M029 closed the remaining dogfood friction that M031 left behind, and it did it in the right order. S01 fixed the formatter root cause first: `compiler/mesh-fmt/src/walker.rs` now routes dotted paths through a dedicated path walker, parenthesized multiline imports survive round-trip formatting, and the proof surface moved beyond `fmt --check` into exact-output regressions at the formatter library and CLI layers. That same slice repaired the already-corrupted `reference-backend/` imports so the formatter proof used canonical source instead of stable-but-wrong text.

S02 then finished the Mesher content cleanup that had been blocked on formatter trust. The API serializers moved off brittle `<>` JSON assembly onto `json {}` for scalar rows and `#{}` interpolation for raw JSONB payloads, storage helpers moved from wrapping `List.map(rows, ...)` calls to pipe style, and the non-SQL token builders moved to interpolation. After that pass, Mesher's remaining `<>` usage was reduced to the deliberate SQL/DDL keep sites in `mesher/storage/{queries,schema}.mpl`.

S03 consumed both of those results and finished the repo-wide cleanup: every over-120-character Mesher import moved to parenthesized multiline form, both dogfood apps were reformatted onto the repaired canonical output, and the final closeout wave fixed the last text-sensitive formatter regressions that the real repos exposed (`pub type` spacing, `table "..."` schema options, top-level/import compaction, pipe indentation, and noisy `fmt --check` success output). The assembled milestone result is not just cleaner source; it is a truthful formatter/build/test surface that keeps Mesher and reference-backend on the same canonical style without hand-maintained exceptions.

## Cross-Slice Verification

- **Implementation exists, not just planning artifacts.** The prescribed `git diff --stat HEAD $(git merge-base HEAD main) -- ':!.gsd/'` check is empty in this closeout because the unit is running on already-integrated `main` (`HEAD == merge-base == main` at `9cdb6506`). The equivalent history proof shows substantial non-`.gsd/` code changes across the milestone: `eb90baf8`, `e045aec8`, `2181ac2f`, `c96262a3`, `4f4dffe7`, `8c737534`, `da683b75`, `7fad1831`, `ddfc1993`, `36736bb4`, and `c2cdb9eb` all modify compiler, Mesher, or reference-backend sources.
- **Formatter library proof passed.** `cargo test -q -p mesh-fmt --lib` now passes `129 passed; 0 failed` from `/tmp/m029-closeout-mesh-fmt.log`, covering the dedicated dotted-path/import walker plus the later spacing regressions exposed by S03.
- **Formatter CLI proof passed.** `cargo test -q -p meshc --test e2e_fmt` now passes `9 passed; 0 failed` from `/tmp/m029-closeout-e2e-fmt.log`, including the exact-output formatter coverage that guards dotted paths, multiline imports, schema spacing, and silent-success behavior.
- **`meshc fmt --check reference-backend` passed with clean dotted paths.** `cargo run -q -p meshc -- fmt --check reference-backend` exited cleanly and produced a `0`-byte success log; `rg -n '^from .*\. ' reference-backend -g '*.mpl'` returned no matches, so the backend no longer contains `Api. Router`-style corruption.
- **`meshc fmt --check mesher` passed with zero remaining long import lines.** `cargo run -q -p meshc -- fmt --check mesher` exited cleanly with a `0`-byte success log. `rg -n '^from .{121,}' mesher -g '*.mpl'` returned no matches, so the long single-line imports that motivated the multiline-import rollout are gone.
- **Both dogfood codebases build clean.** `cargo run -q -p meshc -- build reference-backend` produced `Compiled: reference-backend/reference-backend`, and `cargo run -q -p meshc -- build mesher` produced `Compiled: mesher/mesher`.
- **The full `meshc` e2e acceptance target stayed on the known baseline.** `cargo test -q -p meshc --test e2e -- --nocapture` finished at `318 passed; 10 failed`. The failing set exactly matches the pre-existing try-family baseline documented in prior M031 UAT artifacts: `e2e_cross_module_try_operator`, `e2e_err_binding_pattern`, `e2e_from_try_error_conversion`, `e2e_option_field_extraction`, `e2e_try_chained_result`, `e2e_try_operator_result`, `e2e_try_option_some_path`, `e2e_try_result_binding_arity`, `e2e_try_result_ok_path`, and `e2e_tryfrom_try_operator`. That satisfies the roadmap gate of `318+` passing tests with no new regressions from the M031 baseline.
- **Mesher `<>` cleanup is complete except for the accepted SQL/DDL keep sites.** `rg -n '<>' mesher -g '*.mpl' | cut -d: -f1-2 | sort` now returns exactly five locations: `mesher/storage/queries.mpl:604`, `mesher/storage/queries.mpl:953`, and `mesher/storage/schema.mpl:11-13`. These are the accepted SQL/DDL survivors from the roadmap.
- **Mesher wrapping `List.map(rows, ...)` patterns are gone.** `rg -n 'List\.map\(rows,|Ok\(List\.map\(' mesher -g '*.mpl'` returned no matches, so the old wrapper form has been replaced with the pipe style the milestone targeted.
- **Definition of done is satisfied end-to-end.** All slices are marked complete in the roadmap, all three slice summary files exist, the formatter/library/CLI/build gates pass on current source, and the shared integration points between S01, S02, and S03 hold together under the full milestone verification rerun.

No success criteria were left unmet.

## Requirement Changes

- R024: active → validated — Closeout reran `meshc fmt --check mesher` and `meshc build mesher`, confirmed zero over-120-character import lines, zero wrapping `List.map(rows, ...)` patterns, and only the five accepted SQL/DDL `<>` survivors.
- R026: active → validated — `cargo test -q -p mesh-fmt --lib` passed 129/129, `cargo test -q -p meshc --test e2e_fmt` passed 9/9, and the repaired formatter now preserves dotted module paths and parenthesized multiline imports on the real dogfood apps.
- R027: active → validated — `meshc fmt --check reference-backend` now exits cleanly on current source, and the dotted-path grep for spaced imports returns no matches.

## Forward Intelligence

### What the next milestone should know
- The formatter is finally trustworthy enough to use as a real cleanup tool again, but only because the exact-output regressions now backstop the repo-level `fmt --check` gates. Do not delete those tests while working on M030 formatter or tooling changes.
- The authoritative full `meshc` e2e baseline is still `318 passed / 10 failed`, and the exact expected failure list is now recorded in `.gsd/KNOWLEDGE.md`. Future closeouts should compare against that exact set, not a vague `try_*` description.

### What's fragile
- Text-level formatter regressions are still the thinnest surface. A bad pass can emit parseable-but-wrong output (`Api. Router`) or parse-invalid output (`pubtype`, `table"..."`) that a second pass cannot repair.

### Authoritative diagnostics
- `compiler/meshc/tests/e2e_fmt.rs` plus `cargo test -q -p meshc --test e2e_fmt` — this is the most trustworthy text-level formatter signal because it exercises the real CLI and exact output.
- `cargo run -q -p meshc -- fmt --check mesher` and `cargo run -q -p meshc -- fmt --check reference-backend` — these are the real dogfood truth surfaces once the exact-output tests are green.
- `cargo test -q -p meshc --test e2e -- --nocapture` — authoritative milestone-level acceptance because it proves the formatter/dogfood cleanup did not disturb the broader compiler baseline.

### What assumptions changed
- `fmt --check` alone is enough to prove formatter correctness — false. This milestone needed walker/library/CLI exact-output tests plus real repo smoke targets to catch the failure mode honestly.
- The M029 closeout proof could be deferred after slice completion — false. The full `meshc` e2e rerun materially changed milestone status from "slice-complete" to actually done.

## Files Created/Modified

- `compiler/mesh-fmt/src/walker.rs` — fixed dotted-path formatting, multiline import preservation, top-level/import compaction, pipe indentation, `pub type` spacing, and schema-option spacing.
- `compiler/mesh-fmt/src/lib.rs` — added formatter library regressions for dotted-path and multiline-import correctness.
- `compiler/meshc/tests/e2e_fmt.rs` — added CLI exact-output formatter coverage and silent-success behavior checks.
- `compiler/meshc/src/main.rs` — removed noisy `fmt --check` success output so captured-log proof surfaces stay truthful.
- `mesher/api/alerts.mpl`, `mesher/api/detail.mpl`, `mesher/api/search.mpl` — replaced brittle JSON `<>` assembly with `json {}` or `#{}` interpolation, depending on payload shape.
- `mesher/storage/queries.mpl` and `mesher/storage/schema.mpl` — moved wrapping row-maps to pipe style, converted non-SQL token builders to interpolation, and left only the accepted SQL/DDL `<>` survivors.
- `mesher/main.mpl`, `mesher/ingestion/routes.mpl`, `mesher/api/dashboard.mpl`, `mesher/api/team.mpl`, `mesher/services/project.mpl`, and `mesher/services/user.mpl` — adopted parenthesized multiline imports and the repaired formatter canonical output.
- `reference-backend/main.mpl`, `reference-backend/api/health.mpl`, `reference-backend/api/router.mpl`, `reference-backend/api/jobs.mpl`, `reference-backend/jobs/worker.mpl`, `reference-backend/runtime/registry.mpl`, and `reference-backend/storage/jobs.mpl` — restored canonical dotted imports and formatter-clean output under the fixed formatter.
