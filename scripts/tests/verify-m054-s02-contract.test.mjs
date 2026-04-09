import test from 'node:test'
import assert from 'node:assert/strict'
import fs from 'node:fs'
import os from 'node:os'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const scriptDir = path.dirname(fileURLToPath(import.meta.url))
const root = path.resolve(scriptDir, '..', '..')

const files = {
  verifier: 'scripts/verify-m054-s02.sh',
  scaffold: 'compiler/mesh-pkg/src/scaffold.rs',
  e2e: 'compiler/meshc/tests/e2e_m054_s02.rs',
  postgresReadme: 'examples/todo-postgres/README.md',
}

const publicUrlMarker = 'One public app URL may front multiple starter nodes'
const proxyIngressMarker = 'proxy/platform ingress'
const baseUrlMarker = 'BASE_URL'
const responseHeaderMarker = 'X-Mesh-Continuity-Request-Key'
const requestLookupMarker = "Treat that header as an operator/debug seam: take the returned request key and run `meshc cluster continuity <node-name@host:port> <request-key> --json` against a node when you want the same request's continuity record directly."
const startupListCaveat = 'Use the continuity list form first to discover runtime-owned startup records or for general manual investigation.'
const boundedSignalMarker = 'The response header is a runtime-owned operator/debug seam, not a frontend-aware routing signal.'
const overclaimBoundaryMarker = 'The starter does not promise frontend-aware node selection, sticky-session semantics, or a Fly-specific product contract.'
const staleDiffMarker = 'before/after continuity diff'
const staleFlyMarker = 'Fly.io'
const e2eTestName = 'm054_s02_staged_postgres_public_ingress_directly_correlates_selected_get_todos_request'
const e2eHeaderFailureTestName = 'm054_s02_response_header_helper_fails_closed_on_malformed_missing_empty_and_duplicate_headers'
const e2eDriftFailureTestName = 'm054_s02_route_continuity_summary_fails_closed_on_primary_standby_drift'
const staleVerifierMarkers = [
  'source "$ROOT_DIR/.env"',
  'echo "$DATABASE_URL"',
  "printf '%s\\n' \"$DATABASE_URL\"",
]
const scaffoldReadmeLabel = `${files.scaffold} postgres README template`

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

function extractPostgresTodoReadmeTemplate(scaffoldSource) {
  const match = scaffoldSource.match(/fn postgres_todo_readme\(name: &str\) -> String \{\s*r#"([\s\S]*?)"#\s*\.replace/)
  assert.ok(match, 'unable to locate postgres_todo_readme template in compiler/mesh-pkg/src/scaffold.rs')
  return match[1]
}

function validateDirectCorrelationContract(baseRoot) {
  const errors = []
  const scaffoldSource = readFrom(baseRoot, files.scaffold)
  const scaffoldReadme = extractPostgresTodoReadmeTemplate(scaffoldSource)
  const e2e = readFrom(baseRoot, files.e2e)
  const postgresReadme = readFrom(baseRoot, files.postgresReadme)
  const verifier = readFrom(baseRoot, files.verifier)

  for (const [relativePath, text] of [
    [scaffoldReadmeLabel, scaffoldReadme],
    [files.postgresReadme, postgresReadme],
  ]) {
    requireIncludes(errors, relativePath, text, [
      publicUrlMarker,
      proxyIngressMarker,
      baseUrlMarker,
      responseHeaderMarker,
      requestLookupMarker,
      startupListCaveat,
      boundedSignalMarker,
      overclaimBoundaryMarker,
      'meshc cluster continuity <node-name@host:port> <request-key> --json',
    ])
    requireExcludes(errors, relativePath, text, [
      staleDiffMarker,
      staleFlyMarker,
    ])
  }

  requireIncludes(errors, files.e2e, e2e, [
    e2eTestName,
    e2eHeaderFailureTestName,
    e2eDriftFailureTestName,
    'public-selected-list.request-key.txt',
    'public-selected-list.request-key.json',
    'selected-route-direct-primary-record.json',
    'selected-route-direct-standby-record.json',
    'selected-route.primary-diagnostics.entries.json',
    'selected-route.standby-diagnostics.entries.json',
    'staged-postgres-public-ingress-direct-correlation',
  ])

  requireIncludes(errors, files.verifier, verifier, [
    'DATABASE_URL must be set for scripts/verify-m054-s02.sh',
    'bash scripts/verify-m054-s01.sh',
    'cargo test -p meshc --test e2e_m054_s02 -- --nocapture',
    'retained-m054-s01-verify',
    'staged-postgres-public-ingress-direct-correlation-',
    'public-selected-list.request-key.txt',
    'selected-route-direct-primary-record.json',
    'selected-route.primary-diagnostics.entries.json',
    'verify-m054-s02-contract.test.mjs',
    'todo-postgres.README.md',
    'retained-staged-bundle.manifest.json',
    'assert_no_secret_leaks',
    'assert_retained_bundle_shape',
    'latest-proof-bundle.txt',
    'm054-s02-db-env-preflight',
    'm054-s02-s01-replay',
    'm054-s02-e2e',
    'm054-s02-retain-s01-verify',
    'm054-s02-retain-artifacts',
    'm054-s02-retain-staged-bundle',
    'm054-s02-redaction-drift',
    'm054-s02-bundle-shape',
  ])
  requireExcludes(errors, files.verifier, verifier, staleVerifierMarkers)
  requireOrdered(errors, files.verifier, verifier, [
    'bash scripts/verify-m054-s01.sh',
    'cargo test -p meshc --test e2e_m054_s02 -- --nocapture',
    'm054-s02-retain-s01-verify',
    'm054-s02-retain-artifacts',
    'm054-s02-retain-staged-bundle',
    'm054-s02-redaction-drift',
    'm054-s02-bundle-shape',
  ])

  return errors
}

test('current repo publishes the M054 S02 direct-correlation starter and verifier contract', () => {
  const errors = validateDirectCorrelationContract(root)
  assert.deepEqual(errors, [], errors.join('\n'))
})

test('contract fails closed when the starter wording drops the response-header seam, startup caveat, or bounded operator framing', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m054-s02-starter-')
  copyAllFiles(tmpRoot)

  let mutatedReadme = readFrom(tmpRoot, files.postgresReadme)
  mutatedReadme = mutatedReadme.replace(responseHeaderMarker, 'X-Mesh-Trace-Id')
  mutatedReadme = mutatedReadme.replace(startupListCaveat, 'Use the continuity list form first to discover runtime-owned startup records.')
  mutatedReadme = mutatedReadme.replace(boundedSignalMarker, 'The response header tells the frontend which node handled the request.')
  writeTo(tmpRoot, files.postgresReadme, mutatedReadme)

  let mutatedScaffold = readFrom(tmpRoot, files.scaffold)
  mutatedScaffold = mutatedScaffold.replace(requestLookupMarker, 'Treat that header as a client routing signal.')
  mutatedScaffold = mutatedScaffold.replace(overclaimBoundaryMarker, 'The starter promises sticky sessions and frontend-aware routing.')
  writeTo(tmpRoot, files.scaffold, mutatedScaffold)

  const errors = validateDirectCorrelationContract(tmpRoot)
  assert.ok(errors.some((error) => error.includes(`${files.postgresReadme} missing ${JSON.stringify(responseHeaderMarker)}`)), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes(`${files.postgresReadme} missing ${JSON.stringify(startupListCaveat)}`)), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes(`${files.postgresReadme} missing ${JSON.stringify(boundedSignalMarker)}`)), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes(`${scaffoldReadmeLabel} missing ${JSON.stringify(requestLookupMarker)}`)), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes(`${scaffoldReadmeLabel} missing ${JSON.stringify(overclaimBoundaryMarker)}`)), errors.join('\n'))
})

test('contract fails closed when the assembled verifier stops delegating S01, loses retained-bundle markers, or leaks secrets', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m054-s02-verifier-')
  copyAllFiles(tmpRoot)

  let mutatedVerifier = readFrom(tmpRoot, files.verifier)
  mutatedVerifier = mutatedVerifier.replace(
    'bash scripts/verify-m054-s01.sh',
    'cargo test -p meshc --test e2e_m054_s01 -- --nocapture',
  )
  mutatedVerifier = mutatedVerifier.replaceAll('retained-m054-s01-verify', 'retained-m053-s01-verify')
  mutatedVerifier = mutatedVerifier.replaceAll('latest-proof-bundle.txt', 'latest-proof.json')
  mutatedVerifier = mutatedVerifier.replace('assert_no_secret_leaks', 'echo "$DATABASE_URL"')
  mutatedVerifier = mutatedVerifier.replaceAll('staged-postgres-public-ingress-direct-correlation-', 'staged-postgres-public-ingress-truth-')
  writeTo(tmpRoot, files.verifier, mutatedVerifier)

  const errors = validateDirectCorrelationContract(tmpRoot)
  assert.ok(errors.some((error) => error.includes(`${files.verifier} missing ${JSON.stringify('bash scripts/verify-m054-s01.sh')}`) || error.includes(`${files.verifier} drifted order around ${JSON.stringify('bash scripts/verify-m054-s01.sh')}`)), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes(`${files.verifier} missing ${JSON.stringify('retained-m054-s01-verify')}`)), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes(`${files.verifier} missing ${JSON.stringify('latest-proof-bundle.txt')}`)), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes(`${files.verifier} missing ${JSON.stringify('assert_no_secret_leaks')}`) || error.includes(`${files.verifier} still contains stale text ${JSON.stringify('echo "$DATABASE_URL"')}`)), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes(`${files.verifier} missing ${JSON.stringify('staged-postgres-public-ingress-direct-correlation-')}`)), errors.join('\n'))
})
