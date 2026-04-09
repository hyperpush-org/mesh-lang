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
  clusterProofReadme: 'scripts/fixtures/clustered/cluster-proof/README.md',
  clusterProofTests: 'scripts/fixtures/clustered/cluster-proof/tests/work.test.mpl',
  flyVerifier: 'scripts/verify-m043-s04-fly.sh',
}

const staleLocalProductRunbookUrl = 'https://github.com/snowdamiz/mesh-lang/blob/main/mesher/README.md'
const staleHistoricalRails = [
  'bash scripts/verify-m047-s04.sh',
  'bash scripts/verify-m047-s05.sh',
  'bash scripts/verify-m047-s06.sh',
  'bash scripts/verify-m043-s04-fly.sh --help',
  'CLUSTER_PROOF_FLY_APP=',
  'mesh-cluster-proof.fly.dev',
]
const distributedCalloutMarker = '> **Clustered proof surfaces:**'
const distributedM053MapMarker = 'M053 starter-owned staged deploy + failover proof map'
const distributedFlyBoundaryMarker = 'retained read-only Fly reference lane'
const distributedProofRoleSentence = 'This is the only public-secondary docs page that carries the named clustered verifier rails.'
const distributedProofChainSentence = "The clustered proof story now centers the generated PostgreSQL starter's M053 chain: `bash scripts/verify-m053-s01.sh` owns staged deploy truth and `bash scripts/verify-m053-s02.sh` owns failover truth. Keep hosted/public-surface checks as operational follow-up instead of the routine public proof chain."
const distributedProofFlyBoundaryMarker = 'keep Fly as a retained read-only reference/proof lane for already-deployed environments instead of treating it as a coequal public starter surface'
const distributedProofFlyNote = '> **Note:** The Fly verifier is intentionally read-only and intentionally secondary.'
const clusterProofRetainedMarker = '`scripts/fixtures/clustered/cluster-proof/` is a retained reference/proof fixture for the older Fly-oriented packaging rail.'
const clusterProofStarterBoundary = 'It is not a public starter surface'
const clusterProofScopeMarker = 'bounded read-only/reference environment'
const flyHelpIntro = 'Read-only Fly verifier for the retained `cluster-proof` reference rail.'
const flyHelpBoundary = 'This help surface documents a bounded reference/proof lane; it does not define a public starter surface.'
const flyHelpScopeMarker = 'This script is a retained reference sanity/config/log/probe rail.'
const flyHelpNoMutationsMarker = 'no machine restarts, scale changes, or secret writes'
const flyHelpNotStarterMarker = 'does not promote Fly or `cluster-proof` into a public starter surface'
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

function validateS04DocsReferenceContract(baseRoot) {
  const errors = []
  const repoIdentity = loadRepoIdentity(baseRoot)
  const productRepo = productRepoUrl(repoIdentity)
  const productRunbook = productRunbookUrl(repoIdentity)
  const sqliteStarter = sqliteStarterUrl(repoIdentity)
  const postgresStarter = postgresStarterUrl(repoIdentity)

  const clusteredExample = readFrom(baseRoot, files.clusteredExample)
  const distributed = readFrom(baseRoot, files.distributed)
  const distributedProof = readFrom(baseRoot, files.distributedProof)
  const clusterProofReadme = readFrom(baseRoot, files.clusterProofReadme)
  const clusterProofTests = readFrom(baseRoot, files.clusterProofTests)
  const flyVerifier = readFrom(baseRoot, files.flyVerifier)

  requireIncludes(errors, files.clusteredExample, clusteredExample, [
    '## After the scaffold, pick the follow-on starter',
    'meshc init --template todo-api --db sqlite my_local_todo',
    'meshc init --template todo-api --db postgres my_shared_todo',
    '[Production Backend Proof](/docs/production-backend-proof/)',
    '[Distributed Proof](/docs/distributed-proof/)',
  ])
  requireOrdered(errors, files.clusteredExample, clusteredExample, [
    'meshc init --template todo-api --db sqlite my_local_todo',
    'meshc init --template todo-api --db postgres my_shared_todo',
    '[Production Backend Proof](/docs/production-backend-proof/)',
    '[Distributed Proof](/docs/distributed-proof/)',
  ])

  requireIncludes(errors, files.distributed, distributed, [
    distributedCalloutMarker,
    distributedM053MapMarker,
    distributedFlyBoundaryMarker,
    '[Distributed Proof](/docs/distributed-proof/)',
    '[Production Backend Proof](/docs/production-backend-proof/)',
    productRepo,
    productRunbook,
    sqliteStarter,
    postgresStarter,
  ])
  requireExcludes(errors, files.distributed, distributed, [
    staleLocalProductRunbookUrl,
    ...staleHistoricalRails,
  ])

  requireIncludes(errors, files.distributedProof, distributedProof, [
    distributedProofRoleSentence,
    distributedProofChainSentence,
    '## Public surfaces and verifier rails',
    '## Retained reference rails',
    '## Named proof commands',
    'bash scripts/verify-m053-s01.sh',
    'bash scripts/verify-m053-s02.sh',
    'Keep hosted/public-surface checks as operational follow-up instead of the routine public proof chain.',
    'bash scripts/verify-m043-s04-fly.sh --help',
    distributedProofFlyBoundaryMarker,
    distributedProofFlyNote,
    productRepo,
    productRunbook,
    sqliteStarter,
    postgresStarter,
  ])
  requireExcludes(errors, files.distributedProof, distributedProof, [
    staleLocalProductRunbookUrl,
  ])
  requireOrdered(errors, files.distributedProof, distributedProof, [
    '[Clustered Example](/docs/getting-started/clustered-example/)',
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

  requireIncludes(errors, files.clusterProofReadme, clusterProofReadme, [
    clusterProofRetainedMarker,
    clusterProofStarterBoundary,
    'generated `meshc init --clustered` scaffold and the PostgreSQL Todo starter own the shipped M053 clustered contract',
    'route-free',
    clusterProofScopeMarker,
    'meshc cluster status',
    'meshc cluster continuity',
    'meshc cluster diagnostics',
  ])
  requireExcludes(errors, files.clusterProofReadme, clusterProofReadme, [
    'one of the three equal canonical clustered surfaces',
    'mesh-cluster-proof.fly.dev',
  ])

  requireIncludes(errors, files.flyVerifier, flyVerifier, [
    flyHelpIntro,
    flyHelpBoundary,
    flyHelpScopeMarker,
    flyHelpNoMutationsMarker,
    flyHelpNotStarterMarker,
  ])
  requireExcludes(errors, files.flyVerifier, flyVerifier, [
    'Read-only Fly verifier for the M043 failover/operator rail.',
    'public starter lane',
  ])

  requireIncludes(errors, files.clusterProofTests, clusterProofTests, [
    'assert_contains(readme, "retained reference/proof fixture")',
    'assert_contains(readme, "It is not a public starter surface")',
    'assert_contains(readme, "bounded read-only/reference environment")',
    'assert_contains(verifier, "Read-only Fly verifier for the retained `cluster-proof` reference rail.")',
    'assert_contains(verifier, "it does not define a public starter surface")',
    'assert_contains(verifier, "does not promote Fly or `cluster-proof` into a public starter surface")',
    'assert_not_contains(verifier, "Read-only Fly verifier for the M043 failover/operator rail.")',
    'assert_not_contains(readme, "one of the three equal canonical clustered surfaces")',
  ])

  return errors
}

test('current repo publishes the fail-closed M053 S04 docs and retained Fly reference contract', () => {
  const errors = validateS04DocsReferenceContract(root)
  assert.deepEqual(errors, [], errors.join('\n'))
})

test('contract fails closed when Distributed Proof drops the M053 starter chain or widens Fly into a public starter', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m053-s04-distributed-proof-')
  copyAllFiles(tmpRoot)

  let mutatedDistributedProof = readFrom(tmpRoot, files.distributedProof)
  mutatedDistributedProof = mutatedDistributedProof.replace('Keep hosted/public-surface checks as operational follow-up instead of the routine public proof chain.', 'Fly owns the clustered proof story now.')
  mutatedDistributedProof = mutatedDistributedProof.replace(distributedProofFlyBoundaryMarker, 'keep Fly as the canonical public starter lane')
  mutatedDistributedProof = mutatedDistributedProof.replace('bash scripts/verify-m053-s02.sh', 'bash scripts/verify-m047-s06.sh')
  writeTo(tmpRoot, files.distributedProof, mutatedDistributedProof)

  const errors = validateS04DocsReferenceContract(tmpRoot)
  const errorText = errors.join('\n')
  assert.ok(errorText.includes(distributedProofChainSentence), errorText)
  assert.ok(errorText.includes(distributedProofFlyBoundaryMarker), errorText)
  assert.ok(errorText.includes('bash scripts/verify-m053-s02.sh'), errorText)
})

test('contract fails closed when distributed public docs regress toward local-product or proof-maze-first teaching', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m053-s04-public-docs-')
  copyAllFiles(tmpRoot)

  let mutatedClusteredExample = readFrom(tmpRoot, files.clusteredExample)
  mutatedClusteredExample = mutatedClusteredExample.replace(
    '[Production Backend Proof](/docs/production-backend-proof/)',
    '[Distributed Proof](/docs/distributed-proof/)',
  )
  writeTo(tmpRoot, files.clusteredExample, mutatedClusteredExample)

  let mutatedDistributed = readFrom(tmpRoot, files.distributed)
  mutatedDistributed = mutatedDistributed.replace(productRepoUrl(loadRepoIdentity(tmpRoot)), staleLocalProductRunbookUrl)
  mutatedDistributed = `${mutatedDistributed}\nDirect proof rail: bash scripts/verify-m047-s04.sh\n`
  writeTo(tmpRoot, files.distributed, mutatedDistributed)

  const errors = validateS04DocsReferenceContract(tmpRoot)
  assert.ok(errors.some((error) => error.includes(`${files.clusteredExample} drifted order around ${JSON.stringify('[Distributed Proof](/docs/distributed-proof/)')}`) || error.includes(`${files.clusteredExample} missing ordered marker ${JSON.stringify('[Production Backend Proof](/docs/production-backend-proof/)')}`)), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes(`${files.distributed} missing ${JSON.stringify(productRepoUrl(loadRepoIdentity(tmpRoot)))}`) || error.includes(`${files.distributed} still contains stale text ${JSON.stringify(staleLocalProductRunbookUrl)}`)), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes(`${files.distributed} still contains stale text ${JSON.stringify('bash scripts/verify-m047-s04.sh')}`)), errors.join('\n'))
})

test('contract fails closed when retained Fly assets stop pinning retained-reference wording', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m053-s04-fly-assets-')
  copyAllFiles(tmpRoot)

  let mutatedReadme = readFrom(tmpRoot, files.clusterProofReadme)
  mutatedReadme = mutatedReadme.replace(clusterProofStarterBoundary, 'It is the public starter surface')
  mutatedReadme = mutatedReadme.replace(clusterProofScopeMarker, 'required deploy environment')
  writeTo(tmpRoot, files.clusterProofReadme, mutatedReadme)

  let mutatedFlyVerifier = readFrom(tmpRoot, files.flyVerifier)
  mutatedFlyVerifier = mutatedFlyVerifier.replace(flyHelpIntro, 'Read-only Fly verifier for the public starter rail.')
  mutatedFlyVerifier = mutatedFlyVerifier.replace(flyHelpBoundary, 'This help surface defines the public starter surface.')
  writeTo(tmpRoot, files.flyVerifier, mutatedFlyVerifier)

  let mutatedClusterProofTests = readFrom(tmpRoot, files.clusterProofTests)
  mutatedClusterProofTests = mutatedClusterProofTests.replace(
    'assert_contains(readme, "It is not a public starter surface")',
    'assert_contains(readme, "It is the public starter surface")',
  )
  writeTo(tmpRoot, files.clusterProofTests, mutatedClusterProofTests)

  const errors = validateS04DocsReferenceContract(tmpRoot)
  assert.ok(errors.some((error) => error.includes(`${files.clusterProofReadme} missing ${JSON.stringify(clusterProofStarterBoundary)}`)), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes(`${files.clusterProofReadme} missing ${JSON.stringify(clusterProofScopeMarker)}`)), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes(`${files.flyVerifier} missing ${JSON.stringify(flyHelpIntro)}`)), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes(`${files.flyVerifier} missing ${JSON.stringify(flyHelpBoundary)}`)), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes(`${files.clusterProofTests} missing ${JSON.stringify('assert_contains(readme, "It is not a public starter surface")')}`)), errors.join('\n'))
})
