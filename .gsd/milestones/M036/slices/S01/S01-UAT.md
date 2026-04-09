# S01: Corpus-backed syntax parity for the shared VS Code/docs surface — UAT

**Milestone:** M036
**Written:** 2026-03-28T05:05:17.066Z

## UAT: Corpus-backed syntax parity for the shared VS Code/docs surface

### Preconditions
- Repo checkout contains the S01 changes.
- Rust/Cargo dependencies build successfully for the workspace.
- `website/node_modules` is present so the TextMate and Shiki loaders used by the parity test can resolve.

### Test 1: Full shared-surface verifier stays green
1. Run `bash scripts/verify-m036-s01.sh`.
2. Observe the phase banners.

**Expected:**
- The `compiler-truth` phase runs `cargo test -p mesh-lexer string_interpolation -- --nocapture` and passes.
- The `shared-surface-parity` phase runs `node --test website/scripts/tests/verify-m036-s01-syntax-parity.test.mjs` and reports `pass 3`, `fail 0`.
- No drift output is printed.

### Test 2: The audited corpus includes both real repo snippets and the dedicated edge fixture
1. Open `scripts/fixtures/m036-s01-syntax-corpus.json`.
2. Confirm it contains cases pointing at `mesher/main.mpl`, `reference-backend/main.mpl`, docs markdown files under `website/docs/docs/`, `tests/fixtures/interpolation.mpl`, and `scripts/fixtures/m036-s01/interpolation_edge_cases.mpl`.
3. Confirm the edge fixture cases include both `expectedForms: ["dollar"]` and `expectedForms: ["hash"]`, plus one `expectNoInterpolation: true` plain-string control.

**Expected:**
- The corpus is clearly non-toy and names each case with a stable `id` plus source path/line range.
- The plain-string control exists so false-positive interpolation scopes remain visible.

### Test 3: Shared grammar contract covers both interpolation syntaxes and string kinds
1. Open `tools/editors/vscode-mesh/syntaxes/mesh.tmLanguage.json`.
2. Confirm the shared interpolation repository rule recognizes both `#{` and `${` and is reused by both the double-quoted and triple-quoted string rules.
3. Confirm interpolation bodies include recursive nested-brace handling before `source.mesh`.

**Expected:**
- `#{...}` and `${...}` are modeled by the same shared rule instead of separate drifting implementations.
- Nested map/object braces inside an interpolation body do not terminate the outer interpolation early.

### Test 4: Docs and VS Code surfaces describe only the verified contract
1. Open `tools/editors/vscode-mesh/README.md`.
2. Open `tools/editors/vscode-mesh/CHANGELOG.md`.
3. Open `website/docs/docs/tooling/index.md`.

**Expected:**
- The README and tooling docs explicitly describe the shared TextMate grammar and mention verified support for `#{...}` and `${...}` in double- and triple-quoted strings.
- The changelog records the shared interpolation-parity repair.
- None of these surfaces claim broader syntax behavior than the corpus-backed verifier currently covers.

### Test 5: Edge-case fixture remains covered end to end
1. Open `scripts/fixtures/m036-s01/interpolation_edge_cases.mpl`.
2. Confirm it contains:
   - a triple-quoted `${Map.get(meta, {id: 1})}` case,
   - a double-quoted `#{Map.get(meta, {id: 1})}` case,
   - a plain double-quoted string with no interpolation.
3. Re-run `node --test website/scripts/tests/verify-m036-s01-syntax-parity.test.mjs` if needed.

**Expected:**
- The triple-quoted and double-quoted nested-brace cases both pass under TextMate and Shiki.
- The plain-string control does not acquire interpolation scopes.

### Edge Cases
- If `website/node_modules` is missing or the shared grammar path drifts, the parity test must fail closed with a named missing-dependency or missing-grammar error.
- If a corpus case selects empty lines or omits `expectedForms`, the Node test must fail closed with the named corpus case id/path.
- If a future grammar change regresses one engine only, the verifier must print localized `engine/file/case/form` drift output instead of silently passing on engine-to-engine comparison alone.
