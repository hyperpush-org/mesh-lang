---
estimated_steps: 4
estimated_files: 6
skills_used:
  - test
---

# T01: Reset `cluster-proof/` source to the tiny route-free clustered contract

**Slice:** S04 — Rebuild `cluster-proof/` as tiny packaged proof
**Milestone:** M046

## Description

Delete the legacy proof-app source and make `cluster-proof/` match the tiny route-free package shape so app code only denotes clustered work and delegates startup/inspection to Mesh-owned surfaces.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `cluster-proof/mesh.toml` declaration surface | Fail `meshc build cluster-proof` if `[cluster]` or duplicate declaration state remains. | N/A | Treat any mixed manifest+source declaration state as a contract error instead of silently preferring one. |
| `Node.start_from_env()` bootstrap path in `cluster-proof/main.mpl` | Fail compile/build if deleted modules or route imports survive. | Runtime startup should exit with the existing fail-closed bootstrap error instead of spinning up a partial process. | Do not parse or reshape bootstrap data in package code; surface the runtime error directly. |
| Legacy module deletion (`cluster.mpl`, `config.mpl`, `work_continuity.mpl`) | Fail fast on stale imports or references rather than keeping thin wrappers around deleted seams. | N/A | N/A |

## Load Profile

- **Shared resources**: One runtime-owned startup registration/continuity path plus package bootstrap logs.
- **Per-operation cost**: One trivial declared-work execution returning `2` and no app-owned HTTP/control plane.
- **10x breakpoint**: Duplicate declaration drift or repeated startup boot loops will break proof truth long before CPU or memory matter.

## Negative Tests

- **Malformed inputs**: Mixed manifest+source declaration state, missing deleted modules, or renamed runtime handler strings.
- **Error paths**: Any surviving `HTTP.serve(...)`, `/work`, `/membership`, `Continuity.*`, or package-owned env/timing helpers must fail the source contract.
- **Boundary conditions**: `cluster-proof/work.mpl` keeps exactly one `clustered(work)` declaration and returns `1 + 1` while the runtime name stays `Work.execute_declared_work`.

## Steps

1. Rewrite `cluster-proof/mesh.toml` to a package-only manifest and rewrite `cluster-proof/work.mpl` to own the single source `clustered(work)` declaration plus `declared_work_runtime_name()`.
2. Replace `cluster-proof/main.mpl` with the tiny route-free bootstrap shape: call `Node.start_from_env()`, log success/failure, and omit `HTTP.serve(...)`, routes, and continuity imports.
3. Delete `cluster-proof/cluster.mpl`, `cluster-proof/config.mpl`, and `cluster-proof/work_continuity.mpl`, and remove every remaining source reference to them.
4. Keep the declared handler runtime name stable (`Work.execute_declared_work`) so S02/S03 runtime-owned startup discovery still matches the packaged proof.

## Must-Haves

- [ ] `cluster-proof/mesh.toml` no longer contains `[cluster]` or `declarations`.
- [ ] `cluster-proof/main.mpl` contains exactly one `Node.start_from_env()` boot path and no app-owned routes or continuity calls.
- [ ] `cluster-proof/work.mpl` contains exactly one `clustered(work)` declaration, keeps the runtime name `Work.execute_declared_work`, and returns `1 + 1`.
- [ ] `cluster-proof/cluster.mpl`, `cluster-proof/config.mpl`, and `cluster-proof/work_continuity.mpl` are deleted rather than preserved as thin wrappers.

## Verification

- `cargo run -q -p meshc -- build cluster-proof && test ! -e cluster-proof/cluster.mpl && test ! -e cluster-proof/config.mpl && test ! -e cluster-proof/work_continuity.mpl`

## Observability Impact

- Signals added/changed: route-free `[cluster-proof] runtime bootstrap ...` logs replace HTTP server/config logs.
- How a future agent inspects this: build the package and inspect startup stdout/stderr plus direct source-contract assertions.
- Failure state exposed: stale imports, duplicate declaration drift, or runtime bootstrap failure remain visible without an app-owned route layer.

## Inputs

- `cluster-proof/mesh.toml` — current manifest that still declares clustered work through `[cluster]`.
- `cluster-proof/main.mpl` — legacy HTTP/bootstrap entrypoint that must collapse to runtime-owned startup only.
- `cluster-proof/work.mpl` — current declared-work module whose runtime name must stay stable while the workload becomes trivial.
- `tiny-cluster/mesh.toml` — source-first package-only manifest reference.
- `tiny-cluster/main.mpl` — minimal route-free bootstrap reference.
- `tiny-cluster/work.mpl` — single-source `clustered(work)` contract reference.

## Expected Output

- `cluster-proof/mesh.toml` — rewritten package-only manifest with no `[cluster]` section.
- `cluster-proof/main.mpl` — route-free bootstrap entrypoint that only calls `Node.start_from_env()`.
- `cluster-proof/work.mpl` — tiny `clustered(work)` declared handler returning `1 + 1` with runtime name `Work.execute_declared_work`.
- `cluster-proof/cluster.mpl` — deleted legacy membership/status helper module.
- `cluster-proof/config.mpl` — deleted legacy continuity/bootstrap config helper module.
- `cluster-proof/work_continuity.mpl` — deleted legacy submit/status translation module.
