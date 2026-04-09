---
estimated_steps: 3
estimated_files: 2
skills_used:
  - test
---

# T04: Point the language-owned backend proof page at the product-owned Mesher contract

**Slice:** S02 — Hyperpush Toolchain Contract Outside `mesh-lang`
**Milestone:** M055

## Description

Finish the slice by updating the public-secondary handoff instead of the public first-contact path. This task should keep `production-backend-proof` scaffold/examples-first, but make it route maintainers to the product-owned Mesher runbook/verifier contract and treat the repo-root `mesh-lang` verifier as compatibility-only.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `website/docs/docs/production-backend-proof/index.md` markers | Fail closed on stale repo-root Mesher commands, missing product-owned handoff markers, or lost examples-first ordering. | N/A for source assertions. | Treat mixed public-vs-maintainer routing or stale blob links as docs drift. |
| `scripts/verify-production-proof-surface.sh` | Stop on the first ordering or marker regression and preserve the failing description. | Bounded local shell verifier only. | Treat missing or malformed exact-string checks as proof-surface drift. |
| S01 split-boundary contract | Fail if the updated docs now contradict `WORKSPACE.md` about language-owned versus product-owned surfaces. | Bounded local shell verifier only. | Treat repo-ownership wording drift as a real split-contract regression. |

## Negative Tests

- **Malformed inputs**: public docs that promote repo-root Mesher commands again, missing compatibility-wrapper wording, or loss of the scaffold/examples-first intro.
- **Error paths**: the proof page points at the product-owned Mesher contract but drops the retained backend-only verifier or the failure-inspection map.
- **Boundary conditions**: the page stays public-secondary, `Production Backend Proof` remains out of the footer chain, and S01’s split-boundary verifier stays green after the handoff text changes.

## Steps

1. Update `website/docs/docs/production-backend-proof/index.md` so the deeper-app handoff points at `mesher/README.md` and `bash mesher/scripts/verify-maintainer-surface.sh`, while `bash scripts/verify-m051-s01.sh` is described as the mesh-lang compatibility wrapper.
2. Update `scripts/verify-production-proof-surface.sh` exact-marker and ordering checks to enforce that new handoff and forbid repo-root Mesher commands as the primary story.
3. Re-run the split-boundary contract through `bash scripts/verify-m055-s01.sh` so product-owned versus language-owned wording stays aligned with `WORKSPACE.md`.

## Must-Haves

- [ ] `production-backend-proof` stays examples-first and public-secondary.
- [ ] The deeper Mesher handoff now points at the product-owned runbook/verifier contract.
- [ ] The repo-root Mesher verifier is clearly compatibility-only in the public-secondary docs.

## Verification

- `bash scripts/verify-production-proof-surface.sh`
- `bash scripts/verify-m055-s01.sh`

## Inputs

- `website/docs/docs/production-backend-proof/index.md` — current language-owned backend handoff page.
- `scripts/verify-production-proof-surface.sh` — exact-marker verifier for the handoff page.
- `WORKSPACE.md` — split-boundary ownership contract from S01.
- `mesher/README.md` — product-owned Mesher runbook from T03.
- `mesher/scripts/verify-maintainer-surface.sh` — product-owned verifier from T02.
- `scripts/verify-m051-s01.sh` — compatibility wrapper from T02.

## Expected Output

- `website/docs/docs/production-backend-proof/index.md` — public-secondary docs page aligned to the product-owned Mesher contract.
- `scripts/verify-production-proof-surface.sh` — proof-page contract rail aligned to the new handoff wording.
