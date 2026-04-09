---
id: T02
parent: S01
milestone: M029
provides:
  - Library- and CLI-level exact-output formatter regressions that catch dotted-path corruption before T03 repairs dogfood sources
key_files:
  - compiler/mesh-fmt/src/lib.rs
  - compiler/meshc/tests/e2e_fmt.rs
  - .gsd/milestones/M029/slices/S01/tasks/T02-PLAN.md
key_decisions: []
patterns_established:
  - Exact-output CLI formatter tests are required when semantically wrong formatter output can become idempotent under `meshc fmt --check`.
observability_surfaces:
  - cargo test -q -p meshc --test e2e_fmt fmt_preserves_dotted_paths_exactly -- --nocapture
  - cargo test -q -p mesh-fmt --lib snapshot_parenthesized_dotted_from_import -- --nocapture
  - cargo run -q -p meshc -- fmt --check reference-backend
  - ! rg -n "^from .*\. " reference-backend -g '*.mpl'
duration: 20m
verification_result: passed
completed_at: 2026-03-24 11:36:19 EDT
blocker_discovered: false
---

# T02: Add truthful library and CLI regressions for dotted imports

**Added library and CLI exact-output formatter regressions so dotted imports fail on `Api. Router` / `Foo. Bar` corruption instead of hiding behind idempotent output.**

## What Happened

I extended `compiler/mesh-fmt/src/lib.rs` with library-layer dotted-import coverage in both directions the task asked for: idempotency tests for single-line and parenthesized dotted `from` imports, plus inline snapshot assertions for canonical `from Api.Router import ...` output. That moves the proof above the walker-local tests without waiting for the CLI layer.

In `compiler/meshc/tests/e2e_fmt.rs`, I added a small exact-output helper and a table-driven `fmt_preserves_dotted_paths_exactly` regression that writes real temp files, runs `meshc fmt`, and asserts the full rewritten text for three cases: dotted single-line import, parenthesized multiline dotted import, and qualified impl header. Those assertions fail on the old bug even if `fmt --check` would otherwise be happy with already-corrupted source.

Per the pre-flight contract, I also repaired the task and slice plan artifacts by adding the missing observability language and a targeted CLI verification command. I recorded the non-obvious rule in `.gsd/KNOWLEDGE.md`: `fmt --check` alone is not an authoritative regression surface when wrong formatter output can stabilize into an idempotent bad state.

## Verification

The new semantic proof surfaces passed at both layers: the full `mesh-fmt` library suite passed with the added dotted-import coverage, the targeted library snapshot passed, the new CLI exact-output regression passed, and the pre-existing compiler proof for parenthesized multiline imports stayed green.

I also ran the broader slice checks. They still show the known remaining T03 work rather than a T02 regression: the full `e2e_fmt` suite still fails only on `fmt_check_reference_backend_directory_succeeds`, `meshc fmt --check reference-backend` still reports five stale spaced-import files, and the negative `rg` sweep still finds those same files.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -q -p mesh-fmt --lib` | 0 | ✅ pass | 4.9s |
| 2 | `cargo test -q -p mesh-fmt --lib snapshot_parenthesized_dotted_from_import -- --nocapture` | 0 | ✅ pass | 2.8s |
| 3 | `cargo test -q -p meshc --test e2e_fmt fmt_preserves_dotted_paths_exactly -- --nocapture` | 0 | ✅ pass | 13.5s |
| 4 | `cargo test -q -p meshc --test e2e_fmt -- --nocapture` | 101 | ❌ fail | 5.9s |
| 5 | `cargo test -q -p meshc --test e2e e2e_multiline_import_paren -- --nocapture` | 0 | ✅ pass | 10.2s |
| 6 | `cargo run -q -p meshc -- fmt --check reference-backend` | 1 | ❌ fail | 16.0s |
| 7 | `! rg -n "^from .*\. " reference-backend -g '*.mpl'` | 1 | ❌ fail | 0.11s |
| 8 | `cargo test -q -p mesh-fmt --lib walk_path_preserves_dotted_import_and_impl_paths -- --nocapture` | 0 | ✅ pass | 12.9s |

## Diagnostics

Use `cargo test -q -p meshc --test e2e_fmt fmt_preserves_dotted_paths_exactly -- --nocapture` as the cheapest high-level exact-output signal; it exercises the real `meshc fmt` CLI and will fail with actual rewritten file text if dotted paths regress to `Api. Router` / `Foo. Bar` or if parenthesized multiline imports collapse. Use `cargo test -q -p mesh-fmt --lib snapshot_parenthesized_dotted_from_import -- --nocapture` to isolate library-layer formatting from CLI plumbing. The remaining dogfood drift is still exposed by `cargo run -q -p meshc -- fmt --check reference-backend` and `! rg -n "^from .*\. " reference-backend -g '*.mpl'`.

## Deviations

- I updated `.gsd/milestones/M029/slices/S01/S01-PLAN.md` and `.gsd/milestones/M029/slices/S01/tasks/T02-PLAN.md` before continuing, because the pre-flight checks required explicit observability coverage.
- I implemented the CLI regression as one table-driven exact-output test covering all three required cases instead of three separately named tests.

## Known Issues

- `reference-backend/main.mpl`
- `reference-backend/api/jobs.mpl`
- `reference-backend/api/router.mpl`
- `reference-backend/storage/jobs.mpl`
- `reference-backend/jobs/worker.mpl`

These files still contain spaced dotted imports, so `fmt_check_reference_backend_directory_succeeds`, `cargo run -q -p meshc -- fmt --check reference-backend`, and the stale-import grep remain red until T03 repairs the backend sources.

## Files Created/Modified

- `compiler/mesh-fmt/src/lib.rs` — added dotted-import idempotency and inline snapshot regressions at the library layer.
- `compiler/meshc/tests/e2e_fmt.rs` — added a real `meshc fmt` exact-output helper plus CLI regressions for dotted imports and qualified impl headers.
- `.gsd/milestones/M029/slices/S01/S01-PLAN.md` — added a targeted CLI exact-output verification step and observability note for the new regression surface.
- `.gsd/milestones/M029/slices/S01/tasks/T02-PLAN.md` — added the missing observability-impact section required by pre-flight checks.
- `.gsd/KNOWLEDGE.md` — recorded that `fmt --check` is insufficient when formatter corruption can become idempotent.
