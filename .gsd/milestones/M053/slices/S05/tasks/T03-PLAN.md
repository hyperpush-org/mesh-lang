---
estimated_steps: 1
estimated_files: 7
skills_used: []
---

# T03: Root-cause the authoritative starter-proof failure and retain CI-grade diagnostics.

Use the retained authoritative-starter-failover-proof diagnostics from T02 plus the shipped main SHA to reproduce the failing starter-proof path in a clean, CI-like environment. Drive the failure down to an explicit class (timeout/compile-budget, product assertion, or environment drift) by rerunning the nested S01/S02 entrypoints or the targeted cargo test with cold caches as needed, and preserve the full inner Rust/test logs under .tmp/m053-s05/starter-proof-repro/ instead of relying on the truncated workflow artifact. Write a short root-cause note that names the exact failing command, evidence log, and any diagnostic-retention gaps that must be fixed before rerunning hosted workflows.

## Inputs

- `.tmp/m053-s05/rollout/authoritative-starter-failover-proof-diagnostics/verify/`
- `scripts/verify-m053-s02.sh`
- `scripts/verify-m053-s01.sh`
- `compiler/meshc/tests/e2e_m049_s03.rs`
- `.tmp/m053-s05/rollout/main-shipped-sha.txt`

## Expected Output

- `.tmp/m053-s05/starter-proof-repro/root-cause.md`
- `.tmp/m053-s05/starter-proof-repro/ci-failure-classification.json`
- `.tmp/m053-s05/starter-proof-repro/`

## Verification

test -s .tmp/m053-s05/starter-proof-repro/root-cause.md && python3 - <<'PY'
import json, pathlib
root = pathlib.Path('.tmp/m053-s05/starter-proof-repro')
data = json.loads(root.joinpath('ci-failure-classification.json').read_text())
assert data['failure_class'] in {'timeout', 'assertion', 'environment'}
assert data['failing_command']
log_path = pathlib.Path(data['primary_log'])
assert log_path.exists()
assert log_path.stat().st_size > 0
PY
