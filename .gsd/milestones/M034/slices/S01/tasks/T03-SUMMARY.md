---
id: T03
parent: S01
milestone: M034
provides: []
requires: []
affects: []
key_files: ["registry/src/routes/publish.rs", "registry/src/routes/download.rs", "registry/src/db/packages.rs", "compiler/meshpkg/src/install.rs", "website/docs/docs/tooling/index.md", "scripts/verify-m034-s01.sh", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Upload or confirm the package blob before inserting the version row so registry metadata never becomes public truth ahead of storage truth.", "Treat `meshpkg install <name>` as a fetch-plus-lock operation and surface that contract explicitly through docs, terminal output, and JSON (`lockfile`, `manifest_changed: false`)."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed `cargo test -p meshpkg named_install -- --nocapture`, `cargo test --manifest-path registry/Cargo.toml missing_blob_error -- --nocapture`, `bash -n scripts/verify-m034-s01.sh`, and the two slice-plan `rg` contract checks. The live verifier `set -a && source .env && set +a && bash scripts/verify-m034-s01.sh` now proves the contract/doc greps, real publish, metadata fetches, real download, manifest-driven install, named-install manifest stability, and consumer build/run on the live registry path before failing at the duplicate publish helper’s TLS transport."
completed_at: 2026-03-26T21:27:51.500Z
blocker_discovered: false
---

# T03: Hardened registry truth ordering and the named-install/docs contract, but the live verifier still stops at duplicate-publish because the verifier’s Python urllib POST helper cannot validate the registry certificate on this host.

> Hardened registry truth ordering and the named-install/docs contract, but the live verifier still stops at duplicate-publish because the verifier’s Python urllib POST helper cannot validate the registry certificate on this host.

## What Happened
---
id: T03
parent: S01
milestone: M034
key_files:
  - registry/src/routes/publish.rs
  - registry/src/routes/download.rs
  - registry/src/db/packages.rs
  - compiler/meshpkg/src/install.rs
  - website/docs/docs/tooling/index.md
  - scripts/verify-m034-s01.sh
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Upload or confirm the package blob before inserting the version row so registry metadata never becomes public truth ahead of storage truth.
  - Treat `meshpkg install <name>` as a fetch-plus-lock operation and surface that contract explicitly through docs, terminal output, and JSON (`lockfile`, `manifest_changed: false`).
duration: ""
verification_result: mixed
completed_at: 2026-03-26T21:27:51.502Z
blocker_discovered: false
---

# T03: Hardened registry truth ordering and the named-install/docs contract, but the live verifier still stops at duplicate-publish because the verifier’s Python urllib POST helper cannot validate the registry certificate on this host.

**Hardened registry truth ordering and the named-install/docs contract, but the live verifier still stops at duplicate-publish because the verifier’s Python urllib POST helper cannot validate the registry certificate on this host.**

## What Happened

Reordered registry publish so blob truth is established in R2 before the version row is inserted, reworked download so the blob fetch happens before counter mutation, and fixed the underlying download-counter helper to use a real transaction. On the package-manager side, named install now explicitly behaves as fetch-plus-lock only: `meshpkg install <name>` reports `lockfile` and `manifest_changed: false`, prints honest follow-up guidance with a quoted scoped dependency snippet, and no longer implies a hidden `mesh.toml` mutation. I updated the tooling docs to use quoted scoped dependency keys and to say plainly that named install updates `mesh.lock` but does not edit `mesh.toml`. I also extended `scripts/verify-m034-s01.sh` with docs/CLI contract greps, named-install manifest-stability checks, and stdlib `tomllib` parsing after the first live run exposed an unavailable third-party `toml` module. The remaining failure is verifier-local: the live proof now reaches the duplicate-publish phase, but that phase still uses Python `urllib.request`, which hit `CERTIFICATE_VERIFY_FAILED` against `https://api.packages.meshlang.dev` on this host even though the script’s curl-based registry GET phases succeeded.

## Verification

Passed `cargo test -p meshpkg named_install -- --nocapture`, `cargo test --manifest-path registry/Cargo.toml missing_blob_error -- --nocapture`, `bash -n scripts/verify-m034-s01.sh`, and the two slice-plan `rg` contract checks. The live verifier `set -a && source .env && set +a && bash scripts/verify-m034-s01.sh` now proves the contract/doc greps, real publish, metadata fetches, real download, manifest-driven install, named-install manifest stability, and consumer build/run on the live registry path before failing at the duplicate publish helper’s TLS transport.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshpkg named_install -- --nocapture` | 0 | ✅ pass | 8570ms |
| 2 | `cargo test --manifest-path registry/Cargo.toml missing_blob_error -- --nocapture` | 0 | ✅ pass | 96000ms |
| 3 | `bash -n scripts/verify-m034-s01.sh` | 0 | ✅ pass | 20ms |
| 4 | `rg -n '"your-login/your-package" = "1.0.0"' website/docs/docs/tooling/index.md` | 0 | ✅ pass | 20ms |
| 5 | `rg -n 'does not edit mesh.toml|updates mesh.lock' website/docs/docs/tooling/index.md compiler/meshpkg/src/install.rs` | 0 | ✅ pass | 20ms |
| 6 | `set -a && source .env && set +a && bash scripts/verify-m034-s01.sh` | 1 | ❌ fail | 7000ms |


## Deviations

Added a local support fix in `registry/src/db/packages.rs` because the existing `increment_download` helper claimed atomicity but updated the version and package counters outside a transaction. Also extended named-install JSON output with `lockfile` and `manifest_changed` so the verifier can prove the contract mechanically instead of inferring it from prose.

## Known Issues

`post_duplicate_publish()` in `scripts/verify-m034-s01.sh` still uses Python `urllib.request`; on this host that helper fails TLS verification against `https://api.packages.meshlang.dev` during the duplicate-publish phase even though the rest of the verifier’s curl-based live registry checks succeed. The next recovery step is to switch that duplicate POST helper to `curl` (or another transport that uses the same trusted cert path as the rest of the script) and rerun the live verifier.

## Files Created/Modified

- `registry/src/routes/publish.rs`
- `registry/src/routes/download.rs`
- `registry/src/db/packages.rs`
- `compiler/meshpkg/src/install.rs`
- `website/docs/docs/tooling/index.md`
- `scripts/verify-m034-s01.sh`
- `.gsd/KNOWLEDGE.md`


## Deviations
Added a local support fix in `registry/src/db/packages.rs` because the existing `increment_download` helper claimed atomicity but updated the version and package counters outside a transaction. Also extended named-install JSON output with `lockfile` and `manifest_changed` so the verifier can prove the contract mechanically instead of inferring it from prose.

## Known Issues
`post_duplicate_publish()` in `scripts/verify-m034-s01.sh` still uses Python `urllib.request`; on this host that helper fails TLS verification against `https://api.packages.meshlang.dev` during the duplicate-publish phase even though the rest of the verifier’s curl-based live registry checks succeed. The next recovery step is to switch that duplicate POST helper to `curl` (or another transport that uses the same trusted cert path as the rest of the script) and rerun the live verifier.
