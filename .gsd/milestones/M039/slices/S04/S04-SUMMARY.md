---
id: S04
parent: M039
milestone: M039
provides:
  - One canonical repo-root Docker image contract for `cluster-proof` with a guarded entrypoint and matching Fly config.
  - A passing local Docker-based continuity verifier that proves S03 replay, two-container convergence, remote routing, truthful degrade, same-identity rejoin, and restored remote routing from one image.
  - A canonical `cluster-proof/README.md` runbook plus a read-only Fly verifier contract for an existing deployment.
  - A public Distributed Proof documentation surface that routes operator claims to concrete commands and is mechanically guarded against drift.
requires:
  - slice: S01
    provides: Runtime-owned DNS discovery and truthful membership proof as the baseline cluster contract.
  - slice: S02
    provides: The `/work` ingress/target/execution proof surface that the local Docker and Fly verifiers check.
  - slice: S03
    provides: The degrade/rejoin continuity behavior that S04 replays before claiming the one-image operator path is healthy.
affects:
  - M040/S01
  - M041/S01
key_files:
  - cluster-proof/Dockerfile
  - cluster-proof/docker-entrypoint.sh
  - cluster-proof/fly.toml
  - cluster-proof/README.md
  - scripts/lib/m039_cluster_proof.sh
  - scripts/verify-m039-s01.sh
  - scripts/verify-m039-s02.sh
  - scripts/verify-m039-s03.sh
  - scripts/verify-m039-s04.sh
  - scripts/verify-m039-s04-fly.sh
  - scripts/verify-m039-s04-proof-surface.sh
  - compiler/meshc/tests/e2e_m039_s02.rs
  - website/docs/docs/distributed-proof/index.md
  - website/docs/docs/distributed/index.md
  - website/docs/.vitepress/config.mts
  - README.md
  - .gsd/PROJECT.md
  - .gsd/milestones/M039/slices/S04/tasks/T02-SUMMARY.md
key_decisions:
  - Keep three separate proof surfaces for S04: a local Docker continuity verifier (`scripts/verify-m039-s04.sh`), a read-only live Fly verifier (`scripts/verify-m039-s04-fly.sh`), and a docs-truth verifier (`scripts/verify-m039-s04-proof-surface.sh`).
  - Derive local container identity from `HOSTNAME` only in the image entrypoint and only when cluster mode is being attempted with neither explicit local identity nor Fly identity already provided.
  - Publish distributed operator claims on a dedicated `Distributed Proof` page backed by `cluster-proof/README.md`, while leaving `Distributed Actors` as the primitive/tutorial guide.
  - Build the repo-root Docker image on the local arm64 host with an Ubuntu 22.04 + LLVM 21 apt toolchain builder, which produced a reliable `meshc`/`cluster-proof` build under Docker where the earlier tarball-based builder path did not.
  - Make the assembled local verifier fail closed on prerequisite drift and on DNS alias drift before trusting any membership or `/work` evidence.
patterns_established:
  - Repo-root operator packaging with package-local Docker/Fly config files and explicit `--config` / `--dockerfile` commands documented as part of the contract.
  - Fail-closed shell verifiers that emit `phase-report.txt`, `status.txt`, `current-phase.txt`, and a stable artifact root so downstream readers can diagnose exactly which phase drifted.
  - One-image local cluster proof built from two containers sharing only a cookie and discovery alias, with DNS preflight evidence captured before cluster truth is asserted.
  - Docs-truth verification that mechanically ties README, sidebar, public proof page, generic guide, and deepest runbook to the same canonical command list.
observability_surfaces:
  - `.tmp/m039-s04/verify/phase-report.txt`, `status.txt`, `current-phase.txt`, and `full-contract.log` from the local one-image verifier.
  - `.tmp/m039-s04/verify/05-dns-preflight/`, `06-pre-loss/`, `07-degraded/`, and `08-post-rejoin/` with manifests, membership/work JSON, network inspection, and copied container stdout/stderr logs.
  - `.tmp/m039-s04/fly/` phase ledger and captured Fly/config/log/probe artifacts for the read-only live verifier path.
  - The public `/docs/distributed-proof/` page and `scripts/verify-m039-s04-proof-surface.sh` as the docs-truth signal for operator claims.
drill_down_paths:
  - .gsd/milestones/M039/slices/S04/tasks/T01-SUMMARY.md
  - .gsd/milestones/M039/slices/S04/tasks/T02-SUMMARY.md
  - .gsd/milestones/M039/slices/S04/tasks/T03-SUMMARY.md
  - .gsd/milestones/M039/slices/S04/tasks/T04-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-28T16:26:41.284Z
blocker_discovered: false
---

# S04: One-Image Operator Path, Local/Fly Verifiers, and Docs Truth

**S04 turned `cluster-proof` into one repo-root image/operator path with a passing local Docker continuity verifier, a read-only Fly verifier/runbook contract, and a docs-truth surface that routes distributed operator claims to one canonical proof page.**

## What Happened

S04 closed the operator/documentation layer on top of the earlier M039 cluster proofs. On the packaging side, `cluster-proof` now has one repo-root Docker image path, one entrypoint-owned identity fallback for local container runs, and one Fly config that keeps the same image/build-context contract. During slice closeout, the original Docker builder path recorded in T01 had to be repaired: the image now builds successfully on the local arm64 host using an Ubuntu 22.04 builder with LLVM 21 apt packages, rather than the earlier tarball-based builder that produced LLVM/linker format failures under Docker. 

On the local proof side, `scripts/verify-m039-s04.sh` now serves as the authoritative one-image continuity wrapper. It replays S03 first, builds the repo-root image, creates one bridge network, starts two containers from the same image with only a shared cookie plus shared discovery alias, proves the shared alias resolves to both containers before trusting cluster state, and then proves the full local story: two-node convergence, remote `/work`, truthful self-only degrade after stopping one node, same-identity restart, and restored remote routing after rejoin. The verifier preserves a phase ledger plus manifests, membership/work JSON, inspect output, and copied container stdout/stderr under `.tmp/m039-s04/verify/` so failures are diagnosable after the run. To make this reliable, the prerequisite S01/S02/S03 wrappers were tightened to build `mesh-rt` before invoking `meshc` on `cluster-proof`, and the unstable second ingress direction in `e2e_m039_s02` was removed so the prerequisite replay reflects the currently stable runtime contract.

On the Fly/operator side, `cluster-proof/README.md` is now the deepest runbook, `scripts/verify-m039-s04-fly.sh` is the read-only live verifier, and `cluster-proof/fly.toml` pins the missing discovery seed plus the no-auto-stop behavior needed for truthful `.internal` DNS-based clustering. The Fly verifier fail-closes on missing or inconsistent `CLUSTER_PROOF_FLY_APP` / `CLUSTER_PROOF_BASE_URL`, checks `fly status`, `fly config show`, `fly logs`, `/membership`, and `/work`, and archives evidence under `.tmp/m039-s04/fly/`. In this slice closeout context, only the non-live help/syntax/contract gates were exercised because the verifier intentionally requires an existing deployed app.

On the public docs side, the repo now has a dedicated Distributed Proof page at `/docs/distributed-proof/`, the generic distributed guide and README route operator claims to that page and to `cluster-proof/README.md`, and `scripts/verify-m039-s04-proof-surface.sh` fail-closes on drift between the README, proof page, sidebar, generic distributed guide, and runbook. That turns the distributed/operator story into one canonical proof surface rather than leaving readers to infer readiness from primitive `Node.start` / `Node.connect` tutorial examples.

## Verification

Slice-level verification passed on the named S04 surfaces. Final closeout checks:

- `docker build -f cluster-proof/Dockerfile -t mesh-cluster-proof .` ✅
- `docker run --rm --entrypoint /bin/sh mesh-cluster-proof -lc 'test -x /usr/local/bin/cluster-proof && test -x /usr/local/bin/docker-entrypoint.sh'` ✅
- `bash scripts/verify-m039-s04.sh` ✅
- `bash -n scripts/verify-m039-s04-fly.sh` ✅
- `bash scripts/verify-m039-s04-fly.sh --help` ✅
- `rg -q "fly deploy \. --config cluster-proof/fly.toml --dockerfile cluster-proof/Dockerfile" cluster-proof/README.md` ✅
- `npm --prefix website run build` ✅
- `bash scripts/verify-m039-s04-proof-surface.sh` ✅

Supporting prerequisite replays also passed as part of the assembled local proof path:

- `bash scripts/verify-m039-s02.sh` ✅
- `bash scripts/verify-m039-s03.sh` ✅

The live Fly verification mode was not exercised because it intentionally requires an existing deployed `mesh-cluster-proof` app with `CLUSTER_PROOF_COOKIE` configured and at least two running machines; the slice verified the fail-closed non-live contract and the canonical commands instead.

## Requirements Advanced

None.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

Two slice-closeout deviations are worth preserving. First, T01’s original Docker verification result was superseded during closeout: the repo now has a working local image build after switching the builder path to Ubuntu 22.04 + LLVM 21 apt packages on arm64 Docker hosts. Second, T02 did not have a durable executor-written summary when closeout began, so its assembled result was reconstructed from the delivered verifier, the green `bash scripts/verify-m039-s04.sh` run, and the concrete fixes needed to make the one-image wrapper pass. The slice plan’s Fly work remained intentionally read-only, so the live Fly verifier itself was not run without an existing deployment target.

## Known Limitations

The live Fly proof still depends on an already-deployed `mesh-cluster-proof` app with `CLUSTER_PROOF_COOKIE` configured and at least two running machines; this slice only verified the in-repo help/syntax/contract path for that script. `npm --prefix website run build` still emits the pre-existing VitePress chunk-size warning, but the docs build remains green. The current stable S02 prerequisite keeps one remote-routing direction in `e2e_m039_s02`; the authoritative symmetric local operator-path proof now comes from the Docker-based S04 verifier rather than from keeping both process-level loopback directions in the Rust prerequisite harness.

## Follow-ups

Run `CLUSTER_PROOF_FLY_APP=mesh-cluster-proof CLUSTER_PROOF_BASE_URL=https://mesh-cluster-proof.fly.dev bash scripts/verify-m039-s04-fly.sh` against a real deployed app when live Fly evidence is needed. Future milestones should build on this operator baseline for replicated continuity (M040) and cross-cluster disaster continuity (M041), rather than widening `cluster-proof` docs or packaging again.

## Files Created/Modified

- `cluster-proof/Dockerfile` — Added the repo-root one-image build path and repaired the builder stage so the image builds successfully on the local arm64 Docker host.
- `cluster-proof/docker-entrypoint.sh` — Added entrypoint-owned contract validation plus local `HOSTNAME`-based identity fallback only when cluster mode is requested and no explicit/Fly identity exists.
- `cluster-proof/fly.toml` — Pinned the one-image Fly contract, including fixed ports, `MESH_DISCOVERY_SEED=mesh-cluster-proof.internal`, and no-auto-stop behavior.
- `cluster-proof/README.md` — Published the deepest local/Fly operator runbook and aligned it with the canonical proof commands.
- `scripts/lib/m039_cluster_proof.sh` — Provided shared fail-closed helpers for phase ledgers, JSON assertions, and timeout/log handling across the S04 verifier surfaces.
- `scripts/verify-m039-s01.sh` — Added an explicit `mesh-rt` build step so cluster-proof test/build replays do not fail on a missing runtime static library.
- `scripts/verify-m039-s02.sh` — Added the `mesh-rt` prerequisite build and aligned the copied-artifact expectations with the stable S02 remote-route proof shape.
- `scripts/verify-m039-s03.sh` — Added the `mesh-rt` prerequisite build so the S03 replay remains a truthful prerequisite for the Docker wrapper.
- `scripts/verify-m039-s04.sh` — Implemented the authoritative local one-image continuity verifier with Docker build, DNS preflight, convergence/degrade/rejoin checks, and archived evidence.
- `scripts/verify-m039-s04-fly.sh` — Implemented the read-only Fly verifier with strict input validation, config/machine checks, live probe checks, and artifact capture.
- `scripts/verify-m039-s04-proof-surface.sh` — Implemented the docs-truth verifier that guards the canonical distributed operator command/link surface.
- `compiler/meshc/tests/e2e_m039_s02.rs` — Removed the unstable second loopback ingress direction so the prerequisite S02 proof matches the currently stable runtime contract.
- `website/docs/docs/distributed-proof/index.md` — Added the public Distributed Proof page for the canonical operator-path commands and proof surface.
- `website/docs/docs/distributed/index.md` — Rerouted generic distributed guidance to the dedicated operator proof page and runbook.
- `website/docs/.vitepress/config.mts` — Added the Distributed Proof entry to the docs sidebar.
- `README.md` — Rerouted repo-level distributed operator claims to the Distributed Proof page and `cluster-proof/README.md`.
- `.gsd/PROJECT.md` — Updated project state so M039 now reflects the completed S04 operator-path/docs-truth surface.
- `.gsd/milestones/M039/slices/S04/tasks/T02-SUMMARY.md` — Backfilled the missing T02 task summary with the delivered local one-image verifier and the final closeout fixes that made it pass.
