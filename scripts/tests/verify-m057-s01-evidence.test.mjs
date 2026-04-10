import test from 'node:test'
import assert from 'node:assert/strict'
import fs from 'node:fs'
import os from 'node:os'
import path from 'node:path'
import { spawnSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'

const scriptDir = path.dirname(fileURLToPath(import.meta.url))
const root = path.resolve(scriptDir, '..', '..')
const pythonResult = spawnSync('python3', ['-c', 'import sys; print(sys.executable)'], { encoding: 'utf8' })
assert.equal(pythonResult.status, 0, pythonResult.stderr)
const PYTHON = pythonResult.stdout.trim() || 'python3'

const files = {
  evidenceScript: 'scripts/lib/m057_evidence_index.py',
  inventoryScript: 'scripts/lib/m057_tracker_inventory.py',
  repoIdentity: 'scripts/lib/repo-identity.json',
  workspaceGit: 'scripts/workspace-git.sh',
  project: '.gsd/PROJECT.md',
  decisions: '.gsd/DECISIONS.md',
  m053: '.gsd/milestones/M053/M053-SUMMARY.md',
  m054: '.gsd/milestones/M054/M054-SUMMARY.md',
  m055: '.gsd/milestones/M055/M055-SUMMARY.md',
  m056: '.gsd/milestones/M056/M056-SUMMARY.md',
  meshSnapshot: '.gsd/milestones/M057/slices/S01/mesh-lang-issues.snapshot.json',
  hyperpushSnapshot: '.gsd/milestones/M057/slices/S01/hyperpush-issues.snapshot.json',
  projectFieldsSnapshot: '.gsd/milestones/M057/slices/S01/project-fields.snapshot.json',
  projectItemsSnapshot: '.gsd/milestones/M057/slices/S01/project-items.snapshot.json',
  evidenceJson: '.gsd/milestones/M057/slices/S01/reconciliation-evidence.json',
  evidenceMd: '.gsd/milestones/M057/slices/S01/reconciliation-evidence.md',
  namingMap: '.gsd/milestones/M057/slices/S01/naming-ownership-map.json',
  docsConfig: 'website/docs/.vitepress/config.mts',
  docsNav: 'website/docs/.vitepress/theme/components/NavBar.vue',
  productMockData: 'mesher/frontend-exp/lib/mock-data.ts',
  productPitchPage: 'mesher/landing/app/pitch/page.tsx',
}

function mkTmpDir(t, prefix) {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), prefix))
  t.after(() => fs.rmSync(dir, { recursive: true, force: true }))
  return dir
}

function writeTo(baseRoot, relativePath, content) {
  const absolutePath = path.join(baseRoot, relativePath)
  fs.mkdirSync(path.dirname(absolutePath), { recursive: true })
  fs.writeFileSync(absolutePath, content)
}

function copyRepoFile(baseRoot, relativePath) {
  writeTo(baseRoot, relativePath, fs.readFileSync(path.join(root, relativePath), 'utf8'))
}

function readJson(baseRoot, relativePath) {
  return JSON.parse(fs.readFileSync(path.join(baseRoot, relativePath), 'utf8'))
}

function writeJson(baseRoot, relativePath, payload) {
  writeTo(baseRoot, relativePath, `${JSON.stringify(payload, null, 2)}\n`)
}

function createFixtureRoot(t) {
  const tmpRoot = mkTmpDir(t, 'm057-s01-evidence-')
  const siblingRoot = path.join(path.dirname(tmpRoot), 'hyperpush-mono', 'mesher')
  fs.mkdirSync(siblingRoot, { recursive: true })

  for (const relativePath of [
    files.evidenceScript,
    files.inventoryScript,
    files.repoIdentity,
    files.workspaceGit,
    files.project,
    files.decisions,
    files.m053,
    files.m054,
    files.m055,
    files.m056,
    files.meshSnapshot,
    files.hyperpushSnapshot,
    files.projectFieldsSnapshot,
    files.projectItemsSnapshot,
    files.docsConfig,
    files.docsNav,
  ]) {
    copyRepoFile(tmpRoot, relativePath)
  }

  writeTo(
    siblingRoot,
    'frontend-exp/lib/mock-data.ts',
    fs.readFileSync(path.join(root, files.productMockData), 'utf8'),
  )
  writeTo(
    siblingRoot,
    'landing/app/pitch/page.tsx',
    fs.readFileSync(path.join(root, files.productPitchPage), 'utf8'),
  )

  fs.symlinkSync('../hyperpush-mono/mesher', path.join(tmpRoot, 'mesher'), 'dir')

  return {
    tmpRoot,
    outputDir: path.join(tmpRoot, '.gsd/milestones/M057/slices/S01'),
  }
}

function runEvidence(baseRoot, extraArgs = []) {
  return spawnSync(
    PYTHON,
    [
      path.join(baseRoot, files.evidenceScript),
      '--source-root',
      baseRoot,
      '--output-dir',
      path.join(baseRoot, '.gsd/milestones/M057/slices/S01'),
      ...extraArgs,
    ],
    {
      cwd: baseRoot,
      encoding: 'utf8',
    },
  )
}

test('current repo publishes the M057 S01 evidence bundle and naming map with canonical ownership truth', () => {
  const result = runEvidence(root, ['--check'])
  assert.equal(result.status, 0, `${result.stdout}\n${result.stderr}`)

  const evidence = readJson(root, files.evidenceJson)
  const namingMap = readJson(root, files.namingMap)
  const markdown = fs.readFileSync(path.join(root, files.evidenceMd), 'utf8')

  assert.equal(evidence.version, 'm057-s01-reconciliation-evidence-v1')
  assert.equal(namingMap.version, 'm057-s01-naming-ownership-map-v1')
  assert.equal(evidence.rollup.entry_count, 5)
  assert.equal(namingMap.surfaces.length, 5)

  const evidenceById = new Map(evidence.entries.map((entry) => [entry.evidence_id, entry]))
  const namingById = new Map(namingMap.surfaces.map((surface) => [surface.surface_id, surface]))

  assert.deepEqual(
    [...evidenceById.keys()].sort(),
    [
      'frontend_exp_operator_surfaces_partial',
      'hyperpush_8_docs_bug_misfiled',
      'mesh_launch_foundations_shipped',
      'pitch_route_missing_tracker_coverage',
      'product_repo_naming_normalization',
    ],
  )

  assert.deepEqual(evidenceById.get('hyperpush_8_docs_bug_misfiled').matched_issue_handles, ['hyperpush#8'])
  assert.equal(evidenceById.get('pitch_route_missing_tracker_coverage').derived_gap.surface, '/pitch')
  assert.equal(evidenceById.get('product_repo_naming_normalization').public_repo_truth, 'hyperpush-org/hyperpush')

  const productSurface = namingById.get('hyperpush_product_repo')
  assert.match(productSurface.workspace_path_truth, /hyperpush-mono/)
  assert.equal(productSurface.public_repo_truth, 'hyperpush-org/hyperpush')
  assert.equal(productSurface.normalized_canonical_destination.repo_slug, 'hyperpush-org/hyperpush')

  assert.match(markdown, /hyperpush#8/)
  assert.match(markdown, /\/pitch/)
  assert.match(markdown, /workspace_path_truth/)
  assert.match(markdown, /public_repo_truth/)
})

test('evidence helper fails closed when a required milestone summary is missing', (t) => {
  const fixture = createFixtureRoot(t)
  fs.rmSync(path.join(fixture.tmpRoot, files.m056))

  const result = runEvidence(fixture.tmpRoot, ['--check'])
  assert.notEqual(result.status, 0)
  assert.match(result.stderr, /missing required source file: \.gsd\/milestones\/M056\/M056-SUMMARY\.md/)
  assert.ok(!fs.existsSync(path.join(fixture.outputDir, 'reconciliation-evidence.json')))
})

test('evidence helper fails closed when the mesher compatibility surface is unavailable', (t) => {
  const fixture = createFixtureRoot(t)
  fs.rmSync(path.join(fixture.tmpRoot, 'mesher'), { recursive: true, force: true })

  const result = runEvidence(fixture.tmpRoot, ['--check'])
  assert.notEqual(result.status, 0)
  assert.match(result.stderr, /missing compatibility path mesher\//)
})

test('evidence helper fails closed when repo identity drifts to an unresolved product slug', (t) => {
  const fixture = createFixtureRoot(t)
  const repoIdentity = readJson(fixture.tmpRoot, files.repoIdentity)
  repoIdentity.productRepo.slug = 'hyperpush-org/hyperpush-mono-legacy'
  writeJson(fixture.tmpRoot, files.repoIdentity, repoIdentity)

  const result = runEvidence(fixture.tmpRoot, ['--check'])
  assert.notEqual(result.status, 0)
  assert.match(result.stderr, /repo identity productRepo\.slug must be one of/)
})

test('evidence helper fails closed when the shipped /pitch route file disappears', (t) => {
  const fixture = createFixtureRoot(t)
  fs.rmSync(path.join(path.dirname(fixture.tmpRoot), 'hyperpush-mono', 'mesher', 'landing/app/pitch/page.tsx'))

  const result = runEvidence(fixture.tmpRoot, ['--check'])
  assert.notEqual(result.status, 0)
  assert.match(result.stderr, /missing required source file: mesher\/landing\/app\/pitch\/page\.tsx/)
})

test('evidence helper fails closed when the misfiled docs issue no longer points at the cited Packages nav source', (t) => {
  const fixture = createFixtureRoot(t)
  const configPath = path.join(fixture.tmpRoot, files.docsConfig)
  const config = fs.readFileSync(configPath, 'utf8').replace("{ text: 'Packages', link: '/packages/' }", "{ text: 'Registry', link: '/packages/' }")
  fs.writeFileSync(configPath, config)

  const result = runEvidence(fixture.tmpRoot, ['--check'])
  assert.notEqual(result.status, 0)
  assert.match(result.stderr, /website\/docs\/\.vitepress\/config\.mts missing required marker/)
})
