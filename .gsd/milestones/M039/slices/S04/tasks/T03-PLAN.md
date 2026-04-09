---
estimated_steps: 4
estimated_files: 4
skills_used:
  - flyio-cli-public
  - best-practices
---

# T03: Write the Fly runbook and read-only verifier

**Slice:** S04 — One-Image Operator Path, Local/Fly Verifiers, and Docs Truth
**Milestone:** M039

## Description

Add the operator-facing Fly surface without mutating external state: a package-local runbook that spells out the exact repo-root build/deploy commands and a read-only verifier that inspects an existing Fly deployment for the same cluster contract the local Docker path proves.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Fly CLI status/config/log inspection | fail the task with the exact missing-auth or missing-app context and do not attempt any state-changing fallback | fail the phase with the partial Fly output preserved under `.tmp/m039-s04/fly/` | reject partial or inconsistent machine/config output instead of guessing cluster truth |
| live `/membership` and `/work` probes against the Fly app | preserve the raw response bodies and fail closed with the URL/app context | fail with the last curl error or HTTP status after the bounded wait | reject missing membership/routing fields or self-only responses when the app is supposed to be clustered |
| runbook/operator commands | fail docs-verifier grep checks later rather than leaving an ambiguous operator story | N/A — static content | reject commands that imply `cd cluster-proof` build context or mutate Fly state without approval |

## Load Profile

- **Shared resources**: Fly API/log queries, one already-deployed app with multiple running machines, and the verifier artifact dir.
- **Per-operation cost**: a small number of `fly status`, `fly config show`, `fly logs` calls plus `/membership` and `/work` probes.
- **10x breakpoint**: CLI/log volume and slow remote app convergence before CPU; the verifier should bound waits and keep copied excerpts small and focused.

## Negative Tests

- **Malformed inputs**: missing `CLUSTER_PROOF_FLY_APP`, missing `CLUSTER_PROOF_BASE_URL`, or inconsistent app/URL pairing must fail before any proof attempt.
- **Error paths**: Fly auth failure, fewer than the required running machines, or stale auto-stop configuration must fail the verifier instead of downgrading to warnings.
- **Boundary conditions**: at least two running machines, truthful membership including peers, and `/work` showing remote routing in the live cluster.

## Steps

1. Write `cluster-proof/README.md` as the canonical operator runbook for the image, including repo-root Docker build, local two-container usage, the exact `fly deploy . --config cluster-proof/fly.toml --dockerfile cluster-proof/Dockerfile` command, and the read-only Fly verification path.
2. Implement `scripts/verify-m039-s04-fly.sh` so it requires an existing deployed app and base URL, uses only read-only Fly commands (`fly status`, `fly config show`, `fly logs`) plus `/membership` and `/work` probes, and archives evidence under `.tmp/m039-s04/fly/`.
3. Provide a non-live usage/help path so the script can be syntax-checked in-repo, but keep the real verification mode fail-closed when auth, machine count, config, or cluster truth drift.
4. Finish only when the runbook and verifier agree on the exact commands, required inputs, and read-only constraint.

## Must-Haves

- [ ] `cluster-proof/README.md` is the deepest operator runbook for the one-image local/Fly path and uses the exact repo-root build/deploy commands.
- [ ] `scripts/verify-m039-s04-fly.sh` is read-only, requires an existing Fly app, and checks both Fly machine/config truth and the live `/membership` + `/work` contract.
- [ ] The in-repo verification path can check script syntax/help without live Fly access, while the live mode still fail-closes on real drift.

## Verification

- `bash -n scripts/verify-m039-s04-fly.sh`
- `bash scripts/verify-m039-s04-fly.sh --help`
- `rg -q "fly deploy \\. --config cluster-proof/fly.toml --dockerfile cluster-proof/Dockerfile" cluster-proof/README.md`

## Observability Impact

- Signals added/changed: `.tmp/m039-s04/fly/` status/config/log/probe artifacts and a read-only Fly verification phase ledger.
- How a future agent inspects this: rerun the script against an existing app or inspect the last `.tmp/m039-s04/fly/` bundle.
- Failure state exposed: whether the live drift is missing auth/app state, Fly machine/config truth, or the app’s `/membership` / `/work` contract.

## Inputs

- `cluster-proof/fly.toml` — one-image Fly runtime contract from T01.
- `scripts/lib/m039_cluster_proof.sh` — shared assertion/artifact helpers from T02.
- `reference-backend/README.md` — existing deep operator-runbook shape to mirror.
- `cluster-proof/config.mpl` — current env contract the runbook/verifier must describe accurately.

## Expected Output

- `cluster-proof/README.md` — canonical local/Fly operator runbook for `cluster-proof`.
- `scripts/verify-m039-s04-fly.sh` — read-only Fly verifier with help mode and archived evidence.
