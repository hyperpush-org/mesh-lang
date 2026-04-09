---
id: T01
parent: S01
milestone: M039
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-rt/src/dist/discovery.rs", "compiler/mesh-rt/src/dist/node.rs", "compiler/mesh-rt/src/dist/mod.rs", "compiler/meshc/tests/e2e_m039_s01.rs", ".gsd/KNOWLEDGE.md"]
key_decisions: ["D129: runtime discovery dials temporary `discovery@host:port` targets, but canonical membership only comes from a validated handshake-advertised node name.", "Use tuple socket APIs for bind/connect so IPv6-safe node-name parsing remains valid at the TCP layer."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Task gate passed with `cargo test -p mesh-rt discovery_ -- --nocapture`. Additional runtime regressions passed with `cargo test -p mesh-rt test_parse_node_name_edge_cases -- --nocapture` and `cargo test -p mesh-rt test_handshake_rejects_invalid_remote_name -- --nocapture`. Slice-level verification was run as well: `cargo test -p meshc --test e2e_m039_s01 -- --nocapture` failed as expected because T02/T03 placeholders are still red, and `bash scripts/verify-m039-s01.sh` failed because the verifier script does not exist yet."
completed_at: 2026-03-28T09:22:13.894Z
blocker_discovered: false
---

# T01: Added a runtime-owned DNS discovery loop in mesh-rt with candidate filtering, IPv6-safe node parsing, and validated handshake identities.

> Added a runtime-owned DNS discovery loop in mesh-rt with candidate filtering, IPv6-safe node parsing, and validated handshake identities.

## What Happened
---
id: T01
parent: S01
milestone: M039
key_files:
  - compiler/mesh-rt/src/dist/discovery.rs
  - compiler/mesh-rt/src/dist/node.rs
  - compiler/mesh-rt/src/dist/mod.rs
  - compiler/meshc/tests/e2e_m039_s01.rs
  - .gsd/KNOWLEDGE.md
key_decisions:
  - D129: runtime discovery dials temporary `discovery@host:port` targets, but canonical membership only comes from a validated handshake-advertised node name.
  - Use tuple socket APIs for bind/connect so IPv6-safe node-name parsing remains valid at the TCP layer.
duration: ""
verification_result: mixed
completed_at: 2026-03-28T09:22:13.896Z
blocker_discovered: false
---

# T01: Added a runtime-owned DNS discovery loop in mesh-rt with candidate filtering, IPv6-safe node parsing, and validated handshake identities.

**Added a runtime-owned DNS discovery loop in mesh-rt with candidate filtering, IPv6-safe node parsing, and validated handshake identities.**

## What Happened

Added `compiler/mesh-rt/src/dist/discovery.rs` as the runtime-owned DNS discovery seam. It parses discovery config from environment, resolves one DNS seed with a fixed cluster port, deduplicates answers, filters self and already-connected peers before any dial, and runs a periodic reconcile loop that reuses `mesh_node_connect` through synthesized `discovery@host:port` targets. In `compiler/mesh-rt/src/dist/node.rs`, `parse_node_name(...)` now understands bracketed IPv6 hosts, bind/connect use tuple socket APIs so parsed IPv6 names still work at the TCP layer, and the handshake validates the remote node name before session registration so malformed advertised identities cannot pollute membership truth. I also exported the module through `compiler/mesh-rt/src/dist/mod.rs`, added targeted runtime unit coverage, recorded the IPv6 socket-formatting gotcha in `.gsd/KNOWLEDGE.md`, and created `compiler/meshc/tests/e2e_m039_s01.rs` as an explicit failing slice-level contract for T02/T03.

## Verification

Task gate passed with `cargo test -p mesh-rt discovery_ -- --nocapture`. Additional runtime regressions passed with `cargo test -p mesh-rt test_parse_node_name_edge_cases -- --nocapture` and `cargo test -p mesh-rt test_handshake_rejects_invalid_remote_name -- --nocapture`. Slice-level verification was run as well: `cargo test -p meshc --test e2e_m039_s01 -- --nocapture` failed as expected because T02/T03 placeholders are still red, and `bash scripts/verify-m039-s01.sh` failed because the verifier script does not exist yet.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-rt discovery_ -- --nocapture` | 0 | ✅ pass | 249ms |
| 2 | `cargo test -p mesh-rt test_parse_node_name_edge_cases -- --nocapture` | 0 | ✅ pass | 206ms |
| 3 | `cargo test -p mesh-rt test_handshake_rejects_invalid_remote_name -- --nocapture` | 0 | ✅ pass | 192ms |
| 4 | `cargo test -p meshc --test e2e_m039_s01 -- --nocapture` | 101 | ❌ fail | 6369ms |
| 5 | `bash scripts/verify-m039-s01.sh` | 127 | ❌ fail | 19ms |


## Deviations

Created `compiler/meshc/tests/e2e_m039_s01.rs` during T01 so the slice-level verification target exists and fails closed with named pending proofs instead of a missing-target error.

## Known Issues

`compiler/meshc/tests/e2e_m039_s01.rs` is still a deliberate red contract for T02/T03, `scripts/verify-m039-s01.sh` does not exist yet, and `cargo test -p mesh-rt ...` still emits pre-existing warnings in unrelated runtime modules.

## Files Created/Modified

- `compiler/mesh-rt/src/dist/discovery.rs`
- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/mesh-rt/src/dist/mod.rs`
- `compiler/meshc/tests/e2e_m039_s01.rs`
- `.gsd/KNOWLEDGE.md`


## Deviations
Created `compiler/meshc/tests/e2e_m039_s01.rs` during T01 so the slice-level verification target exists and fails closed with named pending proofs instead of a missing-target error.

## Known Issues
`compiler/meshc/tests/e2e_m039_s01.rs` is still a deliberate red contract for T02/T03, `scripts/verify-m039-s01.sh` does not exist yet, and `cargo test -p mesh-rt ...` still emits pre-existing warnings in unrelated runtime modules.
