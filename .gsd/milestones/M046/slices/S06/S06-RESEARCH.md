# S06 Research — Assembled verification and docs closeout

## Summary

S06 is **closeout/composition work, not new runtime/compiler work**.

The repo already has the three proof layers S06 needs to assemble:

- **Local proof:** `scripts/verify-m046-s03.sh` + `compiler/meshc/tests/e2e_m046_s03.rs`
- **Packaged proof:** `scripts/verify-m046-s04.sh` + `compiler/meshc/tests/e2e_m046_s04.rs`
- **Equal-surface scaffold/docs proof:** `scripts/verify-m046-s05.sh` + `compiler/meshc/tests/e2e_m046_s05.rs`

What is still missing is the **final authoritative S06 wrapper** that republishes those lower rails as one milestone-closeout proof surface, updates docs/readmes to point at it, and likely produces the milestone validation artifact after the assembled rail is green.

There is **no S06 implementation yet** beyond the stub plan header in `.gsd/milestones/M046/slices/S06/S06-PLAN.md`.

## Requirements Focus

S06 primarily supports and/or re-proves the already-landed M046 clustered contract rather than introducing new product behavior.

Most relevant requirements for this slice:

- **R086** — assembled proof should keep the runtime-owned startup/failover/recovery/status story as the only truthful surface
- **R091** — assembled proof should show `meshc cluster status|continuity|diagnostics` is sufficient end to end
- **R090** — scaffold / `tiny-cluster/` / `cluster-proof/` stay equally canonical under the final closeout rail
- **R092** — public docs must point at the final route-free closeout rail, not the now-intermediate S05 rail

Secondary regression coverage that S06 should preserve:

- **R088 / R089 / R093** — local and packaged tiny proofs stay route-free and trivial (`1 + 1`)
- **R087** — no app-owned submit/status route reappears in the assembled public story

Constraint from project knowledge: the **GSD requirements DB still does not know about the M046 requirement family**; visible truth is the checked-in requirements/decisions state, not `gsd_requirement_update`.

## Skills Discovered

Relevant installed skills already exist; no new installs are needed.

- **`rust-testing`** — directly relevant for new `e2e_m046_s06.rs` / wrapper-contract updates
- **`vitepress`** — directly relevant for the website docs closeout under `website/docs/docs/`

`npx skills find` was run for Rust testing and VitePress; repo/user skill inventory already covers both.

## Recommendation

Treat S06 as a **new authoritative wrapper layer** over S05, not as another independent runtime proof stack.

Recommended shape:

1. **Add `scripts/verify-m046-s06.sh`** as the new authoritative milestone closeout rail.
   - Delegate to `bash scripts/verify-m046-s05.sh`
   - Retain the delegated `.tmp/m046-s05/verify/` directory under an S06-owned artifact root
   - Re-run only the **targeted truth rails** that S06 wants to foreground in the final milestone story:
     - local startup/failover/status truth from `e2e_m046_s03`
     - packaged startup/status truth from `e2e_m046_s04`
     - scaffold/docs parity already covered by delegated `verify-m046-s05.sh`
   - Publish one new `.tmp/m046-s06/verify/latest-proof-bundle.txt`

2. **Add `compiler/meshc/tests/e2e_m046_s06.rs`** for the new verifier/docs contract.
   - Follow the S05 pattern: test the script contents, expected phase names, retained bundle names, and authoritative/historical doc references
   - Do **not** create a second low-level cluster harness in this file

3. **Repoint the historical wrapper layer.**
   - `scripts/verify-m045-s05.sh` currently delegates to `verify-m046-s05.sh`
   - For S06, the truthful pattern is probably to make `verify-m045-s05.sh` delegate to `verify-m046-s06.sh` and keep it as a historical alias only
   - That also requires updating `compiler/meshc/tests/e2e_m045_s05.rs`

4. **Promote S06 in docs/readmes and demote S05 to a lower-level subrail.**
   - Anywhere that currently says S05 is authoritative should likely move to S06
   - S05 should remain referenced as the equal-surface subrail underneath the new closeout rail

5. **After the rail is green, finish milestone validation.**
   - `.gsd/milestones/M046/M046-VALIDATION.md` does not exist yet
   - S06 is the natural place to call `gsd_validate_milestone` once the final evidence chain exists

## Implementation Landscape

### 1. Existing authoritative rail to wrap

**`scripts/verify-m046-s05.sh`** is the current assembled equal-surface verifier.

What it already does:

- contract guards over public docs/readmes
- replays `scripts/verify-m046-s03.sh`
- replays `scripts/verify-m046-s04.sh`
- reruns scaffold unit/smoke/e2e rails
- rebuilds website docs
- copies retained S03/S04/S05 evidence under one nested bundle root
- publishes `.tmp/m046-s05/verify/latest-proof-bundle.txt`

Important paths and concepts:

- `.tmp/m046-s05/verify/latest-proof-bundle.txt`
- `.tmp/m046-s05/verify/retained-proof-bundle/`
- nested retained directories:
  - `retained-m046-s03-artifacts`
  - `retained-m046-s04-artifacts`
  - `retained-m046-s05-artifacts`

This means S06 can be built as a **composition layer over an already-composed verifier**.

### 2. Local proof surface S06 should continue to expose

**`compiler/meshc/tests/e2e_m046_s03.rs`** owns the most important live local truth assertions:

- `m046_s03_tiny_cluster_startup_dedupes_and_surfaces_runtime_truth_on_two_nodes`
- `m046_s03_tiny_cluster_failover_proves_promotion_recovery_completion_and_fenced_rejoin_from_cli_surfaces`
- package contract + package smoke rails

**`scripts/verify-m046-s03.sh`** already knows how to replay and retain these under `.tmp/m046-s03/...`.

Key artifact family:

- `tiny-cluster-failover-runtime-truth-*`

Important implementation detail from the current rail:

- the failover proof must choose a port whose deterministic startup request hashes to the primary owner; that logic is already in `e2e_m046_s03.rs` and should be reused, not reimplemented.

### 3. Packaged proof surface S06 should continue to expose

**`compiler/meshc/tests/e2e_m046_s04.rs`** owns the packaged startup/status truth plus package-contract checks:

- `m046_s04_cluster_proof_package_contract_remains_source_first_and_route_free`
- `m046_s04_cluster_proof_package_builds_to_temp_output_and_runs_repo_smoke_rail`
- `m046_s04_cluster_proof_startup_dedupes_and_surfaces_runtime_truth_on_two_nodes`

**`scripts/verify-m046-s04.sh`** already replays those and retains the packaged artifact families.

Key artifact families:

- `cluster-proof-startup-two-node-*`
- `cluster-proof-package-build-and-test-*`
- `cluster-proof-helper-preflight-*`
- `cluster-proof-helper-build-meta-*`

### 4. Shared harness already exists

**`compiler/meshc/tests/support/m046_route_free.rs`** is the shared Rust helper layer for all route-free M046 proof rails.

Natural seam already present:

- project init/archive helpers
- package temp-build helpers
- runtime spawn/stop helpers
- `meshc cluster` polling helpers
- artifact writers/readers

Do not fork this into a new S06-specific runtime harness unless an actual missing helper is discovered.

### 5. Current docs/readmes hardcode S05 as authoritative

These files currently promote **S05**, so they are the likely S06 docs touch set:

- `README.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `tiny-cluster/README.md`
- `cluster-proof/README.md`

Current pattern in those files:

- `verify-m046-s05.sh` is named as the authoritative closeout rail
- `verify-m045-s05.sh` is named as the historical wrapper

S06 likely needs to change that hierarchy to:

- `verify-m046-s06.sh` = authoritative milestone closeout rail
- `verify-m046-s05.sh` = historical / lower-level equal-surface subrail
- `verify-m045-s05.sh` = historical alias wrapper

### 6. Current Rust wrapper tests are pinned to the S05 hierarchy

These tests will need updates if the verifier hierarchy changes:

- `compiler/meshc/tests/e2e_m046_s05.rs`
- `compiler/meshc/tests/e2e_m045_s05.rs`

`e2e_m045_s05.rs` currently asserts that `verify-m045-s05.sh` delegates to `verify-m046-s05.sh` and omits broader direct work. If S06 becomes authoritative, this file must be repointed deliberately.

### 7. Milestone validation is still absent

Present state:

- `.gsd/milestones/M046/M046-VALIDATION.md` **does not exist**
- `.gsd/milestones/M045/M045-VALIDATION.md` is a good structural reference for the expected final artifact shape

If S06 is considered milestone closeout, planner should reserve a final step for milestone validation once the verifier/docs rail is green.

## Natural Seams

### Seam A — New authoritative S06 verifier

Files likely centered here:

- `scripts/verify-m046-s06.sh` (new)
- maybe `scripts/verify-m045-s05.sh` (delegate update)

This is the highest-value first step because it defines the final proof/artifact shape that docs/tests then point at.

### Seam B — Rust contract tests for the new verifier hierarchy

Files likely centered here:

- `compiler/meshc/tests/e2e_m046_s06.rs` (new)
- `compiler/meshc/tests/e2e_m045_s05.rs` (update)
- possibly `compiler/meshc/tests/e2e_m046_s05.rs` if S05 is demoted from authoritative to subrail

These should assert **script contract and bundle names**, not re-implement cluster runtime behavior.

### Seam C — Docs/readme repoint from S05 to S06

Files likely centered here:

- `README.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `tiny-cluster/README.md`
- `cluster-proof/README.md`

This work is mostly mechanical once the verifier hierarchy is settled.

### Seam D — Milestone validation / closeout artifact

Not code-first, but planner should keep it as the last step:

- `gsd_validate_milestone` for `M046`
- likely input evidence sourced from the new S06 verifier plus prior slice summaries

## Constraints and Risks

- **Do not reopen runtime/compiler work.** All evidence points to S06 being a proof/docs assembly slice.
- **Do not build a second runtime harness.** The S05 decision + knowledge notes explicitly say generated-scaffold proof stays centralized in `e2e_m046_s05.rs` + `m046_route_free.rs`.
- **Keep temp-path package builds.** `m046_route_free.rs::build_package_binary_to_output(...)` enforces pre-created external output dirs specifically to avoid churning tracked binaries.
- **Do not ban raw `/status` in `cluster-proof/README.md`.** Existing knowledge notes this would false-fail legitimate wording.
- **Do not treat historical plan text as drift.** Use explicit override markers when enduring plan files are part of proof surfaces.
- **Watch wrapper/test coupling.** `verify-m045-s05.sh` and `e2e_m045_s05.rs` are tightly coupled to the current S05 naming; changing one without the other will immediately break the wrapper rail.
- **Artifact root collisions matter.** `verify-m046-s05.sh` snapshots `.tmp/m046-s05/*` before copying new artifacts. If S06 wraps it and also reruns targeted rails, keep the S06 artifact root separate and use explicit retained-copy steps.

## Skill-Guided Notes

From **`rust-testing`**:

- prefer tests that verify **behavior/contract**, not implementation details
- use descriptive test names
- keep tests independent

Applied here: `e2e_m046_s06.rs` should assert verifier phase names, retained bundle names, and authoritative/historical script references rather than duplicating the full cluster runtime sequence already proven in S03/S04/S05.

From **`vitepress`**:

- default docs work should stay within Markdown content unless nav/config actually changes
- the core truth gate is still the VitePress build

Applied here: S06 docs work is probably limited to markdown/readme repointing; do not expand scope into `.vitepress/config.ts` or theme work unless a broken cross-link or sidebar dependency is discovered.

## Verification Strategy

Minimum slice verification set I would plan around:

1. `cargo test -p meshc --test e2e_m046_s06 m046_s06_ -- --nocapture`
2. `cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture`
3. `bash scripts/verify-m046-s06.sh`

Likely included transitively inside the new authoritative verifier:

- `bash scripts/verify-m046-s05.sh`
- `bash scripts/verify-m046-s03.sh`
- `bash scripts/verify-m046-s04.sh`
- `cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture`
- `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`
- `cargo test -p meshc --test e2e_m046_s05 m046_s05_ -- --nocapture`
- `npm --prefix website run build`

Closeout-only final step after all proof rails are green:

- run `gsd_validate_milestone` for `M046` to create `.gsd/milestones/M046/M046-VALIDATION.md`

## Planner Takeaway

This slice is **targeted**. The repo already has the real proof logic. The work is to:

- add the final S06 wrapper/verifier layer
- repoint the docs/readmes and historical alias hierarchy to that new layer
- add/update the Rust contract tests that pin the new hierarchy
- then seal milestone validation

The safest order is:

1. design the S06 verifier hierarchy and artifact layout
2. add/update Rust contract tests around that hierarchy
3. repoint docs/readmes
4. run the assembled verifier
5. only then validate the milestone
