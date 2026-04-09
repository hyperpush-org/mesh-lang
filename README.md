<div align="center">

# Mesh Language

![Version](https://img.shields.io/badge/version-v12.0-blue.svg?style=flat-square)
![License](https://img.shields.io/badge/license-MIT-green.svg?style=flat-square)
![Build](https://img.shields.io/badge/build-passing-success.svg?style=flat-square)

**Expressive, readable concurrency.**
*Elixir-style syntax. Static type inference. Native single binaries.*

[Get Started](https://meshlang.dev/docs/getting-started/) • [Documentation](https://meshlang.dev) • [Contributing](CONTRIBUTING.md)

</div>

---

## Getting Started

### 1. Install Mesh

**macOS and Linux**

```bash
curl -sSf https://meshlang.dev/install.sh | sh
```

**Windows (PowerShell)**

```powershell
irm https://meshlang.dev/install.ps1 | iex
```

Installer URLs:

- https://meshlang.dev/install.sh
- https://meshlang.dev/install.ps1

### 2. Verify the install

```bash
meshc --version
meshpkg --version
```

Refresh an installed toolchain in place with either binary:

```bash
meshc update
meshpkg update
```

### 3. Start with hello world

```bash
meshc init hello_mesh
cd hello_mesh
```

Open `main.mpl` and replace its contents with:

```mesh
fn main() do
  println("Hello, World!")
end
```

Compile and run it:

```bash
meshc build .
./hello_mesh
```

`main.mpl` remains the default executable entrypoint. If you need a different startup file later, use the optional `[package].entrypoint = "lib/start.mpl"` setting in `mesh.toml`.

### 4. Choose your next starter

Once hello-world runs, pick the starter that matches your next job:

- `meshc init --clustered hello_cluster` — the minimal clustered starter. The generated example uses `@cluster pub fn add()` and the runtime-owned handler name `Work.add`.
- `meshc init --template todo-api --db sqlite todo_api` — the **honest local-only starter** and the **honest local single-node SQLite starter**. It stays local-only, keeps SQLite single-node only, and includes actor-backed write rate limiting. See `examples/todo-sqlite/README.md`.
- `meshc init --template todo-api --db postgres shared_todo` — the **shared/deployable** starter and the **serious shared/deployable PostgreSQL starter**. It uses `HTTP.clustered(1, ...)` for `GET /todos` and `GET /todos/:id`, while `GET /health` and mutating routes stay local. It is the generated starter that owns the staged deploy + failover proof chain once you step onto the proof pages. See `examples/todo-postgres/README.md`.

Then follow the generated project README, or go straight to the docs:

- https://meshlang.dev/docs/getting-started/
- https://meshlang.dev/docs/tooling/

## Where to go next

Keep the public ladder starter/examples-first: the scaffold and `/examples` stay ahead of maintainer proof surfaces.

- **Main getting started guide:** https://meshlang.dev/docs/getting-started/
- **Clustered walkthrough:** use `meshc init --clustered` and then follow https://meshlang.dev/docs/getting-started/clustered-example/
- **SQLite Todo starter:** https://github.com/hyperpush-org/mesh-lang/blob/main/examples/todo-sqlite/README.md — the honest local-only, single-node-only starter.
- **PostgreSQL Todo starter:** https://github.com/hyperpush-org/mesh-lang/blob/main/examples/todo-postgres/README.md — the serious shared/deployable starter and the handoff into the staged deploy + failover proof chain on the proof pages.
- **Production Backend Proof:** https://meshlang.dev/docs/production-backend-proof/ — only after the starter/examples-first ladder, when you need the repo-boundary handoff into the [Hyperpush product repo](https://github.com/hyperpush-org/hyperpush-mono) for maintained app/backend surfaces.
- **Tooling docs:** https://meshlang.dev/docs/tooling/

## Workspace contract for maintainers

M055 is a two-repo split only: the blessed sibling workspace is `mesh-lang/` plus `hyperpush-mono/`.

`website/`, `packages-website/`, `registry/`, installers, and evaluator-facing starters/examples stay language-owned in `mesh-lang` for this milestone.

Repo-local `.gsd` remains authoritative, and cross-repo work uses the lightweight coordination layer in [WORKSPACE.md](WORKSPACE.md).

If you need the old `mesh-lang/mesher` path locally, use `bash scripts/setup-local-workspace.sh`. `mesh-lang` no longer tracks product source; that compatibility path is local-only and is not part of the canonical GitHub structure or CI contract.

## Maintainers / public release proof

If you are working on Mesh itself rather than just using it, start the deeper app/backend handoff at https://meshlang.dev/docs/production-backend-proof/ and then continue in the [Hyperpush product repo](https://github.com/hyperpush-org/hyperpush-mono) for the maintained app/backend surfaces. Keep that path maintainer-facing; the public starter ladder above stays on scaffold output and `/examples`.

Canonical assembled proof command:

```bash
set -a && source .env && set +a && bash scripts/verify-m034-s05.sh
```

Release candidate tags stay split:

- binary candidate: `v<Cargo version>`
- extension candidate: `ext-v<extension version>`

Hosted workflow evidence is expected from:

- `deploy.yml`
- `deploy-services.yml`
- `authoritative-verification.yml`
- `release.yml`
- `extension-release-proof.yml`
- `publish-extension.yml`

Public surfaces checked by that proof include:

- https://meshlang.dev/docs/getting-started/
- https://meshlang.dev/docs/tooling/
- https://packages.meshlang.dev/packages/snowdamiz/mesh-registry-proof
- https://packages.meshlang.dev/search?q=snowdamiz%2Fmesh-registry-proof
- https://api.packages.meshlang.dev/api/v1/packages?search=snowdamiz%2Fmesh-registry-proof

Inspect these retained artifacts after a run:

- `.tmp/m034-s05/verify/candidate-tags.json`
- `.tmp/m034-s05/verify/remote-runs.json`

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE).
