import test from 'node:test'
import assert from 'node:assert/strict'
import fs from 'node:fs'
import os from 'node:os'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const scriptDir = path.dirname(fileURLToPath(import.meta.url))
const root = path.resolve(scriptDir, '..', '..')

const filePaths = {
  repoIdentity: 'scripts/lib/repo-identity.json',
  readme: 'README.md',
  gettingStarted: 'website/docs/docs/getting-started/index.md',
  clusteredExample: 'website/docs/docs/getting-started/clustered-example/index.md',
  tooling: 'website/docs/docs/tooling/index.md',
}

const staleLocalProductMarkers = [
  'mesher/README.md',
  'bash scripts/verify-m051-s01.sh',
  'bash scripts/verify-m051-s02.sh',
  'reference-backend/README.md',
]

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

function copyRepoFile(baseRoot, relativePath) {
  writeTo(baseRoot, relativePath, readFrom(root, relativePath))
}

function mkTmpDir(t, prefix) {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), prefix))
  t.after(() => fs.rmSync(dir, { recursive: true, force: true }))
  return dir
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

function loadRepoIdentity(baseRoot) {
  return JSON.parse(readFrom(baseRoot, filePaths.repoIdentity))
}

function repoDerivedMarkers(baseRoot) {
  const repoIdentity = loadRepoIdentity(baseRoot)
  const productRepoUrl = repoIdentity.productRepo?.repoUrl ?? ''
  const productHandoffLabel = repoIdentity.productHandoff?.label ?? ''
  const productRunbookUrl = `${repoIdentity.productRepo?.blobBaseUrl ?? ''}${repoIdentity.productHandoff?.relativeRunbookPath ?? ''}`
  const proofUrl = `${repoIdentity.languageRepo?.docsRoot ?? ''}production-backend-proof/`
  const sqliteStarterUrl = `${repoIdentity.languageRepo?.blobBaseUrl ?? ''}examples/todo-sqlite/README.md`
  const postgresStarterUrl = `${repoIdentity.languageRepo?.blobBaseUrl ?? ''}examples/todo-postgres/README.md`
  return {
    repoIdentity,
    productRepoUrl,
    productHandoffLabel,
    productRunbookUrl,
    proofUrl,
    sqliteStarterUrl,
    postgresStarterUrl,
  }
}

function validateFirstContactContract(baseRoot) {
  const errors = []
  const {
    productRepoUrl,
    productHandoffLabel,
    productRunbookUrl,
    proofUrl,
    sqliteStarterUrl,
    postgresStarterUrl,
  } = repoDerivedMarkers(baseRoot)

  const readme = readFrom(baseRoot, filePaths.readme)
  const gettingStarted = readFrom(baseRoot, filePaths.gettingStarted)
  const clusteredExample = readFrom(baseRoot, filePaths.clusteredExample)
  const tooling = readFrom(baseRoot, filePaths.tooling)

  const readmeLadderIntro = 'Keep the public ladder starter/examples-first: the scaffold and `/examples` stay ahead of maintainer proof surfaces.'
  const gettingStartedLadderIntro = 'Keep the public first-contact ladder explicit and ordered: clustered scaffold first, then the honest local SQLite starter, then the serious shared/deployable PostgreSQL starter, and only then the maintainer-facing backend proof page.'
  const clusteredExampleLadderIntro = 'Take the public follow-on ladder in order: honest local SQLite starter, serious shared/deployable PostgreSQL starter, then Production Backend Proof only when you need the maintainer-facing deeper backend proof.'
  const toolingLadderIntro = 'Keep the public CLI workflow explicit and examples-first: hello world first, then the clustered scaffold, then the honest local SQLite starter or the serious shared/deployable PostgreSQL starter, and only after that the maintainer-facing backend proof page.'

  requireIncludes(errors, filePaths.readme, readme, [
    'meshc init --clustered hello_cluster',
    'meshc init --template todo-api --db sqlite todo_api',
    'meshc init --template todo-api --db postgres shared_todo',
    readmeLadderIntro,
    proofUrl,
    sqliteStarterUrl,
    postgresStarterUrl,
    productRepoUrl,
    productHandoffLabel,
  ])

  requireIncludes(errors, filePaths.gettingStarted, gettingStarted, [
    '## Choose your next starter',
    'meshc init --clustered hello_cluster',
    'meshc init --template todo-api --db sqlite todo_api',
    'meshc init --template todo-api --db postgres shared_todo',
    gettingStartedLadderIntro,
    '/docs/production-backend-proof/',
    sqliteStarterUrl,
    postgresStarterUrl,
    productRepoUrl,
    productHandoffLabel,
  ])

  requireIncludes(errors, filePaths.clusteredExample, clusteredExample, [
    '## After the scaffold, pick the follow-on starter',
    'meshc init --template todo-api --db sqlite my_local_todo',
    'meshc init --template todo-api --db postgres my_shared_todo',
    clusteredExampleLadderIntro,
    '/docs/distributed-proof/',
    '/docs/production-backend-proof/',
    sqliteStarterUrl,
    postgresStarterUrl,
    productRepoUrl,
    productHandoffLabel,
  ])

  requireIncludes(errors, filePaths.tooling, tooling, [
    'meshc init --clustered my_clustered_app',
    'meshc init --template todo-api --db sqlite my_local_todo',
    'meshc init --template todo-api --db postgres my_shared_todo',
    toolingLadderIntro,
    '/docs/production-backend-proof/',
    sqliteStarterUrl,
    postgresStarterUrl,
    productRepoUrl,
    productHandoffLabel,
    'bash scripts/verify-m050-s02.sh',
    'bash scripts/verify-m048-s05.sh',
    'bash scripts/verify-m049-s05.sh',
  ])

  for (const [relativePath, text] of [
    [filePaths.readme, readme],
    [filePaths.gettingStarted, gettingStarted],
    [filePaths.clusteredExample, clusteredExample],
    [filePaths.tooling, tooling],
  ]) {
    requireExcludes(errors, relativePath, text, staleLocalProductMarkers)
  }

  requireOrdered(errors, filePaths.readme, readme, [
    '### 4. Choose your next starter',
    'meshc init --clustered hello_cluster',
    'meshc init --template todo-api --db sqlite todo_api',
    'meshc init --template todo-api --db postgres shared_todo',
    '## Where to go next',
    readmeLadderIntro,
    '## Maintainers / public release proof',
  ])

  requireOrdered(errors, filePaths.gettingStarted, gettingStarted, [
    '## Choose your next starter',
    'meshc init --clustered hello_cluster',
    'meshc init --template todo-api --db sqlite todo_api',
    'meshc init --template todo-api --db postgres shared_todo',
    '## What\'s Next?',
    gettingStartedLadderIntro,
    'That page is the repo-boundary handoff into the [Hyperpush product repo](https://github.com/hyperpush-org/hyperpush-mono)',
  ])

  requireOrdered(errors, filePaths.clusteredExample, clusteredExample, [
    'This page stays on that scaffold first.',
    '## Generate the scaffold',
    '## After the scaffold, pick the follow-on starter',
    clusteredExampleLadderIntro,
    'meshc init --template todo-api --db sqlite my_local_todo',
    'meshc init --template todo-api --db postgres my_shared_todo',
    '## Need the retained verifier map?',
  ])

  requireOrdered(errors, filePaths.tooling, tooling, [
    '## Package Manager',
    toolingLadderIntro,
    '### Creating a New Project',
    'meshc init --clustered my_clustered_app',
    'meshc init --template todo-api --db sqlite my_local_todo',
    'meshc init --template todo-api --db postgres my_shared_todo',
    '## Assembled first-contact docs verifier',
    'bash scripts/verify-m050-s02.sh',
  ])

  if (readme.includes(productRunbookUrl)) {
    errors.push(`${filePaths.readme} should keep the product runbook behind the repo-boundary handoff instead of linking it directly`)
  }
  if (gettingStarted.includes(productRunbookUrl)) {
    errors.push(`${filePaths.gettingStarted} should not link directly to the product runbook`)
  }
  if (clusteredExample.includes(productRunbookUrl)) {
    errors.push(`${filePaths.clusteredExample} should not link directly to the product runbook`)
  }
  if (tooling.includes(productRunbookUrl)) {
    errors.push(`${filePaths.tooling} should not link directly to the product runbook`)
  }

  return errors
}

test('current repo publishes the first-contact ladder with a repo-boundary product handoff', () => {
  const errors = validateFirstContactContract(root)
  assert.deepEqual(errors, [], errors.join('\n'))
})

test('contract fails closed when README falls back to a local mesh-lang product marker', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m050-s02-readme-')
  for (const relativePath of Object.values(filePaths)) {
    copyRepoFile(tmpRoot, relativePath)
  }

  let mutated = readFrom(tmpRoot, filePaths.readme)
  mutated = mutated.replaceAll(
    'https://github.com/hyperpush-org/hyperpush-mono',
    'https://github.com/snowdamiz/mesh-lang/blob/main/mesher/README.md',
  )
  writeTo(tmpRoot, filePaths.readme, mutated)

  const errors = validateFirstContactContract(tmpRoot)
  assert.ok(errors.some((error) => error.includes('README.md missing "https://github.com/hyperpush-org/hyperpush-mono"')), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes('README.md still contains stale text "mesher/README.md"')), errors.join('\n'))
})

test('contract fails closed when Getting Started collapses the split starters back to one generic todo template', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m050-s02-getting-started-')
  for (const relativePath of Object.values(filePaths)) {
    copyRepoFile(tmpRoot, relativePath)
  }

  let mutated = readFrom(tmpRoot, filePaths.gettingStarted)
  mutated = mutated.replace('meshc init --template todo-api --db sqlite todo_api', 'meshc init --template todo-api todo_api')
  mutated = mutated.replace('meshc init --template todo-api --db postgres shared_todo', 'meshc init --template todo-api shared_todo')
  writeTo(tmpRoot, filePaths.gettingStarted, mutated)

  const errors = validateFirstContactContract(tmpRoot)
  assert.ok(errors.some((error) => error.includes('website/docs/docs/getting-started/index.md missing "meshc init --template todo-api --db sqlite todo_api"')), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes('website/docs/docs/getting-started/index.md missing "meshc init --template todo-api --db postgres shared_todo"')), errors.join('\n'))
})

test('contract fails closed when Tooling turns the product handoff back into a first-contact local runbook', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m050-s02-tooling-')
  for (const relativePath of Object.values(filePaths)) {
    copyRepoFile(tmpRoot, relativePath)
  }

  let mutated = readFrom(tmpRoot, filePaths.tooling)
  mutated = mutated.replace(
    'https://github.com/hyperpush-org/hyperpush-mono',
    'https://github.com/snowdamiz/mesh-lang/blob/main/mesher/README.md',
  )
  mutated = mutated.replace(
    'Keep the deeper proof commands behind Production Backend Proof and Distributed Proof, and keep the product-owned runbook on the far side of the Hyperpush repo handoff instead of turning this first-contact tooling page into a verifier runbook:',
    'Keep the deeper proof commands nearby in `mesher/README.md`, `bash scripts/verify-m051-s01.sh`, and `bash scripts/verify-m051-s02.sh` instead of leaving them behind a repo boundary:',
  )
  writeTo(tmpRoot, filePaths.tooling, mutated)

  const errors = validateFirstContactContract(tmpRoot)
  assert.ok(errors.some((error) => error.includes('website/docs/docs/tooling/index.md missing "https://github.com/hyperpush-org/hyperpush-mono"')), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes('website/docs/docs/tooling/index.md still contains stale text "mesher/README.md"')), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes('website/docs/docs/tooling/index.md still contains stale text "bash scripts/verify-m051-s01.sh"')), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes('website/docs/docs/tooling/index.md still contains stale text "bash scripts/verify-m051-s02.sh"')), errors.join('\n'))
})
