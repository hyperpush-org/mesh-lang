# S02: First-Contact Docs Rewrite

**Goal:** Rewrite the public first-contact docs so Mesh now teaches one coherent evaluator path: install Mesh, run hello-world, then deliberately choose the clustered scaffold, the honest local SQLite Todo starter, or the serious shared/deployable Postgres Todo starter without proof pages or retained verifier maps competing for first contact.
**Demo:** After this: A builder can read Getting Started, run hello-world, then choose `meshc init --clustered`, `meshc init --template todo-api --db sqlite`, or `meshc init --template todo-api --db postgres`, with Tooling reinforcing the same story and exposing the assembled docs-truth command.

## Tasks
- [x] **T01: Rewrote README and Getting Started around the explicit clustered/SQLite/Postgres starter chooser and added a first-contact docs contract.** — ---
estimated_steps: 12
estimated_files: 5
skills_used:
  - vitepress
  - test
---

The highest-risk first-contact drift is still in `Getting Started`: it jumps readers toward proof surfaces too early, never gives the explicit clustered/SQLite/Postgres choice after hello-world, and still points source builds at the stale `hyperpush-org/hyperpush-mono` repo. This task fixes the repo-root and Getting Started entrypoints together and seeds the slice-owned first-contact contract on that seam.

## Steps

1. Use `compiler/mesh-pkg/src/scaffold.rs`, `examples/todo-sqlite/README.md`, `examples/todo-postgres/README.md`, and the current repo `README.md` as copy anchors; rewrite `website/docs/docs/getting-started/index.md` so install -> hello-world stays primary and the post-hello-world branch explicitly offers `meshc init --clustered`, `meshc init --template todo-api --db sqlite`, and `meshc init --template todo-api --db postgres`.
2. Tighten `README.md` starter-path wording only where it needs to match the same chooser language, and replace the stale `hyperpush-org/hyperpush-mono` source-build clone URL with the current `snowdamiz/mesh-lang` repo path.
3. Seed `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs` with README + Getting Started assertions for the three starter commands, the honest local/shared-deployable split, the current repo URL, and the required `Clustered Example` before `Production Backend Proof` ordering.
4. Keep the existing clustered/proof next-step markers truthful enough that `reference-backend/scripts/verify-production-proof-surface.sh` stays green without a wording-only exception path.

## Must-Haves

- [ ] `README.md` and `website/docs/docs/getting-started/index.md` now teach the same explicit clustered/SQLite/Postgres starter choice.
- [ ] The source-build fallback uses `https://github.com/snowdamiz/mesh-lang.git` instead of the stale repo.
- [ ] The new first-contact contract fails closed on missing starter commands, stale repo URLs, or Getting Started/proof ordering drift.
- [ ] The production-proof surface verifier still recognizes the clustered-first, proof-secondary path after the rewrite.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `website/docs/docs/getting-started/index.md` + `README.md` | Fail the contract test on missing chooser markers or stale repo URLs instead of accepting partial copy drift. | N/A — source read only. | Reject reordered or missing clustered/proof next-step markers as onboarding drift. |
| `reference-backend/scripts/verify-production-proof-surface.sh` | Keep the script green or fix the page copy in the same task instead of adding a one-off verifier exception. | Stop the task until the failing verifier log is inspectable. | Treat missing `Clustered Example` / `Production Backend Proof` markers as contract drift, not as optional prose changes. |
| `compiler/mesh-pkg/src/scaffold.rs` + example READMEs | Copy from these anchors instead of inventing new starter language. | N/A — source read only. | Reject starter descriptions that contradict the generated scaffold/example truth. |

## Load Profile

- **Shared resources**: the public starter wording shared by `README.md`, `website/docs/docs/getting-started/index.md`, and the generated scaffold/example READMEs.
- **Per-operation cost**: one Markdown rewrite plus one Node source-contract pass and one proof-surface script replay.
- **10x breakpoint**: wording drift across entrypoints breaks before build cost does; stale commands and URLs are the real failure mode.

## Negative Tests

- **Malformed inputs**: unsplit `meshc init --template todo-api`, stale `hyperpush-org/hyperpush-mono` clone URL, or missing starter chooser commands.
- **Error paths**: proof-first lead-in copy survives above the chooser, or Getting Started loses the `Clustered Example` before `Production Backend Proof` ordering.
- **Boundary conditions**: hello-world remains the first executable path and proof pages stay discoverable but secondary after the rewrite.
  - Estimate: 1h30m
  - Files: README.md, website/docs/docs/getting-started/index.md, scripts/tests/verify-m050-s02-first-contact-contract.test.mjs
  - Verify: node --test scripts/tests/verify-m050-s02-first-contact-contract.test.mjs && bash reference-backend/scripts/verify-production-proof-surface.sh
- [x] **T02: Rewrote Clustered Example around scaffold-first truth and moved retained M047 rail discoverability onto the proof page.** — ---
estimated_steps: 12
estimated_files: 6
skills_used:
  - vitepress
  - bash-scripting
  - test
---

`Clustered Example` already contains the real clustered scaffold facts, but it still points at stale repo URLs and spends too much of the first-contact page on retained proof-rail explanation. The retained M047 docs contracts currently freeze some of that density in place. This task rewrites the page around scaffold/example truth and retargets the retained M047 docs rails so the page can stay public, honest, and lighter.

## Steps

1. Use the clustered README template in `compiler/mesh-pkg/src/scaffold.rs` plus `examples/todo-postgres/README.md` / `examples/todo-sqlite/README.md` as the copy source for `website/docs/docs/getting-started/clustered-example/index.md`, keeping the route-free scaffold, CLI inspection order, and SQLite/Postgres split honest.
2. Replace every stale `hyperpush-org/hyperpush-mono` README/runbook link with the current `snowdamiz/mesh-lang` URL and compress the follow-on proof discussion into a bounded closing section instead of a long inline verifier map.
3. Retarget `compiler/meshc/tests/e2e_m047_s05.rs`, `compiler/meshc/tests/e2e_m047_s06.rs`, and any needed `scripts/verify-m047-s06.sh` contract guards so they pin the lighter first-contact Clustered Example markers, current repo links, and proof-page discoverability rather than old helper-name/proof-map prose.
4. Extend `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs` with Clustered Example assertions so the new slice contract covers this page alongside README and Getting Started.

## Must-Haves

- [ ] `website/docs/docs/getting-started/clustered-example/index.md` matches the scaffold/example truth and only keeps proof rails as bounded follow-on guidance.
- [ ] All public example/runbook links on the page use `snowdamiz/mesh-lang` instead of the stale repo identity.
- [ ] Retained M047 docs contracts stop requiring the old proof-map density just to keep first-contact copy green.
- [ ] The first-contact contract now fails closed on Clustered Example repo-link or starter-split drift.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `website/docs/docs/getting-started/clustered-example/index.md` | Reject stale repo links or over-dense proof-map copy instead of silently preserving them. | N/A — source read only. | Treat missing scaffold-first markers or missing follow-on starter links as first-contact drift. |
| Retained M047 docs tests (`compiler/meshc/tests/e2e_m047_s05.rs`, `compiler/meshc/tests/e2e_m047_s06.rs`) | Fail on the first stale proof-map expectation so the docs contract is updated intentionally rather than weakened. | Preserve the failing test log and stop instead of skipping historical rails. | Reject malformed or missing Clustered Example markers as contract drift, not optional wording changes. |
| `scripts/verify-m047-s06.sh` contract guards | Keep the wrapper truthful if its source-level guards need to change with the lighter page copy. | Preserve the phase log and fail closed. | Reject missing guard markers or missing retained bundle pointers as verifier drift. |

## Load Profile

- **Shared resources**: `website/docs/docs/getting-started/clustered-example/index.md`, retained M047 docs contracts, and the shared first-contact contract file from T01.
- **Per-operation cost**: one Markdown rewrite, one shared Node contract pass, and two targeted Rust docs-contract test replays.
- **10x breakpoint**: over-pinned retained docs contracts break before docs build cost does; the main risk is freezing old proof-heavy copy in place.

## Negative Tests

- **Malformed inputs**: stale `hyperpush-org` links, unsplit todo guidance, or old helper-name/proof-map strings surviving in Clustered Example expectations.
- **Error paths**: retained M047 docs rails still demand the old proof-map paragraph, or the page loses the scaffold-first clustered entrypoint.
- **Boundary conditions**: the page still points to `examples/todo-postgres`, `examples/todo-sqlite`, and `reference-backend/README.md`, while proof rails remain discoverable but clearly secondary.
  - Estimate: 1h45m
  - Files: website/docs/docs/getting-started/clustered-example/index.md, compiler/meshc/tests/e2e_m047_s05.rs, compiler/meshc/tests/e2e_m047_s06.rs, scripts/verify-m047-s06.sh, scripts/tests/verify-m050-s02-first-contact-contract.test.mjs
  - Verify: node --test scripts/tests/verify-m050-s02-first-contact-contract.test.mjs && cargo test -p meshc --test e2e_m047_s05 m047_s05_public_clustered_surfaces_use_source_first_names_and_todo_template -- --nocapture && cargo test -p meshc --test e2e_m047_s06 m047_s06_ -- --nocapture
- [ ] **T03: Reorder Tooling and wire the first-contact verifier into the assembled docs replay** — ---
estimated_steps: 14
estimated_files: 8
skills_used:
  - vitepress
  - bash-scripting
  - test
---

`Tooling` is still telling the right facts, but it presents release/proof runbooks before the first-contact CLI workflow and it has no slice-owned verifier proving the rewritten first-contact copy in built HTML. This task rewrites the page around the starter story, adds the S02 verifier/bundle, and wires that verifier into the active assembled M049 replay so copy drift is caught before the heavier retained rails.

## Steps

1. Reorder `website/docs/docs/tooling/index.md` so install/update, package-manager starter choice, cluster inspection order, and editor support stay above release/proof runbooks while preserving the required retained M036/M048/M049 markers and current `snowdamiz/mesh-lang` example/runbook links.
2. Extend `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs` to cover Tooling markers, then add `scripts/verify-m050-s02.sh` to run the first-contact contract, the retained M047 docs tests, the retained M048/M036 tooling contracts, a real `npm --prefix website run build`, and built-HTML assertions/copies for `Getting Started`, `Clustered Example`, and `Tooling` under `.tmp/m050-s02/verify/`. Run docs build serially inside the verifier; do not overlap it with other site builds.
3. Add `compiler/meshc/tests/e2e_m050_s02.rs` to pin the new verifier's phase order, retained bundle shape, and built-site evidence.
4. Update `scripts/verify-m049-s05.sh`, `scripts/tests/verify-m049-s05-contract.test.mjs`, and `compiler/meshc/tests/e2e_m049_s05.rs` so the active assembled wrapper runs `bash scripts/verify-m050-s02.sh` immediately after the S01 graph preflight and before the heavier retained rails.

## Must-Haves

- [ ] `website/docs/docs/tooling/index.md` now reads as first-contact tooling guidance before release/proof runbooks without losing the required M036/M048/M049 markers.
- [ ] `bash scripts/verify-m050-s02.sh` emits standard `.tmp/m050-s02/verify` artifacts and proves both source-level and built-site first-contact copy.
- [ ] `compiler/meshc/tests/e2e_m050_s02.rs` fails closed when the new verifier's phase markers, retained bundle shape, or built HTML evidence drift.
- [ ] `scripts/verify-m049-s05.sh` replays the new S02 verifier before the heavier retained example/docs wrappers.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `website/docs/docs/tooling/index.md` | Keep the page first-contact and marker-complete; fail on missing M036/M048/M049 public markers instead of silently reordering them away. | N/A — source read only. | Reject stale repo links or missing assembled verifier discoverability as contract drift. |
| `scripts/verify-m050-s02.sh` | Fail on the named phase and retain `.tmp/m050-s02/verify` evidence instead of collapsing source/build drift into a generic docs failure. | Mark the timed-out phase failed, keep `current-phase.txt`, and stop before any later wrapper can hide the drift. | Reject missing built HTML snapshots, missing bundle pointers, or missing phase markers as verifier drift. |
| `scripts/verify-m049-s05.sh` + its contract tests | Keep the new S02 preflight in the assembled order or fail the wrapper contracts instead of letting it become an orphan verifier. | Preserve the wrapper logs and stop before the heavier retained rails continue. | Reject missing `bash scripts/verify-m050-s02.sh` markers or wrong replay order as contract drift. |

## Load Profile

- **Shared resources**: `website/docs/docs/tooling/index.md`, the shared first-contact contract file, `.tmp/m050-s02/verify`, the built VitePress HTML tree, and the active M049 wrapper sources.
- **Per-operation cost**: one docs-page rewrite, one Node contract pass, retained M047/M048/M036 source checks, one real site build, built-HTML assertions, and wrapper contract replays.
- **10x breakpoint**: docs builds and retained artifact copying dominate first, so the slice-owned verifier must stay focused and serial instead of spawning overlapping site builds.

## Negative Tests

- **Malformed inputs**: missing `bash scripts/verify-m049-s05.sh` discoverability, stale `hyperpush-org` links, missing editor/tooling markers, or missing built-HTML snapshots.
- **Error paths**: the Tooling page still opens with proof runbooks, the slice verifier omits a retained contract, or the M049 wrapper forgets to run the new S02 preflight.
- **Boundary conditions**: the page still preserves the M036/M048/M049 retained markers, the new verifier stays env-safe, and built HTML for `Getting Started`, `Clustered Example`, and `Tooling` is copied into the retained bundle for diagnosis.
  - Estimate: 2h
  - Files: website/docs/docs/tooling/index.md, scripts/tests/verify-m050-s02-first-contact-contract.test.mjs, scripts/verify-m050-s02.sh, compiler/meshc/tests/e2e_m050_s02.rs, scripts/verify-m049-s05.sh, scripts/tests/verify-m049-s05-contract.test.mjs, compiler/meshc/tests/e2e_m049_s05.rs
  - Verify: node --test scripts/tests/verify-m048-s05-contract.test.mjs && node --test scripts/tests/verify-m036-s03-contract.test.mjs && cargo test -p meshc --test e2e_m050_s02 -- --nocapture && node --test scripts/tests/verify-m049-s05-contract.test.mjs && cargo test -p meshc --test e2e_m049_s05 -- --nocapture && bash scripts/verify-m050-s02.sh && bash scripts/verify-m049-s05.sh
