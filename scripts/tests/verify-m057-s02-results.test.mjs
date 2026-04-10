import test from 'node:test'
import assert from 'node:assert/strict'
import fs from 'node:fs'
import os from 'node:os'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const scriptDir = path.dirname(fileURLToPath(import.meta.url))
const root = path.resolve(scriptDir, '..', '..')

const files = {
  ledgerJson: '.gsd/milestones/M057/slices/S01/reconciliation-ledger.json',
  planJson: '.gsd/milestones/M057/slices/S02/repo-mutation-plan.json',
  resultsJson: '.gsd/milestones/M057/slices/S02/repo-mutation-results.json',
}

const expected = {
  totalOperations: 43,
  close: 10,
  rewrite: 31,
  transfer: 1,
  create: 1,
  meshLangTotalAfterTransfer: 17,
  hyperpushTotalAfterCreate: 52,
  combinedTotalAfterMutations: 69,
  rewriteScope: 21,
  mockBackedFollowThrough: 7,
  namingNormalizationHandles: ['hyperpush#54', 'hyperpush#55', 'hyperpush#56'],
  closedCloseHandles: [
    'mesh-lang#4',
    'mesh-lang#5',
    'mesh-lang#6',
    'mesh-lang#8',
    'mesh-lang#9',
    'mesh-lang#10',
    'mesh-lang#11',
    'mesh-lang#13',
    'mesh-lang#14',
  ],
  reopenedCloseHandle: 'mesh-lang#3',
  transferredHandle: 'mesh-lang#19',
  transferredUrl: 'https://github.com/hyperpush-org/mesh-lang/issues/19',
  transferredState: 'CLOSED',
  transferredClosedAt: '2026-04-10T17:09:24Z',
  createdHandle: 'hyperpush#58',
  createdUrl: 'https://github.com/hyperpush-org/hyperpush/issues/58',
}

function readJson(baseRoot, relativePath) {
  return JSON.parse(fs.readFileSync(path.join(baseRoot, relativePath), 'utf8'))
}

function mkTmpDir(t, prefix) {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), prefix))
  t.after(() => fs.rmSync(dir, { recursive: true, force: true }))
  return dir
}

function writeJson(filePath, payload) {
  fs.mkdirSync(path.dirname(filePath), { recursive: true })
  fs.writeFileSync(filePath, `${JSON.stringify(payload, null, 2)}\n`)
}

function normalizeMultiline(value) {
  return String(value ?? '').replace(/\r\n/g, '\n').trim()
}

function sortedUnique(values) {
  return [...new Set(values)].sort()
}

function requireSetEqual(actualValues, expectedValues, message) {
  assert.deepEqual(sortedUnique(actualValues), [...expectedValues].sort(), message)
}

function deriveExpectedBuckets(ledger) {
  const rewriteScopeHandles = []
  const mockBackedFollowThroughHandles = []

  for (const row of ledger.rows) {
    if (row.proposed_repo_action_kind === 'rewrite_scope') {
      rewriteScopeHandles.push(row.canonical_issue_handle)
      continue
    }

    const matchedEvidenceIds = Array.isArray(row.matched_evidence_ids) ? row.matched_evidence_ids : []
    if (
      row.proposed_repo_action_kind === 'keep_open' &&
      matchedEvidenceIds.length === 1 &&
      matchedEvidenceIds[0] === 'frontend_exp_operator_surfaces_partial'
    ) {
      mockBackedFollowThroughHandles.push(row.canonical_issue_handle)
    }
  }

  return {
    rewriteScopeHandles,
    mockBackedFollowThroughHandles,
  }
}

function validateResults(results, plan, ledger) {
  assert.equal(results.version, 'm057-s02-repo-mutation-results-v1')
  assert.equal(results.status, 'ok')
  assert.equal(results.mode, 'apply')
  assert.equal(results.source_plan.version, 'm057-s02-repo-mutation-plan-v1')
  assert.equal(results.source_plan.rollup.total_apply, expected.totalOperations)
  assert.equal(results.rollup.total, expected.totalOperations)
  assert.equal(results.rollup.applied, 0)
  assert.equal(results.rollup.already_satisfied, expected.totalOperations)
  assert.equal(results.rollup.failed, 0)
  assert.equal(results.operations.length, expected.totalOperations)

  const planOperationIds = new Set(plan.operations.map((operation) => operation.operation_id))
  requireSetEqual(
    results.operations.map((operation) => operation.operation_id),
    planOperationIds,
    'results must cover the exact plan operation set',
  )

  const operationsById = new Map(results.operations.map((operation) => [operation.operation_id, operation]))
  const closeOperations = results.operations.filter((operation) => operation.operation_kind === 'close')
  const rewriteOperations = results.operations.filter((operation) => operation.operation_kind === 'rewrite')
  const transferOperation = operationsById.get('transfer-hyperpush-8')
  const createOperation = operationsById.get('create-pitch-retrospective-issue')

  assert.equal(closeOperations.length, expected.close)
  assert.equal(rewriteOperations.length, expected.rewrite)
  assert.ok(transferOperation, 'results must preserve the transfer operation')
  assert.ok(createOperation, 'results must preserve the create operation')

  const closeHandles = closeOperations.map((operation) => operation.canonical_issue_handle)
  requireSetEqual(
    closeHandles,
    [...expected.closedCloseHandles, expected.reopenedCloseHandle],
    'close results must retain the expected mesh-lang close bucket handles',
  )

  for (const operation of closeOperations) {
    assert.equal(operation.status, 'already_satisfied')
    assert.ok(operation.matching_comment?.body, `${operation.operation_id} should preserve the closeout comment body`)
    assert.match(operation.matching_comment.body, /Closing as shipped\./)
    assert.ok(
      operation.matching_comment.body.includes(`Canonical issue: \`${operation.canonical_issue_handle}\`.`),
      `${operation.operation_id} should keep its canonical handle in the closeout comment`,
    )

    if (operation.canonical_issue_handle === expected.reopenedCloseHandle) {
      assert.equal(operation.final_state.state, 'OPEN')
      assert.equal(operation.final_state.closed_at, null)
      assert.equal(operation.final_state.state_reason, 'reopened')
      continue
    }

    assert.equal(operation.final_state.state, 'CLOSED')
    assert.ok(operation.final_state.closed_at, `${operation.operation_id} should preserve its close timestamp`)
    assert.equal(operation.final_state.state_reason, 'completed')
  }

  const { rewriteScopeHandles, mockBackedFollowThroughHandles } = deriveExpectedBuckets(ledger)
  const rewriteHandles = rewriteOperations.map((operation) => operation.canonical_issue_handle)
  const namingNormalizationHandles = rewriteHandles.filter(
    (handle) =>
      !rewriteScopeHandles.includes(handle) &&
      !mockBackedFollowThroughHandles.includes(handle),
  )

  assert.equal(rewriteScopeHandles.length, expected.rewriteScope)
  assert.equal(mockBackedFollowThroughHandles.length, expected.mockBackedFollowThrough)
  requireSetEqual(
    rewriteHandles.filter((handle) => rewriteScopeHandles.includes(handle)),
    rewriteScopeHandles,
    'rewrite results must retain all rewrite_scope rows from the ledger',
  )
  requireSetEqual(
    rewriteHandles.filter((handle) => mockBackedFollowThroughHandles.includes(handle)),
    mockBackedFollowThroughHandles,
    'rewrite results must retain all mock-backed follow-through rows from the ledger',
  )
  requireSetEqual(
    namingNormalizationHandles,
    expected.namingNormalizationHandles,
    'the only non-ledger rewrite buckets should be the public naming normalization rows',
  )

  for (const operation of rewriteOperations) {
    assert.equal(operation.status, 'already_satisfied')
    assert.equal(operation.final_state.state, 'OPEN')
    assert.equal(operation.final_state.issue_handle, operation.canonical_issue_handle)
    assert.equal(operation.final_state.repo_slug, 'hyperpush-org/hyperpush')
    assert.equal(operation.final_state.title, operation.requested.title_after)
    assert.equal(normalizeMultiline(operation.final_state.body), normalizeMultiline(operation.requested.body_after))
  }

  for (const handle of expected.namingNormalizationHandles) {
    const operation = rewriteOperations.find((candidate) => candidate.canonical_issue_handle === handle)
    assert.ok(operation, `${handle} must stay in the rewrite output`)
    assert.match(operation.final_state.body, /Public issue wording should refer to `hyperpush-org\/hyperpush`/)
    assert.doesNotMatch(operation.final_state.body, /Public issue wording should refer to `hyperpush-mono`/)
  }

  assert.equal(transferOperation.identity.changes_identity, true)
  assert.equal(transferOperation.requested.identity.after.repo_slug, 'hyperpush-org/mesh-lang')
  assert.equal(transferOperation.final_state.repo_slug, 'hyperpush-org/mesh-lang')
  assert.equal(transferOperation.final_state.issue_handle, expected.transferredHandle)
  assert.equal(transferOperation.final_state.issue_url, expected.transferredUrl)
  assert.equal(transferOperation.final_state.state, expected.transferredState)
  assert.equal(transferOperation.final_state.closed_at, expected.transferredClosedAt)
  assert.equal(transferOperation.final_state.state_reason, 'completed')
  assert.equal(transferOperation.final_state.title, transferOperation.requested.title_after)
  assert.equal(normalizeMultiline(transferOperation.final_state.body), normalizeMultiline(transferOperation.requested.body_after))
  assert.deepEqual(transferOperation.label_resolution.assignable, ['bug', 'documentation'])
  assert.deepEqual(transferOperation.label_resolution.unavailable, ['priority: low'])

  assert.equal(createOperation.identity.changes_identity, true)
  assert.equal(createOperation.requested.identity.after.repo_slug, 'hyperpush-org/hyperpush')
  assert.equal(createOperation.final_state.repo_slug, 'hyperpush-org/hyperpush')
  assert.equal(createOperation.final_state.issue_handle, expected.createdHandle)
  assert.equal(createOperation.final_state.issue_url, expected.createdUrl)
  assert.equal(createOperation.final_state.state, 'CLOSED')
  assert.equal(createOperation.final_state.title, createOperation.requested.title_after)
  assert.equal(normalizeMultiline(createOperation.final_state.body), normalizeMultiline(createOperation.requested.body_after))
  assert.ok(createOperation.matching_comment?.body, 'create operation must retain the retrospective close comment')
  assert.match(createOperation.matching_comment.body, /already shipped during M056/)

  const sourceRepoCounts = ledger.rollup.repo_counts
  assert.equal(sourceRepoCounts['hyperpush-org/mesh-lang'] + 1, expected.meshLangTotalAfterTransfer)
  assert.equal(sourceRepoCounts['hyperpush-org/hyperpush'], expected.hyperpushTotalAfterCreate)
  assert.equal(
    expected.meshLangTotalAfterTransfer + expected.hyperpushTotalAfterCreate,
    expected.combinedTotalAfterMutations,
  )
}

test('current repo publishes the M057 S02 live results artifact with the expected canonical mappings and bucket rollups', () => {
  const ledger = readJson(root, files.ledgerJson)
  const plan = readJson(root, files.planJson)
  const results = readJson(root, files.resultsJson)
  validateResults(results, plan, ledger)
})

test('results artifact fails closed when the transferred issue mapping loses its canonical destination URL', (t) => {
  const ledger = readJson(root, files.ledgerJson)
  const plan = readJson(root, files.planJson)
  const results = readJson(root, files.resultsJson)
  const tmpRoot = mkTmpDir(t, 'm057-s02-results-transfer-')
  const mutated = structuredClone(results)
  mutated.operations.find((operation) => operation.operation_id === 'transfer-hyperpush-8').final_state.issue_url = null
  const filePath = path.join(tmpRoot, 'repo-mutation-results.json')
  writeJson(filePath, mutated)

  assert.throws(
    () => validateResults(readJson(tmpRoot, 'repo-mutation-results.json'), plan, ledger),
    /Expected values to be strictly equal/,
  )
})

test('results artifact fails closed when the retrospective /pitch issue mapping loses its canonical URL', (t) => {
  const ledger = readJson(root, files.ledgerJson)
  const plan = readJson(root, files.planJson)
  const results = readJson(root, files.resultsJson)
  const tmpRoot = mkTmpDir(t, 'm057-s02-results-create-')
  const mutated = structuredClone(results)
  mutated.operations.find((operation) => operation.operation_id === 'create-pitch-retrospective-issue').final_state.issue_url = ''
  const filePath = path.join(tmpRoot, 'repo-mutation-results.json')
  writeJson(filePath, mutated)

  assert.throws(
    () => validateResults(readJson(tmpRoot, 'repo-mutation-results.json'), plan, ledger),
    /Expected values to be strictly equal/,
  )
})

test('results artifact fails closed when the reopened mesh issue is forced back into the shipped-closed bucket', (t) => {
  const ledger = readJson(root, files.ledgerJson)
  const plan = readJson(root, files.planJson)
  const results = readJson(root, files.resultsJson)
  const tmpRoot = mkTmpDir(t, 'm057-s02-results-reopened-close-')
  const mutated = structuredClone(results)
  const reopened = mutated.operations.find((operation) => operation.operation_id === 'close-mesh-lang-3')
  reopened.final_state.state = 'CLOSED'
  reopened.final_state.closed_at = '2026-04-10T08:29:56Z'
  reopened.final_state.state_reason = 'completed'
  const filePath = path.join(tmpRoot, 'repo-mutation-results.json')
  writeJson(filePath, mutated)

  assert.throws(
    () => validateResults(readJson(tmpRoot, 'repo-mutation-results.json'), plan, ledger),
    /OPEN|reopened/,
  )
})

test('results artifact fails closed when a rewritten issue drifts to closed state or bucket coverage becomes incomplete', (t) => {
  const ledger = readJson(root, files.ledgerJson)
  const plan = readJson(root, files.planJson)
  const results = readJson(root, files.resultsJson)
  const tmpRoot = mkTmpDir(t, 'm057-s02-results-rewrite-')
  const mutated = structuredClone(results)
  mutated.operations.find((operation) => operation.operation_id === 'rewrite-hyperpush-54').final_state.state = 'CLOSED'
  mutated.operations = mutated.operations.filter((operation) => operation.operation_id !== 'rewrite-hyperpush-57')
  mutated.rollup.total = mutated.operations.length
  mutated.rollup.already_satisfied = mutated.operations.length
  const filePath = path.join(tmpRoot, 'repo-mutation-results.json')
  writeJson(filePath, mutated)

  assert.throws(
    () => validateResults(readJson(tmpRoot, 'repo-mutation-results.json'), plan, ledger),
    /31|43|OPEN/,
  )
})
