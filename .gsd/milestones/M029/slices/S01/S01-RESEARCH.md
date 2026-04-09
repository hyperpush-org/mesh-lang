# S01: Formatter dot-path and multiline import fix — Research

**Depth:** Targeted. This is formatter-core Rust work in a known subsystem. The risky part is not broad architecture; it is avoiding the wrong fix because the milestone prose overstates the multiline-import half. I followed the `debug-like-expert` skill’s **VERIFY, DON'T ASSUME** rule and reproduced current formatter behavior before drawing seams.

## Requirements Targeted

### Direct
- **R026** — formatter must preserve module dot-paths and parenthesized multiline imports.
- **R027** — reference-backend source must round-trip through `meshc fmt` without `Api. Router` / `Jobs. Worker` corruption.

### Supported / unblocked
- **R024** — mesher multiline-import cleanup in S03 depends on S01 making parenthesized imports safe to format.
- **R011** — this is real dogfood-driven DX work, not speculative formatter polish.

## Summary

The live bug is narrower than the slice title suggests.

1. **Dot-path corruption is real and currently live.** Formatting a temp file containing:
   - `from Api.Router import build_router`
   - `from Api.Router import (build_router, other_handler)`
   produces:
   - `from Api. Router import build_router`
   - `from Api. Router import ( ... )`

2. **Parenthesized multiline imports already preserve structure on current HEAD.** The same combined repro kept the parenthesized import multiline; only the dotted module path was corrupted. Existing unit tests in `compiler/mesh-fmt/src/walker.rs:2545`, `:2555`, and `:2565` plus e2e tests in `compiler/meshc/tests/e2e.rs:6533`, `:6566`, and `:6590` all pass.

3. **Root cause is `PATH`, not `FROM_IMPORT_DECL` / `IMPORT_LIST` routing.**
   - `SyntaxKind::PATH` is still routed through the generic inline spacer in `compiler/mesh-fmt/src/walker.rs:94` → `walk_tokens_inline` at `:2013`.
   - `needs_space_before` at `:2053` excludes `DOT`, but not the following `IDENT`, so `Foo.Bar` becomes `Foo. Bar` when emitted token-by-token.
   - `FIELD_ACCESS` is already special-cased at `compiler/mesh-fmt/src/walker.rs:58` / `:1458`, which is why `Api.Router.build_router()` stays correct.

4. **The parser side is already correct for parenthesized imports.**
   - `parse_from_import_decl` in `compiler/mesh-parser/src/parser/items.rs:178` already opens an `IMPORT_LIST`, optionally consumes `L_PAREN`, and expects `R_PAREN`.
   - `parse_module_path` is separate at `:239`.
   - `FromImportDecl::import_list()` / `ImportList::names()` in `compiler/mesh-parser/src/ast/item.rs:231` / `:240` expose only `Name` nodes, so paren tokens are ignored by downstream consumers.
   - Earlier milestone suspicion that `walk_from_import_decl` failed to delegate correctly is **not supported** by the current code or repro.

5. **`fmt --check` can hide the bug once bad output is already written.**
   - `cargo run -q -p meshc -- fmt --check reference-backend` is green today because the repo is already in the corrupted state.
   - `compiler/meshc/tests/e2e_fmt.rs:113` only checks that `fmt --check reference-backend` succeeds and does not rewrite `api/health.mpl`; it does **not** assert that the file text is semantically correct.
   - `cargo test -q -p mesh-fmt --lib` also passes (119 tests) despite the bug.

6. **`PATH` is reused beyond imports.** `parse_impl_def` in `compiler/mesh-parser/src/parser/items.rs:794`, `:801`, and `:831` also uses `parse_module_path` for qualified trait/type names. A temp formatter repro showed `impl Foo.Bar for Baz.Qux do end` becomes `impl Foo. Bar for Baz. Qux do end`. So a path-specific fix should be treated as a formatter-core change, not an import-only patch.

## Implementation Landscape

### `compiler/mesh-fmt/src/walker.rs`

This is the main execution seam.

- `SyntaxKind::PATH` currently falls through the generic branch at `:94`.
- `walk_tokens_inline` at `:2013` is global behavior used by many leaf-like nodes.
- `walk_field_access` at `:1458` already demonstrates the correct no-space-around-dot pattern.
- `walk_import_list` at `:1333` already preserves parenthesized multiline imports by emitting one imported name per indented line when parens are present.
- `walk_from_import_decl` at `:1381` appears structurally fine; its child `PATH` node is what introduces the bad spaces.

**Natural fix seam:** add a dedicated `walk_path(...)` and route `SyntaxKind::PATH` to it, mirroring the `walk_field_access(...)` no-space dot handling.

Why this seam is safer than touching the generic spacer:
- it localizes behavior to `PATH` nodes instead of changing token spacing globally
- it matches the existing formatter structure (`FIELD_ACCESS` already has specialized handling)
- it naturally fixes both import module paths and impl trait/type paths

**Alternative:** add look-behind state to `walk_tokens_inline` so `IDENT` after `DOT` does not get a space. This is more fragile because it changes the generic leaf formatter for every node kind that still uses `walk_tokens_inline`.

### `compiler/mesh-parser/src/parser/items.rs` and `compiler/mesh-parser/src/ast/item.rs`

These files matter mainly as evidence that no parser fix is required for S01.

- `parse_from_import_decl` (`items.rs:178`) already supports optional parens in the import list.
- `parse_module_path` (`items.rs:239`) builds the `PATH` node that the formatter is mishandling.
- `parse_impl_def` (`items.rs:794`) reuses `parse_module_path` for qualified trait/type names.
- `FromImportDecl::import_list()` / `ImportList::names()` (`ast/item.rs:231`, `:240`) confirm downstream consumers are insulated from paren tokens.

### Existing formatter / CLI test surface

**Unit coverage that exists today**
- `compiler/mesh-fmt/src/walker.rs:2545` — `from_import_paren_single_line`
- `compiler/mesh-fmt/src/walker.rs:2555` — `from_import_paren_multiline`
- `compiler/mesh-fmt/src/walker.rs:2565` — `from_import_paren_trailing_comma`
- `compiler/mesh-fmt/src/lib.rs:244` — `idempotent_field_access`
- `compiler/mesh-fmt/src/lib.rs:254` — `idempotent_from_import`
- `compiler/mesh-fmt/src/lib.rs:541` — `snapshot_from_import`

**Critical gap**
- there is **no** formatter test for a dotted `PATH` in a `FROM_IMPORT_DECL`
- there is **no** combined regression for `from Foo.Bar import ( ... )`
- there is **no** text-level CLI formatter regression in `meshc` proving that dotted paths stay exact after formatting

### Dogfood files that expose the live bug

`reference-backend/` currently contains 12 corrupted `from` imports:
- `reference-backend/main.mpl`
- `reference-backend/storage/jobs.mpl`
- `reference-backend/api/router.mpl`
- `reference-backend/api/jobs.mpl`
- `reference-backend/api/health.mpl`
- `reference-backend/jobs/worker.mpl`

Important detail: `reference-backend/api/health.mpl:1` is the **only current parenthesized multiline import in the dogfood code**, and it is already corrupted as `from Jobs. Worker import (...)`. That makes it the best real-world smoke target once the formatter fix lands.

For future S03 planning, the current line-length scan found **10** `from ... import ...` lines over 120 chars (9 in `mesher/`, 1 in `reference-backend/jobs/worker.mpl`). Mesher files with current >120-char import lines are:
- `mesher/api/alerts.mpl`
- `mesher/api/dashboard.mpl`
- `mesher/api/team.mpl`
- `mesher/ingestion/routes.mpl`
- `mesher/main.mpl` (4 lines)
- `mesher/services/project.mpl`
- `mesher/services/user.mpl`

## Recommendation

Two tasks are enough for S01.

### T01: Fix `PATH` formatting in `compiler/mesh-fmt/src/walker.rs`

**Scope**
- add a path-specific walker or equivalent localized logic
- route `SyntaxKind::PATH` away from generic `walk_tokens_inline`
- leave `walk_import_list` / `walk_from_import_decl` alone unless the executor proves a second live bug after the path fix

**Why first**
- this is the root cause behind the visible dogfood corruption
- it also unblocks trustworthy S03 multiline-import cleanup
- it is the only code change needed by the current evidence

### T02: Close the regression gap with text-level formatter assertions

At minimum add exact-output tests for:
1. `from Api.Router import build_router`
2. `from Api.Router import (\n  build_router,\n  other_handler\n)`
3. `impl Foo.Bar for Baz.Qux do\nend`

Best locations:
- `compiler/mesh-fmt/src/walker.rs` next to the existing `from_import_paren_*` tests for exact output
- optionally `compiler/mesh-fmt/src/lib.rs` for idempotence/snapshot coverage
- only touch `compiler/meshc/tests/e2e_fmt.rs` if you want a CLI-level regression too; do **not** rely on `fmt --check reference-backend` alone as proof

## Verification

### Authoritative slice gate
- `cargo test -q -p mesh-fmt --lib`

### Keep-green guardrails
- `cargo test -q -p meshc --test e2e e2e_multiline_import_paren -- --nocapture`
  - this currently runs the 3 parenthesized import e2e tests together and passed in research
- `cargo run -q -p meshc -- fmt --check reference-backend`
  - should stay green, but remember this is idempotence only, not semantic truth

### Required semantic proof beyond `--check`
Use a text-level assertion or temp-file repro equivalent to prove exact output. The simplest manual repro is:

```bash
tmpdir=$(mktemp -d .tmp_m029_s01_repro.XXXXXX)
cat > "$tmpdir/main.mpl" <<'EOF'
from Api.Router import (
  build_router,
  other_handler
)

impl Foo.Bar for Baz.Qux do
end
EOF
cargo run -q -p meshc -- fmt "$tmpdir/main.mpl"
```

**Current bad output:** `Api. Router` / `Foo. Bar` / `Baz. Qux`

S01 is only done when the formatted file keeps:
- `from Api.Router import (`
- `impl Foo.Bar for Baz.Qux do`

## Risks and Watchouts

1. **Do not “fix” the parser or import-list walker without new evidence.** Current HEAD already preserves multiline parenthesized imports; churn in `walk_import_list` or `parse_from_import_decl` is likely waste.

2. **Do not use `fmt --check` as the primary proof.** It will happily pass on already-corrupted source after the formatter has normalized it into the wrong state.

3. **Treat this as a formatter-core change.** `PATH` is reused outside imports, especially in `impl` headers. The full `mesh-fmt` lib suite is the correct regression gate.

4. **Reference-backend is already a good smoke target after the fix.** There are 12 corrupted imports to repair, including the only live parenthesized multiline import. S03 should not start the broad mesher import conversion until S01 lands.

## Skills Discovered

### Loaded
- `debug-like-expert` — specifically applied its “VERIFY, DON'T ASSUME” rule to reproduce formatter behavior instead of trusting stale milestone assumptions.

### Searched
- `npx skills find "rust"` surfaced relevant candidates, especially:
  - `apollographql/skills@rust-best-practices` (4.4K installs)
  - `jeffallan/claude-skills@rust-engineer` (1.5K installs)

### Installed
- None. This slice is repo-specific formatter analysis on an established Rust codebase, and the existing local skills (`debug-like-expert`, plus the repo’s normal test/lint workflow) were sufficient.