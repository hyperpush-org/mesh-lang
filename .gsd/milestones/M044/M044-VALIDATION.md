---
verdict: pass
remediation_round: 0
---

# Milestone Validation: M044

## Success Criteria Checklist
Validation note: the current rendered `M044-ROADMAP.md` only contains the vision and slice table, so milestone-level success criteria were reconciled against `M044-CONTEXT.md`'s **Final Integrated Acceptance** plus each slice's roadmap “After this” claim.

- [x] **Clustered activation and the declaration boundary are app-level, explicit, and typed instead of proof-app JSON glue.**
  - Evidence: S01 delivered optional `[cluster]` metadata in `mesh.toml`, shared compiler/LSP validation, typed `ContinuityAuthorityStatus` / `ContinuityRecord` / `ContinuitySubmitDecision`, and a typed `cluster-proof` consumer with stale continuity JSON shims removed from `cluster-proof/work_continuity.mpl`.
  - Proof: `bash scripts/verify-m044-s01.sh`; `cargo test -p mesh-pkg m044_s01_clustered_manifest_ -- --nocapture`; `cargo test -p mesh-lsp m044_s01_clustered_manifest_ -- --nocapture`; `cargo test -p meshc --test e2e_m044_s01 m044_s01_manifest_ -- --nocapture`; `cargo test -p meshc --test e2e_m044_s01 m044_s01_typed_continuity_ -- --nocapture`; `cargo test -p meshc --test e2e_m044_s01 m044_s01_continuity_compile_fail_ -- --nocapture`.

- [x] **The same binary runs on two nodes and runtime-owned declared handlers — not app-authored placement code — own clustered execution while undeclared code stays local.**
  - Evidence: S02 delivered shared clustered execution metadata, preserved declared handlers as explicit MIR roots, registered declared work/service wrapper symbols, and moved `cluster-proof` onto `Continuity.submit_declared_work(...)` with hot-path absence checks for legacy app-owned placement/dispatch.
  - Proof: `bash scripts/verify-m044-s02.sh`; `cargo test -p meshc --test e2e_m044_s02 m044_s02_metadata_ -- --nocapture`; `cargo test -p meshc --test e2e_m044_s02 m044_s02_declared_work_ -- --nocapture`; `cargo test -p meshc --test e2e_m044_s02 m044_s02_service_ -- --nocapture`; `cargo test -p meshc --test e2e_m044_s02 m044_s02_cluster_proof_ -- --nocapture`.

- [x] **A freshly scaffolded clustered app can use built-in operator surfaces and the generic `MESH_*` contract instead of proof-app wiring.**
  - Evidence: S03 delivered transient runtime-owned operator queries, public `meshc cluster status|continuity|diagnostics`, and `meshc init --clustered`; the generated scaffold uses the public `MESH_*` contract and is inspectable through the same CLI.
  - Proof: `cargo test -p mesh-rt operator_query_ -- --nocapture`; `cargo test -p mesh-rt operator_diagnostics_ -- --nocapture`; `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`; `cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture`; `cargo test -p meshc --test e2e_m044_s03 m044_s03_scaffold_ -- --nocapture`; `bash scripts/verify-m044-s03.sh`.

- [x] **Killing the active primary triggers bounded automatic promotion only when safe, recovers declared clustered work on the standby, and keeps stale-primary rejoin fenced.**
  - Evidence: S04 removed the Mesh-visible manual promotion surface, delivered bounded automatic promotion/recovery, and retained concrete same-image failover artifacts showing standby promotion, automatic recovery rollover, completion on the standby, and fenced stale-primary rejoin.
  - Proof: `cargo test -p mesh-rt automatic_promotion_ -- --nocapture`; `cargo test -p mesh-rt automatic_recovery_ -- --nocapture`; `cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_promotion_ -- --nocapture`; `cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_resume_ -- --nocapture`; `cargo test -p meshc --test e2e_m044_s04 m044_s04_manual_surface_ -- --nocapture`; `bash scripts/verify-m044-s04.sh`; retained bundle `.tmp/m044-s04/continuity-api-failover-promotion-rejoin-1774859287729589000`.

- [x] **`cluster-proof` is rewritten onto the public clustered-app standard and the public docs/verifiers now teach the scaffold-first clustered-app story.**
  - Evidence: S05 moved `cluster-proof` onto the public `MESH_*` bootstrap contract, deleted `cluster-proof/work_legacy.mpl`, kept only the keyed runtime-owned submit/status path, and made `bash scripts/verify-m044-s05.sh` the authoritative closeout rail that replays S03 + S04 before package/docs truth checks.
  - Proof: `cargo test -p meshc --test e2e_m044_s05 -- --nocapture`; `cargo run -q -p meshc -- build cluster-proof`; `cargo run -q -p meshc -- test cluster-proof/tests`; `test ! -e cluster-proof/work_legacy.mpl`; `bash scripts/verify-m044-s05.sh`; `npm --prefix website run build`.
  - Live replay during validation: `bash scripts/verify-m044-s05.sh` completed successfully with `.tmp/m044-s05/verify/status.txt = ok`, `.tmp/m044-s05/verify/current-phase.txt = complete`, and all phases in `.tmp/m044-s05/verify/phase-report.txt` passed.


## Slice Delivery Audit
| Slice | Planned deliverable | Delivered evidence | Verdict |
|---|---|---|---|
| S01 | App-level clustered opt-in in `mesh.toml`, explicit declared-handler boundary, typed public continuity/authority values, no app-side continuity JSON parsing. | S01 summary/UAT show optional `[cluster]` metadata, shared compiler/LSP validation, typed continuity structs, typed `cluster-proof` consumer, and shim-absence checks enforced by `bash scripts/verify-m044-s01.sh`. | pass |
| S02 | Runtime-owned declared-handler execution on the same binary across nodes, with undeclared code remaining local. | S02 summary/UAT show compiler-owned execution metadata, declared work/service wrapper registration, runtime-owned `submit_declared_work(...)`, and hot-path absence checks proving the new declared-runtime route no longer computes placement or dispatch in app code. | pass |
| S03 | Built-in operator surfaces plus `meshc init --clustered` scaffold using the public clustered-app contract. | S03 summary/UAT show transient runtime operator transport, `meshc cluster` commands, generated scaffold files on the generic `MESH_*` contract, and live scaffold/operator proofs. | pass |
| S04 | Safe bounded automatic promotion, fail-closed ambiguity handling, and stale-primary fencing on rejoin. | S04 summary/UAT show automatic-only failover, removal of `Continuity.promote()` from the Mesh surface, destructive same-image failover proof, and retained standby/primary artifact bundles. | pass |
| S05 | `cluster-proof` fully rewritten onto the public clustered-app standard, legacy explicit clustering path removed, docs/verifiers teach scaffold-first clustered apps. | S05 summary/UAT show public `MESH_*` bootstrap, deletion of `work_legacy.mpl`, final closeout verifier replaying S03/S04, website/docs truth checks, and live validation replay ending green. | pass |

**Audit result:** every roadmap slice substantiates its planned “After this” claim; no slice summary overclaims beyond what its verifier/UAT actually proved.

## Cross-Slice Integration
- **S01 → S02: aligned.** S02 explicitly consumes S01’s shared declaration metadata and typed continuity surface. The S02 implementation and verifier prove the declaration boundary survived parser/compiler/LSP validation and became runtime registration/execution metadata rather than a second ad hoc parser.
- **S02 → S03: aligned.** S03 builds the operator and scaffold story on top of S02’s runtime-owned declared-handler substrate. No app-authored operator wiring is reintroduced; `meshc cluster` reads runtime truth and the scaffold sits on the same declared-handler contract.
- **S02 + S03 → S04: aligned.** S04 layers bounded automatic promotion over the already-runtime-owned declared-work path and keeps operator interaction read-only. The failover proof depends on the S02 execution seam and the S03 public operator framing without widening either contract.
- **S01 + S02 + S03 + S04 → S05: aligned.** S05 replays S03 and S04 inside `bash scripts/verify-m044-s05.sh`, rewrites `cluster-proof` onto the public `MESH_*` contract, removes the legacy work path, and makes the docs/verifier surface depend on the already-proven product rails.
- **Boundary consistency:** later slices preserved the milestone’s scoped non-goals instead of silently widening the claim. The summaries stay consistent about declared-only clustering, read-only operator surfaces, auto-only promotion, one-primary/one-standby topology, no active-active writes, and no exactly-once claim.

**Integration verdict:** no cross-slice boundary mismatches found.

## Requirement Coverage
All active M044 requirements are covered by at least one delivered slice; no active milestone requirement is orphaned.

| Requirement | Covered by | Evidence / coverage note |
|---|---|---|
| R049 — keyed work completes with at-least-once, idempotent semantics without an external durable store | S02, S04 | Runtime-owned declared work landed in S02; safe automatic recovery/rollover on standby landed in S04 via `automatic_recovery_`, `m044_s04_auto_resume_`, and the retained failover bundle. |
| R050 — in-flight continuity is replicated across live nodes with two-node safety as the proof bar | S02, S04 | S02 made continuity part of the declared-handler runtime path; S04 proved mirrored standby promotion/recovery and fenced rejoin on the two-node same-image rail. |
| R052 — one Docker image with a small env-driven operator surface locally and on Fly | S03, S05 | S03 established the generic `MESH_*` clustered-app contract; S05 moved `cluster-proof` onto that public contract and proved Fly-derived identity fallback / old-env rejection. M044’s shipped Fly surface remains read-only inspection, which matches the slice summaries and public docs. |
| R061 — clustered mode is an app-level `mesh.toml` opt-in | S01 | Validated in S01 via manifest parser/LSP/meshc rails and the assembled S01 verifier. |
| R062 — declared clustered handlers compile against typed public continuity/authority surfaces with no app-side continuity JSON parsing | S01, S02, S05 | Typed public surfaces landed in S01, moved into the real declared-runtime path in S02, and remain the only public path in the rewritten `cluster-proof` delivered by S05. |
| R063 — only declared clustered handlers get continuity/failover semantics; undeclared code stays local | S01, S02 | S01 established the explicit declaration boundary; S02 validated that only manifest-declared work/service handlers enter the clustered runtime path. |
| R064 — runtime owns placement, continuity replication, fencing, authority, and failover for declared clustered handlers | S02, S04 | S02 delivered runtime-owned placement/submission/dispatch; S04 completed authority/failover ownership and removed the stale Mesh-visible manual promotion seam. |
| R065 — built-in operator surfaces with runtime API first, CLI second, HTTP optional | S03, S05 | S03 delivered runtime operator query transport plus `meshc cluster`; S05 made those surfaces the primary public/operator story. |
| R066 — `meshc init --clustered` scaffolds a real clustered app using only public surfaces | S03, S05 | S03 delivered and verified the scaffold; S05 elevated it to the primary public story and replayed it in final closeout. |
| R067 — automatic promotion is auto-only, bounded, epoch/fencing-based, and fail-closed on ambiguity | S04 | Validated by runtime rails, destructive `e2e_m044_s04`, removal of `Continuity.promote()`, and the assembled S04 verifier. |
| R068 — declared clustered handler work survives primary loss through bounded automatic promotion | S04, S05 | S04 proved recovery on the promoted standby with fenced rejoin; S05 replays that rail in the final closeout contract. |
| R069 — `cluster-proof` is fully rewritten onto the new clustered-app standard | S05 | Validated by the S05 public-contract + legacy-cleanup rails, package build/tests, absence of `work_legacy.mpl`, and the final closeout verifier. |
| R070 — public docs and proof surfaces teach clustered apps as the primary story | S05 | Validated by the S05 docs/source truth rails, website build, and the scaffold-first docs/runbook rewrite. |

Requirement status transitions themselves should be recorded during milestone completion, but from a validation perspective the milestone’s active requirement set is fully covered by delivered slices and proof surfaces.

## Verdict Rationale
**Verdict: pass.** The milestone-level acceptance contract is satisfied, every roadmap slice substantiates its promised deliverable, and the live terminal acceptance rail still passes after the full body of work landed.

Why this is a pass:
- **Contract verification:** fully addressed. S01/S02 prove the clustered declaration boundary, typed public continuity surface, and runtime-owned declared-handler execution through targeted compiler/LSP/runtime rails and source-boundary absence checks. S04 adds compile-fail/manual-surface-disabled proof and bounded failover/runtime transition rails. S05 adds source/docs truth checks so the public contract cannot silently drift.
- **Integration verification:** fully addressed. S02 proves the same binary can execute declared clustered handlers across nodes; S03 proves the scaffold/operator story on the public `MESH_*` contract; S05 replays S03 and S04 together before package/docs closeout. The slices’ provides/requires chains line up cleanly and no later slice reintroduced proof-app-only seams.
- **Operational verification:** fully addressed within M044’s scoped topology. S04 provides the destructive same-image failover rail, retained failover artifacts, ambiguity rejection, and stale-primary fencing. S05 replays that rail in the final closeout verifier. There is no missing operational proof class for the shipped one-primary/one-standby contract.
- **UAT verification:** addressed. Every slice includes a concrete UAT artifact, and the milestone’s terminal acceptance command was rerun during this validation: `bash scripts/verify-m044-s05.sh` finished green, with `.tmp/m044-s05/verify/status.txt=ok`, `.tmp/m044-s05/verify/current-phase.txt=complete`, and every phase in `phase-report.txt` passed.

Remaining limitations are explicit non-goals, not validation failures: no manual promotion path, no active-active writes, no broader failover topology than one primary plus one standby, no exactly-once claim, and Fly remains a read-only inspection surface rather than a destructive failover rail. Those boundaries are documented consistently across the slice summaries, UATs, and final public docs.

Validation note: because the current rendered `M044-ROADMAP.md` contains only the vision and slice table, this validation keyed the milestone-level checklist to `M044-CONTEXT.md`’s final integrated acceptance plus the slice-level “After this” commitments. That is an artifact-rendering limitation, not a delivery gap.
