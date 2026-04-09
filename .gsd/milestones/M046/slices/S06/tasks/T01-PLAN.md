---
estimated_steps: 4
estimated_files: 3
skills_used:
  - rust-testing
  - test
---

# T01: Pin the S06 closeout hierarchy in Rust contract tests

Add Rust-side content guards for the new S06 proof hierarchy before the scripts and docs move, so stale S05-authoritative assumptions fail closed instead of surviving until the shell verifier runs.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `compiler/meshc/tests/e2e_m046_s06.rs` new contract rail | Fail if the authoritative S06 script, targeted replay phases, retained bundle names, or docs references are missing. | N/A | Treat mixed S05/S06 authority claims or missing bundle markers as contract drift. |
| `compiler/meshc/tests/e2e_m045_s05.rs` historical alias guard | Fail if the M045 wrapper still points at S05 or if it reasserts direct docs/build work instead of delegation. | N/A | Treat missing retained verify files or stale delegated phase names as alias drift. |
| `compiler/meshc/tests/e2e_m046_s05.rs` equal-surface subrail guard | Fail if S05 still claims final authority or if demotion accidentally deletes the retained S05 bundle contract that S06 wraps. | N/A | Treat ambiguous authoritative/historical wording as a regression. |

## Negative Tests

- **Malformed inputs**: missing `latest-proof-bundle.txt`, missing `retained-m046-s05-verify`, missing `retained-m046-s06-artifacts`, or mismatched phase labels.
- **Error paths**: S05 still named authoritative in wrapper/document contracts, `verify-m045-s05.sh` delegating to the wrong script, or docs/readmes still pointing at S05 as present-tense truth.
- **Boundary conditions**: S05 may remain a lower-level equal-surface subrail, but the tests must clearly distinguish authoritative S06 from delegated or historical rails.

## Steps

1. Add `compiler/meshc/tests/e2e_m046_s06.rs` as a pure contract/content guard that asserts the new S06 verifier phases, retained bundle names, targeted S03/S04 truth replays, and public docs references without duplicating the runtime harness.
2. Update `compiler/meshc/tests/e2e_m045_s05.rs` so the historical wrapper test expects delegation to `scripts/verify-m046-s06.sh`, the S06 retained verify directory, and the S06 phase/bundle contract instead of the S05 hierarchy.
3. Update `compiler/meshc/tests/e2e_m046_s05.rs` so S05 stays pinned as the equal-surface subrail S06 wraps, not the final authoritative closeout rail.
4. Keep the assertions focused on script/doc/bundle contract only; do not fork a second startup/failover runtime proof into the S06 test file.

## Must-Haves

- [ ] `compiler/meshc/tests/e2e_m046_s06.rs` exists and pins the S06 verifier hierarchy, phase names, and retained bundle shape.
- [ ] `compiler/meshc/tests/e2e_m045_s05.rs` fails closed if the historical wrapper does not delegate to S06.
- [ ] `compiler/meshc/tests/e2e_m046_s05.rs` still protects the S05 equal-surface subrail but no longer claims final authority.
- [ ] Rust content guards cover authoritative/historical doc references so stale S05 wording fails before the shell verifier runs.

## Done When

- [ ] `cargo test -p meshc --test e2e_m046_s06 m046_s06_ -- --nocapture` passes.
- [ ] `cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture` passes against the repointed hierarchy.

## Inputs

- `compiler/meshc/tests/e2e_m046_s05.rs`
- `compiler/meshc/tests/e2e_m045_s05.rs`
- `scripts/verify-m046-s05.sh`
- `scripts/verify-m045-s05.sh`
- `README.md`
- `website/docs/docs/distributed-proof/index.md`

## Expected Output

- `compiler/meshc/tests/e2e_m046_s06.rs`
- `compiler/meshc/tests/e2e_m045_s05.rs`
- `compiler/meshc/tests/e2e_m046_s05.rs`

## Verification

cargo test -p meshc --test e2e_m046_s06 m046_s06_ -- --nocapture && cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture
