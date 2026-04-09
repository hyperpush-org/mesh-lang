import test from 'node:test'
import assert from 'node:assert/strict'
import fs from 'node:fs'
import os from 'node:os'
import path from 'node:path'

import { materializeCase, materializeCorpus, repoRoot } from './verify-m036-s02-materialize-corpus.mjs'

function makeTempDir() {
  return fs.mkdtempSync(path.join(os.tmpdir(), 'm036-s02-materialize-'))
}

test('materializeCorpus writes per-case .mpl files and preserves case metadata', () => {
  const outDir = makeTempDir()
  const { manifest, manifestPath } = materializeCorpus({
    rootDir: repoRoot,
    outDir,
    manifestPath: path.join(outDir, 'materialized-corpus.json'),
  })

  assert.ok(fs.existsSync(manifestPath), 'materialized manifest should be written')
  assert.equal(manifest.contractVersion, 'm036-s01-syntax-corpus-v1')
  assert.equal(manifest.cases.length, 15, 'should materialize the full shared corpus')

  const docsCase = manifest.cases.find((corpusCase) => corpusCase.id === 'docs-language-basics-preferred-hash')
  assert.ok(docsCase, 'expected docs-backed corpus case to survive materialization')
  assert.equal(docsCase.sourceKind, 'markdown')
  assert.equal(docsCase.startLine, 1)
  assert.equal(docsCase.endLine, 1)
  assert.deepEqual(docsCase.expectedForms, ['hash'])
  const docsText = fs.readFileSync(path.join(repoRoot, docsCase.materializedPath), 'utf8')
  assert.match(docsText, /println\("Hello, #\{name\}!"\)/)
  assert.doesNotMatch(docsText, /```/, 'materialized docs snippets must not include markdown fences')

  const tripleCase = manifest.cases.find((corpusCase) => corpusCase.id === 'docs-language-basics-heredoc-hash')
  assert.ok(tripleCase, 'expected heredoc docs case to survive materialization')
  assert.equal(tripleCase.startLine, 1)
  assert.equal(tripleCase.endLine, 3)
  assert.equal(tripleCase.expectedStringKind, 'triple')

  const fixtureCase = manifest.cases.find((corpusCase) => corpusCase.id === 'fixture-no-interpolation')
  assert.ok(fixtureCase, 'expected .mpl-backed fixture case to survive materialization')
  assert.equal(fixtureCase.sourceKind, 'mesh')
  assert.equal(fixtureCase.expectNoInterpolation, true)
  assert.equal(fixtureCase.startLine, 1)
  assert.equal(fixtureCase.endLine, 1)
  const fixtureText = fs.readFileSync(path.join(repoRoot, fixtureCase.materializedPath), 'utf8')
  assert.equal(fixtureText, '"no interpolation"\n')
})

test('materializeCase fails closed when a markdown selection is not inside a mesh fence', () => {
  const outDir = makeTempDir()

  assert.throws(
    () => materializeCase({
      rootDir: repoRoot,
      outDir,
      caseDef: {
        id: 'bad-markdown-selection',
        path: 'website/docs/docs/language-basics/index.md',
        startLine: 56,
        endLine: 56,
        expectedForms: ['hash'],
        expectedStringKind: 'double',
      },
    }),
    /does not resolve to lines inside a fenced code block/,
  )
})
