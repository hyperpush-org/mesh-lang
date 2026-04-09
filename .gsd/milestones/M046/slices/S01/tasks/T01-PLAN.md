---
estimated_steps: 3
estimated_files: 7
skills_used:
  - rust-best-practices
---

# T01: Add fail-closed `clustered(work)` parser intake

**Slice:** S01 — Dual clustered-work declaration
**Milestone:** M046

## Description

Introduce the narrow source declaration form without widening Mesh into a general annotation system. Keep the accepted syntax to a contextual `clustered(work)` item prefix immediately before `fn|def`, preserve the existing `@` lexer rejection, and expose the marker through `FnDef` so later planning code can consume it deterministically.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `compiler/mesh-parser/src/parser/mod.rs` item dispatch | Emit a targeted parse error and stop before the following function body is swallowed. | N/A | Reject the prefix as invalid item syntax instead of silently treating it as a plain expression statement. |
| `compiler/mesh-parser/src/parser/items.rs` function parser | Keep undecorated functions on the existing path and recover to the next item boundary. | N/A | Leave the decorator accessor empty rather than fabricating clustered metadata. |
| `compiler/mesh-parser/src/ast/item.rs` `FnDef` accessor | Return `None` when the prefix is absent or malformed. | N/A | Keep malformed nodes observable through parser tests instead of hiding them. |

## Load Profile

- **Shared resources**: parser event stream and snapshot tree output.
- **Per-operation cost**: one linear contextual-prefix parse before the existing function-definition parse.
- **10x breakpoint**: the real risk is cascade errors from losing item sync after malformed decorator input, not raw throughput.

## Negative Tests

- **Malformed inputs**: missing `(` or `)`, missing `work`, wrong payload like `clustered(service_call)`, and decorator prefixes not followed by `fn|def`.
- **Error paths**: malformed decorator forms surface parser errors instead of falling through as stray expressions or corrupting the following function body.
- **Boundary conditions**: decorated `pub fn`, decorated private `fn`, decorated `def`, undecorated `fn`, and mixed files containing decorated and undecorated functions.

## Steps

1. Add the minimal composite syntax node(s) and item-dispatch branch needed to recognize contextual `clustered(work)` before `fn|def`, without introducing `@` or reserving `clustered` globally.
2. Extend `parse_fn_def` and `FnDef` AST accessors so the marker is represented as optional metadata on an otherwise ordinary function definition.
3. Add parser snapshots and AST-focused tests that lock the valid syntax and fail-closed recovery behavior.

## Must-Haves

- [ ] Only the narrow `clustered(work)` item prefix is accepted in S01.
- [ ] Undecorated functions continue to parse exactly as before.
- [ ] Malformed decorator forms fail closed with targeted parser errors.
- [ ] `FnDef` exposes a stable accessor the compiler/LSP task can consume.

## Verification

- `cargo test -p mesh-parser --test parser_tests m046_s01_parser_ -- --nocapture`
- `test -f compiler/mesh-parser/tests/snapshots/parser_tests__m046_s01_parser_clustered_work_fn_def.snap`

## Observability Impact

- Signals added/changed: parse errors specific to malformed `clustered(work)` input and snapshot-visible CST shape for decorated functions.
- How a future agent inspects this: rerun `cargo test -p mesh-parser --test parser_tests m046_s01_parser_ -- --nocapture` and inspect the new snapshots.
- Failure state exposed: decorator-node drift or recovery loss shows up as a parser test failure instead of a silent downstream build mismatch.

## Inputs

- `compiler/mesh-parser/src/parser/mod.rs` — current top-level item dispatch only recognizes contextual `from` and raw `fn|def` items.
- `compiler/mesh-parser/src/parser/items.rs` — existing `parse_fn_def(...)` path with no decorator hook.
- `compiler/mesh-parser/src/ast/item.rs` — current `FnDef` accessors available to downstream compiler and LSP code.
- `compiler/mesh-parser/src/syntax_kind.rs` — parser/composite node kinds that will hold the new marker.
- `compiler/mesh-parser/tests/parser_tests.rs` — existing parser snapshot and AST assertion harness.

## Expected Output

- `compiler/mesh-parser/src/parser/mod.rs` — item dispatch recognizes the narrow `clustered(work)` prefix.
- `compiler/mesh-parser/src/parser/items.rs` — function parsing records optional clustered-work metadata without changing undecorated parsing.
- `compiler/mesh-parser/src/ast/item.rs` — `FnDef` exposes an accessor for the parsed clustered-work marker.
- `compiler/mesh-parser/src/syntax_kind.rs` — composite syntax kind(s) exist for the new marker.
- `compiler/mesh-parser/tests/parser_tests.rs` — parser snapshots and AST assertions cover valid and malformed decorator forms.
- `compiler/mesh-parser/tests/snapshots/parser_tests__m046_s01_parser_clustered_work_fn_def.snap` — valid decorated function CST proof.
- `compiler/mesh-parser/tests/snapshots/parser_tests__m046_s01_parser_clustered_work_invalid_prefix.snap` — malformed decorator recovery proof.
