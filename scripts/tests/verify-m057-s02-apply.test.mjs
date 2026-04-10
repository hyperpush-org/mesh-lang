import test from 'node:test'
import assert from 'node:assert/strict'
import fs from 'node:fs'
import os from 'node:os'
import path from 'node:path'
import { spawnSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'

const scriptDir = path.dirname(fileURLToPath(import.meta.url))
const root = path.resolve(scriptDir, '..', '..')
const outputDirRelative = '.gsd/milestones/M057/slices/S02'
const planRelative = `${outputDirRelative}/repo-mutation-plan.json`
const resultsRelative = `${outputDirRelative}/repo-mutation-results.json`
const applyScript = path.join(root, 'scripts/lib/m057_repo_mutation_apply.py')
const planPath = path.join(root, planRelative)
const meshSnapshotPath = path.join(root, '.gsd/milestones/M057/slices/S01/mesh-lang-issues.snapshot.json')
const hyperpushSnapshotPath = path.join(root, '.gsd/milestones/M057/slices/S01/hyperpush-issues.snapshot.json')

const pythonResult = spawnSync('python3', ['-c', 'import sys; print(sys.executable)'], { encoding: 'utf8' })
assert.equal(pythonResult.status, 0, pythonResult.stderr)
const PYTHON = pythonResult.stdout.trim() || 'python3'

function mkTmpDir(t, prefix) {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), prefix))
  t.after(() => fs.rmSync(dir, { recursive: true, force: true }))
  return dir
}

function writeJson(filePath, payload) {
  fs.mkdirSync(path.dirname(filePath), { recursive: true })
  fs.writeFileSync(filePath, `${JSON.stringify(payload, null, 2)}\n`)
}

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, 'utf8'))
}

function copyPlanTo(tempRoot) {
  const targetPath = path.join(tempRoot, planRelative)
  fs.mkdirSync(path.dirname(targetPath), { recursive: true })
  fs.copyFileSync(planPath, targetPath)
  return targetPath
}

function snapshotIssues(snapshotPath) {
  return readJson(snapshotPath).issues
}

function buildFakeStateFromPlan() {
  const plan = readJson(planPath)
  const meshSnapshot = snapshotIssues(meshSnapshotPath)
  const hyperpushSnapshot = snapshotIssues(hyperpushSnapshotPath)
  const nextNumberByRepo = {
    'hyperpush-org/mesh-lang': Math.max(...meshSnapshot.map((issue) => issue.number)) + 1,
    'hyperpush-org/hyperpush': Math.max(...hyperpushSnapshot.map((issue) => issue.number)) + 1,
  }

  const state = {
    nextNumberByRepo,
    redirects: {},
    labelsByRepo: {
      'hyperpush-org/mesh-lang': ['bug', 'documentation', 'enhancement', 'epic', 'roadmap'],
      'hyperpush-org/hyperpush': ['bug', 'documentation', 'priority: low', 'enhancement', 'epic', 'roadmap'],
    },
    repos: {
      'hyperpush-org/mesh-lang': { issues: {} },
      'hyperpush-org/hyperpush': { issues: {} },
    },
  }

  for (const operation of plan.operations) {
    if (!operation.canonical_issue_handle) continue
    const before = operation.identity.before
    const issue = {
      repo: before.repo_slug,
      number: operation.issue_number,
      title: operation.title.before,
      body: operation.body.before,
      labels: [...operation.labels],
      state: operation.operation_kind === 'close' ? 'OPEN' : 'OPEN',
      closed_at: null,
      state_reason: null,
      comments: [],
      repository_url: `https://api.github.com/repos/${before.repo_slug}`,
      url: `https://api.github.com/repos/${before.repo_slug}/issues/${operation.issue_number}`,
      html_url: before.issue_url,
    }
    state.repos[before.repo_slug].issues[String(operation.issue_number)] = issue
  }

  return state
}

function buildFakeGhScript(tempRoot) {
  const ghPath = path.join(tempRoot, 'gh')
  const script = String.raw`#!/usr/bin/env node
const fs = require('fs')
const path = require('path')

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

function repoIssueHandle(repo, number) {
  return repo.split('/').at(-1) + '#' + number
}

function findIssue(state, repo, number) {
  return state.repos[repo]?.issues?.[String(number)] || null
}

function issueResponse(issue) {
  return {
    url: issue.url,
    repository_url: issue.repository_url,
    html_url: issue.html_url,
    number: issue.number,
    title: issue.title,
    body: issue.body,
    state: issue.state.toLowerCase(),
    labels: issue.labels.map((name, index) => ({ id: index + 1, name })),
    closed_at: issue.closed_at,
    state_reason: issue.state_reason,
  }
}

function printJson(payload) {
  process.stdout.write(JSON.stringify(payload))
}

function printInclude(statusCode, body, headers = {}) {
  const statusText = statusCode === 200 ? 'OK' : statusCode === 301 ? 'Moved Permanently' : statusCode === 404 ? 'Not Found' : 'Error'
  process.stdout.write('HTTP/2.0 ' + statusCode + ' ' + statusText + '\n')
  for (const [key, value] of Object.entries(headers)) {
    process.stdout.write(key + ': ' + value + '\n')
  }
  process.stdout.write('\n')
  process.stdout.write(JSON.stringify(body))
}

function readInput(fileFlagValue) {
  if (fileFlagValue === '-') {
    return fs.readFileSync(0, 'utf8')
  }
  return fs.readFileSync(fileFlagValue, 'utf8')
}

function parseApiArgs(args) {
  let include = false
  let method = 'GET'
  let endpoint = null
  let input = null
  for (let i = 0; i < args.length; i += 1) {
    const arg = args[i]
    if (arg === '-i' || arg === '--include') {
      include = true
      continue
    }
    if (arg === '--method' || arg === '-X') {
      method = args[++i]
      continue
    }
    if (arg === '--input') {
      input = readInput(args[++i])
      continue
    }
    if (arg === '-H' || arg === '--header') {
      i += 1
      continue
    }
    if (arg.startsWith('-')) {
      continue
    }
    if (endpoint === null) {
      endpoint = arg
    }
  }
  return { include, method: method.toUpperCase(), endpoint, input }
}

function parseIssueListArgs(args) {
  let repo = null
  let jsonFields = null
  let search = ''
  for (let i = 0; i < args.length; i += 1) {
    const arg = args[i]
    if (arg === '-R' || arg === '--repo') {
      repo = args[++i]
      continue
    }
    if (arg === '--json') {
      jsonFields = args[++i]
      continue
    }
    if (arg === '--search') {
      search = args[++i]
      continue
    }
  }
  return { repo, jsonFields, search }
}

function exactTitleFromSearch(search) {
  const match = search.match(/^"([\s\S]+)"\s+in:title$/)
  return match ? match[1] : null
}

function handleIssueList(state, args) {
  const { repo, search } = parseIssueListArgs(args)
  const exactTitle = exactTitleFromSearch(search)
  const issues = Object.values(state.repos[repo]?.issues || {})
    .filter((issue) => exactTitle === null || issue.title === exactTitle)
    .sort((a, b) => a.number - b.number)
    .map((issue) => ({
      number: issue.number,
      title: issue.title,
      state: issue.state,
      url: issue.html_url,
      closedAt: issue.closed_at,
    }))
  printJson(issues)
}

function handleIssueTransfer(state, args) {
  const number = Number(args[0])
  const destinationRepo = args[1]
  let sourceRepo = null
  for (let i = 2; i < args.length; i += 1) {
    if (args[i] === '-R' || args[i] === '--repo') {
      sourceRepo = args[i + 1]
      i += 1
    }
  }
  const issue = findIssue(state, sourceRepo, number)
  if (!issue) {
    console.error('issue not found for transfer')
    process.exit(1)
  }
  const destinationNumber = state.nextNumberByRepo[destinationRepo]++
  delete state.repos[sourceRepo].issues[String(number)]
  const transferred = {
    ...issue,
    repo: destinationRepo,
    number: destinationNumber,
    repository_url: 'https://api.github.com/repos/' + destinationRepo,
    url: 'https://api.github.com/repos/' + destinationRepo + '/issues/' + destinationNumber,
    html_url: 'https://github.com/' + destinationRepo + '/issues/' + destinationNumber,
  }
  state.repos[destinationRepo].issues[String(destinationNumber)] = transferred
  state.redirects[repoIssueHandle(sourceRepo, number)] = transferred.url
  saveState(state)
}

function handleApi(state, args) {
  const { include, method, endpoint, input } = parseApiArgs(args)
  if (!endpoint) {
    console.error('missing endpoint')
    process.exit(1)
  }
  const normalized = endpoint.startsWith('/') ? endpoint : '/' + endpoint
  const url = new URL('https://api.github.test' + normalized)
  const parts = url.pathname.split('/').filter(Boolean)

  if (parts.length === 4 && parts[0] === 'repos' && parts[3] === 'labels' && method === 'GET') {
    const repo = parts[1] + '/' + parts[2]
    const labels = (state.labelsByRepo[repo] || []).map((name, index) => ({
      id: index + 1,
      name,
      color: 'cccccc',
      description: null,
    }))
    printJson(labels)
    return
  }

  if (parts.length === 5 && parts[0] === 'repos' && parts[3] === 'issues' && method === 'GET') {
    const repo = parts[1] + '/' + parts[2]
    const number = Number(parts[4])
    const redirect = state.redirects[repoIssueHandle(repo, number)]
    if (redirect) {
      const redirectMatch = redirect.match(/repos\/([^/]+)\/([^/]+)\/issues\/(\d+)$/)
      if (!redirectMatch) {
        printInclude(404, { message: 'Not Found' })
        process.exit(1)
      }
      const redirectedRepo = redirectMatch[1] + '/' + redirectMatch[2]
      const redirectedNumber = Number(redirectMatch[3])
      const redirectedIssue = findIssue(state, redirectedRepo, redirectedNumber)
      if (!redirectedIssue) {
        printInclude(404, { message: 'Not Found' })
        process.exit(1)
      }
      if (include) {
        printInclude(200, issueResponse(redirectedIssue))
      } else {
        printJson(issueResponse(redirectedIssue))
      }
      return
    }
    const issue = findIssue(state, repo, number)
    if (!issue) {
      printInclude(404, { message: 'Not Found' })
      process.exit(1)
    }
    if (include) {
      printInclude(200, issueResponse(issue))
    } else {
      printJson(issueResponse(issue))
    }
    return
  }

  if (parts.length === 6 && parts[0] === 'repos' && parts[3] === 'issues' && parts[5] === 'comments' && method === 'GET') {
    const repo = parts[1] + '/' + parts[2]
    const number = Number(parts[4])
    const issue = findIssue(state, repo, number)
    if (!issue) {
      printInclude(404, { message: 'Not Found' })
      process.exit(1)
    }
    printJson(issue.comments)
    return
  }

  if (parts.length === 4 && parts[0] === 'repos' && parts[3] === 'issues' && method === 'POST') {
    const repo = parts[1] + '/' + parts[2]
    const payload = JSON.parse(input || '{}')
    const number = state.nextNumberByRepo[repo]++
    const issue = {
      repo,
      number,
      title: payload.title,
      body: payload.body,
      labels: Array.isArray(payload.labels) ? [...payload.labels] : [],
      state: 'OPEN',
      closed_at: null,
      state_reason: null,
      comments: [],
      repository_url: 'https://api.github.com/repos/' + repo,
      url: 'https://api.github.com/repos/' + repo + '/issues/' + number,
      html_url: 'https://github.com/' + repo + '/issues/' + number,
    }
    state.repos[repo].issues[String(number)] = issue
    saveState(state)
    printJson(issueResponse(issue))
    return
  }

  if (parts.length === 5 && parts[0] === 'repos' && parts[3] === 'issues' && method === 'PATCH') {
    const repo = parts[1] + '/' + parts[2]
    const number = Number(parts[4])
    const issue = findIssue(state, repo, number)
    if (!issue) {
      printInclude(404, { message: 'Not Found' })
      process.exit(1)
    }
    const payload = JSON.parse(input || '{}')
    if (Object.prototype.hasOwnProperty.call(payload, 'title')) issue.title = payload.title
    if (Object.prototype.hasOwnProperty.call(payload, 'body')) issue.body = payload.body
    if (Object.prototype.hasOwnProperty.call(payload, 'labels')) issue.labels = [...payload.labels]
    if (payload.state === 'closed') {
      issue.state = 'CLOSED'
      issue.closed_at = '2026-04-10T00:00:00Z'
      issue.state_reason = payload.state_reason || 'completed'
    }
    saveState(state)
    printJson(issueResponse(issue))
    return
  }

  if (parts.length === 6 && parts[0] === 'repos' && parts[3] === 'issues' && parts[5] === 'comments' && method === 'POST') {
    const repo = parts[1] + '/' + parts[2]
    const number = Number(parts[4])
    const issue = findIssue(state, repo, number)
    if (!issue) {
      printInclude(404, { message: 'Not Found' })
      process.exit(1)
    }
    const payload = JSON.parse(input || '{}')
    const comment = {
      id: issue.comments.length + 1,
      html_url: issue.html_url + '#issuecomment-' + (issue.comments.length + 1),
      body: payload.body,
      created_at: '2026-04-10T00:00:00Z',
      updated_at: '2026-04-10T00:00:00Z',
    }
    issue.comments.push(comment)
    saveState(state)
    printJson(comment)
    return
  }

  console.error('unsupported fake gh api endpoint: ' + method + ' ' + normalized)
  process.exit(1)
}

const state = loadState()
const args = process.argv.slice(2)
if (args[0] === 'api') {
  handleApi(state, args.slice(1))
} else if (args[0] === 'issue' && args[1] === 'transfer') {
  handleIssueTransfer(state, args.slice(2))
} else if (args[0] === 'issue' && args[1] === 'list') {
  handleIssueList(state, args.slice(2))
} else {
  console.error('unsupported fake gh command: ' + args.join(' '))
  process.exit(1)
}
`
  fs.writeFileSync(ghPath, script, { mode: 0o755 })
  return ghPath
}

function runApply(tempRoot, extraArgs = [], extraEnv = {}) {
  const outputDir = path.join(tempRoot, outputDirRelative)
  fs.mkdirSync(outputDir, { recursive: true })
  const result = spawnSync(
    PYTHON,
    [applyScript, '--output-dir', outputDir, ...extraArgs],
    {
      cwd: root,
      encoding: 'utf8',
      env: { ...process.env, ...extraEnv },
    },
  )
  return { ...result, outputDir }
}

test('apply script fails closed when the checked plan file is malformed before precheck', (t) => {
  const tempRoot = mkTmpDir(t, 'm057-s02-apply-malformed-')
  const tempPlanPath = copyPlanTo(tempRoot)
  const plan = readJson(tempPlanPath)
  plan.operations[0].body.after = null
  writeJson(tempPlanPath, plan)

  const result = runApply(tempRoot, ['--check'])
  assert.notEqual(result.status, 0)
  assert.match(result.stderr, /title\.after|body\.after|must be a non-empty string/i)
})

test('apply script records transfer/create outcomes and reruns idempotently against a fake gh backend', (t) => {
  const tempRoot = mkTmpDir(t, 'm057-s02-apply-fake-')
  copyPlanTo(tempRoot)

  const statePath = path.join(tempRoot, 'fake-gh-state.json')
  writeJson(statePath, buildFakeStateFromPlan())
  buildFakeGhScript(tempRoot)
  const env = {
    FAKE_GH_STATE: statePath,
    PATH: `${tempRoot}:${process.env.PATH}`,
  }

  const firstRun = runApply(tempRoot, ['--apply'], env)
  assert.equal(firstRun.status, 0, `${firstRun.stdout}\n${firstRun.stderr}`)

  const results = readJson(path.join(firstRun.outputDir, 'repo-mutation-results.json'))
  assert.equal(results.status, 'ok')
  assert.equal(results.rollup.total, 43)
  assert.equal(results.rollup.applied, 43)
  assert.equal(results.rollup.already_satisfied, 0)

  const transfer = results.operations.find((operation) => operation.operation_id === 'transfer-hyperpush-8')
  const create = results.operations.find((operation) => operation.operation_id === 'create-pitch-retrospective-issue')
  assert.equal(transfer.status, 'applied')
  assert.deepEqual(transfer.label_resolution.assignable, ['bug', 'documentation'])
  assert.deepEqual(transfer.label_resolution.unavailable, ['priority: low'])
  assert.equal(transfer.final_state.repo_slug, 'hyperpush-org/mesh-lang')
  assert.match(transfer.final_state.issue_handle, /^mesh-lang#\d+$/)
  assert.equal(create.status, 'applied')
  assert.deepEqual(create.label_resolution.unavailable, [])
  assert.equal(create.final_state.repo_slug, 'hyperpush-org/hyperpush')
  assert.equal(create.final_state.state, 'CLOSED')

  const fakeState = readJson(statePath)
  const redirectEntries = Object.entries(fakeState.redirects)
  assert.equal(redirectEntries.length, 1)
  assert.equal(redirectEntries[0][0], 'hyperpush#8')
  const transferredNumber = Number(redirectEntries[0][1].match(/issues\/(\d+)$/)[1])
  const transferredIssue = fakeState.repos['hyperpush-org/mesh-lang'].issues[String(transferredNumber)]
  assert.equal(transferredIssue.title, '[Bug]: docs Packages nav link points to /packages instead of opening packages.meshlang.dev in a new tab')
  assert.deepEqual([...transferredIssue.labels].sort(), ['bug', 'documentation'])

  const pitchIssue = Object.values(fakeState.repos['hyperpush-org/hyperpush'].issues).find(
    (issue) => issue.title === '[Feature]: record shipped /pitch evaluator route explicitly',
  )
  assert.ok(pitchIssue)
  assert.equal(pitchIssue.state, 'CLOSED')
  assert.equal(pitchIssue.comments.length, 1)
  assert.match(pitchIssue.comments[0].body, /already shipped during M056/)

  const secondRun = runApply(tempRoot, ['--apply'], env)
  assert.equal(secondRun.status, 0, `${secondRun.stdout}\n${secondRun.stderr}`)

  const rerunResults = readJson(path.join(secondRun.outputDir, 'repo-mutation-results.json'))
  assert.equal(rerunResults.status, 'ok')
  assert.equal(rerunResults.rollup.total, 43)
  assert.equal(rerunResults.rollup.applied, 0)
  assert.equal(rerunResults.rollup.already_satisfied, 43)

  const rerunTransfer = rerunResults.operations.find((operation) => operation.operation_id === 'transfer-hyperpush-8')
  const rerunCreate = rerunResults.operations.find((operation) => operation.operation_id === 'create-pitch-retrospective-issue')
  assert.equal(rerunTransfer.status, 'already_satisfied')
  assert.equal(rerunCreate.status, 'already_satisfied')
})
