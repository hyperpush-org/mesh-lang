# M034: Delivery Truth & Public Release Confidence

**Gathered:** 2026-03-26
**Status:** Ready for planning

## Project Description

This milestone hardens CI/CD and ensures the important truth surfaces are actually included in it. It tests the package manager end to end, turns the public release path into something proven instead of assumed, and makes the current release/deploy/package workflows honest enough that Mesh can claim public readiness on evidence rather than artifact presence.

## Why This Milestone

Mesh already has a believable backend core, but the repo still has a delivery-truth gap. The current workflows build and package a lot, yet the important trust claims are still too easy to infer from green artifact jobs instead of from named proof surfaces. The user explicitly wants to harden CI/CD, ensure everything important is included in it, and test the package manager end to end. That makes this the right next milestone: prove the real public path before spending energy on polish.

## User-Visible Outcome

### When this milestone is complete, the user can:

- cut a Mesh release and know the important package, installer, deploy, and extension surfaces were actually re-checked instead of merely packaged
- publish and consume a real Mesh package through the registry path with end-to-end proof instead of trusting isolated pieces

### Entry point / environment

- Entry point: GitHub Actions workflows, `meshpkg`, release assets, install scripts, registry API, packages website, VS Code extension publish lane
- Environment: CI plus production-like public services
- Live dependencies involved: GitHub Actions, GitHub Releases, Fly.io, package registry, packages website, docs site, extension publication path

## Completion Class

- Contract complete means: named verifiers and workflow checks cover the package-manager, release-asset, installer, deploy-health, and extension-release contracts
- Integration complete means: the real `meshpkg` ↔ registry flow, release assets, install path, docs deploy, packages website, and extension lane work together as one release story
- Operational complete means: tag/release promotion, deploy health, and install/use-after-install smoke are proven on the real path

## Final Integrated Acceptance

To call this milestone complete, we must prove:

- a release-scoped package can be published, resolved, downloaded, installed, and pinned through the real Mesh registry path
- released `meshc` and `meshpkg` assets can be installed and used through the documented install path
- one release candidate can be replayed across binaries, installer, docs deploy, registry/packages-site health, and extension release checks instead of treating those as unrelated green lights

## Risks and Unknowns

- The public release path crosses GitHub releases, Fly deploys, the registry, the packages website, install scripts, and the extension publish lane — subsystem-green can still hide assembled-path failure
- The package-manager path may work in pieces but still fail on the real publish → metadata/download → install → lockfile loop — this is exactly the kind of false confidence the milestone must retire
- Current workflows are build-heavy and may produce artifacts without rerunning the proof surfaces that matter most — that makes release green even when trust should be red
- The extension release lane can publish a packageable extension while syntax/tooling truth is still drifting — M034 must harden the release path without pretending M036’s full editor-parity work is already done

## Existing Codebase / Prior Art

- `.github/workflows/release.yml` — current release workflow builds `meshc` and `meshpkg`, but it is still artifact-heavy relative to the proof bar the user wants
- `.github/workflows/deploy-services.yml` — current Fly deploy path for `registry/` and `packages-website/`, with thin health checks already in place
- `.github/workflows/publish-extension.yml` — separate extension publish lane that packages and publishes, but still needs a harder prepublish truth surface
- `compiler/meshpkg/src/publish.rs` — current package publish path and SHA-256 handling for registry uploads
- `compiler/meshpkg/src/install.rs` — current install path, download/checksum flow, and lockfile writes
- `registry/src/routes/publish.rs` — real registry publish contract, including auth, scope checks, SHA verification, duplicate handling, and blob upload
- `registry/src/routes/download.rs` — real streaming download contract used by `meshpkg install`
- `tools/editors/vscode-mesh/package.json` — extension release metadata and package surface
- `tools/editors/vscode-mesh/syntaxes/mesh.tmLanguage.json` — current shipped grammar, including the known interpolation drift that showed editor-truth claims are ahead of reality

> See `.gsd/DECISIONS.md` for all architectural and pattern decisions — it is an append-only register; read it during planning, append to it during execution.

## Relevant Requirements

- R007 — turns the old package/dependency trust placeholder into a real publish/install/download/lockfile proof path
- R021 — advances the broader package trust and ecosystem maturity sequence, while leaving UI polish itself for M037
- R045 — makes CI/CD and release flows prove the shipped Mesh surfaces instead of only building artifacts
- R046 — directly covers the user’s request to test the package manager end to end on the real path
- R047 — partially advances editor trust by hardening the extension release lane, without claiming full syntax/editor parity yet

## Scope

### In Scope

- harden CI/CD and ensure the important proof surfaces are actually included in it
- test the package manager end to end on the real registry contract
- verify release assets and install scripts against the documented public install path
- harden the VS Code extension release lane so publication is gated by real validation
- add one final assembled public-release proof that ties the separate subsystems together

### Out of Scope / Non-Goals

- full VS Code syntax completeness and the broader editor-support wave — that is M036
- the deeper review/hardening of Mesh’s test implementation as a daily-driver framework for `mesher` — that is M035
- package-manager UI/website polish beyond what is needed to keep the public release path honest — that is M037
- broad new language or runtime features unrelated to delivery, package, release, editor, or testing trust

## Technical Constraints

- The milestone must use the real workflows and live services where the trust claim depends on them; contract-only proof is not enough for the final public-release bar.
- External publication steps are real state changes, so the release path has to be designed around safe, repeatable, diagnosable proof rather than ad hoc manual publishes.
- The current repo already splits release, deploy, and extension publication across separate workflows; M034 has to reconcile those without blurring their responsibilities.
- The current VS Code grammar drift (`#{}` vs `${}`) is a useful signal that docs and packaging success are not enough on their own.

## Integration Points

- GitHub Actions — primary orchestration surface for release, deploy, and extension publication
- GitHub Releases — source of truth for public `meshc` and `meshpkg` assets and checksums
- Fly.io — deploy target for `registry/` and `packages-website/`
- `registry/` — live package publish, metadata, and download API surface
- `packages-website/` — public package browsing and publish entrypoint surface
- `website/` — public docs and install-path contract
- `tools/editors/vscode-mesh/` — extension packaging and publication surface

## Open Questions

- How should the real package publish proof avoid leaving permanent garbage in the public registry while still staying honest? — Current thinking: use a release-scoped verification package naming contract and make cleanup/immutability constraints explicit in the proof design.
- How much of the full public release path can be gated before irreversible publication happens? — Current thinking: validate as much as possible before publish, then keep the final public-ready proof assembled and replayable so failures localize cleanly.
- Where should the top-level public-release verifier live? — Current thinking: use one canonical script or workflow entrypoint that composes the slice-owned verifiers instead of scattering the contract across docs and job names.
