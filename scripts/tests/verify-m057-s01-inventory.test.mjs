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
  script: 'scripts/lib/m057_tracker_inventory.py',
  query: 'scripts/lib/m057_project_items.graphql',
  meshSnapshot: '.gsd/milestones/M057/slices/S01/mesh-lang-issues.snapshot.json',
  hyperpushSnapshot: '.gsd/milestones/M057/slices/S01/hyperpush-issues.snapshot.json',
  projectItemsSnapshot: '.gsd/milestones/M057/slices/S01/project-items.snapshot.json',
  projectFieldsSnapshot: '.gsd/milestones/M057/slices/S01/project-fields.snapshot.json',
  ledgerTest: 'scripts/tests/verify-m057-s01-ledger.test.mjs',
}

const expected = {
  meshLangIssues: 16,
  hyperpushIssues: 52,
  combinedIssues: 68,
  projectItems: 63,
  projectFields: 18,
  projectMeshLang: 16,
  projectHyperpush: 47,
  trackedFieldKeys: [
    'commitment',
    'delivery_mode',
    'domain',
    'hackathon_phase',
    'priority',
    'start_date',
    'status',
    'target_date',
    'title',
    'track',
  ],
  nonProjectHyperpush: [2, 3, 4, 5, 8],
}

function readJson(relativePath) {
  return JSON.parse(fs.readFileSync(path.join(root, relativePath), 'utf8'))
}

function writeTo(baseRoot, relativePath, content) {
  const absolutePath = path.join(baseRoot, relativePath)
  fs.mkdirSync(path.dirname(absolutePath), { recursive: true })
  fs.writeFileSync(absolutePath, content)
}

function copyRepoFile(baseRoot, relativePath) {
  writeTo(baseRoot, relativePath, fs.readFileSync(path.join(root, relativePath), 'utf8'))
}

function mkTmpDir(t, prefix) {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), prefix))
  t.after(() => fs.rmSync(dir, { recursive: true, force: true }))
  return dir
}

function runInventory(baseRoot, args, env = {}) {
  return spawnSync(PYTHON, [path.join(baseRoot, files.script), ...args], {
    cwd: baseRoot,
    encoding: 'utf8',
    env: {
      ...process.env,
      ...env,
    },
  })
}

function baseFakeState() {
  return {
    repoView: {
      'hyperpush-org/mesh-lang': {
        nameWithOwner: 'hyperpush-org/mesh-lang',
        url: 'https://github.com/hyperpush-org/mesh-lang',
      },
      'hyperpush-org/hyperpush': {
        nameWithOwner: 'hyperpush-org/hyperpush',
        url: 'https://github.com/hyperpush-org/hyperpush',
      },
      'hyperpush-org/hyperpush-mono': {
        nameWithOwner: 'hyperpush-org/hyperpush',
        url: 'https://github.com/hyperpush-org/hyperpush',
      },
    },
    issueLists: {
      'hyperpush-org/mesh-lang': [
        {
          number: 1,
          title: 'Mesh tracker item',
          state: 'OPEN',
          labels: [
            {
              id: 'mesh-roadmap',
              name: 'roadmap',
              description: 'Launch roadmap work',
              color: '0E8A16',
            },
          ],
          body: 'Mesh issue body',
          url: 'https://github.com/hyperpush-org/mesh-lang/issues/1',
          createdAt: '2026-04-10T00:00:00Z',
          updatedAt: '2026-04-10T00:00:00Z',
          closedAt: null,
        },
      ],
      'hyperpush-org/hyperpush': [
        {
          number: 8,
          title: 'Hyperpush tracker item',
          state: 'OPEN',
          labels: [
            {
              id: 'hyperpush-roadmap',
              name: 'roadmap',
              description: 'Launch roadmap work',
              color: '0E8A16',
            },
          ],
          body: 'Hyperpush issue body',
          url: 'https://github.com/hyperpush-org/hyperpush/issues/8',
          createdAt: '2026-04-10T00:00:00Z',
          updatedAt: '2026-04-10T00:00:00Z',
          closedAt: null,
        },
      ],
    },
    projectFields: {
      totalCount: 10,
      fields: [
        { id: 'field-title', name: 'Title', type: 'ProjectV2Field' },
        {
          id: 'field-status',
          name: 'Status',
          type: 'ProjectV2SingleSelectField',
          options: [{ id: 'status-todo', name: 'Todo' }],
        },
        {
          id: 'field-domain',
          name: 'Domain',
          type: 'ProjectV2SingleSelectField',
          options: [{ id: 'domain-mesh', name: 'Mesh' }],
        },
        {
          id: 'field-track',
          name: 'Track',
          type: 'ProjectV2SingleSelectField',
          options: [{ id: 'track-foundation', name: 'Mesh Foundation' }],
        },
        {
          id: 'field-commitment',
          name: 'Commitment',
          type: 'ProjectV2SingleSelectField',
          options: [{ id: 'commitment-committed', name: 'Committed' }],
        },
        {
          id: 'field-delivery-mode',
          name: 'Delivery Mode',
          type: 'ProjectV2SingleSelectField',
          options: [{ id: 'delivery-shared', name: 'Shared' }],
        },
        {
          id: 'field-priority',
          name: 'Priority',
          type: 'ProjectV2SingleSelectField',
          options: [{ id: 'priority-p0', name: 'P0' }],
        },
        { id: 'field-start-date', name: 'Start date', type: 'ProjectV2Field' },
        { id: 'field-target-date', name: 'Target date', type: 'ProjectV2Field' },
        {
          id: 'field-hackathon-phase',
          name: 'Hackathon Phase',
          type: 'ProjectV2SingleSelectField',
          options: [{ id: 'phase-1', name: 'Phase 1 — Foundation' }],
        },
      ],
    },
    graphqlPages: {
      __first__: {
        data: {
          organization: {
            projectV2: {
              id: 'PVT_project_1',
              title: 'Hyperpush Launch Roadmap',
              url: 'https://github.com/orgs/hyperpush-org/projects/1',
              items: {
                totalCount: 1,
                pageInfo: {
                  hasNextPage: false,
                  endCursor: null,
                },
                nodes: [
                  {
                    id: 'PVTI_item_1',
                    content: {
                      __typename: 'Issue',
                      number: 1,
                      title: 'Mesh tracker item',
                      state: 'OPEN',
                      url: 'https://github.com/hyperpush-org/mesh-lang/issues/1',
                      repository: {
                        nameWithOwner: 'hyperpush-org/mesh-lang',
                        url: 'https://github.com/hyperpush-org/mesh-lang',
                      },
                    },
                    fieldValues: {
                      pageInfo: {
                        hasNextPage: false,
                        endCursor: null,
                      },
                      nodes: [
                        {
                          __typename: 'ProjectV2ItemFieldTextValue',
                          text: 'Mesh tracker item',
                          field: { id: 'field-title', name: 'Title' },
                        },
                        {
                          __typename: 'ProjectV2ItemFieldSingleSelectValue',
                          name: 'Todo',
                          optionId: 'status-todo',
                          field: { id: 'field-status', name: 'Status' },
                        },
                        {
                          __typename: 'ProjectV2ItemFieldSingleSelectValue',
                          name: 'Mesh',
                          optionId: 'domain-mesh',
                          field: { id: 'field-domain', name: 'Domain' },
                        },
                        {
                          __typename: 'ProjectV2ItemFieldDateValue',
                          date: '2026-04-08',
                          field: { id: 'field-start-date', name: 'Start date' },
                        },
                      ],
                    },
                  },
                ],
              },
            },
          },
        },
      },
    },
  }
}

function fakeStateForMode(mode) {
  const state = baseFakeState()
  if (mode === 'canonical-drift') {
    state.repoView['hyperpush-org/hyperpush-mono'] = {
      nameWithOwner: 'hyperpush-org/hyperpush-mono',
      url: 'https://github.com/hyperpush-org/hyperpush-mono',
    }
    return state
  }
  if (mode === 'missing-labels') {
    delete state.issueLists['hyperpush-org/mesh-lang'][0].labels
    return state
  }
  if (mode === 'missing-closedAt') {
    delete state.issueLists['hyperpush-org/mesh-lang'][0].closedAt
    return state
  }
  if (mode === 'graphql-error') {
    state.graphqlPages.__first__ = {
      errors: [{ message: 'API rate limit exceeded' }],
    }
    return state
  }
  if (mode === 'blank-fields') {
    state.graphqlPages.__first__.data.organization.projectV2.items.nodes[0].fieldValues.nodes = [
      {
        __typename: 'ProjectV2ItemFieldTextValue',
        text: 'Mesh tracker item',
        field: { id: 'field-title', name: 'Title' },
      },
      {
        __typename: 'ProjectV2ItemFieldSingleSelectValue',
        name: 'Todo',
        optionId: 'status-todo',
        field: { id: 'field-status', name: 'Status' },
      },
      {
        __typename: 'ProjectV2ItemFieldSingleSelectValue',
        name: '',
        optionId: null,
        field: { id: 'field-domain', name: 'Domain' },
      },
      {
        __typename: 'ProjectV2ItemFieldDateValue',
        date: null,
        field: { id: 'field-target-date', name: 'Target date' },
      },
    ]
    return state
  }
  throw new Error(`unknown fake gh mode: ${mode}`)
}

function createFakeGhRoot(t, mode) {
  const tmpRoot = mkTmpDir(t, `m057-s01-${mode}-`)
  copyRepoFile(tmpRoot, files.script)
  copyRepoFile(tmpRoot, files.query)

  const statePath = path.join(tmpRoot, 'fake-gh-state.json')
  fs.writeFileSync(statePath, `${JSON.stringify(fakeStateForMode(mode), null, 2)}\n`)

  const binDir = path.join(tmpRoot, 'bin')
  fs.mkdirSync(binDir, { recursive: true })
  const fakeGhPath = path.join(binDir, 'gh')
  fs.writeFileSync(
    fakeGhPath,
    `#!/usr/bin/env python3
import json
import os
import sys
from pathlib import Path

state = json.loads(Path(os.environ['FAKE_GH_STATE']).read_text())
args = sys.argv[1:]

if args[:2] == ['repo', 'view']:
    repo = args[2]
    payload = state['repoView'].get(repo)
    if payload is None:
        print(f'unhandled repo view for {repo}', file=sys.stderr)
        sys.exit(2)
    print(json.dumps(payload))
    sys.exit(0)

if args[:2] == ['issue', 'list']:
    repo = args[args.index('-R') + 1]
    payload = state['issueLists'].get(repo)
    if payload is None:
        print(f'unhandled issue list for {repo}', file=sys.stderr)
        sys.exit(2)
    print(json.dumps(payload))
    sys.exit(0)

if args[:2] == ['project', 'field-list']:
    print(json.dumps(state['projectFields']))
    sys.exit(0)

if args[:2] == ['api', 'graphql']:
    after = '__first__'
    for index, arg in enumerate(args):
        if arg in ('-F', '-f') and index + 1 < len(args):
            field = args[index + 1]
            if field.startswith('after='):
                after = field.split('=', 1)[1]
    payload = state['graphqlPages'].get(after)
    if payload is None:
        print(f'unhandled graphql page for {after}', file=sys.stderr)
        sys.exit(2)
    print(json.dumps(payload))
    sys.exit(0)

print('unhandled gh args: ' + ' '.join(args), file=sys.stderr)
sys.exit(2)
`,
    'utf8',
  )
  fs.chmodSync(fakeGhPath, 0o755)

  return {
    tmpRoot,
    outputDir: path.join(tmpRoot, 'out'),
    env: {
      FAKE_GH_STATE: statePath,
      PATH: `${binDir}:${process.env.PATH}`,
    },
  }
}

test('current repo publishes the M057 S01 live inventory snapshots with canonical repo identity, tracked field keys, and research counts', () => {
  const meshSnapshot = readJson(files.meshSnapshot)
  const hyperpushSnapshot = readJson(files.hyperpushSnapshot)
  const projectItemsSnapshot = readJson(files.projectItemsSnapshot)
  const projectFieldsSnapshot = readJson(files.projectFieldsSnapshot)

  assert.equal(meshSnapshot.version, 'm057-s01-mesh-lang-issues-v1')
  assert.equal(hyperpushSnapshot.version, 'm057-s01-hyperpush-issues-v1')
  assert.equal(projectItemsSnapshot.version, 'm057-s01-project-items-v1')
  assert.equal(projectFieldsSnapshot.version, 'm057-s01-project-fields-v1')

  assert.equal(meshSnapshot.issues.length, expected.meshLangIssues)
  assert.equal(hyperpushSnapshot.issues.length, expected.hyperpushIssues)
  assert.equal(meshSnapshot.issues.length + hyperpushSnapshot.issues.length, expected.combinedIssues)
  assert.equal(projectItemsSnapshot.items.length, expected.projectItems)
  assert.equal(projectFieldsSnapshot.fields.length, expected.projectFields)

  assert.equal(hyperpushSnapshot.repo.canonical_slug, 'hyperpush-org/hyperpush')
  assert.equal(hyperpushSnapshot.canonical_redirect.requested_repo, 'hyperpush-org/hyperpush-mono')
  assert.equal(hyperpushSnapshot.canonical_redirect.canonical_slug, 'hyperpush-org/hyperpush')

  assert.deepEqual(projectItemsSnapshot.tracked_field_keys, expected.trackedFieldKeys)
  assert.equal(projectItemsSnapshot.rollup.repo_counts['hyperpush-org/mesh-lang'], expected.projectMeshLang)
  assert.equal(projectItemsSnapshot.rollup.repo_counts['hyperpush-org/hyperpush'], expected.projectHyperpush)
  assert.equal(projectItemsSnapshot.rollup.field_presence.status, expected.projectItems)

  const repoIssueUrls = new Set([
    ...meshSnapshot.issues.map((issue) => issue.canonical_issue_url),
    ...hyperpushSnapshot.issues.map((issue) => issue.canonical_issue_url),
  ])
  assert.equal(repoIssueUrls.size, expected.combinedIssues)

  const projectItemUrls = new Set(projectItemsSnapshot.items.map((item) => item.canonical_issue_url))
  assert.equal(projectItemUrls.size, expected.projectItems)
  for (const issueUrl of projectItemUrls) {
    assert.ok(repoIssueUrls.has(issueUrl), `orphan project item: ${issueUrl}`)
  }

  const nonProjectHyperpush = hyperpushSnapshot.issues
    .filter((issue) => !projectItemUrls.has(issue.canonical_issue_url))
    .map((issue) => issue.number)
    .sort((a, b) => a - b)
  assert.deepEqual(nonProjectHyperpush, expected.nonProjectHyperpush)

  const fieldNames = new Set(projectFieldsSnapshot.fields.map((field) => field.field_name))
  for (const name of ['Status', 'Domain', 'Track', 'Commitment', 'Delivery Mode', 'Priority', 'Start date', 'Target date', 'Hackathon Phase']) {
    assert.ok(fieldNames.has(name), `missing tracked field ${name}`)
  }

  for (const item of projectItemsSnapshot.items) {
    assert.ok(item.project_item_id)
    assert.deepEqual(Object.keys(item.field_values).sort(), expected.trackedFieldKeys)
  }

  assert.ok(fs.existsSync(path.join(root, files.ledgerTest)), `${files.ledgerTest} should exist as the fail-closed downstream contract rail`)
})

test('inventory helper can re-check the committed live snapshots without refreshing', () => {
  const result = runInventory(root, ['--check', '--output-dir', path.join(root, '.gsd/milestones/M057/slices/S01')])
  assert.equal(result.status, 0, `${result.stdout}\n${result.stderr}`)
  const payload = JSON.parse(result.stdout)
  assert.equal(payload.status, 'ok')
  assert.equal(payload.mode, 'check')
  assert.equal(payload.combined_issues, expected.combinedIssues)
  assert.equal(payload.project_items, expected.projectItems)
})

test('inventory refresh fails closed when gh is unavailable', (t) => {
  const tmpRoot = mkTmpDir(t, 'm057-s01-gh-missing-')
  copyRepoFile(tmpRoot, files.script)
  copyRepoFile(tmpRoot, files.query)

  const result = runInventory(tmpRoot, ['--refresh', '--output-dir', path.join(tmpRoot, 'out')], {
    PATH: '',
  })
  assert.notEqual(result.status, 0)
  assert.match(result.stderr, /gh CLI not found on PATH/)
  assert.ok(!fs.existsSync(path.join(tmpRoot, 'out', 'mesh-lang-issues.snapshot.json')))
})

test('inventory refresh fails closed when the stale hyperpush-mono slug stops canonicalizing to hyperpush', (t) => {
  const fake = createFakeGhRoot(t, 'canonical-drift')
  const result = runInventory(fake.tmpRoot, ['--refresh', '--output-dir', fake.outputDir], fake.env)
  assert.notEqual(result.status, 0)
  assert.match(result.stderr, /hyperpush-alias-repo-view: expected canonical repo hyperpush-org\/hyperpush/)
  assert.ok(!fs.existsSync(path.join(fake.outputDir, 'mesh-lang-issues.snapshot.json')))
})

test('inventory refresh fails closed when issue payloads drop the labels array', (t) => {
  const fake = createFakeGhRoot(t, 'missing-labels')
  const result = runInventory(fake.tmpRoot, ['--refresh', '--output-dir', fake.outputDir], fake.env)
  assert.notEqual(result.status, 0)
  assert.match(result.stderr, /mesh-lang-issues\.issue\[0\]\.labels is required/)
})

test('inventory refresh fails closed when issue payloads drop closedAt', (t) => {
  const fake = createFakeGhRoot(t, 'missing-closedAt')
  const result = runInventory(fake.tmpRoot, ['--refresh', '--output-dir', fake.outputDir], fake.env)
  assert.notEqual(result.status, 0)
  assert.match(result.stderr, /mesh-lang-issues\.issue\[0\]\.closedAt is required/)
})

test('inventory refresh fails closed when GitHub GraphQL returns an explicit rate-limit or permission error', (t) => {
  const fake = createFakeGhRoot(t, 'graphql-error')
  const result = runInventory(fake.tmpRoot, ['--refresh', '--output-dir', fake.outputDir], fake.env)
  assert.notEqual(result.status, 0)
  assert.match(result.stderr, /project-items-page-1: GitHub GraphQL returned errors: API rate limit exceeded/)
  assert.ok(!fs.existsSync(path.join(fake.outputDir, 'project-items.snapshot.json')))
})

test('inventory refresh preserves stable field keys and normalizes blank project field values to null', (t) => {
  const fake = createFakeGhRoot(t, 'blank-fields')
  const result = runInventory(fake.tmpRoot, ['--refresh', '--output-dir', fake.outputDir], fake.env)
  assert.equal(result.status, 0, `${result.stdout}\n${result.stderr}`)

  const projectItemsSnapshot = JSON.parse(fs.readFileSync(path.join(fake.outputDir, 'project-items.snapshot.json'), 'utf8'))
  assert.equal(projectItemsSnapshot.items.length, 1)

  const row = projectItemsSnapshot.items[0]
  assert.deepEqual(Object.keys(row.field_values).sort(), expected.trackedFieldKeys)
  assert.equal(row.field_values.title.value, 'Mesh tracker item')
  assert.equal(row.field_values.status.value, 'Todo')
  assert.equal(row.field_values.domain.value, null)
  assert.equal(row.field_values.target_date.value, null)
  assert.equal(row.field_values.track.value, null)
  assert.equal(row.field_values.commitment.value, null)
  assert.equal(row.field_values.delivery_mode.value, null)
  assert.equal(row.field_values.priority.value, null)
  assert.equal(row.field_values.start_date.value, null)
  assert.equal(row.field_values.hackathon_phase.value, null)
})
