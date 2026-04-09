# S05 Research — Docs-First Example & Proof Closeout

## Summary

S05 is a docs-and-verifier closeout slice, not a runtime/compiler slice. The tiny scaffold-first clustered example already exists and is already proven on the happy path (S02) and failover path (S03). The remaining gap is that the docs IA still treats the clustered story as an embedded aside or routes straight into proof pages, while the current assembled public rail is still `bash scripts/verify-m045-s04.sh`.

The core S05 job is to make the scaffold-first clustered example a first-class docs page and migrate the current public closeout contract from S04 to S05 without breaking the replayable S04 subrail.

## Requirements Focus

Primary requirement ownership/support for this slice:

- **R081** — public docs must teach the simple scaffold-first clustered example first, with deeper proof rails secondary.
- **R077** — the primary clustered example must stay tiny and language-first.
- **R078** — the same tiny example still needs to carry cluster formation, remote execution, and failover truth.
- **R079** — the primary example/docs must not reintroduce app-owned cluster status/routing/failover logic.
- **R080** — `meshc init --clustered` must be the docs-grade entrypoint.

Secondary constraint: keep the existing clustered continuity/operator proof claims honest by **reusing** S02/S03/S04 rails instead of inventing a docs-only success story.

## Skills Discovered

- **`vitepress`** — already installed and directly relevant.
  - Relevant skill guidance used here: VitePress is file-based routing; new docs pages are created by adding Markdown files under `website/docs/docs/...`, while nav/sidebar wiring lives in `website/docs/.vitepress/config.mts`.
- No additional skill installs were needed.

## Current Implementation Landscape

### Authoritative public scaffold contract already exists

`compiler/mesh-pkg/src/scaffold.rs` is already the source of truth for the public clustered example. It generates:

- `main.mpl`
  - `Node.start_from_env()`
  - `BootstrapStatus`
  - `/health`
  - `POST /work/:request_key`
  - `Continuity.submit_declared_work(...)`
  - **no** app-owned status route
  - **no** `Node.start(...)`
  - **no** `Continuity.mark_completed(...)`
- `work.mpl`
  - `pub fn execute_declared_work(request_key :: String, attempt_id :: String) -> Int`
  - fixed `Timer.sleep(250)` demo delay for observable failover windows
- generated `README.md`
  - generic `MESH_*` contract
  - `meshc cluster status|continuity|diagnostics`
  - explicit note that the scaffold does **not** call `Continuity.mark_completed(...)`

This means S05 should **document and verify** the existing scaffold contract, not invent a new public clustered surface.

### The scaffold’s public exercise path is simpler than `cluster-proof`

This distinction is the biggest docs risk and needs to stay explicit.

**Scaffold-first example (`compiler/mesh-pkg/src/scaffold.rs`):**
- `/health`
- `POST /work/:request_key`
- runtime-owned CLI truth via:
  - `meshc cluster status <node> --json`
  - `meshc cluster continuity <node> <request_key> --json`
  - `meshc cluster diagnostics <node> --json`
- no HTTP status route
- no app-owned failover/operator/status translation

**`cluster-proof` deeper proof package:**
- `GET /membership`
- `POST /work` with JSON body
- `GET /work/:request_key`
- read-only Fly verifier
- proof-only env like `CLUSTER_PROOF_WORK_DELAY_MS`

So the new docs-grade page must mirror the scaffold contract, **not** the `cluster-proof` runbook.

### Current docs do not yet teach the scaffold-first story as a first-class path

#### `website/docs/.vitepress/config.mts`

- `Getting Started` sidebar group currently has only:
  - `Introduction`
  - `Production Backend Proof`
- There is **no** dedicated clustered example page in the sidebar.
- This is the main IA gap.

#### `website/docs/docs/getting-started/index.md`

- Mentions `meshc init --clustered`, but only inline.
- The clustered section is awkwardly inserted into the hello-world flow between:
  - “Open `main.mpl` — replace its contents with:”
  - and the hello-world code block.
- It is not a standalone clustered tutorial.
- It does not teach the two-node run or failover flow.

#### `website/docs/docs/tooling/index.md`

- Correctly documents `meshc init --clustered` and the runtime-owned CLI surfaces.
- But it routes clustered users straight to `/docs/distributed-proof/`.
- That makes proof pages, not the scaffold tutorial, the next step.

#### `website/docs/docs/distributed/index.md`

- Correctly stays a primitives guide (`Node.start`, `Node.connect`, etc.).
- Its top note also routes readers directly to `/docs/distributed-proof/`.
- It should stay generic, but likely needs to route to a new clustered-example page first, then to proof docs second.

#### `website/docs/docs/distributed-proof/index.md`

- Already positioned as a proof map, which is good.
- But it currently names `bash scripts/verify-m045-s04.sh` as the assembled local closeout rail.
- It starts from proof surfaces, not from a tutorial page.
- It should remain secondary and likely link back to the new scaffold-first docs page.

#### `README.md`

- Already mentions the clustered scaffold and CLI surfaces.
- Already positions `cluster-proof` as deeper dogfood proof consumer.
- Still names `bash scripts/verify-m045-s04.sh` as the current assembled local closeout rail.
- Would benefit from linking to the new docs-grade clustered page once it exists.

#### `cluster-proof/README.md`

- Correctly positioned as the deeper runbook.
- Still names `bash scripts/verify-m045-s04.sh` as the authoritative local closeout rail.
- Includes proof-only surfaces/env that should stay secondary.

### Theme/landing code likely does not need changes

From the VitePress theme components:

- `website/docs/.vitepress/theme/components/landing/HeroSection.vue`
- `website/docs/.vitepress/theme/components/landing/GetStartedCTA.vue`
- `website/docs/.vitepress/theme/components/landing/LandingFooter.vue`

all already route people to `/docs/getting-started/`.

That means a new page **inside Getting Started** is enough to improve discoverability. S05 probably does **not** need theme work unless the user explicitly wants a direct home-page clustered CTA.

## Verification / Contract State Today

### Existing current rail

`scripts/verify-m045-s04.sh` is the current assembled S04 rail. It already replays:

- `bash scripts/verify-m045-s02.sh`
- `bash scripts/verify-m045-s03.sh`
- `cargo test -p meshc --test e2e_m045_s04 m045_s04_ -- --nocapture`
- `cargo run -q -p meshc -- build cluster-proof`
- `cargo run -q -p meshc -- test cluster-proof/tests`
- `npm --prefix website run build`

So S05 should **reuse** this rail, not duplicate runtime/product proof logic.

### Current S04 docs contract is too shallow for S05

`compiler/meshc/tests/e2e_m045_s04.rs` only checks a narrow set of string-presence assertions across:

- `README.md`
- `cluster-proof/README.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/tooling/index.md`
- `scripts/verify-m045-s04.sh`

It does **not** cover:

- `website/docs/docs/getting-started/index.md`
- sidebar wiring in `website/docs/.vitepress/config.mts`
- existence of a first-class clustered tutorial page
- migration from “S04 is current” to “S05 is current”

### No S05 contract exists yet

Missing files right now:

- `compiler/meshc/tests/e2e_m045_s05.rs`
- `scripts/verify-m045-s05.sh`

That is the natural place to land the new docs-first contract.

## Recommendation

### Recommended public-docs shape

Create a first-class page under **Getting Started**.

Recommended route/path:

- `website/docs/docs/getting-started/clustered-example/index.md`

The exact slug can vary, but it should live under `Getting Started`, not be buried under the generic distributed primitives guide.

Recommended content shape for that page:

1. `meshc init --clustered`
2. what files the scaffold generates (`mesh.toml`, `main.mpl`, `work.mpl`, `README.md`)
3. what the public contract is:
   - `Node.start_from_env()`
   - `POST /work/:request_key`
   - `meshc cluster status|continuity|diagnostics`
4. two-node local run example
5. one submit example using `POST /work/<request_key>`
6. happy-path inspection via the runtime CLI
7. concise failover section on the **same scaffold**
8. only then link to:
   - `/docs/distributed-proof/`
   - `cluster-proof/README.md`

### Recommended verifier migration

Make `scripts/verify-m045-s05.sh` the new current closeout rail.

Use this structure:

- replay `bash scripts/verify-m045-s04.sh`
- run `cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture`
- run `npm --prefix website run build`
- retain copied evidence from `.tmp/m045-s04/verify/` and/or its `latest-failover-bundle.txt`

The best repo-local pattern for this is:

- `scripts/verify-m044-s05.sh`
- `compiler/meshc/tests/e2e_m044_s05.rs`

That pair already demonstrates the exact “new current rail + previous rail becomes historical transition checker” pattern S05 needs.

### Recommended S04 migration

Because public docs currently point at S04, S05 must update `compiler/meshc/tests/e2e_m045_s04.rs` in the **same task** that changes docs text.

Otherwise S04 will immediately go red once docs move to S05.

The cleanest pattern is:

- **S05** becomes the present-tense public contract.
- **S04** becomes a replayable historical cleanup/subrail checker.

## Natural Seams for Planning

### 1. Docs IA + content task

Likely files:

- `website/docs/.vitepress/config.mts`
- `website/docs/docs/getting-started/index.md`
- `website/docs/docs/getting-started/clustered-example/index.md` (new)
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `README.md`
- `cluster-proof/README.md`

Goal:
- make the scaffold-first clustered example a first-class docs page
- keep proof rails secondary
- move “current local closeout rail” wording from S04 to S05

### 2. Contract migration + verifier task

Likely files:

- `compiler/meshc/tests/e2e_m045_s05.rs` (new)
- `compiler/meshc/tests/e2e_m045_s04.rs`
- `scripts/verify-m045-s05.sh` (new)
- possibly `scripts/verify-m045-s04.sh` if its own wording/role needs narrowing

Goal:
- S05 becomes the current public docs/proof closeout rail
- S04 stays replayable without claiming present-tense ownership

### Optional third task only if desired

If the planner wants shell-level docs truth with explicit page/section/sidebar lists, `scripts/verify-m043-s04-proof-surface.sh` is the best template. But a Rust integration test similar to `e2e_m045_s04.rs` is probably enough if it covers:

- new page existence
- sidebar entry
- new current rail name
- routing from intro/tooling/distributed pages to the new page
- proof pages kept secondary

## Build / Proof Order

1. **Pick the public page path and final rail name first** (`verify-m045-s05.sh` + new docs route). Everything else depends on those strings.
2. Update or sketch the contract test expectations before rewriting all docs; S04/S05 migration is tightly coupled.
3. Change docs + sidebar.
4. Add the S05 wrapper verifier.
5. Run the final replay.

## Verification Commands

Existing useful commands:

- `bash scripts/verify-m045-s04.sh`
- `cargo test -p meshc --test e2e_m045_s04 m045_s04_ -- --nocapture`
- `npm --prefix website run build`

Likely new S05 commands:

- `cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture`
- `bash scripts/verify-m045-s05.sh`

## Risks / Constraints

- **Do not teach `cluster-proof` HTTP surfaces as if they belong to the scaffold.** The scaffold does not expose `GET /membership` or `GET /work/:request_key`.
- **Do not leak `CLUSTER_PROOF_WORK_DELAY_MS` into the primary docs-grade example.** That stays in deeper proof docs only.
- The generated scaffold README only covers single-node startup; the S05 docs page has to add the two-node/failover tutorial carefully from S02/S03 proof behavior.
- From existing milestone knowledge:
  - terminal clustered closeout verifiers should replay earlier product rails and retain copied evidence bundles instead of inventing a docs-only final gate
  - pointer files like `latest-proof-bundle.txt` must contain only the retained directory path
  - avoid `bash -lc` inside new verifier scripts on this host; direct repo-local commands are safer
- S02 explicitly noted that pre-submit standby-side operator probing can destabilize the first remote-owned submit on this host. For manual docs examples, prefer:
  - ingress-side cluster status first
  - submit once
  - then continuity/diagnostics inspection

## Planner Takeaway

There is no missing runtime feature here. The public scaffold, happy-path rail, failover rail, and cleanup rail already exist. S05 should:

1. add a first-class clustered tutorial page under Getting Started,
2. reroute the surrounding docs/readmes so that page is the first stop,
3. promote the assembled present-tense contract from S04 to S05,
4. keep S04 replayable as a historical cleanup subrail,
5. reuse `verify-m045-s04.sh` instead of rebuilding product proof logic.
