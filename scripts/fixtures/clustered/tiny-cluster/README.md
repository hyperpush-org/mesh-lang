# tiny-cluster

`tiny-cluster` now lives under `scripts/fixtures/clustered/tiny-cluster/` as the smallest internal route-free clustered fixture in this repository. Keep its package, runtime, and log identities aligned with the generated `meshc init --clustered` scaffold and `cluster-proof/`.

## Package contract

- `mesh.toml` is package-only and intentionally omits manifest cluster declarations
- `main.mpl` has one bootstrap path: `Node.start_from_env()`
- `work.mpl` defines `@cluster pub fn add()`
- the runtime-owned handler name is derived from the ordinary source function name as `Work.add`
- the visible work body stays `1 + 1`
- the project does not own HTTP routes, submit handlers, or work-delay seams

## Runtime contract

Set these environment variables when you want the app to participate in a cluster:

- `MESH_CLUSTER_COOKIE` — shared cluster cookie used for authenticated node traffic
- `MESH_NODE_NAME` — optional advertised node identity (`name@host:port`); defaults to `app@127.0.0.1:$MESH_CLUSTER_PORT`
- `MESH_DISCOVERY_SEED` — discovery seed used by the runtime DNS discovery loop
- `MESH_CLUSTER_PORT` — node listener port (default `4370`)
- `MESH_CONTINUITY_ROLE` — runtime continuity role (`primary` or `standby`)
- `MESH_CONTINUITY_PROMOTION_EPOCH` — bounded promotion epoch (`0` by default)

The runtime automatically starts the source-declared `@cluster` function and closes the continuity record when it returns.

## Smoke rail

```bash
cargo run -q -p meshc -- build scripts/fixtures/clustered/tiny-cluster
cargo run -q -p meshc -- test scripts/fixtures/clustered/tiny-cluster/tests
```

For the repo-wide verifier story, `bash scripts/verify-m047-s04.sh` is the authoritative cutover rail, `bash scripts/verify-m046-s06.sh` and `bash scripts/verify-m046-s05.sh` are historical M046 compatibility aliases into it, and `bash scripts/verify-m045-s05.sh` remains the historical M045 wrapper alias.

## Runtime inspection

Once a built node is running in cluster mode, inspect it through Mesh-owned CLI surfaces instead of app-owned routes:

```bash
meshc cluster status <node-name@host:port> --json
meshc cluster continuity <node-name@host:port> --json
meshc cluster continuity <node-name@host:port> <request-key> --json
meshc cluster diagnostics <node-name@host:port> --json
```

Use the list form first to discover request keys and runtime-owned startup records, then inspect a single record when you want the per-request continuity detail.

## Scope

This directory is intentionally the smallest lower-level route-free proof fixture, not a public onboarding runbook or a separate operator model. Keep it aligned with the generated scaffold and `cluster-proof/`, and treat package-owned control routes as drift rather than documentation variation.
