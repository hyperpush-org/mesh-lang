# S06: Assembled verification and docs closeout

**Goal:** Close M046 with one assembled route-free proof/documentation rail that republishes the local, packaged, and scaffold equal-surface evidence as the final truthful clustered operator story, then seal milestone validation from that rail.
**Demo:** After this: After this: one assembled verifier replays the local and packaged route-free proofs and re-checks startup-triggered work, failover, and status truth end to end.

## Tasks
- [x] **T01: Pinned the S06 closeout hierarchy with Rust contract guards and the assembled verifier/doc alias chain they enforce.** — Add Rust-side content guards for the new S06 proof hierarchy before the scripts and docs move, so stale S05-authoritative assumptions fail closed instead of surviving until the shell verifier runs.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `compiler/meshc/tests/e2e_m046_s06.rs` new contract rail | Fail if the authoritative S06 script, targeted replay phases, retained bundle names, or docs references are missing. | N/A | Treat mixed S05/S06 authority claims or missing bundle markers as contract drift. |
| `compiler/meshc/tests/e2e_m045_s05.rs` historical alias guard | Fail if the M045 wrapper still points at S05 or if it reasserts direct docs/build work instead of delegation. | N/A | Treat missing retained verify files or stale delegated phase names as alias drift. |
| `compiler/meshc/tests/e2e_m046_s05.rs` equal-surface subrail guard | Fail if S05 still claims final authority or if demotion accidentally deletes the retained S05 bundle contract that S06 wraps. | N/A | Treat ambiguous authoritative/historical wording as a regression. |

## Negative Tests

- **Malformed inputs**: missing `latest-proof-bundle.txt`, missing `retained-m046-s05-verify`, missing `retained-m046-s06-artifacts`, or mismatched phase labels.
- **Error paths**: S05 still named authoritative in wrapper/document contracts, `verify-m045-s05.sh` delegating to the wrong script, or docs/readmes still pointing at S05 as present-tense truth.
- **Boundary conditions**: S05 may remain a lower-level equal-surface subrail, but the tests must clearly distinguish authoritative S06 from delegated or historical rails.

## Steps

1. Add `compiler/meshc/tests/e2e_m046_s06.rs` as a pure contract/content guard that asserts the new S06 verifier phases, retained bundle names, targeted S03/S04 truth replays, and public docs references without duplicating the runtime harness.
2. Update `compiler/meshc/tests/e2e_m045_s05.rs` so the historical wrapper test expects delegation to `scripts/verify-m046-s06.sh`, the S06 retained verify directory, and the S06 phase/bundle contract instead of the S05 hierarchy.
3. Update `compiler/meshc/tests/e2e_m046_s05.rs` so S05 stays pinned as the equal-surface subrail S06 wraps, not the final authoritative closeout rail.
4. Keep the assertions focused on script/doc/bundle contract only; do not fork a second startup/failover runtime proof into the S06 test file.

## Must-Haves

- [ ] `compiler/meshc/tests/e2e_m046_s06.rs` exists and pins the S06 verifier hierarchy, phase names, and retained bundle shape.
- [ ] `compiler/meshc/tests/e2e_m045_s05.rs` fails closed if the historical wrapper does not delegate to S06.
- [ ] `compiler/meshc/tests/e2e_m046_s05.rs` still protects the S05 equal-surface subrail but no longer claims final authority.
- [ ] Rust content guards cover authoritative/historical doc references so stale S05 wording fails before the shell verifier runs.

## Done When

- [ ] `cargo test -p meshc --test e2e_m046_s06 m046_s06_ -- --nocapture` passes.
- [ ] `cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture` passes against the repointed hierarchy.
  - Estimate: 2h
  - Files: compiler/meshc/tests/e2e_m046_s06.rs, compiler/meshc/tests/e2e_m045_s05.rs, compiler/meshc/tests/e2e_m046_s05.rs
  - Verify: cargo test -p meshc --test e2e_m046_s06 m046_s06_ -- --nocapture && cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture
- [x] **T02: Verified the S06 closeout rail and historical M045 alias chain, and documented the required sequential replay order.** — Create the final S06 shell verifier as the truthful M046 closeout rail, then make the historical alias layer depend on that rail instead of pretending S05 is still the top of the stack.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `scripts/verify-m046-s06.sh` direct verifier | Fail on the first delegated replay, targeted truth-rail, artifact-copy, or bundle-shape error and keep phase/status/current-phase/latest-proof-bundle artifacts plus command logs. | Bound delegated verifier and targeted test phases; do not report success if any replay hangs. | Treat missing retained verify files, missing copied bundles, or malformed bundle pointers as verifier failures. |
| `scripts/verify-m045-s05.sh` historical wrapper | Fail closed if it still delegates to S05 or if it omits the S06 phase and retained-bundle checks. | Let the delegated S06 timeout propagate instead of masking it as a historical no-op. | Treat missing retained S06 artifacts as alias drift. |
| Existing S03/S04/S05 verifiers and targeted S03/S04 rails | Stop immediately if any lower rail regresses; do not add a second runtime harness or a compensating app-owned control surface. | Preserve the lower-rail timeout/failure semantics and artifact paths inside the S06 retained bundle. | Reject stale route/status/timing seams or missing `meshc cluster` truth artifacts as proof failure. |

## Load Profile

- **Shared resources**: delegated S05 replay, targeted S03/S04 runtime tests, copied `.tmp/m046-s03`, `.tmp/m046-s04`, `.tmp/m046-s05`, and `.tmp/m046-s06` artifact trees.
- **Per-operation cost**: one full equal-surface replay plus targeted startup/failover/package truth reruns and retained-bundle assembly.
- **10x breakpoint**: verifier runtime and artifact churn will fail first; this task is about proof-surface integrity rather than throughput.

## Negative Tests

- **Malformed inputs**: missing `status.txt`, `current-phase.txt`, `phase-report.txt`, `latest-proof-bundle.txt`, or copied retained bundles from delegated rails.
- **Error paths**: stale wrapper delegation to `verify-m046-s05.sh`, missing targeted S03/S04 truth phases, or an S06 verifier that stops checking retained bundle shape.
- **Boundary conditions**: S05 may stay as the equal-surface subrail, but S06 must be the only direct closeout seam and must not reintroduce app-owned status/submit/timing checks.

## Steps

1. Add `scripts/verify-m046-s06.sh` as the direct M046 closeout verifier rooted at `.tmp/m046-s06/verify/`: replay `scripts/verify-m046-s05.sh`, retain the delegated S05 verify directory, rerun the targeted S03 local startup/failover truth rails and the targeted S04 packaged startup truth rail, and copy the fresh retained artifacts under one S06-owned bundle root.
2. Make the S06 verifier publish `status.txt`, `current-phase.txt`, `phase-report.txt`, `full-contract.log`, and `latest-proof-bundle.txt`, and fail closed if any delegated phase, targeted replay, or copied artifact bundle is missing or malformed.
3. Repoint `scripts/verify-m045-s05.sh` so it delegates to `scripts/verify-m046-s06.sh`, retains the delegated verify directory locally, and checks the S06 phase/bundle contract instead of the old S05 boundary.
4. Keep the final assembled verifier route-free: reuse the existing runtime-owned `meshc cluster status|continuity|diagnostics` proof rails and do not add any app-owned status route, submit route, or timing helper to make S06 pass.

## Must-Haves

- [ ] `scripts/verify-m046-s06.sh` is the authoritative closeout verifier for M046 and owns `.tmp/m046-s06/verify/`.
- [ ] The S06 verifier wraps S05, reruns targeted S03/S04 truth rails, and publishes a retained `latest-proof-bundle.txt` pointer for downstream diagnosis.
- [ ] `scripts/verify-m045-s05.sh` becomes a thin historical alias that only passes by delegating to S06.
- [ ] The assembled verifier fails closed on missing retained files, malformed bundle pointers, stale routeful drift, or lower-rail regressions.

## Done When

- [ ] `bash scripts/verify-m046-s06.sh` passes and leaves a diagnosable retained S06 bundle chain.
- [ ] `bash scripts/verify-m045-s05.sh` passes only by delegating to the S06 rail.
  - Estimate: 3h
  - Files: scripts/verify-m046-s06.sh, scripts/verify-m045-s05.sh
  - Verify: cargo test -p meshc --test e2e_m046_s06 m046_s06_ -- --nocapture && cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture && bash scripts/verify-m046-s06.sh && bash scripts/verify-m045-s05.sh
- [x] **T03: Repointed clustered docs/runbooks to the S06 closeout rail and expanded the S06 Rust guard across the full clustered docs surface.** — Promote the new S06 closeout rail across the public clustered story without changing the underlying operator flow or reopening any routeful example seams.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `README.md` and clustered website docs | Fail `npm --prefix website run build` if the new references break markdown, links, or VitePress rendering. | Treat a slow docs build as task failure; do not leave the public story half-rewritten. | Treat contradictory S05/S06 authority wording or revived routeful instructions as docs drift. |
| `tiny-cluster/README.md` and `cluster-proof/README.md` runbooks | Fail the doc/content guards if the package READMEs stop sharing the same status → continuity list → continuity record → diagnostics operator sequence. | N/A | Treat stale wrapper names or request-key-only continuity guidance as incomplete operator documentation. |

## Negative Tests

- **Malformed inputs**: stale references to `scripts/verify-m046-s05.sh` as current truth, `[cluster]`, `Continuity.submit_declared_work(...)`, `/health`, `/work/:request_key`, or proof-only timing edits.
- **Error paths**: docs must clearly demote S05 to the equal-surface subrail and keep `scripts/verify-m045-s05.sh` historical instead of presenting multiple present-tense closeout rails.
- **Boundary conditions**: the three canonical clustered surfaces may keep scope-specific notes, but they must share the same runtime-owned operator flow and final closeout pointer.

## Steps

1. Update `README.md`, `website/docs/docs/distributed-proof/index.md`, `website/docs/docs/distributed/index.md`, `website/docs/docs/tooling/index.md`, and `website/docs/docs/getting-started/clustered-example/index.md` so they name `scripts/verify-m046-s06.sh` as the authoritative route-free closeout rail and demote `scripts/verify-m046-s05.sh` to the lower-level equal-surface subrail.
2. Update `tiny-cluster/README.md` and `cluster-proof/README.md` so their package runbooks still teach the canonical `meshc cluster status`, continuity list, continuity record, diagnostics sequence while pointing repo-wide closeout readers at S06.
3. Keep the three clustered surfaces (`meshc init --clustered`, `tiny-cluster/`, and `cluster-proof/`) explicitly equal and canonical, preserving R090 while preventing stale S05-only wording from becoming the public truth.
4. Remove any remaining routeful or app-owned operator language from the slice-owned docs surfaces instead of trying to explain both stories side by side.

## Must-Haves

- [ ] Public docs and repo/package READMEs name `scripts/verify-m046-s06.sh` as the final closeout rail.
- [ ] `scripts/verify-m046-s05.sh` is described only as the equal-surface subrail and `scripts/verify-m045-s05.sh` remains clearly historical.
- [ ] The operator flow stays `meshc cluster status`, continuity list, continuity record, diagnostics across all clustered surfaces.
- [ ] Routeful/app-owned submit/status/timing instructions do not reappear in the repointed docs surfaces.

## Done When

- [ ] `npm --prefix website run build` passes against the repointed docs.
- [ ] The S06 Rust doc/content guards pass against the updated authoritative-versus-historical wording.
  - Estimate: 2h
  - Files: README.md, website/docs/docs/distributed-proof/index.md, website/docs/docs/distributed/index.md, website/docs/docs/tooling/index.md, website/docs/docs/getting-started/clustered-example/index.md, tiny-cluster/README.md, cluster-proof/README.md
  - Verify: npm --prefix website run build && cargo test -p meshc --test e2e_m046_s06 m046_s06_ -- --nocapture && ! rg -n "\[cluster\]|Continuity\.submit_declared_work|/health|/work/:request_key|Timer\.sleep\(5000\)" README.md website/docs/docs/distributed-proof/index.md website/docs/docs/distributed/index.md website/docs/docs/tooling/index.md website/docs/docs/getting-started/clustered-example/index.md tiny-cluster/README.md cluster-proof/README.md
- [x] **T04: Validated M046 from the green S06 assembled closeout rail and repointed project state at the retained S06 proof bundle.** — Finish the slice by proving the final S06 rail is green, then turn that evidence into the milestone validation artifact instead of claiming closeout from planning alone.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `scripts/verify-m046-s06.sh` full replay | Stop on the first failed phase and inspect `.tmp/m046-s06/verify/` instead of papering over a red proof surface. | Keep the bounded S06 timeout; do not mark the milestone validated if the assembled proof never converges. | Treat missing phase logs, stale bundle pointers, or absent S03/S04/S05/S06 retained artifacts as blocker-level failures. |
| Checked-in requirements and slice summaries | Fail validation if the evidence chain does not directly cover R086, R087, R088, R089, R090, R091, R092, and R093. | N/A | Treat requirements/summary mismatches as validation gaps, not wording issues to hand-wave away. |
| `gsd_validate_milestone` output | Do not claim milestone pass if the tool call or rendered markdown fails. | N/A | Treat a malformed or incomplete validation artifact as unfinished closeout. |

## Load Profile

- **Shared resources**: the full S06 replay, nested retained proof bundles, and the checked-in milestone docs/state files.
- **Per-operation cost**: one full assembled verifier run plus one milestone-validation render grounded in the resulting evidence.
- **10x breakpoint**: the verifier replay dominates cost; validation writing itself is cheap once the evidence is green.

## Negative Tests

- **Malformed inputs**: missing retained bundle pointer, stale S05-only authority claims, or incomplete requirement evidence for docs/verification closeout.
- **Error paths**: red verifier status, missing `phase-report.txt`, missing S06 bundle members, or validation content that cannot explain how the active M046 requirements were re-proved.
- **Boundary conditions**: historical wrapper rails may still exist, but milestone validation must cite the green S06 rail as the final truthful closeout surface.

## Steps

1. Run `bash scripts/verify-m046-s06.sh` and inspect `.tmp/m046-s06/verify/status.txt`, `current-phase.txt`, `phase-report.txt`, and `latest-proof-bundle.txt` to confirm the assembled S03/S04/S05/S06 evidence chain is complete.
2. Update `.gsd/PROJECT.md` so the current project state reflects that S06 is the final M046 closeout rail and that milestone validation is now grounded in the S06 bundle.
3. Use the green S06 evidence, the checked-in requirements, the M046 roadmap, and the completed S03/S04/S05 summaries to populate the M046 success-criteria checklist, slice delivery audit, cross-slice integration, and requirement coverage with current-state proof instead of planning intent.
4. Call `gsd_validate_milestone` for `M046` only after the rail is green, writing `.gsd/milestones/M046/M046-VALIDATION.md` with a `pass` verdict tied directly to the S06 evidence chain.

## Must-Haves

- [ ] The full S06 verifier is green before milestone validation is recorded.
- [ ] `.gsd/PROJECT.md` names S06 as the final M046 closeout rail and points future agents at the S06 retained bundle.
- [ ] `.gsd/milestones/M046/M046-VALIDATION.md` exists and explicitly covers R086, R087, R088, R089, R090, R091, R092, and R093.
- [ ] The validation artifact cites the S06 rail as the final authoritative proof surface instead of S05.

## Done When

- [ ] `bash scripts/verify-m046-s06.sh` passes and leaves a non-empty retained bundle pointer.
- [ ] The M046 validation artifact is rendered and matches the green S06 evidence chain.
  - Estimate: 2h
  - Files: .gsd/PROJECT.md, .gsd/milestones/M046/M046-VALIDATION.md
  - Verify: bash scripts/verify-m046-s06.sh && test -s .gsd/milestones/M046/M046-VALIDATION.md && rg -n "verify-m046-s06|R086|R091|R092" .gsd/milestones/M046/M046-VALIDATION.md .gsd/PROJECT.md
