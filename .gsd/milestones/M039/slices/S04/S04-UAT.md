# S04: One-Image Operator Path, Local/Fly Verifiers, and Docs Truth — UAT

**Milestone:** M039
**Written:** 2026-03-28T16:26:41.284Z

# S04 UAT — One-Image Operator Path, Local/Fly Verifiers, and Docs Truth

## Preconditions
- Run from the repository root.
- Docker Desktop / local Docker engine is running.
- `npm --prefix website` dependencies are already installed.
- `flyctl`, `curl`, and `python3` are installed for the Fly verifier help/contract path.
- Optional live Fly proof requires an existing `mesh-cluster-proof` app with `CLUSTER_PROOF_COOKIE` configured and at least two running machines.

## Test Case 1 — Repo-root image contract builds and packages the runtime
1. Run `docker build -f cluster-proof/Dockerfile -t mesh-cluster-proof .`.
   - **Expected:** The build succeeds from the repo root without `cd cluster-proof`, and the final image is tagged `mesh-cluster-proof`.
2. Run `docker run --rm --entrypoint /bin/sh mesh-cluster-proof -lc 'test -x /usr/local/bin/cluster-proof && test -x /usr/local/bin/docker-entrypoint.sh'`.
   - **Expected:** The command exits 0, proving the runtime binary and entrypoint are present and executable in the final image.
3. Optional negative check: run `docker run --rm -e CLUSTER_PROOF_COOKIE=test-cookie mesh-cluster-proof`.
   - **Expected:** The container exits non-zero with a clear config error because `MESH_DISCOVERY_SEED` is missing.

## Test Case 2 — Local one-image continuity verifier proves the full Docker operator story
1. Run `bash scripts/verify-m039-s04.sh`.
   - **Expected:** The script exits 0 and prints `verify-m039-s04: ok`.
2. Open `.tmp/m039-s04/verify/phase-report.txt`.
   - **Expected:** It records `passed` for `s03-contract`, `docker-image-build`, `docker-network`, `docker-start`, `dns-preflight`, `convergence`, `degrade`, and `rejoin`.
3. Open `.tmp/m039-s04/verify/05-dns-preflight/node-a-seed-resolution.txt` and `node-b-seed-resolution.txt`.
   - **Expected:** Both files show the shared alias resolving to both container IPs.
4. Open `.tmp/m039-s04/verify/06-pre-loss/pre-loss-work.json`.
   - **Expected:** The JSON shows `routed_remotely=true`, `fell_back_locally=false`, and distinct ingress/target/execution nodes.
5. Open `.tmp/m039-s04/verify/07-degraded/degraded-work.json`.
   - **Expected:** The JSON shows `routed_remotely=false`, `fell_back_locally=true`, and ingress/target/execution all equal to the surviving node.
6. Open `.tmp/m039-s04/verify/08-post-rejoin/post-rejoin-work.json`.
   - **Expected:** The JSON shows remote routing restored after the restarted node rejoins.
7. Confirm `.tmp/m039-s04/verify/06-pre-loss/manifest.txt`, `07-degraded/manifest.txt`, and `08-post-rejoin/manifest.txt` exist.
   - **Expected:** Each manifest names the copied stdout/stderr logs and JSON artifacts needed for postmortem debugging.

## Test Case 3 — Fly verifier help path and contract surface are truthful without mutating Fly
1. Run `bash -n scripts/verify-m039-s04-fly.sh`.
   - **Expected:** The script passes syntax check.
2. Run `bash scripts/verify-m039-s04-fly.sh --help`.
   - **Expected:** The help text shows the exact required env vars (`CLUSTER_PROOF_FLY_APP`, `CLUSTER_PROOF_BASE_URL`), states the verifier is read-only, and names `.tmp/m039-s04/fly/` as the artifact root.
3. Run `CLUSTER_PROOF_BASE_URL=https://mesh-cluster-proof.fly.dev bash scripts/verify-m039-s04-fly.sh`.
   - **Expected:** The script fails closed before any proof attempt because `CLUSTER_PROOF_FLY_APP` is missing.
4. Run `CLUSTER_PROOF_FLY_APP=mesh-cluster-proof CLUSTER_PROOF_BASE_URL=https://wrong-app.fly.dev bash scripts/verify-m039-s04-fly.sh`.
   - **Expected:** The script fails closed because the base URL does not match the Fly app hostname.

## Test Case 4 — Optional live Fly proof against an existing deployment
1. Ensure an existing `mesh-cluster-proof` app is deployed with at least two running machines and `CLUSTER_PROOF_COOKIE` configured.
2. Run `CLUSTER_PROOF_FLY_APP=mesh-cluster-proof CLUSTER_PROOF_BASE_URL=https://mesh-cluster-proof.fly.dev bash scripts/verify-m039-s04-fly.sh`.
   - **Expected:** The script exits 0, `.tmp/m039-s04/fly/phase-report.txt` records `passed` for `preflight`, `input-validation`, `fly-status`, `fly-config`, `membership-probe`, `work-probe`, and `fly-logs`, and the captured JSON/log artifacts show at least two running machines plus remote `/work` routing in the live deployment.

## Test Case 5 — Public docs route operator claims to the canonical proof surface
1. Run `npm --prefix website run build`.
   - **Expected:** The docs build completes successfully.
2. Run `bash scripts/verify-m039-s04-proof-surface.sh`.
   - **Expected:** The script exits 0 after confirming the README, Distributed Proof page, generic distributed guide, sidebar, and runbook all share the same canonical commands and links.
3. Open `website/docs/docs/distributed-proof/index.md`.
   - **Expected:** It names `docker build -f cluster-proof/Dockerfile -t mesh-cluster-proof .`, `bash scripts/verify-m039-s04.sh`, `fly deploy . --config cluster-proof/fly.toml --dockerfile cluster-proof/Dockerfile`, `bash scripts/verify-m039-s04-fly.sh`, and `bash scripts/verify-m039-s04-proof-surface.sh`.
4. Open `README.md` and `website/docs/docs/distributed/index.md`.
   - **Expected:** Both route operator-proof readers to the Distributed Proof page and `cluster-proof/README.md` instead of implying the primitive tutorial page is the proof surface.

## Edge Cases / Fail-Closed Expectations
- Missing `MESH_DISCOVERY_SEED` or partial explicit identity in the container path must fail with a clear entrypoint config error instead of starting a misleading clustered runtime.
- If the Docker DNS alias resolves to fewer than two container IPs, `bash scripts/verify-m039-s04.sh` must fail before trusting membership or `/work` evidence.
- If README/proof page/sidebar commands drift, `bash scripts/verify-m039-s04-proof-surface.sh` must fail closed instead of tolerating stale public operator wording.
- If the Fly app/base URL pairing is wrong, `scripts/verify-m039-s04-fly.sh` must fail before any probe attempts.

