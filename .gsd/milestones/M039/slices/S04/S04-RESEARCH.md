# S04 Research — One-Image Operator Path, Local/Fly Verifiers, and Docs Truth

## Summary

S04 is **targeted packaging/operator-path work**, not another runtime slice.

S01–S03 already prove the actual cluster behavior locally:
- runtime-owned DNS discovery
- truthful `/membership`
- runtime-native `/work` routing
- safe degrade after peer loss
- same-identity rejoin and remote-routing recovery

What is still missing is the **operator path and public truth surface** around that proof:
- there is **no `cluster-proof` Dockerfile**
- there is **no `cluster-proof` Fly config**
- there is **no `cluster-proof/README.md` runbook**
- there is **no S04 local/Fly verifier wrapper**
- public distributed docs are still mostly manual `Node.start` / `Node.connect` tutorials and do not route readers to a canonical, replayable proof surface

So the slice should stay narrow: **package the existing proof app as one image, add a local one-image verifier, add a Fly runbook/read-only verifier, and reconcile docs/README to those exact proof surfaces**.

Do **not** reopen `mesh-rt` or add new proof endpoints unless the packaging path exposes a real defect. The existing `/membership` + `/work` contract is already the honest proof surface.

## Requirements Focus

### R052 — One image + small operator surface
Current gap:
- `cluster-proof/config.mpl` already supports Fly identity (`FLY_APP_NAME`, `FLY_REGION`, `FLY_MACHINE_ID`, `FLY_PRIVATE_IP`) and local explicit identity (`CLUSTER_PROOF_NODE_BASENAME`, `CLUSTER_PROOF_ADVERTISE_HOST`), but there is no image/runbook/verifier that turns that into a real operator flow.
- Local Docker use would still be noisy if every container has to be hand-wired with explicit identity env.

What S04 needs to prove:
- same image locally and on Fly
- shared secret + shared discovery seed
- no manual peer lists
- no bespoke per-node bootstrap logic

### R053 — Docs truth
Current gap:
- `website/docs/docs/distributed/index.md` is still a generic API guide centered on `Node.start(...)` and `Node.connect(...)`
- `README.md` has no canonical distributed proof link
- there is no distributed-proof page / runbook / docs-truth verifier analogous to the backend proof surface

What S04 needs to prove:
- public distributed claims point at concrete commands, runbooks, and verifier scripts
- docs only claim what the `cluster-proof` image + local/Fly verification path actually proves

## Skills Discovered

- **Existing skill: `flyio-cli-public`**
  - Relevant rule: prefer read-only Fly operations first (`fly status`, `fly logs`, `fly config show`)
  - Relevant rule: do **not** run state-changing Fly actions (`fly deploy`, scale, secrets changes, app creation/destruction) without explicit user approval
- **Existing skill: `vitepress`**
  - Relevant rule: docs changes should be validated through the actual VitePress site build (`npm --prefix website run build`)
  - Relevant files: `website/package.json`, `website/docs/.vitepress/config.mts`
- **Installed skill: `multi-stage-dockerfile`**
  - Installed from `github/awesome-copilot@multi-stage-dockerfile`
  - Relevant rule: builder/runtime split, minimal runtime stage, exact base images, copy only final artifacts, prefer non-root runtime, rely on `.dockerignore`

## Implementation Landscape

### Existing proof app files

- `cluster-proof/config.mpl`
  - owns the operator env contract
  - current keys:
    - `PORT` (default 8080)
    - `MESH_CLUSTER_PORT` (default 4370)
    - `CLUSTER_PROOF_COOKIE`
    - `MESH_DISCOVERY_SEED`
    - local explicit identity: `CLUSTER_PROOF_NODE_BASENAME` + `CLUSTER_PROOF_ADVERTISE_HOST`
    - Fly identity: `FLY_APP_NAME`, `FLY_REGION`, `FLY_MACHINE_ID`, `FLY_PRIVATE_IP`
  - optional runtime knob exists in `mesh-rt`: `MESH_DISCOVERY_INTERVAL_MS`
  - important: there is **already** a clean Fly identity path; no runtime change is needed for that

- `cluster-proof/main.mpl`
  - starts the HTTP server
  - exposes only:
    - `GET /membership`
    - `GET /work`
  - logs the startup contract without echoing the cookie

- `cluster-proof/cluster.mpl`
  - `/membership` payload shape is already stable
  - fields:
    - `mode`
    - `self`
    - `peers`
    - `membership`
    - `http_port`
    - `cluster_port`
    - `discovery_provider`
    - `discovery_seed`

- `cluster-proof/work.mpl`
  - `/work` payload shape is already stable
  - fields:
    - `ok`
    - `request_id`
    - `ingress_node`
    - `target_node`
    - `execution_node`
    - `routed_remotely`
    - `fell_back_locally`
    - `timed_out`
    - `error`
  - stable observability surfaces already exist:
    - ingress log: `[cluster-proof] work dispatched request_id=... ingress=... target=... routed_remotely=...`
    - execution log: `[cluster-proof] work executed execution=...`

- `cluster-proof/tests/config.test.mpl`
  - already proves Fly node-name composition and explicit-identity validation

- `cluster-proof/tests/work.test.mpl`
  - already proves local work-routing helper logic

### Existing local verifier chain

- `scripts/verify-m039-s01.sh`
  - discovery + membership baseline
- `scripts/verify-m039-s02.sh`
  - routing proof; replays S01 first
- `scripts/verify-m039-s03.sh`
  - degrade/rejoin continuity proof; replays S01 and S02 first
  - preserves stable manifests and artifacts under `.tmp/m039-s03/verify/`

These are the current canonical proof surfaces. S04 should **wrap them**, not replace them.

### Existing local Rust harnesses

- `compiler/meshc/tests/e2e_m039_s01.rs`
- `compiler/meshc/tests/e2e_m039_s02.rs`
- `compiler/meshc/tests/e2e_m039_s03.rs`

Useful findings for planning:
- all three spawn `cluster-proof` by setting:
  - `PORT`
  - `MESH_CLUSTER_PORT`
  - `CLUSTER_PROOF_COOKIE`
  - `MESH_DISCOVERY_SEED`
  - `CLUSTER_PROOF_NODE_BASENAME`
  - `CLUSTER_PROOF_ADVERTISE_HOST`
- artifact naming is already canonical (`*-membership.json`, `*-work.json`, `*.stdout.log`, `*.stderr.log`)
- these harnesses prove the runtime path; S04 does not need a new distributed runtime harness unless the Docker path becomes too brittle for shell-only verification

### Existing packaging/deploy patterns worth copying

- `registry/Dockerfile` + `registry/fly.toml`
  - repo-root Rust multi-stage build pattern
- `packages-website/Dockerfile` + `packages-website/fly.toml`
  - smaller service Docker + Fly config pattern
- root `.dockerignore`
  - already excludes `**/target/`, `.git/`, `node_modules/`, logs, etc.

### Existing docs-truth pattern worth copying exactly

The repo already has one strong pattern for “public proof surface + runbook + docs verifier”:
- `reference-backend/README.md`
- `website/docs/docs/production-backend-proof/index.md`
- `reference-backend/scripts/verify-production-proof-surface.sh`

That is the cleanest model for S04.

### Current gaps (confirmed)

Missing today:
- `cluster-proof/Dockerfile`
- `cluster-proof/fly.toml`
- `cluster-proof/README.md`
- any S04 verifier script
- any distributed-proof page
- any docs-truth verifier for distributed claims

Current docs drift:
- `website/docs/docs/distributed/index.md` is still an API tutorial and does not mention:
  - `cluster-proof`
  - `scripts/verify-m039-s01.sh`
  - `scripts/verify-m039-s02.sh`
  - `scripts/verify-m039-s03.sh`
  - one-image local/Fly proof path
- `README.md` does not link to a canonical distributed proof surface

## Recommendation

### 1. Keep the app contract unchanged

S04 should **not** add new endpoints. The existing proof surface is already good enough:
- `/membership` for truthful cluster state
- `/work` for ingress-vs-execution proof
- current logs for request correlation and execution proof

### 2. Put the “small local operator surface” in the image wrapper, not in Mesh runtime

`cluster-proof/config.mpl` is already good for:
- explicit local identity
- Fly identity

The missing ergonomic piece is **local container defaulting**.

Recommended approach:
- in the image entrypoint/wrapper, if local explicit identity env is absent and Fly env is absent, derive:
  - `CLUSTER_PROOF_NODE_BASENAME=${CLUSTER_PROOF_NODE_BASENAME:-$HOSTNAME}`
  - `CLUSTER_PROOF_ADVERTISE_HOST=${CLUSTER_PROOF_ADVERTISE_HOST:-$HOSTNAME}`
- then `exec` the `cluster-proof` binary

Why this is the clean seam:
- it keeps the app/runtime contract stable
- it makes local Docker runs simple
- it avoids teaching the Mesh app to trust host `HOSTNAME` everywhere, including non-container direct-host cases where that may be misleading

### 3. Build and deploy from repo root context

This is the most important packaging constraint.

The `cluster-proof` image build needs the repo-wide compiler/runtime sources. That means:
- `docker build` must use **repo root context**
- Fly deploy must also use **repo root working directory / build context**

Recommended operator form:
- local build: `docker build -f cluster-proof/Dockerfile .`
- Fly deploy: `fly deploy . --config cluster-proof/fly.toml --dockerfile cluster-proof/Dockerfile`

Do **not** plan around `cd cluster-proof && fly deploy` unless the Dockerfile is rewritten to avoid repo-root copies. Current repo shape makes that the wrong default.

### 4. Use a user-defined Docker network + shared alias for local discovery

Recommended local proof shape:
- create one Docker bridge network
- run two containers from the same image on that network
- give both containers the **same network alias** as the discovery seed
- set distinct hostnames per container
- publish only the HTTP ports to the host; cluster port stays internal

Why this fits the existing runtime:
- `mesh-rt` already expects DNS seed + cluster port
- discovery candidates are bootstrap-only, and handshake identity remains authoritative

Practical note:
- official Docker docs clearly support DNS-by-name on user-defined bridge networks
- multi-container round-robin/shared-alias behavior is commonly used in practice, but S04’s local verifier should still **preflight it explicitly** (for example by checking seed resolution from inside a disposable container or one of the proof containers) before trusting it as evidence

### 5. Separate local verifier, Fly verifier, and docs-truth verifier

They should stay distinct:
- **local verifier**: authoritative S04 closeout gate; fully replayable in repo
- **Fly verifier**: read-only verification against an existing deployed Fly app
- **docs-truth verifier**: grep/contract checker that keeps README + proof page + runbook aligned

That makes failures legible:
- image/packaging regression
- local operator-path regression
- Fly/runtime drift
- docs drift

## Natural seams for planning

### Seam 1 — Image + operator contract

Likely files:
- `cluster-proof/Dockerfile` (new)
- `cluster-proof/docker-entrypoint.sh` or equivalent shell wrapper (new)
- `cluster-proof/fly.toml` (new)
- maybe `.dockerignore` adjustments if the Docker build needs something the root ignore currently excludes

What to build/prove here:
- repo-root Docker build works
- runtime image only contains the final app + minimal wrapper
- local default identity comes from container hostname when explicit identity env is absent
- Fly config keeps machines awake and exposes the HTTP surface correctly

Why first:
- the rest of S04 depends on a real image

### Seam 2 — Local one-image verifier

Likely files:
- `scripts/verify-m039-s04.sh` or `scripts/verify-m039-s04-local.sh` (new)
- maybe helper scripts under `cluster-proof/scripts/` or `scripts/lib/`

What it should prove:
- replay `scripts/verify-m039-s03.sh` first
- build image from repo root
- start two containers from the same image on one user-defined Docker network
- confirm discovery seed resolves to both containers before beginning the cluster proof
- prove:
  - two-node membership
  - remote `/work`
  - node-loss degrade to self-only membership
  - local `/work` fallback
  - same-image restart / same-identity rejoin
  - restored remote `/work`
- preserve response JSON + container logs under `.tmp/m039-s04/verify/`

Recommendation:
- use a shell wrapper and keep artifact naming close to S03
- only reach for a new Rust e2e harness if Docker shell orchestration becomes unmanageably flaky

### Seam 3 — Fly runbook + read-only verifier

Likely files:
- `cluster-proof/README.md` (new)
- `scripts/verify-m039-s04-fly.sh` (new)

What it should do:
- assume an already-deployed Fly app
- use read-only checks only:
  - `fly status`
  - `fly logs`
  - maybe `fly config show`
  - public or Fly-routed `/membership` and `/work` probes
- fail if the app has fewer than the required running machines or if the runtime signals drift from the local proof contract

Constraint from skill + repo rules:
- any deploy/scale/secrets mutation needs explicit user approval
- the verifier itself should therefore stay read-only

### Seam 4 — Docs truth surface

Likely files:
- `cluster-proof/README.md` (deep operator runbook)
- `website/docs/docs/distributed-proof/index.md` (new, recommended)
- `website/docs/docs/distributed/index.md` (update to route to the proof surface)
- `website/docs/.vitepress/config.mts` (sidebar entry)
- `README.md` (link to distributed proof surface)
- `scripts/verify-m039-s04-proof-surface.sh` (new)

Recommendation:
- mirror the backend proof pattern
- keep `website/docs/docs/distributed/index.md` as the generic API guide
- add a distinct proof page for the actual operator/verification story

## Constraints and non-obvious details

- `Node.start` **binds** to the advertised host from the node name (`compiler/mesh-rt/src/dist/node.rs`). That means the Fly/private-IP contract is real listener behavior, not just a label.
- `mesh-rt` discovery is currently DNS-only via `MESH_DISCOVERY_SEED`, with optional `MESH_DISCOVERY_INTERVAL_MS`. There is no broader provider matrix to design around in S04.
- Fly `.internal` DNS only returns **started/running** machines. That means:
  - `auto_stop_machines = 'off'` is part of the real proof contract
  - the Fly verifier should check running-machine truth, not just app health
- `fly.toml`’s Dockerfile path does **not** change build context. If the Dockerfile needs repo-root files, the deploy command/runbook must make repo-root context explicit.
- The stable routing proof is still the pair of:
  - `/work` JSON body
  - execution-node log line
  Do not replace that with a new synthetic coordinator story in S04.
- Existing project knowledge already warns that S01/S02/S03 wrappers are authoritative and fail closed on skipped named tests. S04 should preserve that style rather than inventing looser acceptance criteria.

## Verification plan

Minimum prereq chain that should remain in any S04 wrapper:

```bash
cargo run -q -p meshc -- test cluster-proof/tests
cargo run -q -p meshc -- build cluster-proof
bash scripts/verify-m039-s03.sh
```

Likely new local operator-path gate:

```bash
docker build -f cluster-proof/Dockerfile -t mesh-cluster-proof .
bash scripts/verify-m039-s04.sh
```

Docs truth gate should be explicit and separate:

```bash
npm --prefix website run build
bash scripts/verify-m039-s04-proof-surface.sh
```

Fly verification should be a separate read-only command path against an existing app, not something auto-mode executes blindly.

## Sources

- Fly.io docs — Machine runtime environment (`FLY_APP_NAME`, `FLY_REGION`, `FLY_MACHINE_ID`, `FLY_PRIVATE_IP`)
  - https://fly.io/docs/machines/runtime-environment/
- Fly.io docs — Private networking / `.internal` DNS / only started machines returned
  - https://fly.io/docs/networking/private-networking/
- Fly.io docs — monorepo deployments / working directory as build context / `--config` and `--dockerfile`
  - https://fly.io/docs/launch/monorepo/
- Fly.io docs — `fly.toml` build section gotchas around Dockerfile path vs context
  - https://fly.io/docs/reference/configuration/
- Docker docs — user-defined bridge networking and DNS-by-name
  - https://docs.docker.com/engine/network/drivers/bridge/
- Practical Docker DNS round-robin/shared-alias reference used to assess the local seed approach
  - https://www.compilenrun.com/docs/devops/docker/docker-networking/docker-dns/
