# S05: Docs-First Example & Proof Closeout — UAT

**Milestone:** M045
**Written:** 2026-03-31T02:49:20.778Z

# S05: Docs-First Example & Proof Closeout — UAT

**Milestone:** M045
**Written:** 2026-03-28

## UAT Type

- UAT mode: artifact-driven
- Why this mode is sufficient: S05 changes the public docs/proof contract and the assembled verifier stack, so the truthful acceptance surface is the rendered docs, the Rust docs-contract test, and the retained verifier artifacts rather than a new manual runtime flow.

## Preconditions

- Run from the repo root with the Rust toolchain and Node/npm available.
- `npm --prefix website install` or an equivalent dependency install has already been completed for the docs site.
- No other process is concurrently running the M045 clustered verifiers, especially `bash scripts/verify-m045-s02.sh`, `bash scripts/verify-m045-s04.sh`, or `bash scripts/verify-m045-s05.sh`.

## Smoke Test

Run:

```bash
cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture
```

**Expected:** Cargo reports `running 2 tests` and both S05 assertions pass, proving the repo now treats the clustered tutorial as the first stop and `bash scripts/verify-m045-s05.sh` as the current closeout rail.

## Test Cases

### 1. Getting Started clustered tutorial is the first public clustered path

1. Run:
   ```bash
   npm --prefix website run build
   ```
2. Run:
   ```bash
   rg -n '/docs/getting-started/clustered-example/|meshc init --clustered|meshc cluster status|meshc cluster continuity|meshc cluster diagnostics' \
     website/docs/.vitepress/config.mts \
     website/docs/docs/getting-started/index.md \
     website/docs/docs/getting-started/clustered-example/index.md
   ```
3. **Expected:** The docs build succeeds, the sidebar exposes `Clustered Example`, the Getting Started landing page links to `/docs/getting-started/clustered-example/`, and the tutorial contains the real scaffold command plus the runtime-owned `meshc cluster` inspection commands.

### 2. Public docs/readmes point at S05 as the current closeout rail

1. Run:
   ```bash
   cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture
   ```
2. Optionally inspect the source surfaces directly with:
   ```bash
   rg -n '/docs/getting-started/clustered-example/|verify-m045-s05.sh|verify-m045-s04.sh' \
     README.md \
     cluster-proof/README.md \
     website/docs/docs/tooling/index.md \
     website/docs/docs/distributed/index.md \
     website/docs/docs/distributed-proof/index.md
   ```
3. **Expected:** The S05 test target passes, clustered readers are routed to the scaffold-first tutorial first, `bash scripts/verify-m045-s05.sh` is described as the current closeout rail, and `bash scripts/verify-m045-s04.sh` is described only as the historical/replayable subrail.

### 3. Final wrapper rail replays S04 and retains the lower-level evidence

1. Run:
   ```bash
   bash scripts/verify-m045-s05.sh
   ```
2. Read the resulting verifier state:
   ```bash
   cat .tmp/m045-s05/verify/status.txt
   cat .tmp/m045-s05/verify/current-phase.txt
   cat .tmp/m045-s05/verify/phase-report.txt
   ```
3. Confirm the retained evidence exists:
   ```bash
   find .tmp/m045-s05/verify -maxdepth 2 -type f | sort | rg 'retained-m045-s04-verify|latest-failover-bundle.txt|status.txt|current-phase.txt|phase-report.txt|full-contract.log'
   ```
4. **Expected:** The wrapper exits 0, `status.txt` contains `ok`, `current-phase.txt` contains `complete`, `phase-report.txt` shows the S04 replay/copy/pointer/S05-contract/docs-build phases passing in order, `.tmp/m045-s05/verify/retained-m045-s04-verify/` exists, and `.tmp/m045-s05/verify/latest-failover-bundle.txt` points at the retained S03 failover bundle root.

## Edge Cases

### Retained failover evidence is still reachable from the S05 wrapper

1. After a green wrapper run, read:
   ```bash
   cat .tmp/m045-s05/verify/latest-failover-bundle.txt
   ```
2. Inspect that pointed directory for the expected retained failover bundle contents.
3. **Expected:** The pointer resolves to the retained S03 artifact root and the failover scenario bundle remains available for drill-down; S05 does not invent a new standalone failover artifact directory.

### Historical S04 replay state is preserved verbatim

1. After a green wrapper run, inspect:
   ```bash
   find .tmp/m045-s05/verify/retained-m045-s04-verify -maxdepth 1 -type f | sort
   ```
2. **Expected:** The copied S04 verifier directory contains its own `status.txt`, `current-phase.txt`, `phase-report.txt`, and replay logs so a future agent can debug the lower-level failure without rerunning S04 immediately.

## Failure Signals

- `cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture` reports `running 0 tests`, missing marker assertions, or any failing S05 assertion.
- `npm --prefix website run build` fails or the route-marker sweep cannot find `/docs/getting-started/clustered-example/` and the runtime `meshc cluster` commands.
- `bash scripts/verify-m045-s05.sh` exits non-zero, `status.txt` is not `ok`, `current-phase.txt` stops before `complete`, or the retained S04/S03 evidence files are missing.

## Requirements Proved By This UAT

- R080 — proves that `meshc init --clustered` is now the primary docs-grade clustered example surface.
- R081 — proves that public docs teach the scaffold-first clustered example first and keep deeper proof rails secondary.

## Not Proven By This UAT

- The lower-level runtime ownership work from S02/S03/S04 is not reproven independently here; S05 proves that the final closeout rail reuses those earlier product rails truthfully.
- Live hosted/Fly behavior beyond the existing read-only proof surfaces is not part of this slice UAT.

## Notes for Tester

If the final wrapper rail is red, debug in this order: `.tmp/m045-s05/verify/phase-report.txt` -> the copied `.tmp/m045-s05/verify/retained-m045-s04-verify/` logs -> the path stored in `.tmp/m045-s05/verify/latest-failover-bundle.txt`. S05 is a composition rail, so the root cause will usually live in the retained lower-level evidence rather than in a new S05-specific runtime harness.
