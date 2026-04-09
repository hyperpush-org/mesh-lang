---
estimated_steps: 12
estimated_files: 6
skills_used:
  - vitepress
  - bash-scripting
  - test
---

# T02: Align Clustered Example to scaffold truth and retarget retained M047 docs rails

**Slice:** S02 — First-Contact Docs Rewrite
**Milestone:** M050

## Description

`Clustered Example` already contains the real clustered scaffold facts, but it still points at stale repo URLs and spends too much of the first-contact page on retained proof-rail explanation. The retained M047 docs contracts currently freeze some of that density in place. This task rewrites the page around scaffold/example truth and retargets the retained M047 docs rails so the page can stay public, honest, and lighter.

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

## Verification

- `node --test scripts/tests/verify-m050-s02-first-contact-contract.test.mjs`
- `cargo test -p meshc --test e2e_m047_s05 m047_s05_public_clustered_surfaces_use_source_first_names_and_todo_template -- --nocapture`
- `cargo test -p meshc --test e2e_m047_s06 m047_s06_ -- --nocapture`

## Observability Impact

- Signals added/changed: retained M047 docs tests now fail on repo-link drift, starter-split drift, or proof-map regression instead of stale dense copy.
- How a future agent inspects this: rerun the targeted M047 Rust tests and inspect `scripts/verify-m047-s06.sh` contract-guard output if the wrapper later fails.
- Failure state exposed: stale repo URLs, missing scaffold-first markers, or a proof rail re-expanding into first-contact copy.

## Inputs

- `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs` — README/Get Started contract from T01 that now needs Clustered Example coverage.
- `website/docs/docs/getting-started/clustered-example/index.md` — current clustered walkthrough to rewrite.
- `compiler/mesh-pkg/src/scaffold.rs` — clustered scaffold README template that should anchor public wording and repo links.
- `examples/todo-postgres/README.md` — serious shared/deployable starter wording to mirror.
- `examples/todo-sqlite/README.md` — honest local starter wording to mirror.
- `compiler/meshc/tests/e2e_m047_s05.rs` — retained public-surface contract that currently over-pins old copy.
- `compiler/meshc/tests/e2e_m047_s06.rs` — retained docs contract that must keep the lighter page truthful.
- `scripts/verify-m047-s06.sh` — retained shell verifier whose contract guards must match the new first-contact markers.

## Expected Output

- `website/docs/docs/getting-started/clustered-example/index.md` — clustered walkthrough rewritten around scaffold truth and current repo links.
- `compiler/meshc/tests/e2e_m047_s05.rs` — retained Clustered Example contract updated to the lighter public markers.
- `compiler/meshc/tests/e2e_m047_s06.rs` — retained docs split contract updated for the rewritten first-contact page.
- `scripts/verify-m047-s06.sh` — wrapper guardrails kept consistent with the updated docs contracts.
- `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs` — first-contact contract extended with Clustered Example assertions.
