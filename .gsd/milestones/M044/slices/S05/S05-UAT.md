# S05: Cluster-Proof Rewrite, Docs, and Final Closeout — UAT

**Milestone:** M044
**Written:** 2026-03-30T08:19:36.088Z

# S05: Cluster-Proof Rewrite, Docs, and Final Closeout — UAT

**Milestone:** M044
**Written:** 2026-03-28

## UAT Type

- UAT mode: mixed
- Why this mode is sufficient: S05 changed both live runtime behavior (`cluster-proof` bootstrap/work surfaces) and public proof/docs surfaces. The honest acceptance path therefore needs live e2e/package rails, the assembled wrapper verifier, and the website build.

## Preconditions

- Run from the repo root.
- Rust/Cargo and the website toolchain are installed.
- No stale `cluster-proof` processes are bound to the test ports the e2e rails allocate.
- If a repo `.env` exists, load it before running the assembled verifiers so the environment matches the local proof contract.

## Smoke Test

Run the terminal closeout command:

```bash
bash scripts/verify-m044-s05.sh
```

**Expected:** The command ends with `verify-m044-s05: ok`, `.tmp/m044-s05/verify/status.txt` contains `ok`, `.tmp/m044-s05/verify/current-phase.txt` contains `complete`, and the wrapper retains copied S03/S04 bundles under `.tmp/m044-s05/verify/`.

## Test Cases

### 1. Public clustered-app bootstrap contract replaces the proof-app env dialect

1. Run:
   ```bash
   cargo run -q -p meshc -- test cluster-proof/tests/config.test.mpl
   ```
2. Run:
   ```bash
   cargo test -p meshc --test e2e_m044_s05 m044_s05_public_contract_ -- --nocapture
   ```
3. **Expected:** The config tests pass; the e2e rail proves explicit `MESH_NODE_NAME` startup, Fly-derived identity fallback, malformed `MESH_*` inputs failing closed, and rejection of the old `CLUSTER_PROOF_COOKIE`, `CLUSTER_PROOF_NODE_BASENAME`, and `CLUSTER_PROOF_ADVERTISE_HOST` names.

### 2. Legacy `/work` probe path is gone and keyed runtime-owned work is the only remaining app work surface

1. Confirm the deleted file is absent:
   ```bash
   test ! -e cluster-proof/work_legacy.mpl
   ```
2. Run:
   ```bash
   cargo run -q -p meshc -- build cluster-proof
   cargo run -q -p meshc -- test cluster-proof/tests
   cargo test -p meshc --test e2e_m044_s05 m044_s05_legacy_cleanup_ -- --nocapture
   ```
3. **Expected:** The package builds/tests pass, the S05 legacy-cleanup rail proves keyed submit/status still works, and the old `GET /work` probe surface is absent rather than silently preserved as dead code.

### 3. Final closeout rail replays the real clustered-app and failover product story

1. Run:
   ```bash
   bash scripts/verify-m044-s05.sh
   ```
2. Inspect:
   ```bash
   cat .tmp/m044-s05/verify/status.txt
   cat .tmp/m044-s05/verify/current-phase.txt
   ```
3. **Expected:** The wrapper replays S03 and S04 before the S05-specific rails, ends with `ok` / `complete`, and retains the copied S03/S04 verifier trees plus a valid failover bundle pointer under `.tmp/m044-s05/verify/`.

### 4. Public docs teach scaffold-first clustered apps instead of proof-app folklore

1. Run:
   ```bash
   cargo test -p meshc --test e2e_m044_s05 -- --nocapture
   npm --prefix website run build
   ```
2. **Expected:** The closeout docs/source truth tests pass, the website build succeeds, the docs require `meshc init --clustered` + `meshc cluster` as the primary story, and they reject the deleted `/work` probe and old `CLUSTER_PROOF_*` bootstrap/env wording.

## Edge Cases

### Old bootstrap env names are rejected

1. Run:
   ```bash
   cargo test -p meshc --test e2e_m044_s05 m044_s05_public_contract_old_bootstrap_env_names_fail_closed -- --nocapture
   ```
2. **Expected:** The proof app fails closed instead of accepting `CLUSTER_PROOF_*` aliases for the real clustered contract.

### Malformed clustered bootstrap stays fail-closed

1. Run:
   ```bash
   cargo test -p meshc --test e2e_m044_s05 m044_s05_public_contract_malformed_inputs_fail_closed -- --nocapture
   ```
2. **Expected:** Blank discovery seeds, malformed `MESH_NODE_NAME`, and contradictory topology env do not start a half-valid cluster.

### S04 failover bundle pointer is usable by the S05 wrapper

1. Run:
   ```bash
   bash scripts/verify-m044-s04.sh
   python3 - <<'PY'
from pathlib import Path
p = Path('.tmp/m044-s04/verify/latest-proof-bundle.txt')
text = p.read_text().strip()
print(text)
assert text.startswith('.tmp/m044-s04/continuity-api-failover-promotion-rejoin-')
assert '\n' not in text
PY
   ```
2. **Expected:** `latest-proof-bundle.txt` contains only the retained bundle directory path, so `bash scripts/verify-m044-s05.sh` can copy and validate the failover artifacts without a malformed-pointer failure.

## Failure Signals

- `bash scripts/verify-m044-s05.sh` stops before `verify-m044-s05: ok`.
- `.tmp/m044-s05/verify/status.txt` is not `ok`.
- `cargo test -p meshc --test e2e_m044_s05 -- --nocapture` reports a docs/source truth failure.
- `cluster-proof/work_legacy.mpl` exists again or the legacy-cleanup source-absence rail goes red.
- The website build fails or the docs still mention deleted `CLUSTER_PROOF_*` bootstrap envs or the old `GET /work` probe path.

## Requirements Proved By This UAT

- R069 — `cluster-proof` is fully rewritten onto the new clustered-app standard and the old explicit clustering path is gone.
- R070 — The public docs and proof surfaces teach scaffold-first clustered Mesh apps as the primary story.

## Not Proven By This UAT

- Live destructive failover on Fly; Fly remains a read-only inspection rail.
- Any topology broader than one primary plus one standby.
- Exactly-once execution semantics or a manual promotion/operator override path.

## Notes for Tester

If the assembled wrapper goes red after the runtime/product rails already look healthy, inspect `.tmp/m044-s05/verify/current-phase.txt` first. In this slice the only false-red seams came from stale proof expectations (`e2e_m044_s04.rs`), login-shell verifier snippets, and malformed bundle-pointer output — not from runtime continuity logic regressing.
