---
id: S03
parent: M029
milestone: M029
provides:
  - Mesher now uses canonical parenthesized multiline imports for every remaining over-120-character `from` import, and both dogfood apps are formatter-clean under the repaired canonical output
  - `mesh-fmt` and `meshc fmt --check` now guard the final readability/correctness regressions exposed by the closeout gate (`pubtype`, `table"..."`, noisy success output)
requires:
  - slice: S01
    provides: Formatter support for dotted module paths and parenthesized multiline imports
  - slice: S02
    provides: Mesher JSON/interpolation cleanup and pipe-style cleanup, leaving S03 to finish import rollout and formatter compliance
affects:
  - M030/S01
key_files:
  - compiler/mesh-fmt/src/walker.rs
  - compiler/meshc/src/main.rs
  - compiler/meshc/tests/e2e_fmt.rs
  - mesher/main.mpl
  - mesher/ingestion/routes.mpl
  - mesher/api/alerts.mpl
  - mesher/api/dashboard.mpl
  - mesher/api/team.mpl
  - mesher/services/project.mpl
  - mesher/services/user.mpl
  - reference-backend/api/health.mpl
  - .gsd/milestones/M029/slices/S03/S03-UAT.md
key_decisions:
  - Use `reference-backend/api/health.mpl` as the exact multiline-import anchor for the manual Mesher rewrites, then let later formatter waves own the mechanical normalization
  - Fix the formatter and CLI truth surface when the closeout gate exposed real regressions, instead of weakening the acceptance checks or hand-preserving ugly output
  - Restore already-damaged Mesher type files from pre-format snapshots before rerunning the repaired formatter, because a second pass cannot reconstruct declarations once the CST is truncated
patterns_established:
  - For dogfood cleanup, do the human-authored import-shape rewrites first, prove the repo-wide long-import grep is green, and only then run directory-scoped formatter waves
  - `fmt --check` is not enough for text-sensitive regressions; keep exact-output tests for dotted paths, top-level spacing, pipe indentation, `pub type` headers, schema options, and silent-success CLI behavior
  - If a broken formatter pass emitted `pubtype` or `table"..."`, restore from a pre-format copy before rerunning the fixed formatter
observability_surfaces:
  - `cargo test -q -p mesh-fmt --lib`
  - `cargo test -q -p meshc --test e2e_fmt`
  - `cargo run -q -p meshc -- fmt --check mesher`
  - `cargo run -q -p meshc -- fmt --check reference-backend`
  - `cargo run -q -p meshc -- build mesher`
  - `cargo run -q -p meshc -- build reference-backend`
  - `rg -n '^from .{121,}' mesher -g '*.mpl'`
  - `rg -n '^from .*\. ' mesher reference-backend -g '*.mpl'`
  - `/tmp/m029-s03-fmt-mesher.log`
drill_down_paths:
  - .gsd/milestones/M029/slices/S03/tasks/T01-SUMMARY.md
  - .gsd/milestones/M029/slices/S03/tasks/T02-SUMMARY.md
  - .gsd/milestones/M029/slices/S03/tasks/T03-SUMMARY.md
  - .gsd/milestones/M029/slices/S03/tasks/T04-SUMMARY.md
  - .gsd/milestones/M029/slices/S03/tasks/T05-SUMMARY.md
  - .gsd/milestones/M029/slices/S03/tasks/T06-SUMMARY.md
duration: 3h12m
verification_result: passed
completed_at: 2026-03-24
---

# S03: Multiline imports and final formatter compliance

**Mesher’s remaining long imports were converted to the canonical multiline form, both dogfood apps now pass formatter/build gates under the repaired formatter output, and the last formatter/CLI truth-surface regressions are covered by exact-output tests.**

## What Happened

S03 started as a source-cleanup slice and finished as both cleanup and toolchain hardening.

T01 and T02 did the narrow human-authored work first. They rewrote the remaining over-120-character Mesher `from ... import ...` lines to the parenthesized multiline form, using `reference-backend/api/health.mpl` as the exact anchor. That rollout landed in `mesher/main.mpl`, `mesher/ingestion/routes.mpl`, `mesher/api/{alerts,dashboard,team}.mpl`, and `mesher/services/{project,user}.mpl`. Once those rewrites were in place, the repo-wide Mesher long-import grep went green, which made later failures obviously formatter/UAT backlog instead of leftover source-shape work.

T03 moved `mesher/main.mpl` and the API modules onto canonical formatter output. That wave proved the earlier S01 dotted-path and multiline-import fixes held under real dogfood formatting, but it also exposed that readable canonical output still depended on more formatter work than the slice plan expected.

T04 reproduced three real formatter regressions from the Mesher cleanup wave and fixed them in `compiler/mesh-fmt/src/walker.rs`: adjacent top-level comments/imports were being spread apart, pipe chains were staircasing, and single-statement closures returning multi-field struct literals were collapsing into unreadable one-liners. After the walker fix, the formatter library suite passed again and the already-touched Mesher files were reformatted to the corrected canonical output instead of leaving the ugly intermediate output in the tree.

T05 finished the Mesher service wave. It confirmed that `mesher/services/project.mpl` and `mesher/services/user.mpl` kept their multiline imports intact after formatting, and it established that some remaining aesthetic quirks in the current canonical output — spaces around generic/result-type syntax and compact `do|state|` separators — were pre-existing accepted output rather than new service-specific regressions.

T06 closed the slice. The final gate surfaced two more real truth-surface failures: broken formatter spacing for `pub type` and schema `table "..."` syntax, and noisy success output from `meshc fmt --check` that made the captured-log gate untrustworthy. The slice fixed both problems, added exact-output coverage in `compiler/meshc/tests/e2e_fmt.rs`, restored `mesher/types/event.mpl` and `mesher/types/issue.mpl` from pre-format snapshots before rerunning the repaired formatter, reformatted the remaining Mesher files plus the stale `reference-backend/` backlog, and wrote the final artifact-driven UAT.

By the end of the slice, the Mesher import rollout, formatter cleanup, reference-backend regression pass, and captured-log proof all closed green under one coherent acceptance surface.

## Verification

Slice-level verification passed end to end:

- `cargo test -q -p mesh-fmt --lib`
- `cargo test -q -p meshc --test e2e_fmt`
- `cargo run -q -p meshc -- fmt --check mesher`
- `cargo run -q -p meshc -- fmt --check reference-backend`
- `cargo run -q -p meshc -- build mesher`
- `cargo run -q -p meshc -- build reference-backend`
- `! rg -n '^from .{121,}' mesher -g '*.mpl'`
- `! rg -n '^from .*\. ' mesher reference-backend -g '*.mpl'`
- `cargo run -q -p meshc -- fmt --check mesher > /tmp/m029-s03-fmt-mesher.log 2>&1 && test ! -s /tmp/m029-s03-fmt-mesher.log`
- `test -f .gsd/milestones/M029/slices/S03/S03-UAT.md`

The observability/diagnostic surface from the slice plan also worked as intended: the long-import grep narrowed the remaining manual rewrite surface, the dotted-path grep caught the class of formatter corruption the slice was guarding against, and `/tmp/m029-s03-fmt-mesher.log` became a trustworthy first-failure artifact only after `meshc fmt --check` was made silent on success.

## Requirements Advanced

- R011 — S03 stayed evidence-first. When Mesher cleanup exposed real formatter defects, the work fixed Mesh itself and tightened the proof surface instead of hand-waving the output or weakening the acceptance gate.

## Requirements Validated

- R024 — Mesher now closes the remaining dogfood cleanup contract: the earlier `let _ =` removal remains in place, JSON/interpolation and pipe cleanup were already closed by S02, S03 converted the remaining long imports to the canonical multiline form, and `meshc fmt --check mesher` plus `meshc build mesher` now pass on the cleaned source.

## New Requirements Surfaced

- none

## Requirements Invalidated or Re-scoped

- none

## Deviations

T04 and T06 expanded beyond the original source-only cleanup plan because the truthful closeout gates surfaced real formatter/CLI defects. S03 therefore ended up repairing `compiler/mesh-fmt` and the `meshc fmt --check` success-path contract, then reformatting already-touched Mesher files and the stale `reference-backend/` backlog under the corrected canonical output. That was unplanned, but it was the right deviation: the slice goal was final formatter compliance, not merely rewriting source until a brittle tool happened to pass.

## Known Limitations

- The accepted canonical formatter output still inserts spaces around some generic/result-type syntax and keeps compact `do|state|` separators in some files. Those shapes are now treated as accepted canonical output, not as S03 blockers.
- This slice did not rerun the broader milestone-level `cargo test -p meshc --test e2e` acceptance gate; S03 proved the formatter/build/import-shape surface it owned.

## Follow-ups

- M030 tooling work should preserve the exact-output formatter tests added here and treat silent-success `fmt --check` output as part of the public tooling contract.
- If future formatter work changes canonical spacing again, reformat any previously touched dogfood files in the same change rather than leaving the tree split across two accepted outputs.

## Files Created/Modified

- `compiler/mesh-fmt/src/walker.rs` — repaired top-level spacing, pipe indentation, closure/struct-literal formatting, `pub type` spacing, and schema-option spacing.
- `compiler/meshc/src/main.rs` — made `meshc fmt --check` silent on success so captured-log gates can trust an empty log.
- `compiler/meshc/tests/e2e_fmt.rs` — added exact-output regression coverage for the late formatter and CLI failures surfaced by S03.
- `mesher/main.mpl` — now preserves the canonical multiline route/dashboard/team/alerts imports under formatter output.
- `mesher/ingestion/routes.mpl` — now preserves the multiline `Storage.Queries` import under formatter output.
- `mesher/api/alerts.mpl` — now uses the multiline import shape and canonical formatter output.
- `mesher/api/dashboard.mpl` — now uses the multiline import shape and canonical formatter output.
- `mesher/api/team.mpl` — now uses the multiline import shape and canonical formatter output.
- `mesher/services/project.mpl` — now uses the multiline import shape and canonical formatter output.
- `mesher/services/user.mpl` — now uses the multiline import shape and canonical formatter output.
- `reference-backend/api/health.mpl` — remains the canonical multiline import anchor used by the slice and the UAT smoke target.
- `.gsd/milestones/M029/slices/S03/S03-UAT.md` — records the final artifact-driven acceptance script for the slice.

## Forward Intelligence

### What the next slice should know
- The Mesher manual rewrite surface is gone. If long single-line imports or `Storage. Queries`-style corruption reappear now, treat that as a regression, not unfinished S03 cleanup.
- The most trustworthy formatter regression surfaces after this slice are `cargo test -q -p mesh-fmt --lib`, `cargo test -q -p meshc --test e2e_fmt`, and the empty-log `fmt --check` capture gate.

### What's fragile
- Formatter passes over already-damaged source — once a broken pass has emitted `pubtype` or `table"..."`, a second format run cannot recover the lost declaration body. Keep a pre-format copy when investigating this class of bug.
- Canonical spacing details in `compiler/mesh-fmt/src/walker.rs` — small token-separator regressions can still yield parse-valid but misleading output, so exact-output tests matter.

### Authoritative diagnostics
- `/tmp/m029-s03-fmt-mesher.log` — on a green run it must be empty; if it contains output, you have either a real formatter failure or a CLI noise regression.
- `rg -n '^from .*\. ' mesher reference-backend -g '*.mpl'` — this is still the fastest way to catch the specific dotted-path corruption class S03 was closing.
- `reference-backend/api/health.mpl` — this remains the canonical multiline import smoke fixture for both source review and formatter behavior.

### What assumptions changed
- “S03 is only a Mesher source-cleanup slice.” — Not true once the closeout gates were run. Real formatter and CLI defects were still in the path, and fixing them was necessary for honest formatter compliance.
- “A successful `fmt --check` only needs exit code 0.” — Not true for this repo. Silent success is now part of the proof contract because captured-log gates rely on emptiness, not just status.
