---
verdict: pass
remediation_round: 1
---

# Milestone Validation: M047

## Success Criteria Checklist
- [x] Users can declare clustered functions with `@cluster` / `@cluster(N)` and see replication-count semantics reflected in real compiler/runtime truth.
  - Evidence: S01 delivered the source-first decorator reset across parser, mesh-pkg, meshc, and mesh-lsp; S02 preserved default/explicit replication counts into runtime continuity and `meshc cluster continuity` truth.
- [x] Users can cluster selected HTTP routes with `HTTP.clustered(...)` wrapper syntax inside ordinary router chains, and live route behavior proves the handler is the clustered boundary.
  - Evidence: S07 shipped compiler-known `HTTP.clustered(handler)` / `HTTP.clustered(N, handler)` lowering, runtime dispatch, truthful continuity records keyed to the route handler runtime name, and the dedicated two-node `cargo test -p meshc --test e2e_m047_s07 -- --nocapture` rail.
- [x] Canonical examples, scaffold output, docs, and proof rails no longer teach `clustered(work)` or `.toml` clustering as the public model.
  - Evidence: S04 completed the hard cutover to `@cluster`; S08 removed stale `HTTP.clustered(...) is still not shipped` public wording from README and VitePress docs while keeping the canonical route-free examples first.
- [x] A dedicated scaffold command generates a simple SQLite Todo API with actors, rate limiting, clustered routes, and a complete Dockerfile that reads like a starting point instead of a proof app.
  - Evidence: S05 shipped the Todo scaffold with SQLite/runtime/Docker proof; S08 adopted `HTTP.clustered(1, ...)` truthfully on the selected read routes, retained native and Docker continuity bundles, and kept the route-free `work.mpl` seam explicit.
- [x] One final closeout rail proves the syntax reset, clustered routes, dogfooded examples, scaffold, Docker path, and migration/docs story together.
  - Evidence: S08 recovered the assembled closeout chain so `bash scripts/verify-m047-s05.sh`, `cargo test -p meshc --test e2e_m047_s06 m047_s06_ -- --nocapture`, `bash scripts/verify-m047-s06.sh`, and `npm --prefix website run build` all pass with retained proof bundles.

- **Contract:** MET — parser/typechecker/LSP/compiler/runtime/e2e rails now cover both source-first clustered functions and shipped clustered HTTP wrappers.
- **Integration:** MET — the milestone end state is coherent as `S01 -> S02 -> S07 -> S08`, with S04 preserving the hard cutover and S05/S06 rebased by S08 onto the shipped wrapper truth.
- **Operational:** MET — `.tmp/m047-s05/verify/status.txt = ok`, `.tmp/m047-s06/verify/status.txt = ok`, native + Docker Todo proof bundles are retained, and the docs build completes.
- **UAT:** MET — S07-UAT and S08-UAT enumerate and pass the live two-node route-wrapper rail, native/Docker Todo route proof, and docs/closeout replay.

## Slice Delivery Audit
| Slice | Planned deliverable | Evidence found during re-validation | Verdict |
| --- | --- | --- | --- |
| S01 | Source decorator reset for clustered functions | `@cluster` / `@cluster(N)` shipped across parser, mesh-pkg, meshc, and mesh-lsp; summaries/UAT and M047 S01 rails remain green. | delivered |
| S02 | Runtime replication-count semantics for clustered functions | Replication counts now survive into runtime continuity truth and CLI/operator output; S02 rails and summaries substantiate default `2` plus explicit-count behavior. | delivered |
| S03 | Clustered HTTP route wrapper | The original slice did not land the full feature in its first pass, but its planned deliverable is now satisfied by the remediation chain: S07 ships the wrapper/compiler/runtime path and live HTTP proof. | delivered via remediation |
| S04 | Hard cutover away from `clustered(work)` / `[cluster]` in public surfaces and proof rails | The source-first cutover remains intact and is replayed by later verifier layers; legacy public syntax stays retired. | delivered |
| S05 | Simple clustered Todo scaffold | The SQLite Todo scaffold, routes, actors, rate limiting, and Docker packaging shipped in S05; S08 rebased the selected read routes onto the real `HTTP.clustered(1, ...)` wrapper contract. | delivered |
| S06 | Final docs/migration/assembled proof closeout | The closeout verifier structure shipped in S06 and was later rebased by S08 so the final rails now exercise the truthful public contract instead of the earlier route-free-only wording. | delivered via remediation |
| S07 | Clustered HTTP route wrapper completion | `HTTP.clustered(handler)` / `HTTP.clustered(N, handler)` now lower through deterministic route shims onto the shared declared-handler seam and pass the dedicated live two-node proof. | delivered |
| S08 | Clustered route adoption in scaffold, docs, and closeout proof | The Todo starter, README, VitePress docs, native/Docker proof rails, and assembled M047 verifier now adopt the shipped wrapper truthfully and retain the recovered proof bundles. | delivered |

## Cross-Slice Integration
- **S01 -> S02:** aligned. The source-first declaration seam and runtime-name metadata introduced in S01 are the same surfaces S02 uses for replication-count truth.
- **S01/S02 + S04 -> S07:** aligned after remediation. S07 did not invent a route-local side channel; it reused the shared declared-handler runtime-name/replication-count seam that S01/S02 established, while S04 preserved the source-first cutover boundary.
- **S05/S06 -> S08:** aligned after remediation. S05's scaffold/runtime work and S06's verifier/docs wrapper were both honestly route-free at the time; S08 rebased those downstream surfaces onto the shipped clustered-route seam without reopening the compiler/runtime core.
- **Net effect:** the milestone end state now matches the roadmap vision. The initial `needs-remediation` validation was accurate before S07/S08; after those slices landed, the remaining cross-slice mismatch is resolved and the delivered chain is coherent.

## Requirement Coverage
- **R097** — validated by S01 and S04: `@cluster` / `@cluster(N)` are the public clustered function spellings and legacy `clustered(work)` is retired from canonical public surfaces.
- **R098** — validated by S02 and consumed by S07/S08: omitted counts default to `2`, explicit counts are preserved, and route-local clustered wrappers express replication count truthfully (`HTTP.clustered(1, ...)` in the Todo starter, `HTTP.clustered(2, ...)` default behavior in S07).
- **R099** — validated by S01/S02/S04/S06: clustering remains a general function capability, with canonical route-free `@cluster` examples still leading the public story.
- **R100** — validated by S07 and adopted by S08: router chains now support `HTTP.clustered(handler)` / `HTTP.clustered(N, handler)` and live route behavior proves the wrapper is real.
- **R101** — validated by S07: continuity records and live HTTP responses show the route handler runtime name as the distributed boundary while downstream calls execute naturally inside that clustered request.
- **R102** — validated by S04 and preserved by S08: the old `clustered(work)` public surface is removed rather than kept as a coequal model.
- **R103** — validated by S04, S05, and S08: repo-owned examples, scaffold output, and public proof rails are dogfooded onto the source-first model.
- **R104** — validated by S05 and S08: `meshc init --template todo-api` now generates the SQLite Todo API with real routes, actor-backed rate limiting, selected clustered read routes, and a complete Dockerfile/proof path.
- **R105** — validated by S05 and S08: the scaffold keeps clustering obvious and boilerplate low by using plain `@cluster` and narrow wrapper adoption that reads like a starting point rather than a proof harness.
- **R106** — validated by S04, S08, and the assembled closeout rails: public docs and migration guidance now teach one coherent source-first clustered model, with S07 as the authority for the broader route-wrapper behavior and S08 as the public adoption layer.

## Verdict Rationale
M047 now passes as planned. The original validation captured a real gap: at that point, the repo had completed a strong route-free `@cluster` reset but had not yet shipped the promised clustered HTTP route wrapper or downstream adoption. That gap is no longer present. S07 delivered the missing `HTTP.clustered(...)` compiler/runtime/e2e seam and the live two-node proof rail, and S08 adopted that shipped seam into the Todo scaffold, public docs, and assembled closeout verifiers without weakening the route-free canonical story.

The evidence is consistent across contract, integration, operational, and UAT layers. **Contract** is satisfied by the green parser/typechecker/LSP/codegen/runtime/e2e surfaces for both source-first clustered functions and the clustered route wrapper. **Integration** is satisfied because the final milestone chain is coherent: S04 preserves the hard cutover, S07 provides the missing route-wrapper core, and S08 rebases the scaffold/docs/verifier layer onto that real implementation. **Operational: MET** — the retained native and Docker Todo proof bundles plus `.tmp/m047-s05/verify/status.txt = ok` and `.tmp/m047-s06/verify/status.txt = ok` show the delivered surface actually runs and remains inspectable. **UAT** is satisfied by the named S07 and S08 UAT rails covering live HTTP clustered-route behavior, native/Docker Todo proof, and docs/closeout replay.

Because the remediation slices are complete and the milestone's visible roadmap promises are now met, the correct verdict is `pass`, remediation round `1`. No open milestone-level blocker remains in the validation artifact itself.
