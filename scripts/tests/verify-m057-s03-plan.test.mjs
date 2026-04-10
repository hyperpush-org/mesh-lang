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
  planMd: '.gsd/milestones/M057/slices/S03/project-mutation-plan.md',
}

const expected = {
  version: 'm057-s03-project-mutation-plan-v1',
  delete: 10,
  add: 2,
  update: 23,
  unchanged: 30,
  inheritedRows: 23,
  currentProjectItems: 63,
  desiredProjectItems: 55,
  finalStatusCounts: {
    Todo: 50,
    'In Progress': 3,
    Done: 2,
  },
  repoTotals: {
    mesh_lang: { total: 17, open: 7, closed: 10 },
    hyperpush: { total: 52, open: 47, closed: 5 },
    combined_total: 69,
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
  leaveUntrackedHandles: ['hyperpush#2', 'hyperpush#3', 'hyperpush#4', 'hyperpush#5'],
  namingHandles: ['hyperpush#54', 'hyperpush#55', 'hyperpush#56'],
  namingSnapshotTitles: {
    'hyperpush#54': 'Hyperpush deploy topology: split landing marketing from frontend-exp app routing and runtime boundaries',
    'hyperpush#55': 'Hyperpush deployment: add a production Dockerfile and container startup path for frontend-exp',
    'hyperpush#56': 'Hyperpush deployment: create generic-VM compose stack and health verification for landing + frontend-exp + mesher backend',
  },
}

function readJson(baseRoot, relativePath) {
  return JSON.parse(fs.readFileSync(path.join(baseRoot, relativePath), 'utf8'))
}

function readText(baseRoot, relativePath) {
  return fs.readFileSync(path.join(baseRoot, relativePath), 'utf8')
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

function sorted(values) {
  return [...values].sort()
}

function requireSetEqual(actualValues, expectedValues, message) {
  assert.deepEqual(sorted(actualValues), sorted(expectedValues), message)
}

function findByHandle(rows, handle, label) {
  const row = rows.find((candidate) => candidate.canonical_issue_handle === handle)
  assert.ok(row, `${label} ${handle} should exist`)
  return row
}

function findChange(operation, fieldKey) {
  const change = operation.field_changes.find((candidate) => candidate.field_key === fieldKey)
  assert.ok(change, `${operation.canonical_issue_handle} should include a ${fieldKey} change`)
  return change
}

function validatePlan(plan, markdown) {
  assert.equal(plan.version, expected.version)
  assert.equal(plan.plan_status, 'ready')

  assert.equal(plan.preflight.status, 'ok')
  assert.equal(plan.preflight.exit_code, 0)
  assert.equal(plan.preflight.timed_out, false)
  assert.equal(plan.preflight.parsed.status, 'ok')
  assert.equal(plan.preflight.parsed.phase_report, '.tmp/m057-s02/verify/phase-report.txt')
  assert.equal(plan.preflight.parsed.summary_json, '.tmp/m057-s02/verify/verification-summary.json')
  assert.equal(plan.preflight.parsed.handoff_markdown, '.gsd/milestones/M057/slices/S02/repo-mutation-results.md')
  assert.deepEqual(plan.preflight.parsed.repo_totals, expected.repoTotals)

  assert.equal(plan.rollup.delete, expected.delete)
  assert.equal(plan.rollup.add, expected.add)
  assert.equal(plan.rollup.update, expected.update)
  assert.equal(plan.rollup.unchanged, expected.unchanged)
  assert.equal(plan.rollup.inherited_rows, expected.inheritedRows)
  assert.equal(plan.rollup.current_project_items, expected.currentProjectItems)
  assert.equal(plan.rollup.desired_project_items, expected.desiredProjectItems)
  assert.deepEqual(plan.rollup.final_status_counts, expected.finalStatusCounts)
  assert.deepEqual(plan.rollup.repo_totals, {
    mesh_lang_total: expected.repoTotals.mesh_lang.total,
    hyperpush_total: expected.repoTotals.hyperpush.total,
    combined_total: expected.repoTotals.combined_total,
  })

  assert.equal(plan.inheritance_rollup.rows, expected.inheritedRows)
  assert.equal(plan.inheritance_rollup.field_change_count, 154)
  assert.equal(plan.inheritance_rollup.deepest_chain_length, 3)

  const deleteOps = plan.operations.delete
  const addOps = plan.operations.add
  const updateOps = plan.operations.update

  assert.equal(deleteOps.length, expected.delete)
  assert.equal(addOps.length, expected.add)
  assert.equal(updateOps.length, expected.update)

  requireSetEqual(
    deleteOps.map((operation) => operation.canonical_issue_handle),
    expected.deleteHandles,
    'delete operations must retain the stale cleanup handle set',
  )
  assert.ok(deleteOps.every((operation) => operation.touch_reason === 'remove_stale_cleanup_row'))

  requireSetEqual(
    addOps.map((operation) => operation.canonical_issue_handle),
    expected.addHandles,
    'add operations must retain the canonical replacement handle set',
  )

  const mesh19 = findByHandle(addOps, 'mesh-lang#19', 'add operation')
  assert.equal(mesh19.touch_reason, 'add_canonical_replacement_row')
  assert.equal(mesh19.final_row_state.repo, 'hyperpush-org/mesh-lang')
  assert.equal(mesh19.final_row_state.issue_state, 'CLOSED')
  assert.equal(mesh19.final_row_state.field_values.status.value, 'Done')
  assert.equal(mesh19.final_row_state.field_values.domain.value, 'Mesh')
  assert.equal(mesh19.final_row_state.field_values.priority.value, null)

  const hyperpush58 = findByHandle(addOps, 'hyperpush#58', 'add operation')
  assert.equal(hyperpush58.touch_reason, 'add_missing_tracker_coverage_row')
  assert.equal(hyperpush58.final_row_state.repo, 'hyperpush-org/hyperpush')
  assert.equal(hyperpush58.final_row_state.issue_state, 'CLOSED')
  assert.equal(hyperpush58.final_row_state.field_values.status.value, 'Done')
  assert.equal(hyperpush58.final_row_state.field_values.domain.value, 'Hyperpush')
  assert.equal(hyperpush58.final_row_state.field_values.track.value, null)

  assert.equal(plan.canonical_mapping_handling.hyperpush_8_to_mesh_lang_19.destination_issue_handle, 'mesh-lang#19')
  assert.equal(plan.canonical_mapping_handling.hyperpush_8_to_mesh_lang_19.source_board_membership, 'absent')
  assert.equal(plan.canonical_mapping_handling.hyperpush_8_to_mesh_lang_19.destination_board_membership, 'missing_add_required')
  assert.equal(plan.canonical_mapping_handling.hyperpush_8_to_mesh_lang_19.planned_operation_id, 'add-mesh-lang-19')
  assert.equal(plan.canonical_mapping_handling.pitch_gap_to_hyperpush_58.destination_issue_handle, 'hyperpush#58')
  assert.equal(plan.canonical_mapping_handling.pitch_gap_to_hyperpush_58.planned_operation_id, 'add-hyperpush-58')

  const touchedHandles = new Set([
    ...deleteOps.map((operation) => operation.canonical_issue_handle),
    ...addOps.map((operation) => operation.canonical_issue_handle),
    ...updateOps.map((operation) => operation.canonical_issue_handle),
  ])
  for (const handle of expected.leaveUntrackedHandles) {
    assert.ok(!touchedHandles.has(handle), `${handle} should stay outside the board mutation manifest`)
  }

  const update33 = findByHandle(updateOps, 'hyperpush#33', 'update operation')
  assert.equal(update33.touch_reason, 'inherit_missing_metadata')
  assert.equal(update33.change_count, 8)
  for (const [fieldKey, value] of Object.entries({
    domain: 'Hyperpush',
    track: 'Operator App',
    commitment: 'Committed',
    delivery_mode: 'Shared',
    priority: 'P0',
    start_date: '2026-04-12',
    target_date: '2026-04-30',
    hackathon_phase: 'Phase 3 — Operator App',
  })) {
    const change = findChange(update33, fieldKey)
    assert.equal(change.change_reason, 'parent_chain_inheritance')
    assert.deepEqual(change.inheritance.chain, ['hyperpush#33', 'hyperpush#15'])
    assert.equal(change.after.value, value)
  }

  const update57 = findByHandle(updateOps, 'hyperpush#57', 'update operation')
  assert.equal(update57.touch_reason, 'inherit_missing_metadata')
  assert.equal(update57.change_count, 3)
  for (const [fieldKey, value] of Object.entries({
    domain: 'Hyperpush',
    track: 'Operator App',
    delivery_mode: 'Shared',
  })) {
    const change = findChange(update57, fieldKey)
    assert.equal(change.change_reason, 'parent_chain_inheritance')
    assert.deepEqual(change.inheritance.chain, ['hyperpush#57', 'hyperpush#34', 'hyperpush#15'])
    assert.equal(change.after.value, value)
  }

  assert.equal(plan.verified_noops.length, expected.unchanged)
  for (const handle of expected.namingHandles) {
    const row = findByHandle(plan.verified_noops, handle, 'verified noop')
    assert.equal(row.verification_kind, 'naming_normalization_preserved')
    assert.equal(row.historical_snapshot.title, expected.namingSnapshotTitles[handle])
    assert.equal(row.final_row_state.field_values.title.value, row.current_row.field_values.title.value)
  }

  assert.match(markdown, /## Preflight evidence/)
  assert.match(markdown, /## Canonical mapping handling/)
  assert.match(markdown, /## Delete operations/)
  assert.match(markdown, /## Add operations/)
  assert.match(markdown, /## Update operations/)
  assert.match(markdown, /## Verified no-op rows/)
  assert.match(markdown, /## Inheritance coverage/)
  assert.match(markdown, /\| `hyperpush#8` \| `mesh-lang#19` \| `replacement_mesh_row_must_exist` \| `add-mesh-lang-19` \|/)
  assert.match(markdown, /\| `\/pitch` gap \| `hyperpush#58` \| `replacement_hyperpush_row_must_exist` \| `add-hyperpush-58` \|/)
  assert.match(markdown, /\| `mesh-lang#3` \| `PVTI_/)
  assert.match(markdown, /\| `hyperpush#58` \| `hyperpush-org\/hyperpush` \| `Done` \| `Hyperpush` \|/)
  assert.match(markdown, /\| `mesh-lang#19` \| `hyperpush-org\/mesh-lang` \| `Done` \| `Mesh` \|/)
  assert.match(markdown, /`hyperpush#57` \| `3` \| `domain`: None -> 'Hyperpush' \(from hyperpush#57 -> hyperpush#34 -> hyperpush#15\)/)
}

test('current repo publishes the M057 S03 checked plan with green preflight, canonical add rows, stale deletes, and deterministic inheritance coverage', () => {
  validatePlan(readJson(root, files.planJson), readText(root, files.planMd))
})

test('plan artifact fails closed when a canonical replacement add row disappears', (t) => {
  const tmpRoot = mkTmpDir(t, 'm057-s03-plan-add-')
  const mutated = structuredClone(readJson(root, files.planJson))
  mutated.operations.add = mutated.operations.add.filter((operation) => operation.canonical_issue_handle !== 'mesh-lang#19')
  mutated.rollup.add = mutated.operations.add.length
  mutated.rollup.desired_project_items = 54
  const filePath = path.join(tmpRoot, 'project-mutation-plan.json')
  writeJson(filePath, mutated)

  assert.throws(
    () => validatePlan(readJson(tmpRoot, 'project-mutation-plan.json'), readText(root, files.planMd)),
    /mesh-lang#19|2|55/,
  )
})

test('plan artifact fails closed when the stale cleanup delete set loses a required row', (t) => {
  const tmpRoot = mkTmpDir(t, 'm057-s03-plan-delete-')
  const mutated = structuredClone(readJson(root, files.planJson))
  mutated.operations.delete = mutated.operations.delete.filter((operation) => operation.canonical_issue_handle !== 'mesh-lang#10')
  mutated.rollup.delete = mutated.operations.delete.length
  const filePath = path.join(tmpRoot, 'project-mutation-plan.json')
  writeJson(filePath, mutated)

  assert.throws(
    () => validatePlan(readJson(tmpRoot, 'project-mutation-plan.json'), readText(root, files.planMd)),
    /delete|mesh-lang#10|10/,
  )
})

test('plan artifact fails closed when the deep inheritance chain for hyperpush#57 drifts', (t) => {
  const tmpRoot = mkTmpDir(t, 'm057-s03-plan-inheritance-')
  const mutated = structuredClone(readJson(root, files.planJson))
  const update57 = mutated.operations.update.find((operation) => operation.canonical_issue_handle === 'hyperpush#57')
  update57.field_changes.find((change) => change.field_key === 'track').inheritance.chain = ['hyperpush#57', 'hyperpush#15']
  const filePath = path.join(tmpRoot, 'project-mutation-plan.json')
  writeJson(filePath, mutated)

  assert.throws(
    () => validatePlan(readJson(tmpRoot, 'project-mutation-plan.json'), readText(root, files.planMd)),
    /hyperpush#57|hyperpush#34|Operator App/,
  )
})

test('plan artifact fails closed when a naming-normalized noop loses its preserved historical snapshot', (t) => {
  const tmpRoot = mkTmpDir(t, 'm057-s03-plan-noop-')
  const mutated = structuredClone(readJson(root, files.planJson))
  const noop54 = mutated.verified_noops.find((row) => row.canonical_issue_handle === 'hyperpush#54')
  noop54.verification_kind = 'already_satisfied'
  noop54.historical_snapshot.title = 'drifted title'
  const filePath = path.join(tmpRoot, 'project-mutation-plan.json')
  writeJson(filePath, mutated)

  assert.throws(
    () => validatePlan(readJson(tmpRoot, 'project-mutation-plan.json'), readText(root, files.planMd)),
    /naming_normalization_preserved|landing marketing|hyperpush#54/,
  )
})
