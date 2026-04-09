# S03 UAT: Reference-Backend Dogfood Cleanup

## Preconditions

- Mesh compiler builds successfully (`cargo build -p meshc`)
- `reference-backend/` directory exists with all `.mpl` source files
- Rust toolchain available for `cargo test`

## Test Cases

### TC1: Zero `let _ =` in reference-backend

**Steps:**
1. Run `rg 'let _ =' reference-backend/ -g '*.mpl'`

**Expected:** Exit code 1 (no matches). Zero occurrences across all `.mpl` files.

### TC2: Zero `== true` in reference-backend

**Steps:**
1. Run `rg '== true' reference-backend/ -g '*.mpl'`

**Expected:** Exit code 1 (no matches). Zero occurrences across all `.mpl` files.

### TC3: No full WorkerState struct reconstructions

**Steps:**
1. Run `rg 'WorkerState \{' reference-backend/jobs/worker.mpl`

**Expected:** Exactly 1 match — the struct definition/initial construction in the `init` function. No 15-field reconstruction blocks elsewhere.

### TC4: else if chains present (not nested if/else)

**Steps:**
1. Run `rg -n 'else if' reference-backend/ -g '*.mpl'`
2. Run `rg -n 'else\n\s+if' reference-backend/ -g '*.mpl' --multiline`

**Expected:** Step 1 returns multiple matches (the flattened chains). Step 2 returns 0 matches (no nested patterns remain).

### TC5: Multiline import in api/health.mpl

**Steps:**
1. Run `rg -A5 'from .* import \(' reference-backend/api/health.mpl`

**Expected:** Shows a parenthesized multiline import with one name per line. No single-line import exceeding ~80 characters.

### TC6: reference-backend builds clean

**Steps:**
1. Run `cargo run -p meshc -- build reference-backend`

**Expected:** Exit code 0. Output shows `Compiled: reference-backend/reference-backend`.

### TC7: Formatter passes

**Steps:**
1. Run `cargo run -p meshc -- fmt --check reference-backend`

**Expected:** Exit code 0. Output shows `11 file(s) already formatted`.

### TC8: Project tests pass

**Steps:**
1. Run `cargo run -p meshc -- test reference-backend`

**Expected:** Exit code 0. At least 2 tests pass.

### TC9: Full e2e suite — no regressions

**Steps:**
1. Run `cargo test -p meshc --test e2e`

**Expected:** At least 313 tests pass. The only failures are the 10 pre-existing try-operator tests (`e2e_cross_module_try_operator`, `e2e_err_binding_pattern`, `e2e_from_try_error_conversion`, `e2e_option_field_extraction`, `e2e_try_chained_result`, `e2e_try_operator_result`, `e2e_try_option_some_path`, `e2e_try_result_binding_arity`, `e2e_try_result_ok_path`, `e2e_tryfrom_try_operator`). No new failures.

### TC10: SQL concatenation preserved in storage/jobs.mpl

**Steps:**
1. Run `rg '<>' reference-backend/storage/jobs.mpl`

**Expected:** At least 1 match. `<>` concatenation is intentionally preserved for SQL construction per D029.

## Edge Cases

### EC1: Struct update only changes relevant fields

**Steps:**
1. Open `reference-backend/jobs/worker.mpl` and find `%{state |` patterns
2. Verify each update only lists the fields that actually change for that state transition

**Expected:** No `%{state | ...}` update lists all 15+ fields. Each should list 1-4 fields that are semantically relevant to the transition.

### EC2: Bare expression statements include error-propagating calls

**Steps:**
1. Run `rg 'Repo\.update_where.*\?' reference-backend/storage/jobs.mpl`

**Expected:** The `?` error propagation operator is present on bare `Repo.update_where(...)` calls — confirming that removing `let _ =` preserved error propagation semantics.
