# S05 Research — Equal-surface scaffold alignment

## Summary

- **Primary requirement:** `R090`.
- **Directly supported requirements:** `R086`, `R091`, `R092`, and `R093`.
- This slice is a **scaffold + docs + verifier alignment** slice, not new runtime/compiler semantics. The runtime-owned clustered contract already exists in `tiny-cluster/` (S03) and `cluster-proof/` (S04); the remaining work is to make `meshc init --clustered`, public docs, and verifier rails tell the **same route-free story** and fail closed if they drift.
- The current repo state is split cleanly:
  - `tiny-cluster/` and `cluster-proof/` are already on the new route-free contract.
  - `meshc init --clustered` is still on the old contract: `[cluster]` manifest declarations, HTTP `/health` + `/work/:request_key`, app-owned `Continuity.submit_declared_work(...)`, app-owned replica-count logic, and non-trivial delayed work.
- I verified the fast scaffold tests currently pass **against the old contract**, so S05 must update them in the same change set:
  - `cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture`
  - `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`
- The public docs are also still narrating the old routeful scaffold/package story. The biggest stale surfaces are:
  - `website/docs/docs/getting-started/clustered-example/index.md`
  - `website/docs/docs/tooling/index.md`
  - `website/docs/docs/distributed-proof/index.md`
  - `website/docs/docs/distributed/index.md`
  - `README.md`
- There is **no `m046_s05` verifier/test rail yet**. The current assembled wrapper `scripts/verify-m045-s05.sh` and `compiler/meshc/tests/e2e_m045_s05.rs` explicitly encode the opposite contract: they delegate only to S04 and assert that scaffold/docs parity is still deferred.

## Skills Discovered

No new skill installs were needed; the directly relevant skills are already present.

- `rust-best-practices` (consulted)
  - Relevant rule used here: prefer **focused, descriptive tests** and avoid sprawling duplicated assertions when a shared helper or fixture seam exists.
  - Implication for S05: do not update five scaffold-contract tests by hand with slightly different literal lists if a shared route-free scaffold helper/normalizer can own the contract once.
- `vitepress` (consulted)
  - Relevant rule used here: VitePress uses **file-based routing**; doc work is usually Markdown content changes plus optional sidebar/config edits, not special app wiring.
  - Implication for S05: `website/docs/.vitepress/config.mts` already has sidebar entries for **Clustered Example** and **Distributed Proof**, so the main work is rewriting the Markdown pages truthfully, not adding routing infrastructure.

## Implementation Landscape

### 1. The proof packages are already almost aligned

`tiny-cluster/` and `cluster-proof/` already provide the target clustered-app contract.

Shared route-free core:
- `tiny-cluster/work.mpl` and `cluster-proof/work.mpl` are currently **identical**.
- Both keep:
  - `pub fn declared_work_runtime_name() -> String do "Work.execute_declared_work" end`
  - `clustered(work) pub fn execute_declared_work(...) -> Int do 1 + 1 end`
- `tiny-cluster/main.mpl` and `cluster-proof/main.mpl` differ only by the log prefix (`[tiny-cluster]` vs `[cluster-proof]`).
- Both manifests are package-only and omit `[cluster]`; they differ only in package metadata (`name`, `description`).

This is important because it means S05 does **not** need to invent a new clustered contract. It can treat S03/S04 as the canonical shape and make the scaffold converge on that shape.

### 2. The scaffold is the remaining outlier

`compiler/mesh-pkg/src/scaffold.rs` still generates the legacy clustered app contract.

Current generated drift:
- `mesh.toml` includes:
  - `[cluster]`
  - `enabled = true`
  - `declarations = [{ kind = "work", target = "Work.execute_declared_work" }]`
- `main.mpl` includes:
  - `Env.get_int("PORT", 8080)`
  - `HTTP.serve(...)`
  - `/health`
  - `/work/:request_key`
  - `Continuity.submit_declared_work(...)`
  - app-owned `current_required_replica_count()` based on `Node.list()`
- `work.mpl` includes:
  - no `clustered(work)` marker
  - no `declared_work_runtime_name()` helper
  - `Timer.sleep(250)`
  - non-trivial string-length work based on request/attempt ids
- `README.md` describes:
  - an HTTP app surface (`PORT`)
  - manifest-owned declaration in `mesh.toml`
  - app-owned submission through `Continuity.submit_declared_work(...)`

I generated a temporary scaffold and diffed it against `tiny-cluster/`. The deltas are large and structural, not cosmetic:
- scaffold `main.mpl` is still the 59-line routeful app shell; `tiny-cluster/main.mpl` is the 16-line route-free bootstrap-only file.
- scaffold `work.mpl` is still missing the source declaration and trivial `1 + 1` body.
- scaffold `README.md` is a different operator story entirely.

### 3. The fast scaffold tests currently lock in the old contract

These files all assert some part of the routeful scaffold story and will need to move together:

- `compiler/mesh-pkg/src/scaffold.rs`
  - unit test `scaffold_clustered_project_writes_public_cluster_contract()` currently expects `[cluster]`, `Continuity.submit_declared_work`, and the old README wording.
- `compiler/meshc/tests/tooling_e2e.rs`
  - `test_init_clustered_creates_project()` currently asserts `[cluster]` and `Continuity.submit_declared_work`.
- `compiler/meshc/tests/e2e_m044_s03.rs`
  - scaffold rail currently boots the generated app, waits for `/health`, and validates routeful runtime behavior.
- `compiler/meshc/tests/e2e_m045_s01.rs`
  - scaffold contract still expects `Continuity.submit_declared_work` in generated `main.mpl`.
- `compiler/meshc/tests/e2e_m045_s02.rs`
  - scaffold runtime-completion rails still POST to `/work/{request_key}` and wait on `/health`.
- `compiler/meshc/tests/e2e_m045_s03.rs`
  - scaffold failover rails still depend on `/health` and `/work`.

This is the main test blast radius. If S05 flips the scaffold first without a coordinated test update/repoint, these historical rails will break.

### 4. The public docs still describe the old routeful story

#### `website/docs/docs/getting-started/clustered-example/index.md`
This page is the biggest rewrite target. It currently teaches:
- `[cluster]` in `mesh.toml`
- `Continuity.submit_declared_work(...)`
- `GET /health`
- `POST /work/:request_key`
- manual request submission with `curl`
- failover by editing `Timer.sleep(250)` to `Timer.sleep(5000)`

That is now directly at odds with S03/S04.

#### `website/docs/docs/tooling/index.md`
The tooling page still says `meshc init --clustered` adds:
- a `[cluster]` declaration in `mesh.toml`
- a declared-work boundary in `work.mpl`
- the old M045 proof/verifier names in the narrative block

The CLI surface docs should instead match the new route-free scaffold and likely point to the S05/S06-aligned verifier names once they exist.

#### `website/docs/docs/distributed-proof/index.md`
This page is still on the M045 story:
- calls `cluster-proof` the public proof target
- names `bash scripts/verify-m045-s05.sh` as the current closeout verifier
- still talks about runtime-owned keyed `POST /work` / `GET /work/:request_key`
- still includes the read-only Fly sanity path as a live part of the public proof map

That is stale after S04 and likely needs a new three-surface canonical framing for S05.

#### `website/docs/docs/distributed/index.md`
The distributed actors guide’s top proof callout still points readers at the M045 verifier names and the older `cluster-proof`/Fly framing.

#### `README.md`
The repo README still says the clustered scaffold adds a `[cluster]` declaration and references `bash scripts/verify-m045-s05.sh` as the current closeout rail. It also still frames `cluster-proof` as the deeper dogfood proof behind the scaffold-first story instead of showing the three surfaces as equal canonical examples.

### 5. One subtle intra-proof drift already exists: continuity list vs single-record docs

The two proof READMEs are close, but they are not perfectly aligned:
- `tiny-cluster/README.md` documents `meshc cluster continuity <node-name@host:port> --json`
- `cluster-proof/README.md` documents `meshc cluster continuity <node-name@host:port> <request-key> --json`

`compiler/meshc/src/cluster.rs` makes the actual CLI contract explicit:
- request key is **optional**
- omitted request key => list recent continuity records
- provided request key => inspect one record

S05 should pick one canonical operator sequence and use it everywhere. Best recommendation: document both forms in a consistent order:
1. `status`
2. `continuity` list (discover request key/runtime name)
3. `continuity` record (inspect the chosen request)
4. `diagnostics`

That matches the startup-owned proof flow better than pretending users always start with a known request key.

### 6. The existing route-free test support is the right reuse seam

`compiler/meshc/tests/support/m046_route_free.rs` already owns the hard parts of the new proof style:
- temp `meshc build --output` handling with pre-created parents
- retained `build-meta.json`
- process spawn/log capture
- `meshc cluster status|continuity|diagnostics` polling helpers
- startup diagnostics / continuity matching helpers

This is the obvious place to extend if S05 adds a scaffolded route-free runtime rail. Do **not** build a second bespoke scaffold harness if the same support module can build a generated temp project and run the same CLI checks.

### 7. The current wrapper verifier for S05 encodes the wrong contract on purpose

Current state:
- `scripts/verify-m045-s05.sh` delegates only to `scripts/verify-m046-s04.sh`
- `compiler/meshc/tests/e2e_m045_s05.rs` asserts that `verify-m045-s05.sh` omits:
  - `Clustered Example`
  - `distributed-proof`
  - `tooling/index.md`
  - scaffold/docs parity coverage entirely

This is the cleanest fail-closed seam to flip in S05. Rather than mutating S04’s verifier, S05 should likely:
- add a new authoritative `scripts/verify-m046-s05.sh`
- add a matching `compiler/meshc/tests/e2e_m046_s05.rs`
- repoint `scripts/verify-m045-s05.sh` and `compiler/meshc/tests/e2e_m045_s05.rs` to delegate to / assert the new S05 verifier

### 8. Scope trap: old historical routeful proof scripts still exist

There are many older routeful proof scripts/tests (`m039`, `m042`, `m043`, and some `m044`/`m045` historical rails). Most of them are not the right place to implement S05.

Important distinction:
- **active S05 seams to update now:** scaffold generator, scaffold-facing tests, current public docs, current assembled wrapper/verifier
- **likely defer unless the regression suite forces it:** old routeful proof-surface scripts that are already historical and not part of the current authoritative story

## Recommendation

### 1. Treat S05 as behavioral alignment, not byte-for-byte identity

The three surfaces do not need to be text-identical:
- scaffold needs generated project naming and a user-facing README runbook
- `cluster-proof/` legitimately has packaging-only files (`Dockerfile`, `fly.toml`)
- `tiny-cluster/` is a repo-owned local proof package

What **does** need to align everywhere:
- package-only `mesh.toml` (no `[cluster]` for the canonical source-first path)
- `main.mpl` with one `Node.start_from_env()` bootstrap path
- `work.mpl` with one source `clustered(work)` declaration
- visible work body `1 + 1`
- runtime-owned startup/status/diagnostics story
- no HTTP routes, no `Continuity.submit_declared_work(...)`, no timing knobs

### 2. Make scaffold `work.mpl` match the proof packages as closely as possible

This is the easiest win and the strongest anti-drift move.

Recommended scaffold `work.mpl` shape:
- include `declared_work_runtime_name()`
- include the same `clustered(work)` marker
- keep the same `Work.execute_declared_work` runtime name
- keep the same `1 + 1` body

There is no obvious reason for the scaffold work file to differ from `tiny-cluster/` / `cluster-proof/`.

### 3. Make scaffold `main.mpl` match the route-free package structure

Recommended scaffold `main.mpl` shape:
- `log_bootstrap(status :: BootstrapStatus)`
- `log_bootstrap_failure(reason :: String)`
- `main()` that only calls `Node.start_from_env()` and logs

A generic prefix like `[clustered-app]` is fine, but the control flow should converge on the same 3-function route-free structure as the proof packages.

### 4. Keep the generated scaffold README scaffold-specific, but align its narrative structure

The scaffold README still needs user-facing run instructions, unlike the repo proof packages. Keep the env var section, but change the clustered story to:
- source-owned `clustered(work)` declaration in `work.mpl`
- package-only `mesh.toml`
- automatic startup work on boot
- CLI-only inspection with `meshc cluster status|continuity|diagnostics`
- no HTTP submit/status routes
- cross-reference the repo proof siblings (`tiny-cluster/`, `cluster-proof/`) as equal canonical examples, not as “real vs toy” surfaces

### 5. Add one authoritative S05 verifier instead of spreading checks ad hoc

Recommended shape:
- `scripts/verify-m046-s05.sh` becomes the authoritative alignment verifier
- it should replay the S03 and S04 verifiers, then run scaffold/docs alignment checks, then build docs
- `scripts/verify-m045-s05.sh` becomes a thin historical alias wrapper over `verify-m046-s05.sh`

That gives S06 one stable verifier to assemble instead of another one-off wrapper chain.

## Natural task seams

### Seam A — rewrite the scaffold generator and its unit-level contract

Files:
- `compiler/mesh-pkg/src/scaffold.rs`

Work:
- replace the clustered scaffold templates with the route-free source-first contract
- update the embedded unit tests to assert the new contract

Why first:
- this is the source of truth for `meshc init --clustered`
- every downstream test/doc change depends on the final emitted files

### Seam B — align scaffold-facing Rust tests and add shared contract helpers

Files:
- `compiler/meshc/tests/tooling_e2e.rs`
- `compiler/meshc/tests/support/mod.rs`
- likely a new helper under `compiler/meshc/tests/support/`
- `compiler/meshc/tests/e2e_m046_s05.rs` (new)
- `compiler/meshc/tests/e2e_m045_s05.rs`
- potentially focused updates or repoints for:
  - `compiler/meshc/tests/e2e_m044_s03.rs`
  - `compiler/meshc/tests/e2e_m045_s01.rs`
  - `compiler/meshc/tests/e2e_m045_s02.rs`
  - `compiler/meshc/tests/e2e_m045_s03.rs`

Work:
- add one shared scaffold/proof contract helper instead of duplicating literal contain/omit lists
- add a new S05 route-free scaffold alignment rail
- flip the current M045 S05 wrapper tests from “docs parity deferred” to “delegates to authoritative S05 verifier”

Why second:
- the scaffold changes will instantly invalidate the old fast tests
- the new verifier needs a Rust-side contract rail to pin the current story

### Seam C — rewrite the public docs and README around three equal canonical surfaces

Files:
- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/distributed/index.md`
- `README.md`

Work:
- remove routeful scaffold instructions (`[cluster]`, `/health`, `/work`, `Continuity.submit_declared_work`, `Timer.sleep(...)` failover edits)
- standardize the CLI inspection sequence
- present scaffold, `tiny-cluster/`, and `cluster-proof/` as equal canonical surfaces
- rename proof/verifier references from the M045 wrapper story to the new M046 S05 truth

Why third:
- the docs should be rewritten against the finished scaffold contract, not guessed ahead of it

### Seam D — add the authoritative S05 shell verifier and repoint the wrapper alias

Files:
- `scripts/verify-m046-s05.sh` (new)
- `scripts/verify-m045-s05.sh`

Work:
- create the direct alignment verifier with phase-report/status/current-phase/latest-bundle artifacts
- retain delegated S03/S04 artifacts the same way S04 retained S03
- add docs/scaffold contract guard phases and a `npm --prefix website run build` phase
- repoint the M045 wrapper to delegate to the new S05 verifier

Why last:
- the verifier should lock the final code/docs state, not chase a moving target

## Don’t Hand-Roll

- **Do not** keep both `[cluster]` and source `clustered(work)` in the scaffold. S01 intentionally fails closed on duplicate manifest/source declarations.
- **Do not** preserve `/health`, `/work`, `Continuity.submit_declared_work(...)`, or app-owned replica-count logic in the generated scaffold “just for getting-started convenience.” That would break the whole honesty bar of M046.
- **Do not** create a second bespoke route-free runtime harness for scaffold tests when `compiler/meshc/tests/support/m046_route_free.rs` already owns the hard parts.
- **Do not** make the docs pretend one surface is the “real” one and the others are secondary demos. That is the requirement this slice is closing.
- **Do not** widen the slice into updating every historical routeful verifier unless current regression coverage actually requires it.
- **Do not** document only the single-record `meshc cluster continuity <target> <request-key>` form. Startup-owned work often starts from the list view; docs should reflect that.

## Risks / Unknowns

- **Historical scaffold rails may need an explicit strategy.**
  - `e2e_m044_s03.rs`, `e2e_m045_s01.rs`, `e2e_m045_s02.rs`, and `e2e_m045_s03.rs` still encode the old routeful scaffold. The planner should decide early whether to rewrite them for the new route-free story or reduce them to historical alias coverage so they do not block the slice late.
- **Docs need a canonical continuity workflow decision.**
  - `tiny-cluster/README.md` and `cluster-proof/README.md` already diverge on list vs request-key continuity usage. S05 should fix that intentionally rather than accidentally copying one side.
- **README scope differs across surfaces.**
  - The scaffold README still needs user-run env guidance; the proof READMEs do not. Keep the user-facing runbook while aligning the clustered story.
- **Older historical proof-surface scripts still exist.**
  - If the planner edits public docs aggressively, some unowned historical script expectations may become stale. Confirm whether they are still in any active verification chain before spending slice budget on them.

## Verification

Recommended execution contract for S05:

1. Fast scaffold generator checks
   - `cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture`
   - `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`

2. New/updated scaffold alignment rail
   - `cargo test -p meshc --test e2e_m046_s05 m046_s05_ -- --nocapture`

3. Route-free proof regressions if shared support changes
   - `cargo test -p meshc --test e2e_m046_s03 m046_s03_ -- --nocapture`
   - `cargo test -p meshc --test e2e_m046_s04 m046_s04_ -- --nocapture`

4. Docs build
   - `npm --prefix website run build`

5. Direct verifier + historical alias wrapper
   - `bash scripts/verify-m046-s05.sh`
   - `bash scripts/verify-m045-s05.sh`

Useful baseline note: the current fast scaffold tests are green, but they are green on the **old** routeful contract, so they must be updated atomically with the generator.
