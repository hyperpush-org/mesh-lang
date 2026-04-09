import crypto from 'node:crypto'
import fs from 'node:fs'
import os from 'node:os'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const scriptDir = path.dirname(fileURLToPath(import.meta.url))
export const repoRoot = path.resolve(scriptDir, '..')
export const defaultOutputRoot = path.join(repoRoot, '.tmp', 'm055-s04', 'workspace', 'hyperpush-mono')
export const defaultTemplatesRoot = path.join(repoRoot, 'scripts', 'fixtures', 'm055-s04-hyperpush-root')
export const defaultRepoIdentityPath = path.join(repoRoot, 'scripts', 'lib', 'repo-identity.json')
export const DEFAULT_TIMEOUT_MS = 60_000
const DEFAULT_TEMP_PARENT = os.tmpdir()

const rootTemplateEntries = Object.freeze([
  Object.freeze({ source: 'README.md', target: 'README.md', kind: 'file' }),
  Object.freeze({ source: '.github/dependabot.yml', target: '.github/dependabot.yml', kind: 'file' }),
  Object.freeze({ source: '.github/workflows/deploy-landing.yml', target: '.github/workflows/deploy-landing.yml', kind: 'file' }),
  Object.freeze({ source: 'scripts/verify-landing-surface.sh', target: 'scripts/verify-landing-surface.sh', kind: 'file' }),
  Object.freeze({ source: 'scripts/verify-m051-s01.sh', target: 'scripts/verify-m051-s01.sh', kind: 'file' }),
])

const productSourceEntries = Object.freeze([
  Object.freeze({ source: 'mesher', target: 'mesher', kind: 'dir' }),
])

export const requiredStagedPaths = Object.freeze([
  'README.md',
  '.github/dependabot.yml',
  '.github/workflows/deploy-landing.yml',
  'scripts/verify-landing-surface.sh',
  'scripts/verify-m051-s01.sh',
  'mesher/README.md',
  'mesher/mesh.toml',
  'mesher/scripts/verify-maintainer-surface.sh',
  'mesher/landing/package.json',
  'mesher/landing/package-lock.json',
  'mesher/landing/.env.example',
  'mesher/landing/lib/external-links.ts',
])

const excludedSegmentNames = new Set([
  '.git',
  '.next',
  '.tmp',
  'node_modules',
  'test-results',
  'tmp-banners',
])

const excludedBasenames = new Set([
  '.DS_Store',
])

const excludedSuffixes = Object.freeze([
  '.tsbuildinfo',
])

const excludedExactPaths = new Set([
  'mesher/.env.local',
  'mesher/mesher',
  'mesher/mesher.ll',
  'mesher/landing/.env.local',
])

function normalizePath(value) {
  return path.resolve(value)
}

function toPortablePath(value) {
  return value.replace(/\\/g, '/')
}

function displayPath(rootDir, absolutePath) {
  const resolvedRoot = normalizePath(rootDir)
  const resolvedPath = normalizePath(absolutePath)
  const relative = path.relative(resolvedRoot, resolvedPath)
  if (relative === '') return '.'
  if (!relative.startsWith('..') && !path.isAbsolute(relative)) {
    return toPortablePath(relative)
  }
  return toPortablePath(resolvedPath)
}

function sha256(buffer) {
  return crypto.createHash('sha256').update(buffer).digest('hex')
}

function joinPaths(paths) {
  return paths.length > 0 ? paths.join(', ') : '-'
}

function ensureDirectory(absolutePath) {
  fs.mkdirSync(absolutePath, { recursive: true })
}

function assertNoSymlinksAlongPath(absolutePath, label) {
  const resolved = normalizePath(absolutePath)
  const parsed = path.parse(resolved)
  let current = parsed.root
  const parts = resolved.slice(parsed.root.length).split(path.sep).filter(Boolean)

  for (const part of parts) {
    current = path.join(current, part)
    if (!fs.existsSync(current)) continue
    if (fs.lstatSync(current).isSymbolicLink()) {
      throw new Error(`[m055-s04] refusing symlink path segment for ${label}: ${current}`)
    }
  }
}

function validateTimeoutMs(timeoutMs) {
  if (!Number.isInteger(timeoutMs) || timeoutMs <= 0) {
    throw new Error(`[m055-s04] timeout must be a positive integer; got ${JSON.stringify(timeoutMs)}`)
  }
}

function validateMode(mode) {
  if (mode !== 'write' && mode !== 'check') {
    throw new Error(`[m055-s04] mode must be exactly one of --write or --check; got ${JSON.stringify(mode)}`)
  }
}

function validateOutputRoot(rootDir, outputRoot) {
  const resolved = normalizePath(outputRoot)

  if (path.basename(resolved) !== 'hyperpush-mono') {
    throw new Error(`[m055-s04] output root must end with /hyperpush-mono; got ${displayPath(rootDir, resolved)}`)
  }
  return resolved
}

function readRepoIdentity(identityPath) {
  const resolvedIdentityPath = normalizePath(identityPath)
  if (!fs.existsSync(resolvedIdentityPath)) {
    throw new Error(`[m055-s04] missing repo identity: ${resolvedIdentityPath}`)
  }

  let parsed
  try {
    parsed = JSON.parse(fs.readFileSync(resolvedIdentityPath, 'utf8'))
  } catch (error) {
    throw new Error(`[m055-s04] invalid repo identity JSON at ${resolvedIdentityPath}: ${error instanceof Error ? error.message : String(error)}`)
  }

  return parsed
}

function resolveProductRoot({
  repoRoot,
  productRoot,
  repoIdentityPath = defaultRepoIdentityPath,
  env = process.env,
}) {
  let candidate
  let source

  if (productRoot) {
    candidate = productRoot
    source = 'option:productRoot'
  } else if (env.M055_HYPERPUSH_ROOT) {
    candidate = env.M055_HYPERPUSH_ROOT
    source = 'env:M055_HYPERPUSH_ROOT'
  } else {
    const repoIdentity = readRepoIdentity(repoIdentityPath)
    const productWorkspaceDir = repoIdentity?.productRepo?.workspaceDir
    const languageWorkspaceDir = repoIdentity?.languageRepo?.workspaceDir
    if (typeof productWorkspaceDir !== 'string' || productWorkspaceDir.length === 0) {
      throw new Error(`[m055-s04] repo identity is missing productRepo.workspaceDir: ${normalizePath(repoIdentityPath)}`)
    }
    if (typeof languageWorkspaceDir !== 'string' || languageWorkspaceDir.length === 0) {
      throw new Error(`[m055-s04] repo identity is missing languageRepo.workspaceDir: ${normalizePath(repoIdentityPath)}`)
    }
    candidate = path.join(repoRoot, '..', productWorkspaceDir)
    source = `blessed-sibling:${languageWorkspaceDir}->${productWorkspaceDir}`
  }

  const resolvedProductRoot = normalizePath(candidate)

  if (!fs.existsSync(resolvedProductRoot)) {
    throw new Error(`[m055-s04] missing sibling product repo root ${resolvedProductRoot} (source=${source})`)
  }
  if (!fs.statSync(resolvedProductRoot).isDirectory()) {
    throw new Error(`[m055-s04] sibling product repo root is not a directory: ${resolvedProductRoot} (source=${source})`)
  }

  const maintainerVerifierPath = path.join(resolvedProductRoot, 'mesher', 'scripts', 'verify-maintainer-surface.sh')
  if (!fs.existsSync(maintainerVerifierPath)) {
    throw new Error(`[m055-s04] malformed sibling product repo root ${resolvedProductRoot} (source=${source}); missing mesher/scripts/verify-maintainer-surface.sh`)
  }

  return {
    productRoot: resolvedProductRoot,
    productRootSource: source,
  }
}

function createTreeManifest(rootDir, label) {
  const resolvedRoot = normalizePath(rootDir)
  if (!fs.existsSync(resolvedRoot)) {
    throw new Error(`[m055-s04] ${label} is missing: ${resolvedRoot}`)
  }

  const stat = fs.lstatSync(resolvedRoot)
  if (stat.isSymbolicLink()) {
    throw new Error(`[m055-s04] ${label} is a symlink: ${resolvedRoot}`)
  }
  if (!stat.isDirectory()) {
    throw new Error(`[m055-s04] ${label} is not a directory: ${resolvedRoot}`)
  }

  const entries = []

  function walk(currentDir, relativeDir = '') {
    const children = fs.readdirSync(currentDir, { withFileTypes: true }).sort((left, right) => left.name.localeCompare(right.name))
    for (const child of children) {
      const absoluteChild = path.join(currentDir, child.name)
      const relativeChild = relativeDir ? `${relativeDir}/${child.name}` : child.name
      const childStat = fs.lstatSync(absoluteChild)
      if (childStat.isSymbolicLink()) {
        throw new Error(`[m055-s04] ${label} contains symlink: ${relativeChild}`)
      }
      if (childStat.isDirectory()) {
        entries.push({ path: relativeChild, kind: 'dir' })
        walk(absoluteChild, relativeChild)
        continue
      }
      if (childStat.isFile()) {
        const content = fs.readFileSync(absoluteChild)
        entries.push({
          path: relativeChild,
          kind: 'file',
          size: content.length,
          sha256: sha256(content),
        })
        continue
      }
      throw new Error(`[m055-s04] ${label} contains unsupported entry type: ${relativeChild}`)
    }
  }

  walk(resolvedRoot)

  const fileCount = entries.filter((entry) => entry.kind === 'file').length
  const dirCount = entries.filter((entry) => entry.kind === 'dir').length
  const fingerprint = sha256(Buffer.from(JSON.stringify(entries)))

  return {
    rootDir: resolvedRoot,
    entries,
    fileCount,
    dirCount,
    fingerprint,
  }
}

function relativeExclusionReason(relativePath) {
  const portable = toPortablePath(relativePath)
  const basename = path.posix.basename(portable)
  const segments = portable.split('/').filter(Boolean)

  if (excludedExactPaths.has(portable)) {
    return 'exact-path'
  }
  if (excludedBasenames.has(basename)) {
    return 'basename'
  }
  if (segments.some((segment) => excludedSegmentNames.has(segment))) {
    return 'segment'
  }
  if (excludedSuffixes.some((suffix) => portable.endsWith(suffix))) {
    return 'suffix'
  }
  return null
}

function assertWithinDeadline(startTimeMs, timeoutMs, relativePath) {
  if (Date.now() - startTimeMs > timeoutMs) {
    throw new Error(`[m055-s04] materializer timed out after ${timeoutMs}ms while staging ${relativePath}`)
  }
}

function requireSourcePath(rootDir, absolutePath, label) {
  if (!fs.existsSync(absolutePath)) {
    throw new Error(`[m055-s04] missing required ${label}: ${displayPath(rootDir, absolutePath)}`)
  }
}

function copyFilePreservingMode(sourcePath, targetPath) {
  ensureDirectory(path.dirname(targetPath))
  fs.copyFileSync(sourcePath, targetPath)
  const sourceMode = fs.statSync(sourcePath).mode
  fs.chmodSync(targetPath, sourceMode)
}

function copyDirectoryTree({
  rootDir,
  sourceDir,
  targetDir,
  relativeBase,
  startTimeMs,
  timeoutMs,
  excludedPaths,
}) {
  const sourceStat = fs.lstatSync(sourceDir)
  if (sourceStat.isSymbolicLink()) {
    throw new Error(`[m055-s04] refusing symlink source entry: ${displayPath(rootDir, sourceDir)}`)
  }
  if (!sourceStat.isDirectory()) {
    throw new Error(`[m055-s04] expected directory source entry: ${displayPath(rootDir, sourceDir)}`)
  }

  ensureDirectory(targetDir)

  const children = fs.readdirSync(sourceDir, { withFileTypes: true }).sort((left, right) => left.name.localeCompare(right.name))
  for (const child of children) {
    const childSource = path.join(sourceDir, child.name)
    const childRelative = relativeBase ? `${relativeBase}/${child.name}` : child.name
    const childTarget = path.join(targetDir, child.name)
    const exclusion = relativeExclusionReason(childRelative)
    if (exclusion) {
      excludedPaths.push({ path: childRelative, reason: exclusion })
      continue
    }
    assertWithinDeadline(startTimeMs, timeoutMs, childRelative)
    const childStat = fs.lstatSync(childSource)
    if (childStat.isSymbolicLink()) {
      throw new Error(`[m055-s04] refusing symlink source entry: ${displayPath(rootDir, childSource)}`)
    }
    if (childStat.isDirectory()) {
      copyDirectoryTree({
        rootDir,
        sourceDir: childSource,
        targetDir: childTarget,
        relativeBase: childRelative,
        startTimeMs,
        timeoutMs,
        excludedPaths,
      })
      continue
    }
    if (childStat.isFile()) {
      copyFilePreservingMode(childSource, childTarget)
      continue
    }
    throw new Error(`[m055-s04] unsupported source entry type: ${displayPath(rootDir, childSource)}`)
  }
}

function stageEntry({
  rootDir,
  sourceBase,
  entry,
  stageDir,
  startTimeMs,
  timeoutMs,
  excludedPaths,
}) {
  const sourcePath = path.join(sourceBase, entry.source)
  const targetPath = path.join(stageDir, entry.target)
  requireSourcePath(rootDir, sourcePath, `${entry.kind} source entry`)
  assertWithinDeadline(startTimeMs, timeoutMs, entry.target)

  if (entry.kind === 'file') {
    const exclusion = relativeExclusionReason(entry.target)
    if (exclusion) {
      excludedPaths.push({ path: entry.target, reason: exclusion })
      return
    }
    const stat = fs.lstatSync(sourcePath)
    if (stat.isSymbolicLink()) {
      throw new Error(`[m055-s04] refusing symlink source entry: ${displayPath(rootDir, sourcePath)}`)
    }
    if (!stat.isFile()) {
      throw new Error(`[m055-s04] expected file source entry: ${displayPath(rootDir, sourcePath)}`)
    }
    copyFilePreservingMode(sourcePath, targetPath)
    return
  }

  if (entry.kind === 'dir') {
    copyDirectoryTree({
      rootDir,
      sourceDir: sourcePath,
      targetDir: targetPath,
      relativeBase: entry.target,
      startTimeMs,
      timeoutMs,
      excludedPaths,
    })
    return
  }

  throw new Error(`[m055-s04] unsupported entry kind: ${entry.kind}`)
}

function validateStagedTree(rootDir, stageDir, manifest) {
  const errors = []

  for (const relativePath of requiredStagedPaths) {
    if (!fs.existsSync(path.join(stageDir, relativePath))) {
      errors.push(`missing required staged path ${relativePath}`)
    }
  }

  for (const entry of manifest.entries) {
    const exclusion = relativeExclusionReason(entry.path)
    if (exclusion) {
      errors.push(`excluded local-state path leaked into staged repo: ${entry.path}`)
    }
  }

  if (errors.length > 0) {
    throw new Error(`[m055-s04] staged repo validation failed\n${errors.map((error) => `- ${error}`).join('\n')}`)
  }
}

function createMetadataPaths(outputRoot) {
  const workspaceRoot = path.dirname(outputRoot)
  const basename = path.basename(outputRoot)
  return {
    workspaceRoot,
    manifestPath: path.join(workspaceRoot, `${basename}.manifest.json`),
    summaryPath: path.join(workspaceRoot, `${basename}.stage.json`),
  }
}

function writeJson(pathname, data) {
  ensureDirectory(path.dirname(pathname))
  fs.writeFileSync(pathname, `${JSON.stringify(data, null, 2)}\n`)
}

function replaceOutputRoot({ outputRoot, stageDir }) {
  const backupDir = `${outputRoot}.backup-${process.pid}-${Date.now()}-${crypto.randomBytes(4).toString('hex')}`
  let movedExisting = false

  try {
    if (fs.existsSync(outputRoot)) {
      fs.renameSync(outputRoot, backupDir)
      movedExisting = true
    }
    fs.renameSync(stageDir, outputRoot)
    if (movedExisting && fs.existsSync(backupDir)) {
      fs.rmSync(backupDir, { recursive: true, force: true })
    }
  } catch (error) {
    if (!fs.existsSync(outputRoot) && fs.existsSync(stageDir)) {
      // leave stageDir in place for inspection; caller will surface its path
    }
    if (!fs.existsSync(outputRoot) && movedExisting && fs.existsSync(backupDir)) {
      fs.renameSync(backupDir, outputRoot)
    }
    throw error
  }
}

function createSuccessLines(summary) {
  return [
    `[m055-s04] phase=materialize mode=${summary.mode} result=pass output_root=${displayPath(summary.repoRoot, summary.outputRoot)} files=${summary.manifest.fileCount} dirs=${summary.manifest.dirCount} fingerprint=${summary.manifest.fingerprint}`,
    `[m055-s04] phase=metadata summary=${displayPath(summary.repoRoot, summary.summaryPath)} manifest=${displayPath(summary.repoRoot, summary.manifestPath)} product_root=${displayPath(summary.repoRoot, summary.productRoot)} product_root_source=${summary.productRootSource} source_entries=${summary.sourceEntries.length} excluded_paths=${summary.excludedPaths.length}`,
  ]
}

export function materializeHyperpushMono({
  mode,
  repoRoot: rootOverride = repoRoot,
  productRoot,
  templatesRoot = defaultTemplatesRoot,
  repoIdentityPath = defaultRepoIdentityPath,
  outputRoot = defaultOutputRoot,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  tempParent = DEFAULT_TEMP_PARENT,
  allowCustomOutputRoot = false,
  keepFailedStage = true,
} = {}) {
  validateMode(mode)
  validateTimeoutMs(timeoutMs)

  const resolvedRepoRoot = normalizePath(rootOverride)
  const resolvedTemplatesRoot = normalizePath(templatesRoot)
  const resolvedRepoIdentityPath = normalizePath(repoIdentityPath)
  const { productRoot: resolvedProductRoot, productRootSource } = resolveProductRoot({
    repoRoot: resolvedRepoRoot,
    productRoot,
    repoIdentityPath: resolvedRepoIdentityPath,
  })
  const resolvedOutputRoot = validateOutputRoot(resolvedRepoRoot, outputRoot)
  const resolvedTempParent = normalizePath(tempParent)
  ensureDirectory(resolvedTempParent)
  ensureDirectory(path.dirname(resolvedOutputRoot))

  const startTimeMs = Date.now()
  const stageParent = path.dirname(resolvedOutputRoot)
  const stageDir = fs.mkdtempSync(path.join(stageParent, '.hyperpush-mono.stage-'))
  const excludedPaths = []
  const sourceEntries = [
    ...rootTemplateEntries.map((entry) => ({ sourceBase: resolvedTemplatesRoot, ...entry })),
    ...productSourceEntries.map((entry) => ({ sourceBase: resolvedProductRoot, ...entry })),
  ]

  try {
    for (const entry of sourceEntries) {
      stageEntry({
        rootDir: resolvedRepoRoot,
        sourceBase: entry.sourceBase,
        entry,
        stageDir,
        startTimeMs,
        timeoutMs,
        excludedPaths,
      })
    }

    const manifest = createTreeManifest(stageDir, 'staged hyperpush-mono repo')
    validateStagedTree(resolvedRepoRoot, stageDir, manifest)
    replaceOutputRoot({ outputRoot: resolvedOutputRoot, stageDir })

    const { manifestPath, summaryPath } = createMetadataPaths(resolvedOutputRoot)
    const outputManifest = createTreeManifest(resolvedOutputRoot, 'materialized hyperpush-mono repo')
    const summary = {
      version: 'm055-s04-materialize-v1',
      generatedAt: new Date().toISOString(),
      mode,
      repoRoot: resolvedRepoRoot,
      templatesRoot: resolvedTemplatesRoot,
      repoIdentityPath: resolvedRepoIdentityPath,
      productRoot: resolvedProductRoot,
      productRootSource,
      outputRoot: resolvedOutputRoot,
      manifestPath,
      summaryPath,
      timeoutMs,
      requiredStagedPaths,
      sourceEntries: sourceEntries.map((entry) => ({
        kind: entry.kind,
        sourceBase: displayPath(resolvedRepoRoot, entry.sourceBase),
        source: entry.source,
        target: entry.target,
      })),
      excludedRules: {
        segmentNames: [...excludedSegmentNames].sort(),
        basenames: [...excludedBasenames].sort(),
        suffixes: [...excludedSuffixes],
        exactPaths: [...excludedExactPaths].sort(),
      },
      excludedPaths,
      manifest: outputManifest,
    }

    writeJson(manifestPath, outputManifest)
    writeJson(summaryPath, summary)

    return {
      ...summary,
      lines: createSuccessLines(summary),
    }
  } catch (error) {
    if (!keepFailedStage && fs.existsSync(stageDir)) {
      fs.rmSync(stageDir, { recursive: true, force: true })
    }
    if (error instanceof Error) {
      error.message = `${error.message}\n[m055-s04] stage=${stageDir}`
    }
    throw error
  }
}

export function parseArgs(argv) {
  const options = {}

  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index]
    if (arg === '--write') {
      if (options.mode) {
        throw new Error('[m055-s04] specify only one of --write or --check')
      }
      options.mode = 'write'
      continue
    }
    if (arg === '--check') {
      if (options.mode) {
        throw new Error('[m055-s04] specify only one of --write or --check')
      }
      options.mode = 'check'
      continue
    }
    if (arg === '--output-root') {
      const value = argv[index + 1]
      if (!value || value.startsWith('--')) {
        throw new Error('[m055-s04] --output-root requires a value')
      }
      options.outputRoot = value
      index += 1
      continue
    }
    if (arg === '--temp-parent') {
      const value = argv[index + 1]
      if (!value || value.startsWith('--')) {
        throw new Error('[m055-s04] --temp-parent requires a value')
      }
      options.tempParent = value
      index += 1
      continue
    }
    if (arg === '--timeout-ms') {
      const value = argv[index + 1]
      if (!value || value.startsWith('--')) {
        throw new Error('[m055-s04] --timeout-ms requires a value')
      }
      const parsed = Number.parseInt(value, 10)
      if (!Number.isInteger(parsed) || parsed <= 0) {
        throw new Error(`[m055-s04] --timeout-ms must be a positive integer; got ${JSON.stringify(value)}`)
      }
      options.timeoutMs = parsed
      index += 1
      continue
    }
    throw new Error(`[m055-s04] unknown argument: ${arg}`)
  }

  if (!options.mode) {
    throw new Error('[m055-s04] expected exactly one mode: --write or --check')
  }
  if (options.mode === 'check' && options.outputRoot) {
    throw new Error('[m055-s04] --check always refreshes the standard staged repo; do not pass --output-root')
  }

  return options
}

async function main() {
  const options = parseArgs(process.argv.slice(2))
  const summary = materializeHyperpushMono(options)
  for (const line of summary.lines) {
    process.stdout.write(`${line}\n`)
  }
}

const isEntrypoint = process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)
if (isEntrypoint) {
  main().catch((error) => {
    const message = error instanceof Error ? error.message : String(error)
    process.stderr.write(`${message}\n`)
    process.exitCode = 1
  })
}
