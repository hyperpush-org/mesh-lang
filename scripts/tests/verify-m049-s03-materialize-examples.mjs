import crypto from 'node:crypto'
import fs from 'node:fs'
import os from 'node:os'
import path from 'node:path'
import { spawnSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'

const scriptDir = path.dirname(fileURLToPath(import.meta.url))
export const repoRoot = path.resolve(scriptDir, '..', '..')
export const defaultExamplesRoot = path.join(repoRoot, 'examples')
export const DEFAULT_TIMEOUT_MS = 120_000
const DEFAULT_TEMP_PARENT = os.tmpdir()
const TODO_POSTGRES_MIGRATION_FILENAME = '20260402120000_create_todos.mpl'
const defaultMeshcBin = path.join(
  repoRoot,
  'target',
  'debug',
  process.platform === 'win32' ? 'meshc.exe' : 'meshc',
)

export const exampleDefinitions = Object.freeze([
  Object.freeze({
    name: 'todo-sqlite',
    db: 'sqlite',
    requiredPaths: Object.freeze([
      'mesh.toml',
      'main.mpl',
      'config.mpl',
      'README.md',
      'Dockerfile',
      '.dockerignore',
      'api/health.mpl',
      'api/router.mpl',
      'api/todos.mpl',
      'runtime/registry.mpl',
      'services/rate_limiter.mpl',
      'storage/todos.mpl',
      'types/todo.mpl',
      'tests/config.test.mpl',
      'tests/storage.test.mpl',
    ]),
  }),
  Object.freeze({
    name: 'todo-postgres',
    db: 'postgres',
    requiredPaths: Object.freeze([
      'mesh.toml',
      'main.mpl',
      'work.mpl',
      'config.mpl',
      'README.md',
      'Dockerfile',
      '.dockerignore',
      '.env.example',
      'api/health.mpl',
      'api/router.mpl',
      'api/todos.mpl',
      'runtime/registry.mpl',
      'services/rate_limiter.mpl',
      'storage/todos.mpl',
      'types/todo.mpl',
      'tests/config.test.mpl',
      'scripts/stage-deploy.sh',
      'scripts/apply-deploy-migrations.sh',
      'scripts/deploy-smoke.sh',
      'deploy/todo-postgres.up.sql',
      `migrations/${TODO_POSTGRES_MIGRATION_FILENAME}`,
    ]),
  }),
])

function normalizePath(value) {
  return path.resolve(value)
}

function toPortablePath(value) {
  return value.replace(/\\/g, '/')
}

function displayPath(absolutePath) {
  const resolved = normalizePath(absolutePath)
  const relative = path.relative(repoRoot, resolved)
  if (relative === '') return '.'
  if (!relative.startsWith('..') && !path.isAbsolute(relative)) {
    return toPortablePath(relative)
  }
  return toPortablePath(resolved)
}

function sha256(buffer) {
  return crypto.createHash('sha256').update(buffer).digest('hex')
}

function joinPaths(paths) {
  return paths.length > 0 ? paths.join(', ') : '-'
}

function zeroDiff() {
  return {
    missing: [],
    extra: [],
    changed: [],
  }
}

function hasDiff(diff) {
  return diff.missing.length > 0 || diff.extra.length > 0 || diff.changed.length > 0
}

function assertNoSymlinksAlongPath(absolutePath) {
  const resolved = normalizePath(absolutePath)
  const parsed = path.parse(resolved)
  let current = parsed.root
  const parts = resolved.slice(parsed.root.length).split(path.sep).filter(Boolean)

  for (const part of parts) {
    current = path.join(current, part)
    if (!fs.existsSync(current)) continue
    if (fs.lstatSync(current).isSymbolicLink()) {
      throw new Error(`[m049-s03] refusing symlink path segment: ${displayPath(current)}`)
    }
  }
}

function validateExamplesRoot(examplesRoot) {
  const resolved = normalizePath(examplesRoot)
  if (path.basename(resolved) !== 'examples') {
    throw new Error(
      `[m049-s03] examples root must end with /examples, got ${displayPath(resolved)}`,
    )
  }
  if (fs.existsSync(resolved)) {
    const stat = fs.lstatSync(resolved)
    if (stat.isSymbolicLink()) {
      throw new Error(`[m049-s03] refusing symlink examples root: ${displayPath(resolved)}`)
    }
    if (!stat.isDirectory()) {
      throw new Error(`[m049-s03] examples root is not a directory: ${displayPath(resolved)}`)
    }
  }
  return resolved
}

function validateMeshcBin(candidate, sourceLabel) {
  const resolved = normalizePath(candidate)
  if (!fs.existsSync(resolved)) {
    throw new Error(`[m049-s03] ${sourceLabel} does not exist: ${displayPath(resolved)}`)
  }
  const stat = fs.statSync(resolved)
  if (!stat.isFile()) {
    throw new Error(`[m049-s03] ${sourceLabel} is not a file: ${displayPath(resolved)}`)
  }
  return resolved
}

function resolveMeshcBin({ meshcBin } = {}) {
  if (meshcBin) {
    return validateMeshcBin(meshcBin, '--meshc-bin')
  }
  if (process.env.MESHC_BIN) {
    return validateMeshcBin(process.env.MESHC_BIN, 'MESHC_BIN')
  }
  return validateMeshcBin(defaultMeshcBin, 'default meshc binary')
}

function formatCommand(program, args) {
  return [program, ...args].map((part) => JSON.stringify(part)).join(' ')
}

function ensureDirectory(absolutePath) {
  fs.mkdirSync(absolutePath, { recursive: true })
}

function createTreeManifest(rootDir, label) {
  const resolvedRoot = normalizePath(rootDir)
  if (!fs.existsSync(resolvedRoot)) {
    throw new Error(`[m049-s03] ${label} is missing: ${displayPath(resolvedRoot)}`)
  }

  const rootStat = fs.lstatSync(resolvedRoot)
  if (rootStat.isSymbolicLink()) {
    throw new Error(`[m049-s03] ${label} is a symlink: ${displayPath(resolvedRoot)}`)
  }
  if (!rootStat.isDirectory()) {
    throw new Error(`[m049-s03] ${label} is not a directory: ${displayPath(resolvedRoot)}`)
  }

  const entries = []

  function walk(currentDir, relativeDir = '') {
    const children = fs.readdirSync(currentDir, { withFileTypes: true }).sort((left, right) => left.name.localeCompare(right.name))

    for (const child of children) {
      const absoluteChild = path.join(currentDir, child.name)
      const relativeChild = relativeDir ? `${relativeDir}/${child.name}` : child.name
      const childStats = fs.lstatSync(absoluteChild)

      if (childStats.isSymbolicLink()) {
        throw new Error(`[m049-s03] ${label} contains symlink: ${relativeChild}`)
      }
      if (childStats.isDirectory()) {
        entries.push({ path: relativeChild, kind: 'dir' })
        walk(absoluteChild, relativeChild)
        continue
      }
      if (childStats.isFile()) {
        const content = fs.readFileSync(absoluteChild)
        entries.push({
          path: relativeChild,
          kind: 'file',
          size: content.length,
          sha256: sha256(content),
        })
        continue
      }
      throw new Error(`[m049-s03] ${label} contains unsupported entry type: ${relativeChild}`)
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

function compareManifests(targetManifest, generatedManifest) {
  const targetEntries = new Map(targetManifest.entries.map((entry) => [entry.path, entry]))
  const generatedEntries = new Map(generatedManifest.entries.map((entry) => [entry.path, entry]))
  const allPaths = [...new Set([...targetEntries.keys(), ...generatedEntries.keys()])].sort()
  const diff = zeroDiff()

  for (const relativePath of allPaths) {
    const targetEntry = targetEntries.get(relativePath)
    const generatedEntry = generatedEntries.get(relativePath)

    if (!targetEntry) {
      diff.missing.push(relativePath)
      continue
    }
    if (!generatedEntry) {
      diff.extra.push(relativePath)
      continue
    }
    if (targetEntry.kind !== generatedEntry.kind) {
      diff.changed.push(relativePath)
      continue
    }
    if (targetEntry.kind === 'file') {
      if (targetEntry.size !== generatedEntry.size || targetEntry.sha256 !== generatedEntry.sha256) {
        diff.changed.push(relativePath)
      }
    }
  }

  return diff
}

function validateGeneratedExampleTree(generatedDir, example) {
  const resolved = normalizePath(generatedDir)
  if (!fs.existsSync(resolved)) {
    throw new Error(`[m049-s03] generated ${example.name} tree is missing: ${displayPath(resolved)}`)
  }
  const stat = fs.lstatSync(resolved)
  if (stat.isSymbolicLink()) {
    throw new Error(`[m049-s03] generated ${example.name} tree is a symlink: ${displayPath(resolved)}`)
  }
  if (!stat.isDirectory()) {
    throw new Error(`[m049-s03] generated ${example.name} tree is not a directory: ${displayPath(resolved)}`)
  }

  const missing = example.requiredPaths.filter((relativePath) => !fs.existsSync(path.join(resolved, relativePath)))
  if (missing.length > 0) {
    throw new Error(
      `[m049-s03] generated ${example.name} tree is missing expected scaffold files: ${joinPaths(missing)}`,
    )
  }
}

function validateTargetExampleTree(targetDir, example, { allowAbsent, requireComplete }) {
  const resolved = normalizePath(targetDir)

  if (!fs.existsSync(resolved)) {
    if (allowAbsent) {
      return { exists: false, targetDir: resolved }
    }
    return {
      exists: false,
      targetDir: resolved,
      error: `[m049-s03] target ${example.name} is missing: ${displayPath(resolved)}`,
    }
  }

  const stat = fs.lstatSync(resolved)
  if (stat.isSymbolicLink()) {
    return {
      exists: true,
      targetDir: resolved,
      error: `[m049-s03] refusing symlink target directory: ${displayPath(resolved)}`,
    }
  }
  if (!stat.isDirectory()) {
    return {
      exists: true,
      targetDir: resolved,
      error: `[m049-s03] target ${displayPath(resolved)} is not a directory`,
    }
  }

  const manifest = createTreeManifest(resolved, `target ${example.name}`)
  if (requireComplete) {
    const missing = example.requiredPaths.filter((relativePath) => !fs.existsSync(path.join(resolved, relativePath)))
    if (missing.length > 0) {
      return {
        exists: true,
        targetDir: resolved,
        error: `[m049-s03] target ${displayPath(resolved)} is malformed for ${example.name}; missing required files: ${joinPaths(missing)}`,
      }
    }
  }

  return {
    exists: true,
    targetDir: resolved,
    manifest,
  }
}

function prepareStageDir(parentDir, basename, suffix) {
  const unique = `${suffix}-${process.pid}-${Date.now()}-${crypto.randomBytes(4).toString('hex')}`
  return path.join(parentDir, `.${basename}.${unique}`)
}

function prepareWritePlan(generatedExamples) {
  const writePlan = []

  for (const entry of generatedExamples) {
    const targetParent = path.dirname(entry.targetDir)
    ensureDirectory(targetParent)
    const targetName = path.basename(entry.targetDir)
    const stagingDir = prepareStageDir(targetParent, targetName, 'staging')
    const backupDir = prepareStageDir(targetParent, targetName, 'backup')

    fs.cpSync(entry.generatedDir, stagingDir, {
      recursive: true,
      force: false,
      errorOnExist: true,
    })

    writePlan.push({
      ...entry,
      stagingDir,
      backupDir,
    })
  }

  return writePlan
}

function cleanupWritePlan(writePlan) {
  for (const entry of writePlan) {
    for (const tempDir of [entry.stagingDir, entry.backupDir]) {
      if (tempDir && fs.existsSync(tempDir)) {
        fs.rmSync(tempDir, { recursive: true, force: true })
      }
    }
  }
}

function applyWritePlan(writePlan) {
  const applied = new Set()

  try {
    for (const entry of writePlan) {
      if (entry.targetExists) {
        fs.renameSync(entry.targetDir, entry.backupDir)
      }
      fs.renameSync(entry.stagingDir, entry.targetDir)
      applied.add(entry)
    }
  } catch (error) {
    for (const entry of [...writePlan].reverse()) {
      if (applied.has(entry) && fs.existsSync(entry.targetDir)) {
        fs.rmSync(entry.targetDir, { recursive: true, force: true })
      }
      if (fs.existsSync(entry.backupDir)) {
        fs.renameSync(entry.backupDir, entry.targetDir)
      }
      if (fs.existsSync(entry.stagingDir)) {
        fs.rmSync(entry.stagingDir, { recursive: true, force: true })
      }
    }
    throw error
  }

  for (const entry of writePlan) {
    if (fs.existsSync(entry.backupDir)) {
      fs.rmSync(entry.backupDir, { recursive: true, force: true })
    }
  }
}

function runMeshcInit({ meshcBin, workspaceDir, example, timeoutMs }) {
  ensureDirectory(workspaceDir)
  const args = ['init', '--template', 'todo-api', '--db', example.db, example.name]
  const commandText = formatCommand(meshcBin, args)
  const result = spawnSync(meshcBin, args, {
    cwd: workspaceDir,
    encoding: 'utf8',
    timeout: timeoutMs,
  })

  if (result.error) {
    if (result.error.code === 'ETIMEDOUT') {
      throw new Error(
        `[m049-s03] meshc init timed out command=${commandText} temp=${displayPath(workspaceDir)} timeout_ms=${timeoutMs}`,
      )
    }
    throw new Error(
      `[m049-s03] failed to run meshc init command=${commandText} temp=${displayPath(workspaceDir)} error=${result.error.message}`,
    )
  }

  if (result.status !== 0) {
    const stdout = result.stdout || ''
    const stderr = result.stderr || ''
    throw new Error(
      `[m049-s03] meshc init failed command=${commandText} temp=${displayPath(workspaceDir)} exit=${result.status}\nstdout:\n${stdout}\nstderr:\n${stderr}`,
    )
  }

  const generatedDir = path.join(workspaceDir, example.name)
  validateGeneratedExampleTree(generatedDir, example)

  return {
    generatedDir,
    commandText,
    stdout: result.stdout || '',
    stderr: result.stderr || '',
  }
}

function formatStructuralIssues(issues) {
  return `[m049-s03] validation failed\n${issues.map((issue) => `- ${issue}`).join('\n')}`
}

function formatDiffIssue(entry) {
  return `example=${entry.example.name} target=${displayPath(entry.targetDir)} missing=${joinPaths(entry.diff.missing)} extra=${joinPaths(entry.diff.extra)} changed=${joinPaths(entry.diff.changed)}`
}

function createSuccessLines(summary) {
  const lines = []

  for (const entry of summary.examples) {
    lines.push(
      `[m049-s03] phase=manifest example=${entry.example.name} db=${entry.example.db} files=${entry.generatedManifest.fileCount} dirs=${entry.generatedManifest.dirCount} fingerprint=${entry.generatedManifest.fingerprint}`,
    )

    if (summary.mode === 'check') {
      lines.push(
        `[m049-s03] phase=check example=${entry.example.name} target=${displayPath(entry.targetDir)} result=match missing=- extra=- changed=-`,
      )
      continue
    }

    const priorState = entry.targetExists ? (hasDiff(entry.priorDiff) ? 'replaced' : 'unchanged') : 'created'
    lines.push(
      `[m049-s03] phase=write example=${entry.example.name} target=${displayPath(entry.targetDir)} prior=${priorState} missing=${joinPaths(entry.priorDiff.missing)} extra=${joinPaths(entry.priorDiff.extra)} changed=${joinPaths(entry.priorDiff.changed)} post_write=match`,
    )
  }

  lines.push(
    `[m049-s03] phase=materialize mode=${summary.mode} result=pass examples=${summary.examples.length} examples_root=${displayPath(summary.examplesRoot)}`,
  )

  return lines
}

function validateMode(mode) {
  if (mode !== 'write' && mode !== 'check') {
    throw new Error(`[m049-s03] mode must be exactly one of --write or --check; got ${JSON.stringify(mode)}`)
  }
}

export function materializeExamples({
  mode,
  examplesRoot = defaultExamplesRoot,
  meshcBin,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  tempParent = DEFAULT_TEMP_PARENT,
  keepTemp = false,
} = {}) {
  validateMode(mode)

  const resolvedExamplesRoot = validateExamplesRoot(examplesRoot)
  const resolvedMeshcBin = resolveMeshcBin({ meshcBin })
  const resolvedTempParent = normalizePath(tempParent)
  ensureDirectory(resolvedTempParent)

  const sessionDir = fs.mkdtempSync(path.join(resolvedTempParent, 'm049-s03-materialize-'))
  let shouldCleanup = !keepTemp

  try {
    const generatedExamples = exampleDefinitions.map((example) => {
      const workspaceDir = path.join(sessionDir, 'workspace', example.name)
      const generation = runMeshcInit({
        meshcBin: resolvedMeshcBin,
        workspaceDir,
        example,
        timeoutMs,
      })
      const generatedManifest = createTreeManifest(generation.generatedDir, `generated ${example.name}`)
      return {
        example,
        generatedDir: generation.generatedDir,
        generatedManifest,
        commandText: generation.commandText,
        stdout: generation.stdout,
        stderr: generation.stderr,
        targetDir: path.join(resolvedExamplesRoot, example.name),
      }
    })

    const targetChecks = generatedExamples.map((entry) => ({
      entry,
      check: validateTargetExampleTree(entry.targetDir, entry.example, {
        allowAbsent: mode === 'write',
        requireComplete: mode === 'write',
      }),
    }))

    const structuralIssues = targetChecks
      .filter(({ check }) => typeof check.error === 'string')
      .map(({ check }) => check.error)
    if (structuralIssues.length > 0) {
      shouldCleanup = false
      throw new Error(formatStructuralIssues(structuralIssues))
    }

    if (mode === 'check') {
      const diffIssues = []
      const summary = {
        mode,
        examplesRoot: resolvedExamplesRoot,
        meshcBin: resolvedMeshcBin,
        sessionDir,
        examples: [],
      }

      for (const { entry, check } of targetChecks) {
        const diff = compareManifests(check.manifest, entry.generatedManifest)
        summary.examples.push({
          ...entry,
          targetExists: true,
          priorDiff: diff,
          targetManifest: check.manifest,
        })
        if (hasDiff(diff)) {
          diffIssues.push(formatDiffIssue({ example: entry.example, targetDir: entry.targetDir, diff }))
        }
      }

      if (diffIssues.length > 0) {
        shouldCleanup = false
        throw new Error(`[m049-s03] check failed\n${diffIssues.map((issue) => `- ${issue}`).join('\n')}`)
      }

      summary.lines = createSuccessLines(summary)
      return summary
    }

    const writeSummary = {
      mode,
      examplesRoot: resolvedExamplesRoot,
      meshcBin: resolvedMeshcBin,
      sessionDir,
      examples: targetChecks.map(({ entry, check }) => ({
        ...entry,
        targetExists: check.exists,
        targetManifest: check.manifest ?? null,
        priorDiff: check.exists ? compareManifests(check.manifest, entry.generatedManifest) : zeroDiff(),
      })),
    }

    ensureDirectory(resolvedExamplesRoot)
    const writePlan = prepareWritePlan(writeSummary.examples)

    try {
      applyWritePlan(writePlan)
    } finally {
      cleanupWritePlan(writePlan)
    }

    const postWriteIssues = []
    for (const entry of writeSummary.examples) {
      const targetCheck = validateTargetExampleTree(entry.targetDir, entry.example, {
        allowAbsent: false,
        requireComplete: true,
      })
      if (targetCheck.error) {
        postWriteIssues.push(targetCheck.error)
        continue
      }
      const postWriteDiff = compareManifests(targetCheck.manifest, entry.generatedManifest)
      if (hasDiff(postWriteDiff)) {
        postWriteIssues.push(formatDiffIssue({ example: entry.example, targetDir: entry.targetDir, diff: postWriteDiff }))
      }
      entry.targetManifest = targetCheck.manifest
    }

    if (postWriteIssues.length > 0) {
      shouldCleanup = false
      throw new Error(`[m049-s03] write verification failed\n${postWriteIssues.map((issue) => `- ${issue}`).join('\n')}`)
    }

    writeSummary.lines = createSuccessLines(writeSummary)
    return writeSummary
  } catch (error) {
    shouldCleanup = false
    if (error instanceof Error) {
      error.message = `${error.message}\n[m049-s03] temp=${displayPath(sessionDir)}`
    }
    throw error
  } finally {
    if (shouldCleanup) {
      fs.rmSync(sessionDir, { recursive: true, force: true })
    }
  }
}

export function parseArgs(argv) {
  const options = {}

  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index]
    if (arg === '--write') {
      if (options.mode) {
        throw new Error('[m049-s03] specify only one of --write or --check')
      }
      options.mode = 'write'
      continue
    }
    if (arg === '--check') {
      if (options.mode) {
        throw new Error('[m049-s03] specify only one of --write or --check')
      }
      options.mode = 'check'
      continue
    }
    if (arg === '--meshc-bin') {
      const value = argv[index + 1]
      if (!value || value.startsWith('--')) {
        throw new Error('[m049-s03] --meshc-bin requires a value')
      }
      options.meshcBin = value
      index += 1
      continue
    }
    if (arg === '--examples-root') {
      const value = argv[index + 1]
      if (!value || value.startsWith('--')) {
        throw new Error('[m049-s03] --examples-root requires a value')
      }
      options.examplesRoot = value
      index += 1
      continue
    }
    if (arg === '--temp-parent') {
      const value = argv[index + 1]
      if (!value || value.startsWith('--')) {
        throw new Error('[m049-s03] --temp-parent requires a value')
      }
      options.tempParent = value
      index += 1
      continue
    }
    throw new Error(`[m049-s03] unknown argument: ${arg}`)
  }

  if (!options.mode) {
    throw new Error('[m049-s03] expected exactly one mode: --write or --check')
  }

  return options
}

async function main() {
  const options = parseArgs(process.argv.slice(2))
  const summary = materializeExamples(options)
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
