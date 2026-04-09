---
id: T01
parent: S03
milestone: M028
provides:
  - Overflow-safe formatter group-fit decisions on backend-shaped documents
  - Real CLI coverage for `meshc fmt --check reference-backend`
key_files:
  - compiler/mesh-fmt/src/printer.rs
  - compiler/mesh-fmt/src/lib.rs
  - compiler/meshc/tests/e2e_fmt.rs
  - reference-backend/api/health.mpl
  - reference-backend/**/*.mpl
key_decisions:
  - Treat `measure_flat(...) == usize::MAX` as a forced broken render instead of letting `col + flat_width` overflow.
  - Normalize the real `reference-backend` tree so the verified `--check` path passes on shipped sources, not just reduced fixtures.
patterns_established:
  - Cover formatter regressions at three levels: printer unit test, formatter regression on the real backend file, and CLI directory-level e2e.
observability_surfaces:
  - `cargo run -p meshc -- fmt --check reference-backend`
  - `printer::tests::grouped_hardline_breaks_instead_of_overflowing`
  - `edge_case_tests::reference_backend_health_file_formats_canonically`
  - `fmt_check_reference_backend_directory_succeeds`
duration: 1h 20m
verification_result: passed
completed_at: 2026-03-23 14:46:58 EDT
blocker_discovered: false
---

# T01: Harden formatter and format-on-save on the reference backend

**Made formatter group-fit overflow-safe and normalized `reference-backend` so `meshc fmt --check reference-backend` now passes on the real backend path.**

## What Happened

I first reproduced the live panic with `cargo run -p meshc -- fmt --check reference-backend` and confirmed the overflow came from `compiler/mesh-fmt/src/printer.rs` doing a plain `col + flat_width` addition after `measure_flat(...)` returned `usize::MAX` for a `Hardline`-containing group.

I fixed that root cause by adding an overflow-safe `group_fits_on_line(...)` predicate that treats `usize::MAX` as “must break” and uses `saturating_sub` instead of unchecked addition. I pinned the low-level behavior with a grouped-hardline unit test in `printer.rs`.

At the formatter crate layer, I added a regression in `compiler/mesh-fmt/src/lib.rs` that formats the real `reference-backend/api/health.mpl` file via `include_str!`, asserts it stays canonical, and asserts the result is idempotent.

At the CLI layer, I extended `compiler/meshc/tests/e2e_fmt.rs` with a real `meshc fmt --check reference-backend` integration test. That surfaced a second honest issue: after the panic fix, the backend tree still was not actually canonical, so `--check` failed with named file paths instead of panicking. I then normalized the `reference-backend/**/*.mpl` tree with the fixed formatter so the live command surface matches the slice contract.

I also applied the required pre-flight plan-file fix by adding an explicit failure-path verification step for the future `--coverage` contract to `S03-PLAN.md`, then marked T01 complete there.

## Verification

Task-level verification passed:
- `cargo test -p mesh-fmt -- --nocapture`
- `cargo test -p meshc --test e2e_fmt -- --nocapture`
- `cargo run -p meshc -- fmt --check reference-backend`

Slice-level verification was also run once at this intermediate boundary. Current status is partial, as expected:
- Passing now: formatter crate tests, formatter e2e tests, live `fmt --check` on `reference-backend`, existing tooling e2e, and `mesh-lsp` unit tests.
- Still failing/pending for later slice tasks: `meshc test reference-backend`, `! cargo run -p meshc -- test --coverage reference-backend`, and `cargo test -p meshc --test e2e_lsp -- --nocapture` (target does not exist yet).

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-fmt -- --nocapture` | 0 | ✅ pass | 1s |
| 2 | `cargo test -p meshc --test e2e_fmt -- --nocapture` | 0 | ✅ pass | 6s |
| 3 | `cargo run -p meshc -- fmt --check reference-backend` | 0 | ✅ pass | 6s |
| 4 | `cargo run -p meshc -- test reference-backend` | 1 | ❌ fail | 6s |
| 5 | `! cargo run -p meshc -- test --coverage reference-backend` | 1 | ❌ fail | 5s |
| 6 | `cargo test -p meshc --test tooling_e2e -- --nocapture` | 0 | ✅ pass | 6s |
| 7 | `cargo test -p meshc --test e2e_lsp -- --nocapture` | 101 | ❌ fail | 1s |
| 8 | `cargo test -p mesh-lsp -- --nocapture` | 0 | ✅ pass | 13s |

## Diagnostics

Future agents can inspect this work from three angles:
- Run `cargo run -p meshc -- fmt --check reference-backend` to verify the real backend tree stays formatted and that failures now report file paths instead of panicking.
- Run `cargo test -p mesh-fmt -- --nocapture` to exercise the low-level overflow regression (`grouped_hardline_breaks_instead_of_overflowing`) and the real backend formatter regression (`reference_backend_health_file_formats_canonically`).
- Run `cargo test -p meshc --test e2e_fmt -- --nocapture` to verify the CLI formatter path on the real `reference-backend` directory.

## Deviations

The task plan focused on `reference-backend/api/health.mpl`, but after fixing the overflow the honest runtime output showed the broader `reference-backend` tree was still not canonical. I normalized the full backend Mesh source tree so the required live command `meshc fmt --check reference-backend` passes instead of merely avoiding the panic on one file.

## Known Issues

- `meshc test reference-backend` still rejects directory targets and only accepts `*.test.mpl` files. That remains for T02.
- `meshc test --coverage reference-backend` still exits green with the placeholder `Coverage reporting coming soon`, so the new failure-path verification intentionally still fails. That also remains for T02.
- `cargo test -p meshc --test e2e_lsp -- --nocapture` still fails because the `e2e_lsp` target has not been created yet. That remains for T03.

## Files Created/Modified

- `compiler/mesh-fmt/src/printer.rs` — replaced unsafe group-fit arithmetic with an overflow-safe fit check and added a grouped-hardline regression.
- `compiler/mesh-fmt/src/lib.rs` — added a formatter regression that exercises the real `reference-backend/api/health.mpl` file and asserts canonical/idempotent output.
- `compiler/meshc/tests/e2e_fmt.rs` — added a CLI integration test for `meshc fmt --check reference-backend`.
- `reference-backend/**/*.mpl` — normalized the shipped backend Mesh source tree to canonical formatter output so the live `--check` path passes.
- `.gsd/milestones/M028/slices/S03/S03-PLAN.md` — added a slice-level failure-path verification step and marked T01 done.
