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
  inventoryScript: 'scripts/lib/m057_tracker_inventory.py',
  evidenceScript: 'scripts/lib/m057_evidence_index.py',
  ledgerScript: 'scripts/lib/m057_reconciliation_ledger.py',
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
  ledgerJson: '.gsd/milestones/M057/slices/S01/reconciliation-ledger.json',
  auditMd: '.gsd/milestones/M057/slices/S01/reconciliation-audit.md',
  docsConfig: 'website/docs/.vitepress/config.mts',
  docsNav: 'website/docs/.vitepress/theme/components/NavBar.vue',
  productMockData: 'mesher/frontend-exp/lib/mock-data.ts',
  productPitchPage: 'mesher/landing/app/pitch/page.tsx',
}

const expected = {
  rowsTotal: 68,
  projectBackedRows: 63,
  nonProjectRows: 5,
  derivedGapCount: 1,
  nonProjectHyperpush: [2, 3, 4, 5, 8],
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
  const tmpRoot = mkTmpDir(t, 'm057-s01-ledger-')
  const siblingRoot = path.join(path.dirname(tmpRoot), 'hyperpush-mono', 'mesher')
  fs.mkdirSync(siblingRoot, { recursive: true })

  for (const relativePath of [
    files.inventoryScript,
    files.evidenceScript,
    files.ledgerScript,
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
    files.evidenceJson,
    files.evidenceMd,
    files.namingMap,
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

function runLedger(baseRoot, extraArgs = []) {
  return spawnSync(
    PYTHON,
    [
      path.join(baseRoot, files.ledgerScript),
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

function runLedgerValidator(baseRoot, relativeLedgerPath, relativeAuditPath) {
  return spawnSync(
    PYTHON,
    [
      '-c',
      `
import importlib.util
import json
import pathlib
import sys

base_root = pathlib.Path(sys.argv[1])
script_path = base_root / sys.argv[2]
ledger_path = base_root / sys.argv[3]
audit_path = base_root / sys.argv[4]
snapshot_dir = base_root / '.gsd/milestones/M057/slices/S01'

spec = importlib.util.spec_from_file_location('m057_reconciliation_ledger', script_path)
module = importlib.util.module_from_spec(spec)
sys.path.insert(0, str(script_path.parent))
spec.loader.exec_module(module)

ledger = json.loads(ledger_path.read_text())
audit_markdown = audit_path.read_text()
snapshots = module.load_snapshots(snapshot_dir)
module.validate_ledger_bundle(ledger, audit_markdown, snapshots)
print('ok')
`,
      baseRoot,
      files.ledgerScript,
      relativeLedgerPath,
      relativeAuditPath,
    ],
    {
      cwd: baseRoot,
      encoding: 'utf8',
    },
  )
}

test('current repo publishes the M057 S01 reconciliation ledger, derived gap, and grouped audit proof', () => {
  const result = runLedger(root, ['--check'])
  assert.equal(result.status, 0, `${result.stdout}\n${result.stderr}`)

  const ledger = readJson(root, files.ledgerJson)
  const audit = fs.readFileSync(path.join(root, files.auditMd), 'utf8')

  assert.equal(ledger.version, 'm057-s01-reconciliation-ledger-v1')
  assert.equal(ledger.join_key, 'canonical_issue_url')
  assert.equal(ledger.rollup.rows_total, expected.rowsTotal)
  assert.equal(ledger.rollup.project_backed_rows, expected.projectBackedRows)
  assert.equal(ledger.rollup.non_project_rows, expected.nonProjectRows)
  assert.equal(ledger.rollup.orphan_project_rows, 0)
  assert.equal(ledger.rollup.derived_gap_count, expected.derivedGapCount)
  assert.equal(ledger.rows.length, expected.rowsTotal)
  assert.equal(ledger.derived_gaps.length, expected.derivedGapCount)

  const rowsByHandle = new Map(ledger.rows.map((row) => [row.canonical_issue_handle, row]))
  assert.equal(rowsByHandle.get('mesh-lang#3').primary_audit_bucket, 'shipped-but-open')
  assert.equal(rowsByHandle.get('mesh-lang#3').proposed_repo_action_kind, 'close_as_shipped')
  assert.equal(rowsByHandle.get('hyperpush#8').primary_audit_bucket, 'misfiled')
  assert.equal(rowsByHandle.get('hyperpush#8').proposed_repo_action_kind, 'move_to_mesh_lang')
  assert.equal(rowsByHandle.get('hyperpush#15').primary_audit_bucket, 'keep-open')
  assert.equal(rowsByHandle.get('hyperpush#15').proposed_repo_action_kind, 'keep_open')
  assert.equal(rowsByHandle.get('hyperpush#24').primary_audit_bucket, 'rewrite-split')
  assert.equal(rowsByHandle.get('hyperpush#24').proposed_repo_action_kind, 'rewrite_scope')

  const nonProjectHandles = ledger.rows
    .filter((row) => !row.project_backed)
    .map((row) => row.canonical_issue_handle)
    .sort()
  assert.deepEqual(nonProjectHandles, expected.nonProjectHyperpush.map((number) => `hyperpush#${number}`).sort())

  for (const row of ledger.rows) {
    assert.ok(row.evidence_refs.length > 0, `${row.canonical_issue_handle} must include evidence_refs`)
    assert.ok(row.ownership_truth, `${row.canonical_issue_handle} must include ownership_truth`)
    assert.ok(row.delivery_truth, `${row.canonical_issue_handle} must include delivery_truth`)
    assert.ok(row.proposed_repo_action, `${row.canonical_issue_handle} must include proposed_repo_action`)
    assert.ok(row.proposed_project_action, `${row.canonical_issue_handle} must include proposed_project_action`)
    if (row.project_backed) {
      assert.ok(row.project_item_id, `${row.canonical_issue_handle} must include project_item_id when project-backed`)
    }
  }

  const gap = ledger.derived_gaps[0]
  assert.equal(gap.bucket, 'missing-coverage')
  assert.equal(gap.surface, '/pitch')
  assert.equal(gap.proposed_repo_action_kind, 'create_missing_issue')
  assert.equal(gap.proposed_project_action_kind, 'create_project_item')

  for (const heading of ['## shipped-but-open', '## rewrite/split', '## keep-open', '## misfiled', '## missing-coverage', '## naming-drift']) {
    assert.match(audit, new RegExp(heading.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')))
  }
  assert.match(audit, /\/pitch/)
  assert.match(audit, /hyperpush#8/)
  assert.match(audit, /mesh-lang#3/)
})

test('ledger helper fails closed when two repo issues collide on one canonical issue URL', (t) => {
  const fixture = createFixtureRoot(t)
  const hyperpushSnapshot = readJson(fixture.tmpRoot, files.hyperpushSnapshot)
  hyperpushSnapshot.issues[0].canonical_issue_url = 'https://github.com/hyperpush-org/mesh-lang/issues/3'
  writeJson(fixture.tmpRoot, files.hyperpushSnapshot, hyperpushSnapshot)

  const result = runLedger(fixture.tmpRoot, ['--check'])
  assert.notEqual(result.status, 0)
  assert.match(result.stderr, /duplicate canonical issue URL/i)
  assert.ok(!fs.existsSync(path.join(fixture.outputDir, 'reconciliation-ledger.json')))
})

test('ledger helper fails closed when a project-backed row loses its project_item_id', (t) => {
  const fixture = createFixtureRoot(t)
  const projectItems = readJson(fixture.tmpRoot, files.projectItemsSnapshot)
  projectItems.items[0].project_item_id = ''
  writeJson(fixture.tmpRoot, files.projectItemsSnapshot, projectItems)

  const result = runLedger(fixture.tmpRoot, ['--check'])
  assert.notEqual(result.status, 0)
  assert.match(result.stderr, /project_item_id/i)
})

test('ledger helper fails closed when a project item no longer matches any repo issue row', (t) => {
  const fixture = createFixtureRoot(t)
  const projectItems = readJson(fixture.tmpRoot, files.projectItemsSnapshot)
  projectItems.items[0].canonical_issue_url = 'https://github.com/hyperpush-org/hyperpush/issues/99999'
  writeJson(fixture.tmpRoot, files.projectItemsSnapshot, projectItems)

  const result = runLedger(fixture.tmpRoot, ['--check'])
  assert.notEqual(result.status, 0)
  assert.match(result.stderr, /orphan project/i)
})

test('ledger validator rejects tampered outputs with empty evidence refs and unknown action kinds', (t) => {
  const fixture = createFixtureRoot(t)
  const buildResult = runLedger(fixture.tmpRoot, ['--check'])
  assert.equal(buildResult.status, 0, `${buildResult.stdout}\n${buildResult.stderr}`)

  const ledger = readJson(fixture.tmpRoot, files.ledgerJson)
  ledger.rows[0].evidence_refs = []
  ledger.rows[0].proposed_repo_action_kind = 'invented_action'
  writeJson(fixture.tmpRoot, files.ledgerJson, ledger)

  const result = runLedgerValidator(fixture.tmpRoot, files.ledgerJson, files.auditMd)
  assert.notEqual(result.status, 0)
  assert.match(result.stderr, /unknown proposed_repo_action_kind|must include evidence_refs/)
})
