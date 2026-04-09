---
estimated_steps: 4
estimated_files: 4
skills_used:
  - rust-best-practices
  - test
---

# T01: Add the runtime-owned DNS discovery reconcile loop and pin the candidate/identity contract

**Slice:** S01 — General DNS Discovery & Membership Truth
**Milestone:** M039

## Description

Close the real blocker first: discovery has to live in `mesh-rt`, not in Mesh application code, and it has to connect by candidate socket address while preserving the remote node’s advertised identity from the handshake. Implement the first provider as plain DNS A/AAAA lookup plus a fixed cluster port, but make the reconcile logic explicit and testable instead of burying it in ad hoc peer-list side effects.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| System DNS resolution for the configured seed name | Keep existing sessions untouched, log the failing seed/reason, and retry on the next reconcile tick. | Record a skipped reconcile and leave current membership intact instead of blocking the runtime. | Discard unusable addresses and keep discovery state truthful rather than synthesizing fake peers. |
| Outbound `mesh_node_connect` + handshake | Leave the session table unchanged and log the target/reason so the next tick can retry cleanly. | Back off to the next reconcile interval instead of spawning unbounded connect threads. | Reject invalid advertised names or cookie mismatches without mutating membership truth. |

## Load Profile

- **Shared resources**: DNS lookups, the node session table, and reconcile-triggered outbound connect attempts.
- **Per-operation cost**: one DNS lookup plus up to N filtered connection attempts per reconcile tick.
- **10x breakpoint**: too-frequent reconcile intervals or large answer sets would cause duplicate dial churn first, so filtering and dedupe must happen before connect.

## Negative Tests

- **Malformed inputs**: blank discovery seed, invalid cluster port, zero/negative reconcile interval, and malformed candidate host strings.
- **Error paths**: DNS returns no answers, connect target refuses handshake, or the same candidate appears repeatedly across ticks.
- **Boundary conditions**: duplicate A/AAAA answers, self-address candidates, already-connected peers, and bracketed IPv6 literals in advertised names.

## Steps

1. Add `compiler/mesh-rt/src/dist/discovery.rs` with config parsing, candidate normalization, dedupe, self/connected filtering, and the periodic reconcile loop for one DNS seed name plus fixed cluster port.
2. Wire discovery startup from `mesh_node_start` in `compiler/mesh-rt/src/dist/node.rs`, reusing `mesh_node_connect` with synthesized temporary targets so the handshake-provided remote name remains the membership source of truth.
3. Emit discovery logs that expose provider, seed, accepted/rejected candidates, and last failure reason without ever echoing the shared cookie.
4. Add unit coverage in the discovery module for candidate filtering, duplicate suppression, self-filtering, and IPv6/bracketed-name handling.

## Must-Haves

- [ ] DNS discovery runs inside `mesh-rt` and does not require Mesh code to hand-roll peer resolution.
- [ ] Discovery candidates are filtered against self and already-connected peers before any dial attempt.
- [ ] Advertised node identity remains the canonical membership truth even though discovery starts from a shared seed hostname.

## Verification

- `cargo test -p mesh-rt discovery_ -- --nocapture`

## Observability Impact

- Signals added/changed: discovery reconcile logs with candidate counts and reject reasons.
- How a future agent inspects this: rerun `cargo test -p mesh-rt discovery_ -- --nocapture` and inspect runtime logs from `scripts/verify-m039-s01.sh`.
- Failure state exposed: whether convergence broke in DNS resolution, candidate filtering, or outbound connect/handshake.

## Inputs

- `.gsd/milestones/M039/slices/S01/S01-RESEARCH.md` — research constraints on identity, DNS-first discovery, and the proof boundary.
- `compiler/mesh-rt/src/dist/node.rs` — current node transport, peer-list gossip, and connect/query seams.
- `compiler/mesh-rt/src/dist/mod.rs` — distribution module exports.
- `mesher/main.mpl` — current manual env-driven startup reference to replace at the runtime layer.

## Expected Output

- `compiler/mesh-rt/src/dist/discovery.rs` — runtime-owned DNS discovery config, filtering, and reconcile loop.
- `compiler/mesh-rt/src/dist/node.rs` — discovery bootstrap and candidate-address connect integration.
- `compiler/mesh-rt/src/dist/mod.rs` — discovery module export wiring.
