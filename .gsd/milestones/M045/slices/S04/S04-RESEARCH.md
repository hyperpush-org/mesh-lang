# S04 Research — Remove Legacy Example-Side Cluster Logic

## Summary

- S04 primarily advances **R077, R079, and R080**, while preserving the already-proven tiny-example story behind **R078** and the active clustered continuity/operator requirements **R049 / R050 / R052**.
- The scaffolded clustered example is already the target public shape:
  - `compiler/mesh-pkg/src/scaffold.rs` generates only `Node.start_from_env()`, `/health`, `Continuity.submit_declared_work(...)`, peer-aware replica selection via `Node.list()`, and a fixed `Timer.sleep(250)` work handler.
  - `compiler/meshc/tests/tooling_e2e.rs`, `compiler/meshc/tests/e2e_m045_s02.rs`, and `compiler/meshc/tests/e2e_m045_s03.rs` lock that shape and verify runtime-owned CLI truth.
- The remaining example-owned clustered residue is concentrated in `cluster-proof/`, not the scaffold:
  - `cluster-proof/cluster.mpl` still contains the old deterministic placement/canonical-membership engine; only the membership payload path is still live from `main.mpl`.
  - `cluster-proof/work_continuity.mpl` still owns app-local completion (`Continuity.mark_completed(...)`), the optional `CLUSTER_PROOF_WORK_DELAY_MS` proof knob, `submit_required_replica_count(...)`, a dead `actor execute_work(...)`, and a large amount of JSON/status translation.
  - `cluster-proof/config.mpl` and `cluster-proof/docker-entrypoint.sh` still duplicate continuity topology validation. Some duplication is intentional from S01: runtime bootstrap still does **not** validate continuity role/epoch.
- Public docs already tell a scaffold-first story, but the deeper proof surfaces still anchor on the M044 closeout rail (`scripts/verify-m044-s05.sh`, `cluster-proof/README.md`, `website/docs/docs/distributed-proof/index.md`). That looks like **S05 territory** unless S04 removes a seam those surfaces still name directly.
- `meshc cluster status|continuity|diagnostics` in `compiler/meshc/src/cluster.rs` already exposes the authoritative runtime-owned truth that the M045 scaffold e2e rails consume directly. S04 does not need a new CLI/operator surface.

## Skills Discovered

- Loaded installed skill: `rust-best-practices`.
- Ran `npx skills find "distributed runtime"`.
  - Best result was a generic distributed-systems skill, but nothing was Mesh/Rust-runtime-specific enough to justify installation for this slice.
- Practical takeaway from the loaded Rust skill for this slice:
  - prefer smaller focused Rust tests/verifiers over broad multi-assertion contract blobs
  - remove dead helper layers instead of preserving them behind comment-heavy compatibility shims
  - keep error handling explicit and fail-closed when package/verifier contracts narrow

## Recommendation

Treat S04 as a **targeted deletion/collapse slice inside `cluster-proof`**, not as a new runtime feature wave.

Recommended order:

1. **Delete dead cluster-proof legacy code first**
   - safest target is the unreferenced placement/canonical-placement machinery in `cluster-proof/cluster.mpl`
   - update package tests in the same unit so dead-helper references do not become false failures
2. **Then collapse live but legacy-shaped execution glue**
   - focus on `cluster-proof/work_continuity.mpl`, especially manual completion and wrapper-era leftovers
   - only remove app-local completion if the runtime/codegen declared-work completion path still keeps the package and M045 scaffold rails green
3. **Keep continuity topology validation honest**
   - do **not** delete `cluster-proof/docker-entrypoint.sh` / `cluster-proof/config.mpl` continuity checks unless equivalent fail-closed validation moves lower first
4. **Touch docs/verifiers only where removed seams are explicitly named**
   - leave the full docs-first promotion and public closeout story for S05

Do **not** treat the scaffold’s `Node.list()` replica-count helper or the fixed `Timer.sleep(250)` as obvious cleanup. S03 explicitly uses both as failover-proof stabilizers.

## Implementation Landscape

### 1. Tiny scaffold contract — already the target public shape

- `compiler/mesh-pkg/src/scaffold.rs`
  - Clustered `main.mpl` uses:
    - `Node.start_from_env()`
    - `/health`
    - `Continuity.submit_declared_work("Work.execute_declared_work", ...)`
    - `Node.list()` to choose `required_replica_count`
  - Generated `work.mpl` only sleeps briefly and returns an `Int`.
  - Generated README already frames `meshc cluster status|continuity|diagnostics` as the operator truth surface.
- `compiler/meshc/tests/tooling_e2e.rs`
  - Locks the generated source against legacy seams: no `declared_work_target`, no `Continuity.mark_completed`, no app-owned status route, no `CLUSTER_PROOF_*`, no direct `Node.start(...)`.
- `compiler/meshc/tests/e2e_m045_s02.rs`
  - Verifies remote-owner execution and runtime-owned completion on the scaffold.
- `compiler/meshc/tests/e2e_m045_s03.rs`
  - Verifies failover on the same scaffold and relies on the current peer-aware replica count plus fixed 250ms delay.

### 2. `cluster-proof` — where the remaining legacy lives

- `cluster-proof/main.mpl`
  - Already delegates bootstrap to `Node.start_from_env()`.
  - Still serves `/membership`, `POST /work`, and `GET /work/:request_key`.
  - Depends on `Cluster.membership_payload(...)`, `Config.*`, and `WorkContinuity.*`.
- `cluster-proof/cluster.mpl`
  - **Live**: membership payload shaping (`membership_snapshot`, `membership_payload_json_from_membership`, `membership_payload`).
  - **Legacy/dead**: deterministic placement pipeline:
    - `CanonicalPlacement`
    - `placement_score`, `placement_tie_breaker`
    - `best_placement_index`
    - `build_canonical_placement_from_membership`
    - `canonical_placement`
  - Those placement helpers are not referenced outside this file.
- `cluster-proof/work_continuity.mpl`
  - Owns the keyed submit/status HTTP contract.
  - Still contains legacy-shaped execution glue:
    - `continuity_mark_completed(...)` + `complete_work_execution(...)`
    - dead `actor execute_work(...)`
    - `declared_work_target()` literal helper
    - `work_execution_delay_ms()` / `maybe_delay_work_execution()` using `CLUSTER_PROOF_WORK_DELAY_MS`
    - `submit_required_replica_count(...)`
  - This is the biggest remaining example-owned clustering seam.
- `cluster-proof/config.mpl`
  - Supplies `current_mode()`, durability-policy helpers, continuity topology validation, and `required_replica_count(...)`.
  - Current package tests overfit these internal helper functions.
- `cluster-proof/docker-entrypoint.sh`
  - Mirrors continuity role/epoch validation in shell.
  - S01 explicitly kept this because runtime bootstrap still does not validate continuity topology. Deleting it without replacement would widen the honesty gap.

### 3. Package tests — refactor friction points

- `cluster-proof/tests/config.test.mpl`
  - Tests many helper-level functions directly:
    - `continuity_cluster_role_from_parts`
    - `continuity_promotion_epoch_from_parts`
    - `continuity_replication_health_from_parts`
    - `continuity_topology_error_from_parts`
    - `required_replica_count`
  - Good candidate to rewrite around smaller public seams if helpers disappear.
- `cluster-proof/tests/work.test.mpl`
  - Validates membership JSON shaping, authority/status payload helpers, response helpers, and `required_replica_count(...)`.
  - Needs synchronized cleanup if S04 removes dead placement/config helpers or manual-completion-era response branches.

### 4. Public/runtime truth surfaces already exist

- `compiler/meshc/src/cluster.rs`
  - `meshc cluster status`
  - `meshc cluster continuity`
  - `meshc cluster diagnostics`
- These already return the runtime-owned truth that the M045 scaffold e2e rails consume.
- S04 should prefer these surfaces over adding any new app-owned operator translation.

### 5. Docs/verifiers still anchored to M044 closeout

Current public/closeout surfaces still point at the M044 story:

- `README.md`
- `cluster-proof/README.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/tooling/index.md`
- `scripts/verify-m044-s05.sh`
- `compiler/meshc/tests/e2e_m044_s05.rs`

These files currently enforce the M044 public story, including explicit references to `scripts/verify-m044-s05.sh` and some current `cluster-proof` details like `CLUSTER_PROOF_WORK_DELAY_MS`.

Unless S04 intentionally wants to narrow the public runbook now, these are better treated as **S05 follow-up surfaces**. Do targeted edits only if a removed code seam is still documented as live.

## Natural Task Seams

1. **Dead-code collapse in `cluster-proof/cluster.mpl`**
   - Remove unreferenced placement/canonicalization machinery.
   - Keep `/membership` truthful and stable.
   - Update `cluster-proof/tests/work.test.mpl` if helper names disappear.
2. **Declared-work execution cleanup in `cluster-proof/work_continuity.mpl`**
   - Verify whether manual `Continuity.mark_completed(...)` can be dropped in favor of the runtime/codegen completion path already used by the scaffold.
   - Remove dead `execute_work` actor if still unreferenced.
   - Keep keyed submit/status HTTP contract and error surfaces stable unless the slice explicitly narrows them.
3. **Config/entrypoint collapse**
   - Only safe if continuity role/epoch validation moves lower first.
   - If not, leave this seam alone and document it as an honest remaining limit.
4. **Verifier/test tightening**
   - Add M045-specific cleanup rails instead of weakening the scaffold/failover rails.
   - Prefer a new M045 S04 source/package cleanup contract over mutating S02/S03 rails beyond recognition.

## Risks / Constraints

- **Do not remove S03 proof stabilizers casually.**
  - The scaffold’s peer-aware replica selection (`Node.list()`) and fixed `Timer.sleep(250)` were deliberate S03 decisions.
- **Do not delete continuity topology preflight without a lower replacement.**
  - `cluster-proof/docker-entrypoint.sh` is still covering validation that `Node.start_from_env()` does not own.
- **Manual completion removal is promising but not free.**
  - If `cluster-proof/work_continuity.mpl` stops calling `Continuity.mark_completed(...)`, verify that the runtime/codegen wrapper path actually closes the declared-work record on the `cluster-proof` manifest target, not just the scaffold target.
- **Old M044 public-contract tests will resist cleanup.**
  - `compiler/meshc/tests/e2e_m044_s05.rs` and `scripts/verify-m044-s05.sh` still encode the older public closeout story. S04 should avoid breaking them accidentally unless it is prepared to replace that contract with M045-specific rails.
- **Package tests currently overfit internal helpers.**
  - Refactoring `cluster-proof/config.mpl` without rewriting `cluster-proof/tests/config.test.mpl` will create noisy failures that do not reflect user-visible regressions.

## Verification

Use the smallest truthful stack for S04.

### Always-on package/runtime checks

- `cargo run -q -p meshc -- build cluster-proof`
- `cargo run -q -p meshc -- test cluster-proof/tests`

### Keep the tiny scaffold rails green

- `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`
- `cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture`
- `cargo test -p meshc --test e2e_m045_s03 m045_s03_failover_ -- --nocapture`

### If S04 adds its own cleanup contract

Prefer a new M045 rail instead of overloading old closeout tests:

- `cargo test -p meshc --test e2e_m045_s04 ... -- --nocapture`
- `bash scripts/verify-m045-s04.sh`

That verifier should replay only the required M045 scaffold/package prerequisites and retain any bundle it depends on, following the existing M045 verifier pattern instead of nesting broad old wrappers.

### Docs only if touched

- `npm --prefix website run build`

## Resume Notes for Planner

- The biggest low-risk win is deleting the unused placement engine from `cluster-proof/cluster.mpl`.
- The biggest high-value but riskier win is removing manual completion / dead execution glue from `cluster-proof/work_continuity.mpl` and proving the runtime-owned declared-work wrapper path is sufficient for `cluster-proof` too.
- The continuity-topology validation split between `config.mpl` and `docker-entrypoint.sh` is **not** obviously removable yet; treat it as a separate decision point, not as cleanup collateral.
- Avoid spending S05’s docs-first rewrite budget inside S04 unless a removed seam is still named publicly and would otherwise leave docs lying.
