# Repo Split Execution Plan

Status: execution in progress (product repo published; actual code separation still incomplete)
Owner target: `hyperpush-org`
Language repo target: `hyperpush-org/mesh-lang`
Product repo target: `hyperpush-org/hyperpush-mono`
Local requirement: preserve the current monorepo-like path shape on this machine without keeping product code authoritative in the language repo.

---

## Handoff status — 2026-04-08

### Current truth by repo

#### `hyperpush-org/hyperpush-mono`
- **Product code is now published on remote `main`.**
- Remote `main` currently points at `5bc43b31dae913418721e51896713ab8f39a9e92`.
- The pushed product-root work includes:
  - root `README.md` rewritten as the product extraction root
  - product `.github/dependabot.yml`
  - product `.github/workflows/deploy-landing.yml`
  - `scripts/verify-landing-surface.sh`
  - `mesher/scripts/verify-maintainer-surface.sh`
  - `mesher/scripts/{test,build,migrate,smoke}.sh`
  - Mesher toolchain fixes so sibling `mesh-lang/target` is reused for runtime lookup
  - Mesher README/verifier/e2e updates so the product repo proves its own maintainer surface from the product root
- In other words: **Hyperpush/Mesher now has a real product repo with real product-root proof surfaces on GitHub.**

#### `hyperpush-org/mesh-lang`
- **The actual code separation is not finished yet.**
- `mesh-lang` still tracks `mesher/` in the repo, so GitHub truth still contains a duplicate product tree.
- A prep branch was published to GitHub for the language repo:
  - branch: `m055-repo-split-core`
  - commit: `86268b913c227c5e140d1fb79402c21cc6dcfe01`
  - PR: `hyperpush-org/mesh-lang#1`
- That branch/PR contains the **split plumbing**, not the final extraction:
  - repo identity metadata
  - workspace contract docs
  - split-boundary verifier updates
  - local sibling-workspace helper
  - docs/install/release handoff updates
- That branch is useful because it makes `mesh-lang` understand the split, but **it does not yet remove the duplicated `mesher/` code from `mesh-lang`**.

### Completed in this working tree
- Canonical language repo identity was retargeted from `snowdamiz/mesh-lang` to `hyperpush-org/mesh-lang` in `scripts/lib/repo-identity.json`.
- Public/current language-owned slug surfaces were updated to the new canonical repo identity across installers, docs theme chrome, public starter links, package-site footer links, release/proof helpers, and the affected compiler/script contract tests.
- Language repo policy cleanup landed:
  - `/mesher/` was removed from `.github/CODEOWNERS`
  - `/mesher/landing` was removed from `.github/dependabot.yml`
- `scripts/verify-m055-s01.sh` now resolves the sibling product repo through `scripts/lib/m055-workspace.sh` and runs the landing build from the real product root instead of `mesh-lang/mesher`.
- Added `scripts/setup-local-workspace.sh` to verify sibling repo roots/remotes and create the local-only `mesh-lang/mesher -> ../hyperpush-mono/mesher` compatibility path.
- A real local sibling product repo was materialized at `../hyperpush-mono` with:
  - product root files from `scripts/fixtures/m055-s04-hyperpush-root/`
  - `mesher/` copied from the current language repo tree
  - a local git repo initialized on `main`
  - `origin` set to `https://github.com/hyperpush-org/hyperpush-mono.git`
- Local `mesh-lang` `origin` was repointed to `https://github.com/hyperpush-org/mesh-lang.git` after the GitHub repo appeared.
- Local `../hyperpush-mono` was normalized onto the real remote `origin/main` (`e5fb36a6fe7e9e56f3a608a608abbaaab6764167`) so S04 could stop reading an unborn-HEAD scratch repo.
- `scripts/verify-m055-s04.sh` and `scripts/tests/verify-m055-s04-contract.test.mjs` were retargeted so S04 now proves the real sibling `../hyperpush-mono` repo directly and treats the materialized `.tmp/m055-s04/workspace/hyperpush-mono` tree as a supporting check only.

### Verified so far
- `node --test scripts/tests/verify-m055-s04-contract.test.mjs` passed after the slug updates.
- `node --test scripts/tests/verify-m055-s01-contract.test.mjs` passed after fixing the stale negative-test expectations that still pinned `snowdamiz/mesh-lang`.
- `bash scripts/verify-m055-s01.sh` passed against the real sibling `../hyperpush-mono` product root after hydrating `../hyperpush-mono/mesher/landing` with `npm --prefix mesher/landing ci`.
- After retargeting S04 to the real sibling repo, `node --test scripts/tests/verify-m055-s04-contract.test.mjs` passed again under the new real-product-root contract.
- `bash scripts/verify-m055-s04.sh` now passes end-to-end against the real sibling `../hyperpush-mono` repo, with the staged materializer retained as a supporting check only.
- `cd ../hyperpush-mono && bash scripts/verify-m051-s01.sh` passed after the product-root maintainer proof surfaces were fixed.
- `cd ../hyperpush-mono && bash scripts/verify-landing-surface.sh` passed after the product-root README/dependabot/workflow surfaces were fixed.

### Still outstanding / blocked for actual code separation
- **`mesh-lang` still tracks `mesher/`.** Until that tree is removed, the code is not actually separated even though the product repo now exists and verifies cleanly.
- The current `mesh-lang` PR/branch is **prep only**. It documents and verifies the split, but it does not perform the destructive separation step.
- The core follow-up still needed in `mesh-lang` is:
  1. delete tracked `mesher/**`
  2. keep the split-boundary docs/verifiers/links pointing at `hyperpush-mono`
  3. rerun the M055 wrapper chain after that deletion
  4. publish that as the real separation branch/PR
- The language repo push path is still constrained by the host's HTTPS `git push` timeout (`HTTP 408`). That is why the current `mesh-lang` state on GitHub is a branch/PR rather than a direct `main` push.
- `../hyperpush-mono` still needs local dependency hydration for `mesher/landing` (`npm --prefix mesher/landing ci`) before local landing builds work. The product workflow installs deps itself, so this is a local workspace requirement, not a reason to keep product code in `mesh-lang`.

### Recommended next steps for the next agent
1. Prepare the **real** `mesh-lang` separation branch: remove tracked `mesher/**` from `mesh-lang` while preserving the already-landed split docs/verifiers/helpers.
2. Rerun at least:
   - `node --test scripts/tests/verify-m055-s01-contract.test.mjs`
   - `node --test scripts/tests/verify-m055-s04-contract.test.mjs`
   - `bash scripts/verify-m055-s01.sh`
   - `bash scripts/verify-m055-s04.sh`
3. Publish that branch/PR to `hyperpush-org/mesh-lang` as the actual code-separation change.
4. Only after that branch is reviewed/merged should `mesh-lang/main` be considered truly separated from product code.

## 0.5 Current gap between "split prep" and "actual code separation"

### Split prep that is already done
- `mesh-lang` knows the canonical repo pair (`hyperpush-org/mesh-lang` + `hyperpush-org/hyperpush-mono`).
- Docs/install/release/verifier surfaces in `mesh-lang` now point at the `hyperpush-org` repos.
- Split-boundary verifiers in `mesh-lang` can prove the real sibling `../hyperpush-mono` product repo.
- `hyperpush-mono/main` now contains product-root README/workflow/verifier/runbook/toolchain surfaces and proves them locally.

### What still prevents actual code separation
- `mesh-lang` still has tracked `mesher/**`.
- That means GitHub still shows the product code in **both** repos.
- As long as that duplicate tree remains in `mesh-lang`, the split is operationally prepared but **not** actually complete.

### Concrete definition of the missing final step
Actual code separation happens only when all of these are true at the same time:
1. `hyperpush-org/hyperpush-mono` contains the product code and its product-root proof surfaces.
2. `hyperpush-org/mesh-lang` keeps only language-owned surfaces plus split-boundary helpers/docs.
3. `hyperpush-org/mesh-lang` no longer tracks `mesher/**`.
4. The M055 split-boundary verifiers still pass after that deletion.

Until step 3 is done and verified, treat the current `mesh-lang` PR/branch as **split preparation**, not as the final extraction.


### GitHub truth
- `hyperpush-org/mesh-lang`
  - language/toolchain/compiler/runtime
  - docs site
  - packages site
  - registry
  - installers / release flows
  - starter/examples
  - split-boundary compatibility helpers
- `hyperpush-org/hyperpush-mono`
  - `mesher/`
  - `mesher/landing/`
  - product CI
  - landing deploy
  - product runbooks / maintainer verifiers

### Local truth
Preserve the current practical workspace shape with a local-only assembly layer:

```text
<workspace>/
  mesh-lang/
  hyperpush-mono/
    mesher/
```

Optional local compatibility path inside `mesh-lang/`:

```text
mesh-lang/mesher -> ../hyperpush-mono/mesher
```

Rules:
- GitHub authority is split.
- Local convenience may be monorepo-shaped.
- CI must run from real repo roots only.
- No duplicate authoritative copy of `mesher/` may remain in `mesh-lang` after cutover.

---

## 1. Decisions to lock before execution

These must be resolved before destructive changes:

1. **Language repo migration method**
   - Preferred: transfer / rename current repo to `hyperpush-org/mesh-lang`
   - Alternative: create new repo and push migrated history
2. **Product extraction method**
   - Preferred: `git filter-repo`
   - Alternative: `git subtree split`
3. **Local compatibility path mechanism**
   - Preferred: symlink
   - Alternative: bind mount
4. **Cross-repo CI scope**
   - Preferred: read-only metadata / handoff verification only
   - Avoid: coordinated CI that re-creates monorepo ownership

---

## 2. Phase order

Execute in this order:

1. Re-baseline canonical repo identity around `hyperpush-org`
2. Stand up real `hyperpush-org/hyperpush-mono`
3. Extract `mesher/` with history
4. Move canonical language repo to `hyperpush-org/mesh-lang`
5. Update canonical URLs and workflow assumptions
6. Remove tracked `mesher/` from `mesh-lang`
7. Add local workspace assembly helper
8. Run final verification in both repos

Do **not** delete tracked `mesher/` from `mesh-lang` before steps 2–5 are complete.

---

## 3. `mesh-lang` exact edit list

### 3.1 Repo identity and workspace contract

#### Files
- `scripts/lib/repo-identity.json`
- `WORKSPACE.md`
- `.gsd/PROJECT.md`

#### Required edits
- Change canonical language repo identity from `snowdamiz/mesh-lang` to `hyperpush-org/mesh-lang`
- Keep canonical product repo as `hyperpush-org/hyperpush-mono`
- State explicitly that:
  - `mesher/` is product-owned on GitHub
  - `mesh-lang/mesher` may exist locally only as a compatibility path
  - CI and public docs must not depend on that local compatibility path

#### Acceptance
- No canonical identity file still describes `snowdamiz/mesh-lang` as current truth
- Workspace contract distinguishes GitHub truth from local convenience

---

### 3.2 Split-boundary helpers and wrappers

#### Files
- `scripts/lib/m055-workspace.sh`
- `scripts/verify-m051-s01.sh`
- `scripts/verify-m055-s01.sh`
- `scripts/verify-m055-s03.sh`
- `scripts/verify-m055-s04.sh`

#### Required edits
- Replace canonical language slug with `hyperpush-org/mesh-lang`
- Keep canonical product slug `hyperpush-org/hyperpush-mono`
- Resolve sibling product root in this order:
  1. explicit env override
  2. blessed sibling `../hyperpush-mono`
- Fail closed if:
  - only an in-repo `mesher/` exists
  - sibling product repo is missing
  - repo identity metadata disagrees with actual cutover contract
- Stop treating staged materialization as sufficient proof once the real product repo exists

#### Acceptance
- Compatibility wrappers verify the real split
- They no longer certify the old in-repo `mesher/` arrangement as authoritative

---

### 3.3 Public docs and install/release links

#### Files
- `README.md`
- `website/docs/**`
- `website/docs/public/install.sh`
- `website/docs/public/install.ps1`
- any other docs pages with source blob links or product handoffs

#### Required edits
- Replace canonical language links with `https://github.com/hyperpush-org/mesh-lang`
- Replace product handoff links with `https://github.com/hyperpush-org/hyperpush-mono`
- Remove wording that implies:
  - `mesher/` is still GitHub-owned inside `mesh-lang`
  - product runbooks are repo-local to the language repo
- Keep local sibling-workspace notes only as maintainer/dev guidance, not as public canonical structure

#### Acceptance
- Docs/install surfaces point to the `hyperpush-org` repos only
- Product handoffs land in `hyperpush-mono`

---

### 3.4 GitHub policy/config cleanup

#### Files
- `.github/CODEOWNERS`
- `.github/dependabot.yml`
- `.github/ISSUE_TEMPLATE/config.yml`

#### Required edits
- Remove `/mesher/` from `CODEOWNERS`
- Remove `/mesher/landing` from Dependabot config
- Update support/security/issue links so language issues point at the canonical language repo and product links point at the product repo where appropriate
- Re-check stale `/reference-backend/` ownership while touching `CODEOWNERS`

#### Acceptance
- No GitHub policy file in `mesh-lang` still owns product code or product package updates

---

### 3.5 Historical verifiers and slug references

#### Known high-risk files
- `scripts/verify-m034-s03.sh`
- `scripts/verify-m034-s05.sh`
- `scripts/verify-m047-s06.sh`
- `scripts/tests/verify-m049-s04-onboarding-contract.test.mjs`
- `scripts/tests/verify-m053-s03-contract.test.mjs`
- any verifier/test pinning `snowdamiz/mesh-lang`

#### Required edits
- Replace canonical old slug with `hyperpush-org/mesh-lang`
- Preserve product handoff checks for `hyperpush-org/hyperpush-mono`
- Remove assumptions that `mesher/README.md` is locally authoritative inside the language repo

#### Acceptance
- Canonical slug checks are updated
- Historical rails do not regress to the old ownership model

---

### 3.6 Remove product source from `mesh-lang`

#### Remove
- `mesher/**`

#### Replace with
- handoff docs where needed
- split-boundary helpers
- local workspace assembly helper

#### Acceptance
- Clean clone of `hyperpush-org/mesh-lang` contains no tracked product source tree

---

## 4. `hyperpush-mono` initial file set

Create the new repo with at least the following:

### Product code
- `mesher/**`

### Product root surfaces
- `README.md`
- `.github/dependabot.yml`
- `.github/CODEOWNERS` if needed
- `.github/ISSUE_TEMPLATE/config.yml` if product-specific
- `SUPPORT.md` and `SECURITY.md` if product-owned

### Product workflows
- `.github/workflows/deploy-landing.yml`
- `.github/workflows/ci.yml` (or equivalent) for Mesher + landing verification
- optional dedicated product verifier workflow

### Product verifiers / scripts
- `scripts/verify-landing-surface.sh`
- `scripts/verify-m051-s01.sh` from product root if the compatibility entrypoint is still useful
- any root-level helpers needed by landing deploy/build/test

### Product docs / runbooks
- `mesher/README.md`
- any landing deploy/runbook documents

### Source of truth for initial content
Use:
- `scripts/materialize-hyperpush-mono.mjs`
- `scripts/fixtures/m055-s04-hyperpush-root/`

as templates only. Do **not** treat the staged `.tmp` output as the final source repository.

---

## 5. Workflow ownership map

## 5.1 Keep in `hyperpush-org/mesh-lang`

### `.github/workflows/authoritative-verification.yml`
- Owner: language repo
- Purpose: whitespace + language verification only
- Must not assume product source is local

### `.github/workflows/authoritative-live-proof.yml`
- Owner: language repo
- Purpose: language publish / public proof surfaces

### `.github/workflows/authoritative-starter-failover-proof.yml`
- Owner: language repo
- Purpose: language-owned starter/failover proof surfaces
- Must not rely on tracked `mesher/`

### `.github/workflows/deploy-services.yml`
- Owner: language repo
- Purpose:
  - deploy registry
  - deploy packages site
  - verify language public surface
- Must not deploy Hyperpush landing

### `.github/workflows/deploy.yml`
- Owner: language repo
- Purpose: language-owned deploy flow only

### `.github/workflows/release.yml`
- Owner: language repo
- Purpose: `meshc` / `meshpkg` release pipeline only

### `.github/workflows/publish-extension.yml`
- Owner: language repo
- Purpose: VS Code extension publishing

### `.github/workflows/extension-release-proof.yml`
- Owner: language repo
- Purpose: VS Code extension proof lane

---

## 5.2 Create or move into `hyperpush-org/hyperpush-mono`

### `.github/workflows/ci.yml` (new)
- Owner: product repo
- Purpose:
  - Mesher build/test/migrate/smoke
  - landing build/test
  - product-root verifiers

### `.github/workflows/deploy-landing.yml`
- Owner: product repo
- Purpose: deploy `mesher/landing`

### optional `.github/workflows/verify-product-surface.yml`
- Owner: product repo
- Purpose:
  - run `mesher/scripts/verify-maintainer-surface.sh`
  - run `scripts/verify-landing-surface.sh`

---

## 5.3 CI cutover rules

### `mesh-lang`
Remove all CI assumptions that:
- `mesher/` exists in the repo
- `/mesher/landing` exists in the repo
- product deploy happens here
- product dependencies are updated here

### `hyperpush-mono`
Add CI assumptions that:
- product repo root is authoritative
- `mesher/landing` is local and real
- Mesher package verification is first-class
- landing deploy is product-owned

### Cross-repo CI rule
Cross-repo checks may only:
- read metadata
- verify handoff links
- verify sibling repo resolution rules

They must **not** own sibling-repo build/test/deploy quality gates.

---

## 6. Git operations order

Use this exact high-level order.

### 6.1 Prepare extraction
- create a clean branch for split work
- freeze repo identity changes first
- ensure no in-flight work modifies `mesher/` during extraction

### 6.2 Create product repo with history
Preferred route:
- use `git filter-repo` or `git subtree split` to extract `mesher/` history
- create `hyperpush-org/hyperpush-mono`
- push extracted history there
- layer in product-root files and workflows from the M055 fixtures

### 6.3 Move canonical language repo
Preferred route:
- transfer or rename current language repo to `hyperpush-org/mesh-lang`

Alternative only if necessary:
- create new `hyperpush-org/mesh-lang`
- push full language history there
- update all canonical links immediately

### 6.4 Update canonical URLs and CI
- change `scripts/lib/repo-identity.json`
- update docs/install/release links
- cut workflow ownership over

### 6.5 Remove tracked product tree from language repo
- delete `mesher/**` from `mesh-lang`
- clean CODEOWNERS / Dependabot / product assumptions

### 6.6 Add local-only workspace assembly helper
- add `scripts/setup-local-workspace.sh`
- create local compatibility path on this machine

### 6.7 Run final verification in both repos
- language repo independent verification
- product repo independent verification
- local assembled workspace smoke

---

## 7. Local workspace preservation plan

### Recommended structure
```text
<workspace>/
  mesh-lang/
  hyperpush-mono/
```

### Local-only compatibility path
Create locally:
```text
mesh-lang/mesher -> ../hyperpush-mono/mesher
```

### Add helper script
Create:
- `scripts/setup-local-workspace.sh`

### Script requirements
- verify both sibling repos exist
- verify remotes are:
  - `hyperpush-org/mesh-lang`
  - `hyperpush-org/hyperpush-mono`
- create/repair the local compatibility path
- fail if `mesh-lang/mesher` is a tracked real directory instead of local assembly
- fail on stale sibling repo names or stale remotes

### Important constraints
- local-only
- untracked if necessary
- never required by CI
- never described as canonical GitHub structure

---

## 8. Acceptance checklist

## 8.1 `hyperpush-org/mesh-lang`
- [ ] no tracked `mesher/`
- [ ] `.github/CODEOWNERS` has no `/mesher/`
- [ ] `.github/dependabot.yml` has no `/mesher/landing`
- [ ] docs/install/release links point to `hyperpush-org/mesh-lang`
- [ ] product handoffs point to `hyperpush-org/hyperpush-mono`
- [ ] split-boundary verifiers pass under the new repo identity
- [ ] language CI is green without product-source ownership

## 8.2 `hyperpush-org/hyperpush-mono`
- [ ] `mesher/` present with preserved history
- [ ] product root README/runbooks present
- [ ] landing workflow present and green
- [ ] product CI green
- [ ] maintainer verifier green

## 8.3 Local machine
- [ ] `mesh-lang/mesher` still works at the same path locally
- [ ] local editor/tooling behavior preserved
- [ ] CI does not depend on the local compatibility path

## 8.4 Link truth
- [ ] no canonical references to `snowdamiz/mesh-lang`
- [ ] language source/download/docs links point to `hyperpush-org/mesh-lang`
- [ ] product runbook/handoff links point to `hyperpush-org/hyperpush-mono`

---

## 9. Safety rules / rollback points

### Safety rules
- Do **not** delete tracked `mesher/` from `mesh-lang` until:
  - `hyperpush-org/hyperpush-mono` exists
  - product CI exists
  - split-boundary verification is green under the new identity
- Do **not** cut installer/docs URLs until the canonical language repo is ready
- Do **not** treat local symlink setup as proof of success

### Rollback points
- before product repo extraction push
- before language repo transfer / rename
- before deleting tracked `mesher/` from `mesh-lang`
- before CI ownership cutover

---

## 10. Recommended minimal first execution batch

Use this as the first real implementation batch:

1. Update `scripts/lib/repo-identity.json`
2. Update `WORKSPACE.md`
3. Update split-boundary helpers to the new canonical slugs
4. Create real `hyperpush-org/hyperpush-mono`
5. Install product workflows there
6. Transfer / rename language repo to `hyperpush-org/mesh-lang`
7. Update docs/install/release URLs
8. Remove `/mesher/` from language `CODEOWNERS` and `/mesher/landing` from language Dependabot
9. Remove tracked `mesher/` from `mesh-lang`
10. Add local workspace setup helper
11. Run both repos' verification independently

---

## 11. Practical definition of done

This split is done when:
- GitHub shows `hyperpush-org/mesh-lang` and `hyperpush-org/hyperpush-mono`
- `hyperpush-org/mesh-lang` does **not** track product source
- `hyperpush-org/hyperpush-mono` owns `mesher/` and its CI
- this machine can still work with the same practical path shape
- all canonical docs/install/release links point at the `hyperpush-org` repos

---

## 12. Immediate execution tranche from the current state

The prep work is already good enough to perform the real destructive separation. The remaining work should be treated as one focused `mesh-lang` change set whose job is only to remove tracked product source while preserving the already-landed split boundary.

### 12.1 Preconditions already satisfied
- `hyperpush-org/hyperpush-mono` exists and has real product-root proof surfaces.
- Local sibling product repo `../hyperpush-mono` is real and tracks the correct remote.
- `mesh-lang` already knows the canonical split slugs.
- `scripts/setup-local-workspace.sh` exists, so local path preservation no longer depends on keeping `mesher/` tracked in `mesh-lang`.
- `bash scripts/verify-m055-s01.sh` and `bash scripts/verify-m055-s04.sh` already prove the sibling-repo contract against the real product repo.

### 12.2 Remaining destructive step
In `mesh-lang`, do exactly this next:
1. create a dedicated separation branch off the current split-prep state
2. delete tracked `mesher/**`
3. keep every split-boundary helper, doc handoff, repo-identity surface, and verifier that now points at `hyperpush-mono`
4. rerun the M055 boundary rails after the deletion
5. publish that branch as the **actual** separation PR

### 12.3 Scope guard for that branch
That branch should **not** try to rework unrelated language tooling, landing UI, or broader docs copy. Its purpose is narrow:
- remove duplicated product source from `mesh-lang`
- prove that the language repo still verifies cleanly without it
- preserve local workspace compatibility through the helper/symlink path only

---

## 13. Exact `mesh-lang` cutover sequence

### 13.1 Branch shape
Recommended branch intent:
- current published prep branch remains the plumbing/reference branch
- new branch is the destructive removal branch

Suggested naming:
- `m055-repo-split-remove-mesher`

### 13.2 File-system action
Primary destructive change:
- remove tracked `mesher/**` from `mesh-lang`

Keep in `mesh-lang`:
- `scripts/lib/repo-identity.json`
- `scripts/lib/m055-workspace.sh`
- `scripts/setup-local-workspace.sh`
- all M055 split-boundary verifiers/tests
- public docs/install/release surfaces that now hand off to `hyperpush-mono`
- any retained fixture or verifier content that proves the handoff contract without reintroducing product ownership

Do not keep in `mesh-lang`:
- tracked product source
- tracked landing app source
- language-repo CI that still assumes product code is present locally

### 13.3 Local compatibility repair after deletion
Immediately after removing tracked `mesher/**`, restore the local path shape with:
- `bash scripts/setup-local-workspace.sh`

Expected result:
- GitHub/source control truth: no tracked `mesher/` in `mesh-lang`
- local machine convenience: `mesh-lang/mesher -> ../hyperpush-mono/mesher`

### 13.4 Required verification after deletion
Run these in `mesh-lang` after the delete + compatibility repair:
- `node --test scripts/tests/verify-m055-s01-contract.test.mjs`
- `node --test scripts/tests/verify-m055-s04-contract.test.mjs`
- `bash scripts/verify-m055-s01.sh`
- `bash scripts/verify-m055-s04.sh`

Then confirm the sibling product repo still stands on its own:
- `cd ../hyperpush-mono && bash scripts/verify-m051-s01.sh`
- `cd ../hyperpush-mono && bash scripts/verify-landing-surface.sh`

### 13.5 Minimum success condition for the destructive branch
That branch is only valid if all of the following are true together:
- `git ls-files mesher` returns nothing meaningful in `mesh-lang`
- the local compatibility path exists only as local assembly, not tracked product source
- M055 split-boundary rails still pass
- sibling product-root verifiers still pass

---

## 14. Publish plan for the final separation change

### 14.1 Preferred publication flow
Because direct `main` pushes are still vulnerable to the host's HTTPS `git push` timeout, publish the destructive separation as a normal branch + PR first.

Recommended flow:
1. push `m055-repo-split-remove-mesher`
2. open a PR against `hyperpush-org/mesh-lang`
3. describe it explicitly as the branch that performs the real source separation
4. keep the earlier prep PR as reference only, or supersede it if that is cleaner

### 14.2 What the PR description should state explicitly
- `hyperpush-mono` is already the product authority
- this PR is the step that removes duplicated product source from `mesh-lang`
- local `mesh-lang/mesher` can still exist after merge, but only through the workspace helper/symlink
- the attached verification is boundary-only for `mesh-lang` and product-root-only for `hyperpush-mono`

### 14.3 What not to claim yet
Do not describe the split as complete merely because:
- the product repo exists
- docs link to the right slugs
- the prep PR is open

The split is only complete after the destructive delete branch lands and `mesh-lang` no longer tracks `mesher/**`.

---

## 15. Post-merge local machine state

After the final separation branch merges, the expected steady state on this machine is:

```text
<workspace>/
  mesh-lang/
  hyperpush-mono/
```

With the local compatibility path restored as needed:

```text
mesh-lang/mesher -> ../hyperpush-mono/mesher
```

### Post-merge local checklist
1. fetch latest `mesh-lang/main`
2. run `bash scripts/setup-local-workspace.sh`
3. if landing deps are absent locally, run `cd ../hyperpush-mono && npm --prefix mesher/landing ci`
4. rerun:
   - `bash scripts/verify-m055-s01.sh`
   - `bash scripts/verify-m055-s04.sh`
5. confirm local editors/tools still work against the compatibility path without restoring tracked product source

### Final steady-state rule
- Product authority lives in `hyperpush-mono`
- Language authority lives in `mesh-lang`
- Local path convenience is allowed
- Duplicate GitHub authority is not
