---
estimated_steps: 4
estimated_files: 7
skills_used:
  - rust-best-practices
  - test
---

# T01: Decouple the historical clustered Todo proof from the public scaffold

**Slice:** S02 — SQLite local starter contract
**Milestone:** M049

## Description

Move the old clustered SQLite Todo app off the public `meshc init` path before S02 rewrites the public scaffold. This task keeps the historical M047 native/Docker proof alive, but it does so behind an internal fixture copy seam rather than by forcing the public SQLite starter to keep teaching a stale clustered contract.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Internal legacy fixture tree | Fail the helper before any runtime proof starts and name the missing file; do not silently fall back to the public scaffold. | N/A — local copy only. | Reject partial fixture copies instead of producing a misleading generated project. |
| `compiler/meshc/tests/e2e_m047_s05.rs` and `scripts/verify-m047-s05.sh` | Stop on the first red historical proof phase and preserve retained bundle pointers/logs. | Keep existing bounded timeouts on native/Docker proof. | Reject missing retained markers or bundle-shape drift instead of treating the history as optional. |

## Load Profile

- **Shared resources**: temp workspaces, retained `.tmp/m047-s05` artifacts, and the existing native/Docker proof helpers.
- **Per-operation cost**: one fixture copy plus the existing historical M047 proof replay.
- **10x breakpoint**: fixture drift or retained-artifact collisions show up before compile time.

## Negative Tests

- **Malformed inputs**: missing fixture files, stale helper paths, or a partial copied project tree.
- **Error paths**: historical verifier bundle markers missing, Docker/native proof unable to find the fixture, or any fallback back to public `meshc init`.
- **Boundary conditions**: the retained history stays runnable without reintroducing a second public starter mode.

## Steps

1. Commit a full legacy clustered Todo fixture tree under `scripts/fixtures/m047-s05-clustered-todo/` capturing the current M047 public contract, including the route wrappers, README guidance, and Docker/runtime files that the historical rails still depend on.
2. Change `compiler/meshc/tests/support/m047_todo_scaffold.rs` to copy that fixture into temp workspaces instead of invoking `meshc init --template todo-api`, while preserving the existing archive/build/runtime helper flow.
3. Retarget `compiler/meshc/tests/e2e_m047_s05.rs` and `scripts/verify-m047-s05.sh` to prove the fixture-backed history and keep the same retained native/Docker artifact story.
4. Fail closed if the fixture tree or retained bundle markers drift; do not invent a hidden public init mode to preserve the history.

## Must-Haves

- [ ] Historical M047 clustered Todo proof no longer depends on the public SQLite scaffold.
- [ ] The committed fixture fully captures the old clustered Todo contract needed by the native/Docker rails.
- [ ] `bash scripts/verify-m047-s05.sh` still owns a truthful retained historical proof bundle.

## Verification

- `cargo test -p meshc --test e2e_m047_s05 -- --nocapture`
- `bash scripts/verify-m047-s05.sh`

## Observability Impact

- Signals added/changed: the retained M047 helper and verifier report fixture-copy provenance instead of public `meshc init` provenance.
- How a future agent inspects this: rerun `bash scripts/verify-m047-s05.sh` and inspect the copied fixture-backed `generated-project` plus retained bundle logs.
- Failure state exposed: missing fixture files or malformed retained markers fail before native/Docker proof can claim success.

## Inputs

- `compiler/meshc/tests/support/m047_todo_scaffold.rs` — current helper that shells `meshc init --template todo-api` for the historical M047 proof.
- `compiler/meshc/tests/e2e_m047_s05.rs` — native and Docker runtime proof still coupled to the public scaffold.
- `scripts/verify-m047-s05.sh` — retained M047 wrapper that must keep the same historical bundle shape.
- `compiler/mesh-pkg/src/scaffold.rs` — current public scaffold contract that the fixture must snapshot before S02 rewrites it.

## Expected Output

- `scripts/fixtures/m047-s05-clustered-todo/mesh.toml` — committed fixture manifest for the historical clustered Todo app.
- `scripts/fixtures/m047-s05-clustered-todo/main.mpl` — fixture bootstrap/runtime entrypoint matching the retained M047 contract.
- `scripts/fixtures/m047-s05-clustered-todo/work.mpl` — fixture clustered work declaration preserved for history.
- `scripts/fixtures/m047-s05-clustered-todo/api/router.mpl` — fixture routed clustered-read contract retained for M047.
- `compiler/meshc/tests/support/m047_todo_scaffold.rs` — helper rewritten to copy the fixture instead of invoking public `meshc init`.
- `compiler/meshc/tests/e2e_m047_s05.rs` — runtime/native/Docker proof updated to use the fixture-backed helper.
- `scripts/verify-m047-s05.sh` — retained verifier still owning the historical bundle while proving the fixture-backed history.
