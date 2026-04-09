# S04: Rebuild `cluster-proof/` as tiny packaged proof — UAT

**Milestone:** M046
**Written:** 2026-04-01T00:10:49.546Z

# S04: Rebuild `cluster-proof/` as tiny packaged proof — UAT

**Milestone:** M046  
**Written:** 2026-03-31

## UAT Type

- UAT mode: mixed package-contract + runtime/CLI proof
- Why this mode is sufficient: S04 changes both the packaged source/deployment contract (`cluster-proof/`, Docker, Fly, README, smoke tests) and the live runtime proof surface (`meshc cluster status|continuity|diagnostics`, retained `.tmp/m046-s04/...` bundles, and historical wrapper delegation). The truthful acceptance surface is the exact build/test/e2e/verifier commands that exercise those boundaries end to end.

## Preconditions

- Run from the repository root with Cargo and Docker available.
- `target/`, `.tmp/m046-s04/`, and Docker build cache paths must be writable.
- Loopback ports on `127.0.0.1` and `::1` must be available for the temporary two-node packaged proof.
- No stale `cluster-proof` node processes should be holding prior test ports.

## Smoke Test

Run:

```bash
cargo run -q -p meshc -- build cluster-proof && test ! -e cluster-proof/cluster.mpl && test ! -e cluster-proof/config.mpl && test ! -e cluster-proof/work_continuity.mpl
```

**Expected:** `cluster-proof` builds successfully and the deleted legacy modules remain absent after the build.

## Test Cases

### 1. Package source contract stays route-free and source-owned

1. Run:
   ```bash
   cargo run -q -p meshc -- build cluster-proof && test ! -e cluster-proof/cluster.mpl && test ! -e cluster-proof/config.mpl && test ! -e cluster-proof/work_continuity.mpl
   ```
2. **Expected:**
   - `cluster-proof/mesh.toml` contains `[package]` only and omits `[cluster]` / `declarations`.
   - `cluster-proof/main.mpl` contains exactly one `Node.start_from_env()` path and no `HTTP.serve(...)`, `/work`, `/membership`, `Continuity.*`, or imports from the deleted helper modules.
   - `cluster-proof/work.mpl` contains exactly one `clustered(work)` declaration, keeps runtime name `Work.execute_declared_work`, and returns `1 + 1` with no delay/timing helpers.
   - `cluster-proof/cluster.mpl`, `cluster-proof/config.mpl`, and `cluster-proof/work_continuity.mpl` do not exist.

### 2. README, Dockerfile, Fly config, and package smoke rails stay honest about the packaged route-free binary

1. Run:
   ```bash
   cargo run -q -p meshc -- test cluster-proof/tests && docker build -f cluster-proof/Dockerfile -t mesh-cluster-proof:m046-s04-local .
   ```
2. **Expected:**
   - `meshc test cluster-proof/tests` reports the route-free smoke suite passing.
   - `cluster-proof/README.md` points operators at `meshc cluster status`, `meshc cluster continuity`, and `meshc cluster diagnostics` and does not mention `/work`, `/membership`, `CLUSTER_PROOF_WORK_DELAY_MS`, `docker-entrypoint.sh`, `PORT`, `http_service`, or Fly HTTP hostnames as the current packaged story.
   - `cluster-proof/Dockerfile` copies only `/tmp/cluster-proof` into the runtime image and uses it as the entrypoint directly.
   - `cluster-proof/fly.toml` keeps only the direct-binary build/env contract and omits `http_service` / `PORT`.
   - The Docker image builds successfully from the repo root.

### 3. The packaged proof builds to a temp output path and exposes startup truth only through Mesh-owned CLI/runtime surfaces

1. Run:
   ```bash
   cargo test -p meshc --test e2e_m046_s04 m046_s04_ -- --nocapture
   ```
2. **Expected:**
   - The target reports `running 6 tests` and all six pass.
   - The helper negatives fail closed on a missing temp-output parent directory and malformed `build-meta.json`.
   - The package contract and package-build smoke rails pass.
   - The startup rail builds `cluster-proof` to a temp output path outside `cluster-proof/`, leaves tracked binary snapshots unchanged, boots two nodes, discovers exactly one startup record where `declared_handler_runtime_name == "Work.execute_declared_work"`, and proves completion through `meshc cluster status|continuity|diagnostics` only.
   - The rail retains `.tmp/m046-s04/cluster-proof-startup-two-node-*` with `scenario-meta.json`, `build.log`, `build-meta.json`, `tracked-binary-snapshots.json`, status/continuity/diagnostics JSON, human CLI output, and node stdout/stderr logs.

### 4. The direct packaged verifier replays the whole packaged proof and retains a stable bundle shape

1. Run:
   ```bash
   bash scripts/verify-m046-s04.sh
   ```
2. **Expected:**
   - The script prints `verify-m046-s04: ok`.
   - `.tmp/m046-s04/verify/status.txt` contains `ok` and `.tmp/m046-s04/verify/current-phase.txt` contains `complete`.
   - `.tmp/m046-s04/verify/phase-report.txt` shows `contract-guards`, `mesh-rt-build`, `m046-s03-regression`, `cluster-proof-build`, `cluster-proof-tests`, `m046-s04-e2e`, `m046-s04-artifacts`, and `m046-s04-bundle-shape` all marked `passed`.
   - `.tmp/m046-s04/verify/latest-proof-bundle.txt` points at `.tmp/m046-s04/verify/retained-m046-s04-artifacts`.
   - That retained bundle contains exactly the expected copied artifact directories for helper preflight, helper build-meta, package contract, package build-and-test, and startup-two-node evidence.

### 5. Historical M044/M045 wrapper rails now prove delegation to the packaged verifier instead of the deleted routeful contract

1. Run:
   ```bash
   cargo test -p meshc --test e2e_m044_s05 m044_s05_ -- --nocapture && cargo test -p meshc --test e2e_m045_s04 m045_s04_ -- --nocapture && cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture
   ```
2. **Expected:**
   - All three focused Rust targets pass.
   - The wrapper contract tests assert delegation to `scripts/verify-m046-s04.sh`, retained bundle/status artifacts, and absence of deleted `/work`, `/membership`, delay-hook, and Fly HTTP package claims as current truth.
   - None of these historical alias rails reintroduce scaffold/docs parity assertions that now belong to S05.

## Edge Cases

### Missing temp build parents fail before the packaged proof rail can churn tracked binaries

1. Use the helper negative already covered by `m046_s04_cluster_proof_helpers_require_precreated_temp_output_parent`.
2. **Expected:** the harness writes `build-preflight-error.txt` and fails before attempting to build into an in-package path.

### The README can mention the idea of submit/status endpoints being absent without reviving actual route strings as current truth

1. Inspect `cluster-proof/README.md` after the smoke rail passes.
2. **Expected:** the README may describe that package-owned submit/status endpoints are intentionally absent, but it must not revive real `/work`, `/membership`, Fly HTTP proxy, or delay-hook guidance.

### Temp-path packaged builds must not mutate tracked `cluster-proof` outputs

1. Inspect the retained `tracked-binary-snapshots.json` in the latest `.tmp/m046-s04/verify/retained-m046-s04-artifacts/cluster-proof-startup-two-node-*` directory.
2. **Expected:** `tracked_binary_before == tracked_binary_after` and `tracked_llvm_before == tracked_llvm_after`.

## Failure Signals

- Any `[cluster]`, `declarations`, route strings, continuity helpers, delay knobs, `docker-entrypoint.sh`, `PORT`, or `http_service` drift back into the packaged contract.
- `cluster-proof` builds only in-place or mutates tracked binaries instead of using a temp `--output` path.
- `meshc cluster continuity --json` stops surfacing exactly one startup record with runtime name `Work.execute_declared_work`.
- The direct verifier stops leaving `.tmp/m046-s04/verify/status.txt=ok`, a fully passed `phase-report.txt`, and a retained proof-bundle pointer.
- Historical wrapper tests start asserting deleted routeful/Fly/package stories as current truth again.

## Requirements Proved By This UAT

- R089 — `cluster-proof/` is rebuilt as a tiny packaged route-free proof app with no app-owned clustering, failover, routing, or status logic.
- R086, R087, R091, and R093 are advanced because the packaged proof now stays trivial, route-free, runtime-triggered, and inspectable through Mesh-owned CLI/runtime surfaces.

## Notes for Tester

If the packaged verifier goes red, inspect `.tmp/m046-s04/verify/phase-report.txt`, `.tmp/m046-s04/verify/latest-proof-bundle.txt`, and the referenced `cluster-proof-startup-two-node-*` bundle before editing package source. The intended debugging order is: contract guard drift, temp-build metadata/snapshot drift, CLI JSON truth, then node logs — not reintroducing package-owned routes or proxy health surfaces.
