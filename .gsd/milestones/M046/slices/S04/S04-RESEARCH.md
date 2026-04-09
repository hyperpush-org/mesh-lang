# S04 Research — Rebuild `cluster-proof/` as tiny packaged proof

## Summary

- **Primary requirement:** `R089`.
- **Directly supported requirements:** `R090`, `R092`, `R093`, and preservation room for packaged/deeper operator rails in `R052` / `R082`.
- `tiny-cluster/` already embodies the target app contract for this slice: package-only manifest, `main.mpl` that only calls `Node.start_from_env()`, one source `clustered(work)` declaration, visible `1 + 1` work, and runtime/tooling-only inspection.
- `cluster-proof/` is still a legacy proof app with app-owned trigger/status layers. The current package has:
  - `[cluster]` manifest declarations in `cluster-proof/mesh.toml`
  - HTTP routes in `cluster-proof/main.mpl` (`/membership`, `POST /work`, `GET /work/:request_key`)
  - app-owned continuity/config/placement helpers in `cluster-proof/cluster.mpl`, `cluster-proof/config.mpl`, and `cluster-proof/work_continuity.mpl`
  - a package-owned timing knob `CLUSTER_PROOF_WORK_DELAY_MS` in `cluster-proof/work.mpl`
- This slice is mostly **replacement and deletion**, not compiler/runtime work. The app code should collapse toward the `tiny-cluster/` shape, while the only remaining package-specific surface should be packaging/docs (`README.md`, `Dockerfile`, `fly.toml`).
- Size check confirms that this is not an incremental trim:
  - current `cluster-proof` Mesh source + package tests = **1937 lines** (`main.mpl`, `work.mpl`, `cluster.mpl`, `config.mpl`, `work_continuity.mpl`, `tests/*.mpl`)
  - equivalent `tiny-cluster` Mesh source + package tests = **102 lines**

## Skills Discovered

No new skill installs were needed; the directly relevant skills are already present.

- `flyio-cli-public` (installed)
  - Relevant rule used here: **classify Fly problems as build/packaging vs runtime vs platform**, and prefer **read-only** Fly reasoning first.
  - Implication for S04: do not keep fake HTTP routes or a fake `http_service` just to preserve old live probes. If the packaged proof becomes route-free, Fly verification has to be rethought as packaging/platform work, not hidden inside app code.
- `multi-stage-dockerfile` (installed)
  - Relevant rule used here: keep a clean **builder → runtime** split and **avoid runtime-stage logic that duplicates application/runtime validation**.
  - Implication for S04: keep the Dockerfile minimal, copy only the built binary, and avoid preserving `docker-entrypoint.sh` logic that re-validates continuity topology the runtime should own.

## Implementation Landscape

### Target shape already exists in `tiny-cluster/`

These files are the clearest source of truth for what S04 should copy at the Mesh-package level:

- `tiny-cluster/mesh.toml`
  - plain `[package]` manifest only
  - no `[cluster]`
- `tiny-cluster/main.mpl`
  - `Node.start_from_env()` only
  - no `HTTP.serve`, no routes, no `Continuity.*`
- `tiny-cluster/work.mpl`
  - `clustered(work) pub fn execute_declared_work(...) -> Int do 1 + 1 end`
  - helper `declared_work_runtime_name()` returns `"Work.execute_declared_work"`
- `tiny-cluster/tests/work.test.mpl`
  - package smoke + static file guards for route-free/source-first contract
- `tiny-cluster/README.md`
  - points operators at `meshc cluster status|continuity|diagnostics`
  - explicitly forbids package-owned routes/timing seams in the public story

For S04, the simplest honest implementation is: **make `cluster-proof/` match this package shape first, then add only the packaging-specific layer on top.**

### Current `cluster-proof/` files and what they mean for S04

| Path | Current role | S04 implication |
|---|---|---|
| `cluster-proof/mesh.toml` | Plain package manifest **plus** `[cluster] enabled = true` and `declarations = [{ kind = "work", target = "Work.execute_declared_work" }]` | Must change in the same commit that adds source `clustered(work)`. S01’s shared planner fails closed on same-target manifest+source duplicates, so leaving `[cluster]` while adding the source marker will intentionally break the build. |
| `cluster-proof/main.mpl` | 123-line HTTP app boot path; imports `Config`, `Cluster`, `WorkContinuity`; starts router with `/membership`, `/work`, `/work/:request_key` | Replace, don’t trim. The target is the `tiny-cluster/main.mpl` shape: one `Node.start_from_env()` bootstrap and logging only. |
| `cluster-proof/work.mpl` | 141-line declared-work module with request-key validation, payload hashing, execution logging, and `CLUSTER_PROOF_WORK_DELAY_MS` | Replace with tiny route-free work: keep runtime name `Work.execute_declared_work`, declare with `clustered(work)`, return `1 + 1`, remove all env/timing/HTTP-related helpers. |
| `cluster-proof/cluster.mpl` | Membership JSON shaping and deterministic membership helpers for `/membership` | Delete. This is an app-owned status surface that the route-free package must not keep. |
| `cluster-proof/config.mpl` | Continuity/env validation layer for HTTP app boot | Delete. This duplicates runtime/bootstrap ownership and is exactly the seam M046 is removing from app code. |
| `cluster-proof/work_continuity.mpl` | 672-line app-owned submit/status/authority translation layer over `Continuity.*` plus HTTP response shaping | Delete. Do not relocate this logic; S02 already moved the truthful startup/status surface to runtime/tooling. |
| `cluster-proof/tests/config.test.mpl` | Tests app-owned config/topology helpers | Delete or fully replace; these helpers should disappear with the modules they test. |
| `cluster-proof/tests/work.test.mpl` | Tests app-owned JSON/status payload shapers plus `execute_declared_work()` | Rewrite around the tiny route-free source contract, mirroring `tiny-cluster/tests/work.test.mpl`. |
| `cluster-proof/README.md` | 206-line M045 runbook centered on HTTP routes, old verifier names, `PORT`, and `CLUSTER_PROOF_WORK_DELAY_MS` | Rewrite. The new README should describe the packaged route-free contract and point to `meshc cluster ...`, not `/membership` or `/work`. |
| `cluster-proof/Dockerfile` | Honest builder/runtime split, but runtime contract assumes HTTP (`EXPOSE 8080 4370`, custom entrypoint) | Keep the multi-stage pattern; drop HTTP assumptions. Likely only the binary and cluster port belong in the final contract. |
| `cluster-proof/docker-entrypoint.sh` | 195-line shell validator for continuity env and bootstrap hints | Strong candidate for deletion. It duplicates validation that route-free runtime/bootstrap should own. |
| `cluster-proof/fly.toml` | `app`, `primary_region`, build config, env, and **`[http_service] internal_port = 8080`** | Needs an honest route-free decision. Keeping `http_service` while the app stops listening on HTTP would be fake and likely broken. |

### Current package tests/build status

I verified the current baseline before recommending deletion:

- `cargo run -q -p meshc -- test cluster-proof/tests` — **passes** today on the legacy package.
- `cargo run -q -p meshc -- build cluster-proof --output .tmp/m046-s04-scout/cluster-proof` — **passes** when the output parent directory already exists.
- Hard-won detail: `meshc build ... --output <path>` fails with `Failed to emit object file: "No such file or directory"` if the parent directory does not exist first. Any new e2e proof that builds to a temp path must create the parent dir up front.

### Verification helpers worth reusing instead of recreating

- `compiler/meshc/tests/e2e_m046_s03.rs`
  - already contains the full route-free runtime proof pattern: build to temp output, spawn two nodes, poll `meshc cluster status`, discover startup work through `meshc cluster continuity --json`, and verify diagnostics/failover truth
  - the file is **2039 lines**, and most of the early helpers are generic
  - there is **no shared support module** under `compiler/meshc/tests/`, so S04 either duplicates a giant file or extracts helpers first
- `scripts/verify-m046-s03.sh`
  - already encodes the route-free contract-guard style the new package verifier should follow: static file assertions, focused build/test/e2e, and retained artifacts
  - much better template for S04 than the older `m039/m043` shell libs
- `scripts/lib/m039_cluster_proof.sh` and `scripts/lib/m043_cluster_proof.sh`
  - these are HTTP-JSON helper libraries
  - do **not** extend them for S04; they encode the exact `/membership`/`/work` proof shape this slice is deleting

### Blast radius outside `cluster-proof/`

There are **35 repo files** outside the package source that still describe or test the legacy `cluster-proof` HTTP/submit/delay contract (`/work`, `/membership`, `Continuity.submit_declared_work`, or `CLUSTER_PROOF_WORK_DELAY_MS` in a `cluster-proof` context).

Most important buckets:

- Legacy Rust e2e/tests:
  - `compiler/meshc/tests/e2e_m039_s01.rs`
  - `compiler/meshc/tests/e2e_m039_s02.rs`
  - `compiler/meshc/tests/e2e_m039_s03.rs`
  - `compiler/meshc/tests/e2e_m040_s01.rs`
  - `compiler/meshc/tests/e2e_m042_s01.rs`
  - `compiler/meshc/tests/e2e_m042_s02.rs`
  - `compiler/meshc/tests/e2e_m042_s03.rs`
  - `compiler/meshc/tests/e2e_m043_s01.rs`
  - `compiler/meshc/tests/e2e_m043_s02.rs`
  - `compiler/meshc/tests/e2e_m043_s03.rs`
  - `compiler/meshc/tests/e2e_m044_s01.rs`
  - `compiler/meshc/tests/e2e_m044_s02.rs`
  - `compiler/meshc/tests/e2e_m044_s03.rs`
  - `compiler/meshc/tests/e2e_m044_s04.rs`
  - `compiler/meshc/tests/e2e_m044_s05.rs`
  - `compiler/meshc/tests/e2e_m045_s01.rs`
  - `compiler/meshc/tests/e2e_m045_s02.rs`
  - `compiler/meshc/tests/e2e_m045_s04.rs`
- Legacy shell verifiers / proof-surface scripts:
  - `scripts/verify-m039-s04.sh`
  - `scripts/verify-m039-s04-fly.sh`
  - `scripts/verify-m040-s01.sh`
  - `scripts/verify-m042-s04.sh`
  - `scripts/verify-m042-s04-fly.sh`
  - `scripts/verify-m043-s04-fly.sh`
  - `scripts/verify-m043-s04-proof-surface.sh`
  - `scripts/verify-m044-s05.sh`
  - plus several earlier wrapper scripts that replay those rails
- Docs that still narrate the old route-based story:
  - `website/docs/docs/distributed-proof/index.md`
  - `website/docs/docs/getting-started/clustered-example/index.md`
  - `website/docs/docs/tooling/index.md`
  - `README.md`

This is the main scope trap. S04 does **not** need to solve all docs/scaffold alignment if that drifts into S05/S06, but the planner should choose intentionally which legacy rails are replaced now and which are explicitly deferred.

## Recommendation

### 1. Rebuild the Mesh package by copying the `tiny-cluster/` contract, not by shrinking the old HTTP app

The cleanest route is:

- `cluster-proof/mesh.toml` → package-only manifest (no `[cluster]`)
- `cluster-proof/main.mpl` → `Node.start_from_env()` only
- `cluster-proof/work.mpl` →
  - `pub fn declared_work_runtime_name() -> String do "Work.execute_declared_work" end`
  - `clustered(work) pub fn execute_declared_work(...) -> Int do 1 + 1 end`
- `cluster-proof/tests/work.test.mpl` → tiny-cluster-style package smoke + static source/readme guards
- delete `cluster-proof/cluster.mpl`, `cluster-proof/config.mpl`, `cluster-proof/work_continuity.mpl`, and `cluster-proof/tests/config.test.mpl`

This is the narrowest honest way to satisfy `R089`/`R092`/`R093`.

### 2. Keep the package-specific surface in packaging/docs only

What should remain unique to `cluster-proof/` after the reset is not Mesh app logic; it is the package/deployment envelope:

- `cluster-proof/README.md`
- `cluster-proof/Dockerfile`
- `cluster-proof/fly.toml`
- maybe a tiny runtime log prefix (`[cluster-proof]`) if useful for proof logs

Everything else should look like `tiny-cluster/`.

### 3. Make Docker/Fly honest, even if that means narrowing their immediate scope

Current conflict:

- the new app contract is route-free and does not listen on HTTP
- the current packaged contract still assumes `PORT`, public `/membership`, `/work/:request_key`, and `[http_service] internal_port = 8080`

Most likely honest S04 posture:

- **Dockerfile:** keep the builder/runtime split, copy only the built binary, drop HTTP-specific entrypoint logic, and do not preserve `EXPOSE 8080` just because older scripts curl it
- **Fly config:** either
  - remove `http_service`/`PORT` entirely and treat the packaged proof as a private non-proxied process, or
  - explicitly defer live packaged/Fly verification while keeping only the minimal config needed for later work

What should **not** happen: keeping a fake HTTP listener or health route only so legacy verifiers continue to work.

### 4. Reuse the S03 route-free proof infrastructure

S04 should not invent a second verification style.

Best path:

- create `compiler/meshc/tests/e2e_m046_s04.rs`
- reuse/extract the generic route-free helpers from `compiler/meshc/tests/e2e_m046_s03.rs`
- build/run `cluster-proof` exactly like `tiny-cluster`, then prove startup truth through:
  - `meshc cluster status --json`
  - `meshc cluster continuity --json`
  - `meshc cluster diagnostics --json`
- add a `scripts/verify-m046-s04.sh` patterned after `scripts/verify-m046-s03.sh`

If helpers are extracted, put them in a submodule directory (for example `compiler/meshc/tests/support/...`), not as a new top-level `tests/*.rs` file, so Cargo does not treat the helper as its own integration test target.

### 5. Keep the runtime name stable

Do not rename the declared handler runtime name. Keep `Work.execute_declared_work` so:

- S02 startup identity continues to work unchanged
- S03/S04 can share proof helpers and expectations
- S05 can align scaffold, `tiny-cluster/`, and `cluster-proof/` on one visible contract

## Natural task seams

### Seam A — package source reset
Files:
- `cluster-proof/mesh.toml`
- `cluster-proof/main.mpl`
- `cluster-proof/work.mpl`
- delete `cluster-proof/cluster.mpl`
- delete `cluster-proof/config.mpl`
- delete `cluster-proof/work_continuity.mpl`

Why first:
- this is the actual requirement boundary (`R089`, `R092`, `R093`)
- everything else depends on the final package shape

### Seam B — package smoke tests + README contract
Files:
- `cluster-proof/tests/work.test.mpl`
- delete `cluster-proof/tests/config.test.mpl`
- `cluster-proof/README.md`

Why second:
- once the package source is tiny, the tests and README can lock that new shape in place
- package tests are cheap proof and should fail closed on route/delay regressions

### Seam C — packaged deployment envelope
Files:
- `cluster-proof/Dockerfile`
- `cluster-proof/docker-entrypoint.sh`
- `cluster-proof/fly.toml`

Why separate:
- this is where the real ambiguity lives (route-free app vs prior HTTP/Fly assumptions)
- the planner should isolate packaging truth from package-source truth

### Seam D — new route-free proof rail
Files:
- `compiler/meshc/tests/e2e_m046_s04.rs`
- optional shared helper extraction from `compiler/meshc/tests/e2e_m046_s03.rs`
- `scripts/verify-m046-s04.sh`

Why last:
- proof rail should target the final rebuilt package, not the old one
- this seam can reuse S03 patterns and gives S06 an artifact to assemble later

## Don’t Hand-Roll

- **Do not** keep app-owned `Continuity.submit_declared_work(...)` / status routes in `cluster-proof/` just because the packaged proof used to have them. S02 already moved the honest startup/status story into runtime/tooling.
- **Do not** leave the `[cluster]` manifest declaration in place when adding `clustered(work)`. S01 intentionally rejects same-target manifest+source duplicates.
- **Do not** move `config.mpl` / `work_continuity.mpl` logic into a thinner wrapper module. That would preserve the wrong seam in a smaller disguise.
- **Do not** preserve `CLUSTER_PROOF_WORK_DELAY_MS` or any package-owned delay knob. S03 already retired the timing seam into Mesh-owned runtime behavior.
- **Do not** retrofit the old HTTP shell libs (`scripts/lib/m039_cluster_proof.sh`, `scripts/lib/m043_cluster_proof.sh`) for the new proof. They encode the contract S04 is deleting.
- **Do not** keep a fake Fly/Docker `http_service` or dummy HTTP route only for probe compatibility. If the binary is route-free, the packaging layer must admit that.

## Risks / Unknowns

- **Route-free packaged inspection is not the same problem as local route-free inspection.**
  - Local `tiny-cluster/` proof works because both nodes advertise host-reachable loopback addresses and the host can run `meshc cluster ...` directly.
  - Current Docker/Fly package proofs relied on HTTP precisely because host-side runtime CLI inspection was awkward there.
  - If S04 tries to preserve same-image/Fly live verification immediately, the planner needs a concrete operator path (for example host-reachable advertised node names, a helper operator container, or `fly ssh`/private-network inspection). Otherwise the honest move is to narrow the packaged proof for now.
- **Legacy reference blast radius is real.** Touching all 35 dependent files turns S04 into S05/S06. The planner should choose a strict boundary.
- **Tracked generated artifacts exist.** `cluster-proof/cluster-proof` and `cluster-proof/cluster-proof.ll` are tracked. Any default in-place build will churn them. New e2e rails should prefer `--output` temp paths unless the slice explicitly decides to regenerate committed artifacts.
- **There is no existing shared test support module.** If S04 duplicates S03 wholesale, the test surface will explode. Extraction is likely worth it because S05/S06 will need both packages under one route-free proof story.

## Verification

Minimum slice-proof commands I would plan around:

- `cargo run -q -p meshc -- build cluster-proof`
- `cargo run -q -p meshc -- test cluster-proof/tests`
- `cargo test -p meshc --test e2e_m046_s04 m046_s04_ -- --nocapture`

Recommended regression if S04 extracts shared helper code or touches route-free proof infrastructure:

- `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_package_ -- --nocapture`
- optionally the full `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_ -- --nocapture` rail if helper extraction is broad

Verification detail to remember:

- if the proof builds to a temp output path, pre-create the parent directory first
- if a shell verifier is added, model it after `scripts/verify-m046-s03.sh`, not the old curl-based `cluster-proof` scripts

## Sources

External check used only for the Fly packaging constraint:

- `search-the-web` query: `Fly.io fly.toml app without http_service process-only machine internal service no public HTTP`
  - `https://fly.io/docs/networking/app-services/` — service definitions are for Fly Proxy-routed services; if you do not define a service, Fly Proxy does not route traffic to the app
  - `https://fly.io/docs/blueprints/autostart-internal-apps/` — private apps that want Fly Proxy/autostart still need a `services`/`http_service` definition, which means a route-free binary should not keep `http_service` unless it actually listens on that port
