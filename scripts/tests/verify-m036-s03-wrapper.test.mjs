import test from 'node:test'
import assert from 'node:assert/strict'
import fs from 'node:fs'
import os from 'node:os'
import path from 'node:path'
import { spawnSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'

const scriptDir = path.dirname(fileURLToPath(import.meta.url))
const root = path.resolve(scriptDir, '..', '..')
const sourceWrapperPath = path.join(root, 'scripts', 'verify-m036-s03.sh')

function writeFile(baseDir, relativePath, content, { executable = false } = {}) {
  const absolutePath = path.join(baseDir, relativePath)
  fs.mkdirSync(path.dirname(absolutePath), { recursive: true })
  fs.writeFileSync(absolutePath, content)
  if (executable) {
    fs.chmodSync(absolutePath, 0o755)
  }
  return absolutePath
}

function makeRepo(t) {
  const repoRoot = fs.mkdtempSync(path.join(os.tmpdir(), 'verify-m036-s03-wrapper-'))
  t.after(() => fs.rmSync(repoRoot, { recursive: true, force: true }))

  writeFile(repoRoot, 'scripts/verify-m036-s03.sh', fs.readFileSync(sourceWrapperPath, 'utf8'), {
    executable: true,
  })

  writeFile(
    repoRoot,
    'scripts/tests/verify-m036-s03-contract.test.mjs',
    [
      "import test from 'node:test'",
      "test('contract stub passes', () => {})",
      '',
    ].join('\n')
  )

  writeFile(
    repoRoot,
    'website/package.json',
    JSON.stringify(
      {
        name: 'website',
        private: true,
        scripts: {
          build: 'node ./build-ok.mjs',
        },
      },
      null,
      2
    ) + '\n'
  )
  writeFile(
    repoRoot,
    'website/build-ok.mjs',
    [
      "import fs from 'node:fs'",
      "import path from 'node:path'",
      "const root = process.cwd()",
      "const outPath = path.join(root, 'docs/.vitepress/dist/docs/tooling/index.html')",
      "fs.mkdirSync(path.dirname(outPath), { recursive: true })",
      "fs.writeFileSync(outPath, '<html><body>tooling ok</body></html>\\n')",
      '',
    ].join('\n')
  )

  writeFile(
    repoRoot,
    'scripts/verify-m034-s04-extension.sh',
    [
      '#!/usr/bin/env bash',
      'set -euo pipefail',
      'ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"',
      'VERIFY_ROOT="$ROOT_DIR/.tmp/m034-s04/verify"',
      'mkdir -p "$VERIFY_ROOT"',
      'if [[ "${M036_S03_TEST_VSIX_FAIL:-0}" == "1" ]]; then',
      '  echo "stub vsix proof failed" >&2',
      '  exit 1',
      'fi',
      'printf "ok\n" >"$VERIFY_ROOT/status.txt"',
      'printf "tools/editors/vscode-mesh/dist/fake.vsix\n" >"$VERIFY_ROOT/verified-vsix-path.txt"',
      'echo "stub vsix proof ok"',
      '',
    ].join('\n'),
    { executable: true }
  )

  writeFile(
    repoRoot,
    'tools/editors/vscode-mesh/package.json',
    JSON.stringify(
      {
        name: 'mesh-lang',
        private: true,
        scripts: {
          'test:smoke': 'node ./smoke-ok.mjs',
        },
      },
      null,
      2
    ) + '\n'
  )
  writeFile(
    repoRoot,
    'tools/editors/vscode-mesh/smoke-ok.mjs',
    [
      "import fs from 'node:fs'",
      "import path from 'node:path'",
      "const repoRoot = path.resolve(process.cwd(), '../../..')",
      "const artifactDir = path.join(repoRoot, '.tmp/m036-s03/vscode-smoke')",
      "fs.mkdirSync(artifactDir, { recursive: true })",
      "fs.writeFileSync(path.join(artifactDir, 'context.json'), JSON.stringify({ ok: true }, null, 2) + '\\n')",
      "if (process.env.M036_S03_TEST_VSCODE_FAIL === '1') {",
      "  fs.writeFileSync(path.join(artifactDir, 'smoke.log'), '[runner] failed before pass marker\\n')",
      "  console.error('stub smoke failure; inspect .tmp/m036-s03/vscode-smoke')",
      "  process.exit(1)",
      "}",
      "fs.writeFileSync(path.join(artifactDir, 'smoke.log'), '[runner] Extension Development Host smoke passed\\n')",
      "console.error('[runner] Extension Development Host smoke passed')",
      '',
    ].join('\n')
  )

  writeFile(
    repoRoot,
    'scripts/verify-m036-s02.sh',
    [
      '#!/usr/bin/env bash',
      'set -euo pipefail',
      'ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"',
      'RUN_DIR="$ROOT_DIR/.tmp/m036-s02/all"',
      'mkdir -p "$RUN_DIR"',
      'if [[ ! -x "${NEOVIM_BIN:-}" ]]; then',
      '  echo "missing neovim bin ${NEOVIM_BIN:-<unset>}" >&2',
      '  exit 1',
      'fi',
      'if [[ "${M036_S03_TEST_NEOVIM_FAIL:-0}" == "1" ]]; then',
      '  printf "[m036-s02] phase=syntax result=pass checked_cases=1\n" >"$RUN_DIR/neovim-smoke.log"',
      '  echo "stub neovim failure; inspect .tmp/m036-s02/all" >&2',
      '  exit 1',
      'fi',
      'printf "ran\n" >"$ROOT_DIR/.tmp/m036-s03/neovim-ran.txt"',
      'cat >"$RUN_DIR/neovim-smoke.log" <<\'EOF\'',
      '[m036-s02] phase=syntax result=pass checked_cases=1',
      '[m036-s02] phase=lsp result=pass checked_cases=1',
      'EOF',
      'echo "stub neovim verifier ok"',
      '',
    ].join('\n'),
    { executable: true }
  )

  writeFile(
    repoRoot,
    '.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim',
    ['#!/usr/bin/env bash', 'exit 0', ''].join('\n'),
    { executable: true }
  )

  return repoRoot
}

function runWrapper(repoRoot, extraEnv = {}) {
  return spawnSync('bash', ['scripts/verify-m036-s03.sh'], {
    cwd: repoRoot,
    encoding: 'utf8',
    env: {
      ...process.env,
      ...extraEnv,
    },
  })
}

test('wrapper assembles the full happy-path proof chain and records phase logs', (t) => {
  const repoRoot = makeRepo(t)
  const result = runWrapper(repoRoot)

  assert.equal(result.status, 0, result.stderr || result.stdout)
  for (const phase of ['docs-contract', 'docs-build', 'vsix-proof', 'vscode-smoke', 'neovim']) {
    assert.match(result.stdout, new RegExp(`==> \\[${phase}\\]`), `missing phase banner for ${phase}`)
  }
  assert.match(result.stdout, /verify-m036-s03: ok/)
  assert.equal(
    fs.readFileSync(path.join(repoRoot, '.tmp/m036-s03/status.txt'), 'utf8').trim(),
    'ok'
  )
  assert.ok(fs.existsSync(path.join(repoRoot, '.tmp/m036-s03/docs-contract.log')))
  assert.ok(fs.existsSync(path.join(repoRoot, '.tmp/m036-s03/docs-build.log')))
  assert.ok(fs.existsSync(path.join(repoRoot, '.tmp/m036-s03/vsix-proof.log')))
  assert.ok(fs.existsSync(path.join(repoRoot, '.tmp/m036-s03/vscode-smoke.log')))
  assert.ok(fs.existsSync(path.join(repoRoot, '.tmp/m036-s03/neovim.log')))
})

test('wrapper fails closed in docs-contract when the contract test file is missing', (t) => {
  const repoRoot = makeRepo(t)
  fs.rmSync(path.join(repoRoot, 'scripts/tests/verify-m036-s03-contract.test.mjs'))

  const result = runWrapper(repoRoot)

  assert.notEqual(result.status, 0, 'wrapper should fail when the contract test file is missing')
  assert.match(result.stderr, /first failing phase: docs-contract/)
  assert.match(result.stderr, /missing required file: scripts\/tests\/verify-m036-s03-contract\.test\.mjs/)
  assert.match(result.stderr, /phase log: \.tmp\/m036-s03\/docs-contract-preflight\.log/)
})

test('wrapper fails closed in vscode-smoke when the smoke script is missing from package.json', (t) => {
  const repoRoot = makeRepo(t)
  writeFile(
    repoRoot,
    'tools/editors/vscode-mesh/package.json',
    JSON.stringify({ name: 'mesh-lang', private: true, scripts: {} }, null, 2) + '\n'
  )

  const result = runWrapper(repoRoot)

  assert.notEqual(result.status, 0, 'wrapper should fail when test:smoke is absent')
  assert.match(result.stderr, /first failing phase: vscode-smoke/)
  assert.match(result.stderr, /missing npm script test:smoke in tools\/editors\/vscode-mesh\/package\.json/)
  assert.match(result.stderr, /upstream artifacts: \.tmp\/m036-s03\/vscode-smoke/)
})

test('wrapper stops on vscode-smoke failure and preserves the smoke artifact path', (t) => {
  const repoRoot = makeRepo(t)

  const result = runWrapper(repoRoot, {
    M036_S03_TEST_VSCODE_FAIL: '1',
  })

  assert.notEqual(result.status, 0, 'wrapper should fail when the VS Code smoke fails')
  assert.match(result.stderr, /first failing phase: vscode-smoke/)
  assert.match(result.stderr, /upstream artifacts: \.tmp\/m036-s03\/vscode-smoke/)
  assert.ok(
    !fs.existsSync(path.join(repoRoot, '.tmp/m036-s03/neovim-ran.txt')),
    'neovim phase should not run after a VS Code smoke failure'
  )
})

test('wrapper fails closed in neovim when the documented vendor override is missing', (t) => {
  const repoRoot = makeRepo(t)
  fs.rmSync(path.join(repoRoot, '.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim'))

  const result = runWrapper(repoRoot)

  assert.notEqual(result.status, 0, 'wrapper should fail when the vendor nvim override is missing')
  assert.match(result.stderr, /first failing phase: neovim/)
  assert.match(result.stderr, /missing required executable: \.tmp\/m036-s02\/vendor\/nvim-macos-arm64\/bin\/nvim/)
  assert.match(result.stderr, /upstream artifacts: \.tmp\/m036-s02\/all/)
})
