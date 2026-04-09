# S06: Assembled verification and docs closeout — UAT

**Milestone:** M046
**Written:** 2026-04-01T03:57:59.668Z

## UAT Type

- UAT mode: assembled verifier closeout + docs/runbook authority checks + milestone-validation proof
- Why this mode is sufficient: S06 does not introduce a new runtime subsystem; it closes M046 by assembling the already-shipped route-free proofs into one authoritative verifier, one public docs story, and one milestone-validation artifact. Acceptance therefore has to prove authority/delegation contracts, docs/runbook truth, retained-bundle shape, and final validation state together.

## Preconditions

- Run from the repository root with Cargo, Node/npm, Python 3, and the repo toolchain already available.
- `.tmp/m046-s03/`, `.tmp/m046-s04/`, `.tmp/m046-s05/`, and `.tmp/m046-s06/` must be writable.
- Loopback ports used by the underlying S03/S04/S05 runtime rails must be free.
- No stale verifier run should be mutating `.tmp/m046-s06/verify/` while acceptance is in progress.
- Run `bash scripts/verify-m046-s06.sh` before `bash scripts/verify-m045-s05.sh`; the historical wrapper delegates back into S06 and both scripts rewrite the same S06 verify tree.

## Smoke Test

Run:

```bash
cargo test -p meshc --test e2e_m046_s06 m046_s06_ -- --nocapture
cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture
```

**Expected:** both Rust contract suites pass, proving the authoritative S06 verifier/doc contract and the historical M045 wrapper alias contract before any long replay work starts.

## Test Cases

### 1. Rust contract rails pin S06 as the only present-tense closeout seam

1. Run:
   ```bash
   cargo test -p meshc --test e2e_m046_s06 m046_s06_ -- --nocapture
   cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture
   ```
2. **Expected:**
   - `e2e_m046_s06` runs non-zero tests and passes.
   - `e2e_m045_s05` runs non-zero tests and passes.
   - `scripts/verify-m046-s06.sh` is pinned to the exact S06 phase set: `m046-s05-replay`, `retain-m046-s05-verify`, `m046-s03-startup-truth`, `m046-s03-failover-truth`, `m046-s04-package-startup-truth`, `m046-s06-artifacts`, and `m046-s06-bundle-shape`.
   - `scripts/verify-m045-s05.sh` is pinned to delegation into S06 and to retaining `retained-m046-s06-verify/`, not to replaying S05 directly.
   - `compiler/meshc/tests/e2e_m046_s05.rs` still protects S05 as the equal-surface subrail, but no touched contract rail claims S05 is the final closeout authority.

### 2. Public clustered docs and package runbooks point to S06 and stay route-free

1. Run:
   ```bash
   npm --prefix website run build
   ! rg -n "\[cluster\]|Continuity\.submit_declared_work|/health|/work/:request_key|Timer\.sleep\(5000\)" README.md website/docs/docs/distributed-proof/index.md website/docs/docs/distributed/index.md website/docs/docs/tooling/index.md website/docs/docs/getting-started/clustered-example/index.md tiny-cluster/README.md cluster-proof/README.md
   rg -q 'scripts/verify-m046-s06\.sh' README.md website/docs/docs/distributed-proof/index.md website/docs/docs/distributed/index.md website/docs/docs/tooling/index.md website/docs/docs/getting-started/clustered-example/index.md tiny-cluster/README.md cluster-proof/README.md
   ```
2. **Expected:**
   - The VitePress docs build passes.
   - The touched clustered docs/runbook surfaces omit routeful/operator-drift markers.
   - The same surfaces mention `scripts/verify-m046-s06.sh` as the authoritative closeout rail.
   - S05 can still be mentioned, but only as the lower equal-surface subrail; M045 can still be mentioned, but only as the historical alias wrapper.

### 3. The S06 assembled verifier republishes one milestone-level retained bundle

1. Run:
   ```bash
   bash scripts/verify-m046-s06.sh
   ```
2. **Expected:**
   - The script prints `verify-m046-s06: ok` and exits successfully.
   - `.tmp/m046-s06/verify/status.txt` contains `ok`.
   - `.tmp/m046-s06/verify/current-phase.txt` contains `complete`.
   - `.tmp/m046-s06/verify/phase-report.txt` contains passed markers for:
     - `m046-s05-replay`
     - `retain-m046-s05-verify`
     - `m046-s06-artifacts`
     - `m046-s03-startup-truth`
     - `m046-s03-failover-truth`
     - `m046-s04-package-startup-truth`
     - `m046-s06-bundle-shape`
   - `.tmp/m046-s06/verify/latest-proof-bundle.txt` points at `.tmp/m046-s06/verify/retained-m046-s06-artifacts`.
   - That retained bundle contains:
     - `retained-m046-s05-equal-surface/`
     - `retained-m046-s03-startup/`
     - `retained-m046-s03-failover/`
     - `retained-m046-s04-package-startup/`
   - The S05 retained subtree still contains copied `retained-m046-s03-artifacts/`, `retained-m046-s04-artifacts/`, and `retained-m046-s05-artifacts/` members.

### 4. The historical M045 wrapper passes only by delegating to S06

1. Run:
   ```bash
   bash scripts/verify-m045-s05.sh
   ```
2. **Expected:**
   - The script prints `verify-m045-s05: ok` and exits successfully.
   - `.tmp/m045-s05/verify/status.txt` contains `ok`.
   - `.tmp/m045-s05/verify/current-phase.txt` contains `complete`.
   - `.tmp/m045-s05/verify/latest-proof-bundle.txt` points at `.tmp/m045-s05/verify/retained-m046-s06-verify/retained-m046-s06-artifacts`.
   - `.tmp/m045-s05/verify/retained-m046-s06-verify/` contains copied `status.txt`, `current-phase.txt`, `phase-report.txt`, `full-contract.log`, and `latest-proof-bundle.txt` from the delegated S06 run.
   - The wrapper does not run docs builds, scaffold-unit checks, or a second direct S05 replay of its own beyond delegating into S06.

### 5. Milestone validation and project state are anchored in the S06 rail

1. Run:
   ```bash
   test -s .gsd/milestones/M046/M046-VALIDATION.md
   rg -n "verify-m046-s06|R086|R091|R092" .gsd/milestones/M046/M046-VALIDATION.md .gsd/PROJECT.md
   ```
2. **Expected:**
   - `.gsd/milestones/M046/M046-VALIDATION.md` exists and is non-empty.
   - The validation artifact cites `verify-m046-s06` and explicitly covers the required M046 requirements.
   - `.gsd/PROJECT.md` points future agents at the S06 closeout rail and retained bundle instead of the older S05-only pointer.

## Edge Cases

### Run the S06 and M045 wrapper rails serially, never in parallel

1. Start a fresh S06 run.
2. Wait for it to finish before starting the M045 wrapper.
3. **Expected:** both rails pass. Running them concurrently is invalid because both mutate `.tmp/m046-s06/verify/`.

### Missing retained bundles must fail the assembled closeout rail immediately

1. If `bash scripts/verify-m046-s06.sh` fails, inspect `.tmp/m046-s06/verify/phase-report.txt`, `.tmp/m046-s06/verify/full-contract.log`, and `.tmp/m046-s06/verify/latest-proof-bundle.txt`.
2. **Expected:** missing delegated S05 verify state, a malformed `latest-proof-bundle.txt`, or an absent retained S03/S04 subtree is treated as verifier failure, not as a warning.

### Historical wording may remain, but only as explicitly demoted context

1. Inspect the updated README/docs/runbook surfaces after the build passes.
2. **Expected:** references to S05 or M045 are allowed only when they are clearly labeled lower-level equal-surface or historical alias rails. No surface should present S05 as the final authoritative closeout seam.

## Failure Signals

- `e2e_m046_s06` or `e2e_m045_s05` runs 0 tests or fails to find the expected verifier/doc contract markers.
- Any touched clustered doc/runbook surface reintroduces `[cluster]`, `Continuity.submit_declared_work`, `/health`, `/work/:request_key`, or `Timer.sleep(5000)`.
- `bash scripts/verify-m046-s06.sh` leaves `status.txt != ok`, `current-phase.txt != complete`, or a phase report missing one of the required pass markers.
- `latest-proof-bundle.txt` is empty, malformed, or points at a bundle missing `retained-m046-s05-equal-surface`, `retained-m046-s03-startup`, `retained-m046-s03-failover`, or `retained-m046-s04-package-startup`.
- `bash scripts/verify-m045-s05.sh` goes green without retaining the copied S06 verify tree.
- `.gsd/milestones/M046/M046-VALIDATION.md` stops citing `verify-m046-s06` or loses the explicit requirement coverage markers for R086/R091/R092.

## Requirements Proved By This UAT

- R086 — The final assembled closeout rail proves the runtime/tooling-owned clustered-work contract end to end instead of leaving any app-owned clustered control plane in the proof surfaces.
- R087 — Startup-triggered clustered work remains runtime/tooling owned with no app-side HTTP or explicit continuity submission seam.
- R088 — `tiny-cluster/` remains a real local route-free proof and is freshly replayed in the final bundle.
- R089 — `cluster-proof/` remains the rebuilt route-free packaged proof and is freshly replayed in the final bundle.
- R090 — The scaffold, `tiny-cluster/`, and `cluster-proof/` remain locked to one equal clustered story because the delegated S05 parity bundle is nested inside the final S06 closeout rail.
- R091 — Runtime-owned `meshc cluster status|continuity|diagnostics` surfaces remain sufficient to inspect route-free work and failover truth.
- R092 — The public clustered story remains route-free and tooling-owned across README/docs/package runbooks.
- R093 — The retained S03/S04 proof bundles still revolve around the intentionally trivial visible `1 + 1` workload.

## Notes for Tester

If the assembled closeout rail goes red, debug in this order: `.tmp/m046-s06/verify/phase-report.txt` -> `.tmp/m046-s06/verify/full-contract.log` -> `.tmp/m046-s06/verify/retained-m046-s05-verify/` -> the targeted S03/S04 retained bundles named by `.tmp/m046-s06/verify/latest-proof-bundle.txt`. Do not add a second direct runtime harness, re-promote S05 in docs, or run the historical wrapper in parallel as a shortcut; those are precisely the regressions S06 is meant to catch.
