# S01: Keyed Submit/Status Contract on the Existing Proof Rail

**Goal:** Extend `cluster-proof` from the one-shot `/work` routing proof into a keyed submit/status contract with stable request-key identity, distinct attempt identity, truthful owner/replica visibility, and idempotent retry semantics on the existing proof rail.
**Demo:** After this: An operator submits keyed work to `cluster-proof`, polls keyed status, sees stable request-key vs attempt identity information plus owner/replica placeholders or initial assignment data, and retries the same key on a healthy cluster without duplicate completion leakage.

## Tasks
- [x] **T01: Attempted a keyed `/work` submit/status refactor and test rewrite, but the Mesh package still fails to compile.** — Add the in-memory keyed contract that later continuity work will build on. Parse keyed submit input, bind a stable request key to one logical payload, create distinct attempt identities, record durable status snapshots keyed by request, preserve truthful owner/replica fields, and fail-close conflicting same-key reuse. Keep the route-selection logic reusable for later replica-backed admission instead of burying the new state model inside one handler.
  - Estimate: 2h
  - Files: cluster-proof/main.mpl, cluster-proof/work.mpl, cluster-proof/tests/work.test.mpl, cluster-proof/tests/config.test.mpl
  - Verify: cargo run -q -p meshc -- test cluster-proof/tests
cargo run -q -p meshc -- build cluster-proof
- [x] **T02: Recovered `cluster-proof` keyed runtime compilation, but the keyed contract tests still fail and the T02 e2e/verifier artifacts remain unfinished.** — Add real end-to-end proof for the new contract. Reuse the existing cluster-proof process harness to submit keyed work, poll status, assert stable request-vs-attempt identity, confirm owner/replica visibility fields, and prove same-key retry does not create duplicate completion on a healthy cluster. Capture the contract in a dedicated M040 e2e test file and a repo-root verifier script that archives JSON/log evidence for future slices.
  - Estimate: 2h
  - Files: compiler/meshc/tests/e2e_m040_s01.rs, scripts/verify-m040-s01.sh, cluster-proof/main.mpl, cluster-proof/work.mpl
  - Verify: cargo test -p meshc --test e2e_m040_s01 -- --nocapture
bash scripts/verify-m040-s01.sh
