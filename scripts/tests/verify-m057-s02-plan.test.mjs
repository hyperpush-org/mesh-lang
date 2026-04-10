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
  planScript: 'scripts/lib/m057_repo_mutation_plan.py',
  ledgerScript: 'scripts/lib/m057_reconciliation_ledger.py',
  evidenceScript: 'scripts/lib/m057_evidence_index.py',
  inventoryScript: 'scripts/lib/m057_tracker_inventory.py',
  ledgerJson: '.gsd/milestones/M057/slices/S01/reconciliation-ledger.json',
  auditMd: '.gsd/milestones/M057/slices/S01/reconciliation-audit.md',
  meshSnapshot: '.gsd/milestones/M057/slices/S01/mesh-lang-issues.snapshot.json',
  hyperpushSnapshot: '.gsd/milestones/M057/slices/S01/hyperpush-issues.snapshot.json',
  projectFieldsSnapshot: '.gsd/milestones/M057/slices/S01/project-fields.snapshot.json',
  projectItemsSnapshot: '.gsd/milestones/M057/slices/S01/project-items.snapshot.json',
  meshTemplate: '.github/ISSUE_TEMPLATE/feature_request.yml',
  hyperpushTemplate: '../hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml',
  planJson: '.gsd/milestones/M057/slices/S02/repo-mutation-plan.json',
  planMd: '.gsd/milestones/M057/slices/S02/repo-mutation-plan.md',
}

const expected = {
  close: 10,
  rewrite: 31,
  transfer: 1,
  create: 1,
  skipped: 26,
  totalApply: 43,
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
  const tmpRoot = mkTmpDir(t, 'm057-s02-plan-')
  const siblingRoot = path.join(path.dirname(tmpRoot), 'hyperpush-mono')
  fs.mkdirSync(siblingRoot, { recursive: true })

  for (const relativePath of [
    files.planScript,
    files.ledgerScript,
    files.evidenceScript,
    files.inventoryScript,
    files.ledgerJson,
    files.auditMd,
    files.meshSnapshot,
    files.hyperpushSnapshot,
    files.projectFieldsSnapshot,
    files.projectItemsSnapshot,
    files.meshTemplate,
  ]) {
    copyRepoFile(tmpRoot, relativePath)
  }

  writeTo(
    siblingRoot,
    '.github/ISSUE_TEMPLATE/feature_request.yml',
    fs.readFileSync(path.join(root, files.hyperpushTemplate), 'utf8'),
  )

  return {
    tmpRoot,
    sourceDir: path.join(tmpRoot, '.gsd/milestones/M057/slices/S01'),
    outputDir: path.join(tmpRoot, '.gsd/milestones/M057/slices/S02'),
  }
}

function runPlanner(baseRoot, extraArgs = []) {
  return spawnSync(
    PYTHON,
    [
      path.join(baseRoot, files.planScript),
      '--source-root',
      baseRoot,
      '--source-dir',
      path.join(baseRoot, '.gsd/milestones/M057/slices/S01'),
      '--output-dir',
      path.join(baseRoot, '.gsd/milestones/M057/slices/S02'),
      ...extraArgs,
    ],
    {
      cwd: baseRoot,
      encoding: 'utf8',
    },
  )
}

test('current repo publishes the M057 S02 repo mutation manifest with the expected touched set and exclusions', () => {
  const result = runPlanner(root, ['--check'])
  assert.equal(result.status, 0, `${result.stdout}\n${result.stderr}`)

  const plan = readJson(root, files.planJson)
  const markdown = fs.readFileSync(path.join(root, files.planMd), 'utf8')

  assert.equal(plan.version, 'm057-s02-repo-mutation-plan-v1')
  assert.equal(plan.rollup.close, expected.close)
  assert.equal(plan.rollup.rewrite, expected.rewrite)
  assert.equal(plan.rollup.transfer, expected.transfer)
  assert.equal(plan.rollup.create, expected.create)
  assert.equal(plan.rollup.skipped, expected.skipped)
  assert.equal(plan.rollup.total_apply, expected.totalApply)
  assert.equal(plan.operations.length, expected.totalApply)

  const operationIds = new Map(plan.operations.map((operation) => [operation.operation_id, operation]))
  const skippedHandles = new Set(plan.skipped_rows.map((row) => row.canonical_issue_handle))

  for (const handle of ['hyperpush#3', 'hyperpush#4', 'hyperpush#5']) {
    assert.ok(skippedHandles.has(handle), `${handle} should be excluded from the apply set once already closed`)
  }

  assert.equal(operationIds.get('transfer-hyperpush-8').operation_kind, 'transfer')
  assert.equal(operationIds.get('transfer-hyperpush-8').identity.after.repo_slug, 'hyperpush-org/mesh-lang')
  assert.equal(operationIds.get('transfer-hyperpush-8').title.after, '[Bug]: docs Packages nav link points to /packages instead of opening packages.meshlang.dev in a new tab')

  const pitchOperation = operationIds.get('create-pitch-retrospective-issue')
  assert.equal(pitchOperation.operation_kind, 'create')
  assert.equal(pitchOperation.surface, '/pitch')
  assert.match(pitchOperation.title.after, /\/pitch/)
  assert.match(pitchOperation.comment.body, /already shipped during M056/)
  assert.match(pitchOperation.body.after, /## Area\nlanding app/)

  const closeMesh3 = operationIds.get('close-mesh-lang-3')
  const closeMesh5 = operationIds.get('close-mesh-lang-5')
  assert.match(closeMesh3.comment.body, /mesh-lang#7/)
  assert.match(closeMesh5.comment.body, /mesh-lang#12/)

  const rewrite15 = operationIds.get('rewrite-hyperpush-15')
  const rewrite54 = operationIds.get('rewrite-hyperpush-54')
  assert.match(rewrite15.body.after, /partially mock-backed/i)
  assert.match(rewrite54.title.after, /marketing site/)
  assert.match(rewrite54.body.after, /Public issue wording should refer to `hyperpush-org\/hyperpush`/)

  for (const heading of ['## close', '## rewrite', '## transfer', '## create', '## skipped']) {
    assert.match(markdown, new RegExp(heading.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')))
  }
  assert.match(markdown, /hyperpush#8/)
  assert.match(markdown, /\/pitch/)
})

test('planner fails closed when a project-backed row loses project_item_id', (t) => {
  const fixture = createFixtureRoot(t)
  const ledger = readJson(fixture.tmpRoot, files.ledgerJson)
  const target = ledger.rows.find((row) => row.canonical_issue_handle === 'hyperpush#11')
  target.project_item_id = null
  writeJson(fixture.tmpRoot, files.ledgerJson, ledger)

  const result = runPlanner(fixture.tmpRoot, ['--check'])
  assert.notEqual(result.status, 0)
  assert.match(result.stderr, /project_item_id/i)
  assert.ok(!fs.existsSync(path.join(fixture.outputDir, 'repo-mutation-plan.json')))
})

test('planner fails closed on duplicate canonical issue handles in the source ledger', (t) => {
  const fixture = createFixtureRoot(t)
  const ledger = readJson(fixture.tmpRoot, files.ledgerJson)
  ledger.rows[1].canonical_issue_handle = ledger.rows[0].canonical_issue_handle
  writeJson(fixture.tmpRoot, files.ledgerJson, ledger)

  const result = runPlanner(fixture.tmpRoot, ['--check'])
  assert.notEqual(result.status, 0)
  assert.match(result.stderr, /duplicate canonical issue handle/i)
})

test('planner fails closed on unknown proposed_repo_action_kind values', (t) => {
  const fixture = createFixtureRoot(t)
  const ledger = readJson(fixture.tmpRoot, files.ledgerJson)
  ledger.rows[0].proposed_repo_action_kind = 'invented_action'
  writeJson(fixture.tmpRoot, files.ledgerJson, ledger)

  const result = runPlanner(fixture.tmpRoot, ['--check'])
  assert.notEqual(result.status, 0)
  assert.match(result.stderr, /unknown proposed_repo_action_kind/i)
})

test('planner fails closed when the /pitch derived gap is missing', (t) => {
  const fixture = createFixtureRoot(t)
  const ledger = readJson(fixture.tmpRoot, files.ledgerJson)
  ledger.derived_gaps = []
  writeJson(fixture.tmpRoot, files.ledgerJson, ledger)

  const result = runPlanner(fixture.tmpRoot, ['--check'])
  assert.notEqual(result.status, 0)
  assert.match(result.stderr, /derived gap|\/pitch/i)
})

test('planner refuses to plan a live closeout for already-closed hyperpush#3', (t) => {
  const fixture = createFixtureRoot(t)
  const ledger = readJson(fixture.tmpRoot, files.ledgerJson)
  const target = ledger.rows.find((row) => row.canonical_issue_handle === 'hyperpush#3')
  target.state = 'OPEN'
  writeJson(fixture.tmpRoot, files.ledgerJson, ledger)

  const result = runPlanner(fixture.tmpRoot, ['--check'])
  assert.notEqual(result.status, 0)
  assert.match(result.stderr, /already-closed row hyperpush#3|already closed/i)
})

test('planner falls back to simple heading rendering when the product feature template is unreadable', (t) => {
  const fixture = createFixtureRoot(t)
  fs.rmSync(path.join(path.dirname(fixture.tmpRoot), 'hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml'))

  const result = runPlanner(fixture.tmpRoot, ['--check'])
  assert.equal(result.status, 0, `${result.stdout}\n${result.stderr}`)

  const plan = readJson(fixture.tmpRoot, files.planJson)
  assert.equal(plan.template_context['hyperpush-org/hyperpush'].fallback_used, true)
  assert.match(plan.template_context['hyperpush-org/hyperpush'].fallback_reason, /unreadable template/i)
  assert.match(plan.operations.find((operation) => operation.operation_kind === 'create').body.after, /## Problem statement/)
})
