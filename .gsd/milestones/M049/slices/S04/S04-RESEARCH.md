# S04 Research — Retire top-level proof-app onboarding surfaces

## Summary

- **Primary requirement focus:** `R127` owns this slice. `R116` is the supporting requirement because the replacement public surface is now `examples/todo-sqlite` and `examples/todo-postgres`. `R122` constrains the public story: SQLite stays explicitly local-only, Postgres stays the serious clustered/shared path. `R112`/`R113`/`R114` remain guardrails while docs, scaffold copy, and skills move.
- The user-visible drift is concentrated in a small set of public copy files that still teach **`meshc init --clustered`, `tiny-cluster/`, and `cluster-proof/` as three equal canonical clustered surfaces**.
- The package code itself is no longer the hard part. `tiny-cluster/` and `cluster-proof/` are now tiny self-contained route-free packages. The real cost is path churn: **63 source-ish files** outside `.gsd/` still mention `tiny-cluster` or `cluster-proof`.
- The replacement public targets already exist and are mechanically proven by S03, but **there are currently zero public references to `examples/todo-sqlite` or `examples/todo-postgres`** in `README.md`, `website/docs`, `compiler/mesh-pkg/src/scaffold.rs`, or `tools/skill/mesh`.

## Requirements Focus

- **R127 — owner**
  - `tiny-cluster`, `cluster-proof`, and `reference-backend` must stop acting like coequal onboarding surfaces.
  - Current drift: public clustered docs still elevate `tiny-cluster/` and `cluster-proof/` as first-contact runbooks.
- **R116 — support / likely validation target once S04 lands**
  - Public onboarding should point at generated `/examples`, not proof-app-shaped top-level packages.
  - S03 already delivered the examples and the materializer/parity rail.
- **R122 — constraint**
  - Public wording must preserve the already-landed split:
    - `meshc init --template todo-api --db sqlite` = honest local single-node starter
    - `meshc init --template todo-api --db postgres` = serious clustered/shared starter
- **R112 / R113 / R114 — non-regression guardrails**
  - M048 docs, entrypoint, update, editor, and skill truths must stay green while clustered onboarding copy changes.

## Skills Discovered

- **Loaded existing skill:** `vitepress`
  - Relevant rule used here: check the VitePress config first and treat content-page edits as subordinate to the site config.
  - Current authority is `website/docs/.vitepress/config.mts`.
  - Result: the sidebar/page structure already fits S04. This slice is mostly **content rewrites**, not VitePress routing/theme work.
- **Loaded existing repo-local skill:** `tools/skill/mesh/skills/clustering/SKILL.md`
  - Relevant rules used here:
    - `meshc init --clustered` is the primary public clustered scaffold.
    - `meshc init --template todo-api --db postgres` is the fuller clustered/shared starter.
    - `meshc init --template todo-api --db sqlite` stays the honest local-only starter.
  - Result: public copy should follow that split and stop teaching the old three-surface story.
- **No additional skill installs were needed.** The slice’s core technologies (VitePress docs, Mesh clustering/scaffold contract, Rust/Node test rails) were already covered by installed or repo-local skills.

## Implementation Landscape

### 1. Public onboarding copy still teaches the old proof-app story

These are the main public files that still frame `tiny-cluster/` and `cluster-proof/` as public first-contact surfaces:

- `README.md`
- `compiler/mesh-pkg/src/scaffold.rs`
  - generated `meshc init --clustered` README text currently says the scaffold follows the same contract as `tiny-cluster/` and `cluster-proof/`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/tooling/index.md`
- `tools/skill/mesh/skills/clustering/SKILL.md`

Concrete drift:

- `README.md` still says the clustered-app story is shared with the equal-surface runbooks at `tiny-cluster/README.md` and `cluster-proof/README.md`.
- `website/docs/docs/getting-started/clustered-example/index.md` still opens with “three equal canonical surfaces.”
- `website/docs/docs/distributed/index.md` still tells readers to start with `Clustered Example`, `tiny-cluster/README.md`, or `cluster-proof/README.md` for the verified clustered-app/operator path.
- `website/docs/docs/distributed-proof/index.md` still lists `tiny-cluster/README.md` and `cluster-proof/README.md` as public canonical surfaces and includes direct `meshc build tiny-cluster`, `meshc test tiny-cluster/tests`, `meshc build cluster-proof`, `meshc test cluster-proof/tests` commands.
- `website/docs/docs/tooling/index.md` still tells readers to use the two top-level runbooks as the smallest/deeper route-free surfaces.
- `tools/skill/mesh/skills/clustering/SKILL.md` rule 4 still says the scaffold, `tiny-cluster/`, and `cluster-proof/` all teach the same route-free clustered contract.

### 2. `/examples` is ready, but currently invisible from public onboarding

S03 delivered the public replacements:

- `examples/todo-sqlite/README.md`
- `examples/todo-postgres/README.md`

They already encode the correct split:

- SQLite README = explicit local-only single-node contract, no `work.mpl`, no `HTTP.clustered(...)`, no `meshc cluster` story.
- Postgres README = route-free clustered work in `work.mpl`, explicit-count `HTTP.clustered(1, ...)` only on `GET /todos` / `GET /todos/:id`, local `/health` and mutating routes.

Important current-state finding:

- `rg -l 'examples/todo-(sqlite|postgres)' README.md website/docs compiler/mesh-pkg/src/scaffold.rs tools/skill scripts/tests`
  - returned **0 public-source hits** outside the S03 parity tests.

So S04 does not need to invent new public assets. It needs to **repoint the public story to assets that already exist**.

### 3. The proof packages are already small enough to become internal fixtures

Current root package shapes are minimal.

`tiny-cluster/`:
- `mesh.toml`
- `main.mpl`
- `work.mpl`
- `tests/work.test.mpl`
- `README.md`

`cluster-proof/`:
- `mesh.toml`
- `main.mpl`
- `work.mpl`
- `tests/work.test.mpl`
- `README.md`
- `Dockerfile`
- `fly.toml`
- tracked `cluster-proof.ll`

The code itself is intentionally tiny now:

- `main.mpl` only calls `Node.start_from_env()` and logs bootstrap success/failure.
- `work.mpl` is just `@cluster pub fn add() -> Int do 1 + 1 end`.
- Package tests mostly assert source/readme contract shape and absence of old routeful seams.

That means S04 is a **relocation and contract rewrite** slice, not a language/runtime refactor.

### 4. Keep package names and log prefixes stable when relocating

Many historical rails assert literal package/log/runtime names:

- `[tiny-cluster] ...`
- `[cluster-proof] ...`
- `Work.add`
- node names like `tiny-cluster-primary@...` and `cluster-proof-primary@...`
- Docker/Fly packaging strings around `cluster-proof`

Recommendation from the current tree: **move the directories, but do not rename the internal package names or log prefixes**. Path migration is already expensive; renaming internal package identities would multiply the historical test churn for little value.

### 5. The real blast radius is path churn across tests and verifiers

After excluding `.gsd`, build output, and generated editor/site artifacts, there are still **63 source-ish files** in repo code/docs/scripts/skills that mention `tiny-cluster` or `cluster-proof`.

The important groups are:

#### A. Public story / copy surfaces

- `README.md`
- `compiler/mesh-pkg/src/scaffold.rs`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/tooling/index.md`
- `tools/skill/mesh/skills/clustering/SKILL.md`

#### B. Current authoritative clustered cutover rails

These are the highest-leverage current seams:

- `scripts/verify-m047-s04.sh`
- `scripts/verify-m047-s05.sh`
- `scripts/verify-m047-s06.sh`
- `compiler/meshc/tests/e2e_m047_s04.rs`
- `compiler/meshc/tests/e2e_m047_s05.rs`
- `compiler/meshc/tests/support/m047_todo_scaffold.rs`
- `compiler/meshc/tests/support/m046_route_free.rs`
- `compiler/meshc/tests/e2e_m046_s03.rs`
- `compiler/meshc/tests/e2e_m046_s04.rs`
- `compiler/meshc/tests/e2e_m045_s02.rs`

Important leverage point:

- `scripts/verify-m045-s04.sh`, `scripts/verify-m045-s05.sh`, `scripts/verify-m046-s04.sh`, `scripts/verify-m046-s05.sh`, and `scripts/verify-m046-s06.sh` already delegate into `scripts/verify-m047-s04.sh`.
- So **updating `scripts/verify-m047-s04.sh` buys several retained compatibility wrappers automatically**.

#### C. Older still-direct historical clustered rails

These do **not** currently delegate and still hardcode root package paths directly:

- Rust e2e targets from `compiler/meshc/tests/e2e_m039_s01.rs` through the M044-era clustered tests
- Bash verifiers:
  - `scripts/verify-m039-s01.sh`
  - `scripts/verify-m039-s02.sh`
  - `scripts/verify-m039-s03.sh`
  - `scripts/verify-m039-s04*.sh`
  - `scripts/verify-m040-s01.sh`
  - `scripts/verify-m042-s01.sh`
  - `scripts/verify-m042-s02.sh`
  - `scripts/verify-m042-s03.sh`
  - `scripts/verify-m042-s04*.sh`
  - `scripts/verify-m043-s01.sh`
  - `scripts/verify-m043-s02.sh`
  - `scripts/verify-m043-s03.sh`
  - `scripts/verify-m043-s04*.sh`
  - `scripts/verify-m044-s01.sh`
  - `scripts/verify-m044-s02.sh`
  - `scripts/verify-m044-s04.sh`
  - `scripts/verify-m045-s01.sh`
  - `scripts/verify-m045-s02.sh`
  - `scripts/verify-m046-s03.sh`

These files are why S04 must treat fixture-path centralization as first-class work. Deleting the root dirs first would leave a long tail of future runtime failures.

### 6. There are two non-obvious internal dependencies

#### Hidden dependency A — Todo helper reuses `cluster-proof/Dockerfile`

`compiler/meshc/tests/support/m047_todo_scaffold.rs` builds a Linux output-builder image with:

- `docker build --target builder -f cluster-proof/Dockerfile ...`

That means moving `cluster-proof/` is not just a docs/test issue. The Todo helper’s Linux-output Docker seam must learn the new fixture path **or** get its own dedicated builder Dockerfile.

#### Hidden dependency B — tracked `cluster-proof.ll`

`compiler/meshc/tests/e2e_m046_s04.rs` snapshots:

- `cluster-proof/cluster-proof.ll`

to prove temp builds do not churn tracked package outputs.

If `cluster-proof/` moves, that no-churn assertion must move or be simplified in the same task. Otherwise a relocation can look like a regression in the route-free build rail.

### 7. Best relocation home is `scripts/fixtures/`, not `tests/fixtures/`

Current repo patterns:

- `scripts/fixtures/` already holds frozen or reusable package-level proof inputs such as `scripts/fixtures/m047-s05-clustered-todo/`.
- `tests/fixtures/` is currently mostly parser/lexer/source-snippet data, not full package trees.

For S04, the better home is a **stable internal fixture path under `scripts/fixtures/`**.

Recommendation:

- use a stable non-milestone path such as:
  - `scripts/fixtures/clustered/tiny-cluster/`
  - `scripts/fixtures/clustered/cluster-proof/`
- avoid milestone-scoped names like `m049-s04-*` for the relocated packages themselves, because these become enduring current fixtures, not historical snapshots

### 8. `reference-backend` is not the main relocation target

Current public framing is already more restrained:

- `website/docs/docs/getting-started/index.md` points to `reference-backend` as the deeper backend proof, not the clustered starter surface.
- `website/docs/docs/production-backend-proof/index.md` explicitly frames `reference-backend/README.md` as the deepest runbook for the backend proof story.

So S04 should keep `reference-backend` as the **deeper production proof surface**, not try to relocate or demote it like the two clustered proof packages. The work here is mainly to stop listing it as a coequal starter if that language survives in README-level comparison copy.

## Recommendation

1. **Move `tiny-cluster/` and `cluster-proof/` into stable internal fixture roots under `scripts/fixtures/`.**
   - Keep package names, runtime names, and log prefixes stable.
   - Do not use milestone-specific fixture names for the enduring relocated packages.

2. **Add shared path helpers before deleting the root dirs.**
   - Rust side: extend `compiler/meshc/tests/support/m046_route_free.rs` or add a sibling support module that returns the new fixture roots.
   - Bash side: add one shared fixture-root variable/helper for the older verifier family instead of open-coding new paths in every script.

3. **Update the high-leverage retained rails first.**
   - Start with:
     - `scripts/verify-m047-s04.sh`
     - `compiler/meshc/tests/e2e_m047_s04.rs`
     - `compiler/meshc/tests/e2e_m046_s03.rs`
     - `compiler/meshc/tests/e2e_m046_s04.rs`
     - `compiler/meshc/tests/e2e_m045_s02.rs`
     - `compiler/meshc/tests/support/m047_todo_scaffold.rs`
   - That stabilizes the main clustered fixture path and the hidden Docker builder dependency before sweeping older M039–M044 rails.

4. **Rewrite public copy to point at scaffold + generated examples, not proof-package runbooks.**
   - Public clustered first-contact should become:
     - `meshc init --clustered`
     - `examples/todo-postgres/README.md` (or the explicit Postgres starter command)
     - `examples/todo-sqlite/README.md` (or the explicit SQLite starter command)
   - `reference-backend/README.md` stays public as the deeper backend proof, not the clustered starter.

5. **Update the generated clustered scaffold README and the Mesh clustering skill in the same slice.**
   - Otherwise the website will say one thing while `meshc init --clustered` and the auto-loaded Mesh skill keep teaching the old root-proof-app story.

6. **Add a slice-owned contract rail instead of relying only on legacy M047 wording tests.**
   - Existing pattern to copy:
     - `scripts/tests/verify-m048-s05-contract.test.mjs`
     - `scripts/tests/verify-m048-s04-skill-contract.test.mjs`
     - `scripts/tests/verify-m049-s03-materialize-examples.test.mjs`
   - S04 should get its own onboarding-contract test that fail-closes on:
     - stale public mentions of `tiny-cluster/README.md` or `cluster-proof/README.md`
     - missing example-first replacements
     - stale scaffold/skill copy

## Suggested Task Split

### Task 1 — Relocate the proof packages into internal fixtures and centralize their paths

**Goal:** move `tiny-cluster/` and `cluster-proof/` out of the repo root without changing their internal package identities.

Likely files:
- new stable fixture dirs under `scripts/fixtures/clustered/...`
- `compiler/meshc/tests/support/m046_route_free.rs` or new support module
- `compiler/meshc/tests/support/m047_todo_scaffold.rs`
- `scripts/verify-m047-s04.sh`
- `compiler/meshc/tests/e2e_m046_s03.rs`
- `compiler/meshc/tests/e2e_m046_s04.rs`
- `compiler/meshc/tests/e2e_m045_s02.rs`

Why first:
- Nothing else is safe until the internal fixture path exists.
- This task retires the root dirs structurally, which is the slice’s highest-risk change.

### Task 2 — Rewrite public onboarding copy to scaffold/examples-first

**Goal:** replace the “three equal canonical surfaces” story with scaffold + `/examples`.

Likely files:
- `README.md`
- `compiler/mesh-pkg/src/scaffold.rs`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/tooling/index.md`
- `tools/skill/mesh/skills/clustering/SKILL.md`
- possibly `tools/skill/mesh/SKILL.md` if the root routing text needs example-first wording

Constraint:
- preserve the existing Postgres/SQLite split exactly as landed in S01/S02 and enforced by the Mesh skill contract.

### Task 3 — Sweep the retained/historical rails and add the slice-owned contract proof

**Goal:** keep the repo’s retained proofs runnable after the move, and add an explicit S04 fail-closed contract test for the new onboarding story.

Likely files:
- `scripts/tests/verify-m049-s04-onboarding-contract.test.mjs` (new, recommended)
- `scripts/verify-m047-s05.sh`
- `scripts/verify-m047-s06.sh`
- `compiler/meshc/tests/e2e_m047_s04.rs`
- older M039–M044 scripts/tests that still hardcode root package paths
- maybe `compiler/meshc/tests/e2e_m049_s04.rs` if a Rust-side retained-fixture/path proof is cleaner than growing M047 further

Why separate:
- Once the move and copy rewrite are done, this task is a targeted historical-sweep + proof-hardening pass.
- It is the right place to fix the `cluster-proof.ll` no-churn assertion and the `m047_todo_scaffold.rs` Dockerfile dependency if they were left as follow-through from Task 1.

## Verification

### Cheap proof-of-direction checks

- `rg -n 'tiny-cluster/README.md|cluster-proof/README.md|\btiny-cluster/\b|\bcluster-proof/\b' README.md website/docs compiler/mesh-pkg/src/scaffold.rs tools/skill/mesh`
  - after S04, public hits should be gone or clearly internal-only; public onboarding files should no longer point at root proof-app runbooks
- `rg -n 'examples/todo-sqlite|examples/todo-postgres' README.md website/docs compiler/mesh-pkg/src/scaffold.rs tools/skill/mesh`
  - should show the new public example pointers

### Targeted required rails

- `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check`
  - proves the replacement public examples still match scaffold output exactly
- `node --test scripts/tests/verify-m048-s04-skill-contract.test.mjs`
  - guards the Mesh skill bundle after clustered onboarding wording changes
- `node --test scripts/tests/verify-m048-s05-contract.test.mjs`
  - guards the M048 docs/update/editor contract while touching `README.md` / `website/docs/docs/tooling/index.md`
- `npm --prefix website run build`
  - required docs truth check after public copy changes

### Authoritative retained clustered rail after the move

- `bash scripts/verify-m047-s04.sh`
  - this is the highest-value retained clustered cutover rail to keep green after relocating the packages

### Suggested new slice-owned rail

Add one fail-closed S04 contract test, preferably Node-based unless a Rust harness is clearly better:

- `node --test scripts/tests/verify-m049-s04-onboarding-contract.test.mjs`

It should assert all of these together:
- public docs/scaffold/skill surfaces no longer teach `tiny-cluster/` or `cluster-proof/` as onboarding targets
- public copy now points at scaffold plus `examples/todo-sqlite` / `examples/todo-postgres`
- internal retained references point at lower-level fixtures/support instead of repo-root onboarding packages

## Planner Notes

- This is **not** a good slice for “delete dirs, then fix failures.” The root dirs are referenced too broadly.
- The safe order is: **fixture path first -> authoritative retained rail update -> public copy rewrite -> historical sweep -> delete root dirs**.
- Keep `reference-backend` out of the fixture-relocation work unless a file is still wrongly listing it as a starter. The backend proof story is already intentionally deeper and separate.
