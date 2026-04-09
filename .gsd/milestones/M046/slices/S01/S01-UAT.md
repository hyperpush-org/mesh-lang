# S01: Dual clustered-work declaration — UAT

**Milestone:** M046
**Written:** 2026-03-31T15:46:31.471Z

# S01: Dual clustered-work declaration — UAT

**Milestone:** M046
**Written:** 2026-03-31

## UAT Type

- UAT mode: artifact-driven
- Why this mode is sufficient: S01 is a compiler/parser/LSP slice with no long-running runtime behavior of its own. The truthful acceptance surface is the parser, shared planner, compiler, LSP, and retained manifest regression rails.

## Preconditions

- Run from the repository root with Cargo available.
- No cluster services, env vars, or seeded data are required.
- The workspace should be in a normal dev state with writable `target/`.

## Smoke Test

Run `cargo test -p mesh-pkg m046_s01_ -- --nocapture`.

**Expected:** 5 tests run and pass, proving the shared source-declaration collector/validator accepts source-only clustered work and rejects duplicate, private, and ambiguous targets before any frontend-specific behavior is involved.

## Test Cases

### 1. Parser accepts only the narrow `clustered(work)` item prefix

1. Run `cargo test -p mesh-parser --test parser_tests m046_s01_parser_ -- --nocapture`.
2. Confirm the rail reports `running 10 tests`.
3. Inspect the passing cases for `m046_s01_parser_clustered_work_fn_def`, `m046_s01_parser_clustered_work_invalid_prefix`, `m046_s01_parser_clustered_work_rejects_wrong_target`, and `m046_s01_parser_clustered_work_rejects_prefix_without_fn_or_def`.
4. **Expected:** All 10 tests pass. Valid `clustered(work)` markers appear on `FnDef`, undecorated functions stay unchanged, and malformed prefixes fail closed instead of being treated as ordinary expressions or swallowing the following function body.

### 2. Source-only clustered work reaches the existing declared-handler runtime boundary

1. Run `cargo test -p meshc --test e2e_m046_s01 m046_s01_source_declared_work_llvm_registers_decorated_handler -- --nocapture`.
2. Let the test build its temporary project and inspect emitted LLVM inside the assertion.
3. **Expected:** The test passes and confirms emitted LLVM contains `mesh_register_declared_handler`, `Work.handle_submit`, and `declared_exec_reg___declared_work_work_handle_submit`, proving the source marker converges on the same declared-handler registration surface used by manifest declarations.

### 3. Invalid source declarations fail closed in compiler output

1. Run `cargo test -p meshc --test e2e_m046_s01 m046_s01_ -- --nocapture`.
2. Confirm the rail reports `running 3 tests` and all pass.
3. **Expected:** The private decorated target case passes by asserting a compiler failure before LLVM/codegen with a diagnostic mentioning `source \`clustered(work)\` marker`, `Work.hidden_submit`, and `private function`. The duplicate case passes by asserting an explicit diagnostic mentioning both `mesh.toml` and the source marker for `Work.handle_submit`.

### 4. LSP diagnostics match compiler validation for source-declared work

1. Run `cargo test -p mesh-lsp m046_s01_ -- --nocapture`.
2. Confirm the rail reports `running 3 tests` and all pass.
3. **Expected:** The public source-declared case is diagnostic-clean, while the private-target and duplicate cases surface the same origin-aware failure reasons the compiler emits.

### 5. Manifest-only declared-handler behavior did not regress

1. Run `cargo test -p meshc --test e2e_m044_s01 m044_s01_ -- --nocapture`.
2. Confirm the rail reports `running 15 tests` and all pass.
3. Run `cargo test -p meshc --test e2e_m044_s02 m044_s02_ -- --nocapture`.
4. Confirm the rail reports `running 9 tests` and all pass.
5. **Expected:** The older manifest-only clustered declaration and declared-handler registration surfaces remain green, proving S01 merged source declarations into the existing boundary instead of forking it.

## Edge Cases

### Wrong clustered target stays rejected

1. Use the parser rail above or run `cargo test -p mesh-parser --test parser_tests m046_s01_parser_clustered_work_rejects_wrong_target -- --nocapture`.
2. **Expected:** `clustered(service_call)` is rejected with an unsupported-target error; S01 does not silently widen into a generic decorator system.

### Prefix without `fn|def` does not resynchronize incorrectly

1. Run `cargo test -p mesh-parser --test parser_tests m046_s01_parser_clustered_work_rejects_prefix_without_fn_or_def -- --nocapture`.
2. **Expected:** The parser reports `expected \`fn\` or \`def\` after \`clustered(work)\`` and stops at the item boundary instead of treating the prefix as an expression statement.

## Failure Signals

- The parser rail no longer reports `running 10 tests` with all green results.
- The compiler happy-path rail stops finding `mesh_register_declared_handler` or the declared-work runtime registration/wrapper strings.
- Private or duplicate source-declared work becomes diagnostic-clean in either compiler or LSP output.
- Either retained M044 regression suite turns red, which would indicate the source merge path forked or corrupted the established manifest contract.

## Requirements Proved By This UAT

- R085 — Mesh now supports both manifest and source clustered-work declaration forms and proves they converge on the same declared-handler runtime boundary.

## Not Proven By This UAT

- Runtime-owned startup triggering, route-free status/tooling truth, or failover behavior for clustered work (those belong to later M046 slices).
- A broader decorator/annotation system beyond the narrow `clustered(work)` prefix.
- The older unrelated single-function LLVM verifier failure; this UAT keeps the happy-path compiler proof on the broader M044-shaped fixture for honesty.

## Notes for Tester

These checks are local-only and do not need a running cluster. If the compiler happy-path rail fails only on a smaller source-only fixture but the M044-shaped fixture stays green, treat that as the older LLVM verifier bug noted in the slice summary rather than as evidence that the clustered-work declaration merge path regressed.
