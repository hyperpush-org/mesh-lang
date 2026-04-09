---
verdict: needs-attention
remediation_round: 0
---

# Milestone Validation: M039

## Success Criteria Checklist
- [x] **Automatic cluster formation without manual peer lists.** S01’s canonical verifier (`.tmp/m039-s01/verify/phase-report.txt`) passed `mesh-rt-discovery`, `convergence`, and `node-loss`; the local proof surfaces show two nodes converging from a shared DNS seed with truthful membership.
- [x] **Runtime-native internal work routing is proven, not inferred.** S02’s canonical verifier (`.tmp/m039-s02/verify/phase-report.txt`) passed `s02-remote-route` and `s02-local-fallback`; route artifacts show distinct ingress/target/execution nodes for the healthy cluster and truthful self-only fallback when no peer exists.
- [x] **Single-cluster node loss degrades safely and rejoin restores full behavior without manual repair.** S03’s canonical verifier (`.tmp/m039-s03/verify/phase-report.txt`) passed `s03-degrade` and `s03-rejoin`; S04’s local one-image verifier re-proved the same behavior through Docker artifacts at `.tmp/m039-s04/verify/07-degraded/*` and `.tmp/m039-s04/verify/08-post-rejoin/*`.
- [x] **One proof app / one image / one operator path exists locally.** `bash scripts/verify-m039-s04.sh` was re-run during validation and passed. It built `cluster-proof/Dockerfile`, started two containers from the same image, proved convergence, degrade, and rejoin, and left durable JSON/log evidence under `.tmp/m039-s04/verify/`.
- [~] **Local/Fly verifier + docs truth are delivered, but only local execution was evidenced in this validation pass.** `bash scripts/verify-m039-s04-proof-surface.sh` passed, `cluster-proof/fly.toml` encodes the one-image Fly contract, and `bash scripts/verify-m039-s04-fly.sh --help` confirms the read-only Fly verifier surface. However, no live `.tmp/m039-s04/fly/` artifact set was present from this validation run, so the Fly half of the operator story is implemented/documented but not re-proven here.

## Slice Delivery Audit
| Slice | Planned deliverable / demo claim | Delivered evidence | Verdict |
| --- | --- | --- | --- |
| S01 | General DNS discovery seam plus truthful live membership from runtime sessions | Runtime files exist in `compiler/mesh-rt/src/dist/{discovery.rs,node.rs,mod.rs}`; proof app surfaces exist under `cluster-proof/`; `.tmp/m039-s01/verify/phase-report.txt` shows build/discovery/convergence/node-loss all passed. | PASS |
| S02 | Narrow proof endpoint that distinguishes ingress from execution and proves internal routing | `cluster-proof/work.mpl` and `compiler/meshc/tests/e2e_m039_s02.rs` exist; `.tmp/m039-s02/verify/phase-report.txt` passed remote-route and local-fallback; route-body artifacts show `ingress_node != execution_node` on healthy runs. | PASS |
| S03 | Safe degrade after node loss, continued service, and clean rejoin without manual repair | `compiler/meshc/tests/e2e_m039_s03.rs` contains explicit degrade/rejoin tests; `.tmp/m039-s03/verify/phase-report.txt` passed `s03-degrade` and `s03-rejoin`; artifacts include pre-loss, degraded, and post-rejoin membership/work JSON plus per-node logs. | PASS |
| S04 | One-image operator path, local/Fly verifiers, and docs truth | `cluster-proof/Dockerfile`, `cluster-proof/fly.toml`, `cluster-proof/README.md`, and the distributed-proof docs surface exist; `bash scripts/verify-m039-s04.sh` and `bash scripts/verify-m039-s04-proof-surface.sh` passed during validation; the read-only Fly verifier exists and exposes the intended contract. Live Fly execution evidence was not re-captured in this validation pass. | PASS with attention |

## Cross-Slice Integration
- **S01 → S02 boundary holds.** S02 explicitly replays S01 before proving routing, and `.tmp/m039-s02/verify/phase-report.txt` shows `s01-contract` passed before the routing phases. This substantiates that routing depends on the live membership/discovery truth established in S01 rather than bypassing it.
- **S01/S02 → S03 boundary holds.** `.tmp/m039-s03/verify/phase-report.txt` shows `s01-contract` and `s02-contract` both passed before `s03-degrade` and `s03-rejoin`. The failure/rejoin work reused the earlier `/membership` and `/work` contracts instead of inventing a parallel proof path.
- **S03 → S04 boundary holds.** `scripts/verify-m039-s04.sh` begins by replaying `scripts/verify-m039-s03.sh`; the resulting `.tmp/m039-s04/verify/01-s03-phase-report.txt` shows the full S03 contract passed before the one-image Docker proof. This demonstrates that S04 wrapped the already-proven runtime behavior in an operator package rather than replacing it.
- **Docs/runbook/operator contract alignment holds.** `bash scripts/verify-m039-s04-proof-surface.sh` passed, proving the README, distributed guide, distributed-proof page, runbook, and verifier command list all point to the same canonical surfaces.
- **Boundary mismatch / gap:** no material slice-to-slice mismatch was found locally. The only evidence gap is external: a live Fly verifier artifact (`.tmp/m039-s04/fly/`) was not present from this validation run, so the Fly contract is wired but not freshly reconciled here.

## Requirement Coverage
- **R045** — Covered by S01 and preserved through downstream verifiers. Local automatic convergence from a shared DNS seed without manual peers is evidenced by `.tmp/m039-s01/verify/phase-report.txt` and re-consumed by S02/S03/S04 contract replays.
- **R046** — Covered by S01 and extended operationally by S03/S04. Truthful membership from live runtime sessions is evidenced by S01’s node-loss proof and by S04’s degraded membership artifact `.tmp/m039-s04/verify/07-degraded/degraded-node-a-membership.json`.
- **R047** — Covered and validated by S02, then re-proved in S04. Remote routing is evidenced by `.tmp/m039-s02/verify/phase-report.txt` plus `.tmp/m039-s04/verify/06-pre-loss/pre-loss-work.json` and `.tmp/m039-s04/verify/08-post-rejoin/post-rejoin-work.json`.
- No inlined requirement was invalidated or re-scoped during validation.
- No active M039 requirement lacks slice ownership in the available evidence. The remaining gap is evidentiary rather than implementation-oriented: the repo now contains the Fly verifier and Fly config contract, but this validation pass did not produce a fresh live Fly proof artifact.

## Verdict Rationale
**Verdict: needs-attention.**

The milestone’s core cluster story is substantively delivered and reproducible locally. During validation I re-ran `bash scripts/verify-m039-s04.sh`, which passed and rebuilt the assembled proof from the one-image Docker path all the way through convergence, degraded local fallback, and clean rejoin. I also re-ran `bash scripts/verify-m039-s04-proof-surface.sh`, which passed and confirmed that the public docs, README, runbook, and verifier surfaces are mutually consistent.

**Verification class reconciliation**
- **Contract:** Addressed. S01–S04 canonical verifiers all report passed phase ledgers, and the S04 proof surface plus Fly verifier contract are present.
- **Integration:** Addressed locally, partially evidenced for Fly. Real multi-process / multi-container clustered runs were proven locally in S01–S04. The Fly integration contract is implemented (`cluster-proof/fly.toml`, `scripts/verify-m039-s04-fly.sh`, docs/runbook wiring), but no fresh live Fly artifact set was available in this validation pass.
- **Operational:** Addressed locally. Node loss, degraded truthful membership, continued work acceptance, and rejoin recovery are all evidenced by S03 and the re-run S04 Docker verifier.
- **UAT:** Adequately addressed for local proof surfaces through the existing slice UAT/verifier structure, but no separate live Fly UAT artifact was available here.

Because the missing evidence is confined to the live Fly execution path, while the implementation, read-only verifier, config contract, and documentation truth are all in place, the gap is **minor but real** rather than remediation-worthy.

**Deferred Work Inventory**
- Capture and retain one durable `.tmp/m039-s04/fly/` artifact set from `scripts/verify-m039-s04-fly.sh` against a live deployed app so the Fly half of the S04 story is evidenced alongside the already-proven local operator path.
