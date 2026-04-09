---
id: S01
parent: M039
milestone: M039
provides:
  - A runtime-owned DNS discovery seam in `mesh-rt` with dedupe, self/connected filtering, IPv6-safe dialing, and handshake-validated membership identity.
  - A new narrow `cluster-proof/` app that reports truthful `/membership` state from live runtime sessions with a small env-driven config contract.
  - A canonical local replay surface (`scripts/verify-m039-s01.sh`) plus per-node logs and phase reports for discovery convergence and membership shrinkage debugging.
requires:
  []
affects:
  - S02
  - S03
  - S04
key_files:
  - compiler/mesh-rt/src/dist/discovery.rs
  - compiler/mesh-rt/src/dist/node.rs
  - cluster-proof/config.mpl
  - cluster-proof/cluster.mpl
  - cluster-proof/main.mpl
  - cluster-proof/tests/config.test.mpl
  - compiler/meshc/tests/e2e_m039_s01.rs
  - scripts/verify-m039-s01.sh
  - .gsd/PROJECT.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - D129: DNS discovery in `mesh-rt` dials temporary `discovery@host:port` targets, but canonical membership only comes from validated handshake-advertised node names.
  - D131: The `cluster-proof` app derives membership from `Node.self()` plus `Node.list()`, explicitly including self, instead of discovery candidates or registry guesses.
  - D130: `scripts/verify-m039-s01.sh` is the authoritative local acceptance surface and must fail closed when a named Cargo filter does not execute a real test count.
  - The `/membership` handler uses a typed `MembershipPayload` JSON path plus env-string-backed port fields because more direct request-path helper lowering was still unstable on this Mesh shape.
patterns_established:
  - Treat discovery answers as bootstrap candidates only; session truth must come from validated handshake-advertised node identities.
  - Derive membership endpoints from live runtime session surfaces (`Node.self()` plus `Node.list()`), and always include self so a zero-peer node still reports truthful cluster state.
  - For distributed proof slices, keep one canonical replay wrapper that preserves phase logs, per-node stdout/stderr, and fail-closed non-zero test-count checks.
observability_surfaces:
  - `mesh discovery:` stderr logs from `mesh-rt` that include provider, seed, resolved/accepted counts, accepted targets, rejected targets, and last error.
  - `[cluster-proof]` startup logs that name mode, advertised node identity, discovery provider/seed, and HTTP/cluster ports without echoing the cookie.
  - `.tmp/m039-s01/verify/phase-report.txt` as the authoritative verifier phase ledger.
  - `.tmp/m039-s01/e2e-m039-s01-*/node-*.stdout.log` and `.stderr.log` as durable per-node evidence for convergence and node-loss runs.
drill_down_paths:
  - .gsd/milestones/M039/slices/S01/tasks/T01-SUMMARY.md
  - .gsd/milestones/M039/slices/S01/tasks/T02-SUMMARY.md
  - .gsd/milestones/M039/slices/S01/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-28T10:09:03.342Z
blocker_discovered: false
---

# S01: General DNS Discovery & Membership Truth

**Runtime-owned DNS discovery plus a narrow `cluster-proof/` app now prove local automatic cluster formation and truthful membership shrinkage after node loss without manual peer lists.**

## What Happened

S01 moved cluster formation out of Mesh application code and into `mesh-rt`, then wrapped that runtime seam in a narrow proof surface that reports actual live membership instead of bootstrap guesses. `compiler/mesh-rt/src/dist/discovery.rs` now owns DNS-first discovery config, candidate normalization, dedupe, self/already-connected filtering, periodic reconcile, and provider/seed/reject-reason logging. `compiler/mesh-rt/src/dist/node.rs` was tightened so bracketed IPv6 node names parse cleanly, listener/connect paths use tuple socket APIs instead of brittle string rebuilding, and handshake-advertised node names are validated before session registration so discovery bootstrap addresses cannot become fake membership truth.

On top of that runtime seam, S01 introduced `cluster-proof/` as the new proof app instead of extending Mesher again. `cluster-proof/config.mpl` owns the small env contract and explicit/Fly-friendly advertised-identity construction. `cluster-proof/cluster.mpl` derives `/membership` from `Node.self()` plus `Node.list()`, explicitly includes self in the reported membership, and uses a typed `MembershipPayload` JSON path plus env-string-backed port fields to avoid the compiled-Mesh request-path lowering crashes that initially blocked the slice. `cluster-proof/main.mpl` starts the runtime once, logs the advertised identity plus discovery context, and serves one read-only endpoint.

The slice closes with evidence, not just code. `compiler/meshc/tests/e2e_m039_s01.rs` now contains a two-node convergence proof and a node-loss shrinkage proof, both of which leave per-node stdout/stderr logs under `.tmp/m039-s01/`. `scripts/verify-m039-s01.sh` is the canonical local replay wrapper: it builds `cluster-proof`, runs the named runtime and e2e gates in order, records phase state under `.tmp/m039-s01/verify/phase-report.txt`, and fails closed if a named Cargo filter runs zero tests or omits the `running N test` line. The assembled result is a truthful local discovery-and-membership contract that downstream slices can now build on for work routing, rejoin, and Fly/operator proof rather than re-solving bootstrap and observability basics.

## Verification

Re-ran the slice acceptance surface and all planned gates passed:

- `cargo test -p mesh-rt discovery_ -- --nocapture`
- `cargo run -q -p meshc -- build cluster-proof`
- `cargo test -p meshc --test e2e_m039_s01 e2e_m039_s01_converges_without_manual_peers -- --nocapture`
- `cargo test -p meshc --test e2e_m039_s01 e2e_m039_s01_membership_updates_after_node_loss -- --nocapture`
- `bash scripts/verify-m039-s01.sh`
- `cargo test -p meshc --test e2e_m039_s01 -- --nocapture`

Observability and diagnostic surfaces were also checked:
- `.tmp/m039-s01/verify/phase-report.txt` recorded all four verifier phases as passed.
- Per-node stdout/stderr artifacts existed under `.tmp/m039-s01/e2e-m039-s01-converges-*` and `.tmp/m039-s01/e2e-m039-s01-node-loss-*`.
- Recent node logs showed the expected startup lines (`Config loaded`, `Node started`, `HTTP server starting`) and runtime discovery lines (`mesh discovery: provider=dns seed=localhost ... accepted_targets=... rejected_targets=...:self last_error=...`).
- The node-loss proof left the surviving node with truthful self-only membership and zero peers, not stale discovery candidate state.

## Requirements Advanced

- R045 — S01 implemented the general runtime discovery seam in `mesh-rt`, proved local automatic convergence from a shared DNS seed without manual peer lists, and established the canonical local verifier/artifact path for that capability.
- R046 — S01 moved membership truth onto live runtime sessions, proved local join and node-loss shrinkage through `cluster-proof` plus named e2e tests, and exposed discovery/member diagnostics that future rejoin and Fly work can reuse.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

Added `cluster-proof/tests/config.test.mpl` for malformed-config coverage even though the task’s expected-output list emphasized the app files and Rust harness, because the slice needed a small honest proof that invalid identity/cookie cases fail before live runtime.

The `/membership` endpoint still returns the required `self` key, but it now reaches that contract through a typed payload plus a local JSON-key rename workaround rather than a more direct request-path helper shape. That is a local implementation workaround for current Mesh lowering brittleness, not a change to the proof contract.

## Known Limitations

This slice is still a local proof surface. It does not yet prove Fly-backed discovery, one-image operator flow, runtime-native work routing, or clean rejoin after a lost node returns.

`cluster-proof/cluster.mpl` still depends on the typed-payload/JSON-key workaround for the final `self` field. If request-path lowering changes, that path is a likely regression seam.

The local proof intentionally depends on dual-stack `localhost` behavior so the two nodes can advertise distinct IPv4/IPv6 identities from the same discovery seed. Hosts without both families will fail clearly, but they are not a substitute for the later Fly proof path.

## Follow-ups

S02 should preserve the current membership contract and per-node log pattern while extending the proof app to distinguish ingress node from execution node for runtime-native work routing.

S03 should reuse the current node-loss harness and per-node artifact layout when it adds clean rejoin proof, rather than inventing a second failure harness.

S04 should wrap the same `cluster-proof` image and verifier style into the one-image/Fly operator path and reconcile `website/docs/docs/distributed/index.md` to the now-real S01 proof surfaces.

## Files Created/Modified

- `compiler/mesh-rt/src/dist/discovery.rs` — Added the runtime-owned DNS discovery config, candidate filtering, dedupe, reconcile loop, and discovery logs.
- `compiler/mesh-rt/src/dist/node.rs` — Added discovery bootstrap wiring, bracketed IPv6-safe node parsing, tuple-socket bind/connect behavior, and handshake node-name validation before session registration.
- `compiler/mesh-rt/src/dist/mod.rs` — Exported the new discovery module from the distributed runtime surface.
- `cluster-proof/config.mpl` — Added the env contract, advertised-identity builder, and config validation for cluster vs standalone startup.
- `cluster-proof/cluster.mpl` — Implemented the truthful `/membership` payload from `Node.self()` and `Node.list()`, including the typed JSON workaround for the final response shape.
- `cluster-proof/main.mpl` — Started the runtime once, emitted startup logs, and exposed the single read-only HTTP membership endpoint.
- `cluster-proof/tests/config.test.mpl` — Added pure config tests for malformed identity and missing-cookie cases.
- `compiler/meshc/tests/e2e_m039_s01.rs` — Added convergence and node-loss e2e proofs, early child-exit failure handling, and durable per-node stdout/stderr capture.
- `scripts/verify-m039-s01.sh` — Added the canonical local replay wrapper with phase-report output and fail-closed non-zero test-count checks.
- `.gsd/PROJECT.md` — Updated current project state to reflect that M039/S01 now proves local DNS discovery and truthful membership, while later distributed slices remain open.
- `.gsd/KNOWLEDGE.md` — Recorded the `cluster-proof` payload workaround and the fail-closed verifier rule for future M039 work.
