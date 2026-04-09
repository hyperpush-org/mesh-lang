---
id: S03
parent: M054
milestone: M054
provides:
  - A fail-closed public docs/metadata/OG contract for the serious Postgres starter’s bounded one-public-URL load-balancing story.
requires:
  - slice: S01
    provides: One-public-URL ingress truth, bounded serious-starter copy, and the retained S01 public-ingress proof bundle.
  - slice: S02
    provides: Runtime-owned `X-Mesh-Continuity-Request-Key` correlation, direct continuity lookup flow, and the delegated S02 verify bundle reused by the S03 wrapper.
affects:
  []
key_files:
  - website/docs/index.md
  - website/docs/.vitepress/config.mts
  - website/docs/docs/distributed-proof/index.md
  - website/scripts/generate-og-image.py
  - website/docs/public/og-image-v2.png
  - scripts/tests/verify-m054-s03-contract.test.mjs
  - compiler/meshc/tests/e2e_m054_s03.rs
  - scripts/verify-m054-s03.sh
  - .gsd/PROJECT.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Keep S03 scoped to VitePress homepage metadata, Distributed Proof, OG asset sync, and repo-owned docs/verifier guardrails; do not pull `mesher/landing` into the closeout surface.
  - Keep exact prose markers in the fast Node source-contract test, while the shell wrapper proves built HTML fragments, retained-bundle shape, delegated S02 replay, and redaction drift.
patterns_established:
  - Keep exact public-copy markers in a fast Node source-contract rail, then let the heavier shell wrapper check only rendered HTML fragments, retained-bundle shape, and redaction.
  - Assembled docs closeout verifiers should delegate earlier proof rails by copying their entire verify directories unchanged and republishing a fresh `latest-proof-bundle.txt` pointer.
  - Treat the public load-balancing model as bounded: one public app URL may choose ingress, runtime placement begins after ingress, and request-key direct lookup is the operator/debug seam.
observability_surfaces:
  - `.tmp/m054-s03/verify/{status.txt,current-phase.txt,phase-report.txt}` expose the assembled verifier’s phase-level truth.
  - `.tmp/m054-s03/verify/built-html-summary.json` records the built homepage/proof HTML contract checks that must stay true after VitePress render.
  - `.tmp/m054-s03/proof-bundles/retained-public-docs-proof-1775494129741439000/retained-m054-s02-verify/` preserves the delegated S02 verify tree and bundle pointer for downstream debugging.
drill_down_paths:
  - .gsd/milestones/M054/slices/S03/tasks/T01-SUMMARY.md
  - .gsd/milestones/M054/slices/S03/tasks/T02-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-06T16:52:36.575Z
blocker_discovered: false
---

# S03: Public contract and guarded claims

**Homepage metadata, Distributed Proof, the OG asset, and the assembled docs verifier now all tell the same bounded one-public-URL/server-side runtime-placement story and fail closed on drift.**

## What Happened

T01 aligned the evaluator-facing docs and social-preview surfaces to the serious starter’s already-proven boundary instead of leaving homepage metadata and proof pages on an older generic load-balancing claim. `website/docs/index.md` and `website/docs/.vitepress/config.mts` now share the same bounded description string: one public app URL may front multiple Mesh nodes, runtime placement stays server-side, and operator truth stays on `meshc cluster`. `website/docs/docs/distributed-proof/index.md` now explains where proxy/platform ingress ends, where Mesh runtime placement begins, when to use `X-Mesh-Continuity-Request-Key` for direct `meshc cluster continuity <node> <request-key> --json` lookup, and when continuity-list discovery still matters for startup/manual inspection. The OG generator source and rendered `website/docs/public/og-image-v2.png` were updated to carry that same contract so the public story stays consistent across homepage, docs, and social-preview copy.

T02 turned that wording into a fail-closed proof surface. `scripts/tests/verify-m054-s03-contract.test.mjs` owns the exact source-copy markers and stale-marker exclusions for homepage metadata, VitePress defaults, Distributed Proof, and OG generator text. `compiler/meshc/tests/e2e_m054_s03.rs` pins the wrapper layering, required phase markers, retained bundle contents, and redaction guarantees under Cargo. `scripts/verify-m054-s03.sh` now replays `bash scripts/verify-m054-s02.sh`, reruns OG generation and VitePress build, asserts the built homepage and Distributed Proof HTML fragments, copies the delegated S02 verify tree plus source/build artifacts into a fresh retained bundle, republishes its own `latest-proof-bundle.txt`, and scans that retained bundle for `DATABASE_URL` leakage.

Together, the slice closes the public-contract layer of R123. S01 already proved one-public-URL ingress truth for the serious starter. S02 already proved direct request-key follow-through to one continuity record. S03 makes the public homepage/docs/OG surfaces say exactly that shipped story—and nothing more—while retaining one auditable `.tmp/m054-s03/proof-bundles/retained-public-docs-proof-1775494129741439000` bundle that downstream slices can inspect instead of re-deriving the contract from source alone.

## Verification

Verified the slice at all declared seams:
- `node --test scripts/tests/verify-m054-s03-contract.test.mjs` passed (3/3) and proved the source-copy markers plus stale-marker exclusions.
- `cargo test -p meshc --test e2e_m054_s03 -- --nocapture` passed (4 tests) and proved the shell-wrapper layering, phase markers, retained-bundle contract, and redaction expectations.
- `npm --prefix website run generate:og` regenerated `website/docs/public/og-image-v2.png` successfully.
- `npm --prefix website run build` completed successfully and emitted built homepage/proof HTML.
- `DATABASE_URL=<redacted> bash scripts/verify-m054-s03.sh` passed end to end against a disposable local Docker Postgres admin URL, replayed `bash scripts/verify-m054-s02.sh`, and published `.tmp/m054-s03/proof-bundles/retained-public-docs-proof-1775494129741439000`.
- `.tmp/m054-s03/verify/built-html-summary.json` recorded all built-site checks as `true` for homepage description presence/old-tagline absence plus Distributed Proof boundary/header-lookup/list-first/non-goal markers.
- `.tmp/m054-s03/verify/phase-report.txt` shows every S03 phase passed, including delegated S02 replay, built HTML assertions, retained bundle copy, redaction drift, and final bundle-shape validation.

## Requirements Advanced

- R123 — Aligned homepage metadata, VitePress defaults, Distributed Proof, and OG copy to the proven server-side runtime-placement story, then added repo-owned drift guards and a retained built-site proof bundle.

## Requirements Validated

- R123 — Validated by `bash scripts/verify-m054-s01.sh`, `bash scripts/verify-m054-s02.sh`, `node --test scripts/tests/verify-m054-s03-contract.test.mjs`, `cargo test -p meshc --test e2e_m054_s03 -- --nocapture`, `npm --prefix website run generate:og`, `npm --prefix website run build`, and `DATABASE_URL=<redacted> bash scripts/verify-m054-s03.sh`, which together prove one-public-URL ingress truth, runtime-owned request-key follow-through, bounded public copy, retained built HTML/OG evidence, and fail-closed redaction/bundle-shape guards.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

None.

## Known Limitations

- The public contract remains intentionally bounded: this slice still does **not** claim sticky sessions, frontend-aware routing, client-visible topology, or Fly-as-product parity.
- `scripts/verify-m054-s03.sh` still depends on a valid Postgres `DATABASE_URL` because it delegates the staged starter proof from S02; docs-only edits do not bypass that runtime-proof dependency.
- The current GSD requirements DB does not know about `R123`, so `gsd_requirement_update` still fails with `Requirement R123 not found` even though the checked-in `.gsd/REQUIREMENTS.md` renders it. Until that DB mismatch is repaired, the visible truth is the checked-in requirements file plus D427 and this slice summary.

## Follow-ups

- Repair the GSD requirements DB entry for `R123` so DB-backed status projections match the checked-in `.gsd/REQUIREMENTS.md` contract.
- If later copy changes the bounded load-balancing story, update `scripts/tests/verify-m054-s03-contract.test.mjs`, `compiler/meshc/tests/e2e_m054_s03.rs`, and `scripts/verify-m054-s03.sh` together instead of broadening only the shell wrapper.
- Keep any `mesher/landing` wording reset on its own verified rail rather than piggybacking this docs-closeout surface.

## Files Created/Modified

- `website/docs/index.md` — Replaced the homepage frontmatter description with the bounded one-public-URL/server-side runtime-placement contract.
- `website/docs/.vitepress/config.mts` — Aligned default VitePress metadata and social-image alt text with the same bounded public contract.
- `website/docs/docs/distributed-proof/index.md` — Rewrote Distributed Proof around the ingress/runtime boundary, direct request-key lookup flow, bounded non-goals, and the M053 starter verifier chain.
- `website/scripts/generate-og-image.py` — Updated the OG generator subtitle/badges to the bounded clustered-public story and regenerated the rendered asset.
- `website/docs/public/og-image-v2.png` — Regenerated the public OG image asset used by the docs site and social previews.
- `scripts/tests/verify-m054-s03-contract.test.mjs` — Added the fast source-contract test that fail-closes on stale homepage/proof/OG wording.
- `compiler/meshc/tests/e2e_m054_s03.rs` — Added the Cargo rail that pins the S03 shell-wrapper layering, retained-bundle contract, and redaction expectations.
- `scripts/verify-m054-s03.sh` — Added the assembled S03 verifier that replays S02, rebuilds docs/OG output, retains built HTML plus delegated proof state, and scans the bundle for secret leaks.
- `.gsd/PROJECT.md` — Updated the living project state to include the completed S03 public-contract guardrails.
- `.gsd/KNOWLEDGE.md` — Recorded the R123 requirements-DB mismatch so future closers do not waste time trying the same failing DB update.
