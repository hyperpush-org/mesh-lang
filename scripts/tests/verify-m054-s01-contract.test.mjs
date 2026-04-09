import test from 'node:test'
import assert from 'node:assert/strict'
import fs from 'node:fs'
import os from 'node:os'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const scriptDir = path.dirname(fileURLToPath(import.meta.url))
const root = path.resolve(scriptDir, '..', '..')

const files = {
  verifier: 'scripts/verify-m054-s01.sh',
  scaffold: 'compiler/mesh-pkg/src/scaffold.rs',
  e2e: 'compiler/meshc/tests/e2e_m054_s01.rs',
  postgresReadme: 'examples/todo-postgres/README.md',
  sqliteReadme: 'examples/todo-sqlite/README.md',
}

const publicUrlMarker = 'One public app URL may front multiple starter nodes'
const proxyIngressMarker = 'proxy/platform ingress'
const baseUrlMarker = 'The smoke helper treats `BASE_URL` as the public app URL.'
const clusterStatusMarker = 'meshc cluster status'
const clusterContinuityMarker = 'meshc cluster continuity'
const clusterDiagnosticsMarker = 'meshc cluster diagnostics'
const frontendSelectionMarker = 'frontend-aware node selection'
const flyBoundaryMarker = 'Fly-specific product contract'
const sqliteBranchMarker = 'meshc init --template todo-api --db sqlite'
const sqliteLocalOnlyMarker = 'there is no `work.mpl`, `HTTP.clustered(...)`, or `meshc cluster` story in this starter'
const e2eReadmeTestName = 'm054_s01_postgres_readme_keeps_one_public_url_contract_and_sqlite_boundary'
const staleVerifierMarkers = [
  'source "$ROOT_DIR/.env"',
  'cat .env',
  'echo "$DATABASE_URL"',
  "printf '%s\\n' \"$DATABASE_URL\"",
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

function copyAllFiles(baseRoot) {
  for (const relativePath of Object.values(files)) {
    copyRepoFile(baseRoot, relativePath)
  }
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

function validateStarterContract(baseRoot) {
  const errors = []
  const scaffold = readFrom(baseRoot, files.scaffold)
  const e2e = readFrom(baseRoot, files.e2e)
  const postgresReadme = readFrom(baseRoot, files.postgresReadme)
  const sqliteReadme = readFrom(baseRoot, files.sqliteReadme)
  const verifier = readFrom(baseRoot, files.verifier)

  for (const [relativePath, text] of [
    [files.scaffold, scaffold],
    [files.postgresReadme, postgresReadme],
  ]) {
    requireIncludes(errors, relativePath, text, [
      publicUrlMarker,
      proxyIngressMarker,
      baseUrlMarker,
      clusterStatusMarker,
      clusterContinuityMarker,
      clusterDiagnosticsMarker,
      frontendSelectionMarker,
      flyBoundaryMarker,
      sqliteBranchMarker,
    ])
  }
  requireExcludes(errors, files.postgresReadme, postgresReadme, ['Fly.io', 'clustered(work)'])

  requireIncludes(errors, files.sqliteReadme, sqliteReadme, [
    sqliteLocalOnlyMarker,
    sqliteBranchMarker,
  ])
  requireExcludes(errors, files.sqliteReadme, sqliteReadme, [
    publicUrlMarker,
    clusterStatusMarker,
  ])

  requireIncludes(errors, files.e2e, e2e, [
    e2eReadmeTestName,
    publicUrlMarker,
    flyBoundaryMarker,
    sqliteBranchMarker,
    sqliteLocalOnlyMarker,
  ])

  requireIncludes(errors, files.verifier, verifier, [
    'DATABASE_URL must be set for scripts/verify-m054-s01.sh',
    'cargo test -p mesh-pkg m049_s01_postgres_scaffold_ -- --nocapture',
    'cargo build -q -p meshc',
    'node scripts/tests/verify-m049-s03-materialize-examples.mjs --check',
    'cargo test -p meshc --test e2e_m047_s04 m047_s04_example_readmes_define_the_public_postgres_vs_sqlite_split -- --nocapture',
    'cargo test -p meshc --test e2e_m054_s01 -- --nocapture',
    'staged-postgres-public-ingress-truth-',
    'public-ingress-truncated-backend-',
    'public-ingress-non-json-and-missing-fields-',
    'latest-proof-bundle.txt',
    'retained-staged-bundle.manifest.json',
    'verify-m054-s01-contract.test.mjs',
    'todo-postgres.README.md',
    'todo-sqlite.README.md',
    'm054-s01-redaction-drift',
    'm054-s01-bundle-shape',
    'assert_no_secret_leaks',
    'assert_retained_bundle_shape',
  ])
  requireExcludes(errors, files.verifier, verifier, staleVerifierMarkers)
  requireOrdered(errors, files.verifier, verifier, [
    'cargo test -p mesh-pkg m049_s01_postgres_scaffold_ -- --nocapture',
    'cargo build -q -p meshc',
    'node scripts/tests/verify-m049-s03-materialize-examples.mjs --check',
    'cargo test -p meshc --test e2e_m047_s04 m047_s04_example_readmes_define_the_public_postgres_vs_sqlite_split -- --nocapture',
    'cargo test -p meshc --test e2e_m054_s01 -- --nocapture',
    'm054-s01-retain-artifacts',
    'm054-s01-retain-staged-bundle',
    'm054-s01-redaction-drift',
    'm054-s01-bundle-shape',
  ])

  return errors
}

test('current repo publishes the M054 S01 one-public-URL starter contract', () => {
  const errors = validateStarterContract(root)
  assert.deepEqual(errors, [], errors.join('\n'))
})

test('contract fails closed when the starter wording drifts back toward direct-node or Fly-specific claims', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m054-s01-starter-')
  copyAllFiles(tmpRoot)

  let mutatedPostgres = readFrom(tmpRoot, files.postgresReadme)
  mutatedPostgres = mutatedPostgres.replace(publicUrlMarker, 'Direct node ports are the public contract')
  mutatedPostgres = mutatedPostgres.replace(flyBoundaryMarker, 'Fly.io product contract')
  writeTo(tmpRoot, files.postgresReadme, mutatedPostgres)

  let mutatedScaffold = readFrom(tmpRoot, files.scaffold)
  mutatedScaffold = mutatedScaffold.replace(proxyIngressMarker, 'frontend picker')
  writeTo(tmpRoot, files.scaffold, mutatedScaffold)

  let mutatedSqlite = readFrom(tmpRoot, files.sqliteReadme)
  mutatedSqlite = mutatedSqlite.replace(sqliteLocalOnlyMarker, 'this starter is clustered too')
  writeTo(tmpRoot, files.sqliteReadme, mutatedSqlite)

  let mutatedE2e = readFrom(tmpRoot, files.e2e)
  mutatedE2e = mutatedE2e.replace(e2eReadmeTestName, 'm054_s01_removed_readme_contract')
  writeTo(tmpRoot, files.e2e, mutatedE2e)

  const errors = validateStarterContract(tmpRoot)
  assert.ok(errors.some((error) => error.includes(`${files.postgresReadme} missing ${JSON.stringify(publicUrlMarker)}`)), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes(`${files.postgresReadme} missing ${JSON.stringify(flyBoundaryMarker)}`) || error.includes(`${files.postgresReadme} still contains stale text ${JSON.stringify('Fly.io')}`)), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes(`${files.sqliteReadme} missing ${JSON.stringify(sqliteLocalOnlyMarker)}`)), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes(`${files.e2e} missing ${JSON.stringify(e2eReadmeTestName)}`)), errors.join('\n'))
})

test('contract fails closed when the assembled verifier drops a phase, bundle marker, or redaction boundary', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m054-s01-verifier-')
  copyAllFiles(tmpRoot)

  let mutatedVerifier = readFrom(tmpRoot, files.verifier)
  mutatedVerifier = mutatedVerifier.replace(
    'node scripts/tests/verify-m049-s03-materialize-examples.mjs --check',
    'cargo test -p meshc --test e2e_m054_s01 -- --nocapture\n  node scripts/tests/verify-m049-s03-materialize-examples.mjs --check',
  )
  mutatedVerifier = mutatedVerifier.replaceAll('latest-proof-bundle.txt', 'latest-proof.json')
  mutatedVerifier = mutatedVerifier.replace('assert_no_secret_leaks', 'echo "$DATABASE_URL"')
  mutatedVerifier = mutatedVerifier.replace('public-ingress-truncated-backend-', 'public-ingress-bad-target-')
  writeTo(tmpRoot, files.verifier, mutatedVerifier)

  const errors = validateStarterContract(tmpRoot)
  assert.ok(errors.some((error) => error.includes(`${files.verifier} missing ${JSON.stringify('latest-proof-bundle.txt')}`)), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes(`${files.verifier} missing ${JSON.stringify('assert_no_secret_leaks')}`) || error.includes(`${files.verifier} still contains stale text ${JSON.stringify('echo "$DATABASE_URL"')}`)), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes(`${files.verifier} drifted order around`) || error.includes(`${files.verifier} missing ordered marker ${JSON.stringify('node scripts/tests/verify-m049-s03-materialize-examples.mjs --check')}`)), errors.join('\n'))
})
