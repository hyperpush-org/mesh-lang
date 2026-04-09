---
estimated_steps: 18
estimated_files: 4
skills_used:
  - best-practices
  - multi-stage-dockerfile
---

# T03: Tighten the small-env operator contract

Make the same-image operator rail fail closed at the smallest honest boundary: valid primary/standby/stale-primary env should keep working, while contradictory continuity role/epoch input should fail before ambiguous runtime startup.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `cluster-proof` entrypoint/config startup | Emit a precise config error and stop before the package starts serving misleading operator surfaces. | N/A | Reject blank or contradictory role/epoch env instead of letting startup drift into a later failure. |
| `cluster-proof/tests/config.test.mpl` regression suite | Treat any failure as proof that the small-env contract changed unexpectedly and stop before reusing the verifier. | N/A | N/A |

## Negative Tests

- **Malformed inputs**: blank role, invalid role, blank epoch, non-integer epoch, and standby-with-epoch-1 startup must fail closed.
- **Error paths**: partial cluster identity env and contradictory continuity env must report a concrete config error without leaking secrets.
- **Boundary conditions**: valid primary, standby, and stale-primary restart env must still boot unchanged so the packaged verifier stays honest.

## Steps

1. Tighten `cluster-proof/docker-entrypoint.sh` so same-image cluster mode rejects missing or contradictory `MESH_CONTINUITY_ROLE` / `MESH_CONTINUITY_PROMOTION_EPOCH` combinations early while preserving standalone mode and the HOSTNAME/Fly identity fallback.
2. Update `cluster-proof/tests/config.test.mpl` (and `cluster-proof/config.mpl` only if needed) so valid primary/standby/stale-primary env and invalid role/epoch combinations stay covered.
3. Keep `scripts/verify-m043-s03.sh` aligned with the improved startup failure surface so operator misconfiguration points to the right log/artifact path without logging `CLUSTER_PROOF_COOKIE`.

## Must-Haves

- [ ] Valid primary, standby, and stale-primary restart flows still boot unchanged.
- [ ] Invalid continuity role/epoch env fails before ambiguous runtime startup.
- [ ] No verifier or entrypoint path prints `CLUSTER_PROOF_COOKIE` or similar secrets.

## Inputs

- `cluster-proof/docker-entrypoint.sh`
- `cluster-proof/tests/config.test.mpl`
- `cluster-proof/config.mpl`
- `scripts/verify-m043-s03.sh`

## Expected Output

- `cluster-proof/docker-entrypoint.sh`
- `cluster-proof/tests/config.test.mpl`
- `scripts/verify-m043-s03.sh`

## Verification

cargo run -q -p meshc -- test cluster-proof/tests && bash scripts/verify-m043-s03.sh

## Observability Impact

Moves bad continuity role/epoch input to an earlier, clearer startup failure surface and keeps the packaged verifier pointed at the exact failing log/artifact instead of a later ambiguous runtime error.
