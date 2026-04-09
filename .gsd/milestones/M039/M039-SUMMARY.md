---
id: M039
title: "Auto-Discovery & Native Cluster Balancing"
status: complete
completed_at: 2026-03-28T16:41:07.995Z
key_decisions:
  - Treat DNS discovery results as bootstrap candidates only; canonical membership truth comes from validated handshake-advertised node identities in live runtime sessions.
  - Keep `cluster-proof` narrow by separating truthful `/membership` state from the `/work` routing proof surface.
  - Use a direct spawn-and-return `/work` path plus execution-node logs as the honest internal-routing proof while distributed payload transport remains scalar-only.
  - Keep request correlation ingress-owned across degrade/rejoin and preserve run-numbered logs/artifacts instead of relying on restart-unsafe cross-node payload state.
  - Ship three distinct S04 proof surfaces: the local Docker continuity verifier, the read-only Fly verifier contract, and the docs-truth verifier.
  - Derive local container identity from `HOSTNAME` only in the image entrypoint, and only when cluster mode is requested without an explicit local or Fly identity already set.
key_files:
  - compiler/mesh-rt/src/dist/discovery.rs
  - compiler/mesh-rt/src/dist/node.rs
  - cluster-proof/config.mpl
  - cluster-proof/cluster.mpl
  - cluster-proof/work.mpl
  - cluster-proof/Dockerfile
  - cluster-proof/docker-entrypoint.sh
  - cluster-proof/fly.toml
  - compiler/meshc/tests/e2e_m039_s01.rs
  - compiler/meshc/tests/e2e_m039_s02.rs
  - compiler/meshc/tests/e2e_m039_s03.rs
  - scripts/verify-m039-s01.sh
  - scripts/verify-m039-s02.sh
  - scripts/verify-m039-s03.sh
  - scripts/verify-m039-s04.sh
  - scripts/verify-m039-s04-fly.sh
  - scripts/verify-m039-s04-proof-surface.sh
  - website/docs/docs/distributed-proof/index.md
  - website/docs/docs/distributed/index.md
  - README.md
lessons_learned:
  - For milestone closeout on local `main`, `origin/main` is the correct non-`.gsd` diff baseline; comparing `HEAD` to `merge-base(HEAD, main)` can falsely look empty even when the milestone changed real code.
  - Distributed proof surfaces stay honest when each later verifier replays earlier slice contracts before asserting new behavior; S04’s one-image proof is trustworthy because it replays S03, which replays S01 and S02.
  - For clustered operator proofs, capture DNS preflight evidence before trusting membership or routing artifacts; otherwise network-alias drift and runtime regressions can look identical after the fact.
  - Public distributed docs should be mechanically tied to the runbook and verifier command list, not maintained as a parallel narrative surface.
---

# M039: Auto-Discovery & Native Cluster Balancing

**M039 turned Mesh’s distributed-runtime claims into a replayable cluster proof: nodes auto-discover from a shared DNS seed, route work internally across the runtime, survive single-node loss and same-identity rejoin, and expose a canonical one-image local operator/docs surface.**

## What Happened

M039 assembled the distributed cluster story in four slices and re-verified it as one contract. S01 moved discovery and membership truth into `mesh-rt`, added the narrow `cluster-proof` app, and established the authoritative local verifier/artifact layout. S02 extended that proof app with a typed `/work` route that distinguishes ingress, target, and execution nodes while preserving `/membership` as the truthful cluster-state surface. S03 added the continuity proof for node loss, safe self-only degrade, continued work acceptance, same-identity rejoin, and restored remote routing. S04 wrapped the proven runtime behavior in a repo-root Docker image, a guarded entrypoint, a local one-image Docker verifier, a read-only Fly verifier contract, and a mechanically-checked distributed-proof docs surface.

For milestone closeout, I verified the assembled implementation instead of trusting slice-level completion alone. Using the repo’s closeout baseline rule, `git diff --stat "$(git merge-base HEAD origin/main)" HEAD -- ':!.gsd/'` showed substantial non-`.gsd` code changes across `mesh-rt`, `cluster-proof`, `meshc` e2e harnesses, shell verifiers, Docker/Fly packaging, and docs surfaces. I re-ran `bash scripts/verify-m039-s04.sh`, which passed and rebuilt the entire local story from the one-image operator path: S03 replay succeeded, the Docker image built, DNS alias preflight passed, pre-loss two-node convergence and remote `/work` routing passed, degraded self-only membership plus local-fallback `/work` passed after node loss, and post-rejoin two-node membership plus restored remote `/work` passed after same-identity restart. I also re-ran `bash scripts/verify-m039-s04-proof-surface.sh`, `bash -n scripts/verify-m039-s04-fly.sh`, `bash scripts/verify-m039-s04-fly.sh --help`, and `npm --prefix website run build`; all passed. A live Fly replay was attempted with the canonical `mesh-cluster-proof` app name, but Fly returned `Could not find App "mesh-cluster-proof"`, so the repo’s Fly verifier/config/runbook contract is present while fresh live-environment evidence remains external to this closeout environment.

The resulting milestone is now honest about what Mesh can prove today: local automatic discovery from a shared DNS seed, truthful membership derived from live runtime sessions, runtime-native internal work routing, single-cluster safe degrade and clean rejoin, a canonical local one-image operator proof, and docs/runbook surfaces that point to the real verifier commands instead of tutorial-only distributed claims.

## Success Criteria Results

## Success Criteria Verification

- ✅ **Automatic cluster formation without manual peer lists is proven.**
  - `bash scripts/verify-m039-s04.sh` passed.
  - `.tmp/m039-s04/verify/05-dns-preflight/` captured shared-seed resolution to both container IPs before cluster truth was trusted.
  - `.tmp/m039-s04/verify/06-pre-loss/pre-loss-node-a-membership.json` shows `membership=["node-a@node-a:4370","node-b@node-b:4370"]` with `peers=["node-b@node-b:4370"]` after starting two nodes from one image and one shared discovery alias.
  - S01’s canonical verifier remains green via prerequisite replay inside S03/S04.

- ✅ **Runtime-native internal balancing is proven, not inferred.**
  - S02’s contract was replayed through S03 and then through S04.
  - `.tmp/m039-s04/verify/06-pre-loss/pre-loss-work.json` shows `ingress_node=node-a@node-a:4370`, `target_node=node-b@node-b:4370`, `execution_node=node-b@node-b:4370`, and `routed_remotely=true`.
  - `.tmp/m039-s04/verify/08-post-rejoin/post-rejoin-work.json` re-proves the same internal routing after recovery.

- ✅ **Single-cluster node loss degrades safely and same-identity rejoin restores full behavior without manual repair.**
  - `bash scripts/verify-m039-s04.sh` passed `degrade` and `rejoin` phases.
  - `.tmp/m039-s04/verify/07-degraded/degraded-node-a-membership.json` shows truthful self-only membership after peer loss.
  - `.tmp/m039-s04/verify/07-degraded/degraded-work.json` shows `fell_back_locally=true` and `routed_remotely=false` during degrade.
  - `.tmp/m039-s04/verify/08-post-rejoin/post-rejoin-node-a-membership.json` restores two-node membership, and `.tmp/m039-s04/verify/08-post-rejoin/post-rejoin-work.json` restores remote execution.

- ✅ **A canonical one-image local operator path and docs-truth surface exist for the M039 cluster story.**
  - `bash scripts/verify-m039-s04.sh` built `cluster-proof/Dockerfile` and proved the full continuity story from one repo-root image.
  - `bash scripts/verify-m039-s04-proof-surface.sh` passed, confirming README, distributed guide, distributed-proof page, sidebar, and runbook stay aligned to the canonical proof commands.
  - `npm --prefix website run build` passed, so the public docs surface renders successfully.
  - `bash -n scripts/verify-m039-s04-fly.sh` and `bash scripts/verify-m039-s04-fly.sh --help` passed, confirming the read-only Fly verifier contract is shipped and callable.
  - Attention note, not closeout failure: a fresh live Fly artifact was not capturable here because `fly status -a mesh-cluster-proof --json` failed with `Could not find App "mesh-cluster-proof"`. The in-repo Fly contract is delivered; live evidence remains an environment follow-up, not an implementation gap.

## Code-Change Verification

- ✅ Non-`.gsd` code changes exist. Using the repo’s closeout baseline rule, `git diff --stat "$(git merge-base HEAD origin/main)" HEAD -- ':!.gsd/'` shows real changes in `cluster-proof/`, `compiler/mesh-rt/`, `compiler/meshc/tests/e2e_m039_*`, `scripts/verify-m039-s0*.sh`, docs surfaces, and Docker/Fly packaging.

## Definition of Done Results

## Definition of Done Verification

- ✅ **All roadmap slices are complete.** `S01`, `S02`, `S03`, and `S04` are all present and marked complete in `.gsd/milestones/M039/slices/*`.
- ✅ **All slice summaries exist.** `find .gsd/milestones/M039/slices -maxdepth 2 -name 'S*-SUMMARY.md'` returned all four slice summaries.
- ✅ **Slice UAT artifacts exist.** `find .gsd/milestones/M039/slices -maxdepth 2 -name 'S*-UAT.md'` returned `S01-UAT.md` through `S04-UAT.md`.
- ✅ **Cross-slice integration holds.**
  - `.tmp/m039-s04/verify/01-s03-phase-report.txt` shows S04 replays S03 successfully.
  - That S03 replay, in turn, shows `s01-contract` and `s02-contract` passed before `s03-degrade` and `s03-rejoin`.
  - This proves S04 wrapped the previously validated discovery, membership, routing, degrade, and rejoin contracts instead of bypassing them.
- ✅ **Operator/docs surfaces integrate with runtime proof instead of drifting from it.** `bash scripts/verify-m039-s04-proof-surface.sh` and `npm --prefix website run build` both passed.
- ✅ **No roadmap horizontal-checklist failure surfaced.** The rendered roadmap in this worktree contains the slice overview only; no separate horizontal checklist items were present to audit.

## Requirement Outcomes

## Requirement Status Outcomes

| Requirement | Transition | Evidence |
| --- | --- | --- |
| R045 | active → validated | S01 established the runtime discovery seam; milestone closeout re-proved it via `bash scripts/verify-m039-s04.sh`, `.tmp/m039-s04/verify/05-dns-preflight/`, and `.tmp/m039-s04/verify/06-pre-loss/pre-loss-node-a-membership.json`, which show two nodes discovering each other from a shared DNS seed without manual peer lists. |
| R046 | active → validated | S01 proved truthful membership from live runtime sessions; S03 and S04 extended that proof through loss and rejoin. `.tmp/m039-s04/verify/07-degraded/degraded-node-a-membership.json` shows self-only shrinkage after node loss, and `.tmp/m039-s04/verify/08-post-rejoin/post-rejoin-node-a-membership.json` shows membership truth restored after same-identity rejoin. |
| R047 | active → validated | S02 validated runtime-native balancing, and milestone closeout re-proved it through `.tmp/m039-s04/verify/06-pre-loss/pre-loss-work.json` and `.tmp/m039-s04/verify/08-post-rejoin/post-rejoin-work.json`, where ingress and execution nodes differ and `routed_remotely=true`. |
| R048 | active → validated | `bash scripts/verify-m039-s03.sh` validated degrade/rejoin continuity, and `bash scripts/verify-m039-s04.sh` re-proved the full sequence from one image: degraded self-only membership plus local `/work` fallback at `.tmp/m039-s04/verify/07-degraded/*`, then restored two-node membership and remote routing at `.tmp/m039-s04/verify/08-post-rejoin/*`. |
| R053 | active → validated | `bash scripts/verify-m039-s04-proof-surface.sh`, `npm --prefix website run build`, `cluster-proof/README.md`, and `website/docs/docs/distributed-proof/index.md` now mechanically tie public distributed claims to the canonical local verifier/doc/runbook surfaces. |
| R052 | remains active | The repo now contains the one-image local proof, Fly config, and read-only Fly verifier contract, but closeout could not capture fresh live Fly evidence because the canonical app was not deployed in this environment (`Could not find App "mesh-cluster-proof"`). |

No M039 requirement was invalidated or re-scoped during closeout.

## Deviations

- The S04 operator-path closeout repaired the Docker builder path to an Ubuntu 22.04 + LLVM 21 builder on the local arm64 host; the earlier tarball-based builder was not the stable final contract.
- Live Fly verification could not be re-proved during milestone closeout because the canonical `mesh-cluster-proof` app did not exist in this environment. The shipped Fly contract was verified through help/syntax/docs surfaces instead, so R052 remains active even though the local one-image operator path is complete.

## Follow-ups

- Capture one durable live `.tmp/m039-s04/fly/` artifact bundle by running `CLUSTER_PROOF_FLY_APP=<existing-app> CLUSTER_PROOF_BASE_URL=https://<app>.fly.dev bash scripts/verify-m039-s04-fly.sh` against a real deployed app.
- Carry the ingress-owned request-correlation and fail-closed verifier patterns into M040’s replicated continuity work instead of inventing a new proof style.
- Keep R052 active until the same-image Fly path is evidenced against a real deployment, not just documented and syntax-checked.
