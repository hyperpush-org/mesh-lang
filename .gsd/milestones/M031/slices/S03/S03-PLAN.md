# S03: Reference-Backend Dogfood Cleanup

**Goal:** Eliminate all workaround patterns from `reference-backend/` — zero `let _ =`, zero `== true`, struct update syntax replaces full reconstruction, `else if` replaces nested chains, long import goes multiline.
**Demo:** `rg 'let _ =' reference-backend/ -g '*.mpl'` returns 0 results. `rg '== true' reference-backend/ -g '*.mpl'` returns 0 results. `cargo run -p meshc -- build reference-backend` succeeds. `cargo run -p meshc -- fmt --check reference-backend` passes. Full e2e suite passes at ≥313 tests.

## Must-Haves

- All 53 `let _ =` bindings removed — bare expression statements throughout
- All 15 `== true` comparisons removed — direct Bool usage with `if fn_call() do`
- All 8 `WorkerState { ... }` full reconstructions replaced with `%{state | field: value}` struct update
- All ~7 nested `if/else/if` chains flattened to `else if`
- 410-char import in `api/health.mpl` converted to parenthesized multiline import
- `<>` kept in `storage/jobs.mpl` SQL construction per D029
- All existing e2e tests pass (≥313)
- `meshc fmt --check reference-backend` passes

## Verification

- `rg 'let _ =' reference-backend/ -g '*.mpl'` → 0 matches
- `rg '== true' reference-backend/ -g '*.mpl'` → 0 matches
- `cargo run -p meshc -- build reference-backend` → success
- `cargo run -p meshc -- fmt --check reference-backend` → passes
- `cargo run -p meshc -- test reference-backend` → passes
- `cargo test -p meshc --test e2e` → ≥313 pass
- `cargo run -p meshc -- build reference-backend && echo "build-ok"` → confirms no codegen regression from refactoring

## Tasks

- [x] **T01: Clean up worker.mpl — bare expressions, struct update, else-if, Bool conditions** `est:45m`
  - Why: worker.mpl contains 80% of the anti-patterns (44 `let _ =`, 11 `== true`, 8 struct reconstructions, 3 nested if/else chains). It's the largest file and the most sensitive to e2e test regressions.
  - Files: `reference-backend/jobs/worker.mpl`
  - Do: (1) Remove all 44 `let _ =` prefixes — bare expressions for side effects. (2) Remove all 11 `== true` — use direct Bool in conditions, converting `if fn_call() == true do` to `if fn_call() do`. (3) Replace all 8 full `WorkerState { ... }` 18-field reconstructions with `%{state | changed_fields: values}` struct update. (4) Flatten 3 nested if/else chains to `else if`. (5) Verify the file compiles: `cargo run -p meshc -- build reference-backend`.
  - Verify: `cargo run -p meshc -- build reference-backend` succeeds; `rg 'let _ =' reference-backend/jobs/worker.mpl` returns 0; `rg '== true' reference-backend/jobs/worker.mpl` returns 0; `rg 'WorkerState \{' reference-backend/jobs/worker.mpl` returns only the struct definition (1 match).
  - Done when: worker.mpl has zero `let _ =`, zero `== true`, zero full struct reconstructions, and the backend compiles clean.

- [x] **T02: Clean up remaining files, verify full suite, formatter gate** `est:30m`
  - Why: 5 more files need cleanup (9 total `let _ =`, 4 `== true`, 4 nested if/else chains, 1 long import). Then the full verification gate proves zero regressions.
  - Files: `reference-backend/api/health.mpl`, `reference-backend/api/jobs.mpl`, `reference-backend/storage/jobs.mpl`, `reference-backend/main.mpl`, `reference-backend/runtime/registry.mpl`
  - Do: (1) `api/health.mpl`: Remove 4 `== true`, flatten 4 nested if/else chains to `else if`, convert the 410-char import to parenthesized multiline. (2) `api/jobs.mpl`: Remove 4 `let _ =`. (3) `storage/jobs.mpl`: Remove 2 `let _ =` on `Repo.update_where(...) ?` calls — bare `Repo.update_where(...) ?` is fine, the `?` still propagates errors. (4) `main.mpl`: Remove 2 `let _ =`. (5) `runtime/registry.mpl`: Remove 1 `let _ =`. (6) Run full verification gate: build, fmt --check, test, full e2e suite, anti-pattern grep.
  - Verify: `rg 'let _ =' reference-backend/ -g '*.mpl'` → 0; `rg '== true' reference-backend/ -g '*.mpl'` → 0; `cargo run -p meshc -- build reference-backend` → success; `cargo run -p meshc -- fmt --check reference-backend` → passes; `cargo run -p meshc -- test reference-backend` → passes; `cargo test -p meshc --test e2e` → ≥313 pass.
  - Done when: All anti-pattern greps return 0. Build, formatter, tests, and full e2e suite pass.

## Observability / Diagnostics

This slice is a strictly behavior-preserving refactoring — no runtime signals change. The existing log messages, JSON output, and `/health` status surface remain identical. A future agent can verify the cleanup held by re-running the anti-pattern greps and confirming the e2e suite passes at the same count.

**Failure visibility:** If a transformation breaks runtime behavior, the e2e test suite (`cargo test -p meshc --test e2e`) will catch it — the reference-backend tests exercise worker boot, claim, recovery, crash, and multi-instance flows.

## Files Likely Touched

- `reference-backend/jobs/worker.mpl`
- `reference-backend/api/health.mpl`
- `reference-backend/api/jobs.mpl`
- `reference-backend/storage/jobs.mpl`
- `reference-backend/main.mpl`
- `reference-backend/runtime/registry.mpl`
