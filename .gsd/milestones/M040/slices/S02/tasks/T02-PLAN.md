---
estimated_steps: 4
estimated_files: 7
skills_used:
  - rust-best-practices
  - best-practices
  - test
---

# T02: Canonicalize owner/replica placement and wire replica safety policy through the existing env rail

**Slice:** S02 — Replica-Backed Admission, Fail-Closed Policy, and Owner-Loss Convergence
**Milestone:** M040

## Description

The current routing rule still treats “first non-self peer in membership order” as truth, but `Node.list()` comes from hash-map iteration and is not a canonical durability contract. This task replaces that accidental rule with deterministic owner/replica placement that every node computes the same way from the same membership snapshot.

At the same time, add exactly one small durability-policy surface on the existing operator rail. The clustered proof should default to the two-node replica-safety bar without inventing a second deployment path. Validation must stay fail-closed in `cluster-proof/config.mpl`, and that same policy must flow through `cluster-proof/docker-entrypoint.sh` and `cluster-proof/fly.toml` rather than through a parallel script or image.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| canonical membership/placement helpers | reject the placement as invalid and fail closed instead of silently using raw `Node.list()` order | N/A — placement is synchronous | reject malformed/empty member identities instead of generating a fake owner/replica pair |
| env/config parsing | stop startup with an explicit config error | N/A — parsing is synchronous | reject partial/invalid durability-policy env instead of falling back to permissive behavior |
| operator bootstrap rail (`docker-entrypoint.sh`, `fly.toml`) | preserve the existing one-image startup contract and fail closed on bad env | N/A — shell/bootstrap path is synchronous | refuse ambiguous config rather than starting in an under-specified durability mode |

## Load Profile

- **Shared resources**: current membership snapshot, deterministic selection helper, and startup config env parsing.
- **Per-operation cost**: one canonical membership normalization plus one owner/replica selection per keyed submit.
- **10x breakpoint**: placement is cheap; the real risk is truth drift if multiple nodes derive different owner/replica pairs from the same cluster view.

## Negative Tests

- **Malformed inputs**: invalid or partial durability-policy env, malformed node names, or empty canonical membership after filtering.
- **Error paths**: under-replicated cluster mode, ambiguous placement, and startup with cluster hints but incomplete durability config.
- **Boundary conditions**: single-node fallback, two-node proof cluster, and mixed IPv4/IPv6 node names that still must sort/select deterministically.

## Steps

1. Add canonical membership/placement helpers so the same `request_key` and live membership always produce the same owner/replica pair on every node.
2. Replace the current first-peer routing contract and rewrite the existing work tests that encode raw membership order as truth.
3. Thread one small fail-closed durability-policy value through `cluster-proof/config.mpl`, `docker-entrypoint.sh`, and `fly.toml` on the existing operator rail.
4. Prove the new placement and config behavior with package tests plus a clean `cluster-proof` build.

## Must-Haves

- [ ] Owner/replica placement is deterministic across nodes and no longer depends on raw `Node.list()` ordering.
- [ ] Clustered startup fails closed when the durability policy is partial, invalid, or incompatible with the live membership.
- [ ] The operator-visible policy lives only on the existing env-driven rail.

## Verification

- Prove the updated placement/config contract inside the Mesh package.
- `cargo run -q -p meshc -- test cluster-proof/tests`
- `cargo run -q -p meshc -- build cluster-proof`

## Observability Impact

- Signals added/changed: startup and submit logs should surface the selected owner/replica pair and the effective durability mode or required replica count.
- How a future agent inspects this: inspect `cluster-proof` stdout plus the package tests in `cluster-proof/tests/work.test.mpl` and `cluster-proof/tests/config.test.mpl`.
- Failure state exposed: under-replication and bad config become explicit startup/submit errors instead of accidental routing behavior.

## Inputs

- `cluster-proof/work.mpl` — current route-selection and keyed submit contract.
- `cluster-proof/cluster.mpl` — membership helpers that currently expose non-canonical ordering.
- `cluster-proof/config.mpl` — existing fail-closed env/config seam.
- `cluster-proof/tests/work.test.mpl` — current tests that still enshrine first-peer membership order.
- `cluster-proof/tests/config.test.mpl` — config validation contract to extend with durability-policy cases.
- `cluster-proof/docker-entrypoint.sh` — one-image bootstrap path that must stay authoritative.
- `cluster-proof/fly.toml` — existing env rail where clustered proof config must remain visible.

## Expected Output

- `cluster-proof/cluster.mpl` — canonical membership/placement helpers suitable for deterministic owner/replica selection.
- `cluster-proof/work.mpl` — keyed routing logic updated to consume canonical owner/replica truth.
- `cluster-proof/config.mpl` — fail-closed durability-policy parsing and validation.
- `cluster-proof/tests/work.test.mpl` — placement tests rewritten around deterministic truth instead of raw peer order.
- `cluster-proof/tests/config.test.mpl` — new config tests for durability-policy validation.
- `cluster-proof/docker-entrypoint.sh` — bootstrap validation aligned with the new fail-closed policy surface.
- `cluster-proof/fly.toml` — existing env rail updated only as needed to expose the policy in the proof environment.
