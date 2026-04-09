import test from 'node:test'
import assert from 'node:assert/strict'
import fs from 'node:fs'
import os from 'node:os'
import path from 'node:path'
import { spawnSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'

const scriptDir = path.dirname(fileURLToPath(import.meta.url))
const root = path.resolve(scriptDir, '..', '..')

const starterWorkflowPath = '.github/workflows/authoritative-starter-failover-proof.yml'
const callerWorkflowPath = '.github/workflows/authoritative-verification.yml'
const servicesWorkflowPath = '.github/workflows/deploy-services.yml'
const releaseWorkflowPath = '.github/workflows/release.yml'
const workflowVerifierPath = 'scripts/verify-m034-s02-workflows.sh'
const hostedVerifierPath = 'scripts/verify-m053-s03.sh'
const meshcCargoPath = 'compiler/meshc/Cargo.toml'
const meshpkgCargoPath = 'compiler/meshpkg/Cargo.toml'

const MAIN_SHA = '1111111111111111111111111111111111111111'
const STALE_MAIN_SHA = '2222222222222222222222222222222222222222'
const TAG_OBJECT_SHA = '3333333333333333333333333333333333333333'
const TAG_SHA = '4444444444444444444444444444444444444444'

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

function writeExecutable(baseRoot, relativePath, content) {
  writeTo(baseRoot, relativePath, content)
  fs.chmodSync(path.join(baseRoot, relativePath), 0o755)
}

function copyRepoFile(baseRoot, relativePath) {
  writeTo(baseRoot, relativePath, readFrom(root, relativePath))
}

function mkTmpDir(t, prefix) {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), prefix))
  if (!process.env.GSD_KEEP_TMP) {
    t.after(() => fs.rmSync(dir, { recursive: true, force: true }))
  }
  return dir
}

function escapeRegex(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
}

function extractJobBlock(text, jobId) {
  const pattern = new RegExp(`\n  ${escapeRegex(jobId)}:\n([\\s\\S]*?)(?=\n  [A-Za-z0-9_-]+:\n|$)`)
  const match = text.match(pattern)
  return match ? match[0] : ''
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
      errors.push(`${relativePath} still contains forbidden text ${JSON.stringify(needle)}`)
    }
  }
}

function requireMatch(errors, relativePath, text, regex, label) {
  if (!regex.test(text)) {
    errors.push(`${relativePath} missing ${label}`)
  }
}

function requireCount(errors, relativePath, text, needle, expectedCount) {
  const actualCount = text.split(needle).length - 1
  if (actualCount !== expectedCount) {
    errors.push(`${relativePath} expected ${expectedCount} instance(s) of ${JSON.stringify(needle)} but found ${actualCount}`)
  }
}

function validateHostedStarterContract(baseRoot) {
  const errors = []
  const starterWorkflow = readFrom(baseRoot, starterWorkflowPath)
  const callerWorkflow = readFrom(baseRoot, callerWorkflowPath)
  const releaseWorkflow = readFrom(baseRoot, releaseWorkflowPath)
  const workflowVerifier = readFrom(baseRoot, workflowVerifierPath)

  const starterJob = extractJobBlock(starterWorkflow, 'starter-failover-proof')
  const callerStarterJob = extractJobBlock(callerWorkflow, 'starter-failover-proof')
  const releaseStarterJob = extractJobBlock(releaseWorkflow, 'authoritative-starter-failover-proof')
  const releaseJob = extractJobBlock(releaseWorkflow, 'release')

  requireIncludes(errors, starterWorkflowPath, starterWorkflow, [
    'name: Authoritative starter failover proof',
    'workflow_call:',
    'contents: read',
    'postgres:16',
    'POSTGRES_USER: postgres',
    'POSTGRES_PASSWORD: postgres',
    'POSTGRES_DB: mesh_starter',
    '5432:5432',
    'pg_isready -U postgres -d mesh_starter',
    'Initialize starter proof diagnostics',
    'mkdir -p .tmp/m053-s02/verify',
    'hosted-workflow-metadata.txt',
    'Verify starter failover proof entrypoint',
    'test -f scripts/verify-m053-s02.sh',
    'Set up Node.js',
    'actions/setup-node@v4',
    'node-version: 20',
    'Cache LLVM',
    'Install LLVM 21 (Linux x86_64)',
    'Set LLVM prefix (Linux tarball)',
    'Install Rust',
    'Cargo cache',
    'key: authoritative-starter-failover-proof-x86_64-unknown-linux-gnu',
    'Export runner-local DATABASE_URL',
    'echo "::add-mask::$db_url"',
    'echo "DATABASE_URL=$db_url" >> "$GITHUB_ENV"',
    'Wait for runner-local Postgres',
    'sock.connect(("127.0.0.1", 5432))',
    'Run authoritative starter failover proof',
    'id: proof',
    'bash scripts/verify-m053-s02.sh',
    'Upload starter failover diagnostics',
    'actions/upload-artifact@v4',
    'name: authoritative-starter-failover-proof-diagnostics',
    'path: .tmp/m053-s02/**',
    'if-no-files-found: error',
  ])

  requireMatch(
    errors,
    starterWorkflowPath,
    starterJob,
    /starter-failover-proof:[\s\S]*runs-on: ubuntu-24\.04[\s\S]*timeout-minutes: 180/,
    'starter reusable job shape',
  )
  requireMatch(
    errors,
    starterWorkflowPath,
    starterJob,
    /services:[\s\S]*postgres:[\s\S]*image: postgres:16[\s\S]*POSTGRES_USER: postgres[\s\S]*POSTGRES_PASSWORD: postgres[\s\S]*POSTGRES_DB: mesh_starter[\s\S]*5432:5432[\s\S]*pg_isready -U postgres -d mesh_starter/,
    'runner-local Postgres service contract',
  )
  requireMatch(
    errors,
    starterWorkflowPath,
    starterJob,
    /Set up Node\.js[\s\S]*actions\/setup-node@v4[\s\S]*node-version: 20/,
    'Node setup contract',
  )
  requireMatch(
    errors,
    starterWorkflowPath,
    starterJob,
    /Run authoritative starter failover proof[\s\S]*id: proof[\s\S]*run: bash scripts\/verify-m053-s02\.sh/,
    'starter proof entrypoint contract',
  )
  requireMatch(
    errors,
    starterWorkflowPath,
    starterJob,
    /Upload starter failover diagnostics[\s\S]*if: failure\(\)[\s\S]*authoritative-starter-failover-proof-diagnostics[\s\S]*\.tmp\/m053-s02\/\*\*/,
    'starter proof diagnostics upload contract',
  )
  requireExcludes(errors, starterWorkflowPath, starterWorkflow, [
    'MESH_PUBLISH_OWNER',
    'MESH_PUBLISH_TOKEN',
    'bash scripts/verify-m034-s01.sh',
    'packages.meshlang.dev',
    'flyctl',
  ])
  requireCount(errors, starterWorkflowPath, starterWorkflow, 'bash scripts/verify-m053-s02.sh', 1)

  requireMatch(
    errors,
    callerWorkflowPath,
    callerStarterJob,
    /starter-failover-proof:[\s\S]*name: Authoritative starter failover proof[\s\S]*needs: whitespace-guard[\s\S]*uses: \.\/\.github\/workflows\/authoritative-starter-failover-proof\.yml/,
    'authoritative-verification starter caller job',
  )
  requireExcludes(errors, callerWorkflowPath, callerStarterJob, [
    'if:',
    'head.repo.full_name == github.repository',
    'secrets:',
    'MESH_PUBLISH_TOKEN',
    'MESH_PUBLISH_OWNER',
    'bash scripts/verify-m053-s02.sh',
  ])
  requireCount(errors, callerWorkflowPath, callerWorkflow, './.github/workflows/authoritative-live-proof.yml', 1)
  requireCount(errors, callerWorkflowPath, callerWorkflow, './.github/workflows/authoritative-starter-failover-proof.yml', 1)

  requireMatch(
    errors,
    releaseWorkflowPath,
    releaseStarterJob,
    /authoritative-starter-failover-proof:[\s\S]*name: Authoritative starter failover proof[\s\S]*if: startsWith\(github\.ref, 'refs\/tags\/v'\)[\s\S]*uses: \.\/\.github\/workflows\/authoritative-starter-failover-proof\.yml/,
    'release starter proof caller job',
  )
  requireExcludes(errors, releaseWorkflowPath, releaseStarterJob, [
    'secrets:',
    'MESH_PUBLISH_TOKEN',
    'MESH_PUBLISH_OWNER',
    'bash scripts/verify-m053-s02.sh',
  ])
  requireMatch(
    errors,
    releaseWorkflowPath,
    releaseJob,
    /needs: \[build, build-meshpkg, authoritative-live-proof, authoritative-starter-failover-proof, verify-release-assets\]/,
    'release job starter-proof prerequisite',
  )
  requireCount(errors, releaseWorkflowPath, releaseWorkflow, './.github/workflows/authoritative-live-proof.yml', 1)
  requireCount(errors, releaseWorkflowPath, releaseWorkflow, './.github/workflows/authoritative-starter-failover-proof.yml', 1)
  requireExcludes(errors, releaseWorkflowPath, releaseWorkflow, ['bash scripts/verify-m053-s02.sh'])

  requireIncludes(errors, workflowVerifierPath, workflowVerifier, [
    'STARTER_REUSABLE_WORKFLOW_PATH=".github/workflows/authoritative-starter-failover-proof.yml"',
    'run_starter_reusable_contract_check',
    'jobs.keys == ["whitespace-guard", "live-proof", "starter-failover-proof"]',
    'jobs.keys == ["build", "build-meshpkg", "authoritative-live-proof", "authoritative-starter-failover-proof", "verify-release-assets", "release"]',
    'expected_release_needs = %w[authoritative-live-proof authoritative-starter-failover-proof build build-meshpkg verify-release-assets]',
    'authoritative-starter-failover-proof-diagnostics',
    'bash scripts/verify-m053-s02.sh',
  ])

  return errors
}

function deepClone(value) {
  return JSON.parse(JSON.stringify(value))
}

function defaultHostedEvidenceScenario() {
  return {
    gitLsRemote: {
      'refs/heads/main': {
        lines: [[MAIN_SHA, 'refs/heads/main']],
      },
      'refs/tags/v0.1.0|refs/tags/v0.1.0^{}': {
        lines: [
          [TAG_OBJECT_SHA, 'refs/tags/v0.1.0'],
          [TAG_SHA, 'refs/tags/v0.1.0^{}'],
        ],
      },
    },
    ghRunList: {
      'authoritative-verification.yml|push|main': {
        json: [
          {
            databaseId: 101,
            workflowName: 'Authoritative verification',
            event: 'push',
            status: 'completed',
            conclusion: 'success',
            headBranch: 'main',
            headSha: MAIN_SHA,
            displayTitle: 'main hosted verification',
            createdAt: '2026-04-05T20:00:00Z',
            url: 'https://example.test/runs/101',
          },
        ],
      },
      'deploy-services.yml|push|main': {
        json: [
          {
            databaseId: 202,
            workflowName: 'Deploy Services to Fly.io',
            event: 'push',
            status: 'completed',
            conclusion: 'success',
            headBranch: 'main',
            headSha: MAIN_SHA,
            displayTitle: 'main deploy-services',
            createdAt: '2026-04-05T20:05:00Z',
            url: 'https://example.test/runs/202',
          },
        ],
      },
      'release.yml|push|v0.1.0': {
        json: [
          {
            databaseId: 303,
            workflowName: 'Release',
            event: 'push',
            status: 'completed',
            conclusion: 'success',
            headBranch: 'v0.1.0',
            headSha: TAG_SHA,
            displayTitle: 'release v0.1.0',
            createdAt: '2026-04-05T20:10:00Z',
            url: 'https://example.test/runs/303',
          },
        ],
      },
      'authoritative-verification.yml||': {
        json: [
          {
            databaseId: 101,
            workflowName: 'Authoritative verification',
            event: 'push',
            status: 'completed',
            conclusion: 'success',
            headBranch: 'main',
            headSha: MAIN_SHA,
            displayTitle: 'main hosted verification',
            createdAt: '2026-04-05T20:00:00Z',
            url: 'https://example.test/runs/101',
          },
        ],
      },
      'deploy-services.yml||': {
        json: [
          {
            databaseId: 202,
            workflowName: 'Deploy Services to Fly.io',
            event: 'push',
            status: 'completed',
            conclusion: 'success',
            headBranch: 'main',
            headSha: MAIN_SHA,
            displayTitle: 'main deploy-services',
            createdAt: '2026-04-05T20:05:00Z',
            url: 'https://example.test/runs/202',
          },
        ],
      },
      'release.yml||': {
        json: [
          {
            databaseId: 303,
            workflowName: 'Release',
            event: 'push',
            status: 'completed',
            conclusion: 'success',
            headBranch: 'v0.1.0',
            headSha: TAG_SHA,
            displayTitle: 'release v0.1.0',
            createdAt: '2026-04-05T20:10:00Z',
            url: 'https://example.test/runs/303',
          },
        ],
      },
    },
    ghRunView: {
      '101': {
        json: {
          databaseId: 101,
          workflowName: 'Authoritative verification',
          event: 'push',
          status: 'completed',
          conclusion: 'success',
          headBranch: 'main',
          headSha: MAIN_SHA,
          displayTitle: 'main hosted verification',
          url: 'https://example.test/runs/101',
          jobs: [
            {
              name: 'Whitespace guard',
              status: 'completed',
              conclusion: 'success',
              steps: [{ name: 'Verify incoming diff is whitespace-clean' }],
            },
            {
              name: 'Authoritative live proof',
              status: 'completed',
              conclusion: 'success',
              steps: [{ name: 'Run authoritative live proof' }],
            },
            {
              name: 'Authoritative starter failover proof / starter-failover-proof',
              status: 'completed',
              conclusion: 'success',
              steps: [{ name: 'Run authoritative starter failover proof' }],
            },
          ],
        },
      },
      '202': {
        json: {
          databaseId: 202,
          workflowName: 'Deploy Services to Fly.io',
          event: 'push',
          status: 'completed',
          conclusion: 'success',
          headBranch: 'main',
          headSha: MAIN_SHA,
          displayTitle: 'main deploy-services',
          url: 'https://example.test/runs/202',
          jobs: [
            {
              name: 'Deploy mesh-registry',
              status: 'completed',
              conclusion: 'success',
              steps: [{ name: 'Deploy registry to Fly.io' }],
            },
            {
              name: 'Deploy mesh-packages website',
              status: 'completed',
              conclusion: 'success',
              steps: [{ name: 'Deploy packages website to Fly.io' }],
            },
            {
              name: 'Post-deploy health checks',
              status: 'completed',
              conclusion: 'success',
              steps: [
                { name: 'Checkout' },
                { name: 'Verify public surface contract' },
              ],
            },
          ],
        },
      },
      '303': {
        json: {
          databaseId: 303,
          workflowName: 'Release',
          event: 'push',
          status: 'completed',
          conclusion: 'success',
          headBranch: 'v0.1.0',
          headSha: TAG_SHA,
          displayTitle: 'release v0.1.0',
          url: 'https://example.test/runs/303',
          jobs: [
            {
              name: 'Authoritative live proof',
              status: 'completed',
              conclusion: 'success',
              steps: [{ name: 'Run authoritative live proof' }],
            },
            {
              name: 'Authoritative starter failover proof / starter-failover-proof',
              status: 'completed',
              conclusion: 'success',
              steps: [{ name: 'Run authoritative starter failover proof' }],
            },
            {
              name: 'Create Release',
              status: 'completed',
              conclusion: 'success',
              steps: [{ name: 'Create GitHub Release' }],
            },
          ],
        },
      },
    },
  }
}

function installHostedEvidenceStubs(baseRoot, scenario) {
  const dataPath = path.join(baseRoot, 'stub-data.json')
  writeTo(baseRoot, 'stub-data.json', `${JSON.stringify(scenario, null, 2)}\n`)

  writeExecutable(
    baseRoot,
    'bin/git',
    `#!/usr/bin/env python3
import json
import os
import sys
from pathlib import Path

data = json.loads(Path(os.environ['M053_S03_STUB_DATA']).read_text())
args = sys.argv[1:]
if args[:3] != ['ls-remote', '--quiet', 'origin']:
    print('unexpected git invocation: ' + ' '.join(args), file=sys.stderr)
    raise SystemExit(86)
key = '|'.join(args[3:])
entry = data.get('gitLsRemote', {}).get(key)
if entry is None:
    print(f'missing git ls-remote stub for {key}', file=sys.stderr)
    raise SystemExit(87)
if isinstance(entry.get('stderr'), str):
    sys.stderr.write(entry['stderr'])
if isinstance(entry.get('stdout'), str):
    sys.stdout.write(entry['stdout'])
elif isinstance(entry.get('lines'), list):
    for sha, ref_name in entry['lines']:
        sys.stdout.write(f'{sha}\\t{ref_name}\\n')
raise SystemExit(entry.get('exitCode', 0))
`,
  )

  writeExecutable(
    baseRoot,
    'bin/gh',
    `#!/usr/bin/env python3
import json
import os
import sys
from pathlib import Path

data = json.loads(Path(os.environ['M053_S03_STUB_DATA']).read_text())
args = sys.argv[1:]

def value_for(flag):
    try:
        index = args.index(flag)
    except ValueError:
        return ''
    return args[index + 1] if index + 1 < len(args) else ''

if not args or args[0] != 'run':
    print('unexpected gh invocation: ' + ' '.join(args), file=sys.stderr)
    raise SystemExit(86)

if len(args) >= 2 and args[1] == 'list':
    workflow = value_for('--workflow')
    event = value_for('--event')
    branch = value_for('--branch')
    key = f'{workflow}|{event}|{branch}'
    entry = data.get('ghRunList', {}).get(key)
    if entry is None:
        print(f'missing gh run list stub for {key}', file=sys.stderr)
        raise SystemExit(87)
    if isinstance(entry.get('stderr'), str):
        sys.stderr.write(entry['stderr'])
    if isinstance(entry.get('stdout'), str):
        sys.stdout.write(entry['stdout'])
    else:
        sys.stdout.write(json.dumps(entry.get('json', [])))
    raise SystemExit(entry.get('exitCode', 0))

if len(args) >= 3 and args[1] == 'view':
    run_id = args[2]
    entry = data.get('ghRunView', {}).get(str(run_id))
    if entry is None:
        print(f'missing gh run view stub for {run_id}', file=sys.stderr)
        raise SystemExit(88)
    if isinstance(entry.get('stderr'), str):
        sys.stderr.write(entry['stderr'])
    if isinstance(entry.get('stdout'), str):
        sys.stdout.write(entry['stdout'])
    else:
        sys.stdout.write(json.dumps(entry.get('json', {})))
    raise SystemExit(entry.get('exitCode', 0))

print('unexpected gh subcommand: ' + ' '.join(args), file=sys.stderr)
raise SystemExit(89)
`,
  )

  return dataPath
}

function createHostedVerifierFixture(t, scenario = defaultHostedEvidenceScenario()) {
  const tmpRoot = mkTmpDir(t, 'verify-m053-s03-hosted-')
  for (const relativePath of [
    hostedVerifierPath,
    callerWorkflowPath,
    servicesWorkflowPath,
    releaseWorkflowPath,
    meshcCargoPath,
    meshpkgCargoPath,
  ]) {
    copyRepoFile(tmpRoot, relativePath)
  }

  const dataPath = installHostedEvidenceStubs(tmpRoot, scenario)
  return {
    tmpRoot,
    dataPath,
    verifyRoot: path.join(tmpRoot, '.tmp', 'm053-s03', 'verify'),
  }
}

function runHostedVerifier(baseRoot, dataPath, envOverrides = {}) {
  return spawnSync('bash', [path.join(baseRoot, hostedVerifierPath)], {
    cwd: baseRoot,
    env: {
      ...process.env,
      PATH: `${path.join(baseRoot, 'bin')}:${process.env.PATH}`,
      NODE_OPTIONS: '',
      M053_S03_STUB_DATA: dataPath,
      M053_S03_GIT_BIN: path.join(baseRoot, 'bin', 'git'),
      M053_S03_GH_BIN: path.join(baseRoot, 'bin', 'gh'),
      M053_S03_GH_REPO: 'hyperpush-org/hyperpush-mono',
      ...envOverrides,
    },
    encoding: 'utf8',
  })
}

function readVerifyArtifact(verifyRoot, relativePath) {
  return fs.readFileSync(path.join(verifyRoot, relativePath), 'utf8')
}

function readJsonArtifact(verifyRoot, relativePath) {
  return JSON.parse(readVerifyArtifact(verifyRoot, relativePath))
}

function findWorkflow(remoteRuns, workflowFile) {
  const workflow = remoteRuns.workflows.find((entry) => entry.workflowFile === workflowFile)
  assert.ok(workflow, `missing workflow entry for ${workflowFile}`)
  return workflow
}

test('current workflows keep the hosted starter failover proof wired into authoritative main/tag gates', () => {
  const errors = validateHostedStarterContract(root)
  assert.deepEqual(errors, [], errors.join('\n'))
})

test('contract fails closed when the reusable starter workflow loses Postgres, the proof entrypoint, or diagnostics upload', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m053-s03-reusable-')
  for (const relativePath of [starterWorkflowPath, callerWorkflowPath, releaseWorkflowPath, workflowVerifierPath]) {
    copyRepoFile(tmpRoot, relativePath)
  }

  let mutatedWorkflow = readFrom(tmpRoot, starterWorkflowPath)
  mutatedWorkflow = mutatedWorkflow.replace('image: postgres:16', 'image: redis:7')
  mutatedWorkflow = mutatedWorkflow.replace('run: bash scripts/verify-m053-s02.sh', 'run: bash scripts/verify-m034-s01.sh')
  mutatedWorkflow = mutatedWorkflow.replace('path: .tmp/m053-s02/**', 'path: .tmp/m053-s01/**')
  writeTo(tmpRoot, starterWorkflowPath, mutatedWorkflow)

  const errors = validateHostedStarterContract(tmpRoot)
  assert.ok(errors.some((error) => error.includes('runner-local Postgres service contract') || error.includes('"postgres:16"')), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes('starter proof entrypoint contract') || error.includes('bash scripts/verify-m053-s02.sh')), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes('starter proof diagnostics upload contract') || error.includes('.tmp/m053-s02/**')), errors.join('\n'))
})

test('contract fails closed when authoritative-verification drops or fork-guards the secret-free starter proof lane', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m053-s03-caller-')
  for (const relativePath of [starterWorkflowPath, callerWorkflowPath, releaseWorkflowPath, workflowVerifierPath]) {
    copyRepoFile(tmpRoot, relativePath)
  }

  let mutatedCaller = readFrom(tmpRoot, callerWorkflowPath)
  mutatedCaller = mutatedCaller.replace(
    '  starter-failover-proof:\n    name: Authoritative starter failover proof\n    needs: whitespace-guard\n    uses: ./.github/workflows/authoritative-starter-failover-proof.yml\n',
    '  starter-failover-proof:\n    name: Authoritative starter failover proof\n    needs: whitespace-guard\n    if: github.event_name != \'pull_request\'\n    uses: ./.github/workflows/authoritative-live-proof.yml\n',
  )
  writeTo(tmpRoot, callerWorkflowPath, mutatedCaller)

  const errors = validateHostedStarterContract(tmpRoot)
  assert.ok(errors.some((error) => error.includes('authoritative-verification starter caller job') || error.includes('authoritative-starter-failover-proof.yml')), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes('head.repo.full_name == github.repository') || error.includes('secrets:') || error.includes('still contains forbidden text')), errors.join('\n'))
})

test('contract fails closed when release drops the starter proof tag gate or omits it from release needs', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m053-s03-release-')
  for (const relativePath of [starterWorkflowPath, callerWorkflowPath, releaseWorkflowPath, workflowVerifierPath]) {
    copyRepoFile(tmpRoot, relativePath)
  }

  let mutatedRelease = readFrom(tmpRoot, releaseWorkflowPath)
  mutatedRelease = mutatedRelease.replace(
    '  authoritative-starter-failover-proof:\n    name: Authoritative starter failover proof\n    if: startsWith(github.ref, \'refs/tags/v\')\n    uses: ./.github/workflows/authoritative-starter-failover-proof.yml\n',
    '  authoritative-starter-failover-proof:\n    name: Authoritative starter failover proof\n    uses: ./.github/workflows/authoritative-live-proof.yml\n',
  )
  mutatedRelease = mutatedRelease.replace(
    'needs: [build, build-meshpkg, authoritative-live-proof, authoritative-starter-failover-proof, verify-release-assets]',
    'needs: [build, build-meshpkg, authoritative-live-proof, verify-release-assets]',
  )
  writeTo(tmpRoot, releaseWorkflowPath, mutatedRelease)

  const errors = validateHostedStarterContract(tmpRoot)
  assert.ok(errors.some((error) => error.includes('release starter proof caller job') || error.includes('authoritative-starter-failover-proof.yml')), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes('release job starter-proof prerequisite')), errors.join('\n'))
})

test('contract fails closed when the local workflow verifier forgets the starter reusable lane', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m053-s03-verifier-')
  for (const relativePath of [starterWorkflowPath, callerWorkflowPath, releaseWorkflowPath, workflowVerifierPath]) {
    copyRepoFile(tmpRoot, relativePath)
  }

  let mutatedVerifier = readFrom(tmpRoot, workflowVerifierPath)
  mutatedVerifier = mutatedVerifier.replace(
    'STARTER_REUSABLE_WORKFLOW_PATH=".github/workflows/authoritative-starter-failover-proof.yml"',
    'STARTER_REUSABLE_WORKFLOW_PATH=".github/workflows/authoritative-live-proof.yml"',
  )
  mutatedVerifier = mutatedVerifier.replace('run_starter_reusable_contract_check', 'run_starter_contract_check')
  mutatedVerifier = mutatedVerifier.replace('authoritative-starter-failover-proof-diagnostics', 'authoritative-live-proof-diagnostics')
  writeTo(tmpRoot, workflowVerifierPath, mutatedVerifier)

  const errors = validateHostedStarterContract(tmpRoot)
  assert.ok(errors.some((error) => error.includes('STARTER_REUSABLE_WORKFLOW_PATH') || error.includes('run_starter_reusable_contract_check')), errors.join('\n'))
  assert.ok(errors.some((error) => error.includes('authoritative-starter-failover-proof-diagnostics')), errors.join('\n'))
})

test('hosted verifier succeeds with fresh mainline packages evidence and fresh release-tag starter proof', (t) => {
  const { tmpRoot, dataPath, verifyRoot } = createHostedVerifierFixture(t)
  const result = runHostedVerifier(tmpRoot, dataPath, { GH_TOKEN: 'test-gh-token' })

  assert.equal(result.status, 0, result.stderr || result.stdout)
  assert.equal(readVerifyArtifact(verifyRoot, 'status.txt').trim(), 'ok')
  assert.equal(readVerifyArtifact(verifyRoot, 'current-phase.txt').trim(), 'complete')

  const phaseReport = readVerifyArtifact(verifyRoot, 'phase-report.txt')
  assert.match(phaseReport, /^gh-preflight\tpassed$/m)
  assert.match(phaseReport, /^candidate-refs\tpassed$/m)
  assert.match(phaseReport, /^remote-evidence\tpassed$/m)
  assert.match(phaseReport, /^artifact-contract\tpassed$/m)

  const candidateRefs = readJsonArtifact(verifyRoot, 'candidate-refs.json')
  assert.equal(candidateRefs.binaryTag, 'v0.1.0')
  assert.deepEqual(
    candidateRefs.workflows.map((workflow) => workflow.workflowFile),
    ['authoritative-verification.yml', 'deploy-services.yml', 'release.yml'],
  )

  const remoteRuns = readJsonArtifact(verifyRoot, 'remote-runs.json')
  assert.equal(remoteRuns.binaryTag, 'v0.1.0')
  assert.equal(remoteRuns.workflows.length, 3)

  const authoritative = findWorkflow(remoteRuns, 'authoritative-verification.yml')
  assert.equal(authoritative.status, 'ok')
  assert.equal(authoritative.expectedHeadSha, MAIN_SHA)
  assert.equal(authoritative.observedHeadSha, MAIN_SHA)
  assert.equal(authoritative.headShaMatchesExpected, true)
  assert.deepEqual(authoritative.requiredJobs, ['Hosted starter failover proof'])
  assert.match(authoritative.matchedJobs['Hosted starter failover proof'].actualName, /Authoritative starter failover proof/)

  const deployServices = findWorkflow(remoteRuns, 'deploy-services.yml')
  assert.equal(deployServices.status, 'ok')
  assert.equal(deployServices.expectedHeadSha, MAIN_SHA)
  assert.equal(deployServices.observedHeadSha, MAIN_SHA)
  assert.deepEqual(deployServices.requiredJobs, ['Deploy mesh-registry', 'Deploy mesh-packages website', 'Post-deploy health checks'])
  assert.deepEqual(deployServices.forbiddenJobs, ['Deploy hyperpush landing'])
  assert.deepEqual(deployServices.forbiddenSteps['Post-deploy health checks'], ['Verify hyperpush landing'])
  assert.equal(deployServices.requiredSteps['Post-deploy health checks'][0], 'Verify public surface contract')
  assert.equal(deployServices.matchedJobs['Deploy mesh-registry'].actualName, 'Deploy mesh-registry')
  assert.equal(deployServices.matchedJobs['Deploy mesh-packages website'].actualName, 'Deploy mesh-packages website')

  const release = findWorkflow(remoteRuns, 'release.yml')
  assert.equal(release.status, 'ok')
  assert.equal(release.expectedHeadSha, TAG_SHA)
  assert.equal(release.observedHeadSha, TAG_SHA)
  assert.equal(release.expectedResolvedRef, 'refs/tags/v0.1.0^{}')
  assert.match(release.matchedJobs['Hosted starter failover proof'].actualName, /Authoritative starter failover proof/)

  for (const artifactPath of [
    'authoritative-verification-list.log',
    'authoritative-verification-view.log',
    'deploy-services-list.log',
    'deploy-services-view.log',
    'release-expected-ref.log',
    'full-contract.log',
  ]) {
    assert.ok(fs.existsSync(path.join(verifyRoot, artifactPath)), `missing ${artifactPath}`)
  }
})

test('hosted verifier fails closed before any remote query when GH_TOKEN is missing', (t) => {
  const { tmpRoot, dataPath, verifyRoot } = createHostedVerifierFixture(t)
  const result = runHostedVerifier(tmpRoot, dataPath, { GH_TOKEN: '' })

  assert.notEqual(result.status, 0)
  assert.equal(readVerifyArtifact(verifyRoot, 'status.txt').trim(), 'failed')
  assert.equal(readVerifyArtifact(verifyRoot, 'current-phase.txt').trim(), 'gh-preflight')
  const phaseReport = readVerifyArtifact(verifyRoot, 'phase-report.txt')
  assert.match(phaseReport, /^gh-preflight\tstarted$/m)
  assert.match(phaseReport, /^gh-preflight\tfailed$/m)
  assert.match(result.stdout, /missing GH_TOKEN, repo slug, or required executables/)
  assert.ok(!fs.existsSync(path.join(verifyRoot, 'candidate-refs.json')), 'preflight failures must stop before candidate refs are emitted')
})

test('hosted verifier fails closed when authoritative main evidence is stale against origin main', (t) => {
  const scenario = defaultHostedEvidenceScenario()
  scenario.ghRunList['authoritative-verification.yml|push|main'].json[0].headSha = STALE_MAIN_SHA
  scenario.ghRunList['authoritative-verification.yml||'].json[0].headSha = STALE_MAIN_SHA
  scenario.ghRunView['101'].json.headSha = STALE_MAIN_SHA

  const { tmpRoot, dataPath, verifyRoot } = createHostedVerifierFixture(t, scenario)
  const result = runHostedVerifier(tmpRoot, dataPath, { GH_TOKEN: 'test-gh-token' })

  assert.notEqual(result.status, 0)
  assert.equal(readVerifyArtifact(verifyRoot, 'status.txt').trim(), 'failed')
  assert.equal(readVerifyArtifact(verifyRoot, 'current-phase.txt').trim(), 'remote-evidence')

  const authoritative = findWorkflow(readJsonArtifact(verifyRoot, 'remote-runs.json'), 'authoritative-verification.yml')
  assert.equal(authoritative.status, 'failed')
  assert.equal(authoritative.freshnessStatus, 'failed')
  assert.equal(authoritative.headShaMatchesExpected, false)
  assert.equal(authoritative.expectedHeadSha, MAIN_SHA)
  assert.equal(authoritative.observedHeadSha, STALE_MAIN_SHA)
  assert.match(authoritative.failure, /did not match expected/)
})

test('hosted verifier fails closed when deploy-services proof exists only on the release tag and not on main', (t) => {
  const scenario = defaultHostedEvidenceScenario()
  scenario.ghRunList['deploy-services.yml|push|main'] = { json: [] }
  scenario.ghRunList['deploy-services.yml||'] = {
    json: [
      {
        databaseId: 404,
        workflowName: 'Deploy Services to Fly.io',
        event: 'push',
        status: 'completed',
        conclusion: 'success',
        headBranch: 'v0.1.0',
        headSha: TAG_SHA,
        displayTitle: 'tag-only deploy-services',
        createdAt: '2026-04-05T20:15:00Z',
        url: 'https://example.test/runs/404',
      },
    ],
  }

  const { tmpRoot, dataPath, verifyRoot } = createHostedVerifierFixture(t, scenario)
  const result = runHostedVerifier(tmpRoot, dataPath, { GH_TOKEN: 'test-gh-token' })

  assert.notEqual(result.status, 0)
  const deployServices = findWorkflow(readJsonArtifact(verifyRoot, 'remote-runs.json'), 'deploy-services.yml')
  assert.equal(deployServices.status, 'failed')
  assert.equal(deployServices.freshnessStatus, 'failed')
  assert.equal(deployServices.headShaMatchesExpected, false)
  assert.equal(deployServices.latestAvailableRun.headBranch, 'v0.1.0')
  assert.equal(deployServices.latestAvailableRun.headSha, TAG_SHA)
  assert.match(deployServices.failure, /has no hosted run.*'main'.*latest available was 'v0\.1\.0'/)
})

test('hosted verifier fails closed when the starter proof job is missing from authoritative hosted evidence', (t) => {
  const scenario = defaultHostedEvidenceScenario()
  scenario.ghRunView['101'].json.jobs = scenario.ghRunView['101'].json.jobs.filter(
    (job) => !String(job.name).includes('Authoritative starter failover proof'),
  )

  const { tmpRoot, dataPath, verifyRoot } = createHostedVerifierFixture(t, scenario)
  const result = runHostedVerifier(tmpRoot, dataPath, { GH_TOKEN: 'test-gh-token' })

  assert.notEqual(result.status, 0)
  const authoritative = findWorkflow(readJsonArtifact(verifyRoot, 'remote-runs.json'), 'authoritative-verification.yml')
  assert.equal(authoritative.status, 'failed')
  assert.deepEqual(authoritative.requiredJobs, ['Hosted starter failover proof'])
  assert.match(authoritative.failure, /missing required jobs: \['Hosted starter failover proof'\]/)
})

test('hosted verifier fails closed when deploy-services omits Verify public surface contract', (t) => {
  const scenario = defaultHostedEvidenceScenario()
  scenario.ghRunView['202'].json.jobs = scenario.ghRunView['202'].json.jobs.map((job) => {
    if (job.name !== 'Post-deploy health checks') {
      return job
    }
    return {
      ...job,
      steps: job.steps.filter((step) => step.name !== 'Verify public surface contract'),
    }
  })

  const { tmpRoot, dataPath, verifyRoot } = createHostedVerifierFixture(t, scenario)
  const result = runHostedVerifier(tmpRoot, dataPath, { GH_TOKEN: 'test-gh-token' })

  assert.notEqual(result.status, 0)
  const deployServices = findWorkflow(readJsonArtifact(verifyRoot, 'remote-runs.json'), 'deploy-services.yml')
  assert.equal(deployServices.status, 'failed')
  assert.match(deployServices.failure, /missing required steps: \['Post-deploy health checks: Verify public surface contract'\]/)
})

test('hosted verifier fails closed when deploy-services reintroduces the Hyperpush landing job or landing check', (t) => {
  const scenario = defaultHostedEvidenceScenario()
  scenario.ghRunView['202'].json.jobs.splice(2, 0, {
    name: 'Deploy hyperpush landing',
    status: 'completed',
    conclusion: 'success',
    steps: [{ name: 'Deploy landing to Fly.io' }],
  })
  scenario.ghRunView['202'].json.jobs = scenario.ghRunView['202'].json.jobs.map((job) => {
    if (job.name !== 'Post-deploy health checks') {
      return job
    }
    return {
      ...job,
      steps: [...job.steps, { name: 'Verify hyperpush landing' }],
    }
  })

  const { tmpRoot, dataPath, verifyRoot } = createHostedVerifierFixture(t, scenario)
  const result = runHostedVerifier(tmpRoot, dataPath, { GH_TOKEN: 'test-gh-token' })

  assert.notEqual(result.status, 0)
  const deployServices = findWorkflow(readJsonArtifact(verifyRoot, 'remote-runs.json'), 'deploy-services.yml')
  assert.equal(deployServices.status, 'failed')
  assert.match(deployServices.failure, /forbidden jobs|forbidden steps/)
})

test('hosted verifier fails closed when remote workflow discovery reports a missing authoritative workflow', (t) => {
  const scenario = defaultHostedEvidenceScenario()
  scenario.ghRunList['authoritative-verification.yml|push|main'] = {
    exitCode: 1,
    stderr: 'could not find any workflows named authoritative-verification.yml\n',
  }

  const { tmpRoot, dataPath, verifyRoot } = createHostedVerifierFixture(t, scenario)
  const result = runHostedVerifier(tmpRoot, dataPath, { GH_TOKEN: 'test-gh-token' })

  assert.notEqual(result.status, 0)
  const authoritative = findWorkflow(readJsonArtifact(verifyRoot, 'remote-runs.json'), 'authoritative-verification.yml')
  assert.equal(authoritative.status, 'failed')
  assert.match(authoritative.failure, /missing on the remote default branch/)
  assert.ok(fs.existsSync(path.join(verifyRoot, 'authoritative-verification-list.log')), 'missing-workflow failures should keep query logs')
})

test('hosted verifier fails closed when gh run view returns malformed JSON', (t) => {
  const scenario = defaultHostedEvidenceScenario()
  scenario.ghRunView['303'] = {
    exitCode: 0,
    stdout: '{not-json\n',
  }

  const { tmpRoot, dataPath, verifyRoot } = createHostedVerifierFixture(t, scenario)
  const result = runHostedVerifier(tmpRoot, dataPath, { GH_TOKEN: 'test-gh-token' })

  assert.notEqual(result.status, 0)
  const release = findWorkflow(readJsonArtifact(verifyRoot, 'remote-runs.json'), 'release.yml')
  assert.equal(release.status, 'failed')
  assert.match(release.failure, /gh run view output was not valid JSON/)
})

test('hosted verifier artifact contract fails closed when remote-runs.json is removed before the final check', (t) => {
  const scenario = defaultHostedEvidenceScenario()
  const { tmpRoot, dataPath, verifyRoot } = createHostedVerifierFixture(t, scenario)

  const mutatedScript = readFrom(tmpRoot, hostedVerifierPath).replace(
    'run_remote_evidence\nrun_artifact_contract',
    'run_remote_evidence\nrm -f "$REMOTE_RUNS_PATH"\nrun_artifact_contract',
  )
  writeTo(tmpRoot, hostedVerifierPath, mutatedScript)

  const result = runHostedVerifier(tmpRoot, dataPath, { GH_TOKEN: 'test-gh-token' })
  assert.notEqual(result.status, 0)
  assert.equal(readVerifyArtifact(verifyRoot, 'status.txt').trim(), 'failed')
  assert.equal(readVerifyArtifact(verifyRoot, 'current-phase.txt').trim(), 'artifact-contract')
  assert.match(readVerifyArtifact(verifyRoot, 'phase-report.txt'), /^artifact-contract\tfailed$/m)
  assert.match(result.stdout, /hosted verifier artifact contract drifted/)
  assert.match(result.stdout, /missing required artifact remote-runs\.json/)
})
