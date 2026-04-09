# S01: General DNS Discovery & Membership Truth — UAT

**Milestone:** M039
**Written:** 2026-03-28T10:09:03.342Z

# S01: General DNS Discovery & Membership Truth — UAT

**Milestone:** M039
**Written:** 2026-03-28

## UAT Type

- UAT mode: live-runtime
- Why this mode is sufficient: S01’s contract is runtime discovery and truthful membership under real multi-process conditions. Compile-only checks are not enough.

## Preconditions

- Run from the repo root with Cargo/Rust available.
- Do not supply any manual peer-list configuration; the proof must rely on the DNS discovery seed only.
- The host should resolve both `127.0.0.1` and `::1` for `localhost`; the local proof uses dual-stack `localhost` with distinct advertised IPv4/IPv6 identities.

## Smoke Test

1. Run `bash scripts/verify-m039-s01.sh`.
2. **Expected:** The command exits 0 and `.tmp/m039-s01/verify/phase-report.txt` records `passed` for `build-cluster-proof`, `mesh-rt-discovery`, `convergence`, and `node-loss`.

## Test Cases

### 1. Runtime discovery contract builds and its filtering coverage is live

1. Run `cargo run -q -p meshc -- build cluster-proof`.
2. Run `cargo test -p mesh-rt discovery_ -- --nocapture`.
3. **Expected:** `cluster-proof/cluster-proof` is produced, and the six `discovery_` tests pass, covering invalid config rejection, duplicate suppression, self-filtering, and bracketed IPv6 handling.

### 2. Two nodes converge automatically without a manual peer list

1. Run `cargo test -p meshc --test e2e_m039_s01 e2e_m039_s01_converges_without_manual_peers -- --nocapture`.
2. Inspect the newest `.tmp/m039-s01/e2e-m039-s01-converges-*/node-a.stderr.log` and `node-b.stderr.log` files.
3. **Expected:** Cargo reports `running 1 test` and the test passes. The per-node logs show `mesh discovery:` lines with one accepted target, one `:self` rejected target, and the proof endpoint converges to `membership` length 2 with each node reporting itself plus one peer.

### 3. Membership shrinks truthfully after one node is lost

1. Run `cargo test -p meshc --test e2e_m039_s01 e2e_m039_s01_membership_updates_after_node_loss -- --nocapture`.
2. Inspect the newest `.tmp/m039-s01/e2e-m039-s01-node-loss-*/node-a.stdout.log` and `node-a.stderr.log` files.
3. **Expected:** Cargo reports `running 1 test` and the test passes. After node B is killed, the surviving node reports `membership` length 1, `peers` length 0, and logs still show truthful discovery/filtering rather than stale peer state.

### 4. The assembled slice proof stays green end to end

1. Run `cargo test -p meshc --test e2e_m039_s01 -- --nocapture`.
2. Run `bash scripts/verify-m039-s01.sh`.
3. **Expected:** The full e2e file reports `running 2 tests` and passes. The wrapper exits 0, and the phase report still shows all four phases as passed.

## Edge Cases

### Malformed config fails before live runtime startup

1. Run `cargo run -q -p meshc -- test cluster-proof/tests`.
2. **Expected:** The config tests pass, proving blank/malformed identity and missing-cookie cases are rejected by the pure config layer rather than becoming dishonest cluster startup.

### Dual-stack localhost self-filtering stays truthful

1. Run `cargo test -p mesh-rt discovery_ -- --nocapture`.
2. Inspect the newest `.tmp/m039-s01/e2e-m039-s01-converges-*/node-*.stderr.log` files.
3. **Expected:** The runtime tests pass, and the recent node logs show one accepted target and one rejected `:self` target for the shared `localhost` seed instead of double-counting both local addresses as peers.

## Failure Signals

- `bash scripts/verify-m039-s01.sh` exits non-zero or `.tmp/m039-s01/verify/phase-report.txt` is missing a `passed` line.
- Cargo reports `running 0 tests` for any named `e2e_m039_s01` filter.
- Per-node stderr logs do not contain `mesh discovery:` lines or show malformed membership identities being accepted.
- The `/membership` response shape loses `self`, `peers`, or `membership`, or a surviving node still reports a dead peer after the node-loss proof.

## Requirements Proved By This UAT

- R045 — proves the local portion of automatic cluster formation through the general DNS discovery seam without manual peer lists.
- R046 — proves the local join/loss portion of truthful membership reporting; clean rejoin remains later work.

## Not Proven By This UAT

- R047 runtime-native work routing across the cluster.
- Fly-backed one-image operator proof and public docs truth.
- Clean rejoin after node loss or the broader continuity guarantees planned for later milestones.

## Notes for Tester

- The local proof is intentionally dual-stack: one node advertises IPv4 and the other advertises IPv6, both seeded from `localhost`.
- `/membership` currently reaches the final `self` JSON key through a local typed-payload workaround. Treat payload-shape regressions here as real runtime bugs, not as harmless harness noise.
- When debugging failures, start with `.tmp/m039-s01/verify/phase-report.txt` and the newest per-node stderr logs; they are the most authoritative signals for whether the break is in build, discovery, convergence, or node-loss shrinkage.
