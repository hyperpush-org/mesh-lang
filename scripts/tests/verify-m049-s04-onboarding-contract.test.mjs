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
  scaffold: 'compiler/mesh-pkg/src/scaffold.rs',
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

function requireNoMatch(errors, relativePath, text, pattern, label) {
  if (pattern.test(text)) {
    errors.push(`${relativePath} still contains ${label}`)
  }
}

function loadRepoIdentity(baseRoot) {
  const raw = readFrom(baseRoot, filePaths.repoIdentity)
  return JSON.parse(raw)
}

function productRunbookUrl(repoIdentity) {
  const blobBaseUrl = repoIdentity.productRepo?.blobBaseUrl ?? ''
  const relativeRunbookPath = repoIdentity.productHandoff?.relativeRunbookPath ?? ''
  return `${blobBaseUrl}${relativeRunbookPath}`
}

function productionBackendProofUrl(repoIdentity) {
  return `${repoIdentity.languageRepo.docsRoot}production-backend-proof/`
}

function sqliteStarterUrl(repoIdentity) {
  return `${repoIdentity.languageRepo.blobBaseUrl}examples/todo-sqlite/README.md`
}

function postgresStarterUrl(repoIdentity) {
  return `${repoIdentity.languageRepo.blobBaseUrl}examples/todo-postgres/README.md`
}

function extractClusteredScaffoldReadme(scaffoldSource) {
  const match = scaffoldSource.match(/let readme = format!\(\s*r#"([\s\S]*?)"#,\s*name = name,/)
  assert.ok(match, 'unable to locate clustered scaffold README template in compiler/mesh-pkg/src/scaffold.rs')
  return match[1]
}

function validateOnboardingContract(baseRoot) {
  const errors = []
  const repoIdentity = loadRepoIdentity(baseRoot)
  const proofUrl = productionBackendProofUrl(repoIdentity)
  const postgresUrl = postgresStarterUrl(repoIdentity)
  const sqliteUrl = sqliteStarterUrl(repoIdentity)
  const handoffRepoUrl = repoIdentity.productRepo.repoUrl
  const handoffLabel = repoIdentity.productHandoff?.label
  const handoffRunbookPath = repoIdentity.productHandoff?.relativeRunbookPath
  const handoffRunbookUrl = productRunbookUrl(repoIdentity)

  if (handoffLabel !== 'Hyperpush product repo') {
    errors.push(`${filePaths.repoIdentity} productHandoff.label drifted from "Hyperpush product repo"`)
  }
  if (handoffRunbookPath !== 'mesher/README.md') {
    errors.push(`${filePaths.repoIdentity} productHandoff.relativeRunbookPath drifted from "mesher/README.md"`)
  }

  const readme = readFrom(baseRoot, filePaths.readme)
  const scaffoldSource = readFrom(baseRoot, filePaths.scaffold)
  const scaffoldReadme = extractClusteredScaffoldReadme(scaffoldSource)
  const gettingStarted = readFrom(baseRoot, filePaths.gettingStarted)
  const clusteredExample = readFrom(baseRoot, filePaths.clusteredExample)
  const tooling = readFrom(baseRoot, filePaths.tooling)

  requireIncludes(errors, filePaths.readme, readme, [
    'starter/examples-first',
    postgresUrl,
    sqliteUrl,
    proofUrl,
    handoffRepoUrl,
    handoffLabel,
  ])

  requireIncludes(errors, filePaths.scaffold, scaffoldSource, [
    'scaffold_public_links()',
    'scripts/lib/repo-identity.json',
    'todo_postgres_readme_url = public_links.todo_postgres_readme_url',
    'todo_sqlite_readme_url = public_links.todo_sqlite_readme_url',
    'production_backend_proof_url = public_links.production_backend_proof_url',
    'product_repo_url = public_links.product_repo_url',
    'product_handoff_label = public_links.product_handoff_label',
    'product_runbook_url = public_links.product_runbook_url',
  ])

  requireIncludes(errors, `${filePaths.scaffold} clustered README template`, scaffoldReadme, [
    '{todo_postgres_readme_url}',
    '{todo_sqlite_readme_url}',
    '{production_backend_proof_url}',
    '{product_repo_url}',
    '{product_handoff_label}',
    '{product_runbook_url}',
  ])

  requireIncludes(errors, filePaths.gettingStarted, gettingStarted, [
    '## Choose your next starter',
    'meshc init --clustered hello_cluster',
    'meshc init --template todo-api --db sqlite todo_api',
    'meshc init --template todo-api --db postgres shared_todo',
    postgresUrl,
    sqliteUrl,
    '/docs/production-backend-proof/',
    handoffRepoUrl,
    handoffLabel,
  ])

  requireIncludes(errors, filePaths.clusteredExample, clusteredExample, [
    '## After the scaffold, pick the follow-on starter',
    'meshc init --template todo-api --db sqlite my_local_todo',
    'meshc init --template todo-api --db postgres my_shared_todo',
    postgresUrl,
    sqliteUrl,
    '/docs/distributed-proof/',
    '/docs/production-backend-proof/',
    handoffRepoUrl,
    handoffLabel,
  ])

  requireIncludes(errors, filePaths.tooling, tooling, [
    'starter/examples-first',
    'meshc init --clustered my_clustered_app',
    'meshc init --template todo-api --db sqlite my_local_todo',
    'meshc init --template todo-api --db postgres my_shared_todo',
    postgresUrl,
    sqliteUrl,
    '/docs/production-backend-proof/',
    handoffRepoUrl,
    handoffLabel,
  ])

  for (const [relativePath, text] of [
    [filePaths.readme, readme],
    [`${filePaths.scaffold} clustered README template`, scaffoldReadme],
    [filePaths.gettingStarted, gettingStarted],
    [filePaths.clusteredExample, clusteredExample],
    [filePaths.tooling, tooling],
  ]) {
    requireNoMatch(errors, relativePath, text, /mesher\/README\.md(?!\]\(https:\/\/github\.com\/hyperpush-org\/hyperpush-mono\/blob\/main\/mesher\/README\.md\))/, 'local mesh-lang product runbook path')
    requireNoMatch(errors, relativePath, text, /bash scripts\/verify-m051-s0[12]\.sh/, 'local mesh-lang product verifier command')
    requireNoMatch(errors, relativePath, text, /reference-backend\/README\.md/, 'stale backend runbook handoff')
  }

  return errors
}

test('current repo publishes a repo-boundary product handoff across onboarding surfaces', () => {
  const errors = validateOnboardingContract(root)
  assert.deepEqual(errors, [], errors.join('\n'))
})

test('contract fails closed when repo identity loses the product handoff marker', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m049-s04-repo-identity-')
  for (const relativePath of Object.values(filePaths)) {
    copyRepoFile(tmpRoot, relativePath)
  }

  const repoIdentity = loadRepoIdentity(tmpRoot)
  delete repoIdentity.productHandoff.label
  delete repoIdentity.productHandoff.relativeRunbookPath
  writeTo(tmpRoot, filePaths.repoIdentity, `${JSON.stringify(repoIdentity, null, 2)}\n`)

  const errors = validateOnboardingContract(tmpRoot)
  assert.ok(errors.some((error) => error.includes('productHandoff.label drifted')), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes('productHandoff.relativeRunbookPath drifted')), errors.join('\n'))
})

test('contract fails closed when README falls back to a local mesh-lang product handoff', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m049-s04-readme-')
  for (const relativePath of Object.values(filePaths)) {
    copyRepoFile(tmpRoot, relativePath)
  }

  let mutated = readFrom(tmpRoot, filePaths.readme)
  mutated = mutated.replaceAll(
    'https://github.com/hyperpush-org/hyperpush-mono',
    'https://github.com/snowdamiz/mesh-lang/blob/main/mesher/README.md',
  )
  writeTo(tmpRoot, filePaths.readme, mutated)

  const errors = validateOnboardingContract(tmpRoot)
  assert.ok(errors.some((error) => error.includes('README.md missing "https://github.com/hyperpush-org/hyperpush-mono"')), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes('README.md still contains local mesh-lang product runbook path')), errors.join('\n'))
})

test('contract fails closed when the clustered scaffold README reintroduces local verifier commands', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m049-s04-scaffold-')
  for (const relativePath of Object.values(filePaths)) {
    copyRepoFile(tmpRoot, relativePath)
  }

  let mutated = readFrom(tmpRoot, filePaths.scaffold)
  mutated = mutated.replace(
    'Keep any product-owned runbook reading on the far side of that handoff at [{product_handoff_runbook}]({product_runbook_url}) instead of teaching local mesh-lang product paths here.',
    'Keep the local mesh-lang product proof nearby at `mesher/README.md`, `bash scripts/verify-m051-s01.sh`, and `bash scripts/verify-m051-s02.sh`.',
  )
  writeTo(tmpRoot, filePaths.scaffold, mutated)

  const errors = validateOnboardingContract(tmpRoot)
  assert.ok(errors.some((error) => error.includes('compiler/mesh-pkg/src/scaffold.rs clustered README template missing "{product_runbook_url}"')), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes('compiler/mesh-pkg/src/scaffold.rs clustered README template still contains local mesh-lang product runbook path')), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes('compiler/mesh-pkg/src/scaffold.rs clustered README template still contains local mesh-lang product verifier command')), errors.join('\n'))
})
