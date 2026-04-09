---
id: S05
parent: M046
milestone: M046
provides:
  - A route-free clustered scaffold contract that matches `tiny-cluster/` and `cluster-proof/` on source shape, runtime name, startup flow, and CLI-only inspection.
  - An authoritative generated-scaffold runtime rail (`compiler/meshc/tests/e2e_m046_s05.rs`) plus direct closeout verifier (`scripts/verify-m046-s05.sh`) for equal-surface proof.
  - Public docs and package READMEs that now teach one canonical route-free clustered-work/operator story instead of split scaffold-vs-proof narratives.
  - A retained S03/S04/S05 proof-bundle chain rooted at `.tmp/m046-s05/verify/latest-proof-bundle.txt` for downstream diagnosis and S06 closeout reuse.
requires:
  - slice: S03
    provides: The local-first route-free `tiny-cluster/` proof, the runtime-owned `startup_dispatch_window` observability seam, and the retained S03 verifier bundle that S05 republishes under the assembled proof chain.
  - slice: S04
    provides: The packaged route-free `cluster-proof/` contract, the shared `m046_route_free` harness, and the retained S04 verifier bundle that S05 reuses and nests under the direct closeout rail.
affects:
  - S06
key_files:
  - compiler/mesh-pkg/src/scaffold.rs
  - compiler/meshc/tests/tooling_e2e.rs
  - compiler/meshc/tests/support/m046_route_free.rs
  - compiler/meshc/tests/e2e_m046_s05.rs
  - compiler/meshc/tests/e2e_m044_s03.rs
  - compiler/meshc/tests/e2e_m045_s01.rs
  - compiler/meshc/tests/e2e_m045_s02.rs
  - compiler/meshc/tests/e2e_m045_s03.rs
  - website/docs/docs/getting-started/clustered-example/index.md
  - website/docs/docs/tooling/index.md
  - website/docs/docs/distributed-proof/index.md
  - website/docs/docs/distributed/index.md
  - README.md
  - tiny-cluster/README.md
  - cluster-proof/README.md
  - scripts/verify-m046-s05.sh
  - scripts/verify-m045-s05.sh
  - compiler/meshc/tests/e2e_m045_s05.rs
  - compiler/meshc/tests/e2e_m046_s05.rs
  - .gsd/PROJECT.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - D252: Keep scaffold, proof packages, docs, and the historical closeout wrapper in lockstep through one authoritative equal-surface verifier and one canonical status → continuity list → continuity record → diagnostics operator flow.
  - D253: Make the clustered scaffold source-owned and route-free: package-only `mesh.toml`, `Node.start_from_env()` bootstrap, one `clustered(work)` declaration, stable runtime name `Work.execute_declared_work`, and CLI-only inspection surfaces.
  - D254: Centralize live generated-scaffold runtime proof in `compiler/meshc/tests/e2e_m046_s05.rs` plus `compiler/meshc/tests/support/m046_route_free.rs`, while shrinking older scaffold rails to contract/delegation guards.
  - D255: Make `scripts/verify-m046-s05.sh` the sole direct closeout seam and keep `scripts/verify-m045-s05.sh` as a thin retained-alias wrapper.
  - D258: Treat R090 as validated by the passing equal-surface scaffold/package/verifier chain; treat R092 as validated by the route-free public docs/readme/verifier surfaces recorded in D256/D257.
patterns_established:
  - Keep one authoritative live equal-surface rail (`e2e_m046_s05` + `scripts/verify-m046-s05.sh`) and shrink historical scaffold/package wrappers to contract/delegation guards instead of replaying deleted routeful behavior.
  - Share generated-scaffold setup, temp builds, node launch, and CLI JSON polling through `compiler/meshc/tests/support/m046_route_free.rs` instead of forking a scaffold-specific runtime harness.
  - Lock every public/operator surface to the same runtime-owned runbook: `meshc cluster status`, continuity list, continuity record, then diagnostics.
  - Retain copied S03/S04/S05 artifacts behind one `latest-proof-bundle.txt` pointer so future slices can debug the assembled proof chain without reconstructing earlier replay directories.
observability_surfaces:
  - `meshc cluster status --json` on the generated scaffold, `tiny-cluster`, and `cluster-proof` as the equal-surface health/membership/authority signal.
  - `meshc cluster continuity --json` list mode as the canonical discovery step for startup records and request keys across all three clustered-example surfaces.
  - `meshc cluster continuity <node> <request-key> --json` as the canonical single-record truth surface for completed startup work and failover state.
  - `meshc cluster diagnostics --json` as the shared runtime-owned troubleshooting surface for generated scaffolds and proof packages.
  - Retained S05 verifier artifacts under `.tmp/m046-s05/verify/` including `status.txt`, `current-phase.txt`, `phase-report.txt`, `full-contract.log`, and `latest-proof-bundle.txt`.
  - The nested retained proof bundle at `.tmp/m046-s05/verify/retained-proof-bundle/`, which copies `retained-m046-s03-artifacts`, `retained-m046-s04-artifacts`, and `retained-m046-s05-artifacts` behind one pointer for downstream replay.
drill_down_paths:
  - .gsd/milestones/M046/slices/S05/tasks/T01-SUMMARY.md
  - .gsd/milestones/M046/slices/S05/tasks/T02-SUMMARY.md
  - .gsd/milestones/M046/slices/S05/tasks/T03-SUMMARY.md
  - .gsd/milestones/M046/slices/S05/tasks/T04-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-01T02:07:01.454Z
blocker_discovered: false
---

# S05: Equal-surface scaffold alignment

**Aligned the clustered scaffold, `tiny-cluster/`, `cluster-proof/`, docs, and verifier rails onto one route-free equal-surface clustered-work contract with fail-closed drift checks.**

## What Happened

S05 closed the parity gap between the three clustered-example surfaces instead of adding new runtime behavior. The clustered scaffold generated by `meshc init --clustered` now tells the same story as `tiny-cluster/` and rebuilt `cluster-proof/`: package-only `mesh.toml`, source-owned `clustered(work)`, one `Node.start_from_env()` bootstrap path, trivial visible `1 + 1` work, and runtime/tooling-owned inspection through `meshc cluster status|continuity|diagnostics`. The slice removed the last routeful scaffold residues, rewrote the generated README around the same route-free operator flow, and made the fast scaffold unit plus CLI smoke tests fail immediately if `[cluster]`, `HTTP.serve(...)`, `/health`, `/work`, `Continuity.submit_declared_work(...)`, or proof-only timing seams come back.

S05 also established one authoritative runtime proof seam for the generated scaffold. `compiler/meshc/tests/support/m046_route_free.rs` now owns the shared generated-scaffold init/archive helpers, and `compiler/meshc/tests/e2e_m046_s05.rs` proves that the generated scaffold matches the route-free package contract on disk, boots on two nodes, exposes startup truth only through `meshc cluster status`, continuity list, continuity record lookup, and diagnostics, and retains generated-project/build/log/JSON evidence under `.tmp/m046-s05/...`. Older scaffold-era M044/M045 rails no longer revive deleted HTTP flows; they now act as fail-closed source/readme/delegation guards around the shared S05 proof seam.

The public story now matches the runtime story. The clustered getting-started page, tooling docs, distributed-proof page, distributed guide intro, repo README, `tiny-cluster/README.md`, and `cluster-proof/README.md` all present the same equal canonical contract and the same operator sequence: status, continuity list, continuity record, then diagnostics. `scripts/verify-m046-s05.sh` is now the authoritative closeout rail for that story, while `scripts/verify-m045-s05.sh` is retained only as the historical alias name. The direct S05 verifier replays retained S03 and S04 evidence, reruns scaffold/docs parity checks, and assembles one retained proof-bundle root so downstream slices can diagnose drift from a single pointer instead of reopening three independent proof surfaces.

## Verification

Ran every slice-plan verification command and the full chain passed end to end.

- `cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture` — passed.
- `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture` — passed.
- `cargo test -p meshc --test e2e_m044_s03 m044_s03_scaffold_ -- --nocapture` — passed.
- `cargo test -p meshc --test e2e_m045_s01 m045_s01_ -- --nocapture` — passed.
- `cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture` — passed.
- `cargo test -p meshc --test e2e_m045_s03 m045_s03_ -- --nocapture` — passed.
- `cargo test -p meshc --test e2e_m046_s05 m046_s05_ -- --nocapture` — passed.
- `npm --prefix website run build` — passed.
- `bash scripts/verify-m046-s05.sh` — passed.
- `bash scripts/verify-m045-s05.sh` — passed.

I also reran the full chained replay in one background job and it completed green in about 959 seconds. The resulting direct verifier artifacts show `.tmp/m046-s05/verify/status.txt=ok`, `.tmp/m046-s05/verify/current-phase.txt=complete`, a fully passed `.tmp/m046-s05/verify/phase-report.txt`, and `.tmp/m046-s05/verify/latest-proof-bundle.txt` pointing at `.tmp/m046-s05/verify/retained-proof-bundle`.

## Operational Readiness

- **Health signal:** `meshc cluster status --json` shows membership/authority truth on the generated scaffold and proof packages; `meshc cluster continuity --json` list plus single-record lookup show startup/completion truth; the direct verifier exposes `.tmp/m046-s05/verify/status.txt=ok` and a complete passed phase report.
- **Failure signal:** Any reintroduced `[cluster]`, `/health`, `/work`, explicit continuity submission calls, diverged README/docs operator flow, missing retained bundle pointer, or failed delegated S03/S04 replay immediately fails either the fast contract rails or the direct S05 verifier.
- **Recovery procedure:** Rerun `bash scripts/verify-m046-s05.sh`, inspect `.tmp/m046-s05/verify/phase-report.txt` and `full-contract.log`, then follow `.tmp/m046-s05/verify/latest-proof-bundle.txt` into the nested retained `retained-m046-s03-artifacts`, `retained-m046-s04-artifacts`, and `retained-m046-s05-artifacts` directories to localize whether drift is in scaffold generation, runtime proof, docs/readmes, or retained-bundle assembly.
- **Monitoring gaps:** S05 proves parity and retained-bundle diagnosis, but S06 still owns the final milestone-level assembled replay that restates the local and packaged route-free startup/failover/status story as one end-to-end closeout surface.

## Requirements Advanced

- R085 — S05 rewrote the scaffold and public docs to lead with source-owned `clustered(work)` while keeping manifest support package-only, which advances the requirement that the language-facing declaration surface remains the canonical story across examples.
- R086 — By deleting the last routeful scaffold control plane and proving the generated scaffold through runtime-owned status/continuity/diagnostics only, S05 extends the runtime-owned clustered-work contract onto the last remaining example surface.
- R088 — S05 keeps `tiny-cluster/` in equal-surface lockstep with the generated scaffold and packaged proof, so the local-first route-free proof remains part of the authoritative clustered-example story instead of drifting into a separate model.
- R089 — S05 keeps rebuilt `cluster-proof/` aligned with the scaffold and `tiny-cluster/` in docs, verifier scope, and operator runbooks, so the packaged route-free proof remains a canonical peer rather than a special-case package narrative.
- R091 — The generated scaffold equal-surface rail now proves the same runtime-owned `meshc cluster status|continuity|diagnostics` inspection path as the proof packages, broadening the practical trust surface for Mesh-owned operator tooling.

## Requirements Validated

- R090 — Validated by `cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture`, `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`, `cargo test -p meshc --test e2e_m044_s03 m044_s03_scaffold_ -- --nocapture`, `cargo test -p meshc --test e2e_m045_s01 m045_s01_ -- --nocapture`, `cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture`, `cargo test -p meshc --test e2e_m045_s03 m045_s03_ -- --nocapture`, `cargo test -p meshc --test e2e_m046_s05 m046_s05_ -- --nocapture`, and `bash scripts/verify-m046-s05.sh`, which together keep `meshc init --clustered`, `tiny-cluster/`, and `cluster-proof/` behaviorally locked to one route-free clustered-work contract.
- R092 — Validated by `npm --prefix website run build`, the routeful-string guards over clustered README/docs surfaces, `cargo test -p meshc --test e2e_m046_s05 m046_s05_ -- --nocapture`, and `bash scripts/verify-m046-s05.sh`, which together prove the public clustered story and closeout rails no longer depend on HTTP submit/status routes for proof or operator truth.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

No goal-level deviation from the slice plan. During T04, the delegated S03 replay exposed stale tiny-cluster README/verifier wording, so the slice also corrected those guards to keep the assembled S05 verifier truthful instead of letting a valid route-free operator flow false-fail inside the retained bundle chain.

## Known Limitations

None.

## Follow-ups

S06 should use `scripts/verify-m046-s05.sh` and the retained `.tmp/m046-s05/verify/latest-proof-bundle.txt` chain as the assembled starting point, then replay the local and packaged route-free proofs together and finish milestone-level validation/docs closeout on top of that single truthful seam.

## Files Created/Modified

- `compiler/mesh-pkg/src/scaffold.rs` — Rewrote the clustered scaffold templates and README contract to emit the route-free equal-surface story and reject routeful drift.
- `compiler/meshc/tests/tooling_e2e.rs` — Updated the clustered init smoke test to assert the new scaffold file set and ban deleted routeful strings.
- `compiler/meshc/tests/support/m046_route_free.rs` — Extended the shared route-free harness with generated-scaffold init/archive helpers used by the live equal-surface proof.
- `compiler/meshc/tests/e2e_m046_s05.rs` — Added the authoritative generated-scaffold equal-surface e2e rail covering parity, two-node startup, CLI-only continuity discovery, and retained artifacts.
- `compiler/meshc/tests/e2e_m044_s03.rs` — Narrowed the older scaffold-era regression rail to route-free generated-project contract and runtime-truth checks.
- `compiler/meshc/tests/e2e_m045_s01.rs` — Repointed historical M045 scaffold/bootstrap rails at the route-free scaffold contract and package parity story.
- `compiler/meshc/tests/e2e_m045_s02.rs` — Reworked the historical runtime-completion rail to prove route-free scaffold completion through CLI-only surfaces.
- `compiler/meshc/tests/e2e_m045_s03.rs` — Reduced the historical failover rail to helper/delegation guards around the shared S05 proof seam.
- `website/docs/docs/getting-started/clustered-example/index.md` — Rewrote clustered getting-started, tooling, and distributed-proof docs plus repo/package READMEs around one equal canonical route-free story.
- `website/docs/docs/tooling/index.md` — Rewrote clustered getting-started, tooling, and distributed-proof docs plus repo/package READMEs around one equal canonical route-free story.
- `website/docs/docs/distributed-proof/index.md` — Rewrote clustered getting-started, tooling, and distributed-proof docs plus repo/package READMEs around one equal canonical route-free story.
- `website/docs/docs/distributed/index.md` — Rewrote clustered getting-started, tooling, and distributed-proof docs plus repo/package READMEs around one equal canonical route-free story.
- `README.md` — Updated the repo overview and public proof map to point at the S05 equal-surface closeout rail.
- `tiny-cluster/README.md` — Aligned the local proof runbook on the shared status → continuity list → continuity record → diagnostics operator sequence.
- `cluster-proof/README.md` — Aligned the packaged proof runbook on the shared status → continuity list → continuity record → diagnostics operator sequence.
- `scripts/verify-m046-s05.sh` — Added the authoritative direct S05 closeout verifier with retained S03/S04/S05 bundle chaining and docs/scaffold replay.
- `scripts/verify-m045-s05.sh` — Reduced the historical M045 wrapper to a thin retained-alias delegate to the authoritative S05 verifier.
- `compiler/meshc/tests/e2e_m045_s05.rs` — Pinned the Rust-side wrapper/content-guard contract to the new S05 phase names, bundle shape, and docs/scaffold scope.
- `compiler/meshc/tests/e2e_m046_s05.rs` — Pinned the Rust-side direct closeout rail to the new S05 phase names, bundle shape, and docs/scaffold scope.
- `.gsd/PROJECT.md` — Updated current project state to reflect S05 completion and leave only S06 assembled closeout open in M046.
- `.gsd/KNOWLEDGE.md` — Recorded reusable guidance for keeping generated-scaffold proof centralized and for debugging the retained S05 proof bundle chain.
