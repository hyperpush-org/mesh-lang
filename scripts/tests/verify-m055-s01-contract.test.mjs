import test from 'node:test'
import assert from 'node:assert/strict'
import fs from 'node:fs'
import os from 'node:os'
import path from 'node:path'
import { spawnSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'

const scriptDir = path.dirname(fileURLToPath(import.meta.url))
const root = path.resolve(scriptDir, '..', '..')

const files = {
  workspace: 'WORKSPACE.md',
  readme: 'README.md',
  contributing: 'CONTRIBUTING.md',
  project: '.gsd/PROJECT.md',
  repoIdentity: 'scripts/lib/repo-identity.json',
  helper: 'scripts/lib/m034_public_surface_contract.py',
  gettingStarted: 'website/docs/docs/getting-started/index.md',
  tooling: 'website/docs/docs/tooling/index.md',
  installSourceSh: 'tools/install/install.sh',
  installSourcePs1: 'tools/install/install.ps1',
  installPublicSh: 'website/docs/public/install.sh',
  installPublicPs1: 'website/docs/public/install.ps1',
  packagesLayout: 'packages-website/src/routes/+layout.svelte',
  landingLinks: 'mesher/landing/lib/external-links.ts',
  packageJson: 'tools/editors/vscode-mesh/package.json',
  knowledge: '.gsd/KNOWLEDGE.md',
  verifier: 'scripts/verify-m055-s01.sh',
}

const expectedRepoIdentityVersion = 'm055-s01-repo-identity-v1'
const expectedLanguageRepo = {
  workspaceDir: 'mesh-lang',
  slug: 'hyperpush-org/mesh-lang',
  repoUrl: 'https://github.com/hyperpush-org/mesh-lang',
  gitUrl: 'https://github.com/hyperpush-org/mesh-lang.git',
  issuesUrl: 'https://github.com/hyperpush-org/mesh-lang/issues',
  blobBaseUrl: 'https://github.com/hyperpush-org/mesh-lang/blob/main/',
  installerRoot: 'https://meshlang.dev',
  docsRoot: 'https://meshlang.dev/docs/',
}
const expectedProductRepo = {
  workspaceDir: 'hyperpush-mono',
  slug: 'hyperpush-org/hyperpush-mono',
  repoUrl: 'https://github.com/hyperpush-org/hyperpush-mono',
  gitUrl: 'https://github.com/hyperpush-org/hyperpush-mono.git',
  issuesUrl: 'https://github.com/hyperpush-org/hyperpush-mono/issues',
  blobBaseUrl: 'https://github.com/hyperpush-org/hyperpush-mono/blob/main/',
  installerRoot: null,
  docsRoot: null,
}
const languageInstallShUrl = `${expectedLanguageRepo.installerRoot}/install.sh`
const languageInstallPs1Url = `${expectedLanguageRepo.installerRoot}/install.ps1`
const languageGettingStartedUrl = `${expectedLanguageRepo.docsRoot}getting-started/`
const languageToolingUrl = `${expectedLanguageRepo.docsRoot}tooling/`
const languageWorkspaceUrl = `${expectedLanguageRepo.blobBaseUrl}WORKSPACE.md`
const packagesLanguageRepoLabel = 'mesh-lang repo'
const packagesWorkspaceLabel = 'Workspace'
const landingProductRepoDisplay = `github.com/${expectedProductRepo.slug}`
const vscodeRepositoryDirectory = 'tools/editors/vscode-mesh'
const helperIdentityPathMarker = 'scripts/lib/repo-identity.json'
const helperForbiddenIdentityCopies = [
  expectedLanguageRepo.slug,
  expectedProductRepo.slug,
  expectedLanguageRepo.repoUrl,
  expectedProductRepo.repoUrl,
  expectedLanguageRepo.gitUrl,
  expectedProductRepo.gitUrl,
  expectedLanguageRepo.issuesUrl,
  expectedProductRepo.issuesUrl,
  expectedLanguageRepo.blobBaseUrl,
  expectedProductRepo.blobBaseUrl,
  languageInstallShUrl,
  languageInstallPs1Url,
  languageGettingStartedUrl,
  languageToolingUrl,
]

const workspaceTitle = '# Workspace Contract'
const workspaceIntro = 'This document is the maintainer-facing workspace contract for M055.'
const workspaceTwoRepoMarker = 'M055 is a two-repo split only: `mesh-lang` stays the language repo, and `hyperpush-mono` becomes the product repo that absorbs `mesher/`.'
const workspaceLayoutHeading = '## Blessed sibling workspace'
const workspaceLayoutMeshLang = '  mesh-lang/'
const workspaceLayoutHyperpush = '  hyperpush-mono/'
const workspaceNotSiblingMarker = 'Only these two sibling repos are part of the blessed M055 workspace. `mesh-packages/` and `mesh-website/` are not sibling repos in this milestone.'
const workspaceOwnershipHeading = '## Repo ownership'
const workspaceLanguageOwnedMarker = '`website/`, `packages-website/`, `registry/`, installers, and evaluator-facing examples remain language-owned inside `mesh-lang` for M055.'
const workspaceProductMarker = '`mesher/` is the product surface that moves to `hyperpush-mono`; extraction happens in later M055 slices.'
const workspaceGsdHeading = '## Repo-local GSD authority'
const workspaceGsdMarker = 'Repo-local `.gsd/` stays authoritative for repo-owned work.'
const workspaceNoUmbrellaMarker = 'Do not replace repo-local `.gsd/` with one umbrella milestone tree that pretends to own both repos.'
const workspaceCoordHeading = '## Coordination layer boundary'
const workspaceCoordMarker = 'Cross-repo work goes through a lightweight sibling-workspace coordination layer.'
const workspaceCoordBoundaryMarker = 'The coordination layer points at repo-local proofs; it does not replace repo-local plans, `.tmp/` bundles, or verifier entrypoints.'
const readmeWorkspaceHeading = '## Workspace contract for maintainers'
const readmeWorkspaceMarker = 'M055 is a two-repo split only: the blessed sibling workspace is `mesh-lang/` plus `hyperpush-mono/`.'
const readmeLanguageOwnedMarker = '`website/`, `packages-website/`, `registry/`, installers, and evaluator-facing starters/examples stay language-owned in `mesh-lang` for this milestone.'
const readmeGsdMarker = 'Repo-local `.gsd` remains authoritative, and cross-repo work uses the lightweight coordination layer in [WORKSPACE.md](WORKSPACE.md).'
const contributingWorkspaceHeading = '## M055 workspace contract'
const contributingTwoRepoMarker = 'M055 is a two-repo split only: `mesh-lang` plus `hyperpush-mono`.'
const contributingHandoffMarker = '`hyperpush-mono` is the product repo that will absorb `mesher/`.'
const contributingLanguageOwnedMarker = 'For this milestone, `website/`, `packages-website/`, `registry/`, installers, and evaluator-facing examples remain language-owned inside `mesh-lang`.'
const contributingGsdMarker = 'Repo-local `.gsd` stays authoritative; cross-repo work should use the lightweight coordination layer instead of one umbrella milestone tree. See [WORKSPACE.md](WORKSPACE.md) for the durable split contract.'
const projectM055Marker = 'M055 is now the active split-contract milestone.'
const projectTwoRepoMarker = 'The durable target is a two-repo sibling workspace: `mesh-lang` keeps the language/toolchain/docs/installers/registry/packages/public-site surfaces, and `hyperpush-mono` becomes the product repo that absorbs `mesher/`.'
const projectTransitionalMarker = 'The source still lives in one checkout today, but that layout is transitional rather than the durable ownership model.'
const projectWorkspaceMarker = '`WORKSPACE.md` is the maintainer-facing contract for the blessed sibling layout, and repo-local `.gsd` remains authoritative instead of yielding to one umbrella workspace tree.'
const staleFourRepoLayout = '  mesh-packages/'
const staleFourRepoLayoutTwo = '  mesh-website/'
const staleFourRepoSentence = 'The blessed sibling workspace is `mesh-lang/`, `mesh-packages/`, `mesh-website/`, and `hyperpush-mono/`.'
const staleUmbrellaSentence = 'One umbrella `.gsd/` tree owns both repos.'
const staleOwnershipSentence = '`website/`, `packages-website/`, `registry/`, and installers move into separate sibling repos during M055.'
const workspaceVerifierHeading = '## Authoritative split-boundary verifier'
const workspaceVerifierCommandMarker = 'Run `bash scripts/verify-m055-s01.sh` before changing split-boundary ownership text, repo identity, or the repo-local `.gsd` handoff.'
const workspaceVerifierDebugMarker = 'If it fails, start with `.tmp/m055-s01/verify/phase-report.txt` and then read the failing per-phase log in `.tmp/m055-s01/verify/`.'
const contributingVerifierCommandMarker = 'Run `bash scripts/verify-m055-s01.sh` before changing workspace ownership text, repo identity, or the repo-local `.gsd` seam. It writes `.tmp/m055-s01/verify/status.txt`, `current-phase.txt`, `phase-report.txt`, and `full-contract.log`; start with `phase-report.txt` when the wrapper goes red.'
const knowledgeVerifierMarker = 'For M055/S01 split-boundary regressions, start with `bash scripts/verify-m055-s01.sh`, then read `.tmp/m055-s01/verify/phase-report.txt` and the failing per-phase log. If the repo-local `.gsd` seam is red, rerun `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_package_contract_remains_source_first_and_route_free -- --nocapture` directly before changing `scripts/fixtures/clustered/tiny-cluster/`, `.gsd/milestones/M046/slices/S03/S03-PLAN.md`, or `scripts/verify-m046-s03.sh`.'
const m055VerifierCommand = 'bash scripts/verify-m055-s01.sh'
const m055NodeContractCommand = 'node --test scripts/tests/verify-m055-s01-contract.test.mjs'
const m055LocalDocsCommand = 'python3 scripts/lib/m034_public_surface_contract.py local-docs --root .'
const m055PackagesBuildCommand = 'npm --prefix packages-website run build'
const m055LandingBuildCommand = "bash -c 'cd \"$1\" && npm --prefix mesher/landing run build' _ \"$HYPERPUSH_ROOT\""
const m055GsdRegressionCommand = 'cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_package_contract_remains_source_first_and_route_free -- --nocapture'
const m055VerifierPhases = {
  'm055-s01-contract': m055NodeContractCommand,
  'm055-s01-local-docs': m055LocalDocsCommand,
  'm055-s01-packages-build': m055PackagesBuildCommand,
  'm055-s01-landing-build': m055LandingBuildCommand,
  'm055-s01-gsd-regression': m055GsdRegressionCommand,
}

function readFrom(baseRoot, relativePath) {
  const absolutePath = path.join(baseRoot, relativePath)
  assert.ok(fs.existsSync(absolutePath), `missing ${relativePath}`)
  return fs.readFileSync(absolutePath, 'utf8')
}

function writeTo(baseRoot, relativePath, content) {
  const absolutePath = path.join(baseRoot, relativePath)
  fs.mkdirSync(path.dirname(absolutePath), { recursive: true })
  fs.writeFileSync(absolutePath, content)
}

function writeJson(baseRoot, relativePath, value) {
  writeTo(baseRoot, relativePath, `${JSON.stringify(value, null, 2)}\n`)
}

function copyRepoFile(baseRoot, relativePath) {
  writeTo(baseRoot, relativePath, readFrom(root, relativePath))
}

function copyAllFiles(baseRoot) {
  for (const relativePath of Object.values(files)) {
    copyRepoFile(baseRoot, relativePath)
  }
}

function mkTmpDir(t, prefix) {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), prefix))
  t.after(() => fs.rmSync(dir, { recursive: true, force: true }))
  return dir
}

function runHelper(baseRoot, args) {
  return spawnSync('python3', [path.join(baseRoot, files.helper), ...args], {
    cwd: baseRoot,
    encoding: 'utf8',
  })
}

function requireIncludes(errors, relativePath, text, needles) {
  for (const needle of needles) {
    if (!text.includes(needle)) {
      errors.push(`${relativePath} missing ${JSON.stringify(needle)}`)
    }
  }
}

function requireExcludes(errors, relativePath, text, needles) {
  for (const needle of needles) {
    if (text.includes(needle)) {
      errors.push(`${relativePath} still contains stale text ${JSON.stringify(needle)}`)
    }
  }
}

function requireOrdered(errors, relativePath, text, markers) {
  let previousIndex = -1
  for (const marker of markers) {
    const index = text.indexOf(marker)
    if (index === -1) {
      errors.push(`${relativePath} missing ordered marker ${JSON.stringify(marker)}`)
      return
    }
    if (index <= previousIndex) {
      errors.push(`${relativePath} drifted order around ${JSON.stringify(marker)}`)
      return
    }
    previousIndex = index
  }
}

function validateWorkspaceContract(baseRoot) {
  const errors = []
  const workspace = readFrom(baseRoot, files.workspace)
  const readme = readFrom(baseRoot, files.readme)
  const contributing = readFrom(baseRoot, files.contributing)
  const project = readFrom(baseRoot, files.project)

  requireIncludes(errors, files.workspace, workspace, [
    workspaceTitle,
    workspaceIntro,
    workspaceTwoRepoMarker,
    workspaceLayoutHeading,
    workspaceLayoutMeshLang,
    workspaceLayoutHyperpush,
    workspaceNotSiblingMarker,
    workspaceOwnershipHeading,
    workspaceLanguageOwnedMarker,
    workspaceProductMarker,
    workspaceGsdHeading,
    workspaceGsdMarker,
    workspaceNoUmbrellaMarker,
    workspaceCoordHeading,
    workspaceCoordMarker,
    workspaceCoordBoundaryMarker,
  ])
  requireExcludes(errors, files.workspace, workspace, [
    staleFourRepoLayout,
    staleFourRepoLayoutTwo,
    staleFourRepoSentence,
  ])
  requireOrdered(errors, files.workspace, workspace, [
    workspaceTitle,
    workspaceTwoRepoMarker,
    workspaceLayoutHeading,
    workspaceNotSiblingMarker,
    workspaceOwnershipHeading,
    workspaceLanguageOwnedMarker,
    workspaceGsdHeading,
    workspaceGsdMarker,
    workspaceNoUmbrellaMarker,
    workspaceCoordHeading,
    workspaceCoordMarker,
    workspaceCoordBoundaryMarker,
  ])

  requireIncludes(errors, files.readme, readme, [
    readmeWorkspaceHeading,
    readmeWorkspaceMarker,
    readmeLanguageOwnedMarker,
    readmeGsdMarker,
  ])
  requireOrdered(errors, files.readme, readme, [
    '## Where to go next',
    readmeWorkspaceHeading,
    readmeWorkspaceMarker,
    readmeGsdMarker,
    '## Maintainers / public release proof',
  ])

  requireIncludes(errors, files.contributing, contributing, [
    contributingWorkspaceHeading,
    contributingTwoRepoMarker,
    contributingHandoffMarker,
    contributingLanguageOwnedMarker,
    contributingGsdMarker,
  ])
  requireOrdered(errors, files.contributing, contributing, [
    '## Development setup',
    contributingWorkspaceHeading,
    contributingTwoRepoMarker,
    contributingGsdMarker,
    '## Common commands',
  ])

  requireIncludes(errors, files.project, project, [
    projectM055Marker,
    projectTwoRepoMarker,
    projectTransitionalMarker,
    projectWorkspaceMarker,
  ])
  requireOrdered(errors, files.project, project, [
    '## What This Is',
    projectM055Marker,
    projectTwoRepoMarker,
    projectWorkspaceMarker,
    'M048 is complete.',
  ])

  return errors
}

function validateRepoSection(errors, relativePath, payload, sectionName, expected) {
  if (!payload || typeof payload !== 'object' || Array.isArray(payload)) {
    errors.push(`${relativePath} ${sectionName} must be an object`)
    return
  }

  for (const [key, expectedValue] of Object.entries(expected)) {
    if (!(key in payload)) {
      errors.push(`${relativePath} missing ${sectionName}.${key}`)
      continue
    }
    if (payload[key] !== expectedValue) {
      errors.push(
        `${relativePath} ${sectionName}.${key} expected ${JSON.stringify(expectedValue)} but found ${JSON.stringify(payload[key])}`,
      )
    }
  }
}

function validateHelperDescribe(errors, baseRoot) {
  const result = runHelper(baseRoot, ['describe'])
  if (result.status !== 0) {
    const output = result.stderr.trim() || result.stdout.trim() || 'unknown helper failure'
    errors.push(`scripts/lib/m034_public_surface_contract.py describe failed: ${output}`)
    return
  }

  let payload
  try {
    payload = JSON.parse(result.stdout)
  } catch (error) {
    errors.push(`scripts/lib/m034_public_surface_contract.py describe returned invalid JSON: ${error.message}`)
    return
  }

  if (payload.helperPath !== files.helper) {
    errors.push(`helperPath expected ${JSON.stringify(files.helper)} but found ${JSON.stringify(payload.helperPath)}`)
  }
  if (payload.repoIdentityPath !== files.repoIdentity) {
    errors.push(`repoIdentityPath expected ${JSON.stringify(files.repoIdentity)} but found ${JSON.stringify(payload.repoIdentityPath)}`)
  }
  if (payload.repoIdentity?.version !== expectedRepoIdentityVersion) {
    errors.push(
      `repoIdentity.version expected ${JSON.stringify(expectedRepoIdentityVersion)} but found ${JSON.stringify(payload.repoIdentity?.version)}`,
    )
  }
  if (JSON.stringify(payload.repoIdentity?.languageRepo) !== JSON.stringify(expectedLanguageRepo)) {
    errors.push('repoIdentity.languageRepo drifted from the canonical language repo contract')
  }
  if (JSON.stringify(payload.repoIdentity?.productRepo) !== JSON.stringify(expectedProductRepo)) {
    errors.push('repoIdentity.productRepo drifted from the canonical product repo contract')
  }
}

function validateSplitAwarePublicIdentitySurfaces(errors, baseRoot) {
  const packagesLayout = readFrom(baseRoot, files.packagesLayout)
  requireIncludes(errors, files.packagesLayout, packagesLayout, [
    expectedLanguageRepo.repoUrl,
    languageWorkspaceUrl,
    packagesLanguageRepoLabel,
    packagesWorkspaceLabel,
  ])
  requireExcludes(errors, files.packagesLayout, packagesLayout, [
    expectedProductRepo.repoUrl,
    expectedProductRepo.slug,
    expectedProductRepo.gitUrl,
    expectedProductRepo.issuesUrl,
  ])

  const landingLinks = readFrom(baseRoot, files.landingLinks)
  requireIncludes(errors, files.landingLinks, landingLinks, [
    expectedProductRepo.slug,
    expectedProductRepo.repoUrl,
    landingProductRepoDisplay,
  ])
  requireExcludes(errors, files.landingLinks, landingLinks, [
    expectedLanguageRepo.repoUrl,
    expectedLanguageRepo.slug,
    expectedLanguageRepo.gitUrl,
    expectedLanguageRepo.issuesUrl,
  ])

  const packageJson = JSON.parse(readFrom(baseRoot, files.packageJson))
  if (packageJson.repository?.url !== expectedLanguageRepo.gitUrl) {
    errors.push(
      `${files.packageJson} repository.url expected ${JSON.stringify(expectedLanguageRepo.gitUrl)} but found ${JSON.stringify(packageJson.repository?.url)}`,
    )
  }
  if (packageJson.repository?.directory !== vscodeRepositoryDirectory) {
    errors.push(
      `${files.packageJson} repository.directory expected ${JSON.stringify(vscodeRepositoryDirectory)} but found ${JSON.stringify(packageJson.repository?.directory)}`,
    )
  }
  if (packageJson.bugs?.url !== expectedLanguageRepo.issuesUrl) {
    errors.push(
      `${files.packageJson} bugs.url expected ${JSON.stringify(expectedLanguageRepo.issuesUrl)} but found ${JSON.stringify(packageJson.bugs?.url)}`,
    )
  }
  if (packageJson.homepage !== expectedLanguageRepo.installerRoot) {
    errors.push(
      `${files.packageJson} homepage expected ${JSON.stringify(expectedLanguageRepo.installerRoot)} but found ${JSON.stringify(packageJson.homepage)}`,
    )
  }

  const packageJsonText = JSON.stringify(packageJson)
  for (const forbidden of [
    expectedProductRepo.slug,
    expectedProductRepo.repoUrl,
    expectedProductRepo.gitUrl,
    expectedProductRepo.issuesUrl,
  ]) {
    if (packageJsonText.includes(forbidden)) {
      errors.push(`${files.packageJson} still contains stale text ${JSON.stringify(forbidden)}`)
    }
  }
}

function validateRepoIdentityContract(baseRoot) {
  const errors = []
  let repoIdentity
  try {
    repoIdentity = JSON.parse(readFrom(baseRoot, files.repoIdentity))
  } catch (error) {
    errors.push(`${files.repoIdentity} is not valid JSON: ${error.message}`)
    return errors
  }

  if (repoIdentity.version !== expectedRepoIdentityVersion) {
    errors.push(
      `${files.repoIdentity} version expected ${JSON.stringify(expectedRepoIdentityVersion)} but found ${JSON.stringify(repoIdentity.version)}`,
    )
  }
  validateRepoSection(errors, files.repoIdentity, repoIdentity.languageRepo, 'languageRepo', expectedLanguageRepo)
  validateRepoSection(errors, files.repoIdentity, repoIdentity.productRepo, 'productRepo', expectedProductRepo)

  const helperSource = readFrom(baseRoot, files.helper)
  requireIncludes(errors, files.helper, helperSource, [helperIdentityPathMarker])
  requireExcludes(errors, files.helper, helperSource, helperForbiddenIdentityCopies)
  validateHelperDescribe(errors, baseRoot)

  const installSourceSh = readFrom(baseRoot, files.installSourceSh)
  const installPublicSh = readFrom(baseRoot, files.installPublicSh)
  if (installSourceSh !== installPublicSh) {
    errors.push(`${files.installSourceSh} drifted from ${files.installPublicSh}`)
  }
  requireIncludes(errors, files.installSourceSh, installSourceSh, [`REPO="${expectedLanguageRepo.slug}"`])

  const installSourcePs1 = readFrom(baseRoot, files.installSourcePs1)
  const installPublicPs1 = readFrom(baseRoot, files.installPublicPs1)
  if (installSourcePs1 !== installPublicPs1) {
    errors.push(`${files.installSourcePs1} drifted from ${files.installPublicPs1}`)
  }
  requireIncludes(errors, files.installSourcePs1, installSourcePs1, [`$Repo = "${expectedLanguageRepo.slug}"`])

  validateSplitAwarePublicIdentitySurfaces(errors, baseRoot)

  return errors
}

function validateVerifierContract(baseRoot) {
  const errors = []
  const verifier = readFrom(baseRoot, files.verifier)
  const workspace = readFrom(baseRoot, files.workspace)
  const contributing = readFrom(baseRoot, files.contributing)
  const knowledge = readFrom(baseRoot, files.knowledge)

  requireIncludes(errors, files.verifier, verifier, [
    'PHASE_REPORT_PATH="$ARTIFACT_DIR/phase-report.txt"',
    'STATUS_PATH="$ARTIFACT_DIR/status.txt"',
    'CURRENT_PHASE_PATH="$ARTIFACT_DIR/current-phase.txt"',
    'exec > >(tee "$ARTIFACT_DIR/full-contract.log") 2>&1',
    'assert_test_filter_ran() {',
    'record_phase "$phase" timed_out',
    'phase report missing passed marker',
    m055NodeContractCommand,
    m055LocalDocsCommand,
    m055PackagesBuildCommand,
    m055LandingBuildCommand,
    m055GsdRegressionCommand,
    'verify-m055-s01: ok',
  ])

  requireExcludes(errors, files.verifier, verifier, [
    'bash scripts/verify-m046-s03.sh',
    'cargo test -p meshc --test e2e_m046_s03 -- --nocapture',
    'cargo run -q -p meshc -- build scripts/fixtures/clustered/tiny-cluster',
    'cargo run -q -p meshc -- test scripts/fixtures/clustered/tiny-cluster/tests',
    './mesher/landing/node_modules/.bin/tsc --noEmit -p mesher/landing/tsconfig.json',
    'npm --prefix website run build',
  ])

  const verifierLines = verifier.split('\n')
  const actualRunExpectSuccess = {}
  for (let lineIndex = 0; lineIndex < verifierLines.length; lineIndex += 1) {
    const line = verifierLines[lineIndex]
    const stripped = line.trim()
    if (!stripped.startsWith('run_expect_success ')) {
      continue
    }
    const parts = stripped.split(/\s+/)
    if (parts.length < 6) {
      errors.push(`${files.verifier} has malformed run_expect_success line ${JSON.stringify(line)}`)
      continue
    }
    const phase = parts[1]
    if (!line.trimEnd().endsWith('\\')) {
      errors.push(`${files.verifier} replay line no longer uses the expected continuation form for ${JSON.stringify(phase)}`)
      continue
    }
    const commandLine = verifierLines[lineIndex + 1]?.trim()
    if (!commandLine) {
      errors.push(`${files.verifier} is missing the command continuation for ${JSON.stringify(phase)}`)
      continue
    }
    actualRunExpectSuccess[phase] = commandLine
  }

  const expectedPhases = Object.keys(m055VerifierPhases).sort()
  const actualPhases = Object.keys(actualRunExpectSuccess).sort()
  if (JSON.stringify(actualPhases) !== JSON.stringify(expectedPhases)) {
    errors.push(
      `${files.verifier} replay phases drifted: expected ${JSON.stringify(expectedPhases)}, got ${JSON.stringify(actualPhases)}`,
    )
  }
  for (const [phase, expectedCommand] of Object.entries(m055VerifierPhases)) {
    if (actualRunExpectSuccess[phase] !== expectedCommand) {
      errors.push(
        `${files.verifier} replay command drifted for ${phase}: expected ${JSON.stringify(expectedCommand)} but found ${JSON.stringify(actualRunExpectSuccess[phase])}`,
      )
    }
  }

  requireIncludes(errors, files.workspace, workspace, [
    workspaceVerifierHeading,
    workspaceVerifierCommandMarker,
    workspaceVerifierDebugMarker,
  ])
  requireOrdered(errors, files.workspace, workspace, [
    workspaceCoordHeading,
    workspaceCoordBoundaryMarker,
    workspaceVerifierHeading,
    workspaceVerifierCommandMarker,
    workspaceVerifierDebugMarker,
    '## Working rule',
  ])

  requireIncludes(errors, files.contributing, contributing, [contributingVerifierCommandMarker])
  requireOrdered(errors, files.contributing, contributing, [
    contributingGsdMarker,
    contributingVerifierCommandMarker,
    '## Common commands',
  ])

  requireIncludes(errors, files.knowledge, knowledge, [knowledgeVerifierMarker])

  return errors
}

test('current repo publishes the M055 S01 two-repo workspace and repo-local GSD contract', () => {
  const errors = validateWorkspaceContract(root)
  assert.deepEqual(errors, [], errors.join('\n'))
})

test('current repo publishes the M055 S01 assembled verifier and debug entrypoint contract', () => {
  const errors = validateVerifierContract(root)
  assert.deepEqual(errors, [], errors.join('\n'))
})

test('assembled verifier contract fails closed when the repo-local .gsd cargo rail or phase markers drift', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m055-s01-wrapper-')
  copyRepoFile(tmpRoot, files.workspace)
  copyRepoFile(tmpRoot, files.contributing)
  copyRepoFile(tmpRoot, files.knowledge)
  copyRepoFile(tmpRoot, files.verifier)

  let mutatedVerifier = readFrom(tmpRoot, files.verifier)
  mutatedVerifier = mutatedVerifier.replace(m055GsdRegressionCommand, 'bash scripts/verify-m046-s03.sh')
  mutatedVerifier = mutatedVerifier.replace('phase-report.txt', 'phase.log')
  writeTo(tmpRoot, files.verifier, mutatedVerifier)

  const errors = validateVerifierContract(tmpRoot)
  const joinedErrors = errors.join('\n')
  assert.match(joinedErrors, /scripts\/verify-m055-s01\.sh missing "PHASE_REPORT_PATH=\\\"\$ARTIFACT_DIR\/phase-report\.txt\\\""/)
  assert.match(joinedErrors, /scripts\/verify-m055-s01\.sh still contains stale text "bash scripts\/verify-m046-s03\.sh"/)
  assert.match(joinedErrors, /scripts\/verify-m055-s01\.sh replay command drifted for m055-s01-gsd-regression/)
})

test('verifier discoverability contract fails closed when WORKSPACE, CONTRIBUTING, or KNOWLEDGE stop pointing at the M055 rail', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m055-s01-docs-')
  copyRepoFile(tmpRoot, files.workspace)
  copyRepoFile(tmpRoot, files.contributing)
  copyRepoFile(tmpRoot, files.knowledge)
  copyRepoFile(tmpRoot, files.verifier)

  let mutatedWorkspace = readFrom(tmpRoot, files.workspace)
  mutatedWorkspace = mutatedWorkspace.replace(m055VerifierCommand, 'bash scripts/verify-m054-s03.sh')
  writeTo(tmpRoot, files.workspace, mutatedWorkspace)

  let mutatedContributing = readFrom(tmpRoot, files.contributing)
  mutatedContributing = mutatedContributing.replace(m055VerifierCommand, 'bash scripts/verify-m054-s03.sh')
  writeTo(tmpRoot, files.contributing, mutatedContributing)

  let mutatedKnowledge = readFrom(tmpRoot, files.knowledge)
  mutatedKnowledge = mutatedKnowledge.replace(m055VerifierCommand, 'bash scripts/verify-m054-s03.sh')
  mutatedKnowledge = mutatedKnowledge.replace(m055GsdRegressionCommand, 'cargo test -p meshc --test e2e_m046_s03 -- --nocapture')
  writeTo(tmpRoot, files.knowledge, mutatedKnowledge)

  const errors = validateVerifierContract(tmpRoot)
  const joinedErrors = errors.join('\n')
  assert.match(joinedErrors, /WORKSPACE\.md missing "Run `bash scripts\/verify-m055-s01\.sh` before changing split-boundary ownership text, repo identity, or the repo-local `.gsd` handoff\."/)
  assert.match(joinedErrors, /CONTRIBUTING\.md missing "Run `bash scripts\/verify-m055-s01\.sh` before changing workspace ownership text, repo identity, or the repo-local `.gsd` seam\. It writes `.tmp\/m055-s01\/verify\/status\.txt`, `current-phase\.txt`, `phase-report\.txt`, and `full-contract\.log`; start with `phase-report\.txt` when the wrapper goes red\."/)
  assert.match(joinedErrors, /\.gsd\/KNOWLEDGE\.md missing "For M055\/S01 split-boundary regressions, start with `bash scripts\/verify-m055-s01\.sh`/)
})


test('current repo publishes one canonical repo identity contract plus split-aware public package, landing, and editor surfaces', () => {
  const errors = validateRepoIdentityContract(root)
  assert.deepEqual(errors, [], errors.join('\n'))
})

test('contract fails closed when WORKSPACE drifts back to a four-repo layout or umbrella GSD authority', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m055-s01-workspace-')
  copyAllFiles(tmpRoot)

  let mutatedWorkspace = readFrom(tmpRoot, files.workspace)
  mutatedWorkspace = mutatedWorkspace.replace(workspaceTwoRepoMarker, staleFourRepoSentence)
  mutatedWorkspace = mutatedWorkspace.replace(
    `${workspaceLayoutMeshLang}\n${workspaceLayoutHyperpush}`,
    `${workspaceLayoutMeshLang}\n${staleFourRepoLayout}\n${staleFourRepoLayoutTwo}\n${workspaceLayoutHyperpush}`,
  )
  mutatedWorkspace = mutatedWorkspace.replace(workspaceNotSiblingMarker, 'All four repos are sibling workspaces in this milestone.')
  mutatedWorkspace = mutatedWorkspace.replace(workspaceGsdMarker, staleUmbrellaSentence)
  mutatedWorkspace = mutatedWorkspace.replace(workspaceNoUmbrellaMarker, 'The umbrella workspace owns every repo-local plan and verifier.')
  mutatedWorkspace = mutatedWorkspace.replace(workspaceProductMarker, '`mesher/` stays in `mesh-lang` as a permanent monorepo path.')
  writeTo(tmpRoot, files.workspace, mutatedWorkspace)

  const errors = validateWorkspaceContract(tmpRoot)
  const joinedErrors = errors.join('\n')
  assert.match(joinedErrors, /WORKSPACE\.md missing "M055 is a two-repo split only: `mesh-lang` stays the language repo, and `hyperpush-mono` becomes the product repo that absorbs `mesher\/`\."/)
  assert.match(joinedErrors, /WORKSPACE\.md still contains stale text "  mesh-packages\/"/)
  assert.match(joinedErrors, /WORKSPACE\.md still contains stale text "  mesh-website\/"/)
  assert.match(joinedErrors, /WORKSPACE\.md missing "Repo-local `.gsd\/` stays authoritative for repo-owned work\."/)
  assert.match(joinedErrors, /WORKSPACE\.md missing "Do not replace repo-local `.gsd\/` with one umbrella milestone tree that pretends to own both repos\."/)
  assert.match(joinedErrors, /WORKSPACE\.md missing "`mesher\/` is the product surface that moves to `hyperpush-mono`; extraction happens in later M055 slices\."/)
})

test('contract fails closed when README or CONTRIBUTING mention the split but stop linking maintainers to WORKSPACE', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m055-s01-entrypoints-')
  copyAllFiles(tmpRoot)

  let mutatedReadme = readFrom(tmpRoot, files.readme)
  mutatedReadme = mutatedReadme.replace(readmeGsdMarker, 'Repo-local `.gsd` remains authoritative, and cross-repo work uses the lightweight coordination layer described in the workspace notes.')
  writeTo(tmpRoot, files.readme, mutatedReadme)

  let mutatedContributing = readFrom(tmpRoot, files.contributing)
  mutatedContributing = mutatedContributing.replace(contributingGsdMarker, 'Repo-local `.gsd` stays authoritative; cross-repo work should use the lightweight coordination layer instead of one umbrella milestone tree.')
  writeTo(tmpRoot, files.contributing, mutatedContributing)

  const errors = validateWorkspaceContract(tmpRoot)
  const joinedErrors = errors.join('\n')
  assert.match(joinedErrors, /README\.md missing "Repo-local `.gsd` remains authoritative, and cross-repo work uses the lightweight coordination layer in \[WORKSPACE\.md\]\(WORKSPACE\.md\)\."/)
  assert.match(joinedErrors, /CONTRIBUTING\.md missing "Repo-local `.gsd` stays authoritative; cross-repo work should use the lightweight coordination layer instead of one umbrella milestone tree\. See \[WORKSPACE\.md\]\(WORKSPACE\.md\) for the durable split contract\."/)
})

test('contract fails closed when language-owned boundary text disappears from WORKSPACE or PROJECT', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m055-s01-boundary-')
  copyAllFiles(tmpRoot)

  let mutatedWorkspace = readFrom(tmpRoot, files.workspace)
  mutatedWorkspace = mutatedWorkspace.replace(workspaceLanguageOwnedMarker, staleOwnershipSentence)
  writeTo(tmpRoot, files.workspace, mutatedWorkspace)

  let mutatedProject = readFrom(tmpRoot, files.project)
  mutatedProject = mutatedProject.replace(projectTwoRepoMarker, 'The durable target is a multi-repo breakup where docs, packages, registry, installers, and the product all move independently.')
  mutatedProject = mutatedProject.replace(projectWorkspaceMarker, 'A workspace-level umbrella plan will replace repo-local ownership once extraction starts.')
  writeTo(tmpRoot, files.project, mutatedProject)

  const errors = validateWorkspaceContract(tmpRoot)
  const joinedErrors = errors.join('\n')
  assert.match(joinedErrors, /WORKSPACE\.md missing "`website\/`, `packages-website\/`, `registry\/`, installers, and evaluator-facing examples remain language-owned inside `mesh-lang` for M055\."/)
  assert.match(joinedErrors, /\.gsd\/PROJECT\.md missing "The durable target is a two-repo sibling workspace: `mesh-lang` keeps the language\/toolchain\/docs\/installers\/registry\/packages\/public-site surfaces, and `hyperpush-mono` becomes the product repo that absorbs `mesher\/`\."/)
  assert.match(joinedErrors, /\.gsd\/PROJECT\.md missing "`WORKSPACE\.md` is the maintainer-facing contract for the blessed sibling layout, and repo-local `.gsd` remains authoritative instead of yielding to one umbrella workspace tree\."/)
})

test('repo identity contract fails closed when scripts/lib/repo-identity.json is malformed JSON', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m055-s01-repo-identity-json-')
  copyAllFiles(tmpRoot)
  writeTo(tmpRoot, files.repoIdentity, '{"version":')

  const errors = validateRepoIdentityContract(tmpRoot)
  const joinedErrors = errors.join('\n')
  assert.match(joinedErrors, /scripts\/lib\/repo-identity\.json is not valid JSON:/)
})

test('repo identity contract fails closed when mesh-lang or hyperpush-mono identity fields disappear', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m055-s01-repo-identity-fields-')
  copyAllFiles(tmpRoot)

  const repoIdentity = JSON.parse(readFrom(tmpRoot, files.repoIdentity))
  delete repoIdentity.productRepo.workspaceDir
  delete repoIdentity.productRepo.issuesUrl
  delete repoIdentity.languageRepo.docsRoot
  writeJson(tmpRoot, files.repoIdentity, repoIdentity)

  const errors = validateRepoIdentityContract(tmpRoot)
  const joinedErrors = errors.join('\n')
  assert.match(joinedErrors, /scripts\/lib\/repo-identity\.json missing productRepo\.workspaceDir/)
  assert.match(joinedErrors, /scripts\/lib\/repo-identity\.json missing productRepo\.issuesUrl/)
  assert.match(joinedErrors, /scripts\/lib\/repo-identity\.json missing languageRepo\.docsRoot/)
})

test('repo identity contract fails closed when installer source and docs-served copies diverge', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m055-s01-installer-parity-')
  copyAllFiles(tmpRoot)

  const mutatedInstallSh = `${readFrom(tmpRoot, files.installPublicSh)}\n# stale public copy\n`
  writeTo(tmpRoot, files.installPublicSh, mutatedInstallSh)

  const mutatedInstallPs1 = `${readFrom(tmpRoot, files.installPublicPs1)}\n# stale public copy\n`
  writeTo(tmpRoot, files.installPublicPs1, mutatedInstallPs1)

  const errors = validateRepoIdentityContract(tmpRoot)
  const joinedErrors = errors.join('\n')
  assert.match(joinedErrors, /tools\/install\/install\.sh drifted from website\/docs\/public\/install\.sh/)
  assert.match(joinedErrors, /tools\/install\/install\.ps1 drifted from website\/docs\/public\/install\.ps1/)
})

test('repo identity contract fails closed when the packages footer drifts toward product-owned repo markers', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m055-s01-packages-footer-')
  copyAllFiles(tmpRoot)

  let mutatedLayout = readFrom(tmpRoot, files.packagesLayout)
  mutatedLayout = mutatedLayout.replace(expectedLanguageRepo.repoUrl, expectedProductRepo.repoUrl)
  mutatedLayout = mutatedLayout.replace(languageWorkspaceUrl, `${expectedProductRepo.blobBaseUrl}WORKSPACE.md`)
  mutatedLayout = mutatedLayout.replace(packagesLanguageRepoLabel, 'GitHub')
  writeTo(tmpRoot, files.packagesLayout, mutatedLayout)

  const errors = validateRepoIdentityContract(tmpRoot)
  const joinedErrors = errors.join('\n')
  assert.match(joinedErrors, /packages-website\/src\/routes\/\+layout\.svelte missing "https:\/\/github\.com\/hyperpush-org\/mesh-lang"/)
  assert.match(joinedErrors, /packages-website\/src\/routes\/\+layout\.svelte missing "https:\/\/github\.com\/hyperpush-org\/mesh-lang\/blob\/main\/WORKSPACE\.md"/)
  assert.match(joinedErrors, /packages-website\/src\/routes\/\+layout\.svelte missing "mesh-lang repo"/)
  assert.match(joinedErrors, /packages-website\/src\/routes\/\+layout\.svelte still contains stale text "https:\/\/github\.com\/hyperpush-org\/hyperpush-mono"/)
})

test('repo identity contract fails closed when landing external links drift back to mesh-lang', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m055-s01-landing-links-')
  copyAllFiles(tmpRoot)

  let mutatedLinks = readFrom(tmpRoot, files.landingLinks)
  mutatedLinks = mutatedLinks.replaceAll(expectedProductRepo.slug, expectedLanguageRepo.slug)
  writeTo(tmpRoot, files.landingLinks, mutatedLinks)

  const errors = validateRepoIdentityContract(tmpRoot)
  const joinedErrors = errors.join('\n')
  assert.match(joinedErrors, /mesher\/landing\/lib\/external-links\.ts missing "hyperpush-org\/hyperpush-mono"/)
  assert.match(joinedErrors, /mesher\/landing\/lib\/external-links\.ts missing "https:\/\/github\.com\/hyperpush-org\/hyperpush-mono"/)
  assert.match(joinedErrors, /mesher\/landing\/lib\/external-links\.ts still contains stale text "hyperpush-org\/mesh-lang"/)
})

test('repo identity contract fails closed when VS Code metadata mixes product repo identity into the language extension', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m055-s01-vscode-metadata-')
  copyAllFiles(tmpRoot)

  const packageJson = JSON.parse(readFrom(tmpRoot, files.packageJson))
  packageJson.repository.url = expectedProductRepo.gitUrl
  packageJson.bugs.url = expectedProductRepo.issuesUrl
  writeJson(tmpRoot, files.packageJson, packageJson)

  const errors = validateRepoIdentityContract(tmpRoot)
  const joinedErrors = errors.join('\n')
  assert.match(joinedErrors, /tools\/editors\/vscode-mesh\/package\.json repository\.url expected "https:\/\/github\.com\/hyperpush-org\/mesh-lang\.git" but found "https:\/\/github\.com\/hyperpush-org\/hyperpush-mono\.git"/)
  assert.match(joinedErrors, /tools\/editors\/vscode-mesh\/package\.json bugs\.url expected "https:\/\/github\.com\/hyperpush-org\/mesh-lang\/issues" but found "https:\/\/github\.com\/hyperpush-org\/hyperpush-mono\/issues"/)
  assert.match(joinedErrors, /tools\/editors\/vscode-mesh\/package\.json still contains stale text "hyperpush-org\/hyperpush-mono"/)
})

test('local-docs helper fails closed when repo identity disagrees with installer-owned consumers', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m055-s01-helper-identity-')
  copyAllFiles(tmpRoot)

  const repoIdentity = JSON.parse(readFrom(tmpRoot, files.repoIdentity))
  repoIdentity.languageRepo.slug = 'hyperpush-org/mesh-lang-next'
  repoIdentity.languageRepo.repoUrl = 'https://github.com/hyperpush-org/mesh-lang-next'
  repoIdentity.languageRepo.gitUrl = 'https://github.com/hyperpush-org/mesh-lang-next.git'
  repoIdentity.languageRepo.issuesUrl = 'https://github.com/hyperpush-org/mesh-lang-next/issues'
  repoIdentity.languageRepo.blobBaseUrl = 'https://github.com/hyperpush-org/mesh-lang-next/blob/main/'
  writeJson(tmpRoot, files.repoIdentity, repoIdentity)

  const result = runHelper(tmpRoot, ['local-docs', '--root', tmpRoot])
  assert.notEqual(result.status, 0, 'local-docs should fail when repo identity no longer matches installer/docs consumers')
  assert.match(result.stderr, /website\/docs\/public\/install\.sh missing 'REPO="hyperpush-org\/mesh-lang-next"'/)
  assert.match(result.stderr, /tools\/editors\/vscode-mesh\/package\.json repository\.url drifted away from https:\/\/github\.com\/hyperpush-org\/mesh-lang-next\.git/)
  assert.match(result.stderr, /tools\/editors\/vscode-mesh\/package\.json bugs\.url drifted away from https:\/\/github\.com\/hyperpush-org\/mesh-lang-next\/issues/)
})
