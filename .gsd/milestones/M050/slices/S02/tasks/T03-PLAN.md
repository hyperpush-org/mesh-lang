---
estimated_steps: 14
estimated_files: 8
skills_used:
  - vitepress
  - bash-scripting
  - test
---

# T03: Reorder Tooling and wire the first-contact verifier into the assembled docs replay

**Slice:** S02 — First-Contact Docs Rewrite
**Milestone:** M050

## Description

`Tooling` is still telling the right facts, but it presents release/proof runbooks before the first-contact CLI workflow and it has no slice-owned verifier proving the rewritten first-contact copy in built HTML. This task rewrites the page around the starter story, adds the S02 verifier/bundle, and wires that verifier into the active assembled M049 replay so copy drift is caught before the heavier retained rails.

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

## Verification

- `node --test scripts/tests/verify-m048-s05-contract.test.mjs`
- `node --test scripts/tests/verify-m036-s03-contract.test.mjs`
- `cargo test -p meshc --test e2e_m050_s02 -- --nocapture`
- `node --test scripts/tests/verify-m049-s05-contract.test.mjs`
- `cargo test -p meshc --test e2e_m049_s05 -- --nocapture`
- `bash scripts/verify-m050-s02.sh`
- `bash scripts/verify-m049-s05.sh`

## Observability Impact

- Signals added/changed: `.tmp/m050-s02/verify/{status.txt,current-phase.txt,phase-report.txt,full-contract.log,latest-proof-bundle.txt}` plus copied built HTML snapshots for `getting-started`, `clustered-example`, and `tooling`.
- How a future agent inspects this: run `bash scripts/verify-m050-s02.sh` or open the retained bundle pointer from `.tmp/m050-s02/verify/latest-proof-bundle.txt`.
- Failure state exposed: the first failing source/build phase and the exact page or contract file that drifted.

## Inputs

- `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs` — shared first-contact source contract from T01/T02 that now needs Tooling coverage.
- `website/docs/docs/tooling/index.md` — current tooling page that still front-loads release/proof runbooks.
- `scripts/tests/verify-m048-s05-contract.test.mjs` — retained tooling markers that must stay present.
- `scripts/tests/verify-m036-s03-contract.test.mjs` — retained editor-support contract that Tooling must not break.
- `website/docs/docs/getting-started/clustered-example/index.md` — rewritten clustered walkthrough that Tooling must reinforce consistently.
- `scripts/verify-m049-s05.sh` — active assembled wrapper that should replay the new S02 verifier.
- `scripts/tests/verify-m049-s05-contract.test.mjs` — source-level order/bundle contract for the assembled wrapper.
- `compiler/meshc/tests/e2e_m049_s05.rs` — Rust-side wrapper contract that must acknowledge the new preflight.

## Expected Output

- `website/docs/docs/tooling/index.md` — Tooling rewritten around the first-contact starter story while preserving retained markers.
- `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs` — first-contact source contract extended with Tooling assertions.
- `scripts/verify-m050-s02.sh` — slice-owned source + built-site verifier for first-contact docs truth.
- `compiler/meshc/tests/e2e_m050_s02.rs` — Rust contract for the new slice verifier.
- `scripts/verify-m049-s05.sh` — active assembled wrapper updated to replay the S02 verifier early.
- `scripts/tests/verify-m049-s05-contract.test.mjs` — wrapper source contract updated for the new preflight order.
- `compiler/meshc/tests/e2e_m049_s05.rs` — Rust wrapper contract updated for the new preflight order.
- `.tmp/m050-s02/verify/latest-proof-bundle.txt` — retained pointer to the slice verifier evidence bundle.
