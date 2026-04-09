---
estimated_steps: 4
estimated_files: 4
skills_used: []
---

# T08: Added scripts/verify-m044-s02.sh as a fail-closed S02 rail and recorded that the declared-runtime substrate it expects is still missing.

1. Rewrite scripts/verify-m044-s02.sh to replay the S01 rail, refresh mesh-rt, run the named metadata/declared_work/service/cluster_proof filters, then rebuild and retest cluster-proof in order.
2. Fail closed on missing running-N-test evidence or zero-test filters, and preserve per-phase logs plus copied e2e bundles under .tmp/m044-s02/verify/.
3. Add narrow absence checks proving the new declared-runtime submit/status hot path in cluster-proof/work_continuity.mpl no longer depends on current_target_selection(...) or direct Node.spawn(...), while allowing the explicitly legacy surfaces that survive until S05.
4. Treat bash scripts/verify-m044-s02.sh as the slice stop condition and align the named test prefixes in compiler/meshc/tests/e2e_m044_s02.rs with the verifier.

## Inputs

- `scripts/verify-m044-s01.sh`
- `compiler/meshc/tests/e2e_m044_s02.rs`
- `cluster-proof/work_continuity.mpl`

## Expected Output

- `scripts/verify-m044-s02.sh proves S01 replay, S02 named rails, cluster-proof dogfood, and scoped hot-path absence checks`
- `The verifier fails closed on zero-test filters and preserves retained artifacts under .tmp/m044-s02/verify/`
- `bash scripts/verify-m044-s02.sh is the authoritative S02 stop condition`

## Verification

bash scripts/verify-m044-s02.sh
