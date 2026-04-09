import test from 'node:test'
import assert from 'node:assert/strict'
import fs from 'node:fs'
import os from 'node:os'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const scriptDir = path.dirname(fileURLToPath(import.meta.url))
const root = path.resolve(scriptDir, '..', '..')

const files = {
  index: 'website/docs/index.md',
  config: 'website/docs/.vitepress/config.mts',
  proof: 'website/docs/docs/distributed-proof/index.md',
  og: 'website/scripts/generate-og-image.py',
}

const homepageDescription = 'One public app URL fronts multiple Mesh nodes. Runtime placement stays server-side, and operator truth stays on meshc cluster.'
const oldGenericTagline = 'Built-in failover, load balancing, and exactly-once semantics'
const proofBoundaryMarker = 'A proxy/platform ingress may expose one public app URL in front of multiple nodes, but that is where the public routing story ends.'
const proofHeaderMarker = 'clustered `GET /todos` and `GET /todos/:id` responses include `X-Mesh-Continuity-Request-Key`; when you have that header, jump straight to the same request with `meshc cluster continuity <node-name@host:port> <request-key> --json`'
const proofListMarker = 'continuity-list discovery stays for startup records and manual inspection when you do not already have a request key'
const proofWorkflowMarker = 'If a clustered HTTP response returned `X-Mesh-Continuity-Request-Key`, run `meshc cluster continuity <node-name@host:port> <request-key> --json` directly for that same public request.'
const proofNonGoalMarker = 'sticky sessions, frontend-aware routing, or client-visible topology claims'
const flyEvidenceMarker = 'keep Fly as a retained read-only reference/proof lane for already-deployed environments instead of treating it as a coequal public starter surface'
const ogSubtitleMarker = "subtitle = 'One public app URL. Server-side runtime placement. Operator truth stays on meshc cluster.'"
const ogBadgeMarker = "for badge in ['@cluster', 'LLVM native', 'Type-safe', 'One public URL']"
const configAltMarker = 'one public app URL, server-side runtime placement, and operator truth on meshc cluster.'

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

function validatePublicDocsContract(baseRoot) {
  const errors = []
  const index = readFrom(baseRoot, files.index)
  const config = readFrom(baseRoot, files.config)
  const proof = readFrom(baseRoot, files.proof)
  const og = readFrom(baseRoot, files.og)

  requireIncludes(errors, files.index, index, [
    `description: ${homepageDescription}`,
  ])
  requireExcludes(errors, files.index, index, [oldGenericTagline])

  requireIncludes(errors, files.config, config, [
    homepageDescription,
    configAltMarker,
  ])
  requireExcludes(errors, files.config, config, [oldGenericTagline])

  requireIncludes(errors, files.proof, proof, [
    proofBoundaryMarker,
    proofHeaderMarker,
    proofListMarker,
    proofWorkflowMarker,
    proofNonGoalMarker,
    flyEvidenceMarker,
  ])
  requireOrdered(errors, files.proof, proof, [
    proofBoundaryMarker,
    proofHeaderMarker,
    proofWorkflowMarker,
    proofNonGoalMarker,
  ])

  requireIncludes(errors, files.og, og, [
    ogSubtitleMarker,
    ogBadgeMarker,
  ])
  requireExcludes(errors, files.og, og, [oldGenericTagline])

  return errors
}

test('current repo publishes the M054 S03 bounded homepage, proof-page, and OG contract', () => {
  const errors = validatePublicDocsContract(root)
  assert.deepEqual(errors, [], errors.join('\n'))
})

test('contract fails closed when homepage metadata or OG copy drifts back to the old generic load-balancing claim', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m054-s03-home-')
  copyAllFiles(tmpRoot)

  let mutatedIndex = readFrom(tmpRoot, files.index)
  mutatedIndex = mutatedIndex.replace(homepageDescription, 'One annotation to distribute work across a fleet. Built-in failover, load balancing, and exactly-once semantics — no orchestration layer required.')
  writeTo(tmpRoot, files.index, mutatedIndex)

  let mutatedConfig = readFrom(tmpRoot, files.config)
  mutatedConfig = mutatedConfig.replace(homepageDescription, 'Built-in failover, load balancing, and exactly-once semantics')
  writeTo(tmpRoot, files.config, mutatedConfig)

  let mutatedOg = readFrom(tmpRoot, files.og)
  mutatedOg = mutatedOg.replace(ogSubtitleMarker, "subtitle = 'One annotation. Native speed. Auto-failover, load balancing, and exactly-once semantics.'")
  writeTo(tmpRoot, files.og, mutatedOg)

  const errors = validatePublicDocsContract(tmpRoot)
  assert.ok(errors.some((error) => error.includes(`${files.index} missing ${JSON.stringify(`description: ${homepageDescription}`)}`)), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes(`${files.config} missing ${JSON.stringify(homepageDescription)}`) || error.includes(`${files.config} still contains stale text ${JSON.stringify(oldGenericTagline)}`)), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes(`${files.og} missing ${JSON.stringify(ogSubtitleMarker)}`)), errors.join('\n'))
})

test('contract fails closed when Distributed Proof drops the ingress/runtime boundary or the direct request-key lookup flow', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m054-s03-proof-')
  copyAllFiles(tmpRoot)

  let mutatedProof = readFrom(tmpRoot, files.proof)
  mutatedProof = mutatedProof.replace(proofBoundaryMarker, 'A proxy forwards traffic.')
  mutatedProof = mutatedProof.replace(proofHeaderMarker, 'clustered routes are load balanced automatically')
  mutatedProof = mutatedProof.replace(proofWorkflowMarker, 'Always diff continuity lists before looking at a single record.')
  mutatedProof = mutatedProof.replace(proofNonGoalMarker, 'sticky sessions are supported')
  writeTo(tmpRoot, files.proof, mutatedProof)

  const errors = validatePublicDocsContract(tmpRoot)
  assert.ok(errors.some((error) => error.includes(`${files.proof} missing ${JSON.stringify(proofBoundaryMarker)}`)), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes(`${files.proof} missing ${JSON.stringify(proofHeaderMarker)}`)), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes(`${files.proof} missing ${JSON.stringify(proofWorkflowMarker)}`)), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes(`${files.proof} missing ${JSON.stringify(proofNonGoalMarker)}`)), errors.join('\n'))
})
