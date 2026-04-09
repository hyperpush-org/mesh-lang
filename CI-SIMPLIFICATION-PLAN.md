# CI Simplification Plan

Last updated: 2026-04-09

## Intent

Reduce CI to the smallest set of checks that protect real product and language contracts.

This plan assumes two principles:

1. **No workaround-driven CI design.**
   - Do not keep workflows alive with opt-in flags, seeded artifact folders, or verifier-of-verifier layers unless the underlying contract genuinely needs them.
   - If a workflow is not truly required on normal pushes/PRs, remove it from that trigger surface.

2. **Keep checks that prove shipped behavior, delete checks that only prove the CI plumbing.**
   - Build/test/package/release correctness is valuable.
   - Tests that mainly assert YAML text, step names, artifact pointer files, or wrapper delegation chains are usually noise.

---

## Current state snapshot

### `mesh-lang`

Current CI/workflow surface:

- 8 workflow files under `.github/workflows/`
- ~51 CI/verifier-related files in the current `m034` / `m053` / `m055` / workflow-contract / whitespace surface
- 19 `scripts/tests/*contract.test.mjs` files

Current top-level workflow split:

- `authoritative-verification.yml`
- `authoritative-live-proof.yml`
- `authoritative-starter-failover-proof.yml`
- `deploy.yml`
- `deploy-services.yml`
- `release.yml`
- `extension-release-proof.yml`
- `publish-extension.yml`

### `hyperpush`

Current CI/workflow surface:

- 2 workflow files under `.github/workflows/`
- small but still verifier-heavy shell surface

Current workflows:

- `ci.yml`
- `deploy-landing.yml`

---

## What looks bloated

### `mesh-lang`

The current bloat is not mostly in the number of workflows. It is in the **meta-contract layer** around them.

Symptoms:

- workflow-contract scripts testing YAML structure in detail
- contract tests that test those workflow-contract scripts
- hosted-evidence scripts that inspect GitHub Actions runs as if they were product behavior
- wrapper scripts that replay older wrapper scripts and then validate retained artifact bundle shapes
- failure handling focused on exact artifact existence and step naming rather than on the underlying compiler/package/release truth

This creates several bad failure modes:

- CI fails because the workflow shape changed, not because the shipped product broke
- CI fails because diagnostics upload paths are absent, masking the real failure
- release/deploy checks run on normal `main` pushes even when they depend on external configuration, secrets, or hosted services
- fixing one workflow often requires editing multiple second-order verifier scripts and contract tests

### `hyperpush`

The product repo is much smaller, but it still inherits some unnecessary complexity:

- the root wrapper/verifier chain is heavier than needed for day-to-day product CI
- the Mesher maintainer job currently proves more than a normal PR needs
- landing verification includes contract text checks that may be better kept as ordinary docs/source checks inside a simpler CI job

---

## Target end state

## `mesh-lang`

Keep only three CI tiers.

### Tier 1 — PR/main blocking checks

These should be fast, deterministic, secret-free, and tied directly to code correctness.

Keep:

- formatting / whitespace / basic hygiene
- core Rust build + tests
- targeted package/install smoke that does **not** publish to external services
- targeted starter/failover smoke only if it is stable and proves a real runtime contract

Remove from PR/main:

- live publish to external registry
- hosted-evidence replay against GitHub Actions history
- deploy workflows that require GitHub Pages or Fly to be configured
- YAML contract tests that only verify workflow structure

### Tier 2 — release/tag checks

Keep only checks needed to ship binaries/extensions/packages.

Keep:

- release artifact build
- installer smoke
- extension packaging/publish proof

Move here if still needed:

- expensive starter failover proof
- one real release-path smoke

### Tier 3 — manual or scheduled operational probes

Only keep these if they answer an operational question that normal CI cannot.

Candidates:

- live registry publish/install proof against production infra
- hosted deploy/public-surface probes
- external availability checks

If kept, they should be:

- `workflow_dispatch` or scheduled only
- clearly labeled non-blocking operational probes
- not part of normal PR/main gating

---

## `hyperpush`

Keep only two blocking jobs.

### 1. `mesher-smoke`

Purpose:

- prove the product can use the sibling `mesh-lang` toolchain
- prove Mesher compiles/tests against that toolchain
- prove one honest local runtime smoke

Keep:

- checkout product repo
- checkout sibling `mesh-lang`
- build `meshc` and `mesh-rt`
- run package tests
- optionally run one local Postgres-backed smoke if it is stable and fast

Remove:

- retained artifact bundle validation for routine CI
- meta-proof layers about wrapper output shape unless they catch real product breakage

### 2. `web-build`

Purpose:

- prove `mesher/landing` builds
- prove `mesher/frontend-exp` builds

Keep:

- dependency install
- build commands
- minimal source/contract checks only when they prevent genuine split-boundary drift

Remove:

- text-heavy verifier layers that duplicate what the build or a simple source grep already proves

### Deploy workflow

`deploy-landing.yml` should exist only if it is a real deploy workflow.

If it only builds, fold that behavior into `ci.yml` and delete the separate deploy workflow.
If it truly deploys, keep it manual or push-to-main only, but do not duplicate CI-only checks there.

---

## Files and surfaces to review first for deletion or downgrade

## `mesh-lang`

High-probability delete/downgrade candidates:

- `scripts/tests/verify-m034-s05-contract.test.mjs`
- `scripts/tests/verify-m053-s03-contract.test.mjs`
- `scripts/tests/verify-m055-s03-contract.test.mjs`
- similar `*contract.test.mjs` files that only enforce workflow/verifier structure
- `scripts/verify-m034-s02-workflows.sh`
- `scripts/verify-m034-s04-workflows.sh`
- `scripts/verify-m034-s05-workflows.sh`
- `scripts/verify-m053-s03.sh`
- wrapper-style closeout scripts whose main purpose is replaying other wrappers and checking retained artifact pointers

High-probability keep candidates:

- `scripts/verify-whitespace.sh` if kept simple
- direct build/test/smoke scripts that prove compiler/runtime/package behavior
- split-boundary checks that catch real repo contamination after the split

## `hyperpush`

High-probability simplify candidates:

- `scripts/verify-m051-s01.sh`
- `mesher/scripts/verify-maintainer-surface.sh`
- `scripts/verify-landing-surface.sh`

Not necessarily delete, but reduce them to:

- one real Mesher smoke contract
- one real landing/source boundary contract
- no artifact-shape theater

---

## Proposed execution phases

## Phase 1 — Inventory and classify every check

For every workflow job and every verifier script, classify it as one of:

- **keep-blocking**
- **keep-release**
- **keep-manual**
- **delete**

Decision rule:

- If removing it would allow a real broken release/product/runtime contract through, keep it.
- If removing it would only stop CI from arguing about its own plumbing, delete it.

Deliverable:

- one table listing each workflow/job/script and its classification

## Phase 2 — Collapse `mesh-lang` mainline CI

Target:

- one primary PR/main workflow
- no live external publish on PR/main
- no hosted-evidence checks on PR/main
- no deploy workflows pretending to be CI gates

Deliverable:

- simplified `.github/workflows/authoritative-verification.yml` or replacement `ci.yml`
- expensive/operational jobs moved to `workflow_dispatch` or tag-only workflows

## Phase 3 — Delete meta-contract tests

Target:

- remove tests that verify workflow YAML details or verifier-script internals unless they protect a real ship contract

Deliverable:

- delete the workflow-contract test layer
- keep only tests that exercise shipped tool behavior

## Phase 4 — Simplify `hyperpush` CI

Target:

- keep `ci.yml` to two jobs max
- either delete `deploy-landing.yml` or make it a true deploy-only workflow
- shrink product verifiers to real build/smoke checks

Deliverable:

- smaller product CI
- smaller verifier scripts

## Phase 5 — Re-baseline and document

Target:

- every remaining workflow has a short reason to exist
- every blocking job is fast enough and stable enough to deserve blocking status

Deliverable:

- updated README or maintainer docs with the final CI map
- no second-order contract tests needed to understand the system

---

## Concrete simplification rules

Apply these consistently:

1. **No workflow should exist only to validate another workflow.**
2. **No script should exist only to validate another script’s retained artifacts.**
3. **Normal CI must not require production secrets or hosted services unless that repo truly ships through them on every merge.**
4. **External live proof belongs in manual/scheduled ops, not standard PR gating.**
5. **If a check can be replaced by one direct build/test/smoke command, prefer the direct command.**
6. **If a failure message is about step names, artifact paths, or pointer files instead of product behavior, that layer is a deletion candidate.**

---

## Recommended first cuts

If this plan is approved, the first implementation pass should do only these:

### `mesh-lang`

1. Remove PR/main dependency on live publish proof.
2. Remove PR/main dependency on hosted GitHub-run evidence.
3. Move deploy/public-surface live checks to manual or scheduled workflows.
4. Delete workflow-contract tests that only assert YAML shape.

### `hyperpush`

1. Keep sibling toolchain build + Mesher smoke.
2. Keep landing/frontend builds.
3. Reduce verifier scripts to the smallest direct commands that prove those surfaces.

This should cut a large amount of CI noise without touching the real compiler/runtime/product contracts.

---

## Success criteria

The simplification is done when:

- PR failures are mostly about broken code, not broken CI scaffolding
- release failures are mostly about shipping artifacts, not workflow metadata drift
- routine pushes do not depend on external production systems unless intentionally required
- the workflow graph is understandable without reading multiple verifier-of-verifier layers
- both repos have a CI surface that matches their actual ownership after the split

---

## Non-goals for the first pass

- redesigning the entire release process
- deleting every shell verifier immediately
- changing product/runtime behavior to suit CI
- keeping compatibility for every historical retained-proof rail

The first pass should prioritize **deleting meta-CI** and **preserving real checks**.
