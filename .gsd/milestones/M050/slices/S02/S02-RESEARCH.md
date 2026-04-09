# M050 / S02 Research — First-Contact Docs Rewrite

## Skills Discovered

- `vitepress` — relevant because S02 is Markdown + VitePress content work, and the skill reminder is useful here: file-based routes keep pages public even when they are not promoted in primary nav, so S02 can stay focused on page content instead of reopening `.vitepress` graph logic unless a real route change appears.
- `bash-scripting` — relevant for the repo’s verifier pattern. If S02 adds or updates a shell verifier, keep the existing fail-closed shape: strict mode, explicit prerequisites, named phases, and retained artifacts under `.tmp/`.

No new skill installs were needed.

## Requirement Focus

### Primary requirements this slice advances
- **R117** — first-contact public docs must read like evaluator/builder guidance instead of proof-maze routing.
- **R118** — clustered guidance must route through Getting Started + Clustered Example before proof pages, while keeping the low-level/runtime-owned split understandable.
- **R120** — README/docs/proof-page routing should converge on one coherent public graph instead of competing first-contact stories.

### Supporting constraints that still shape the implementation
- **R116** — public docs should point at generated `examples/` surfaces instead of proof-app onboarding.
- **R122** — SQLite stays explicitly local; Postgres stays the serious shared/deployable starter.
- **R127** — `tiny-cluster`, `cluster-proof`, and `reference-backend` must not re-emerge as coequal first-contact paths.

## Summary

S01 already solved the structural graph problem. The remaining drift is now concentrated in **content**, mainly:

- `website/docs/docs/getting-started/index.md`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/tooling/index.md`

The main implementation risk is not VitePress anymore. It is the **retained docs-contract blast radius**:

- `compiler/meshc/tests/e2e_m047_s05.rs`
- `compiler/meshc/tests/e2e_m047_s06.rs`
- `reference-backend/scripts/verify-production-proof-surface.sh`
- `scripts/tests/verify-m049-s05-contract.test.mjs`
- `scripts/tests/verify-m048-s05-contract.test.mjs`
- `scripts/tests/verify-m036-s03-contract.test.mjs`

Those rails still pin a lot of first-contact wording/markers. S02 can rewrite the pages, but it needs to do that **with the retained contracts in view**, especially the M047 S05/S06 assertions that still force Clustered Example and Tooling to inline proof rails and migration notes.

## Implementation Landscape

### `website/docs/docs/getting-started/index.md`
**Role:** first-contact install + hello-world page.

**Current state:**
- Top-of-page callout still routes readers to Production Backend Proof immediately.
- `## What's Next?` only sends readers to Clustered Example + Production Backend Proof + language docs.
- It does **not** currently give the explicit three-way post-hello-world choice the slice wants.
- Its source-build fallback still uses the stale repo URL:
  - `git clone https://github.com/hyperpush-org/hyperpush-mono.git`

**Implication for S02:**
- This page needs the clearest new chooser surface: after hello-world, branch to:
  - `meshc init --clustered`
  - `meshc init --template todo-api --db sqlite`
  - `meshc init --template todo-api --db postgres`
- It also needs the stale clone URL corrected.
- Current retained proof checks only guard that Clustered Example appears before Production Backend Proof in the next-step bullets; they do **not** guard the new starter-choice flow. S02 needs its own focused contract coverage here.

### `website/docs/docs/getting-started/clustered-example/index.md`
**Role:** public clustered scaffold walkthrough.

**Current state:**
- Good route-free scaffold truth is already present (`meshc init --clustered`, `Node.start_from_env()`, `@cluster pub fn add()`, runtime CLI inspection order).
- The page still uses stale GitHub links pointing at `hyperpush-org/hyperpush-mono` for:
  - `examples/todo-postgres/README.md`
  - `examples/todo-sqlite/README.md`
  - `reference-backend/README.md`
- The end of the page still carries a long verifier-map paragraph inside the first-contact surface.

**Better truth anchor already exists:**
- `compiler/mesh-pkg/src/scaffold.rs` clustered README template already uses the current repo URLs (`snowdamiz/mesh-lang`) and already expresses the public clustered contract cleanly.

**Implication for S02:**
- Treat the clustered README template in `compiler/mesh-pkg/src/scaffold.rs` as the best copy source for this page.
- Fix the stale GitHub URLs here.
- If the page is supposed to stop inlining the long proof-rail map, S02 must retarget the M047 S05/S06 docs assertions that currently require those markers.

### `website/docs/docs/tooling/index.md`
**Role:** public tooling surface.

**Current state:**
- The page starts with:
  - install
  - update
  - **Release Assembly Runbook**
  - **Assembled contract verifier**
  - **Assembled scaffold/example verifier**
- That proof/runbook material currently lands before Formatter, REPL, Package Manager, testing, LSP, and editor guidance.
- The package-manager section already carries the right starter split, but it also still inlines the long clustered proof map.
- This page also uses stale `hyperpush-org/hyperpush-mono` GitHub links for example/readme references.

**Existing contract pressure:**
- `scripts/tests/verify-m048-s05-contract.test.mjs` pins update, entrypoint, grammar, and `bash scripts/verify-m048-s05.sh` markers.
- `scripts/tests/verify-m036-s03-contract.test.mjs` pins support tiers + editor contract wording.
- `scripts/tests/verify-m049-s05-contract.test.mjs` pins explicit SQLite/Postgres starter commands, `shared/deployable`, and `bash scripts/verify-m049-s05.sh` discoverability.

**Implication for S02:**
- Reordering sections is safe; the retained tests mostly pin **presence**, not heading order.
- The low-risk move is to keep install/update/package-manager/editor content intact, move proof/runbook sections lower, and expose the assembled public starter/docs verifier through the existing `bash scripts/verify-m049-s05.sh` marker rather than inventing a second public mega-command.
- Fix the stale GitHub URLs here.

### `compiler/mesh-pkg/src/scaffold.rs`
**Role:** generated clustered README + Todo README truth source.

**Why it matters:**
- The clustered README template already has the right current repo URLs and clustered contract phrasing.
- The generated Todo READMEs already carry the SQLite-vs-Postgres split truth the docs should point at.

**Implication for S02:**
- Use this as a **copy anchor**, not a rewrite target.
- Do **not** edit generator-owned README templates unless the public contract itself changes.

### `examples/todo-sqlite/README.md` and `examples/todo-postgres/README.md`
**Role:** truthful follow-on surfaces for local vs shared/deployable starters.

**Current state:**
- Already explicit and aligned with M049.
- Already proven by:
  - `compiler/meshc/tests/tooling_e2e.rs`
  - `scripts/tests/verify-m049-s03-materialize-examples.mjs --check`
  - `compiler/meshc/tests/e2e_m049_s03.rs`

**Implication for S02:**
- Prefer linking to them more clearly rather than rewriting them.
- They are generator-owned; touching them expands blast radius into scaffold/materializer parity.

## Constraints and Surprises

### 1. The CLI still supports unsplit `meshc init --template todo-api`, but public docs must stay explicit
`compiler/meshc/tests/tooling_e2e.rs` still proves that `meshc init --template todo-api` defaults to the local SQLite starter.

That is **CLI behavior**, not public-docs guidance.

S02 should keep using:
- `meshc init --template todo-api --db sqlite`
- `meshc init --template todo-api --db postgres`

Do not simplify public docs back to the generic unsplit command.

### 2. First-contact docs currently use two different repository identities
Current generated/public truth is split:
- `compiler/mesh-pkg/src/scaffold.rs` clustered README template uses `https://github.com/snowdamiz/mesh-lang/...`
- `website/docs/docs/production-backend-proof/index.md` already uses `snowdamiz/mesh-lang`
- but first-contact docs still use `https://github.com/hyperpush-org/hyperpush-mono/...`

This is a real coherence bug and currently appears in:
- `website/docs/docs/getting-started/index.md`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/tooling/index.md`
- plus S03-owned distributed pages (`website/docs/docs/distributed*.md`)

There is currently **no focused test** for these first-contact repo URLs.

### 3. The real blocker for simplifying Clustered Example / Tooling now lives in retained M047 S05/S06 tests
Current retained assertions still expect first-contact pages to mention things that S02 probably wants to demote or move out:
- `bash scripts/verify-m047-s04.sh`
- `bash scripts/verify-m047-s05.sh`
- `bash scripts/verify-m047-s06.sh`
- `cargo test -p meshc --test e2e_m047_s07 -- --nocapture`
- `execute_declared_work(...)`
- `Work.execute_declared_work`

The key files are:
- `compiler/meshc/tests/e2e_m047_s05.rs`
- `compiler/meshc/tests/e2e_m047_s06.rs`

If S02 wants Clustered Example and Tooling to stop acting like mini proof-map pages, these tests have to be retargeted in the same slice.

### 4. Getting Started next-step bullets are pinned by the production-proof verifier
`reference-backend/scripts/verify-production-proof-surface.sh` checks exact order/text markers for:
- `- [Clustered Example](/docs/getting-started/clustered-example/)`
- `- [Production Backend Proof](/docs/production-backend-proof/)`

If S02 rewrites `## What's Next?`, keep those bullets or update the verifier and its dependent Rust contract in the same task.

### 5. No VitePress graph work is obviously needed
Per the VitePress skill and S01 summary, the graph/footer problem is already handled in:
- `website/docs/.vitepress/config.mts`
- `website/docs/.vitepress/theme/composables/usePrevNext.ts`

S02 looks content-only unless a real navigation requirement changes. Reopening those files by default would be scope drift.

## Recommendation

### Build/prove first
1. **Retained-contract seam first:** decide which first-contact proof-map markers can leave Clustered Example / Tooling, then retarget the M047 S05/S06 docs assertions accordingly.
2. **Getting Started rewrite second:** add the explicit post-hello-world starter chooser and fix the stale clone URL.
3. **Clustered Example + Tooling rewrite third:** align them to the scaffold/example truth and fix stale GitHub URLs.
4. **Only then** run the heavier assembled verifier.

### Natural task seams

#### Seam A — First-contact contract retargeting
Files likely involved:
- `compiler/meshc/tests/e2e_m047_s05.rs`
- `compiler/meshc/tests/e2e_m047_s06.rs`
- `scripts/tests/verify-m049-s05-contract.test.mjs` (only if assembled-command discoverability changes)
- `reference-backend/scripts/verify-production-proof-surface.sh` (only if Getting Started bullet wording/order changes)
- new S02-owned source contract test, likely under `scripts/tests/`

Goal:
- stop requiring full proof-rail maps and migration-helper prose on the first-contact pages
- start requiring the new starter-choice / repo-link / deeper-proof-demotion markers instead

#### Seam B — Getting Started rewrite
Files likely involved:
- `website/docs/docs/getting-started/index.md`
- maybe `README.md` if the slice explicitly chooses to advance the README half of R120 now

Goal:
- install -> hello-world -> explicit next project choice
- keep Clustered Example ahead of Production Backend Proof
- add the SQLite/Postgres follow-on split without reopening the nav graph
- fix the stale source-build clone URL

#### Seam C — Clustered Example + Tooling rewrite
Files likely involved:
- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/tooling/index.md`

Goal:
- keep scaffold/example truth intact
- replace stale `hyperpush-org/hyperpush-mono` links with current repo links
- move proof-runbook density out of the primary reading path
- keep retained M036/M048/M049 markers intact on the tooling page

## Verification

### Page/local contract checks that should exist or be updated
- new S02-focused source contract test for first-contact docs (Getting Started + Clustered Example + Tooling; README only if S02 includes it)
- `node --test scripts/tests/verify-m049-s04-onboarding-contract.test.mjs`
- `node --test scripts/tests/verify-m049-s05-contract.test.mjs`
- `node --test scripts/tests/verify-m048-s05-contract.test.mjs`
- `node --test scripts/tests/verify-m036-s03-contract.test.mjs`

### Existing truth rails the slice should delegate to, not replace
- `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`
- `cargo test -p meshc --test tooling_e2e test_init_todo_template_ -- --nocapture`
- `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check`
- `cargo test -p meshc --test e2e_m049_s03 -- --nocapture`

### Built-site / assembled verification
- `npm --prefix website run build`
- `bash scripts/verify-m049-s05.sh`

`bash scripts/verify-m049-s05.sh` already plays the assembled public-story role. The missing piece is a focused S02 source contract for the first-contact copy, not a brand new mega-wrapper.

## Planner Notes

- Do **not** start by editing `compiler/mesh-pkg/src/scaffold.rs` or the generated example READMEs. They are already the best truth anchors.
- Do **not** start by reopening `.vitepress/config.mts` or `usePrevNext.ts`. S01 already owns the graph mechanics.
- The highest-value content cleanup is in the three first-contact docs pages.
- The highest-risk code change is retargeting the retained M047 S05/S06 docs assertions so first-contact pages can stop inlining proof rails.
- If R120 is interpreted strictly for this slice, decide up front whether `README.md` is in scope. The page already preserves the starter split and Clustered Example precedence, so README work is helpful but not the clearest blocker compared with the three website docs pages.
