import test from 'node:test'
import assert from 'node:assert/strict'
import fs from 'node:fs'
import os from 'node:os'
import path from 'node:path'
import { spawnSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'

const scriptDir = path.dirname(fileURLToPath(import.meta.url))
const root = path.resolve(scriptDir, '..', '..')
const outputDirRelative = '.gsd/milestones/M057/slices/S03'
const s01DirRelative = '.gsd/milestones/M057/slices/S01'
const applyScript = path.join(root, 'scripts/lib/m057_project_mutation_apply.py')
const planPath = path.join(root, outputDirRelative, 'project-mutation-plan.json')
const fieldSnapshotPath = path.join(root, s01DirRelative, 'project-fields.snapshot.json')
const pythonResult = spawnSync('python3', ['-c', 'import sys; print(sys.executable)'], { encoding: 'utf8' })
assert.equal(pythonResult.status, 0, pythonResult.stderr)
const PYTHON = pythonResult.stdout.trim() || 'python3'

const plan = readJson(planPath)
const fieldSnapshot = readJson(fieldSnapshotPath)
const fieldById = new Map(fieldSnapshot.fields.map((field) => [field.field_id, field]))
const optionByFieldId = new Map(
  fieldSnapshot.fields.map((field) => [
    field.field_id,
    new Map((field.options || []).map((option) => [option.id, option.name])),
  ]),
)

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, 'utf8'))
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

function copyIntoTempRoot(tempRoot, relativePath) {
  const targetPath = path.join(tempRoot, relativePath)
  fs.mkdirSync(path.dirname(targetPath), { recursive: true })
  fs.copyFileSync(path.join(root, relativePath), targetPath)
  return targetPath
}

function parseIssueUrl(issueUrl) {
  const url = new URL(issueUrl)
  const parts = url.pathname.split('/').filter(Boolean)
  const owner = parts[0]
  const repo = parts[1]
  const number = Number(parts[3])
  return {
    repo: `${owner}/${repo}`,
    number,
    issue_handle: `${repo}#${number}`,
  }
}

function cloneFieldValues(fieldValues) {
  return structuredClone(fieldValues)
}

function buildRowState({ projectItemId, issueUrl, issueState, fieldValues, repo, number }) {
  const derived = parseIssueUrl(issueUrl)
  return {
    project_item_id: projectItemId,
    canonical_issue_url: issueUrl,
    issue: {
      repo: repo || derived.repo,
      number: number || derived.number,
      title: fieldValues.title.value,
      state: issueState || 'OPEN',
      url: issueUrl,
    },
    field_values: cloneFieldValues(fieldValues),
  }
}

function buildInitialProjectRows() {
  const rows = []

  for (const operation of plan.operations.delete) {
    const currentRow = operation.current_row
    rows.push(
      buildRowState({
        projectItemId: currentRow.project_item_id,
        issueUrl: currentRow.issue_url,
        issueState: currentRow.field_values.status.value === 'Done' ? 'CLOSED' : 'OPEN',
        fieldValues: currentRow.field_values,
      }),
    )
  }

  for (const operation of plan.operations.update) {
    const currentRow = operation.current_row
    rows.push(
      buildRowState({
        projectItemId: currentRow.project_item_id,
        issueUrl: currentRow.issue_url,
        issueState: operation.final_row_state.issue_state,
        fieldValues: currentRow.field_values,
        repo: operation.final_row_state.repo,
        number: operation.final_row_state.number,
      }),
    )
  }

  for (const row of plan.verified_noops) {
    rows.push(
      buildRowState({
        projectItemId: row.current_row.project_item_id,
        issueUrl: row.current_row.issue_url,
        issueState: row.final_row_state.issue_state,
        fieldValues: row.current_row.field_values,
        repo: row.final_row_state.repo,
        number: row.final_row_state.number,
      }),
    )
  }

  rows.sort((left, right) => left.canonical_issue_url.localeCompare(right.canonical_issue_url))
  assert.equal(rows.length, plan.rollup.current_project_items)
  return rows
}

function buildAddTemplates() {
  return Object.fromEntries(
    plan.operations.add.map((operation, index) => {
      const finalRow = operation.final_row_state
      return [
        operation.canonical_issue_url,
        {
          repo: finalRow.repo,
          number: finalRow.number,
          title: finalRow.field_values.title.value,
          state: finalRow.issue_state,
          seed_item_id: `PVTI_fake_add_${index + 1}`,
          field_values: structuredClone(finalRow.field_values),
        },
      ]
    }),
  )
}

function buildFakeState({ simulatePostAddPaginationDrift = false } = {}) {
  return {
    project: {
      id: fieldSnapshot.project.id,
      title: fieldSnapshot.project.title,
      url: fieldSnapshot.project.url,
      owner: fieldSnapshot.project.owner,
      number: fieldSnapshot.project.number,
    },
    projectFields: {
      totalCount: fieldSnapshot.field_count,
      fields: fieldSnapshot.fields.map((field) => ({
        id: field.field_id,
        name: field.field_name,
        type: field.field_type,
        options: (field.options || []).map((option) => ({ id: option.id, name: option.name })),
      })),
    },
    trackedFieldKeys: [...fieldSnapshot.tracked_field_keys],
    items: buildInitialProjectRows(),
    nextItemCounter: 1,
    addTemplates: buildAddTemplates(),
    simulatePostAddPaginationDrift,
    transientGraphqlDriftRemaining: 0,
  }
}

function buildFakeGhScript(tempRoot) {
  const ghPath = path.join(tempRoot, 'gh')
  const script = String.raw`#!/usr/bin/env node
const fs = require('fs')

const statePath = process.env.FAKE_GH_STATE
if (!statePath) {
  console.error('FAKE_GH_STATE is required')
  process.exit(1)
}

function loadState() {
  return JSON.parse(fs.readFileSync(statePath, 'utf8'))
}

function saveState(state) {
  fs.writeFileSync(statePath, JSON.stringify(state, null, 2) + '\n')
}

function printJson(payload) {
  process.stdout.write(JSON.stringify(payload))
}

function parseArgs(args) {
  const positional = []
  const flags = new Map()
  for (let i = 0; i < args.length; i += 1) {
    const arg = args[i]
    if (arg.startsWith('--')) {
      const next = args[i + 1]
      if (next && !next.startsWith('-')) {
        flags.set(arg, next)
        i += 1
      } else {
        flags.set(arg, true)
      }
      continue
    }
    positional.push(arg)
  }
  return { positional, flags }
}

function fieldDefinitionById(state, fieldId) {
  const field = state.projectFields.fields.find((candidate) => candidate.id === fieldId)
  if (!field) {
    console.error('unknown field id: ' + fieldId)
    process.exit(1)
  }
  return field
}

function optionNameFor(field, optionId) {
  return (field.options || []).find((candidate) => candidate.id === optionId)?.name || null
}

function findItemById(state, itemId) {
  return state.items.find((item) => item.project_item_id === itemId) || null
}

function defaultFieldValues(state, title) {
  const values = {}
  for (const field of state.projectFields.fields) {
    const fieldKey = field.name.toLowerCase().replace(/[^a-z0-9]+/g, '_').replace(/^_+|_+$/g, '')
    values[fieldKey] = {
      field_id: field.id,
      field_name: field.name,
      field_key: fieldKey,
      field_type: field.type,
      value: field.name === 'Title' ? title : null,
      option_id: null,
      value_type: field.name === 'Title' ? 'ProjectV2ItemFieldTextValue' : null,
    }
  }
  return values
}

function buildValueNodes(row) {
  const nodes = []
  for (const fieldValue of Object.values(row.field_values)) {
    if (fieldValue.field_key === 'title') {
      nodes.push({
        __typename: 'ProjectV2ItemFieldTextValue',
        text: fieldValue.value,
        field: { id: fieldValue.field_id, name: fieldValue.field_name },
      })
      continue
    }
    if (fieldValue.value == null && fieldValue.option_id == null) continue
    if (fieldValue.field_key === 'start_date' || fieldValue.field_key === 'target_date') {
      nodes.push({
        __typename: 'ProjectV2ItemFieldDateValue',
        date: fieldValue.value,
        field: { id: fieldValue.field_id, name: fieldValue.field_name },
      })
      continue
    }
    nodes.push({
      __typename: 'ProjectV2ItemFieldSingleSelectValue',
      name: fieldValue.value,
      optionId: fieldValue.option_id,
      field: { id: fieldValue.field_id, name: fieldValue.field_name },
    })
  }
  return nodes
}

function handleProjectFieldList(state) {
  printJson(state.projectFields)
}

function handleProjectItemDelete(state, args) {
  const { flags } = parseArgs(args)
  const itemId = flags.get('--id')
  const before = state.items.length
  state.items = state.items.filter((item) => item.project_item_id !== itemId)
  if (state.items.length === before) {
    console.error('item not found for delete: ' + itemId)
    process.exit(1)
  }
  saveState(state)
  printJson({ id: itemId, deleted: true })
}

function handleProjectItemAdd(state, args) {
  const { flags } = parseArgs(args)
  const issueUrl = flags.get('--url')
  const template = state.addTemplates[issueUrl]
  if (!template) {
    console.error('unexpected add url: ' + issueUrl)
    process.exit(1)
  }
  const itemId = 'PVTI_fake_added_' + String(state.nextItemCounter).padStart(2, '0')
  state.nextItemCounter += 1
  state.items.push({
    project_item_id: itemId,
    canonical_issue_url: issueUrl,
    issue: {
      repo: template.repo,
      number: template.number,
      title: template.title,
      state: template.state,
      url: issueUrl,
    },
    field_values: defaultFieldValues(state, template.title),
  })
  state.items.sort((left, right) => left.canonical_issue_url.localeCompare(right.canonical_issue_url))
  if (state.simulatePostAddPaginationDrift && state.transientGraphqlDriftRemaining === 0) {
    state.transientGraphqlDriftRemaining = 1
  }
  saveState(state)
  printJson({ id: itemId })
}

function handleProjectItemEdit(state, args) {
  const { flags } = parseArgs(args)
  const itemId = flags.get('--id')
  const fieldId = flags.get('--field-id')
  const item = findItemById(state, itemId)
  if (!item) {
    console.error('item not found for edit: ' + itemId)
    process.exit(1)
  }
  const field = fieldDefinitionById(state, fieldId)
  const fieldKey = field.name.toLowerCase().replace(/[^a-z0-9]+/g, '_').replace(/^_+|_+$/g, '')
  const nextFieldValue = item.field_values[fieldKey]
  if (!nextFieldValue) {
    console.error('field missing on item: ' + fieldKey)
    process.exit(1)
  }
  if (flags.has('--clear')) {
    nextFieldValue.value = null
    nextFieldValue.option_id = null
    nextFieldValue.value_type = null
  } else if (flags.has('--single-select-option-id')) {
    const optionId = flags.get('--single-select-option-id')
    nextFieldValue.option_id = optionId
    nextFieldValue.value = optionNameFor(field, optionId)
    nextFieldValue.value_type = 'ProjectV2ItemFieldSingleSelectValue'
  } else if (flags.has('--date')) {
    nextFieldValue.value = flags.get('--date')
    nextFieldValue.option_id = null
    nextFieldValue.value_type = 'ProjectV2ItemFieldDateValue'
  } else if (flags.has('--text')) {
    nextFieldValue.value = flags.get('--text')
    nextFieldValue.option_id = null
    nextFieldValue.value_type = 'ProjectV2ItemFieldTextValue'
    if (fieldKey === 'title') {
      item.issue.title = nextFieldValue.value
    }
  } else {
    console.error('unsupported item-edit flags')
    process.exit(1)
  }
  saveState(state)
  printJson({ id: itemId, fieldId })
}

function handleProject(state, args) {
  if (args[0] === 'field-list') {
    handleProjectFieldList(state)
    return
  }
  if (args[0] === 'item-delete') {
    handleProjectItemDelete(state, args.slice(1))
    return
  }
  if (args[0] === 'item-add') {
    handleProjectItemAdd(state, args.slice(1))
    return
  }
  if (args[0] === 'item-edit') {
    handleProjectItemEdit(state, args.slice(1))
    return
  }
  console.error('unsupported fake gh project command: ' + args.join(' '))
  process.exit(1)
}

function handleGraphql(state, args) {
  const { flags } = parseArgs(args)
  const after = flags.get('-F') && String(flags.get('-F')).startsWith('after=') ? String(flags.get('-F')).slice('after='.length) : null
  let afterCursor = null
  for (let i = 0; i < args.length; i += 1) {
    if ((args[i] === '-F' || args[i] === '-f') && args[i + 1] && String(args[i + 1]).startsWith('after=')) {
      afterCursor = String(args[i + 1]).slice('after='.length)
    }
  }
  const start = afterCursor ? Number(afterCursor) : 0
  const pageSize = 50
  const pageRows = state.items.slice(start, start + pageSize)
  const nextStart = start + pageRows.length
  const hasNextPage = nextStart < state.items.length
  const totalCount = state.transientGraphqlDriftRemaining > 0 && afterCursor ? state.items.length + 1 : state.items.length
  if (state.transientGraphqlDriftRemaining > 0 && afterCursor) {
    state.transientGraphqlDriftRemaining -= 1
    saveState(state)
  }
  const nodes = pageRows.map((row) => ({
    id: row.project_item_id,
    content: {
      __typename: 'Issue',
      number: row.issue.number,
      title: row.issue.title,
      state: row.issue.state,
      url: row.canonical_issue_url,
      repository: {
        nameWithOwner: row.issue.repo,
        url: 'https://github.com/' + row.issue.repo,
      },
    },
    fieldValues: {
      pageInfo: {
        hasNextPage: false,
        endCursor: null,
      },
      nodes: buildValueNodes(row),
    },
  }))
  printJson({
    data: {
      organization: {
        projectV2: {
          id: state.project.id,
          title: state.project.title,
          url: state.project.url,
          items: {
            totalCount,
            pageInfo: {
              hasNextPage,
              endCursor: hasNextPage ? String(nextStart) : null,
            },
            nodes,
          },
        },
      },
    },
  })
}

const state = loadState()
const args = process.argv.slice(2)
if (args[0] === 'project') {
  handleProject(state, args.slice(1))
} else if (args[0] === 'api' && args[1] === 'graphql') {
  handleGraphql(state, args.slice(2))
} else {
  console.error('unsupported fake gh command: ' + args.join(' '))
  process.exit(1)
}
`
  fs.writeFileSync(ghPath, script, { mode: 0o755 })
  return ghPath
}

function runApply(tempRoot, extraArgs = [], extraEnv = {}) {
  const sourceRoot = tempRoot
  const s01Dir = path.join(tempRoot, s01DirRelative)
  const outputDir = path.join(tempRoot, outputDirRelative)
  fs.mkdirSync(s01Dir, { recursive: true })
  fs.mkdirSync(outputDir, { recursive: true })
  const result = spawnSync(
    PYTHON,
    [
      applyScript,
      '--source-root', sourceRoot,
      '--s01-dir', s01Dir,
      '--output-dir', outputDir,
      ...extraArgs,
    ],
    {
      cwd: root,
      encoding: 'utf8',
      env: { ...process.env, ...extraEnv },
    },
  )
  return { ...result, sourceRoot, s01Dir, outputDir }
}

test('apply helper fails closed when the checked S03 plan file is invalid JSON before any live project mutation', (t) => {
  const tempRoot = mkTmpDir(t, 'm057-s03-apply-malformed-')
  copyIntoTempRoot(tempRoot, path.join(outputDirRelative, 'project-mutation-plan.json'))
  copyIntoTempRoot(tempRoot, path.join(s01DirRelative, 'project-fields.snapshot.json'))

  const tempPlanPath = path.join(tempRoot, outputDirRelative, 'project-mutation-plan.json')
  fs.writeFileSync(tempPlanPath, '{\n  "version": "broken"\n', 'utf8')

  const result = runApply(tempRoot, ['--check'], { PATH: '' })
  assert.notEqual(result.status, 0)
  assert.match(result.stderr, /not valid JSON/i)
})

test('apply helper mutates the checked project board deterministically and reruns idempotently against a fake gh backend', (t) => {
  const tempRoot = mkTmpDir(t, 'm057-s03-apply-fake-')
  copyIntoTempRoot(tempRoot, path.join(outputDirRelative, 'project-mutation-plan.json'))
  copyIntoTempRoot(tempRoot, path.join(s01DirRelative, 'project-fields.snapshot.json'))

  const statePath = path.join(tempRoot, 'fake-gh-state.json')
  writeJson(statePath, buildFakeState())
  buildFakeGhScript(tempRoot)

  const env = {
    FAKE_GH_STATE: statePath,
    PATH: `${tempRoot}:${process.env.PATH}`,
  }

  const firstRun = runApply(tempRoot, ['--apply'], env)
  assert.equal(firstRun.status, 0, `${firstRun.stdout}\n${firstRun.stderr}`)

  const firstPayload = JSON.parse(firstRun.stdout)
  const expectedLastOperationPath = fs.realpathSync(path.join(tempRoot, '.tmp/m057-s03/apply/last-operation.txt'))
  assert.equal(firstPayload.status, 'ok')
  assert.equal(firstPayload.rollup.total, 35)
  assert.equal(firstPayload.rollup.applied, 35)
  assert.equal(firstPayload.rollup.already_satisfied, 0)
  assert.equal(firstPayload.last_attempted_operation_id, 'update-hyperpush-57')
  assert.equal(firstPayload.last_operation_path, expectedLastOperationPath)

  const resultsPath = path.join(firstRun.outputDir, 'project-mutation-results.json')
  const markdownPath = path.join(firstRun.outputDir, 'project-mutation-results.md')
  const lastOperationPath = path.join(tempRoot, '.tmp/m057-s03/apply/last-operation.txt')

  const results = readJson(resultsPath)
  assert.equal(results.status, 'ok')
  assert.equal(results.mode, 'apply')
  assert.equal(results.rollup.total, 35)
  assert.equal(results.rollup.applied, 35)
  assert.equal(results.rollup.already_satisfied, 0)
  assert.equal(results.rollup.failed, 0)
  assert.deepEqual(results.final_live_capture.status_counts, plan.rollup.final_status_counts)
  assert.equal(results.final_live_capture.rollup.total_items, plan.rollup.desired_project_items)
  assert.equal(results.operations[0].operation_id, 'delete-mesh-lang-10')
  assert.equal(results.operations.at(-1).operation_id, 'update-hyperpush-57')

  const addMesh19 = results.operations.find((operation) => operation.operation_id === 'add-mesh-lang-19')
  const addHyperpush58 = results.operations.find((operation) => operation.operation_id === 'add-hyperpush-58')
  const update29 = results.operations.find((operation) => operation.operation_id === 'update-hyperpush-29')
  assert.equal(addMesh19.status, 'applied')
  assert.equal(addHyperpush58.status, 'applied')
  assert.equal(update29.status, 'applied')
  assert.equal(addMesh19.command_log.length, 3)
  assert.equal(addHyperpush58.command_log.length, 3)
  assert.equal(update29.command_log.length, 3)
  assert.equal(addMesh19.final_state.field_values.status.value, 'Done')
  assert.equal(addMesh19.final_state.field_values.domain.value, 'Mesh')
  assert.equal(addHyperpush58.final_state.field_values.status.value, 'Done')
  assert.equal(addHyperpush58.final_state.field_values.domain.value, 'Hyperpush')
  assert.equal(update29.final_state.field_values.track.value, 'Core Parity')

  assert.equal(results.canonical_mapping_results.hyperpush_8_to_mesh_lang_19.source_board_membership, 'absent')
  assert.equal(results.canonical_mapping_results.hyperpush_8_to_mesh_lang_19.destination_board_membership, 'present')
  assert.equal(results.canonical_mapping_results.pitch_gap_to_hyperpush_58.destination_board_membership, 'present')
  assert.equal(results.representative_rows.done.issue_handle, 'mesh-lang#19')
  assert.equal(results.representative_rows.in_progress.issue_handle, 'hyperpush#54')
  assert.equal(results.representative_rows.todo.issue_handle, 'hyperpush#29')
  assert.deepEqual(
    results.naming_preserved_rows.map((row) => row.issue_handle),
    ['hyperpush#54', 'hyperpush#55', 'hyperpush#56'],
  )

  const markdown = fs.readFileSync(markdownPath, 'utf8')
  assert.match(markdown, /## Final board state/)
  assert.match(markdown, /## Canonical mapping handling/)
  assert.match(markdown, /## Representative done \/ active \/ next rows/)
  assert.match(markdown, /\| `add-mesh-lang-19` \| `add` \| `applied` \| `mesh-lang#19` \|/)

  const lastOperation = fs.readFileSync(lastOperationPath, 'utf8')
  assert.match(lastOperation, /operation_id=update-hyperpush-57/)
  assert.match(lastOperation, /canonical_issue_handle=hyperpush#57/)

  const fakeState = readJson(statePath)
  assert.equal(fakeState.items.length, plan.rollup.desired_project_items)
  const mesh19State = fakeState.items.find((item) => item.canonical_issue_url === 'https://github.com/hyperpush-org/mesh-lang/issues/19')
  const hyperpush58State = fakeState.items.find((item) => item.canonical_issue_url === 'https://github.com/hyperpush-org/hyperpush/issues/58')
  assert.ok(mesh19State)
  assert.ok(hyperpush58State)
  assert.equal(mesh19State.field_values.status.value, 'Done')
  assert.equal(mesh19State.field_values.domain.value, 'Mesh')
  assert.equal(hyperpush58State.field_values.status.value, 'Done')
  assert.equal(hyperpush58State.field_values.domain.value, 'Hyperpush')

  const secondRun = runApply(tempRoot, ['--apply'], env)
  assert.equal(secondRun.status, 0, `${secondRun.stdout}\n${secondRun.stderr}`)

  const rerunPayload = JSON.parse(secondRun.stdout)
  assert.equal(rerunPayload.rollup.total, 35)
  assert.equal(rerunPayload.rollup.applied, 0)
  assert.equal(rerunPayload.rollup.already_satisfied, 35)

  const rerunResults = readJson(resultsPath)
  assert.equal(rerunResults.status, 'ok')
  assert.equal(rerunResults.rollup.total, 35)
  assert.equal(rerunResults.rollup.applied, 0)
  assert.equal(rerunResults.rollup.already_satisfied, 35)

  const rerunMesh19 = rerunResults.operations.find((operation) => operation.operation_id === 'add-mesh-lang-19')
  const rerunDelete10 = rerunResults.operations.find((operation) => operation.operation_id === 'delete-mesh-lang-10')
  const rerunUpdate29 = rerunResults.operations.find((operation) => operation.operation_id === 'update-hyperpush-29')
  assert.equal(rerunMesh19.status, 'already_satisfied')
  assert.equal(rerunDelete10.status, 'already_satisfied')
  assert.equal(rerunUpdate29.status, 'already_satisfied')
  assert.equal(rerunMesh19.command_log.length, 0)
  assert.equal(rerunDelete10.command_log.length, 0)
  assert.equal(rerunUpdate29.command_log.length, 0)
})

test('apply helper retries transient post-add project pagination drift and still completes the checked manifest', (t) => {
  const tempRoot = mkTmpDir(t, 'm057-s03-apply-drift-')
  copyIntoTempRoot(tempRoot, path.join(outputDirRelative, 'project-mutation-plan.json'))
  copyIntoTempRoot(tempRoot, path.join(s01DirRelative, 'project-fields.snapshot.json'))

  const statePath = path.join(tempRoot, 'fake-gh-state.json')
  writeJson(statePath, buildFakeState({ simulatePostAddPaginationDrift: true }))
  buildFakeGhScript(tempRoot)

  const env = {
    FAKE_GH_STATE: statePath,
    PATH: `${tempRoot}:${process.env.PATH}`,
  }

  const applyRun = runApply(tempRoot, ['--apply'], env)
  assert.equal(applyRun.status, 0, `${applyRun.stdout}\n${applyRun.stderr}`)

  const results = readJson(path.join(applyRun.outputDir, 'project-mutation-results.json'))
  assert.equal(results.status, 'ok')
  assert.equal(results.rollup.applied, 35)
  assert.equal(results.rollup.failed, 0)
  assert.equal(results.final_live_capture.rollup.total_items, plan.rollup.desired_project_items)
  assert.equal(results.canonical_mapping_results.hyperpush_8_to_mesh_lang_19.destination_board_membership, 'present')
})
