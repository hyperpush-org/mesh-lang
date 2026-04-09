---
estimated_steps: 4
estimated_files: 4
skills_used:
  - test
---

# T01: Bound safe auto-promotion to provable node-loss conditions

**Slice:** S04 — Bounded Automatic Promotion
**Milestone:** M044

## Description

R067 starts at the disconnect path. This task adds the runtime-owned decision boundary for when a standby may promote itself after peer loss, and it makes the refusal path explicit and inspectable instead of leaving later work to guess why failover did or did not happen.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Runtime node membership/session truth in `compiler/mesh-rt/src/dist/node.rs` | Refuse promotion and emit an explicit bounded-failover diagnostic. | Treat the peer as ambiguous and stay fenced/standby. | Reject the peer/membership state as unsafe and record the refusal reason. |
| Continuity authority projection in `compiler/mesh-rt/src/dist/continuity.rs` | Keep the existing authority and surface a fail-closed reason. | Do not promote until the state transition completes locally. | Reject inconsistent mirrored/epoch truth instead of promoting anyway. |
| Structured diagnostics in `compiler/mesh-rt/src/dist/operator.rs` | Preserve stderr continuity logs and return an explicit recording failure for tests. | Keep promotion logic bounded; diagnostics failure must not force promotion. | Drop malformed diagnostic payloads rather than polluting the buffer. |

## Load Profile

- **Shared resources**: live node session state, the continuity registry, and the bounded operator-diagnostic ring buffer.
- **Per-operation cost**: one disconnect scan plus one authority/record projection pass and bounded diagnostic writes.
- **10x breakpoint**: peer flapping and large mirrored record sets will stress disconnect-time scans first; the decision path must stay bounded rather than doing unbounded replay work here.

## Negative Tests

- **Malformed inputs**: invalid authority role/epoch combinations, malformed peer identity/session state, and inconsistent mirrored record shapes.
- **Error paths**: primary loss with no mirrored record, ambiguous or multi-peer loss, and stale higher-epoch truth already present on rejoin.
- **Boundary conditions**: healthy two-node mirror, empty continuity registry, and standby records that should stay degraded instead of promoting.

## Steps

1. Define the bounded safety rule for automatic promotion from the real disconnect path in `compiler/mesh-rt/src/dist/node.rs`, using live peer/session truth instead of proof-app heuristics.
2. Extend `compiler/mesh-rt/src/dist/continuity.rs` so safe promotion advances authority and epoch while ambiguous cases fail closed with explicit reasons.
3. Emit structured operator diagnostics and correlated `mesh-rt continuity` log lines for both auto-promotion and auto-promotion refusal.
4. Add runtime and destructive e2e coverage that proves safe promotion and ambiguous refusal without any manual promote call.

## Must-Haves

- [ ] A standby only auto-promotes when the runtime can prove the supported one-primary/one-standby transition is safe from current session and mirrored continuity truth.
- [ ] Ambiguous or insufficient state refuses promotion explicitly instead of inferring safety from peer disappearance alone.
- [ ] Operator diagnostics and retained test artifacts make the promote-vs-refuse decision legible after the fact.

## Verification

- `cargo test -p mesh-rt automatic_promotion_ -- --nocapture`
- `cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_promotion_ -- --nocapture`

## Observability Impact

- Signals added/changed: structured auto-promotion and auto-promotion-refused diagnostics plus correlated `mesh-rt continuity` transition logs.
- How a future agent inspects this: rerun the named runtime/e2e filters and inspect the retained `e2e_m044_s04` artifact bundle for request key, attempt id, role, epoch, and refusal reason.
- Failure state exposed: why the standby promoted or refused, including the exact ambiguity/fencing reason.

## Inputs

- `compiler/mesh-rt/src/dist/node.rs` — current disconnect handling and live peer/session truth.
- `compiler/mesh-rt/src/dist/continuity.rs` — current authority mutation and owner-loss/degraded record projection.
- `compiler/mesh-rt/src/dist/operator.rs` — existing structured diagnostic buffer used by `meshc cluster diagnostics`.
- `compiler/meshc/tests/e2e_m043_s03.rs` — the current destructive same-image proof shape that still assumes manual promotion.

## Expected Output

- `compiler/mesh-rt/src/dist/node.rs` — bounded auto-promotion trigger and safety gate.
- `compiler/mesh-rt/src/dist/continuity.rs` — safe promotion/refusal record transitions and tests.
- `compiler/mesh-rt/src/dist/operator.rs` — structured diagnostics for promote vs refuse.
- `compiler/meshc/tests/e2e_m044_s04.rs` — new S04 proof file with safe and ambiguous promotion assertions.
