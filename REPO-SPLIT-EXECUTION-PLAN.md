# Repo Split Execution Plan

Status: execution in progress (GitHub cutover still blocked)
Owner target: `hyperpush-org`
Language repo target: `hyperpush-org/mesh-lang`
Product repo target: `hyperpush-org/hyperpush-mono`
Local requirement: preserve the current monorepo-like path shape on this machine without keeping product code authoritative in the language repo.

---

## Handoff status — 2026-04-08

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

### Still outstanding / blocked
- The real sibling `../hyperpush-mono` repo needed local product-root fixes to go green. Those are local commits right now:
  - `a38cf754f9ccc80ef3341c0cf7779b422df50dfe` — Add product-root landing and maintainer proof surfaces
  - `0f841a12dca6dfb8662e5fd2140dc0f86b8cf31f` — Use sibling mesh-lang target dir for Mesher toolchain
  - `541ba58e213f665692b97ed76fe00b3e15eb711e` — Fix extracted Mesher maintainer proof surfaces
  - `5bc43b31bf29abcc56bd180d6db14fd669f03e95` — Restore product root split surfaces
  They still need whatever outbound/push workflow the user wants before GitHub truth matches the now-green local split proof.
- Tracked `mesher/` has **not** been removed from `mesh-lang` yet.
- The sibling product repo still needs a local dependency hydrate before `mesher/landing` will build (`npm --prefix mesher/landing ci` in `../hyperpush-mono`). That is workspace state, not yet a guaranteed remote CI truth unless the product workflow installs deps itself (the local verifier is green on that contract now).

### Recommended next steps for the next agent
1. Decide whether to push the local `../hyperpush-mono` fix commits upstream; without that, GitHub truth still lags the now-green local S04 proof.
2. Rerun the broader M055 wrapper chain from the language repo if you want one more assembled proof after the product commits are pushed.
3. Once the product repo truth is in place, remove tracked `mesher/` from `mesh-lang` and rerun the split-boundary verifiers.
4. Keep using `.tmp/m055-s04/verify/phase-report.txt` and `.tmp/m055-s04/verify/retained-proof-bundle/` as the authoritative resume point if S04 regresses.

## 0. Target end state

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
