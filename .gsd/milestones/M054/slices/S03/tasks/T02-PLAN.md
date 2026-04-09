---
estimated_steps: 4
estimated_files: 3
skills_used:
  - bash-scripting
  - test
  - vitepress
---

# T02: Add an S03 docs-contract test and assembled verifier that replays S02

**Slice:** S03 — Public contract and guarded claims
**Milestone:** M054

## Description

Turn the wording into an enforceable surface. The slice only closes if public docs drift is caught cheaply in source, then replayed through one assembled verifier that reuses S02’s proof bundle instead of re-implementing the runtime story.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| delegated `bash scripts/verify-m054-s02.sh` replay | Stop immediately and preserve the copied S02 verify tree; do not claim green docs truth when the underlying starter proof is red. | Treat a hung delegated replay as a real acceptance blocker and surface the failing S02 phase in the S03 bundle. | Fail closed if the delegated verify directory is missing status/pointer markers or if the copied S02 bundle shape drifts. |
| source-contract / built-HTML assertions in the new S03 rails | Reject the change instead of weakening older M050/M054 guards; a missing or moved marker is drift, not a warning. | Bound the VitePress build and HTML inspection phases and keep their logs in the retained bundle. | Fail closed if built HTML still contains stale copy, the OG asset is missing, or the retained bundle leaks unredacted `DATABASE_URL`. |

## Load Profile

- **Shared resources**: `.tmp/m054-s02/verify`, the new `.tmp/m054-s03/verify` bundle, VitePress build output, and copied HTML/OG artifacts.
- **Per-operation cost**: one source-contract test, one Rust verifier-contract test, one delegated S02 replay, one OG regeneration, and one docs build.
- **10x breakpoint**: repeated full wrapper replays and bundle copies dominate before the contract checks themselves become expensive.

## Negative Tests

- **Malformed inputs**: stale homepage tagline markers, stale distributed-proof operator-flow markers, missing built HTML snapshots, missing OG asset, and malformed delegated bundle pointers.
- **Error paths**: S02 delegation missing, built HTML assertion drift, redaction leak, or wrapper phases passing without retaining the proof bundle.
- **Boundary conditions**: homepage and distributed-proof built HTML match the edited source, the S03 wrapper repoints `latest-proof-bundle.txt` to its own retained bundle, and the copied S02 verifier state stays intact.

## Steps

1. Add `scripts/tests/verify-m054-s03-contract.test.mjs` to guard the homepage/config/distributed-proof/OG-generator source markers and stale-marker exclusions without widening older M050 or M054 tests.
2. Add `compiler/meshc/tests/e2e_m054_s03.rs` to archive the same source files, assert the S03 wrapper layering/bundle contract, and keep the verifier behavior under Cargo like the older docs-closeout rails.
3. Add `scripts/verify-m054-s03.sh` so the assembled rail delegates `bash scripts/verify-m054-s02.sh`, runs the new source/Rust contract tests, regenerates the OG asset, builds VitePress, copies homepage/distributed-proof built HTML and the OG asset into a retained bundle, and fail-closes on bundle-shape or redaction drift.
4. Reuse the S02/starter wording oracle and copied verify trees instead of mutating `.tmp/m054-s02/verify` or inventing a second proof story.

## Must-Haves

- [ ] `scripts/tests/verify-m054-s03-contract.test.mjs` catches stale public-copy markers and missing bounded markers in the homepage, VitePress config, distributed-proof page, and OG generator.
- [ ] `compiler/meshc/tests/e2e_m054_s03.rs` pins the S03 verifier layering and retained-bundle contract in a repo-owned Cargo rail.
- [ ] `scripts/verify-m054-s03.sh` delegates S02 unchanged, reruns OG generation + VitePress build, retains built HTML/OG evidence, and republishes its own `latest-proof-bundle.txt`.
- [ ] The retained S03 bundle includes the copied S02 verify tree, built HTML snapshots, OG asset evidence, phase logs, and no unredacted `DATABASE_URL`.

## Verification

- `node --test scripts/tests/verify-m054-s03-contract.test.mjs`
- `cargo test -p meshc --test e2e_m054_s03 -- --nocapture`
- `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} bash scripts/verify-m054-s03.sh`

## Observability Impact

- Signals added/changed: `.tmp/m054-s03/verify/status.txt`, `.tmp/m054-s03/verify/current-phase.txt`, `.tmp/m054-s03/verify/phase-report.txt`, built-HTML assertion summary, and retained `latest-proof-bundle.txt`.
- How a future agent inspects this: start with `.tmp/m054-s03/verify/phase-report.txt`, then open the retained bundle’s copied `retained-m054-s02-verify/` directory and built HTML/OG artifacts.
- Failure state exposed: delegated S02 failure, stale built HTML, missing OG asset, malformed proof-bundle pointer, and redaction drift.

## Inputs

- `website/docs/index.md` — T01’s bounded homepage copy that the new contracts must guard in source and built HTML.
- `website/docs/.vitepress/config.mts` — T01’s default-description/metadata sink that must stay aligned with homepage wording.
- `website/docs/docs/distributed-proof/index.md` — T01’s updated proof-page copy that the built-HTML rail must preserve.
- `website/scripts/generate-og-image.py` — T01’s OG source surface that the new contract must guard.
- `website/docs/public/og-image-v2.png` — regenerated OG asset that the assembled verifier must retain as evidence.
- `scripts/tests/verify-m054-s02-contract.test.mjs` — existing M054 source-contract pattern to extend without widening its responsibility.
- `scripts/verify-m054-s02.sh` — delegated runtime proof rail and retained-bundle copy pattern to reuse unchanged.
- `scripts/verify-m050-s03.sh` — built-HTML retention/assertion pattern for public docs surfaces.

## Expected Output

- `scripts/tests/verify-m054-s03-contract.test.mjs` — dedicated S03 source contract for homepage, distributed-proof, and OG-generator wording.
- `compiler/meshc/tests/e2e_m054_s03.rs` — Cargo-facing verifier-layer contract for the new S03 wrapper and retained bundle.
- `scripts/verify-m054-s03.sh` — assembled S03 replay that delegates S02, rebuilds docs/OG evidence, and republishes a retained proof bundle.
