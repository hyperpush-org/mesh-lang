---
id: T04
parent: S08
milestone: M047
provides: []
requires: []
affects: []
key_files: ["README.md", "website/docs/docs/tooling/index.md", "website/docs/docs/getting-started/clustered-example/index.md", "website/docs/docs/distributed/index.md", "website/docs/docs/distributed-proof/index.md", "compiler/meshc/tests/e2e_m047_s06.rs", "scripts/verify-m047-s06.sh", "compiler/meshc/tests/e2e_m047_s04.rs", "compiler/meshc/tests/e2e_m047_s05.rs", ".gsd/DECISIONS.md"]
key_decisions: ["Keep the public clustered story route-free first, while scoping shipped wrapper adoption to the Todo starter's `GET /todos` and `GET /todos/:id` routes via explicit-count `HTTP.clustered(1, ...)`.", "Point default-count and two-node clustered-route behavior at the repo S07 rail instead of implying the Todo starter or route-free docs prove that broader surface."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the targeted S06 doc contract, the delegated S04 and S05 doc-contract rebases that the assembled closeout rail depends on, the full `bash scripts/verify-m047-s06.sh` wrapper, and the standalone website build from the task contract. `cargo test -p meshc --test e2e_m047_s06 m047_s06_ -- --nocapture`, `cargo test -p meshc --test e2e_m047_s04 -- --nocapture`, `cargo test -p meshc --test e2e_m047_s05 m047_s05_public_clustered_surfaces_use_source_first_names_and_todo_template -- --nocapture`, `bash scripts/verify-m047-s06.sh`, and `npm --prefix website run build` all passed."
completed_at: 2026-04-02T03:39:56.606Z
blocker_discovered: false
---

# T04: Rebased the public clustered docs and closeout rails onto truthful Todo `HTTP.clustered(1, ...)` adoption while keeping route-free `@cluster` surfaces canonical.

> Rebased the public clustered docs and closeout rails onto truthful Todo `HTTP.clustered(1, ...)` adoption while keeping route-free `@cluster` surfaces canonical.

## What Happened
---
id: T04
parent: S08
milestone: M047
key_files:
  - README.md
  - website/docs/docs/tooling/index.md
  - website/docs/docs/getting-started/clustered-example/index.md
  - website/docs/docs/distributed/index.md
  - website/docs/docs/distributed-proof/index.md
  - compiler/meshc/tests/e2e_m047_s06.rs
  - scripts/verify-m047-s06.sh
  - compiler/meshc/tests/e2e_m047_s04.rs
  - compiler/meshc/tests/e2e_m047_s05.rs
  - .gsd/DECISIONS.md
key_decisions:
  - Keep the public clustered story route-free first, while scoping shipped wrapper adoption to the Todo starter's `GET /todos` and `GET /todos/:id` routes via explicit-count `HTTP.clustered(1, ...)`.
  - Point default-count and two-node clustered-route behavior at the repo S07 rail instead of implying the Todo starter or route-free docs prove that broader surface.
duration: ""
verification_result: passed
completed_at: 2026-04-02T03:39:56.610Z
blocker_discovered: false
---

# T04: Rebased the public clustered docs and closeout rails onto truthful Todo `HTTP.clustered(1, ...)` adoption while keeping route-free `@cluster` surfaces canonical.

**Rebased the public clustered docs and closeout rails onto truthful Todo `HTTP.clustered(1, ...)` adoption while keeping route-free `@cluster` surfaces canonical.**

## What Happened

Updated README.md plus the public VitePress clustered surfaces (`tooling`, `clustered-example`, `distributed`, and `distributed-proof`) so they no longer teach `HTTP.clustered(...)` as blanket-unshipped. The docs now state the narrower truth: the canonical public clustered story is still route-free `@cluster`, the Todo starter only dogfoods explicit-count `HTTP.clustered(1, ...)` on `GET /todos` and `GET /todos/:id`, `GET /health` plus mutating routes stay local, and default-count/two-node wrapper behavior belongs to the repo S07 rail. I then rebased `compiler/meshc/tests/e2e_m047_s06.rs` and `scripts/verify-m047-s06.sh` to require that narrower authority split and reject both stale 'not shipped' language and blanket overclaims. Because the assembled S06 verifier delegates through the existing S05 and S04 rails, I also narrowed the stale delegated doc-contract assertions in `compiler/meshc/tests/e2e_m047_s04.rs` and `compiler/meshc/tests/e2e_m047_s05.rs` so those older rails accept the truthful Todo starter docs instead of pre-adoption blanket expectations. I recorded the resulting public-doc scope choice in decision D307.

## Verification

Verified the targeted S06 doc contract, the delegated S04 and S05 doc-contract rebases that the assembled closeout rail depends on, the full `bash scripts/verify-m047-s06.sh` wrapper, and the standalone website build from the task contract. `cargo test -p meshc --test e2e_m047_s06 m047_s06_ -- --nocapture`, `cargo test -p meshc --test e2e_m047_s04 -- --nocapture`, `cargo test -p meshc --test e2e_m047_s05 m047_s05_public_clustered_surfaces_use_source_first_names_and_todo_template -- --nocapture`, `bash scripts/verify-m047-s06.sh`, and `npm --prefix website run build` all passed.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m047_s06 m047_s06_ -- --nocapture` | 0 | ✅ pass | 7490ms |
| 2 | `cargo test -p meshc --test e2e_m047_s04 -- --nocapture` | 0 | ✅ pass | 3840ms |
| 3 | `cargo test -p meshc --test e2e_m047_s05 m047_s05_public_clustered_surfaces_use_source_first_names_and_todo_template -- --nocapture` | 0 | ✅ pass | 8490ms |
| 4 | `bash scripts/verify-m047-s06.sh` | 0 | ✅ pass | 325300ms |
| 5 | `npm --prefix website run build` | 0 | ✅ pass | 60171ms |


## Deviations

Had to update delegated S04 and S05 doc-contract tests even though the task plan named only S06-facing inputs, because the assembled S06 verifier wraps those older rails and they still encoded the pre-adoption `HTTP.clustered(...)` public contract.

## Known Issues

`npm --prefix website run build` still emits the pre-existing VitePress chunk-size warning about bundles larger than 500 kB after minification; the build passes and this task did not change that behavior.

## Files Created/Modified

- `README.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `compiler/meshc/tests/e2e_m047_s06.rs`
- `scripts/verify-m047-s06.sh`
- `compiler/meshc/tests/e2e_m047_s04.rs`
- `compiler/meshc/tests/e2e_m047_s05.rs`
- `.gsd/DECISIONS.md`


## Deviations
Had to update delegated S04 and S05 doc-contract tests even though the task plan named only S06-facing inputs, because the assembled S06 verifier wraps those older rails and they still encoded the pre-adoption `HTTP.clustered(...)` public contract.

## Known Issues
`npm --prefix website run build` still emits the pre-existing VitePress chunk-size warning about bundles larger than 500 kB after minification; the build passes and this task did not change that behavior.
