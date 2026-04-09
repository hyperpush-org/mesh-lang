import test from 'node:test'
import assert from 'node:assert/strict'
import fs from 'node:fs'
import os from 'node:os'
import path from 'node:path'
import { spawnSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'

const scriptDir = path.dirname(fileURLToPath(import.meta.url))
const root = path.resolve(scriptDir, '..', '..')

const files = {
  workflow: '.github/workflows/deploy-services.yml',
  repoIdentity: 'scripts/lib/repo-identity.json',
  helper: 'scripts/lib/m034_public_surface_contract.py',
  m034WorkflowVerifier: 'scripts/verify-m034-s05-workflows.sh',
  m034Verifier: 'scripts/verify-m034-s05.sh',
  m053Verifier: 'scripts/verify-m053-s03.sh',
  verifier: 'scripts/verify-m055-s03.sh',
  contractTest: 'scripts/tests/verify-m055-s03-contract.test.mjs',
}

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

function mkTmpDir(t, prefix) {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), prefix))
  t.after(() => fs.rmSync(dir, { recursive: true, force: true }))
  return dir
}

function describeContract(baseRoot) {
  const result = spawnSync('python3', [path.join(baseRoot, files.helper), 'describe'], {
    cwd: baseRoot,
    encoding: 'utf8',
  })
  assert.equal(result.status, 0, result.stderr || result.stdout)
  return JSON.parse(result.stdout)
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
      errors.push(`${relativePath} still contains ${JSON.stringify(needle)}`)
    }
  }
}

function requireOrder(errors, relativePath, text, markers) {
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

function validateLanguageOwnedDeployContract(baseRoot) {
  const errors = []
  const contract = describeContract(baseRoot)
  const workflow = readFrom(baseRoot, files.workflow)
  const m034WorkflowVerifier = readFrom(baseRoot, files.m034WorkflowVerifier)
  const m034Verifier = readFrom(baseRoot, files.m034Verifier)
  const m053Verifier = readFrom(baseRoot, files.m053Verifier)

  assert.equal(contract.contractVersion, 'm034-s07-public-surface-v2')
  assert.deepEqual(contract.workflowContract.deployServicesJobs, [
    'deploy-registry',
    'deploy-packages-website',
    'health-check',
  ])
  assert.deepEqual(contract.workflowContract.deployServicesJobNames, [
    'Deploy mesh-registry',
    'Deploy mesh-packages website',
    'Post-deploy health checks',
  ])
  assert.deepEqual(contract.workflowContract.deployServicesHealthCheckSteps, ['Verify public surface contract'])
  assert.deepEqual(contract.workflowContract.deployServicesForbiddenJobNames, ['Deploy hyperpush landing'])
  assert.deepEqual(contract.workflowContract.deployServicesForbiddenHealthCheckSteps, ['Verify hyperpush landing'])
  assert.equal(contract.workflowContract.deployServicesRequiredHeadBranch, 'main')
  assert.equal(contract.workflowContract.deployServicesExpectedRef, 'refs/heads/main')

  requireIncludes(errors, files.workflow, workflow, [
    'name: Deploy Services to Fly.io',
    'deploy-registry:',
    'name: Deploy mesh-registry',
    'working-directory: registry',
    'deploy-packages-website:',
    'name: Deploy mesh-packages website',
    'working-directory: packages-website',
    'health-check:',
    'needs: [deploy-registry, deploy-packages-website]',
    '- name: Verify public surface contract',
    contract.workflowContract.deployServicesCommand,
  ])
  requireExcludes(errors, files.workflow, workflow, [
    'deploy-hyperpush-landing:',
    'Deploy hyperpush landing',
    'Verify hyperpush landing',
    'https://hyperpush-landing.fly.dev',
    '/api/blog/posts',
    '/community/blog',
  ])

  requireIncludes(errors, files.m034WorkflowVerifier, m034WorkflowVerifier, [
    'jobs.keys == ["deploy-registry", "deploy-packages-website", "health-check"]',
    'deploy-services workflow must define deploy-registry, deploy-packages-website, and health-check jobs',
    'expected_needs = %w[deploy-packages-website deploy-registry]',
    'health-check job must depend on deploy-registry and deploy-packages-website',
    'health-check job must only keep Checkout and Verify public surface contract steps',
    'deploy-services workflow must not keep pre-S07 inline public proof logic',
  ])
  requireExcludes(errors, files.m034WorkflowVerifier, m034WorkflowVerifier, [
    'verify_fly_deploy_job.call("deploy-hyperpush-landing"',
    'health-check job must verify the hyperpush landing deployment',
    'Verify hyperpush landing step must probe the Fly hostname',
  ])

  requireIncludes(errors, files.m034Verifier, m034Verifier, [
    "'workflowFile': 'deploy-services.yml'",
    "'requiredHeadBranch': 'main'",
    "'expectedRef': 'refs/heads/main'",
    "'Deploy mesh-registry'",
    "'Deploy mesh-packages website'",
    "'Post-deploy health checks'",
    "'Verify public surface contract'",
    "'forbiddenJobs': ['Deploy hyperpush landing']",
    "'forbiddenSteps': {",
    'hosted run still includes forbidden jobs',
    'hosted run still includes forbidden steps',
  ])

  requireIncludes(errors, files.m053Verifier, m053Verifier, [
    "'workflowFile': 'deploy-services.yml'",
    "'requiredHeadBranch': 'main'",
    "'expectedRef': 'refs/heads/main'",
    "'requiredJobs': ['Deploy mesh-registry', 'Deploy mesh-packages website', 'Post-deploy health checks']",
    "'forbiddenJobs': ['Deploy hyperpush landing']",
    "'forbiddenSteps': {",
    'hosted run still includes forbidden jobs',
    'hosted run still includes forbidden steps',
    "'forbiddenJobs'",
    "'forbiddenSteps'",
  ])
  requireExcludes(errors, files.m053Verifier, m053Verifier, [
    "'requiredJobs': ['Deploy mesh-packages website', 'Post-deploy health checks']",
  ])

  return errors
}

function validateS03VerifierContract(baseRoot) {
  const errors = []
  const verifier = readFrom(baseRoot, files.verifier)
  const contractTest = readFrom(baseRoot, files.contractTest)

  requireIncludes(errors, files.verifier, verifier, [
    'ARTIFACT_ROOT=".tmp/m055-s03"',
    'status.txt',
    'current-phase.txt',
    'phase-report.txt',
    'full-contract.log',
    'latest-proof-bundle.txt',
    'RETAINED_PROOF_BUNDLE_DIR="$ARTIFACT_DIR/retained-proof-bundle"',
    'bash scripts/verify-m055-s01.sh',
    'bash scripts/verify-m050-s02.sh',
    'bash scripts/verify-m050-s03.sh',
    'bash scripts/verify-m051-s04.sh',
    'bash scripts/verify-m034-s05-workflows.sh',
    'python3 scripts/lib/m034_public_surface_contract.py local-docs --root .',
    'npm --prefix packages-website run build',
    'retain-m055-s01-verify',
    'retain-m050-s02-verify',
    'retain-m050-s03-verify',
    'retain-m051-s04-verify',
    'retain-m034-s05-workflows',
    'm055-s03-bundle-shape',
    'deploy-services.yml',
    'verify-m055-s03-contract.test.mjs',
    'printf \'%s\\n\' "$RETAINED_PROOF_BUNDLE_DIR" >"$LATEST_PROOF_BUNDLE_PATH"',
    'assert_retained_bundle_shape',
    'verify-m055-s03: ok',
  ])
  requireOrder(errors, files.verifier, verifier, [
    'run_expect_success m055-s01-wrapper',
    'run_expect_success m050-s02-wrapper',
    'run_expect_success m050-s03-wrapper',
    'run_expect_success m051-s04-wrapper',
    'run_expect_success m034-s05-workflows',
    'run_expect_success local-docs',
    'run_expect_success packages-build',
    'begin_phase retain-m055-s01-verify',
    'begin_phase retain-m050-s02-verify',
    'begin_phase retain-m050-s03-verify',
    'begin_phase retain-m051-s04-verify',
    'begin_phase retain-m034-s05-workflows',
    'begin_phase m055-s03-bundle-shape',
  ])
  requireExcludes(errors, files.verifier, verifier, [
    'mesher/landing',
    'deploy-hyperpush-landing',
    'Verify hyperpush landing',
  ])

  requireIncludes(errors, files.contractTest, contractTest, [
    'validateLanguageOwnedDeployContract',
    'validateS03VerifierContract',
    'current sources keep the language-owned deploy/public-surface graph and hosted verifiers aligned',
    'contract fails closed when landing deploy steps or landing-specific hosted evidence expectations come back',
    'current sources keep the M055 S03 assembled verifier and retained-bundle contract intact',
    'contract fails closed when the M055 S03 verifier loses a required phase or pointer marker',
  ])

  return errors
}

test('current sources keep the language-owned deploy/public-surface graph and hosted verifiers aligned', () => {
  const errors = validateLanguageOwnedDeployContract(root)
  assert.deepEqual(errors, [], errors.join('\n'))
})

test('contract fails closed when landing deploy steps or landing-specific hosted evidence expectations come back', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m055-s03-deploy-')
  for (const relativePath of [files.workflow, files.repoIdentity, files.helper, files.m034WorkflowVerifier, files.m034Verifier, files.m053Verifier]) {
    copyRepoFile(tmpRoot, relativePath)
  }

  let workflow = readFrom(tmpRoot, files.workflow)
  workflow = workflow.replace(
    '  health-check:\n    name: Post-deploy health checks\n    needs: [deploy-registry, deploy-packages-website]\n',
    '  deploy-hyperpush-landing:\n    name: Deploy hyperpush landing\n    runs-on: ubuntu-latest\n    timeout-minutes: 20\n    steps:\n      - name: Checkout\n        uses: actions/checkout@v4\n\n      - name: Setup flyctl\n        uses: superfly/flyctl-actions/setup-flyctl@master\n\n      - name: Deploy landing to Fly.io\n        run: flyctl deploy --remote-only\n        working-directory: mesher/landing\n        env:\n          FLY_API_TOKEN: ${{ secrets.FLY_API_TOKEN }}\n\n  health-check:\n    name: Post-deploy health checks\n    needs: [deploy-registry, deploy-packages-website, deploy-hyperpush-landing]\n',
  )
  workflow = workflow.replace(
    '- name: Verify public surface contract\n        shell: bash\n        timeout-minutes: 8\n        run: |\n          set -euo pipefail\n          python3 scripts/lib/m034_public_surface_contract.py public-http --root "$GITHUB_WORKSPACE" --artifact-dir "$RUNNER_TEMP/m034-public-surface-contract"\n',
    '- name: Verify public surface contract\n        shell: bash\n        timeout-minutes: 8\n        run: |\n          set -euo pipefail\n          python3 scripts/lib/m034_public_surface_contract.py public-http --root "$GITHUB_WORKSPACE" --artifact-dir "$RUNNER_TEMP/m034-public-surface-contract"\n\n      - name: Verify hyperpush landing\n        shell: bash\n        timeout-minutes: 5\n        run: |\n          set -euo pipefail\n          curl -fsSL https://hyperpush-landing.fly.dev > "$RUNNER_TEMP/hyperpush-landing.html"\n          grep -q "hyperpush" "$RUNNER_TEMP/hyperpush-landing.html"\n',
  )
  writeTo(tmpRoot, files.workflow, workflow)

  writeTo(
    tmpRoot,
    files.m034Verifier,
    readFrom(tmpRoot, files.m034Verifier).replace(
      "'forbiddenJobs': ['Deploy hyperpush landing']",
      "'requiredJobs': ['Deploy hyperpush landing']",
    ),
  )
  writeTo(
    tmpRoot,
    files.m053Verifier,
    readFrom(tmpRoot, files.m053Verifier).replace(
      "'forbiddenSteps': {\n            'Post-deploy health checks': ['Verify hyperpush landing'],\n        },",
      "'requiredSteps': {\n            'Post-deploy health checks': ['Verify public surface contract', 'Verify hyperpush landing'],\n        },",
    ),
  )

  const errors = validateLanguageOwnedDeployContract(tmpRoot)
  assert.ok(errors.some((error) => error.includes('deploy-hyperpush-landing') || error.includes('Verify hyperpush landing')), errors.join('\n'))
})

test('current sources keep the M055 S03 assembled verifier and retained-bundle contract intact', () => {
  const errors = validateS03VerifierContract(root)
  assert.deepEqual(errors, [], errors.join('\n'))
})

test('contract fails closed when the M055 S03 verifier loses a required phase or pointer marker', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m055-s03-wrapper-')
  for (const relativePath of [files.verifier, files.contractTest]) {
    copyRepoFile(tmpRoot, relativePath)
  }

  let verifier = readFrom(tmpRoot, files.verifier)
  verifier = verifier.replace(
    'run_expect_success m051-s04-wrapper m051-s04-wrapper 5400 ".tmp/m051-s04/verify" \\\n  bash scripts/verify-m051-s04.sh\n',
    '',
  )
  verifier = verifier.replace('latest-proof-bundle.txt', 'bundle-pointer.txt')
  writeTo(tmpRoot, files.verifier, verifier)

  const errors = validateS03VerifierContract(tmpRoot)
  assert.ok(errors.some((error) => error.includes('m051-s04-wrapper') || error.includes('latest-proof-bundle.txt')), errors.join('\n'))
})
