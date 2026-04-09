import fs from 'node:fs'
import os from 'node:os'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const scriptDir = path.dirname(fileURLToPath(import.meta.url))
export const repoRoot = path.resolve(scriptDir, '..', '..')
export const defaultCorpusPath = path.join(repoRoot, 'scripts/fixtures/m036-s01-syntax-corpus.json')

function relativeRepoPath(absolutePath) {
  return path.relative(repoRoot, absolutePath).replace(/\\/g, '/')
}

function readText(absolutePath, label) {
  if (!fs.existsSync(absolutePath)) {
    throw new Error(`[m036-s02] missing ${label}: ${relativeRepoPath(absolutePath)}`)
  }
  return fs.readFileSync(absolutePath, 'utf8')
}

function readJson(absolutePath, label) {
  return JSON.parse(readText(absolutePath, label))
}

function splitLines(text) {
  return text.replace(/\r\n/g, '\n').split('\n')
}

function assertLineRange(lines, caseDef) {
  if (!Number.isInteger(caseDef.startLine) || !Number.isInteger(caseDef.endLine) || caseDef.startLine < 1 || caseDef.endLine < caseDef.startLine) {
    throw new Error(`[m036-s02] corpus case ${caseDef.id} (${caseDef.path}) has an invalid line range ${caseDef.startLine}-${caseDef.endLine}`)
  }
  if (caseDef.endLine > lines.length) {
    throw new Error(`[m036-s02] corpus case ${caseDef.id} (${caseDef.path}) selected lines ${caseDef.startLine}-${caseDef.endLine} outside the source length ${lines.length}`)
  }
}

function findMarkdownFence(lines, caseDef) {
  let activeFence = null

  for (let lineNr = 1; lineNr <= lines.length; lineNr += 1) {
    const line = lines[lineNr - 1]
    const match = line.match(/^```\s*([^\s`]*)\s*$/)
    if (!match) continue

    if (!activeFence) {
      activeFence = {
        openerLine: lineNr,
        language: (match[1] || '').trim().toLowerCase(),
      }
      continue
    }

    activeFence.closerLine = lineNr
    activeFence.contentStartLine = activeFence.openerLine + 1
    activeFence.contentEndLine = activeFence.closerLine - 1

    const containsSelection =
      caseDef.startLine >= activeFence.contentStartLine &&
      caseDef.endLine <= activeFence.contentEndLine

    if (containsSelection) {
      return activeFence
    }

    activeFence = null
  }

  throw new Error(
    `[m036-s02] corpus case ${caseDef.id} (${caseDef.path}) does not resolve to lines inside a fenced code block (${caseDef.startLine}-${caseDef.endLine})`,
  )
}

function materializedFilename(index, caseId) {
  const safeId = String(caseId || `case-${index + 1}`)
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '')
    .slice(0, 80) || `case-${index + 1}`
  return `${String(index + 1).padStart(2, '0')}-${safeId}.mpl`
}

export function materializeCase({ rootDir = repoRoot, caseDef, outDir, index = 0 }) {
  if (!caseDef || typeof caseDef !== 'object') {
    throw new Error('[m036-s02] expected caseDef to be an object')
  }
  if (typeof caseDef.id !== 'string' || caseDef.id === '') {
    throw new Error('[m036-s02] corpus cases must declare a non-empty id')
  }
  if (typeof caseDef.path !== 'string' || caseDef.path === '') {
    throw new Error(`[m036-s02] corpus case ${caseDef.id} is missing its source path`)
  }
  if (typeof outDir !== 'string' || outDir === '') {
    throw new Error(`[m036-s02] corpus case ${caseDef.id} requires an output directory`)
  }

  const absoluteSourcePath = path.resolve(rootDir, caseDef.path)
  const sourceText = readText(absoluteSourcePath, `corpus source for ${caseDef.id}`)
  const sourceLines = splitLines(sourceText)
  assertLineRange(sourceLines, caseDef)

  const sourceKind = caseDef.path.endsWith('.md') ? 'markdown' : 'mesh'
  let snippetLines = sourceLines.slice(caseDef.startLine - 1, caseDef.endLine)

  if (sourceKind === 'markdown') {
    const fence = findMarkdownFence(sourceLines, caseDef)
    if (!['mesh', 'mpl'].includes(fence.language)) {
      throw new Error(
        `[m036-s02] corpus case ${caseDef.id} (${caseDef.path}) is inside a non-mesh fenced block (${fence.language || 'plain'})`,
      )
    }
    if (snippetLines.some((line) => /^```/.test(line))) {
      throw new Error(`[m036-s02] corpus case ${caseDef.id} (${caseDef.path}) rendered markdown fence markers instead of .mpl text`)
    }
  }

  const snippetText = snippetLines.join('\n')
  if (!snippetText.trim()) {
    throw new Error(
      `[m036-s02] corpus case ${caseDef.id} (${caseDef.path}) resolved to empty materialized text (${caseDef.startLine}-${caseDef.endLine})`,
    )
  }

  fs.mkdirSync(outDir, { recursive: true })
  const absoluteMaterializedPath = path.join(outDir, materializedFilename(index, caseDef.id))
  fs.writeFileSync(absoluteMaterializedPath, `${snippetText}\n`)

  const materializedLines = splitLines(snippetText)

  return {
    ...caseDef,
    path: caseDef.path,
    sourcePath: caseDef.path,
    sourceStartLine: caseDef.startLine,
    sourceEndLine: caseDef.endLine,
    sourceKind,
    materializedPath: path.relative(rootDir, absoluteMaterializedPath).replace(/\\/g, '/'),
    startLine: 1,
    endLine: materializedLines.length,
  }
}

export function materializeCorpus({
  rootDir = repoRoot,
  corpusPath = defaultCorpusPath,
  outDir = path.join(os.tmpdir(), 'm036-s02-corpus'),
  manifestPath = path.join(outDir, 'materialized-corpus.json'),
} = {}) {
  const absoluteCorpusPath = path.resolve(corpusPath)
  const corpus = readJson(absoluteCorpusPath, 'syntax corpus manifest')

  if (corpus.contractVersion !== 'm036-s01-syntax-corpus-v1') {
    throw new Error(`[m036-s02] unexpected corpus contract version ${JSON.stringify(corpus.contractVersion)}`)
  }
  if (!Array.isArray(corpus.cases) || corpus.cases.length === 0) {
    throw new Error('[m036-s02] syntax corpus manifest must contain at least one case')
  }

  const casesDir = path.join(outDir, 'cases')
  fs.mkdirSync(casesDir, { recursive: true })

  const materializedCases = corpus.cases.map((caseDef, index) => materializeCase({
    rootDir,
    caseDef,
    outDir: casesDir,
    index,
  }))

  const manifest = {
    contractVersion: corpus.contractVersion,
    generatedBy: 'scripts/tests/verify-m036-s02-materialize-corpus.mjs',
    sourceCorpusPath: path.relative(rootDir, absoluteCorpusPath).replace(/\\/g, '/'),
    cases: materializedCases,
  }

  fs.mkdirSync(path.dirname(manifestPath), { recursive: true })
  fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`)

  return {
    manifest,
    manifestPath,
    casesDir,
  }
}

function parseArgs(argv) {
  const options = {}
  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index]
    if (arg === '--out-dir') {
      options.outDir = argv[index + 1]
      index += 1
    } else if (arg === '--manifest-path') {
      options.manifestPath = argv[index + 1]
      index += 1
    } else if (arg === '--corpus-path') {
      options.corpusPath = argv[index + 1]
      index += 1
    } else if (arg === '--root-dir') {
      options.rootDir = argv[index + 1]
      index += 1
    } else {
      throw new Error(`[m036-s02] unknown argument: ${arg}`)
    }
  }
  return options
}

async function main() {
  const options = parseArgs(process.argv.slice(2))
  const { manifest, manifestPath } = materializeCorpus(options)

  for (const corpusCase of manifest.cases) {
    process.stdout.write(
      `[m036-s02] phase=corpus case=${corpusCase.id} source=${corpusCase.sourcePath}:${corpusCase.sourceStartLine}-${corpusCase.sourceEndLine} materialized=${corpusCase.materializedPath} source_kind=${corpusCase.sourceKind}\n`,
    )
  }

  process.stdout.write(`[m036-s02] phase=corpus result=pass manifest=${path.relative(options.rootDir || repoRoot, manifestPath).replace(/\\/g, '/')} checked_cases=${manifest.cases.length}\n`)
}

const isEntrypoint = process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)
if (isEntrypoint) {
  main().catch((error) => {
    const message = error instanceof Error ? error.message : String(error)
    process.stderr.write(`[m036-s02] phase=corpus result=fail ${message}\n`)
    process.exitCode = 1
  })
}
