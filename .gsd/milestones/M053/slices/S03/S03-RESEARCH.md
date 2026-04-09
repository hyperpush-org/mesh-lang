# S03 Research — Hosted evidence chain fails on starter deploy or packages drift

## Summary

- **Primary requirement ownership:** S03 primarily owns **R121** (packages site must be part of the normal CI/deploy contract). It also operationalizes the hosted side of **R122** by making the generated Postgres starter’s deploy/failover proof part of hosted evidence rather than a local-only retained rail.
- **Upstream constraints still apply:**
  - **R115/R122:** keep the generated Postgres starter as the serious deployable path, with SQLite still explicitly local-only.
  - **R116/R117:** hosted proof must stay **starter-owned** and evaluator-facing; do not replace it with a proof-app or maintainer-only rail.
  - **R120:** packages/docs/landing must remain one coherent public story.
- **Current hosted proof is split in two:**
  - starter/release truth lives in `.github/workflows/authoritative-live-proof.yml`, `.github/workflows/authoritative-verification.yml`, and `.github/workflows/release.yml`
  - packages deploy/public-surface truth lives in `.github/workflows/deploy-services.yml` plus `scripts/lib/m034_public_surface_contract.py`
- **There is no M053 hosted chain yet.** Nothing in `.github/workflows/` mentions `verify-m053-s01.sh` or `verify-m053-s02.sh`, and there is no slice-owned M053 hosted verifier script yet.
- **Important gap:** `scripts/verify-m034-s05.sh` already checks hosted workflows via `gh run list/view`, but its `workflow_specs` treat `deploy-services.yml` as a **tag** proof (`requiredHeadBranch = binaryTag`), not a **mainline** proof. That means current remote evidence still does not make packages drift a main-branch ownership signal.
- **Hosted starter proof is technically ready:** `scripts/verify-m053-s02.sh` only needs one admin-style `DATABASE_URL`; it creates isolated databases itself via `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs::create_isolated_database(...)`. A GitHub Actions Postgres service container is sufficient; Fly is not required for the serious-starter hosted gate.
- **Do not stuff S02 into the existing live-proof job without redesign.** `authoritative-live-proof.yml` is currently budgeted for the M034 publish proof (`timeout-minutes: 45`, proof step `timeout-minutes: 35`), while `scripts/verify-m053-s02.sh` internally gives up to **3600s** for the S01 replay and **5400s** for the S02 failover e2e. Even if the real run is shorter, the current workflow budget is the wrong shape.

## Skills Discovered

Relevant installed skills already present:

- `github-workflows`
- `flyio-cli-public`

No additional skill installs were needed.

### Skill-informed implementation rules

- From **`github-workflows`**: treat workflow work as an **observable contract**, not a syntax exercise. The planner should require run-level evidence (job names, step names, expected refs/head SHAs, pass/fail surfaces), not “YAML parses” as success.
- From **`flyio-cli-public`**: keep Fly as a **reference proof environment**, not the product contract. S03 should not move the serious starter proof onto Fly-only semantics or make Fly the public dependency for R122.

## Recommendation

1. **Add a separate hosted starter-proof reusable workflow** for `bash scripts/verify-m053-s02.sh` instead of expanding `authoritative-live-proof.yml` directly.
   - Reason: current M034 live-proof workflow is tightly pinned by local contract verifiers and has a much smaller timeout/profile.
   - The new workflow should provision Postgres locally on the runner (service container), export `DATABASE_URL`, run `bash scripts/verify-m053-s02.sh`, and upload `.tmp/m053-s02/**` diagnostics on failure.

2. **Wire that hosted starter-proof workflow into the normal main/tag workflow graph** through:
   - `.github/workflows/authoritative-verification.yml` for mainline/trusted PR visibility
   - `.github/workflows/release.yml` for tag/release gating

3. **Keep packages verification in `deploy-services.yml` and keep using `scripts/lib/m034_public_surface_contract.py`.**
   - Do **not** re-inline curls/grep into YAML. `scripts/verify-m034-s05-workflows.sh` and `scripts/tests/verify-m034-s05-contract.test.mjs` explicitly ban the old inline proof style.

4. **Add a slice-owned hosted-evidence verifier** that reuses the M034 remote-evidence pattern (`gh run list/view` + `git ls-remote` freshness matching) to prove that:
   - the new hosted starter-proof lane is green on the expected ref
   - `deploy-services.yml` is green on the expected ref
   - packages-site health/public-surface failures or starter deploy/failover failures invalidate the hosted evidence contract

5. **Treat advisory requirement #2 as the actual operational contract for S03.**
   - “Hosted mainline evidence must fail when starter deploy proof or packages-site public surface breaks” is the sharpest expression of the slice.
   - Advisory #3 is mostly S04 wording work, not S03.

## Implementation Landscape

### Existing starter-proof side

- **`scripts/verify-m053-s01.sh`**
  - Replays the generated Postgres starter deploy truth.
  - Publishes `.tmp/m053-s01/verify/status.txt`, `current-phase.txt`, `phase-report.txt`, `latest-proof-bundle.txt`, and a retained bundle.
- **`scripts/verify-m053-s02.sh`**
  - Replays S01 first, then the clustered failover proof.
  - Publishes the same stable marker pattern under `.tmp/m053-s02/verify/` plus a retained failover bundle.
  - This is the strongest existing seam for hosted gating because downstream code can key on `status.txt`, `phase-report.txt`, and `latest-proof-bundle.txt` instead of guessing from raw logs.
- **`compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs`**
  - `create_isolated_database(...)` derives and creates per-run databases from one admin `DATABASE_URL`.
  - This is why a GitHub Actions Postgres service container is enough.
- **`compiler/meshc/tests/support/m053_todo_postgres_deploy.rs`**
  - Keeps the two-node staged runtime helper layer isolated from workflow concerns. S03 should consume the verifier scripts, not re-encode runtime behavior in YAML.

### Existing hosted-proof side

- **`.github/workflows/authoritative-live-proof.yml`**
  - Reusable `workflow_call` workflow.
  - Currently the **only** workflow file allowed to directly run `bash scripts/verify-m034-s01.sh`.
  - Pinned by `scripts/verify-m034-s02-workflows.sh` to one job named `Authoritative live proof`.
- **`.github/workflows/authoritative-verification.yml`**
  - Current mainline/trusted-PR caller workflow.
  - Currently only `whitespace-guard` + one reusable `live-proof` job.
  - Any new hosted starter-proof job here will require matching updates in `scripts/verify-m034-s02-workflows.sh`.
- **`.github/workflows/release.yml`**
  - Current tag workflow.
  - Reuses `authoritative-live-proof.yml` and gates `Create Release` on it.
  - Current exact job set is pinned by `scripts/verify-m034-s02-workflows.sh`.

### Existing packages/deploy side

- **`.github/workflows/deploy-services.yml`**
  - Already deploys `registry/`, `packages-website/`, and `mesher/landing/`.
  - `health-check` already runs `python3 scripts/lib/m034_public_surface_contract.py public-http ...`.
  - This already makes packages-site public-surface drift fail the **deploy-services** workflow itself.
- **`scripts/lib/m034_public_surface_contract.py`**
  - Single source of truth for packages/docs/installers public contract.
  - `public-http` checks installers, docs pages, packages detail page, packages search page, and registry search API.
  - Reuse this; do not duplicate the probes.
- **`scripts/verify-m034-s05-workflows.sh`**
  - Exact workflow contract sweep for `deploy.yml` and `deploy-services.yml`.
  - It requires the packages/landing checks to stay in named jobs/steps and explicitly rejects legacy inline proof logic.
- **`scripts/tests/verify-m034-s05-contract.test.mjs`**
  - Node contract test that locks the shared helper ownership, required steps, and remote-evidence semantics.

### Existing remote-evidence seam

- **`scripts/verify-m034-s05.sh`**
  - Already contains the reusable machinery for hosted evidence:
    - derives expected refs/tags from repo sources
    - resolves expected remote SHA with `git ls-remote`
    - fetches latest hosted run with `gh run list`
    - validates job/step presence with `gh run view`
    - records freshness status (`expectedHeadSha`, `observedHeadSha`, `headShaMatchesExpected`)
  - This is the right pattern to reuse for S03.
  - **Critical current limitation:** its `deploy-services.yml` spec expects the **binary tag** run, not the **main** run.

### Existing workflow/test patterns worth copying

- **`scripts/tests/verify-m049-s05-contract.test.mjs`**
  - Good example of a fail-closed Node contract test for an assembled verifier script.
- **`scripts/tests/verify-m034-s06-contract.test.mjs`**
  - Good example of a fail-closed harness around a remote-evidence/archive wrapper.

## Natural Seams for Planning

### Seam 1 — Hosted starter-proof workflow
Files likely involved:
- `.github/workflows/` (new reusable workflow file)
- `.github/workflows/authoritative-verification.yml`
- `.github/workflows/release.yml`
- `scripts/verify-m034-s02-workflows.sh`

Why separate:
- runtime/DB/timeout profile is materially different from the M034 publish proof
- avoids overloading the existing reusable live-proof contract

### Seam 2 — Packages deploy contract stays centralized
Files likely involved:
- `.github/workflows/deploy-services.yml`
- `scripts/verify-m034-s05-workflows.sh`
- `scripts/tests/verify-m034-s05-contract.test.mjs`
- `scripts/lib/m034_public_surface_contract.py`

Why separate:
- packages-site drift is already owned here; S03 should strengthen ownership and evidence, not reimplement surface checks elsewhere

### Seam 3 — Slice-owned hosted evidence verifier
Files likely involved:
- `scripts/verify-m053-s03.sh` (new)
- `scripts/tests/verify-m053-s03-contract.test.mjs` (new)
- possibly targeted updates to `README.md` / tooling docs only if discoverability of the new rail matters

Why separate:
- this is where “starter proof + packages drift must fail the hosted chain” can be expressed clearly without contaminating lower-level verifier responsibilities
- can reuse the proven M034 remote-evidence/freshness pattern instead of scraping Actions pages or hand-rolling ad hoc checks

## Constraints and Fragilities

- **Timeout mismatch is real.** Do not assume the current 35-minute proof step can absorb `verify-m053-s02.sh`.
- **Workflow contract verifiers are rigid.**
  - `scripts/verify-m034-s02-workflows.sh` expects exact job sets for the authoritative workflows.
  - `scripts/verify-m034-s05-workflows.sh` expects exact job sets/step names for deploy-services.
  - Any workflow-graph change without verifier updates will fail immediately.
- **Packages-site Docker packaging is a known footgun.**
  - `.gsd/KNOWLEDGE.md` already records that `packages-website` must keep the `npm ci -> npm run build -> npm prune --omit=dev` builder pattern.
  - Do not reintroduce runtime `npm install --omit=dev --ignore-scripts`.
- **Do not let Fly leak into the public starter contract.**
  - Keep Fly reference-only per milestone context and the `flyio-cli-public` skill boundary.
  - S03 should prove the starter on hosted CI with runner-local Postgres, not by rewriting the contract as “Fly is the deploy path.”
- **Remote workflow evidence must be freshness-aware.**
  - The M034 helper’s `git ls-remote` + `gh run view` pattern is there for a reason: stale green runs are not valid proof.
- **If a new workflow file is introduced, remote evidence will not see it until it lands on remote `main`.**
  - `.gsd/KNOWLEDGE.md` already notes that `gh run list --workflow <file>` returning `workflow not found` can simply mean the file has not shipped yet.

## Verification Plan

Use the lightest sufficient layers in this order:

1. **Local workflow contract checks**
   - `bash scripts/verify-m034-s02-workflows.sh`
   - `bash scripts/verify-m034-s05-workflows.sh`

2. **Local contract tests for verifier ownership**
   - `node --test scripts/tests/verify-m034-s05-contract.test.mjs`
   - add and run a new `node --test scripts/tests/verify-m053-s03-contract.test.mjs`

3. **Starter-proof replay with disposable local Postgres**
   - run a disposable Docker Postgres
   - export `DATABASE_URL`
   - `bash scripts/verify-m053-s02.sh`
   - confirm `.tmp/m053-s02/verify/status.txt = ok`, `current-phase.txt = complete`, and `latest-proof-bundle.txt` points at a retained bundle

4. **Hosted evidence verification**
   - require explicit run-level evidence: workflow file, expected ref, expected SHA, observed SHA, required job names, required step names, final conclusion
   - planner should prefer the existing `gh run list/view` freshness pattern already used in `scripts/verify-m034-s05.sh`

5. **Packages public-surface verification**
   - keep using `python3 scripts/lib/m034_public_surface_contract.py public-http ...`
   - do not replace with workflow-inline probes

## Planner Notes

- The most important design decision is **whether the hosted starter proof gets its own reusable workflow**. Repo evidence strongly favors **yes** because of timeout shape, current verifier pinning, and separation from the M034 publish proof.
- The second important decision is **where the hosted cross-workflow truth is assembled**. The existing codebase already has a strong answer: use a slice-owned verifier that consumes stable workflow/job/step evidence, not loose log-grepping.
- The smallest safe path is:
  1. add starter-proof hosted workflow
  2. wire it into authoritative main/tag workflows
  3. extend hosted evidence verification to include it plus packages deploy health
  4. pin the whole contract with a new Node contract test
