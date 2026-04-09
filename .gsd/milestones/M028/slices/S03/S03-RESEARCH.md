# S03: Daily-Driver Tooling Trust — Research

**Date:** 2026-03-23

## Summary

S03 primarily owns **R006** and directly supports **R008**. The current repo already has real tooling surface area — `meshc fmt`, `meshc test`, `meshc lsp`, a VS Code extension, and website docs — but the trust gap is now concrete instead of abstract: the small happy-path tooling tests are green while the real backend proof target still breaks under normal developer workflows. The biggest live failure is that `meshc fmt --check reference-backend` panics on `reference-backend/api/health.mpl`, which also means LSP formatting is unsafe on the same file because `compiler/mesh-lsp/src/server.rs` calls `mesh_fmt::format_source(...)` directly.

The second major gap is that the documented test workflow is not true. The docs repeatedly teach `meshc test .` / `meshc test <dir>`, but the CLI currently treats `[PATH]` as a **specific `*.test.mpl` file only** and rejects directories. On top of that, `--coverage` is still a success-exit stub (`"Coverage reporting coming soon"`), and `reference-backend/` has **no `*.test.mpl` files at all**, so the canonical backend path still lacks a real Mesh-native test workflow. The LSP is better internally than the slice summary implied — unit tests cover diagnostics, definition, completions, and signature help — but repo-level proof is still shallow because `compiler/meshc/tests/tooling_e2e.rs` only checks `meshc lsp --help`, not real JSON-RPC behavior against backend-shaped code.

## Recommendation

Treat S03 as four linked trust repairs, in this order: **(1) make formatter/LSP formatting safe on the real backend, (2) make `meshc test`’s documented workflow true and attach it to `reference-backend`, (3) add real LSP integration proof, (4) reconcile docs/examples/extensions to the verified behavior**. This matches the project hard rule "work is not done until the relevant verification has passed" and the `debug-like-expert` rule **verify, don’t assume**: I reran the real commands instead of trusting existing docs/tests. It also matches the `review` skill’s guidance to prioritize real bugs over style nits and the `test` skill’s guidance to extend existing conventions instead of inventing new harnesses.

For command truth, prefer **making behavior match the documented daily workflow where the workflow is reasonable** (`meshc test .` / directory support), but prefer **making docs honest where the docs are simply wrong or stale** (`meshc new` vs `meshc init`, `mesh fmt` vs `meshc fmt`, VS Code `.vsix` version drift). For coverage, full line-level instrumentation looks deeper than the rest of S03 because the current implementation is a hard stub in `test_runner.rs`; the minimum credible outcome is at least to stop silently treating stubbed coverage as success and document the honest state against the reference backend. If time permits beyond that, a small real coverage artifact is a bonus, not the first unblocker.

## Requirement Targeting

- **R006 (owner):** directly blocked today by the formatter panic on `reference-backend`, by the false `meshc test .` docs path, by the no-op `--coverage` success path, and by missing repo-level LSP interaction proof.
- **R008 (supports):** directly affected because `README.md`, `website/docs/docs/tooling/index.md`, `website/docs/docs/testing/index.md`, and `website/docs/docs/cheatsheet/index.md` currently describe commands/features that are either stale or not fully proven.

## Implementation Landscape

### Key Files

- `compiler/mesh-fmt/src/printer.rs` — immediate formatter crash locus; `print()` does `col + flat_width` and overflows when `measure_flat()` returns `usize::MAX` for a group that cannot render flat.
- `compiler/mesh-fmt/src/lib.rs` — formatter regression surface; already has idempotency/known-limitation tests and is the natural place for a reduced reproducer or fixture-backed regression.
- `reference-backend/api/health.mpl` — current real-backend formatter reproducer; `meshc fmt --check` panics here first.
- `compiler/meshc/src/main.rs` — CLI contract for `fmt`, `test`, `init`, `lsp`; the `Test` arg help says `[PATH]` is a specific test file even though docs teach directory usage.
- `compiler/meshc/src/test_runner.rs` — current file-only path handling, recursive discovery for omitted-path mode, and the `--coverage` success stub.
- `compiler/meshc/tests/e2e_fmt.rs` — formatter e2e suite; currently passes while missing the real-backend crash case.
- `compiler/meshc/tests/tooling_e2e.rs` — current repo-level tooling smoke coverage; good place to extend command-truth checks, but currently too shallow for LSP trust.
- `compiler/meshc/tests/e2e_reference_backend.rs` — authoritative backend proof target already used by S01/S02; natural place to anchor backend-shaped tooling proofs or reuse helpers.
- `compiler/mesh-lsp/src/server.rs` — actual LSP capability surface; advertises hover, definition, document symbols, formatting, completions, and signature help, and formatting delegates straight to `mesh_fmt::format_source(...)`.
- `compiler/mesh-lsp/src/analysis.rs` — diagnostics/hover plumbing; contains an explicit comment that hover may still be wrong at some positions due to tree/source coordinate mismatch (`analysis.rs:356`).
- `tools/editors/vscode-mesh/src/extension.ts` — VS Code client bootstrap; starts `meshc lsp` and is the real editor entrypoint.
- `tools/editors/vscode-mesh/package.json` — extension version/source of truth (`0.3.0`), currently out of sync with website install docs.
- `tools/editors/vscode-mesh/README.md` — extension feature truth; lists completions, signature help, and document symbols that the website tooling page under-documents.
- `README.md` — public tooling claim drift (`mesh fmt` instead of `meshc fmt`).
- `website/docs/docs/tooling/index.md` — biggest docs drift cluster: `meshc new`, `meshc test .`, stale VS Code `.vsix` name, incomplete/older LSP feature list.
- `website/docs/docs/testing/index.md` — false directory-based test invocation plus coverage-stub narrative.
- `website/docs/docs/cheatsheet/index.md` — also teaches `meshc test .`.
- `reference-backend/README.md` — current canonical backend proof doc; good place to add honest fmt/test/LSP workflow after behavior is fixed.

### Observed Reality / Repros

These were run against this worktree and are the most useful starting points:

```bash
cargo test --manifest-path /Users/sn0w/Documents/dev/mesh-lang/.gsd/worktrees/M028/Cargo.toml -p meshc --test tooling_e2e -- --nocapture
# passes (8 tests) but only shallowly covers CLI/tooling

cargo test --manifest-path /Users/sn0w/Documents/dev/mesh-lang/.gsd/worktrees/M028/Cargo.toml -p meshc --test e2e_fmt -- --nocapture
# passes (6 tests) but misses reference-backend formatter behavior

cargo test --manifest-path /Users/sn0w/Documents/dev/mesh-lang/.gsd/worktrees/M028/Cargo.toml -p meshc --test e2e_reference_backend e2e_reference_backend_builds -- --nocapture
# passes; real backend still builds

cargo run --manifest-path /Users/sn0w/Documents/dev/mesh-lang/.gsd/worktrees/M028/Cargo.toml -p meshc -- fmt --check /Users/sn0w/Documents/dev/mesh-lang/.gsd/worktrees/M028/reference-backend
# panics: compiler/mesh-fmt/src/printer.rs:99:20 attempt to add with overflow

cargo run --manifest-path /Users/sn0w/Documents/dev/mesh-lang/.gsd/worktrees/M028/Cargo.toml -p meshc -- test /Users/sn0w/Documents/dev/mesh-lang/.gsd/worktrees/M028/reference-backend
# error: '.../reference-backend' is not a *.test.mpl file

cargo run --manifest-path /Users/sn0w/Documents/dev/mesh-lang/.gsd/worktrees/M028/Cargo.toml -p meshc -- test --coverage /Users/sn0w/Documents/dev/mesh-lang/.gsd/worktrees/M028/mesher/tests/validation.test.mpl
# prints only: Coverage reporting coming soon

cargo run --manifest-path /Users/sn0w/Documents/dev/mesh-lang/.gsd/worktrees/M028/Cargo.toml -p meshc -- new demo
# error: unrecognized subcommand 'new'

cargo test --manifest-path /Users/sn0w/Documents/dev/mesh-lang/.gsd/worktrees/M028/Cargo.toml -p mesh-lsp -- --nocapture
# passes 43 unit tests; still no transport/editor e2e proof
```

### Natural Task Seams

#### 1. Formatter + LSP formatting hardening

Scope:
- `compiler/mesh-fmt/src/printer.rs`
- `compiler/mesh-fmt/src/lib.rs`
- `compiler/meshc/tests/e2e_fmt.rs` (or a new formatter regression test)
- possibly `compiler/meshc/tests/tooling_e2e.rs` for a CLI-level regression

Why this is its own seam:
- One fix likely unblocks both `meshc fmt` and LSP formatting because both share `mesh_fmt::format_source(...)`.
- The real reproducer already exists: `reference-backend/api/health.mpl`.
- This is the most damaging DX bug because it turns routine formatting into a panic on the milestone’s canonical backend.

What to watch:
- `measure_flat()` intentionally returns `usize::MAX` for hardlines; `print()` currently assumes `col + flat_width` is safe.
- `mesh-fmt/src/lib.rs` already records known limitations (multiline pipes, interface methods); if this fix exposes more unsupported constructs, capture them honestly instead of overclaiming.

#### 2. Test runner contract + backend-native test surface

Scope:
- `compiler/meshc/src/main.rs`
- `compiler/meshc/src/test_runner.rs`
- `compiler/meshc/tests/tooling_e2e.rs`
- potentially `reference-backend/tests/*.test.mpl` (new)
- `reference-backend/README.md`
- `website/docs/docs/testing/index.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/cheatsheet/index.md`

Why this is its own seam:
- The code/docs contract is currently false in a user-visible way.
- `reference-backend` currently has no Mesh-native tests, so the daily-driver `meshc test` story is not anchored to the canonical backend.
- `test_runner.rs` already has recursive discovery helpers; directory-path support is a local behavioral change, not a brand-new subsystem.

What to watch:
- `find_project_dir_for_test()` already walks up from a specific test file to the nearest `main.mpl`; if directory support is added, keep that project-root logic correct.
- Decide early whether `--coverage` should become an explicit unsupported error, a clearly-marked partial artifact, or a real implementation. Do not leave it as a silent success stub.

#### 3. LSP integration proof and editor truth

Scope:
- `compiler/mesh-lsp/src/server.rs`
- `compiler/mesh-lsp/src/analysis.rs`
- `compiler/meshc/tests/tooling_e2e.rs` or a new dedicated LSP integration test
- `tools/editors/vscode-mesh/src/extension.ts`
- `tools/editors/vscode-mesh/README.md`
- `website/docs/docs/tooling/index.md`

Why this is its own seam:
- Internal unit coverage is decent, but the repo lacks a real request/response proof that the language server behaves correctly over JSON-RPC on backend-shaped code.
- The website under-documents features that the extension actually exposes (completions, signature help, document symbols), while also not proving them.

What to watch:
- `analysis.rs:356` explicitly documents a hover-position correctness caveat; if transport-level tests reproduce it on real files, that becomes an execution task, not just a docs task.
- Because `server.rs` formatting path shares the formatter, formatter hardening should land before LSP e2e finalization.

#### 4. Docs / examples / command truth cleanup

Scope:
- `README.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/testing/index.md`
- `website/docs/docs/cheatsheet/index.md`
- `reference-backend/README.md`
- possibly `tools/editors/vscode-mesh/README.md` if feature wording should be normalized

Why this is its own seam:
- Several issues are simple truth mismatches and should be corrected only after code behavior is finalized.
- This work is what converts R006 improvements into R008-visible proof.

Known drift to reconcile:
- `README.md:44` says `mesh fmt`; CLI is `meshc fmt`.
- `website/docs/docs/tooling/index.md:111-114,320` says `meshc new`; CLI is `meshc init`.
- `website/docs/docs/tooling/index.md:164-165`, `website/docs/docs/testing/index.md:13-14`, and `website/docs/docs/cheatsheet/index.md:388` teach directory-based `meshc test` even though the implementation rejects directories.
- `website/docs/docs/tooling/index.md:297` installs `mesh-lang-0.1.0.vsix`, but the extension package is `0.3.0`.
- Website tooling docs list a narrower LSP feature set than `tools/editors/vscode-mesh/README.md` and `compiler/mesh-lsp/src/server.rs`.

### Build Order

1. **Fix the formatter panic on `reference-backend` first.**
   - This is the highest-confidence real failure and it blocks both CLI formatting and editor formatting on the canonical backend.
   - Add a regression that formats either `reference-backend/api/health.mpl` directly or a reduced snippet derived from it.

2. **Make the `meshc test` contract honest and backend-usable next.**
   - Either implement directory-path support (preferred) or explicitly narrow the docs/help everywhere, but do not leave code/docs split.
   - Add at least one `reference-backend`-anchored Mesh test path so `meshc test` is no longer disconnected from the milestone proof target.
   - Resolve the coverage story enough that it is no longer a success-exit placeholder.

3. **Add real LSP integration verification after formatter stability.**
   - Use backend-shaped files and assert actual diagnostics / hover / definition / formatting / completion behavior over live LSP requests, not just unit helpers and `--help`.

4. **Only then sync docs/examples/extension instructions.**
   - The final docs pass should encode verified commands and surfaces, not speculative future behavior.

## Verification

Minimum verification set for slice closure should be tied to the reference backend, not only temp files:

### Formatter / diagnostics

```bash
cargo test --manifest-path /Users/sn0w/Documents/dev/mesh-lang/.gsd/worktrees/M028/Cargo.toml -p meshc --test e2e_fmt -- --nocapture
cargo run --manifest-path /Users/sn0w/Documents/dev/mesh-lang/.gsd/worktrees/M028/Cargo.toml -p meshc -- fmt --check /Users/sn0w/Documents/dev/mesh-lang/.gsd/worktrees/M028/reference-backend
cargo test --manifest-path /Users/sn0w/Documents/dev/mesh-lang/.gsd/worktrees/M028/Cargo.toml -p meshc --test e2e_reference_backend e2e_reference_backend_builds -- --nocapture
```

### Test runner / coverage story

```bash
cargo run --manifest-path /Users/sn0w/Documents/dev/mesh-lang/.gsd/worktrees/M028/Cargo.toml -p meshc -- test --help
cargo run --manifest-path /Users/sn0w/Documents/dev/mesh-lang/.gsd/worktrees/M028/Cargo.toml -p meshc -- test <reference-backend test target>
cargo test --manifest-path /Users/sn0w/Documents/dev/mesh-lang/.gsd/worktrees/M028/Cargo.toml -p meshc --test tooling_e2e -- --nocapture
```

### LSP

```bash
cargo test --manifest-path /Users/sn0w/Documents/dev/mesh-lang/.gsd/worktrees/M028/Cargo.toml -p mesh-lsp -- --nocapture
cargo test --manifest-path /Users/sn0w/Documents/dev/mesh-lang/.gsd/worktrees/M028/Cargo.toml -p meshc --test tooling_e2e -- --nocapture
# or a new dedicated LSP integration test target if created
```

### Docs truth

Manually rerun every documented command that changed, especially:
- `meshc init ...`
- `meshc fmt ...`
- `meshc test ...`
- VS Code extension build/install commands in `tools/editors/vscode-mesh/`
- any new `reference-backend` tooling commands added to docs

## Skill Discovery

No installed skill directly covered the missing Rust/Postgres/LSP-specific implementation work. The most relevant uninstalled skills I found were:

- **Rust:** `npx skills add apollographql/skills@rust-best-practices` or `npx skills add wshobson/agents@rust-async-patterns`
- **PostgreSQL:** `npx skills add wshobson/agents@postgresql-table-design` or `npx skills add github/awesome-copilot@postgresql-optimization`
- **LSP:** `npx skills add anton-abyzov/specweave@lsp-integration`

These are promising if execution needs outside patterns, but none are required just to scope S03.

## Risks / Constraints

- The formatter panic is likely a small code fix with a large blast radius because LSP formatting shares the same path; verify both surfaces together.
- Full coverage instrumentation could sprawl beyond S03 if treated as a greenfield subsystem. Keep the first decision explicit: honest non-support vs partial artifact vs real implementation.
- If directory-path support is added to `meshc test`, be careful not to regress the existing “specific test file” flow or the project-root discovery logic used for nested tests.
- Because the `lint` and `test` skills both stress using existing project conventions, prefer extending `tooling_e2e.rs`, `e2e_fmt.rs`, and `e2e_reference_backend.rs` before creating many brand-new harnesses.
