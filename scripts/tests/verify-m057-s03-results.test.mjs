import test from 'node:test'
import assert from 'node:assert/strict'
import fs from 'node:fs'
import os from 'node:os'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const scriptDir = path.dirname(fileURLToPath(import.meta.url))
const root = path.resolve(scriptDir, '..', '..')

const files = {
  planJson: '.gsd/milestones/M057/slices/S03/project-mutation-plan.json',
  resultsJson: '.gsd/milestones/M057/slices/S03/project-mutation-results.json',
  resultsMd: '.gsd/milestones/M057/slices/S03/project-mutation-results.md',
}

const expected = {
  version: 'm057-s03-project-mutation-results-v1',
  planVersion: 'm057-s03-project-mutation-plan-v1',
  delete: 10,
  add: 2,
  update: 23,
  total: 35,
  applied: 0,
  alreadySatisfied: 35,
  failed: 0,
  totalItems: 55,
  repoCounts: {
    'hyperpush-org/hyperpush': 48,
    'hyperpush-org/mesh-lang': 7,
  },
  statusCounts: {
    Done: 2,
    'In Progress': 3,
    Todo: 50,
  },
  deleteHandles: [
    'mesh-lang#3',
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
  addHandles: ['hyperpush#58', 'mesh-lang#19'],
  updateHandles: [
    'hyperpush#29',
    'hyperpush#30',
    'hyperpush#31',
    'hyperpush#32',
    'hyperpush#33',
    'hyperpush#34',
    'hyperpush#35',
    'hyperpush#36',
    'hyperpush#37',
    'hyperpush#38',
    'hyperpush#39',
    'hyperpush#40',
    'hyperpush#41',
    'hyperpush#42',
    'hyperpush#43',
    'hyperpush#44',
    'hyperpush#45',
    'hyperpush#46',
    'hyperpush#47',
    'hyperpush#48',
    'hyperpush#49',
    'hyperpush#50',
    'hyperpush#57',
  ],
  namingTitles: {
    'hyperpush#54': 'Hyperpush deploy topology: split marketing site from operator app routing and product runtime boundaries',
    'hyperpush#55': 'Hyperpush deployment: add a production Dockerfile and container startup path for the operator app',
    'hyperpush#56': 'Hyperpush deployment: create generic-VM compose stack and health verification for the marketing site, operator app, and product backend',
  },
  inheritedRows: {
    'hyperpush#29': {
      status: 'Todo',
      domain: 'Hyperpush',
      track: 'Core Parity',
      commitment: 'Committed',
      delivery_mode: 'Shared',
      priority: 'P0',
      start_date: '2026-04-10',
      target_date: '2026-04-24',
      hackathon_phase: 'Phase 2 — Parity',
    },
    'hyperpush#33': {
      status: 'Todo',
      domain: 'Hyperpush',
      track: 'Operator App',
      commitment: 'Committed',
      delivery_mode: 'Shared',
      priority: 'P0',
      start_date: '2026-04-12',
      target_date: '2026-04-30',
      hackathon_phase: 'Phase 3 — Operator App',
    },
    'hyperpush#35': {
      status: 'Todo',
      domain: 'Hyperpush',
      track: 'SaaS Growth',
      commitment: 'Planned',
      delivery_mode: 'SaaS-only',
      priority: 'P1',
      start_date: '2026-04-20',
      target_date: '2026-05-06',
      hackathon_phase: 'Phase 3 — Operator App',
    },
    'hyperpush#57': {
      status: 'Todo',
      domain: 'Hyperpush',
      track: 'Operator App',
      commitment: 'Committed',
      delivery_mode: 'Shared',
      priority: 'P0',
      start_date: '2026-04-12',
      target_date: '2026-04-30',
      hackathon_phase: 'Phase 3 — Operator App',
    },
  },
}

function readJson(baseRoot, relativePath) {
  return JSON.parse(fs.readFileSync(path.join(baseRoot, relativePath), 'utf8'))
}

function readText(baseRoot, relativePath) {
  return fs.readFileSync(path.join(baseRoot, relativePath), 'utf8')
}

function writeJson(filePath, payload) {
  fs.mkdirSync(path.dirname(filePath), { recursive: true })
  fs.writeFileSync(filePath, `${JSON.stringify(payload, null, 2)}\n`)
}

function mkTmpDir(t, prefix) {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), prefix))
  t.after(() => fs.rmSync(dir, { recursive: true, force: true }))
  return dir
}

function sorted(values) {
  return [...values].sort()
}

function requireSetEqual(actualValues, expectedValues, message) {
  assert.deepEqual(sorted(actualValues), sorted(expectedValues), message)
}

function flattenPlanOperationIds(plan) {
  return [
    ...plan.operations.delete,
    ...plan.operations.add,
    ...plan.operations.update,
  ].map((operation) => operation.operation_id)
}

function findByHandle(rows, handle, label) {
  const row = rows.find((candidate) => candidate.canonical_issue_handle === handle || candidate.issue_handle === handle)
  assert.ok(row, `${label} ${handle} should exist`)
  return row
}

function assertFieldValues(row, expectedFields, label) {
  const fieldValues = row.field_values
  for (const [fieldKey, expectedValue] of Object.entries(expectedFields)) {
    assert.equal(fieldValues[fieldKey].value, expectedValue, `${label} ${fieldKey} drifted`)
  }
}

function validateResults(results, plan, markdown) {
  assert.equal(results.version, expected.version)
  assert.equal(results.mode, 'apply')
  assert.equal(results.status, 'ok')
  assert.equal(results.source_script, 'scripts/lib/m057_project_mutation_apply.py')

  assert.equal(results.source_plan.path, '.gsd/milestones/M057/slices/S03/project-mutation-plan.json')
  assert.equal(results.source_plan.version, expected.planVersion)
  assert.equal(results.source_plan.repo_precheck.status, 'ok')
  assert.equal(results.source_plan.repo_precheck.exit_code, 0)
  assert.equal(results.source_plan.repo_precheck.parsed.status, 'ok')
  assert.equal(results.source_plan.repo_precheck.parsed.phase_report, '.tmp/m057-s02/verify/phase-report.txt')
  assert.equal(results.source_plan.repo_precheck.parsed.summary_json, '.tmp/m057-s02/verify/verification-summary.json')

  assert.equal(results.rollup.planned.delete, expected.delete)
  assert.equal(results.rollup.planned.add, expected.add)
  assert.equal(results.rollup.planned.update, expected.update)
  assert.equal(results.rollup.total, expected.total)
  assert.equal(results.rollup.applied, expected.applied)
  assert.equal(results.rollup.already_satisfied, expected.alreadySatisfied)
  assert.equal(results.rollup.failed, expected.failed)
  assert.deepEqual(results.rollup.completed_by_kind, {
    delete: expected.delete,
    add: expected.add,
    update: expected.update,
  })

  assert.equal(results.operations.length, expected.total)
  requireSetEqual(
    results.operations.map((operation) => operation.operation_id),
    flattenPlanOperationIds(plan),
    'results must preserve the exact checked plan operation set',
  )

  const deleteOps = results.operations.filter((operation) => operation.operation_kind === 'delete')
  const addOps = results.operations.filter((operation) => operation.operation_kind === 'add')
  const updateOps = results.operations.filter((operation) => operation.operation_kind === 'update')

  requireSetEqual(deleteOps.map((operation) => operation.canonical_issue_handle), expected.deleteHandles, 'delete touched-set drifted')
  requireSetEqual(addOps.map((operation) => operation.canonical_issue_handle), expected.addHandles, 'add touched-set drifted')
  requireSetEqual(updateOps.map((operation) => operation.canonical_issue_handle), expected.updateHandles, 'update touched-set drifted')

  for (const operation of results.operations) {
    assert.equal(operation.status, 'already_satisfied', `${operation.operation_id} should collapse to already_satisfied on the rerun snapshot`)
    assert.equal(Array.isArray(operation.command_log), true)
    assert.equal(operation.command_log.length, 0, `${operation.operation_id} should not rerun live mutations in the steady-state snapshot`)
  }

  for (const operation of deleteOps) {
    assert.equal(operation.final_state, null)
    assert.equal(operation.project_item_id_after, null)
    assert.equal(operation.skipped_reason, 'project_row_already_absent')
  }

  const mesh19 = findByHandle(addOps, 'mesh-lang#19', 'add operation')
  assert.equal(mesh19.final_state.issue_state, 'CLOSED')
  assert.equal(mesh19.final_state.project_item_id, 'PVTI_lADOEExRVs4BUM59zgpovuo')
  assert.equal(mesh19.final_state.field_values.title.value, '[Bug]: docs Packages nav link points to /packages instead of opening packages.meshlang.dev in a new tab')
  assertFieldValues(mesh19.final_state, {
    status: 'Done',
    domain: 'Mesh',
    track: null,
    commitment: null,
    delivery_mode: null,
    priority: null,
    start_date: null,
    target_date: null,
    hackathon_phase: null,
  }, 'mesh-lang#19')

  const hyperpush58 = findByHandle(addOps, 'hyperpush#58', 'add operation')
  assert.equal(hyperpush58.final_state.issue_state, 'CLOSED')
  assert.equal(hyperpush58.final_state.project_item_id, 'PVTI_lADOEExRVs4BUM59zgpoujA')
  assert.equal(hyperpush58.final_state.field_values.title.value, '[Feature]: record shipped /pitch evaluator route explicitly')
  assertFieldValues(hyperpush58.final_state, {
    status: 'Done',
    domain: 'Hyperpush',
    track: null,
    commitment: null,
    delivery_mode: null,
    priority: null,
    start_date: null,
    target_date: null,
    hackathon_phase: null,
  }, 'hyperpush#58')

  assert.deepEqual(results.final_live_capture.rollup.repo_counts, expected.repoCounts)
  assert.deepEqual(results.final_live_capture.status_counts, expected.statusCounts)
  assert.equal(results.final_live_capture.rollup.total_items, expected.totalItems)
  assert.equal(results.initial_live_capture.rollup.total_items, expected.totalItems)

  assert.deepEqual(results.canonical_mapping_results, {
    hyperpush_8_to_mesh_lang_19: {
      source_issue_handle: 'hyperpush#8',
      destination_issue_handle: 'mesh-lang#19',
      source_board_membership: 'absent',
      destination_board_membership: 'present',
      destination_project_item_id: 'PVTI_lADOEExRVs4BUM59zgpovuo',
    },
    pitch_gap_to_hyperpush_58: {
      gap_id: 'product_pitch_route_shipped_without_tracker_row',
      destination_issue_handle: 'hyperpush#58',
      destination_board_membership: 'present',
      destination_project_item_id: 'PVTI_lADOEExRVs4BUM59zgpoujA',
    },
  })

  assert.equal(results.representative_rows.done.issue_handle, 'mesh-lang#19')
  assert.equal(results.representative_rows.done.project_item_id, 'PVTI_lADOEExRVs4BUM59zgpovuo')
  assertFieldValues(results.representative_rows.done, { status: 'Done', domain: 'Mesh', track: null }, 'representative done')

  assert.equal(results.representative_rows.in_progress.issue_handle, 'hyperpush#54')
  assert.equal(results.representative_rows.in_progress.project_item_id, 'PVTI_lADOEExRVs4BUM59zgpjg5Q')
  assertFieldValues(results.representative_rows.in_progress, { status: 'In Progress', domain: 'Hyperpush', track: 'Deployment' }, 'representative in_progress')

  assert.equal(results.representative_rows.todo.issue_handle, 'hyperpush#29')
  assert.equal(results.representative_rows.todo.project_item_id, 'PVTI_lADOEExRVs4BUM59zgpjTg8')
  assertFieldValues(results.representative_rows.todo, { status: 'Todo', domain: 'Hyperpush', track: 'Core Parity' }, 'representative todo')

  requireSetEqual(
    results.naming_preserved_rows.map((row) => row.issue_handle),
    Object.keys(expected.namingTitles),
    'naming-preserved row set drifted',
  )
  for (const [handle, title] of Object.entries(expected.namingTitles)) {
    const row = findByHandle(results.naming_preserved_rows, handle, 'naming-preserved row')
    assert.equal(row.field_values.title.value, title)
    assert.equal(row.field_values.status.value, 'In Progress')
    assert.equal(row.field_values.domain.value, 'Hyperpush')
    assert.equal(row.field_values.track.value, 'Deployment')
    assert.doesNotMatch(row.field_values.title.value, /frontend-exp|landing marketing|mesher backend/)
  }

  for (const [handle, expectedFields] of Object.entries(expected.inheritedRows)) {
    const row = findByHandle(updateOps, handle, 'inherited row').final_state
    assertFieldValues(row, expectedFields, handle)
  }

  assert.match(markdown, /# M057 S03 Board Truth Verification/)
  assert.match(markdown, /## Final verified board truth/)
  assert.match(markdown, /## Canonical mapping handling/)
  assert.match(markdown, /## Removed stale cleanup rows/)
  assert.match(markdown, /## Naming-normalized active rows/)
  assert.match(markdown, /## Inherited metadata spot checks/)
  assert.match(markdown, /mesh-lang#19/)
  assert.match(markdown, /hyperpush#58/)
  assert.match(markdown, /hyperpush#54/)
  assert.match(markdown, /hyperpush#57/)
}

test('current repo publishes the M057 S03 results artifact with the full touched-set, canonical replacements, representative rows, and inherited metadata expectations locked', () => {
  validateResults(
    readJson(root, files.resultsJson),
    readJson(root, files.planJson),
    readText(root, files.resultsMd),
  )
})

test('results artifact fails closed when a canonical replacement destination disappears from the mapping summary', (t) => {
  const tmpRoot = mkTmpDir(t, 'm057-s03-results-mapping-')
  const plan = readJson(root, files.planJson)
  const mutated = structuredClone(readJson(root, files.resultsJson))
  mutated.canonical_mapping_results.hyperpush_8_to_mesh_lang_19.destination_board_membership = 'absent'
  mutated.canonical_mapping_results.hyperpush_8_to_mesh_lang_19.destination_project_item_id = null
  const jsonPath = path.join(tmpRoot, 'project-mutation-results.json')
  writeJson(jsonPath, mutated)

  assert.throws(
    () => validateResults(readJson(tmpRoot, 'project-mutation-results.json'), plan, readText(root, files.resultsMd)),
    /destination_board_membership|present|Expected values to be strictly deep-equal/,
  )
})

test('results artifact fails closed when a touched update row drops out of the rerun snapshot', (t) => {
  const tmpRoot = mkTmpDir(t, 'm057-s03-results-coverage-')
  const plan = readJson(root, files.planJson)
  const mutated = structuredClone(readJson(root, files.resultsJson))
  mutated.operations = mutated.operations.filter((operation) => operation.operation_id !== 'update-hyperpush-57')
  mutated.rollup.total = mutated.operations.length
  mutated.rollup.already_satisfied = mutated.operations.length
  mutated.rollup.completed_by_kind.update = 22
  const jsonPath = path.join(tmpRoot, 'project-mutation-results.json')
  writeJson(jsonPath, mutated)

  assert.throws(
    () => validateResults(readJson(tmpRoot, 'project-mutation-results.json'), plan, readText(root, files.resultsMd)),
    /hyperpush#57|22|34 !== 35|Expected values to be strictly equal/,
  )
})

test('results artifact fails closed when a naming-normalized live title regresses to stale public wording', (t) => {
  const tmpRoot = mkTmpDir(t, 'm057-s03-results-naming-')
  const plan = readJson(root, files.planJson)
  const mutated = structuredClone(readJson(root, files.resultsJson))
  const row = mutated.naming_preserved_rows.find((candidate) => candidate.issue_handle === 'hyperpush#54')
  row.field_values.title.value = 'Hyperpush deploy topology: split landing marketing from frontend-exp app routing and runtime boundaries'
  const jsonPath = path.join(tmpRoot, 'project-mutation-results.json')
  writeJson(jsonPath, mutated)

  assert.throws(
    () => validateResults(readJson(tmpRoot, 'project-mutation-results.json'), plan, readText(root, files.resultsMd)),
    /frontend-exp|landing marketing/,
  )
})

test('results artifact fails closed when an inherited metadata row drifts away from its checked field values', (t) => {
  const tmpRoot = mkTmpDir(t, 'm057-s03-results-inheritance-')
  const plan = readJson(root, files.planJson)
  const mutated = structuredClone(readJson(root, files.resultsJson))
  const row = mutated.operations.find((candidate) => candidate.operation_id === 'update-hyperpush-35').final_state
  row.field_values.delivery_mode.value = 'Shared'
  row.field_values.delivery_mode.option_id = '52e5e6ee'
  const jsonPath = path.join(tmpRoot, 'project-mutation-results.json')
  writeJson(jsonPath, mutated)

  assert.throws(
    () => validateResults(readJson(tmpRoot, 'project-mutation-results.json'), plan, readText(root, files.resultsMd)),
    /SaaS-only|delivery_mode|Shared/,
  )
})
