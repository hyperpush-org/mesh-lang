import test from 'node:test'
import assert from 'node:assert/strict'
import fs from 'node:fs'
import os from 'node:os'
import path from 'node:path'
import { spawnSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'

import {
  defaultOutputRoot,
  materializeHyperpushMono,
  parseArgs,
  repoRoot,
} from '../materialize-hyperpush-mono.mjs'

const scriptDir = path.dirname(fileURLToPath(import.meta.url))
const scriptPath = path.join(repoRoot, 'scripts', 'materialize-hyperpush-mono.mjs')
const templatesRoot = path.join(repoRoot, 'scripts', 'fixtures', 'm055-s04-hyperpush-root')

function mkTmpDir(t, prefix) {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), prefix))
  t.after(() => fs.rmSync(dir, { recursive: true, force: true }))
  return dir
}

function writeFiles(rootDir, files) {
  for (const [relativePath, content] of Object.entries(files)) {
    const absolutePath = path.join(rootDir, relativePath)
    fs.mkdirSync(path.dirname(absolutePath), { recursive: true })
    fs.writeFileSync(absolutePath, content)
  }
}

function copyFixtureTemplates(targetRoot) {
  const targetTemplates = path.join(targetRoot, 'scripts', 'fixtures', 'm055-s04-hyperpush-root')
  fs.cpSync(templatesRoot, targetTemplates, { recursive: true })
}

function createFakeWorkspace(t) {
  const workspaceRoot = mkTmpDir(t, 'm055-s04-materialize-')
  const meshLangRoot = path.join(workspaceRoot, 'mesh-lang')
  const productRoot = path.join(workspaceRoot, 'hyperpush-mono')
  fs.mkdirSync(meshLangRoot, { recursive: true })
  fs.mkdirSync(productRoot, { recursive: true })

  copyFixtureTemplates(meshLangRoot)
  writeFiles(productRoot, {
    'mesher/README.md': '# Mesher\n\nUse `bash scripts/verify-maintainer-surface.sh`.\n',
    'mesher/mesh.toml': '[package]\nname = "mesher"\nversion = "0.1.0"\n',
    'mesher/scripts/verify-maintainer-surface.sh': '#!/usr/bin/env bash\nset -euo pipefail\necho "verify-maintainer-surface: ok"\n',
    'mesher/landing/package.json': '{"name":"landing","private":true}\n',
    'mesher/landing/package-lock.json': '{"name":"landing","lockfileVersion":3}\n',
    'mesher/landing/.env.example': 'NEXT_PUBLIC_DISCORD_URL=https://discord.gg/6SRhbZw7ZG\n',
    'mesher/landing/lib/external-links.ts': 'const PRODUCT_REPO_URL = "https://github.com/hyperpush-org/hyperpush-mono"\nconst PRODUCT_REPO_DISPLAY = "github.com/hyperpush-org/hyperpush-mono"\nexport const GITHUB_URL = PRODUCT_REPO_URL\nexport const GITHUB_DISPLAY = PRODUCT_REPO_DISPLAY\nexport const DISCORD_URL = process.env.NEXT_PUBLIC_DISCORD_URL ?? "https://discord.gg/6SRhbZw7ZG"\n',
    'mesher/landing/node_modules/leak.js': 'leak\n',
    'mesher/landing/.next/cache.txt': 'leak\n',
    'mesher/landing/test-results/report.json': '{}\n',
    'mesher/landing/tmp-banners/banner.html': '<html></html>\n',
    'mesher/landing/tsconfig.tsbuildinfo': '{}\n',
    'mesher/.env.local': 'DATABASE_URL=postgres://secret\n',
    'mesher/mesher': 'binary\n',
    'mesher/mesher.ll': 'ir\n',
    'mesher/.git/config': '[core]\nrepositoryformatversion = 0\n',
  })
  fs.chmodSync(path.join(productRoot, 'mesher', 'scripts', 'verify-maintainer-surface.sh'), 0o755)

  return { workspaceRoot, meshLangRoot, productRoot }
}

function readJson(pathname) {
  return JSON.parse(fs.readFileSync(pathname, 'utf8'))
}

function runCli(args, options = {}) {
  return spawnSync(process.execPath, [scriptPath, ...args], {
    cwd: repoRoot,
    encoding: 'utf8',
    ...options,
  })
}

test('materializeHyperpushMono writes required root surfaces, excludes local state, and records metadata', (t) => {
  const { meshLangRoot, productRoot } = createFakeWorkspace(t)
  const outputRoot = path.join(meshLangRoot, '.tmp', 'workspace', 'hyperpush-mono')

  const summary = materializeHyperpushMono({
    mode: 'write',
    repoRoot: meshLangRoot,
    productRoot,
    templatesRoot: path.join(meshLangRoot, 'scripts', 'fixtures', 'm055-s04-hyperpush-root'),
    outputRoot,
    allowCustomOutputRoot: true,
  })

  assert.ok(fs.existsSync(path.join(outputRoot, 'README.md')))
  assert.ok(fs.existsSync(path.join(outputRoot, '.github', 'workflows', 'deploy-landing.yml')))
  assert.ok(fs.existsSync(path.join(outputRoot, '.github', 'dependabot.yml')))
  assert.ok(fs.existsSync(path.join(outputRoot, 'scripts', 'verify-landing-surface.sh')))
  assert.ok(fs.existsSync(path.join(outputRoot, 'scripts', 'verify-m051-s01.sh')))
  assert.ok(fs.existsSync(path.join(outputRoot, 'mesher', 'README.md')))
  assert.ok(fs.existsSync(path.join(outputRoot, 'mesher', 'scripts', 'verify-maintainer-surface.sh')))
  assert.ok(fs.existsSync(path.join(outputRoot, 'mesher', 'landing', 'package.json')))
  assert.ok(fs.existsSync(path.join(outputRoot, 'mesher', 'landing', 'package-lock.json')))
  assert.ok(fs.existsSync(path.join(outputRoot, 'mesher', 'landing', '.env.example')))
  assert.ok(fs.existsSync(path.join(outputRoot, 'mesher', 'landing', 'lib', 'external-links.ts')))

  for (const leakedRelativePath of [
    'mesher/.env.local',
    'mesher/mesher',
    'mesher/mesher.ll',
    'mesher/.git',
    'mesher/landing/node_modules',
    'mesher/landing/.next',
    'mesher/landing/test-results',
    'mesher/landing/tmp-banners',
    'mesher/landing/tsconfig.tsbuildinfo',
  ]) {
    assert.ok(!fs.existsSync(path.join(outputRoot, leakedRelativePath)), `${leakedRelativePath} should be excluded`)
  }

  assert.match(summary.lines[0], /phase=materialize mode=write result=pass/)
  assert.match(summary.lines[1], /phase=metadata/)
  assert.equal(summary.productRoot, productRoot)
  assert.equal(summary.productRootSource, 'option:productRoot')
  assert.ok(summary.excludedPaths.some((entry) => entry.path === 'mesher/landing/node_modules'))
  assert.ok(summary.excludedPaths.some((entry) => entry.path === 'mesher/.env.local'))

  const manifest = readJson(path.join(path.dirname(outputRoot), 'hyperpush-mono.manifest.json'))
  const stageSummary = readJson(path.join(path.dirname(outputRoot), 'hyperpush-mono.stage.json'))
  assert.equal(stageSummary.outputRoot, outputRoot)
  assert.equal(stageSummary.productRoot, productRoot)
  assert.equal(stageSummary.productRootSource, 'option:productRoot')
  assert.equal(stageSummary.manifest.fingerprint, manifest.fingerprint)
})

test('materializeHyperpushMono fails closed on missing required product-root templates and keeps the failed stage for inspection', (t) => {
  const { meshLangRoot, productRoot } = createFakeWorkspace(t)
  fs.rmSync(path.join(meshLangRoot, 'scripts', 'fixtures', 'm055-s04-hyperpush-root', 'README.md'))

  assert.throws(
    () => materializeHyperpushMono({
      mode: 'write',
      repoRoot: meshLangRoot,
      productRoot,
      templatesRoot: path.join(meshLangRoot, 'scripts', 'fixtures', 'm055-s04-hyperpush-root'),
      outputRoot: path.join(meshLangRoot, '.tmp', 'workspace', 'hyperpush-mono'),
      allowCustomOutputRoot: true,
    }),
    (error) => {
      assert.match(error.message, /missing required file source entry/)
      assert.match(error.message, /stage=/)
      const stageMatch = error.message.match(/\[m055-s04\] stage=(.+)$/m)
      assert.ok(stageMatch, error.message)
      assert.ok(fs.existsSync(stageMatch[1]), `expected failed stage to remain at ${stageMatch[1]}`)
      return true
    },
  )
})

test('staged root verifier passes against the current extracted root templates and staged repo shape', (t) => {
  const { meshLangRoot, productRoot } = createFakeWorkspace(t)
  const outputRoot = path.join(meshLangRoot, '.tmp', 'workspace', 'hyperpush-mono')

  materializeHyperpushMono({
    mode: 'check',
    repoRoot: meshLangRoot,
    productRoot,
    templatesRoot: path.join(meshLangRoot, 'scripts', 'fixtures', 'm055-s04-hyperpush-root'),
    outputRoot,
    allowCustomOutputRoot: true,
  })

  const verifyResult = spawnSync('bash', [path.join(outputRoot, 'scripts', 'verify-landing-surface.sh')], {
    cwd: outputRoot,
    encoding: 'utf8',
  })

  assert.equal(verifyResult.status, 0, `${verifyResult.stdout}\n${verifyResult.stderr}`)
  assert.match(verifyResult.stdout, /verify-landing-surface: ok/)
  assert.ok(fs.existsSync(path.join(outputRoot, '.tmp', 'm055-s04', 'landing-surface', 'verify', 'phase-report.txt')))
})

test('CLI check mode refreshes the standard staged hyperpush-mono repo and rejects output-root overrides', (t) => {
  const { productRoot } = createFakeWorkspace(t)

  const override = runCli(['--check', '--output-root', path.join(repoRoot, '.tmp', 'ignored', 'hyperpush-mono')])
  assert.equal(override.status, 1)
  assert.match(override.stderr, /--check always refreshes the standard staged repo/)

  const result = runCli(['--check'], {
    env: {
      ...process.env,
      M055_HYPERPUSH_ROOT: productRoot,
    },
  })
  assert.equal(result.status, 0, `${result.stdout}\n${result.stderr}`)
  assert.match(result.stdout, /phase=materialize mode=check result=pass/)
  assert.match(result.stdout, /product_root_source=env:M055_HYPERPUSH_ROOT/)
  assert.ok(fs.existsSync(path.join(defaultOutputRoot, 'README.md')))
  assert.ok(fs.existsSync(path.join(defaultOutputRoot, '.github', 'workflows', 'deploy-landing.yml')))
  assert.ok(fs.existsSync(path.join(defaultOutputRoot, 'scripts', 'verify-landing-surface.sh')))
  assert.ok(fs.existsSync(path.join(defaultOutputRoot, 'scripts', 'verify-m051-s01.sh')))
  assert.ok(fs.existsSync(path.join(defaultOutputRoot, 'mesher', 'README.md')))
  assert.ok(!fs.existsSync(path.join(defaultOutputRoot, 'mesher', 'landing', 'node_modules')))
  assert.ok(!fs.existsSync(path.join(defaultOutputRoot, 'mesher', 'landing', '.next')))

  const stageSummary = readJson(path.join(path.dirname(defaultOutputRoot), 'hyperpush-mono.stage.json'))
  assert.equal(stageSummary.outputRoot, defaultOutputRoot)
  assert.equal(stageSummary.mode, 'check')
  assert.equal(stageSummary.productRoot, productRoot)
  assert.equal(stageSummary.productRootSource, 'env:M055_HYPERPUSH_ROOT')
})

test('parseArgs enforces one mode and known flags', () => {
  assert.throws(() => parseArgs([]), /expected exactly one mode/)
  assert.throws(() => parseArgs(['--write', '--check']), /specify only one of --write or --check/)
  assert.throws(() => parseArgs(['--write', '--bogus']), /unknown argument: --bogus/)
})
