# S06 Research — Docs, migration, and assembled proof closeout

## Summary

S06 is not a pure docs-polish slice anymore. The repo already has most of the final-closeout shape, but the last honest proof surface is still blocked lower in the stack.

Key findings:

1. **`scripts/verify-m047-s05.sh` is already the lower-level assembled rail** for this milestone. It replays the S04 cutover verifier, the Todo scaffold unit/tooling/e2e rails, and the VitePress build, then retains an artifact bundle under `.tmp/m047-s05/verify/`.
2. **The current red blocker is not Todo-template-specific.** I reproduced `cargo test -p meshc --test e2e_m047_s05 -- --nocapture`, and the generated app still exits before `/health` with `[todo-api] Database init failed: bad parameter or other API misuse`.
3. **The SQLite failure is broader than the scaffold.** A minimal built Mesh package under `.tmp/m047-s06-scout/sqlite-file-repro/` reproduces the same failure with `Sqlite.execute(db, "CREATE TABLE t (id INTEGER)", [])?` after `meshc build`; a simpler `Sqlite.open`/`Sqlite.close` binary succeeds. That points at a built-binary/AOT `Sqlite.execute` seam, not Todo routing or docs drift.
4. **Public docs are still split between S04 cutover truth and final M047 closeout.** `README.md` and `website/docs/docs/tooling/index.md` mention `meshc init --template todo-api`, but `website/docs/docs/getting-started/clustered-example/index.md`, `website/docs/docs/distributed-proof/index.md`, and `website/docs/docs/distributed/index.md` still frame only the three route-free surfaces and still point only to `bash scripts/verify-m047-s04.sh` as the live M047 authority.
5. **There is no final S06 closeout rail yet.** `scripts/verify-m047-s06.sh` and `compiler/meshc/tests/e2e_m047_s06.rs` do not exist.
6. **Current Todo proof stops at `docker build`.** `compiler/meshc/tests/e2e_m047_s05.rs` builds the generated Docker image but never `docker run`s it. The milestone language says the scaffold Dockerfile should build and run, so final closeout still needs a container-runtime proof.
7. **Important planning correction:** `.gsd/milestones/M047/slices/S06/S06-PLAN.md` says “exercise the Todo API and clustered routes,” but the milestone context, S03 blocker, S04/S05 docs, and current code all still say `HTTP.clustered(...)` is unshipped. S06 must prove ordinary `HTTP.on_*` Todo routes plus ordinary `@cluster` startup work, not clustered route wrappers.

## Requirements Focus

Primary ownership/support for this slice:

- **R104** — the Todo scaffold only becomes milestone-truthful when the generated project can be regenerated, built, exercised, and containerized end to end.
- **R105** — the final docs/proof story must present the Todo template as a starter, not as a proof-harness or fake clustered-route demo.
- **R106** — docs, README, verifier references, and migration guidance need one coherent source-first `@cluster` story across route-free examples and the fuller Todo starter.

Requirements S06 must preserve but should not reopen:

- **R102 / R103** — legacy `clustered(work)` / manifest clustering remain cut off, and repo-owned examples stay on ordinary `@cluster` names.
- **R099** — clustering remains a general function capability; S06 must not rewrite the story into an HTTP-only `todo-api` narrative.

Requirements S06 should **not** claim:

- **R100 / R101** — route-local `HTTP.clustered(...)` is still absent. The final closeout must stay explicit about that.

## Skills Discovered

No new skill installs were needed; the relevant skills are already present.

- **`vitepress`**
  - Relevant rule applied here: if S06 adds a new docs page, it must also update `website/docs/.vitepress/config.mts`; VitePress uses file-based routing, but sidebar/nav visibility still comes from config.
  - Practical implication: editing existing pages is the lowest-friction path; minting a new Todo proof page is possible, but it adds config churn.
- **`multi-stage-dockerfile`**
  - Relevant rule applied here: preserve the builder/runtime split, keep `.dockerignore`, and avoid falling back to repo-root-only Docker assumptions.
  - Practical implication: `compiler/mesh-pkg/src/scaffold.rs` already emits a real builder/runtime Dockerfile; if S06 touches it, keep that structure and add runtime proof instead of simplifying it away.
- **`SQLite Database Expert`**
  - Relevant rule applied here: keep parameterized SQL, isolate DB work behind a stable seam, and add a focused regression when touching database behavior.
  - Practical implication: the generated CRUD code already uses `?` placeholders; if S06 has to absorb the runtime blocker, it should fix the AOT SQLite seam first and add a built-package regression instead of scattering DB changes through the HTTP handlers.

## Current Implementation Landscape

### 1. `scripts/verify-m047-s05.sh` is already the lower-level assembled verifier draft

File: `scripts/verify-m047-s05.sh`

Current phases:

- `bash scripts/verify-m047-s04.sh`
- `cargo test -p mesh-pkg m047_s05 -- --nocapture`
- `cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture`
- `cargo test -p meshc --test e2e_m047_s05 -- --nocapture`
- `npm --prefix website run build`

Retained surfaces:

- `.tmp/m047-s05/verify/status.txt`
- `.tmp/m047-s05/verify/current-phase.txt`
- `.tmp/m047-s05/verify/phase-report.txt`
- `.tmp/m047-s05/verify/full-contract.log`
- `.tmp/m047-s05/verify/latest-proof-bundle.txt`
- `.tmp/m047-s05/verify/retained-m047-s04-verify/`
- `.tmp/m047-s05/verify/retained-m047-s05-artifacts/`

Important constraint for any wrapper: this script snapshots `.tmp/m047-s05/` before running the Todo e2e and then copies new artifact directories afterward. A higher-level S06 wrapper should **not** write its own artifacts into `.tmp/m047-s05/` or run anything concurrently that churns that tree.

### 2. `compiler/meshc/tests/e2e_m047_s05.rs` already defines most of the Todo proof boundary

Files:

- `compiler/meshc/tests/e2e_m047_s05.rs`
- `compiler/meshc/tests/support/m047_todo_scaffold.rs`

What it already proves:

- `meshc init --template todo-api`
- generated file set and public markers
- built native binary startup and `/health`
- CRUD routes
- rate limiting
- restart persistence
- `docker build`
- public docs/readme/source-first contract strings
- `scripts/verify-m047-s05.sh` text contract

What it does **not** prove yet:

- `docker run` of the generated image
- `/health` or CRUD from inside the container
- any final S06 authoritative closeout rail

Natural extension seam: `compiler/meshc/tests/support/m047_todo_scaffold.rs` already has `docker_build()` / `docker_remove()`. If S06 adds container smoke, that helper module is the right place to add container start/stop/log/port-wait helpers.

### 3. The current red blocker localizes below the scaffold

Reproduced failing rail:

- `cargo test -p meshc --test e2e_m047_s05 -- --nocapture`

Observed retained evidence:

- `.tmp/m047-s05/todo-scaffold-runtime-truth-1775067057490455000/first-run.stdout.log`
- `.tmp/m047-s05/todo-scaffold-runtime-truth-1775067057490455000/health-first.timeout.txt`

Runtime output:

- `[todo-api] Database init failed: bad parameter or other API misuse`
- health probe never gets past `Connection refused`

I also reproduced the same failure outside the scaffold with a minimal built package:

- project root: `.tmp/m047-s06-scout/sqlite-file-repro/`
- build/run command: `cargo run -q -p meshc -- build .tmp/m047-s06-scout/sqlite-file-repro --output .tmp/m047-s06-scout/sqlite-file-repro-bin && ./.tmp/m047-s06-scout/sqlite-file-repro-bin`
- result for `Sqlite.execute(db, "CREATE TABLE t (id INTEGER)", [])?`: `err=bad parameter or other API misuse`
- result for `Sqlite.open(":memory:")` + `Sqlite.close(db)`: `open-close-ok`

That strongly suggests the blocker is an AOT/built-binary `Sqlite.execute` seam, not a Todo-template-specific SQL string or HTTP problem.

Most likely code seams if S06 has to absorb this blocker:

- `compiler/mesh-rt/src/db/sqlite.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- any call/literal lowering that feeds the `params` pointer into `mesh_sqlite_execute`

Coverage gap that let this escape:

- `compiler/mesh-rt/src/db/sqlite.rs` unit tests call runtime SQLite intrinsics directly.
- `compiler/meshc/tests/e2e.rs` SQLite coverage uses `compile_and_run`, not a built package binary.
- There is no preexisting built-package SQLite execute regression outside the Todo scaffold rail.

### 4. Docs are partially updated, but the final public story is still split

Files that already mention the Todo template:

- `README.md`
- `website/docs/docs/tooling/index.md`

Files that still frame only the three route-free clustered surfaces:

- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/distributed/index.md`

Current public migration wording is also incomplete. The docs tell users to replace `clustered(work)` and remove `[cluster]`, but they do **not** explicitly tell existing users to migrate helper-shaped public function names like `execute_declared_work(...)` / `Work.execute_declared_work` to ordinary names like `add()` or `sync_todos()`.

That gap matters because D289 explicitly changed the canonical public function shape, and S04/S05 split the user-visible migration across slices.

### 5. Historical contract tests constrain how S06 can update wording

Files:

- `compiler/meshc/tests/e2e_m047_s04.rs`
- `compiler/meshc/tests/e2e_m046_s05.rs`
- `compiler/meshc/tests/e2e_m046_s06.rs`
- `compiler/meshc/tests/e2e_m045_s05.rs`

These tests still require the current docs to mention:

- `bash scripts/verify-m047-s04.sh` as the authoritative **cutover** rail
- M045/M046 wrapper scripts as historical compatibility aliases

Important nuance: they do **not** forbid S06 from adding a new authoritative **assembled closeout** rail. They fail only if S06 removes the cutover language or revives the older M046 authority story.

So the safe path is **additive** docs work:

- keep `verify-m047-s04.sh` as the authoritative cutover rail
- add `verify-m047-s06.sh` as the authoritative assembled closeout rail
- present `verify-m047-s05.sh` as the lower-level Todo scaffold subrail

### 6. VitePress config favors editing existing pages over adding a new page

File: `website/docs/.vitepress/config.mts`

The existing sidebar already exposes:

- Clustered Example
- Distributed Proof
- Developer Tools

That means the lowest-cost doc closeout is to update those existing pages plus `README.md`. If S06 adds a new Todo-specific docs page, it will need sidebar config changes in `website/docs/.vitepress/config.mts` as well.

### 7. Container-run proof has a reusable pattern elsewhere in the repo

Useful source file:

- `compiler/meshc/tests/e2e_m043_s03.rs`

That file already has container lifecycle helpers for:

- `docker create`
- `docker start -a`
- waiting for published host ports
- capturing stdout/stderr logs
- stop/kill/remove cleanup
- `docker inspect`

If S06 adds `docker run` proof to the Todo template, reusing/extracting that pattern into `compiler/meshc/tests/support/m047_todo_scaffold.rs` is cheaper and safer than inventing a second container harness from scratch.

## Recommendation

### Product / proof recommendation

Treat `scripts/verify-m047-s05.sh` as the **delegated lower-level Todo proof rail**, not the final milestone-closeout surface.

S06 should add:

- `scripts/verify-m047-s06.sh` as the final authoritative assembled closeout rail
- `compiler/meshc/tests/e2e_m047_s06.rs` as the contract test for that rail and the final doc authority language

But S06 should do that **only after** the built-binary SQLite blocker is green.

### Scope / honesty recommendation

Do **not** claim or test `HTTP.clustered(...)` in S06.

The final assembled proof should explicitly cover:

- ordinary `@cluster` startup work
- ordinary `HTTP.on_get` / `HTTP.on_post` / `HTTP.on_put` / `HTTP.on_delete` Todo routes
- docs and migration guidance that say route wrappers are still future work

Anything else would fabricate R100/R101 progress.

### Docs / migration recommendation

Keep the public story in two layers instead of collapsing everything into one page:

1. **Route-free canonical clustered surfaces stay canonical**
   - `meshc init --clustered`
   - `tiny-cluster/`
   - `cluster-proof/`
2. **Todo template becomes the fuller starter that preserves the same `@cluster` contract**
   - `meshc init --template todo-api`
   - several ordinary HTTP routes
   - actor-backed rate limiting
   - complete Dockerfile

That avoids rewriting the S04 route-free story into an HTTP-only story while still satisfying R104/R105/R106.

Public docs should add one explicit migration sentence beyond the current `clustered(work)`/`[cluster]` guidance:

- migrate helper-shaped public examples like `execute_declared_work(...)` / `Work.execute_declared_work` to ordinary function names (`add()`, `sync_todos()`, or domain-specific verbs)

### Docker recommendation

Final closeout should not stop at `docker build`.

To match the milestone language, S06 should add a lightweight container-runtime proof:

- build the generated image
- run the container with the documented env/volume shape
- wait for the published HTTP port
- hit at least `/health`
- ideally hit one CRUD route as well

If the Dockerfile is touched, keep the current multi-stage/public-installer structure from `compiler/mesh-pkg/src/scaffold.rs`; do not regress to repo-root-only assumptions.

## Natural Seams for Planning

### 1. Repair or quarantine the built-binary SQLite execute seam first

Primary files:

- `compiler/mesh-rt/src/db/sqlite.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/meshc/tests/e2e_m047_s05.rs`
- optionally `compiler/meshc/tests/e2e.rs` if a generic built-package SQLite regression is added

Why first:

- Until this is green, neither host runtime proof nor Docker runtime proof is honest.

Fastest verifier:

- `cargo test -p meshc --test e2e_m047_s05 -- --nocapture`

Useful local reproducer:

- `cargo run -q -p meshc -- build .tmp/m047-s06-scout/sqlite-file-repro --output .tmp/m047-s06-scout/sqlite-file-repro-bin && ./.tmp/m047-s06-scout/sqlite-file-repro-bin`

### 2. Extend the Todo proof from `docker build` to container runtime truth

Primary files:

- `compiler/meshc/tests/support/m047_todo_scaffold.rs`
- `compiler/meshc/tests/e2e_m047_s05.rs`
- optionally reuse patterns from `compiler/meshc/tests/e2e_m043_s03.rs`

Deliverable:

- container lifecycle helpers
- retained container logs/inspect output
- `/health` (and ideally one CRUD) assertions against the containerized app

### 3. Add the final S06 wrapper rail

Primary files:

- `scripts/verify-m047-s06.sh` (new)
- `compiler/meshc/tests/e2e_m047_s06.rs` (new)

Recommended shape:

- run delegated `bash scripts/verify-m047-s05.sh`
- retain `.tmp/m047-s05/verify/` under `.tmp/m047-s06/verify/retained-m047-s05-verify/`
- run the S06 contract/doc-authority test
- write its own `.tmp/m047-s06/verify/{status.txt,current-phase.txt,phase-report.txt,full-contract.log,latest-proof-bundle.txt}`

Keep S04 and S05 semantics separate:

- S04 = authoritative cutover rail
- S05 = lower-level Todo scaffold/runtime/docs subrail
- S06 = final authoritative assembled closeout rail

### 4. Finish the public docs/migration copy

Most likely files:

- `README.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/.vitepress/config.mts` only if a new page is added

What to add:

- final assembled closeout rail reference (`verify-m047-s06.sh`)
- lower-level Todo rail reference (`verify-m047-s05.sh`)
- explicit migration from helper-shaped function names to ordinary names
- explicit reminder that `HTTP.clustered(...)` is still not shipped

What to preserve:

- `verify-m047-s04.sh` as the authoritative cutover rail
- existing historical M045/M046 alias wording required by the legacy contract tests

## Verification Plan

Recommended verification order:

1. **Unblock runtime truth first**
   - `cargo test -p meshc --test e2e_m047_s05 -- --nocapture`
2. **Verify the new S06 contract surface**
   - `cargo test -p meshc --test e2e_m047_s06 -- --nocapture`
3. **Run the final assembled closeout rail**
   - `bash scripts/verify-m047-s06.sh`

Expected delegated checks inside the final rail:

- `bash scripts/verify-m047-s04.sh`
- `cargo test -p mesh-pkg m047_s05 -- --nocapture`
- `cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture`
- `cargo test -p meshc --test e2e_m047_s05 -- --nocapture`
- `npm --prefix website run build`

Container-specific truth to require somewhere in the stack:

- generated image builds
- generated container reaches `/health`
- generated container exposes the expected handler/db/rate-limit metadata
- ideally one containerized CRUD path succeeds

## Risks / Unknowns

- The SQLite blocker may be a more general built-binary intrinsic/ABI issue than S05 assumed. If so, S06 planning needs to treat it as prerequisite runtime work, not docs polish.
- If docs updates replace the current S04 cutover wording instead of adding S06 wording alongside it, the M045/M046 historical contract tests will fail.
- If S06 adds a new public docs page instead of editing the existing clustered/proof/tooling pages, `website/docs/.vitepress/config.mts` must be updated and more link surfaces will drift.
- If `verify-m047-s06.sh` writes into `.tmp/m047-s05/` instead of its own `.tmp/m047-s06/` tree, it will collide with the delegated S05 snapshot/retention logic.
- The generated Todo app must remain an ordinary HTTP starter plus route-free `@cluster` work. Any attempt to sneak in fake clustered routes will undermine R099/R106 and contradict the preloaded milestone context.
