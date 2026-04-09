# S05: Assembled contract proof and minimal public touchpoints — UAT

**Milestone:** M048
**Written:** 2026-04-02T19:09:12.164Z

# S05: Assembled contract proof and minimal public touchpoints — UAT

**Milestone:** M048  
**Written:** 2026-04-02

## UAT Type

- UAT mode: mixed docs-contract + assembled verifier replay + retained-bundle inspection
- Why this mode is sufficient: S05 did not add new product runtime behavior. It closed the milestone by making the minimal public touchpoints truthful and by composing the retained S01-S04 rails into one diagnosable closeout entrypoint. Acceptance therefore has to prove both the public wording and the assembled proof bundle.

## Preconditions

- Run from the repository root.
- Rust, Node/npm, and Neovim are available locally.
- `target/` and `.tmp/m048-s05/` are writable.
- The retained S01-S04 rails are available in the working tree, including `scripts/verify-m036-s01.sh`, `scripts/verify-m036-s02.sh`, `scripts/tests/verify-m036-s02-contract.test.mjs`, and `scripts/tests/verify-m048-s04-skill-contract.test.mjs`.
- Allow enough wall-clock time for the full assembled replay; it re-runs cargo, editor smoke, docs build, and retained artifact copying.

## Smoke Test

Run:

```bash
node --test scripts/tests/verify-m048-s05-contract.test.mjs
npm --prefix website run build
bash scripts/verify-m048-s05.sh
test "$(cat .tmp/m048-s05/verify/status.txt)" = "ok" && test "$(cat .tmp/m048-s05/verify/current-phase.txt)" = "complete"
```

**Expected:** all four commands pass. The docs contract should fail fast on wording drift, the website build should stay green, the assembled verifier should replay the retained S01-S04 proof rails successfully, and the status/current-phase files should end in `ok` / `complete`.

## Test Cases

### 1. Minimal public touchpoints stay truthful and fail closed on drift

1. Run:
   ```bash
   node --test scripts/tests/verify-m048-s05-contract.test.mjs
   ```
2. **Expected:** all 4 assertions pass.
3. Inspect `README.md`.
4. **Expected:** it teaches `meshc update` / `meshpkg update`, keeps `main.mpl` as the default executable entrypoint, documents optional `[package].entrypoint = "lib/start.mpl"`, and points readers at `bash scripts/verify-m048-s05.sh`.
5. Inspect `website/docs/docs/tooling/index.md`.
6. **Expected:** it includes a short toolchain-update section, an override-entry manifest example, a truthful `meshpkg publish` note about preserving nested project-root-relative `.mpl` paths while excluding hidden/test-only files, a manifest-first editor/grammar note, and the assembled verifier entrypoint.
7. Inspect `tools/editors/vscode-mesh/README.md`.
8. **Expected:** it documents same-file definition plus manifest-first override-entry hover/diagnostics proof, mentions `@cluster` / `@cluster(N)` and both interpolation forms, and does **not** reintroduce `jump to definitions across files`.

### 2. Website docs build still passes after the public-truth edits

1. Run:
   ```bash
   npm --prefix website run build
   ```
2. **Expected:** the VitePress build completes successfully.
3. **Expected:** the build stays compatible with the new README/tooling wording and does not surface markdown or broken-link regressions from the S05 doc edits.

### 3. The assembled S05 verifier replays the retained S01-S04 rails in truthful order

1. Run:
   ```bash
   bash scripts/verify-m048-s05.sh
   ```
2. **Expected:** the script exits 0 and prints `verify-m048-s05: ok`.
3. Inspect `.tmp/m048-s05/verify/phase-report.txt`.
4. **Expected:** the following named phases are present and end in `passed`: `docs-contract`, `m048-s01-entrypoint`, `m048-s02-lsp-neovim`, `m048-s02-vscode`, `m048-s02-publish`, `m048-s03-toolchain-update-core`, `m048-s03-toolchain-update-help`, `m048-s03-toolchain-update-cli`, `m048-s03-toolchain-update-e2e`, `m048-s04-shared-grammar`, `m048-s04-neovim-syntax`, `m048-s04-neovim-contract`, `m048-s04-skill-contract`, `docs-build`, `retain-fixed-m036-artifacts`, `retain-m048-s01-artifacts`, `retain-m048-s03-artifacts`, and `m048-s05-bundle-shape`.
5. **Expected:** the replay reaches the docs build only after the retained S01-S04 rails succeed, so a drift in public touchpoints or an upstream retained rail stops the wrapper early with the failing phase named explicitly.

### 4. Status files and retained bundle pointers stay usable as the top-level diagnostics surface

1. Run:
   ```bash
   cat .tmp/m048-s05/verify/status.txt
   cat .tmp/m048-s05/verify/current-phase.txt
   cat .tmp/m048-s05/verify/latest-proof-bundle.txt
   find .tmp/m048-s05/verify/retained-proof-bundle -maxdepth 1 -mindepth 1 | sort
   ```
2. **Expected:** `status.txt` is `ok` and `current-phase.txt` is `complete` after a green replay.
3. **Expected:** `latest-proof-bundle.txt` points at `.tmp/m048-s05/verify/retained-proof-bundle`.
4. **Expected:** the retained bundle contains the fixed editor artifacts (`retained-m036-s02-lsp`, `retained-m036-s02-syntax`, `retained-m036-s03-vscode-smoke`) and the fresh timestamped M048 snapshots (`retained-m048-s01-artifacts`, `retained-m048-s03-artifacts`) together.

## Edge Cases

### Public wording drift should stop the long replay before cargo/editor/docs work begins

1. Temporarily remove one required marker from one of the three public touchpoints.
2. Run:
   ```bash
   node --test scripts/tests/verify-m048-s05-contract.test.mjs
   ```
3. **Expected:** the Node contract fails immediately and names the missing or banned wording, without requiring the assembled verifier replay.

### VS Code docs must stay bounded to the proved surface

1. Reintroduce the sentence `jump to definitions across files` in `tools/editors/vscode-mesh/README.md`.
2. Run:
   ```bash
   node --test scripts/tests/verify-m048-s05-contract.test.mjs
   ```
3. **Expected:** the contract test fails closed on the stale wording.

### Retained-bundle shape must distinguish fixed M036 artifacts from per-run M048 evidence

1. After a green replay, inspect the top-level retained bundle contents.
2. **Expected:** editor artifacts come from the fixed `.tmp/m036-s02` / `.tmp/m036-s03` directories, while entrypoint and updater evidence come from fresh timestamped `.tmp/m048-s01/*` and `.tmp/m048-s03/*` snapshots.
3. **Expected:** missing any of those required buckets should make the `m048-s05-bundle-shape` phase fail.

## Failure Signals

- `README.md`, tooling docs, or the VS Code README lose required markers for update commands, override entrypoints, grammar truth, or the assembled verifier.
- The VS Code README reintroduces stale cross-file definition claims.
- `bash scripts/verify-m048-s05.sh` no longer runs the docs-contract phase first or stops reporting named phase status under `.tmp/m048-s05/verify/phase-report.txt`.
- `status.txt`, `current-phase.txt`, or `latest-proof-bundle.txt` are missing or inconsistent with the retained bundle on disk.
- The retained bundle stops including the fixed M036 editor artifacts or the fresh M048 entrypoint/update snapshots.

## Requirements Proved By This UAT

- R112 — The default-plus-override executable-entry contract is now validated end to end by the assembled S05 replay and retained proof bundle.
- R113 — The installer-backed update commands remain part of the green assembled replay and the first-contact public docs surface.
- R114 — The grammar and init-skill parity rails remain part of the green assembled replay, while the minimal public touchpoints stay aligned with the bounded proved surface.

## Notes for Tester

If the assembled closeout rail goes red, debug in this order: `scripts/tests/verify-m048-s05-contract.test.mjs`, `.tmp/m048-s05/verify/phase-report.txt`, `.tmp/m048-s05/verify/status.txt` / `current-phase.txt`, `.tmp/m048-s05/verify/latest-proof-bundle.txt`, then the specific retained subtree named by the failing phase. Do not widen the public docs contract or the VS Code README beyond the actually proved surface just to make the docs sound nicer; S05 exists specifically to keep those first-contact surfaces honest.
