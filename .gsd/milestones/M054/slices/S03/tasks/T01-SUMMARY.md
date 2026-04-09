---
id: T01
parent: S03
milestone: M054
provides: []
requires: []
affects: []
key_files: ["website/docs/index.md", "website/docs/.vitepress/config.mts", "website/docs/docs/distributed-proof/index.md", "website/scripts/generate-og-image.py", "website/docs/public/og-image-v2.png", "scripts/tests/verify-m054-s03-contract.test.mjs", "compiler/meshc/tests/e2e_m054_s03.rs", "scripts/verify-m054-s03.sh", ".gsd/milestones/M054/slices/S03/tasks/T01-SUMMARY.md"]
key_decisions: ["Reused the serious starter’s one-public-URL/runtime-owned-placement vocabulary for homepage metadata and OG copy instead of inventing a second site-level tagline.", "Split the Distributed Proof operator flow into direct `X-Mesh-Continuity-Request-Key` lookup for clustered HTTP and continuity-list discovery for startup/manual inspection.", "Made the S03 shell wrapper retain copied built HTML, the regenerated OG asset, and the delegated S02 verify tree so failures can be debugged from a self-contained bundle."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the task-owned copy with `npm --prefix website run generate:og` and `npm --prefix website run build`, then checked the built homepage and Distributed Proof HTML for the new bounded markers and the absence of the stale generic tagline. Ran the new slice guard rails with `node --test scripts/tests/verify-m054-s03-contract.test.mjs`, `cargo test -p meshc --test e2e_m054_s03 -- --nocapture`, and `DATABASE_URL=<redacted> bash scripts/verify-m054-s03.sh` against a disposable local Docker Postgres; the assembled replay finished green and left `.tmp/m054-s03/verify/status.txt=ok`, `.tmp/m054-s03/verify/current-phase.txt=complete`, a fully passed phase report, and a retained proof bundle pointer."
completed_at: 2026-04-06T16:30:48.675Z
blocker_discovered: false
---

# T01: Aligned the public docs and OG copy to the one-public-URL runtime-owned placement story, and added S03 drift guards.

> Aligned the public docs and OG copy to the one-public-URL runtime-owned placement story, and added S03 drift guards.

## What Happened
---
id: T01
parent: S03
milestone: M054
key_files:
  - website/docs/index.md
  - website/docs/.vitepress/config.mts
  - website/docs/docs/distributed-proof/index.md
  - website/scripts/generate-og-image.py
  - website/docs/public/og-image-v2.png
  - scripts/tests/verify-m054-s03-contract.test.mjs
  - compiler/meshc/tests/e2e_m054_s03.rs
  - scripts/verify-m054-s03.sh
  - .gsd/milestones/M054/slices/S03/tasks/T01-SUMMARY.md
key_decisions:
  - Reused the serious starter’s one-public-URL/runtime-owned-placement vocabulary for homepage metadata and OG copy instead of inventing a second site-level tagline.
  - Split the Distributed Proof operator flow into direct `X-Mesh-Continuity-Request-Key` lookup for clustered HTTP and continuity-list discovery for startup/manual inspection.
  - Made the S03 shell wrapper retain copied built HTML, the regenerated OG asset, and the delegated S02 verify tree so failures can be debugged from a self-contained bundle.
duration: ""
verification_result: passed
completed_at: 2026-04-06T16:30:48.676Z
blocker_discovered: false
---

# T01: Aligned the public docs and OG copy to the one-public-URL runtime-owned placement story, and added S03 drift guards.

**Aligned the public docs and OG copy to the one-public-URL runtime-owned placement story, and added S03 drift guards.**

## What Happened

Replaced the stale homepage/site metadata copy with the bounded one-public-URL/runtime-owned-placement story already used by the serious PostgreSQL starter, rewrote Distributed Proof to explain the proxy-ingress vs Mesh-runtime boundary plus the direct request-key lookup flow, and updated the OG generator + rendered asset to match. Because the slice-level verification surfaces were missing locally, this task also added the S03 Node contract, Rust verifier contract, and assembled shell wrapper so the docs/asset story can fail closed from copied evidence instead of relying on live local state.

## Verification

Verified the task-owned copy with `npm --prefix website run generate:og` and `npm --prefix website run build`, then checked the built homepage and Distributed Proof HTML for the new bounded markers and the absence of the stale generic tagline. Ran the new slice guard rails with `node --test scripts/tests/verify-m054-s03-contract.test.mjs`, `cargo test -p meshc --test e2e_m054_s03 -- --nocapture`, and `DATABASE_URL=<redacted> bash scripts/verify-m054-s03.sh` against a disposable local Docker Postgres; the assembled replay finished green and left `.tmp/m054-s03/verify/status.txt=ok`, `.tmp/m054-s03/verify/current-phase.txt=complete`, a fully passed phase report, and a retained proof bundle pointer.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix website run generate:og` | 0 | ✅ pass | 2583ms |
| 2 | `npm --prefix website run build` | 0 | ✅ pass | 32137ms |
| 3 | `python3 built-artifact existence check for website/docs/.vitepress/dist/{index,docs/distributed-proof/index}.html and website/docs/public/og-image-v2.png` | 0 | ✅ pass | 165ms |
| 4 | `python3 built-HTML contract check for homepage/proof markers and stale-text absence` | 0 | ✅ pass | 171ms |
| 5 | `node --test scripts/tests/verify-m054-s03-contract.test.mjs` | 0 | ✅ pass | 933ms |
| 6 | `cargo test -p meshc --test e2e_m054_s03 -- --nocapture` | 0 | ✅ pass | 8064ms |
| 7 | `DATABASE_URL=<redacted> bash scripts/verify-m054-s03.sh` | 0 | ✅ pass | 95700ms |


## Deviations

Pulled the planned S03 guard files (`scripts/tests/verify-m054-s03-contract.test.mjs`, `compiler/meshc/tests/e2e_m054_s03.rs`, and `scripts/verify-m054-s03.sh`) into T01 because the first-task verification surfaces were missing locally and the slice verification commands needed real fail-closed implementations.

## Known Issues

None.

## Files Created/Modified

- `website/docs/index.md`
- `website/docs/.vitepress/config.mts`
- `website/docs/docs/distributed-proof/index.md`
- `website/scripts/generate-og-image.py`
- `website/docs/public/og-image-v2.png`
- `scripts/tests/verify-m054-s03-contract.test.mjs`
- `compiler/meshc/tests/e2e_m054_s03.rs`
- `scripts/verify-m054-s03.sh`
- `.gsd/milestones/M054/slices/S03/tasks/T01-SUMMARY.md`


## Deviations
Pulled the planned S03 guard files (`scripts/tests/verify-m054-s03-contract.test.mjs`, `compiler/meshc/tests/e2e_m054_s03.rs`, and `scripts/verify-m054-s03.sh`) into T01 because the first-task verification surfaces were missing locally and the slice verification commands needed real fail-closed implementations.

## Known Issues
None.
