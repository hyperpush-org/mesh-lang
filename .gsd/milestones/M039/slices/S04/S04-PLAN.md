# S04: One-Image Operator Path, Local/Fly Verifiers, and Docs Truth

**Goal:** Turn the existing `cluster-proof` runtime contract into one real operator path: one repo-root Docker image, one shared-seed/shared-cookie local verifier, one read-only Fly verifier/runbook, and one public distributed proof surface that only claims what those exact commands prove.
**Demo:** After this: After this: one Docker image, one env-driven operator contract, and one canonical verifier/doc path prove the M039 cluster story locally and on Fly.

## Tasks
- [x] **T01: Added the repo-root `cluster-proof` Dockerfile, entrypoint contract guard, and Fly config, but Docker verification is still blocked by a host-side base-image metadata timeout.** — Add the packaging seam that S04 depends on: one repo-root Docker build for `cluster-proof`, a runtime entrypoint that supplies local hostname-based identity defaults only when appropriate, and a Fly config that matches the same image/operator contract.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| repo-root Docker build and Mesh compiler outputs | stop the task, keep the build log, and treat missing runtime artifacts as a packaging failure | fail the task with the last build output rather than retrying blindly | reject partial builds or missing binaries instead of papering over them with fallback shell logic |
| runtime env contract in the entrypoint | exit before starting the binary and surface the missing/invalid contract clearly | N/A — contract resolution is synchronous | reject conflicting identity inputs and preserve the existing `cluster-proof` validation semantics |
| Fly app config | fail the task on mismatched ports, auto-stop settings, or wrong build path assumptions | N/A — config validation is static | reject docs/config drift that implies `cd cluster-proof && fly deploy` works when the build still needs repo-root context |

## Load Profile

- **Shared resources**: Docker layer cache, Cargo build artifacts, and the runtime image filesystem.
- **Per-operation cost**: one repo-root image build plus short container-entrypoint smoke checks.
- **10x breakpoint**: build time and image size grow before CPU; the task should keep a small runtime stage and avoid copying the whole repo into the final image.

## Negative Tests

- **Malformed inputs**: missing cookie, missing seed, incomplete explicit identity, and partial Fly identity must still fail with clear config errors.
- **Error paths**: repo-root build-context mistakes, missing runtime binary in the final image, or conflicting identity precedence must fail before a misleading startup.
- **Boundary conditions**: local hostname fallback should only apply when neither explicit local identity nor Fly identity is set.

## Steps

1. Add `cluster-proof/Dockerfile` as a repo-root build-context image that compiles the `cluster-proof` package and copies only the runtime artifact plus the entrypoint into the final stage.
2. Add `cluster-proof/docker-entrypoint.sh` so local containers default `CLUSTER_PROOF_NODE_BASENAME` and `CLUSTER_PROOF_ADVERTISE_HOST` from `HOSTNAME` only when explicit local identity and Fly identity are both absent, then `exec` the packaged binary.
3. Add `cluster-proof/fly.toml` for the same image, pin the HTTP port, and keep machines awake so `.internal` DNS can return running peers truthfully.
4. Adjust root-level ignore/build-path details only if the repo-root Docker build needs them, without widening the app/runtime config surface.

## Must-Haves

- [ ] `docker build -f cluster-proof/Dockerfile .` works from repo root and produces a runtime image for `cluster-proof`.
- [ ] The image wrapper keeps the current `cluster-proof` config contract intact while supplying local container hostname defaults when no other identity source exists.
- [ ] `cluster-proof/fly.toml` matches the one-image contract and assumes repo-root `fly deploy . --config cluster-proof/fly.toml --dockerfile cluster-proof/Dockerfile` usage rather than a fake subdirectory build context.
  - Estimate: 90m
  - Files: cluster-proof/Dockerfile, cluster-proof/docker-entrypoint.sh, cluster-proof/fly.toml, .dockerignore
  - Verify: docker build -f cluster-proof/Dockerfile -t mesh-cluster-proof .
docker run --rm --entrypoint /bin/sh mesh-cluster-proof -lc 'test -x /usr/local/bin/cluster-proof && test -x /usr/local/bin/docker-entrypoint.sh'
- [x] **T02: Assemble the local one-image continuity verifier** — Add the authoritative local S04 acceptance surface so the one-image operator story is proven against real containers, not inferred from the earlier process-level harnesses.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `scripts/verify-m039-s03.sh` prerequisite replay | stop immediately and refuse to claim S04 passed on a broken local runtime contract | fail the phase with the copied prerequisite log and current-phase marker | treat missing phase reports or zero-test evidence as verifier drift, not as a flaky prerequisite |
| Docker network/DNS alias preflight | fail closed with the container/network inspection output and preserve the partial artifact dir | tear down containers, preserve the preflight logs, and mark the phase failed | reject seed-resolution evidence that does not show both containers behind the shared alias |
| container `/membership` and `/work` probes | preserve raw JSON bodies and container logs, then fail the phase | kill the bounded wait, keep the partial evidence, and fail with the last raw body or curl error | reject missing routing/membership fields instead of synthesizing cluster truth |

## Load Profile

- **Shared resources**: two running containers, one user-defined Docker bridge network, copied container stdout/stderr logs, and `.tmp/m039-s04/verify/` artifact directories.
- **Per-operation cost**: one prerequisite verifier replay, one image build, repeated membership polls, and three `/work` probes across pre-loss, degraded, and post-rejoin phases.
- **10x breakpoint**: wall-clock time and artifact sprawl before CPU; the wrapper must bound waits and clean up containers deterministically.

## Negative Tests

- **Malformed inputs**: missing membership keys, missing `request_id`/`execution_node`, or partial DNS preflight output must fail the wrapper with the raw artifact preserved.
- **Error paths**: bad shared seed, bad cookie, container crash, or missing logs must fail the phase instead of leaving hanging containers behind.
- **Boundary conditions**: two-node convergence, truthful self-only degrade, same-identity restart, and restored remote routing from the same image.

## Steps

1. Add a small helper library if needed for repeated JSON assertions, container cleanup, or artifact copying so the local and Fly verifiers can share the same contract checks.
2. Implement `scripts/verify-m039-s04.sh` to replay `scripts/verify-m039-s03.sh`, build the image from repo root, create one Docker bridge network, start two containers from the same image with one shared discovery alias/seed and one shared cookie, and preflight that the shared alias resolves to both containers before trusting the cluster proof.
3. Probe `/membership` and `/work` to prove two-node membership, remote routing, truthful degrade after stopping one container, truthful local fallback during degrade, same-identity restart, and restored remote routing after rejoin; archive JSON bodies, manifests, and container logs under `.tmp/m039-s04/verify/`.
4. Fail closed on zero-test prerequisite drift, DNS preflight drift, malformed JSON, or missing copied logs, and finish only when `bash scripts/verify-m039-s04.sh` passes from a clean run.

## Must-Haves

- [ ] The local verifier uses the same image for both containers and proves clustering from only a shared cookie plus shared discovery seed — no manual peer lists and no per-node bootstrap logic.
- [ ] `.tmp/m039-s04/verify/` preserves phase-by-phase logs, manifests, membership/work JSON, and copied container stdout/stderr for pre-loss, degraded, and post-rejoin debugging.
- [ ] A passing wrapper proves the full local chain: S03 replay, repo-root image build, two-container convergence, remote `/work`, degrade/local fallback, same-identity restart, and restored remote `/work`.
  - Estimate: 2h
  - Files: scripts/verify-m039-s04.sh, scripts/lib/m039_cluster_proof.sh, cluster-proof/Dockerfile, cluster-proof/docker-entrypoint.sh
  - Verify: bash scripts/verify-m039-s04.sh
- [x] **T03: Added the cluster-proof Fly runbook and fail-closed read-only verifier, and pinned the missing Fly discovery-seed config.** — Add the operator-facing Fly surface without mutating external state: a package-local runbook that spells out the exact repo-root build/deploy commands and a read-only verifier that inspects an existing Fly deployment for the same cluster contract the local Docker path proves.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Fly CLI status/config/log inspection | fail the task with the exact missing-auth or missing-app context and do not attempt any state-changing fallback | fail the phase with the partial Fly output preserved under `.tmp/m039-s04/fly/` | reject partial or inconsistent machine/config output instead of guessing cluster truth |
| live `/membership` and `/work` probes against the Fly app | preserve the raw response bodies and fail closed with the URL/app context | fail with the last curl error or HTTP status after the bounded wait | reject missing membership/routing fields or self-only responses when the app is supposed to be clustered |
| runbook/operator commands | fail docs-verifier grep checks later rather than leaving an ambiguous operator story | N/A — static content | reject commands that imply `cd cluster-proof` build context or mutate Fly state without approval |

## Load Profile

- **Shared resources**: Fly API/log queries, one already-deployed app with multiple running machines, and the verifier artifact dir.
- **Per-operation cost**: a small number of `fly status` / `fly config show` / `fly logs` calls plus `/membership` and `/work` probes.
- **10x breakpoint**: CLI/log volume and slow remote app convergence before CPU; the verifier should bound waits and keep copied excerpts small and focused.

## Negative Tests

- **Malformed inputs**: missing `CLUSTER_PROOF_FLY_APP`, missing `CLUSTER_PROOF_BASE_URL`, or inconsistent app/URL pairing must fail before any proof attempt.
- **Error paths**: Fly auth failure, fewer than the required running machines, or stale auto-stop configuration must fail the verifier instead of downgrading to warnings.
- **Boundary conditions**: at least two running machines, truthful membership including peers, and `/work` showing remote routing in the live cluster.

## Steps

1. Write `cluster-proof/README.md` as the canonical operator runbook for the image, including repo-root Docker build, local two-container usage, the exact `fly deploy . --config cluster-proof/fly.toml --dockerfile cluster-proof/Dockerfile` command, and the read-only Fly verification path.
2. Implement `scripts/verify-m039-s04-fly.sh` so it requires an existing deployed app and base URL, uses only read-only Fly commands (`fly status`, `fly config show`, `fly logs`) plus `/membership` and `/work` probes, and archives evidence under `.tmp/m039-s04/fly/`.
3. Provide a non-live usage/help path so the script can be syntax-checked in-repo, but keep the real verification mode fail-closed when auth, machine count, config, or cluster truth drift.
4. Finish only when the runbook and verifier agree on the exact commands, required inputs, and read-only constraint.

## Must-Haves

- [ ] `cluster-proof/README.md` is the deepest operator runbook for the one-image local/Fly path and uses the exact repo-root build/deploy commands.
- [ ] `scripts/verify-m039-s04-fly.sh` is read-only, requires an existing Fly app, and checks both Fly machine/config truth and the live `/membership` + `/work` contract.
- [ ] The in-repo verification path can check script syntax/help without live Fly access, while the live mode still fail-closes on real drift.
  - Estimate: 90m
  - Files: cluster-proof/README.md, scripts/verify-m039-s04-fly.sh, scripts/lib/m039_cluster_proof.sh, cluster-proof/fly.toml
  - Verify: bash -n scripts/verify-m039-s04-fly.sh
bash scripts/verify-m039-s04-fly.sh --help
rg -q "fly deploy \\. --config cluster-proof/fly.toml --dockerfile cluster-proof/Dockerfile" cluster-proof/README.md
- [x] **T04: Published the Distributed Proof page, rerouted operator-facing docs to it, and added a fail-closed proof-surface verifier.** — Reconcile the public docs surface so distributed/operator claims point at concrete commands and proof artifacts instead of leaving readers in the generic `Node.start` / `Node.connect` tutorial path.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| VitePress docs build | fail the task with the build log and stop before claiming docs truth | fail with the bounded build log rather than skipping the site build | reject broken links/frontmatter/sidebar config rather than relying on unchecked markdown |
| docs-truth verifier script | fail closed on missing commands, links, or stale phrases across the runbook/README/proof page/sidebar | N/A — static content | reject partial matches that do not prove the same canonical commands are shared |

## Load Profile

- **Shared resources**: one VitePress build and one repo-local grep/contract verifier.
- **Per-operation cost**: static markdown/config updates plus one site build.
- **10x breakpoint**: docs drift and stale-command sprawl before compute cost; the verifier should centralize the canonical command list.

## Negative Tests

- **Malformed inputs**: missing proof-page frontmatter, missing sidebar link, or missing runbook/proof links must fail the verifier.
- **Error paths**: stale generic distributed claims, stale README links, or docs that mention commands not present in the real runbook/scripts must fail closed.
- **Boundary conditions**: the generic distributed guide still teaches primitives, but all operator-proof claims route to the new distributed-proof page and `cluster-proof/README.md`.

## Steps

1. Add `website/docs/docs/distributed-proof/index.md` mirroring the production-backend-proof pattern, with the exact local verifier, Fly verifier, runbook, and contract summary for S04.
2. Update `website/docs/docs/distributed/index.md`, `website/docs/.vitepress/config.mts`, and `README.md` so generic distributed guidance routes operator claims to the proof page and runbook instead of implying the tutorial path is the proof surface.
3. Add `scripts/verify-m039-s04-proof-surface.sh` to assert the README, proof page, sidebar, generic distributed page, and `cluster-proof/README.md` share the same canonical commands and links while rejecting stale operator wording.
4. Finish only when the docs build passes and the docs-truth verifier passes from repo root.

## Must-Haves

- [ ] The public distributed proof page points at the exact S04 local/Fly verifier commands and the `cluster-proof/README.md` runbook.
- [ ] `README.md`, the sidebar, and the generic distributed guide route readers to the proof surface instead of leaving operator claims in the tutorial page.
- [ ] `scripts/verify-m039-s04-proof-surface.sh` fail-closes on docs drift and becomes the named verifier for R053.
  - Estimate: 75m
  - Files: website/docs/docs/distributed-proof/index.md, website/docs/docs/distributed/index.md, website/docs/.vitepress/config.mts, README.md, scripts/verify-m039-s04-proof-surface.sh
  - Verify: npm --prefix website run build
bash scripts/verify-m039-s04-proof-surface.sh
