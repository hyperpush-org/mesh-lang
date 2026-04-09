# S04 Research — Hard cutover and dogfood migration

## Summary

S04 is a coordinated cutover slice, not a local example cleanup. The repo’s live clustered teaching surface is still the M046 route-free model:

- `tiny-cluster/`, `cluster-proof/`, and `meshc init --clustered` all teach `clustered(work)`
- the generated scaffold, both package examples, docs pages, and historical verifier rails all hardcode `declared_work_runtime_name()` and `Work.execute_declared_work`
- parser/pkg/compiler still accept both `@cluster` and legacy `clustered(work)`, and mesh-pkg still fully parses `[cluster]` manifest declarations

S02 already proved the runtime does not need the old helper-driven public story anymore. `compiler/meshc/tests/e2e_m047_s02.rs` shows ordinary source-declared `@cluster` functions register under generic runtime names derived from module/function identity, surface replication counts correctly, and do not fall back to the legacy helper model. That means the public examples can move to `@cluster` without inventing a new runtime seam.

The main planning constraint is that S03’s retained summary says `HTTP.clustered(...)` never actually landed. So S04 can honestly hard-cut the ordinary clustered function teaching surfaces, but it should not present clustered route wrappers as already shipped proof unless a later slice lands them for real.

## Requirements Targeted

This slice directly owns or strongly supports:

- **R102** — hard-cut the old public syntax instead of keeping both models alive
- **R103** — dogfood the new clustered model across repo-owned examples and proof rails
- **R106** — teach one coherent source-first clustered model to users

It also supports:

- **R097** — public examples/scaffold/docs should actually use `@cluster` / `@cluster(N)`
- **R098** — canonical surfaces should teach default-vs-explicit replication counts from source, not manifest folklore
- **R099** — keep clustering obviously general-function-first, not route-only

## Skills Discovered

- **vitepress** — already installed and relevant; loaded for docs-side guidance
  - Applied rules: VitePress uses file-based routing, docs live under `website/docs/docs/**/*.md`, and site config lives in `website/docs/.vitepress/config.mts`.
  - For this slice, page content edits are the primary seam; only touch `config.mts` if navigation/sidebar text or page routing must change.

No new skills were installed. No external library docs were needed.

## Current State Inventory

### 1. Canonical package examples are still M046-shaped

**Files:**
- `tiny-cluster/work.mpl`
- `tiny-cluster/main.mpl`
- `tiny-cluster/README.md`
- `tiny-cluster/tests/work.test.mpl`
- `cluster-proof/work.mpl`
- `cluster-proof/main.mpl`
- `cluster-proof/README.md`
- `cluster-proof/tests/work.test.mpl`
- `cluster-proof/Dockerfile`
- `cluster-proof/fly.toml`

**What they do now:**
- both packages use:
  - `pub fn declared_work_runtime_name() -> String do "Work.execute_declared_work" end`
  - `clustered(work) pub fn execute_declared_work(...) -> Int do 1 + 1 end`
- both READMEs explicitly teach the single `clustered(work)` declaration and the stable runtime name
- both package test files grep for `clustered(work)`, `Work.execute_declared_work`, route-free behavior, and README wording
- `cluster-proof` packaging files are already route-free and binary-only; they are not the main cutover problem

**Important constraint:**
`compiler/meshc/tests/e2e_m046_s05.rs` currently requires the generated scaffold `work.mpl` to be **byte-for-byte identical** to both `tiny-cluster/work.mpl` and `cluster-proof/work.mpl`. That makes these three surfaces one migration unit.

### 2. The generated clustered scaffold is still teaching the old public model

**Files:**
- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/meshc/tests/tooling_e2e.rs`
- `compiler/meshc/tests/support/m046_route_free.rs`

**What they do now:**
- `scaffold_clustered_project(...)` writes a package-only `mesh.toml`, route-free `main.mpl`, and a `work.mpl` containing `declared_work_runtime_name()` + `clustered(work)`
- the generated README repeats the same M046 contract and exact phrase `automatically starts the source-declared \`clustered(work)\` handler`
- `tooling_e2e.rs::test_init_clustered_creates_project()` asserts for the helper, the legacy syntax, and the README wording
- `support/m046_route_free.rs` is the shared helper for scaffold/package runtime-truth tests, temp-path builds, and retained artifacts

**Natural seam:**
This is the best place to define the new public scaffold contract once, then update downstream tests/docs/examples to match it.

### 3. Parser/pkg compatibility bridge is still live everywhere

**Files:**
- `compiler/mesh-parser/src/parser/items.rs`
- `compiler/mesh-parser/src/ast/item.rs`
- `compiler/mesh-parser/src/syntax_kind.rs`
- `compiler/mesh-parser/tests/parser_tests.rs`
- `compiler/mesh-pkg/src/manifest.rs`
- `compiler/meshc/tests/e2e_m047_s01.rs`
- `compiler/mesh-lsp/src/analysis.rs` (tests reference `[cluster]` manifest paths)

**What they do now:**
- parser explicitly accepts both `@cluster` and legacy `clustered(work)` via `starts_clustered_fn_def(...)`, `parse_cluster_decorator_decl(...)`, and `parse_legacy_clustered_work_decl(...)`
- AST exposes `ClusteredDeclSyntax::{SourceDecorator, LegacyCompat}`
- mesh-pkg still parses `[cluster]` through `Manifest.cluster`, `ClusterConfig`, `ClusteredDeclaration`, and validates manifest/source combined declarations
- mesh-pkg source provenance text still names `"\`clustered(work)\` marker"`
- parser tests still have a full legacy `clustered(work)` coverage block
- M047/S01 e2e tests still use manifest `[cluster]` as part of duplicate-declaration/fail-closed proofs

**Planning implication:**
If S04 is a real bridge end, not just a docs/examples reset, the parser/pkg/compiler/editor compatibility layer needs a coordinated change. This is not isolated to examples.

### 4. Docs are VitePress content and still narrate the M046 route-free story

**Files:**
- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/distributed/index.md`
- `README.md`
- `website/docs/.vitepress/config.mts`

**What they do now:**
- clustered example page shows literal `declared_work_runtime_name()` + `clustered(work)` code
- tooling page describes `meshc init --clustered` as emitting a declared `Work.execute_declared_work` boundary
- distributed proof page says `work.mpl` owns `declared_work_runtime_name()` plus the single `clustered(work)` declaration
- README repeats the same story and points at M046 verifier names
- VitePress config is normal file-based config under `website/docs/.vitepress/config.mts`; no special docs abstraction exists

**Skill-informed note:**
Per the VitePress skill, the content seam is just the markdown pages under `website/docs/docs/**`. Unless S04 changes page structure or sidebar labels, `config.mts` probably does not need edits.

### 5. Historical verifier/test rails pin exact old wording and file content

**Files:**
- `compiler/meshc/tests/e2e_m045_s01.rs`
- `compiler/meshc/tests/e2e_m045_s02.rs`
- `compiler/meshc/tests/e2e_m045_s03.rs`
- `compiler/meshc/tests/e2e_m046_s03.rs`
- `compiler/meshc/tests/e2e_m046_s04.rs`
- `compiler/meshc/tests/e2e_m046_s05.rs`
- `compiler/meshc/tests/e2e_m046_s06.rs`
- `scripts/verify-m046-s05.sh`
- `scripts/verify-m046-s06.sh`
- `scripts/verify-m046-s04.sh`
- `scripts/verify-m045-s05.sh`

**What they do now:**
- assert exact M046 strings in docs and READMEs
- assert exact source text like `clustered(work)`, `declared_work_runtime_name()`, and `Work.execute_declared_work`
- assert scenario metadata contains `startup_runtime_name == "Work.execute_declared_work"`
- replay old M046 verifier names from docs/README wording

**Important constraint:**
S04 cannot safely change docs/example wording without updating these rails in the same task. They are not generic smoke tests; many are exact-string contract checks.

### 6. There is no M047 S04 proof surface yet

**Current absence:**
- no `compiler/meshc/tests/e2e_m047_s04.rs`
- no `scripts/verify-m047-s04.sh`

That means the slice currently has no repo-owned acceptance rail for the cutover it is supposed to perform. Planner should allocate explicit work for adding one, instead of only mutating old M046 rails.

### 7. `tiny-cluster-prefered/` is stale prior art, not current truth

**Files:**
- `tiny-cluster-prefered/add.mpl`
- `tiny-cluster-prefered/lib/subtract.mpl`
- `tiny-cluster-prefered/mesh.toml`

**What it shows:**
- source-level `@cluster(3)` and `@cluster(2)` examples
- but also an obsolete `[cluster]` manifest shape using `clusters = ...`, which is not current schema

**Planning implication:**
This directory is currently contradictory “preferred” prior art. S04 should either migrate it into a truthful fixture/example or clearly demote/delete it; leaving it untouched keeps an obsolete hybrid model in-tree.

## Key Finding: runtime name stability does not require the old helper

This is the most useful implementation shortcut.

`compiler/meshc/tests/e2e_m047_s02.rs` proves that ordinary `@cluster` functions already register under generic runtime names derived from module/function identity, e.g. `Work.handle_submit` and `Work.handle_retry`, with no `declared_work_runtime_name()` helper.

That means S04 can likely do this:

```mesh
@cluster
pub fn execute_declared_work(_request_key :: String, _attempt_id :: String) -> Int do
  1 + 1
end
```

and still preserve runtime name `Work.execute_declared_work` purely by function naming.

That is the cleanest public cutover because it:
- removes boilerplate
- preserves the current runtime/CLI request-key identity and operator truth
- minimizes runtime churn
- aligns with R105’s low-boilerplate goal

The planner should treat helper removal as a public-surface cleanup opportunity, not as a runtime redesign.

## Blockers / Ambiguities

### 1. S03 route wrapper is still missing

The retained S03 summary says `HTTP.clustered(...)` did not land. So S04 should **not** teach clustered routes as already shipped unless the implementation lands elsewhere first.

Honest scope for S04 from current tree:
- hard-cut ordinary clustered-function syntax and examples to `@cluster`
- migrate scaffold/package/docs/verifier wording off `clustered(work)` and manifest clustering
- defer route-wrapper claims to later slice work

### 2. Public cutover vs actual rejection is still a real decision

There are two viable S04 interpretations:

1. **Public-surface cutover only**
   - examples/scaffold/docs/tests stop teaching `clustered(work)` and `[cluster]`
   - parser/pkg may still accept them as hidden compatibility for one more slice

2. **Real bridge end**
   - parser/pkg/compiler stop accepting legacy syntax/manifest clustering as normal input
   - diagnostics/tests move accordingly

The codebase is currently set up for both syntaxes to coexist. If the intent is a true hard cut, the compiler/pkg work must be planned explicitly.

Given D268 (“keep `clustered(work)` parser-compatible until S04”), the second reading is more faithful, but it is materially larger.

## Recommendation

Plan S04 as four ordered tasks, not one sweep.

### Task group 1 — Define the compatibility boundary

Decide first whether S04 ends the bridge in code or only in public surfaces.

If **hard cut in code**:
- parser: `compiler/mesh-parser/src/parser/items.rs`, `compiler/mesh-parser/src/ast/item.rs`, `compiler/mesh-parser/src/syntax_kind.rs`, `compiler/mesh-parser/tests/parser_tests.rs`
- pkg: `compiler/mesh-pkg/src/manifest.rs`
- compiler/LSP tests: `compiler/meshc/tests/e2e_m047_s01.rs`, mesh-lsp manifest-related tests

If **public cutover only**:
- leave parser/pkg acceptance alone for now
- do not keep legacy wording anywhere user-facing

This is the riskiest decision and should be resolved first.

### Task group 2 — Migrate the canonical clustered surfaces together

Change in one unit:
- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/meshc/tests/tooling_e2e.rs`
- `tiny-cluster/work.mpl`, `tiny-cluster/tests/work.test.mpl`, `tiny-cluster/README.md`
- `cluster-proof/work.mpl`, `cluster-proof/tests/work.test.mpl`, `cluster-proof/README.md`

Recommended target shape:
- keep `main.mpl` route-free and `Node.start_from_env()` only
- switch `work.mpl` to `@cluster`
- keep function name `execute_declared_work` so runtime name stays stable
- remove `declared_work_runtime_name()` if runtime-name stability is still preserved

Do this before docs, because the docs should describe the actual scaffold/package output.

### Task group 3 — Rewrite docs and top-level README

Update:
- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/distributed/index.md`
- `README.md`

Per the VitePress skill, stay inside content pages unless nav/sidebar text must change. Then run `npm --prefix website run build` as the site-level proof.

### Task group 4 — Add an M047-owned verification rail and demote historical M046 wording

Because no M047 S04 rail exists yet, add one. Likely components:
- new `compiler/meshc/tests/e2e_m047_s04.rs`
- new `scripts/verify-m047-s04.sh`

Reuse `compiler/meshc/tests/support/m046_route_free.rs` rather than writing a new harness. That helper already owns:
- clustered scaffold generation
- temp-path package builds
- runtime bootstrap/continuity polling
- retained artifact patterns

After the new M047 rail exists, update or alias the old M046/M045 verifier references so they stop being the public teaching surface.

## Natural Seams for the Planner

### Seam A — compiler compatibility
Independent if the planner decides S04 should actually end legacy acceptance.

### Seam B — public source surfaces
The scaffold, `tiny-cluster`, and `cluster-proof` should move together because parity tests already bind them together.

### Seam C — docs
Can be executed after source surfaces settle. Mostly markdown-only.

### Seam D — assembled proof
New M047 rail plus any historical alias cleanup. Reuse `m046_route_free` helpers.

## Verification Strategy

### Existing rails worth replaying during implementation

- `cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture`
- `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`
- `cargo run -q -p meshc -- build tiny-cluster`
- `cargo run -q -p meshc -- test tiny-cluster/tests`
- `cargo run -q -p meshc -- build cluster-proof`
- `cargo run -q -p meshc -- test cluster-proof/tests`
- `npm --prefix website run build`

### If hard cut includes parser/pkg behavior
Add or update named M047 parser/pkg/compiler rails, because the current legacy tests are numerous and explicit.

### New rail S04 needs
A truthful M047 S04 verifier should at minimum prove:
- canonical surfaces no longer teach `clustered(work)` or `[cluster]`
- scaffold/package examples still build and package smoke passes
- runtime truth still surfaces `Work.execute_declared_work` (or the chosen runtime name) through CLI continuity/diagnostics
- docs build and point at the new M047 verifier instead of M046 as the public closeout story

## Don’t Hand-Roll

Reuse `compiler/meshc/tests/support/m046_route_free.rs` for any new scaffold/package runtime rail. It already encodes the right patterns for:
- temp build outputs outside tracked package dirs
- retained build metadata
- two-node bootstrap/status/continuity/diagnostic polling
- artifact retention that historical verifiers already understand

Creating a second ad hoc clustered-package harness would just duplicate fragile process-control code.

## Planner Notes

- The easiest truthful public migration is **not** to rename the function. Keep `execute_declared_work` if you want runtime name continuity with less harness churn.
- The biggest hidden cost is not codegen/runtime; it is the dense web of exact-string tests and verifier references that still assert the M046 story.
- `tiny-cluster-prefered/` is unresolved drift. Don’t leave it as-is after S04.
- Do not let S04 silently start teaching `HTTP.clustered(...)` unless there is real code/test evidence for it. The retained S03 blocker says there is not.
