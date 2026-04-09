---
id: M044
title: "First-Class Clustered Apps & Bounded Auto-Promotion"
status: complete
completed_at: 2026-03-30T08:45:14.648Z
key_decisions:
  - D183/D189 — clustered semantics remain opt-in and explicit through `mesh.toml` declarations for service/message/work handlers.
  - D184 — built-in runtime truth first, CLI second, HTTP optional became the operator surface contract.
  - D185/D203/D204 — automatic promotion is auto-only; manual promotion surfaces were removed and ambiguity stays fail-closed.
  - D195/D198 — declared-handler execution metadata stays compiler-owned from manifest validation through codegen/runtime registration, with explicit MIR roots preserving declared symbols.
  - D196 — declared service handlers are exposed through dedicated `__declared_service_*` wrappers instead of compiler-internal helper names.
  - D201 — live operator inspection must use a transient authenticated query channel that never joins the inspected cluster.
  - D206 — `cluster-proof` moved directly onto the public clustered-app `MESH_*` bootstrap contract with no compatibility aliases for the old env dialect.
  - S05 closeout uses one assembled verifier (`bash scripts/verify-m044-s05.sh`) that replays earlier product rails instead of a docs-only final gate.
key_files:
  - compiler/mesh-pkg/src/manifest.rs
  - compiler/mesh-pkg/src/scaffold.rs
  - compiler/meshc/src/main.rs
  - compiler/meshc/src/cluster.rs
  - compiler/mesh-codegen/src/declared.rs
  - compiler/mesh-codegen/src/mir/mono.rs
  - compiler/mesh-rt/src/dist/node.rs
  - compiler/meshc/tests/e2e_m044_s01.rs
  - compiler/meshc/tests/e2e_m044_s02.rs
  - compiler/meshc/tests/e2e_m044_s03.rs
  - compiler/meshc/tests/e2e_m044_s04.rs
  - compiler/meshc/tests/e2e_m044_s05.rs
  - cluster-proof/mesh.toml
  - cluster-proof/work_continuity.mpl
  - scripts/verify-m044-s05.sh
  - website/docs/docs/distributed/index.md
lessons_learned:
  - Manifest-declared clustered execution needs one shared execution-metadata seam from manifest validation through runtime registration; re-deriving handler identity from names drifts quickly.
  - Runtime-spawned declared work must register actor-style wrapper entrypoints with matching deserialization bodies; raw typed Mesh function symbols are not a truthful spawn ABI.
  - Read-only operator inspection cannot reuse the ordinary node-join transport, because the act of querying would falsify the cluster membership it is supposed to report.
  - Terminal milestone verifiers should replay earlier product rails and retain copied evidence bundles instead of inventing a docs-only closeout path.
---

# M044: First-Class Clustered Apps & Bounded Auto-Promotion

**M044 turned Mesh’s continuity/failover substrate into a public clustered-app model with manifest-declared clustered handlers, runtime-owned execution and operator surfaces, bounded automatic promotion, and a rewritten `cluster-proof` dogfood package.**

## What Happened

M044 completed the shift from proof-app-specific continuity plumbing to a first-class clustered-app model for ordinary Mesh applications.

S01 established the public contract: apps can opt into clustered mode in `mesh.toml`, declare clustered handlers explicitly, and consume typed `Continuity` / authority values without app-side continuity JSON parsing. S02 then carried that declaration boundary through the compiler/runtime seam by preserving declared executable symbols through MIR pruning, registering declared handlers explicitly, and moving declared work/service execution onto runtime-owned placement/submission/dispatch instead of app-owned target selection. S03 added the public operator/bootstrap surfaces on top of that substrate: a transient authenticated runtime query path that does not join the inspected cluster, `meshc cluster {status,continuity,diagnostics}` read-only inspection, and `meshc init --clustered` scaffold generation on the generic `MESH_*` contract. S04 completed the bounded failover contract by removing manual promotion from the Mesh-facing surface, proving automatic promotion/recovery on the same-image rail, and fencing stale-primary rejoin. S05 rewrote `cluster-proof` onto the public clustered-app contract, removed the old explicit clustering path, and made `bash scripts/verify-m044-s05.sh` the authoritative assembled acceptance rail by replaying the scaffold/operator, failover, package, and docs surfaces together.

The milestone also closed a few non-obvious seams that mattered to truthfulness. The local `main` branch already pointed at `HEAD`, so the naive `git diff --stat HEAD $(git merge-base HEAD main) -- ':!.gsd/'` check came back empty even though real code landed; in this repo state, `origin/main` was the truthful integration baseline and showed the non-`.gsd` compiler/runtime/proof/docs diff. S01 had to land the typed continuity ABI before the `cluster-proof` rewrite could be honest. S02 had to recover through the later registry/monomorphization/service-wrapper tasks so declared handlers survived pruning and registered cleanly. S05 then repaired stale verifier seams rather than weakening the contract: an old S04 log expectation, a login-shell docs step, and a noisy failover-bundle pointer file were all making the final wrapper falsely red after the product work itself was green.

### Decision Re-evaluation

| Decision | Verdict | Evidence from delivered work | Revisit next milestone? |
|---|---|---|---|
| D183 — only declared handlers are clustered | Still valid | S02 kept declared work/service inside the runtime-owned path and hot-path absence checks proved undeclared/local behavior stayed out of the clustered submit/status path. | No |
| D184 — runtime API first, CLI second, HTTP optional | Still valid | S03 shipped the transient runtime query transport plus `meshc cluster`, and S05 centered docs on scaffold + CLI rather than app-authored admin routes. | No |
| D185 / D203 / D204 — automatic promotion only; no manual promote surface | Still valid | S04 removed the Mesh-visible `Continuity.promote()` path and the old manual `/promote` story, while the manual-surface-disabled rails stayed green. | No |
| D186 / D189 — metadata opt-in through `mesh.toml` with explicit declaration targets | Still valid | S01 manifest parser/compiler/LSP rails and the `cluster-proof/mesh.toml` dogfood path held through final closeout. | No |
| D187 / D206 — `cluster-proof` must move fully onto the public clustered-app model with direct `MESH_*` bootstrap | Still valid | S05 removed the legacy explicit path, kept only the public clustered-app contract, and rejected legacy `CLUSTER_PROOF_*` folklore in docs/verifiers. | No |
| D190 — typed continuity payloads stay String/Int/Bool-backed structs for now | Still valid but intentionally bounded | The typed Mesh API held across S01–S05 and kept the milestone narrow; builtin enum-backed statuses remain future API work, not missing M044 scope. | Only if a later API wave adds builtin enums |
| D195 / D198 — keep shared execution metadata and explicit MIR roots for declared handlers | Still valid | S02’s declared-work/service rails depended on carrying manifest execution metadata forward and preserving declared symbols through MIR/codegen. | No |
| D196 — expose declared services through distinct wrapper symbols | Still valid | The runtime registry now sees `__declared_service_*` wrappers instead of compiler-internal `__service_*` helpers, and the S02 service rails passed on that seam. | No |
| D201 — operator inspection must use a transient non-registering query channel | Still valid | S03 proved the CLI can inspect a live node without joining the cluster or corrupting membership truth. | No |

No decision made during M044 needs immediate rollback. The only bounded revisit is D190 if/when Mesh grows builtin enum-backed public status types.

## Success Criteria Results

- [x] **Clustered opt-in and typed public surface** — Met. S01 added optional `[cluster]` metadata in `mesh.toml`, shared compiler/LSP validation, and typed `ContinuityAuthorityStatus`, `ContinuityRecord`, and `ContinuitySubmitDecision` values. Evidence: `cargo test -p mesh-pkg m044_s01_clustered_manifest_ -- --nocapture`, `cargo test -p mesh-lsp m044_s01_clustered_manifest_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s01 m044_s01_manifest_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s01 m044_s01_typed_continuity_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s01 m044_s01_continuity_compile_fail_ -- --nocapture`, and `bash scripts/verify-m044-s01.sh`.
- [x] **Runtime-owned declared-handler execution** — Met. S02 moved declared work/service execution onto runtime-owned placement/submission/dispatch while undeclared code stayed local. Evidence: `cargo test -p meshc --test e2e_m044_s02 m044_s02_metadata_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s02 m044_s02_declared_work_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s02 m044_s02_service_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s02 m044_s02_cluster_proof_ -- --nocapture`, and `bash scripts/verify-m044-s02.sh`, including retained hot-path absence checks proving the new submit/status path no longer computes placement or dispatches directly in app code.
- [x] **Built-in operator surfaces and clustered scaffold** — Met. S03 shipped runtime-owned read-only inspection and scaffold generation on the generic public contract. Evidence: `cargo test -p mesh-rt operator_query_ -- --nocapture`, `cargo test -p mesh-rt operator_diagnostics_ -- --nocapture`, `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`, `cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s03 m044_s03_scaffold_ -- --nocapture`, `bash scripts/verify-m044-s03.sh`, and `npm --prefix website run build`.
- [x] **Bounded automatic promotion** — Met. S04 proved auto-only promotion/recovery, ambiguity fail-closed behavior, and stale-primary fencing on rejoin while keeping operator inspection read-only. Evidence: `cargo test -p mesh-rt automatic_promotion_ -- --nocapture`, `cargo test -p mesh-rt automatic_recovery_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_promotion_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_resume_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s04 m044_s04_manual_surface_ -- --nocapture`, and `bash scripts/verify-m044-s04.sh` with retained failover artifacts.
- [x] **Cluster-proof rewrite and scaffold-first public story** — Met. S05 rewrote `cluster-proof` onto the public clustered-app contract, removed the legacy explicit clustering path, and made scaffold + CLI the primary docs/onboarding story. Evidence: `cargo test -p meshc --test e2e_m044_s05 m044_s05_ -- --nocapture`, `cargo run -q -p meshc -- build cluster-proof`, `cargo run -q -p meshc -- test cluster-proof/tests`, `test ! -e cluster-proof/work_legacy.mpl`, `bash scripts/verify-m044-s05.sh`, and `npm --prefix website run build`.

## Definition of Done Results

- [x] **All roadmap slices complete** — The milestone roadmap shows S01–S05 done, and every slice summary reports `verification_result: passed`.
- [x] **All slice summaries and UAT artifacts exist** — The milestone directory contains `S01` through `S05`, and each slice has both `S##-SUMMARY.md` and `S##-UAT.md` present.
- [x] **Cross-slice integration works as one assembled product** — `bash scripts/verify-m044-s05.sh` passed live during milestone closeout (`verify-m044-s05: ok`) and replayed `bash scripts/verify-m044-s03.sh`, `bash scripts/verify-m044-s04.sh`, `cargo test -p meshc --test e2e_m044_s05 m044_s05_ -- --nocapture`, `cargo run -q -p meshc -- build cluster-proof`, `cargo run -q -p meshc -- test cluster-proof/tests`, and `npm --prefix website run build`.
- [x] **Real non-`.gsd` code shipped** — The naive `main` merge-base diff was empty only because local `main` already pointed at `HEAD`; using the truthful integration baseline, `git diff --stat $(git merge-base HEAD origin/main)..HEAD -- ...` showed 96 relevant non-`.gsd` files changed across compiler/runtime/proof/docs surfaces.
- [x] **Horizontal checklist** — The roadmap does not contain an explicit Horizontal Checklist section, so there were no extra unchecked cross-cutting items beyond the verified success criteria and definition-of-done checks.

## Requirement Outcomes

| Requirement | Transition | Evidence |
|---|---|---|
| R061 | active -> validated | S01 proved the app-level clustered opt-in contract with optional `[cluster]` metadata, shared compiler/LSP validation, `cluster-proof/mesh.toml`, and the green `m044_s01_clustered_manifest_` / `m044_s01_manifest_` rails plus `bash scripts/verify-m044-s01.sh`. |
| R062 | active -> validated | S01 replaced the public stringly continuity seam with typed Mesh structs across typeck, MIR, codegen, runtime, and `cluster-proof`, proved by `m044_s01_typed_continuity_`, `m044_s01_continuity_compile_fail_`, and the S01 shim-absence checks. |
| R063 | active -> validated | S02 proved the declared boundary end to end with `m044_s02_declared_work_`, `m044_s02_service_`, `m044_s02_cluster_proof_`, and `bash scripts/verify-m044-s02.sh`, including hot-path absence checks showing undeclared/local behavior stays out of the clustered runtime path. |
| R064 | active -> validated | The milestone as a whole now proves runtime ownership of declared-handler execution: S02 moved placement/submission/dispatch to the runtime-owned declared-handler path, and S04 added runtime-owned authority/failover/recovery/fencing on top of that path through the `automatic_promotion_`, `automatic_recovery_`, `m044_s04_auto_promotion_`, `m044_s04_auto_resume_`, and assembled S04/S05 verifiers. |
| R065 | active -> validated | S03 shipped runtime-owned transient operator queries plus `meshc cluster status|continuity|diagnostics --json`, proved by `operator_query_`, `operator_diagnostics_`, `m044_s03_operator_`, and `bash scripts/verify-m044-s03.sh`; S05 kept those built-in surfaces as the primary operator story in docs and closeout. |
| R066 | active -> validated | S03 added `meshc init --clustered` with a public `MESH_*` clustered scaffold, proved by `test_init_clustered_creates_project`, `m044_s03_scaffold_`, and `bash scripts/verify-m044-s03.sh`; S05 made that scaffold the primary onboarding story. |
| R067 | active -> validated | S04 removed the manual promotion surface, kept authority mutation runtime-internal, and proved auto-only failover via `automatic_promotion_`, `m044_s04_auto_promotion_`, `m044_s04_manual_surface_`, and `bash scripts/verify-m044-s04.sh`. |
| R068 | active -> validated | S04 proved declared clustered work survives primary loss through automatic promotion/recovery and stale-primary fencing with `automatic_recovery_`, `m044_s04_auto_resume_`, retained failover artifacts, and the assembled S04 verifier; S05 replays that rail in final closeout. |
| R069 | active -> validated | S05 rewrote `cluster-proof` onto the public clustered-app contract, removed the legacy explicit clustering path, and proved it with `cargo test -p meshc --test e2e_m044_s05 -- --nocapture`, `cargo run -q -p meshc -- build cluster-proof`, `cargo run -q -p meshc -- test cluster-proof/tests`, `test ! -e cluster-proof/work_legacy.mpl`, and `bash scripts/verify-m044-s05.sh`. |
| R070 | active -> validated | S05 made scaffold-first clustered apps the public story, proved by `cargo test -p meshc --test e2e_m044_s05 -- --nocapture`, `bash scripts/verify-m044-s05.sh`, `npm --prefix website run build`, and the exact docs/source truth checks over README + distributed/tooling/proof pages. |

## Deviations

S01 had to land the typed continuity ABI before the `cluster-proof` rewrite could be honest. S02 closed through later recovery tasks after registry, MIR-root preservation, and declared-service wrapper seams surfaced. S05 also had to repair stale verifier expectations (an old S04 log marker, a login-shell docs step, and a noisy failover-bundle pointer file) so the final assembled acceptance rail matched already-green product truth.

## Follow-ups

Future milestones should treat broader operator controls, multi-standby or active-active failover, consensus-backed authority, exactly-once execution claims, and enum-backed continuity status types as new explicit requirements rather than implicit extensions of the bounded M044 contract. Use `bash scripts/verify-m044-s05.sh` as the terminal local acceptance rail for any future work that modifies the clustered-app/public-failover story.
