import test from 'node:test'
import assert from 'node:assert/strict'
import fs from 'node:fs'
import os from 'node:os'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const scriptDir = path.dirname(fileURLToPath(import.meta.url))
const root = path.resolve(scriptDir, '..', '..')

const files = {
  repoIdentity: 'scripts/lib/repo-identity.json',
  clusteredExample: 'website/docs/docs/getting-started/clustered-example/index.md',
  distributed: 'website/docs/docs/distributed/index.md',
  distributedProof: 'website/docs/docs/distributed-proof/index.md',
  productionBackendProof: 'website/docs/docs/production-backend-proof/index.md',
}

const staleLocalProductRunbookUrl = 'https://github.com/snowdamiz/mesh-lang/blob/main/mesher/README.md'
const staleBackendRunbookUrl = 'https://github.com/snowdamiz/mesh-lang/blob/main/reference-backend/README.md'
const staleFixturePath = 'scripts/fixtures/backend/reference-backend/'
const clusteredExampleLink = '[Clustered Example](/docs/getting-started/clustered-example/)'
const distributedProofLink = '[Distributed Proof](/docs/distributed-proof/)'
const productionBackendProofLink = '[Production Backend Proof](/docs/production-backend-proof/)'
const productVerifierCommand = 'bash mesher/scripts/verify-maintainer-surface.sh'
const compatibilityVerifierCommand = 'bash scripts/verify-m051-s01.sh'
const retainedVerifierCommand = 'bash scripts/verify-m051-s02.sh'
const proofSurfaceVerifierCommand = 'bash scripts/verify-production-proof-surface.sh'
const distributedRoleSentence = '> **Clustered proof surfaces:**'
const distributedProofRoleSentence = 'This is the only public-secondary docs page that carries the named clustered verifier rails.'
const distributedProofChainSentence = "The clustered proof story now centers the generated PostgreSQL starter's M053 chain: `bash scripts/verify-m053-s01.sh` owns staged deploy truth and `bash scripts/verify-m053-s02.sh` owns failover truth. Keep hosted/public-surface checks as operational follow-up instead of the routine public proof chain."
const distributedBoundaryMarker = 'mesh-lang keeps only the public proof-page wrappers and retained compatibility rails on this side of the boundary.'
const distributedProofBoundaryMarker = 'The local `verify-m051*` rails stay retained compatibility wrappers, not the public clustered story.'
const productionRoleSentence = 'This is the compact public-secondary handoff for Mesh\'s backend proof story.'
const productionBoundarySentence = 'This page is the repo-boundary handoff from mesh-lang into the maintained backend/app contract.'
const productionPrimaryBoundaryMarker = 'mesh-lang keeps only the public proof-page contract and retained compatibility wrappers on this side of the boundary.'
const distributedProofPostgresBullet = '- [`examples/todo-postgres/README.md`](https://github.com/hyperpush-org/mesh-lang/blob/main/examples/todo-postgres/README.md) — the serious shared/deployable starter that owns the shipped clustered contract'
const distributedProofSqliteBullet = '- [`examples/todo-sqlite/README.md`](https://github.com/hyperpush-org/mesh-lang/blob/main/examples/todo-sqlite/README.md) — the honest local single-node SQLite starter, not a clustered/operator proof surface'
const distributedProofS01Bullet = '- `bash scripts/verify-m053-s01.sh` — starter-owned staged deploy proof that retains the generated PostgreSQL bundle plus bundled artifacts'
const distributedProofS02Bullet = '- `bash scripts/verify-m053-s02.sh` — starter-owned failover proof that replays S01, exercises the staged PostgreSQL starter under failover, and retains the failover proof bundle'
const distributedProofProductionBullet = '- [Production Backend Proof](/docs/production-backend-proof/) — the compact backend proof handoff before any maintainer-only surface'
const distributedProofProductRepoBullet = '- [Hyperpush product repo](https://github.com/hyperpush-org/hyperpush-mono) — repo-boundary maintained-app/backend handoff'
const distributedProofProductRunbookBullet = '- [`mesher/README.md`](https://github.com/hyperpush-org/hyperpush-mono/blob/main/mesher/README.md) — deeper maintained app runbook after the repo-boundary handoff'

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

function loadRepoIdentity(baseRoot) {
  return JSON.parse(readFrom(baseRoot, files.repoIdentity))
}

function productRepoUrl(repoIdentity) {
  return repoIdentity.productRepo.repoUrl
}

function productRunbookUrl(repoIdentity) {
  return `${repoIdentity.productRepo.blobBaseUrl}${repoIdentity.productHandoff.relativeRunbookPath}`
}

function sqliteStarterUrl(repoIdentity) {
  return `${repoIdentity.languageRepo.blobBaseUrl}examples/todo-sqlite/README.md`
}

function postgresStarterUrl(repoIdentity) {
  return `${repoIdentity.languageRepo.blobBaseUrl}examples/todo-postgres/README.md`
}

function validateSecondarySurfaces(baseRoot) {
  const errors = []
  const repoIdentity = loadRepoIdentity(baseRoot)
  const productRepo = productRepoUrl(repoIdentity)
  const productRunbook = productRunbookUrl(repoIdentity)
  const sqliteStarter = sqliteStarterUrl(repoIdentity)
  const postgresStarter = postgresStarterUrl(repoIdentity)

  const clusteredExample = readFrom(baseRoot, files.clusteredExample)
  const distributed = readFrom(baseRoot, files.distributed)
  const distributedProof = readFrom(baseRoot, files.distributedProof)
  const productionBackendProof = readFrom(baseRoot, files.productionBackendProof)

  requireIncludes(errors, files.clusteredExample, clusteredExample, [
    '## After the scaffold, pick the follow-on starter',
    'meshc init --template todo-api --db sqlite my_local_todo',
    'meshc init --template todo-api --db postgres my_shared_todo',
    productionBackendProofLink,
    distributedProofLink,
  ])
  requireOrdered(errors, files.clusteredExample, clusteredExample, [
    'meshc init --template todo-api --db sqlite my_local_todo',
    'meshc init --template todo-api --db postgres my_shared_todo',
    productionBackendProofLink,
    distributedProofLink,
  ])

  requireIncludes(errors, files.distributed, distributed, [
    distributedRoleSentence,
    clusteredExampleLink,
    distributedProofLink,
    productionBackendProofLink,
    productRepo,
    productRunbook,
    sqliteStarter,
    postgresStarter,
    distributedBoundaryMarker,
  ])
  requireExcludes(errors, files.distributed, distributed, [
    staleLocalProductRunbookUrl,
    compatibilityVerifierCommand,
    retainedVerifierCommand,
    staleBackendRunbookUrl,
    staleFixturePath,
  ])
  requireOrdered(errors, files.distributed, distributed, [
    clusteredExampleLink,
    distributedProofLink,
    productionBackendProofLink,
    productRepo,
    productRunbook,
  ])

  requireIncludes(errors, files.distributedProof, distributedProof, [
    distributedProofRoleSentence,
    distributedProofChainSentence,
    '## Public surfaces and verifier rails',
    '## Retained reference rails',
    '## Named proof commands',
    clusteredExampleLink,
    postgresStarter,
    sqliteStarter,
    'bash scripts/verify-m053-s01.sh',
    'bash scripts/verify-m053-s02.sh',
    productionBackendProofLink,
    productRepo,
    productRunbook,
    distributedProofBoundaryMarker,
    compatibilityVerifierCommand,
    retainedVerifierCommand,
    'bash scripts/verify-m043-s04-fly.sh --help',
  ])
  requireExcludes(errors, files.distributedProof, distributedProof, [
    staleLocalProductRunbookUrl,
    staleBackendRunbookUrl,
    staleFixturePath,
  ])
  requireOrdered(errors, files.distributedProof, distributedProof, [
    clusteredExampleLink,
    distributedProofPostgresBullet,
    distributedProofSqliteBullet,
    distributedProofS01Bullet,
    distributedProofS02Bullet,
    distributedProofProductionBullet,
    distributedProofProductRepoBullet,
    distributedProofProductRunbookBullet,
  ])
  requireOrdered(errors, files.distributedProof, distributedProof, [
    '## Public surfaces and verifier rails',
    '## Retained reference rails',
    '## Named proof commands',
  ])
  requireOrdered(errors, files.distributedProof, distributedProof, [
    compatibilityVerifierCommand,
    retainedVerifierCommand,
  ])

  requireIncludes(errors, files.productionBackendProof, productionBackendProof, [
    productionRoleSentence,
    productionBoundarySentence,
    productionPrimaryBoundaryMarker,
    '## Canonical surfaces',
    '## Named maintainer verifiers',
    '## Retained backend-only recovery signals',
    '## When to use this page vs the generic guides',
    '## Failure inspection map',
    clusteredExampleLink,
    sqliteStarter,
    postgresStarter,
    productRepo,
    productRunbook,
    productVerifierCommand,
    compatibilityVerifierCommand,
    retainedVerifierCommand,
    proofSurfaceVerifierCommand,
    'restart_count',
    'last_exit_reason',
    'recovered_jobs',
    'last_recovery_at',
    'last_recovery_job_id',
    'last_recovery_count',
    'recovery_active',
  ])
  requireExcludes(errors, files.productionBackendProof, productionBackendProof, [
    staleLocalProductRunbookUrl,
    staleBackendRunbookUrl,
    staleFixturePath,
  ])
  requireOrdered(errors, files.productionBackendProof, productionBackendProof, [
    '## Canonical surfaces',
    '## Named maintainer verifiers',
    '## Retained backend-only recovery signals',
    '## When to use this page vs the generic guides',
    '## Failure inspection map',
  ])
  requireOrdered(errors, files.productionBackendProof, productionBackendProof, [
    clusteredExampleLink,
    sqliteStarter,
    postgresStarter,
    productRepo,
    productRunbook,
    productVerifierCommand,
    compatibilityVerifierCommand,
    retainedVerifierCommand,
  ])

  return errors
}

test('current repo publishes the T02 secondary-surface repo-boundary product handoff', () => {
  const errors = validateSecondarySurfaces(root)
  assert.deepEqual(errors, [], errors.join('\n'))
})

test('contract fails closed when Distributed Actors or Production Backend Proof reintroduce local mesh-lang product paths', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m050-s03-local-product-')
  copyAllFiles(tmpRoot)

  let mutatedDistributed = readFrom(tmpRoot, files.distributed)
  mutatedDistributed = mutatedDistributed.replace(productRunbookUrl(loadRepoIdentity(tmpRoot)), staleLocalProductRunbookUrl)
  mutatedDistributed = `${mutatedDistributed}\nCompatibility shortcut: ${compatibilityVerifierCommand}\n`
  writeTo(tmpRoot, files.distributed, mutatedDistributed)

  let mutatedProductionProof = readFrom(tmpRoot, files.productionBackendProof)
  mutatedProductionProof = mutatedProductionProof.replace(productRepoUrl(loadRepoIdentity(tmpRoot)), staleLocalProductRunbookUrl)
  writeTo(tmpRoot, files.productionBackendProof, mutatedProductionProof)

  const errors = validateSecondarySurfaces(tmpRoot)
  assert.ok(errors.some((error) => error.includes(`${files.distributed} still contains stale text ${JSON.stringify(staleLocalProductRunbookUrl)}`)), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes(`${files.distributed} still contains stale text ${JSON.stringify(compatibilityVerifierCommand)}`)), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes(`${files.productionBackendProof} missing ${JSON.stringify(productRepoUrl(loadRepoIdentity(tmpRoot)))}`) || error.includes(`${files.productionBackendProof} still contains stale text ${JSON.stringify(staleLocalProductRunbookUrl)}`)), errors.join('\n'))
})

test('contract fails closed when Distributed Proof loses the repo-boundary handoff or reorders retained wrappers ahead of the product surfaces', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m050-s03-proof-order-')
  copyAllFiles(tmpRoot)

  let mutated = readFrom(tmpRoot, files.distributedProof)
  mutated = mutated.replace(productRepoUrl(loadRepoIdentity(tmpRoot)), staleLocalProductRunbookUrl)
  mutated = mutated.replace(distributedProofBoundaryMarker, 'The local verify-m051 rails are the public clustered story.')
  mutated = mutated.replace(
    '- `bash scripts/verify-m051-s01.sh` — mesh-lang compatibility wrapper that confirms the public handoff still points at the product-owned Mesher verifier\n- `bash scripts/verify-m051-s02.sh` — retained backend-only verifier replay kept behind the repo-boundary handoff\n- `bash scripts/verify-m047-s04.sh` — authoritative M047 cutover rail for the source-first route-free clustered contract',
    '- `bash scripts/verify-m051-s02.sh` — retained backend-only verifier replay kept behind the repo-boundary handoff\n- `bash scripts/verify-m051-s01.sh` — mesh-lang compatibility wrapper that confirms the public handoff still points at the product-owned Mesher verifier\n- `bash scripts/verify-m047-s04.sh` — authoritative M047 cutover rail for the source-first route-free clustered contract',
  )
  writeTo(tmpRoot, files.distributedProof, mutated)

  const errors = validateSecondarySurfaces(tmpRoot)
  assert.ok(errors.some((error) => error.includes(`${files.distributedProof} missing ${JSON.stringify(productRepoUrl(loadRepoIdentity(tmpRoot)))}`) || error.includes(`${files.distributedProof} still contains stale text ${JSON.stringify(staleLocalProductRunbookUrl)}`)), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes(`${files.distributedProof} missing ${JSON.stringify(distributedProofBoundaryMarker)}`)), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes(`${files.distributedProof} drifted order around ${JSON.stringify(retainedVerifierCommand)}`)), errors.join('\n'))
})

test('contract fails closed when the clustered-example or proof pages lose the SQLite/Postgres/public-proof ordering boundary', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m050-s03-starter-order-')
  copyAllFiles(tmpRoot)

  let mutatedClusteredExample = readFrom(tmpRoot, files.clusteredExample)
  mutatedClusteredExample = mutatedClusteredExample.replace(
    'meshc init --template todo-api --db sqlite my_local_todo',
    'meshc init --template todo-api --db postgres my_shared_todo',
  )
  writeTo(tmpRoot, files.clusteredExample, mutatedClusteredExample)

  let mutatedDistributedProof = readFrom(tmpRoot, files.distributedProof)
  mutatedDistributedProof = mutatedDistributedProof.replace(
    distributedProofSqliteBullet,
    '- [`examples/todo-sqlite/README.md`](https://example.invalid/not-sqlite) — broken sqlite boundary',
  )
  writeTo(tmpRoot, files.distributedProof, mutatedDistributedProof)

  const errors = validateSecondarySurfaces(tmpRoot)
  const errorText = errors.join('\n')
  assert.ok(errorText.includes('meshc init --template todo-api --db sqlite my_local_todo'), errorText)
  assert.ok(errorText.includes(distributedProofSqliteBullet) || errorText.includes(distributedProofS01Bullet), errorText)
})
