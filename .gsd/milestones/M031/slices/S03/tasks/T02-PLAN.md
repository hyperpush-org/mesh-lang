---
estimated_steps: 5
estimated_files: 5
skills_used: []
---

# T02: Clean up remaining files, verify full suite, formatter gate

**Slice:** S03 — Reference-Backend Dogfood Cleanup
**Milestone:** M031

## Description

Clean up the 5 remaining files with anti-patterns, then run the full verification gate to prove zero regressions across the entire `reference-backend/`.

### File-by-file changes:

**`api/health.mpl`** (130 lines):
- Remove 4 `== true` — all in Bool-returning condition checks
- Flatten 4 nested if/else chains to `else if` — `is_recovering_status`, `worker_liveness`, `health_status`, `recovery_active`
- Convert the 410-char import on line 1 to parenthesized multiline: `from Jobs.Worker import (\n  fn1,\n  fn2,\n  ...\n)`

**`api/jobs.mpl`** (78 lines):
- Remove 4 `let _ =`

**`storage/jobs.mpl`** (107 lines):
- Remove 2 `let _ =` on `Repo.update_where(...) ?` calls. The bare `Repo.update_where(...) ?` expression still propagates errors via `?` — identical behavior. Keep `<>` in SQL construction per D029.

**`main.mpl`** (91 lines):
- Remove 2 `let _ =`

**`runtime/registry.mpl`** (44 lines):
- Remove 1 `let _ =`

All changes are mechanical and behavior-preserving. After cleanup, run the full verification gate.

## Steps

1. Read and clean `reference-backend/api/health.mpl`: remove 4 `== true`, flatten 4 nested if/else chains, convert long import to multiline.
2. Read and clean `reference-backend/api/jobs.mpl`: remove 4 `let _ =`.
3. Read and clean `reference-backend/storage/jobs.mpl`: remove 2 `let _ =` from `Repo.update_where` calls.
4. Read and clean `reference-backend/main.mpl` (2 `let _ =`) and `reference-backend/runtime/registry.mpl` (1 `let _ =`).
5. Run full verification gate: `cargo run -p meshc -- build reference-backend`, `cargo run -p meshc -- fmt --check reference-backend`, `cargo run -p meshc -- test reference-backend`, `cargo test -p meshc --test e2e`, `rg 'let _ =' reference-backend/ -g '*.mpl'`, `rg '== true' reference-backend/ -g '*.mpl'`.

## Must-Haves

- [ ] Zero `let _ =` across all `reference-backend/*.mpl` files
- [ ] Zero `== true` across all `reference-backend/*.mpl` files
- [ ] `api/health.mpl` import is parenthesized multiline
- [ ] All nested if/else chains in `api/health.mpl` flattened to `else if`
- [ ] `cargo run -p meshc -- build reference-backend` succeeds
- [ ] `cargo run -p meshc -- fmt --check reference-backend` passes
- [ ] `cargo run -p meshc -- test reference-backend` passes
- [ ] `cargo test -p meshc --test e2e` → ≥313 pass

## Verification

- `rg 'let _ =' reference-backend/ -g '*.mpl'` → 0 matches
- `rg '== true' reference-backend/ -g '*.mpl'` → 0 matches
- `cargo run -p meshc -- build reference-backend` → success
- `cargo run -p meshc -- fmt --check reference-backend` → passes
- `cargo run -p meshc -- test reference-backend` → passes
- `cargo test -p meshc --test e2e` → ≥313 pass

## Inputs

- `reference-backend/jobs/worker.mpl` — cleaned by T01 (confirms build still works after T01's changes)
- `reference-backend/api/health.mpl` — 130 lines, 4 `== true`, 4 nested if/else, 1 long import
- `reference-backend/api/jobs.mpl` — 78 lines, 4 `let _ =`
- `reference-backend/storage/jobs.mpl` — 107 lines, 2 `let _ =` on Repo.update_where calls
- `reference-backend/main.mpl` — 91 lines, 2 `let _ =`
- `reference-backend/runtime/registry.mpl` — 44 lines, 1 `let _ =`

## Expected Output

- `reference-backend/api/health.mpl` — cleaned: zero `== true`, `else if` chains, multiline import
- `reference-backend/api/jobs.mpl` — cleaned: zero `let _ =`
- `reference-backend/storage/jobs.mpl` — cleaned: zero `let _ =`
- `reference-backend/main.mpl` — cleaned: zero `let _ =`
- `reference-backend/runtime/registry.mpl` — cleaned: zero `let _ =`
