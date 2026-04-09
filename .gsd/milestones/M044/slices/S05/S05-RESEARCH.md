# S05 Research — Cluster-Proof Rewrite, Docs, and Final Closeout

## Summary
S05 owns the last product-boundary cleanup for M044, not new distributed semantics. The risky work is concentrated in three places: `cluster-proof` still carries a legacy explicit-clustering probe path, it still uses a proof-app-specific env/config contract instead of the public clustered-app `MESH_*` contract, and the public docs/verifiers still describe the proof surface as if S05 were pending.

The runtime-owned keyed continuity path is already real. `cluster-proof` now submits declared work through `Continuity.submit_declared_work(...)` and reads typed continuity/authority state, but the package still exposes `GET /work` via `WorkLegacy`, still ships `CLUSTER_PROOF_*` env names in code/docs/runtime packaging, and still has dead/manual leftovers in `work_continuity.mpl`.

S05 should be planned as a targeted closeout slice:
1. align `cluster-proof` with the public clustered-app contract,
2. delete the old explicit legacy probe path,
3. add a dedicated closeout verifier that proves the docs/source boundary mechanically.

## Requirement Focus
- **R069** — full `cluster-proof` rewrite onto the public clustered-app standard, with the old explicit clustering path removed from its code.
- **R070** — move the primary docs/proof story to “build a clustered Mesh app,” with `cluster-proof` as the deeper proof consumer/runbook rather than the first abstraction users meet.
- Preserve already-validated M044 behavior, especially:
  - S03 public scaffold/operator truth (`meshc init --clustered`, `meshc cluster ...`)
  - S04 bounded automatic promotion + recovery (`bash scripts/verify-m044-s04.sh`)

## Skills Discovered
- **Loaded:** `vitepress`
  - Relevant rule used here: inspect the VitePress site config before proposing new docs plumbing. `website/docs/.vitepress/config.mts` already wires the existing `Distributed Proof` page (`distributed-proof` entry at line 119), so S05 is content/verifier work, not route/sidebar creation.
- **Already installed and relevant if execution widens:** `multi-stage-dockerfile`, `flyio-cli-public`, `github-workflows`
- **New installs:** none needed. The core external technologies for this slice already have installed skills.

## Implementation Landscape

### 1) `cluster-proof` still carries the old explicit path
Evidence:
- `cluster-proof/main.mpl:21,98,105-107`
  - imports `WorkLegacy`
  - starts legacy work services
  - wires `GET /work` alongside the keyed `POST /work` / `GET /work/:request_key` path
- `cluster-proof/work_legacy.mpl`
  - entire legacy routing probe implementation
- `cluster-proof/work.mpl:3-228`
  - `TargetSelection`, canonical placement, `current_target_selection(...)`
  - all probe-era routing/placement helpers still live here
- `cluster-proof/work_continuity.mpl`
  - still imports `TargetSelection` / `current_target_selection`
  - `run_legacy_probe_record(...)` at ~747
  - `submit_from_selection(...)` at ~893
  - legacy dispatch helpers around `dispatch_work(...)`
  - dead manual-promotion leftovers (`promotion_response_status_code(...)` around line 137, `log_promotion*` around line 574)
  - the real hot path is `handle_valid_submit(...)` using `Continuity.submit_declared_work(...)` around line 936, then `handle_work_submit` / `handle_work_status` near the end of the file

What this means:
- The runtime-owned declared-work/status path is already the truthful clustered surface.
- The remaining explicit-clustering code is mostly local to `cluster-proof/`, package tests, and verifier/source-absence checks.
- This is a cleanup/removal slice, not a new runtime feature slice.

### 2) `cluster-proof` still uses the proof-app env dialect instead of the public clustered-app contract
Evidence:
- `cluster-proof/main.mpl:35` reads `CLUSTER_PROOF_COOKIE`
- `cluster-proof/config.mpl` defines:
  - `CLUSTER_PROOF_COOKIE`
  - `CLUSTER_PROOF_NODE_BASENAME`
  - `CLUSTER_PROOF_ADVERTISE_HOST`
  - `CLUSTER_PROOF_DURABILITY`
- `cluster-proof/docker-entrypoint.sh` duplicates the same proof-specific contract in shell
- `cluster-proof/fly.toml` still sets `CLUSTER_PROOF_DURABILITY`
- `cluster-proof/README.md` documents the same proof-specific env surface

Contrast with the public clustered scaffold already shipped in `compiler/mesh-pkg/src/scaffold.rs`:
- scaffolded app uses:
  - `MESH_CLUSTER_COOKIE`
  - `MESH_NODE_NAME`
  - `MESH_DISCOVERY_SEED`
  - `MESH_CLUSTER_PORT`
  - `MESH_CONTINUITY_ROLE`
  - `MESH_CONTINUITY_PROMOTION_EPOCH`
- scaffold tests explicitly assert the generated app does **not** contain `CLUSTER_PROOF_*`

What this means:
- S05’s core rewrite seam is aligning `cluster-proof` with the scaffold/public clustered-app contract.
- The planner should decide up front whether S05 is a hard cut to `MESH_*` or a public switch with temporary compatibility aliases. That decision affects code, harnesses, Docker/Fly, and docs together.

### 3) The M044 harnesses still spawn `cluster-proof` with old env names
Evidence:
- `compiler/meshc/tests/e2e_m044_s03.rs:191-198`
  - `CLUSTER_PROOF_COOKIE`
  - `CLUSTER_PROOF_NODE_BASENAME`
  - `CLUSTER_PROOF_ADVERTISE_HOST`
  - `CLUSTER_PROOF_WORK_DELAY_MS`
- `compiler/meshc/tests/e2e_m044_s04.rs:202-212`
  - same proof-app env dialect on the failover rail
- package tests still pin legacy/proof-only behavior:
  - `cluster-proof/tests/work.test.mpl` checks `legacy_target_node(...)`, `TargetSelection`, and `promotion_response_status_code(...)`
  - `cluster-proof/tests/config.test.mpl` is built around the current proof-specific config module

What this means:
- If execution flips the app contract, these M044 rails must change in the same task.
- The old env names also still exist in older M039–M043 cluster-proof e2es and read-only Fly verifiers. A hard cut without compatibility or caller updates will widen the blast radius beyond M044-specific rails.

### 4) Public docs are partially updated, but the closeout story is still incomplete
Concrete stale spots:
- `website/docs/docs/tooling/index.md:230`
  - still says automatic promotion and the final `cluster-proof` rewrite remain later distributed slices
- `website/docs/docs/distributed/index.md:220`
  - still calls Distributed Proof the canonical public proof surface for the **M043** failover contract
- `website/docs/docs/distributed-proof/index.md`
  - still leads with `cluster-proof` as the proof target and still references `scripts/verify-m043-s04-fly.sh`
- `cluster-proof/README.md`
  - still documents `GET /work — legacy routing probe`
  - still documents `CLUSTER_PROOF_*` envs
  - still positions the package as the deep runbook for the bounded failover story

Already-aligned/public-story files:
- `README.md` points to distributed proof and the deeper runbook
- `website/docs/docs/getting-started/index.md` already teaches `meshc init --clustered`
- `compiler/mesh-pkg/src/scaffold.rs` already embodies the public clustered-app contract
- VitePress route/sidebar wiring already exists; no new docs route is required unless S05 deliberately adds one

What this means:
- S05 docs work is rewrite-and-tighten, not docs-site plumbing.
- The primary story can move scaffold-first/public-contract-first without changing site structure.

### 5) Existing verifier coverage is not enough for S05 closeout
Evidence:
- `scripts/verify-m044-s04.sh`
  - authoritative behavior rail
  - doc check is intentionally shallow: a few literal checks in four files
- `scripts/verify-m043-s04-proof-surface.sh`
  - strong prior art for exact-literal docs verification
  - checks canonical command lists, stale wording rejection, README/docs/runbook alignment, and VitePress sidebar wiring
- there is no `scripts/verify-m044-s05.sh` or `compiler/meshc/tests/e2e_m044_s05.rs`

What this means:
- S05 likely needs a dedicated assembled closeout rail instead of overloading `verify-m044-s04.sh` with more greps.
- The M043 proof-surface script is the best local template for the docs-truth half of S05.

## Natural Seams / Task Split

### Seam A — Rewrite `cluster-proof` onto the public clustered-app contract
Primary files:
- `cluster-proof/main.mpl`
- `cluster-proof/config.mpl`
- `cluster-proof/docker-entrypoint.sh`
- `cluster-proof/fly.toml`
- `cluster-proof/README.md`
- `compiler/meshc/tests/e2e_m044_s03.rs`
- `compiler/meshc/tests/e2e_m044_s04.rs`

Goal:
- move `cluster-proof` from the proof-app env dialect onto the public `MESH_*` contract the scaffold already uses
- decide whether any proof-only knob remains (for example, a work-delay env used only by destructive rails)

Planning note:
- This is the first decision to make, because it determines how much compatibility work the slice needs.

### Seam B — Remove the old explicit clustering path
Primary files:
- `cluster-proof/work_legacy.mpl`
- `cluster-proof/work.mpl`
- `cluster-proof/work_continuity.mpl`
- `cluster-proof/main.mpl`
- `cluster-proof/tests/work.test.mpl`

Goal:
- remove `GET /work` legacy probe surface and the app-owned placement/dispatch helpers that only existed for it
- remove dead manual-promotion leftovers from `work_continuity.mpl`
- keep only the keyed declared-work submit/status path and typed runtime authority/status surfaces

Planning note:
- `work.mpl` is mixed-use today. It still contains keyed-path payload types (`WorkSubmitBody`, `WorkStatusPayload`), request-key validation, and `effective_work_node_name()`. Expect a split or shrink, not a blind delete.

### Seam C — Docs / runbook rewrite + closeout verifier
Primary files:
- `README.md`
- `website/docs/docs/getting-started/index.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `cluster-proof/README.md`
- likely new `scripts/verify-m044-s05.sh`
- possibly updated references to `scripts/verify-m043-s04-fly.sh`

Goal:
- make “build a clustered Mesh app” the primary public story
- keep `cluster-proof` as the deeper proof consumer/runbook
- encode the wording contract mechanically so it cannot drift again

Planning note:
- the VitePress page route already exists; this is content + verifier work unless the team intentionally wants a new page

## Recommended Build Order
1. **Decide the target contract first**
   - Does S05 make `cluster-proof` fully `MESH_*`-first now?
   - Or does it publish `MESH_*` publicly while keeping internal compatibility aliases until old harnesses are retired?
2. **Remove the legacy path once the startup/env contract is fixed**
   - delete `WorkLegacy`
   - shrink/split `Work`
   - strip dead promotion leftovers from `WorkContinuity`
3. **Close the slice with a dedicated verifier**
   - replay S04 behavior
   - prove source-boundary cleanup
   - prove the scaffold-first/docs truth mechanically

This order minimizes rework. The distributed behavior is already proven; the main risk is product-boundary drift.

## Verification Surface
Existing rails that must remain green:
- `bash scripts/verify-m044-s04.sh`
- `cargo run -q -p meshc -- build cluster-proof`
- `cargo run -q -p meshc -- test cluster-proof/tests`
- `cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture`
- `cargo test -p meshc --test e2e_m044_s03 m044_s03_scaffold_ -- --nocapture`
- `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`
- `npm --prefix website run build`

S05-specific checks worth adding:
- source-absence checks for:
  - `WorkLegacy`
  - `handle_work_probe`
  - legacy `GET /work` route wiring
  - app-owned placement helpers in the new keyed hot path
- docs truth checks for stale wording:
  - “remain later distributed slices”
  - “M043 failover contract” on the public M044 proof surface
  - old verifier names where S05 wants M044-closeout wording
- if the env contract changes publicly, source/doc absence checks for `CLUSTER_PROOF_*` outside intentional proof-only/internal knobs

## Key Risks / Constraints
- **Old milestone harnesses still use `CLUSTER_PROOF_*`.** A hard public-contract cut without compatibility or caller updates will create regression noise outside M044.
- **`work.mpl` is not purely legacy.** It still owns keyed-path models and validation helpers.
- **`work_continuity.mpl` mixes dead and live code.** This is a careful cleanup task, not a straight delete.
- **Docs routing is already correct.** Per the loaded `vitepress` skill, check the site config before adding routes; existing `Distributed Proof` wiring is already present, so extra site-plumbing work is probably wasted.
- **Observed generated-output churn:** package build activity dirtied `cluster-proof/cluster-proof` in the working tree. Treat package outputs as generated noise, not as source-of-truth planning seams.

## Recommendation
Plan S05 as a three-part closeout:
1. rewrite `cluster-proof` startup/config onto the public clustered-app contract,
2. remove `WorkLegacy` and the remaining explicit clustering helpers,
3. add a dedicated S05 verifier that replays S04 behavior and mechanically proves the docs/source boundaries.

That is smaller and safer than inventing any new runtime behavior. The runtime semantics are already there; S05 is about making the product boundary honest and consistent.

## Resume Notes
- The cleanest deletion seam for R069 is the `GET /work` path still wired from `cluster-proof/main.mpl:105` through `cluster-proof/work_legacy.mpl`.
- The highest-leverage prior art for the docs closeout is `scripts/verify-m043-s04-proof-surface.sh`; reuse its literal/stale-wording pattern instead of bolting more ad hoc grep onto `verify-m044-s04.sh`.
- The one public docs line that is definitely stale right now is `website/docs/docs/tooling/index.md:230`.
