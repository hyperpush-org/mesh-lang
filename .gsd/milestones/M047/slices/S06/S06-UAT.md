# S06: Docs, migration, and assembled proof closeout — UAT

**Milestone:** M047
**Written:** 2026-04-01T22:30:48.067Z

# S06: Docs, migration, and assembled proof closeout — UAT

**Milestone:** M047
**Written:** 2026-04-01

## UAT Type

- UAT mode: mixed (artifact inspection + contract rails + delegated runtime evidence)
- Why this mode is sufficient: S06 only counts as done if the built-package SQLite regression, the public docs/migration contract, and the delegated native+Docker Todo proof all survive under one retained closeout bundle.

## Preconditions

- Run from the repo root.
- Rust workspace dependencies are installed.
- Docker is available locally because the S06 wrapper replays the S05 Todo/native+container rail.
- `npm --prefix website install` has already been satisfied for the docs build.
- Do **not** run `scripts/verify-m047-s05.sh` and `scripts/verify-m047-s06.sh` in parallel; they intentionally rewrite `.tmp/m047-s05/verify/` and `.tmp/m047-s06/verify/` trees.

## Smoke Test

1. Run `cargo test -p meshc --test e2e_sqlite_built_package -- --nocapture`.
2. Run `cargo test -p meshc --test e2e_m047_s06 -- --nocapture`.
3. Run `bash scripts/verify-m047-s06.sh`.
4. **Expected:** all three commands pass, `.tmp/m047-s06/verify/status.txt` becomes `ok`, `.tmp/m047-s06/verify/current-phase.txt` becomes `complete`, and `.tmp/m047-s06/verify/latest-proof-bundle.txt` points at `.tmp/m047-s06/verify/retained-proof-bundle`.

## Test Cases

### 1. Built-package SQLite execute/query regression stays green

1. Run `cargo test -p meshc --test e2e_sqlite_built_package -- --nocapture`.
2. Locate the newest `.tmp/m047-s06/sqlite-built-package-execute-*` directory.
3. Open `run.stdout.log` in that directory.
4. **Expected:** it contains these lines in order: `schema=ok`, `insert=1`, `count=1`, `mismatch_err=column index out of range`, `done`.
5. Inspect the retained bundle contents under `package/` and the sibling logs.
6. **Expected:** the emitted built package, run logs, and mismatch evidence are all retained so a future agent can debug the AOT seam without rerunning the broader Todo harness.

### 2. Docs and migration contract stays source-first

1. Run `cargo test -p meshc --test e2e_m047_s06 -- --nocapture`.
2. Open `README.md`, `website/docs/docs/tooling/index.md`, `website/docs/docs/getting-started/clustered-example/index.md`, `website/docs/docs/distributed-proof/index.md`, and `website/docs/docs/distributed/index.md`.
3. **Expected:** each surface references `bash scripts/verify-m047-s04.sh`, `bash scripts/verify-m047-s05.sh`, and `bash scripts/verify-m047-s06.sh`.
4. **Expected:** each surface includes migration guidance from `clustered(work)` / `[cluster]` / `execute_declared_work(...)` / `Work.execute_declared_work` to ordinary source-first verbs such as `add()` or `sync_todos()`.
5. **Expected:** each surface references `meshc init --template todo-api` as the fuller starter layered on top of the same route-free `@cluster` contract.
6. **Expected:** every mention of `HTTP.clustered(...)` is paired with the statement that it is still not shipped.

### 3. The assembled verifier owns the final closeout bundle

1. Run `bash scripts/verify-m047-s06.sh`.
2. Open `.tmp/m047-s06/verify/status.txt`, `.tmp/m047-s06/verify/current-phase.txt`, `.tmp/m047-s06/verify/phase-report.txt`, and `.tmp/m047-s06/verify/latest-proof-bundle.txt`.
3. **Expected:** `status.txt` is `ok`, `current-phase.txt` is `complete`, and `phase-report.txt` contains pass markers for `contract-guards`, `m047-s05-replay`, `retain-m047-s05-verify`, `m047-s06-e2e`, `m047-s06-docs-build`, `m047-s06-artifacts`, and `m047-s06-bundle-shape`.
4. Follow the path from `latest-proof-bundle.txt`.
5. **Expected:** it opens `.tmp/m047-s06/verify/retained-proof-bundle/`, which contains both `retained-m047-s05-verify/` and `retained-m047-s06-artifacts/`.
6. Open `.tmp/m047-s06/verify/retained-m047-s06-artifacts.manifest.txt`.
7. **Expected:** it lists exactly one `docs-authority-contract-*`, one `rail-layering-contract-*`, and one `verifier-contract-*` artifact directory.

### 4. Delegated S05 runtime proof is preserved, not hidden

1. Open `.tmp/m047-s06/verify/retained-m047-s05-verify/status.txt`, `.tmp/m047-s06/verify/retained-m047-s05-verify/current-phase.txt`, `.tmp/m047-s06/verify/retained-m047-s05-verify/phase-report.txt`, and `.tmp/m047-s06/verify/retained-m047-s05-verify/latest-proof-bundle.txt`.
2. **Expected:** the retained verifier finishes `ok` / `complete` and still shows pass markers for `m047-s04-replay`, `retain-m047-s04-verify`, `m047-s05-pkg`, `m047-s05-tooling`, `m047-s05-e2e`, `m047-s05-docs-build`, `retain-m047-s05-artifacts`, and `m047-s05-bundle-shape`.
3. Follow the retained S05 bundle pointer.
4. **Expected:** native/container Todo evidence remains inspectable there, including health/CRUD snapshots, `docker-output.file.txt`, `docker-inspect.json`, container stdout/stderr, and persisted SQLite artifacts.
5. **Expected:** S06 copied that state into its own bundle instead of writing new assembled-proof files back into `.tmp/m047-s05/verify/`.

### 5. Docs build result stays attached to the closeout rail

1. Open `.tmp/m047-s06/verify/11-m047-s06-docs-build.log` after a successful wrapper run.
2. **Expected:** it ends with `build complete`, and the enclosing wrapper still finishes `ok`.
3. **Expected:** future docs regressions fail the S06 wrapper instead of silently drifting away from the final closeout rail.

## Edge Cases

### Legacy helper names reappear in public docs

1. Inspect the public docs surfaces for `execute_declared_work(...)` and `Work.execute_declared_work`.
2. **Expected:** those strings appear only inside migration guidance, never as the current public example shape; `e2e_m047_s06` fails if they become present-tense authority.

### `HTTP.clustered(...)` is overclaimed

1. Inspect the public docs surfaces for `HTTP.clustered(...)`.
2. **Expected:** every mention says it is still not shipped; `e2e_m047_s06` fails if docs imply the wrapper exists now.

### Malformed delegated handoff

1. Inspect `.tmp/m047-s06/verify/retained-m047-s05-verify/`.
2. **Expected:** it contains `status.txt`, `current-phase.txt`, `phase-report.txt`, `full-contract.log`, and `latest-proof-bundle.txt`; if any are missing or the pointer is empty, S06 should fail closed instead of claiming assembled success.

### Non-Linux Docker truth

1. If validating on macOS or Windows, inspect the retained S05 bundle for Linux prebuild evidence before trusting container runtime proof.
2. **Expected:** the Docker story remains `build Linux ./output first, then package it`, not `host-native docker build success alone proves runtime truth`.

## Notes for Tester

- S06 is the final docs/proof wrapper, not a new clustered HTTP feature slice. The truthful public story remains route-free `@cluster` surfaces plus the fuller Todo starter.
- When something goes red, start with `.tmp/m047-s06/verify/phase-report.txt` and `.tmp/m047-s06/verify/latest-proof-bundle.txt`, then inspect the copied S05 verifier tree before leaf artifacts.
