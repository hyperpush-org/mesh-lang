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
  workspace: 'WORKSPACE.md',
  readme: 'mesher/README.md',
  toolchain: 'mesher/scripts/lib/mesh-toolchain.sh',
  s02Contract: 'scripts/tests/verify-m055-s02-contract.test.mjs',
  repoIdentity: 'scripts/lib/repo-identity.json',
  helper: 'scripts/lib/m055-workspace.sh',
  materializer: 'scripts/materialize-hyperpush-mono.mjs',
  verifyM051: 'scripts/verify-m051-s01.sh',
  verifyM053: 'scripts/verify-m053-s03.sh',
  verifyM055S03: 'scripts/verify-m055-s03.sh',
  verifyM055S04: 'scripts/verify-m055-s04.sh',
}

const hostedRepoSourceMarker = 'repo-identity:scripts/lib/repo-identity.json#languageRepo.slug'

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

function validateS04RetargetContract(baseRoot) {
  const errors = []
  const workspace = readFrom(baseRoot, files.workspace)
  const readme = readFrom(baseRoot, files.readme)
  const toolchain = readFrom(baseRoot, files.toolchain)
  const s02Contract = readFrom(baseRoot, files.s02Contract)
  const helper = readFrom(baseRoot, files.helper)
  const verifyM051 = readFrom(baseRoot, files.verifyM051)
  const verifyM053 = readFrom(baseRoot, files.verifyM053)
  const repoIdentity = JSON.parse(readFrom(baseRoot, files.repoIdentity))

  if (repoIdentity.languageRepo?.workspaceDir !== 'mesh-lang') {
    errors.push(`${files.repoIdentity} language repo workspaceDir drifted from mesh-lang`)
  }
  if (repoIdentity.productRepo?.workspaceDir !== 'hyperpush-mono') {
    errors.push(`${files.repoIdentity} product repo workspaceDir drifted from hyperpush-mono`)
  }
  if (repoIdentity.languageRepo?.slug !== 'hyperpush-org/mesh-lang') {
    errors.push(`${files.repoIdentity} language repo slug drifted from hyperpush-org/mesh-lang`)
  }

  requireIncludes(errors, files.workspace, workspace, [
    'mesh-lang/',
    'hyperpush-mono/',
    '`hyperpush-mono/mesher/...`',
    'Do not flatten the product package to `<workspace>/mesher`',
    '## Mesh-lang compatibility boundaries',
    '`bash scripts/verify-m051-s01.sh` from `mesh-lang/` must resolve the sibling product repo from `M055_HYPERPUSH_ROOT` or the blessed `../hyperpush-mono` root.',
    '`bash scripts/verify-m053-s03.sh` must derive the default language repo slug from `scripts/lib/repo-identity.json`, not from the current `origin` remote.',
  ])

  requireIncludes(errors, files.readme, readme, [
    '`mesh-lang/mesher`',
    '`hyperpush-mono/mesher`',
    'Flattening the product package to `<workspace>/mesher` is stale and unsupported.',
    '`../../mesh-lang` relative to `hyperpush-mono/mesher`',
  ])
  requireExcludes(errors, files.readme, readme, ['a blessed `../mesh-lang` checkout next to the extracted Mesher workspace'])

  requireIncludes(errors, files.toolchain, toolchain, [
    'nested product root',
    'stale direct-sibling',
    'expected sibling mesh-lang repo at',
    'hyperpush-mono',
  ])
  requireExcludes(errors, files.toolchain, toolchain, ['local sibling_root="$MESHER_PACKAGE_DIR/../mesh-lang"'])

  requireIncludes(errors, files.s02Contract, s02Contract, [
    "installPackage(tmpRoot, 'hyperpush-mono/mesher')",
    'missing the blessed sibling mesh-lang checkout',
    'mixed direct-sibling and nested sibling mesh-lang roots as drift',
    'source=sibling-workspace',
  ])

  requireIncludes(errors, files.helper, helper, [
    'M055_HYPERPUSH_ROOT',
    'mesher/scripts/verify-maintainer-surface.sh',
    'languageRepo.slug',
    hostedRepoSourceMarker,
    'stale in-repo mesher path',
    'not authoritative',
  ])

  requireIncludes(errors, files.verifyM051, verifyM051, [
    'source "$ROOT_DIR/scripts/lib/m055-workspace.sh"',
    'm055_resolve_hyperpush_root',
    'resolved product repo root',
    'M055_HYPERPUSH_ROOT_SOURCE',
  ])
  requireExcludes(errors, files.verifyM051, verifyM051, [
    '$ROOT_DIR/mesher/scripts/verify-maintainer-surface.sh',
  ])

  requireIncludes(errors, files.verifyM053, verifyM053, [
    'source "$ROOT_DIR/scripts/lib/m055-workspace.sh"',
    'm055_resolve_language_repo_slug',
    'repositorySource',
    'repository: ${GH_REPO} (source=${GH_REPO_SOURCE})',
  ])
  requireExcludes(errors, files.verifyM053, verifyM053, [
    "remote', 'get-url', 'origin'",
    'could not derive GitHub repo slug from origin remote',
  ])

  return errors
}

function validateS04AssembledVerifierContract(baseRoot) {
  const errors = []
  const materializer = readFrom(baseRoot, files.materializer)
  const verifyM055S03 = readFrom(baseRoot, files.verifyM055S03)
  const verifyM055S04 = readFrom(baseRoot, files.verifyM055S04)
  const contractTest = readFrom(baseRoot, 'scripts/tests/verify-m055-s04-contract.test.mjs')

  requireIncludes(errors, files.materializer, materializer, [
    "defaultOutputRoot = path.join(repoRoot, '.tmp', 'm055-s04', 'workspace', 'hyperpush-mono')",
    "version: 'm055-s04-materialize-v1'",
    '[m055-s04] phase=materialize mode=${summary.mode} result=pass',
  ])

  requireIncludes(errors, files.verifyM055S03, verifyM055S03, [
    'ARTIFACT_ROOT=".tmp/m055-s03"',
    'verify-m055-s03: ok',
    'latest-proof-bundle.txt',
  ])

  requireIncludes(errors, files.verifyM055S04, verifyM055S04, [
    'ARTIFACT_ROOT=".tmp/m055-s04"',
    'WORKSPACE_ROOT="$ARTIFACT_ROOT/workspace"',
    'STAGED_PRODUCT_ROOT="$ROOT_DIR/$WORKSPACE_ROOT/hyperpush-mono"',
    'STAGED_PRODUCT_SUMMARY_PATH="$ROOT_DIR/$WORKSPACE_ROOT/hyperpush-mono.stage.json"',
    'STAGED_PRODUCT_MANIFEST_PATH="$ROOT_DIR/$WORKSPACE_ROOT/hyperpush-mono.manifest.json"',
    'REAL_PRODUCT_ROOT=""',
    'LANGUAGE_REPO_METADATA_PATH="$ARTIFACT_DIR/language-repo.meta.json"',
    'PRODUCT_REPO_METADATA_PATH="$ARTIFACT_DIR/product-repo.meta.json"',
    'LANGUAGE_PROOF_BUNDLE_POINTER_PATH="$ARTIFACT_DIR/language-proof-bundle.txt"',
    'PRODUCT_PROOF_BUNDLE_POINTER_PATH="$ARTIFACT_DIR/product-proof-bundle.txt"',
    'RETAINED_PROOF_BUNDLE_DIR="$ARTIFACT_DIR/retained-proof-bundle"',
    'node scripts/materialize-hyperpush-mono.mjs --check',
    'if ! m055_resolve_hyperpush_root "$ROOT_DIR" >"$ARTIFACT_DIR/init.product-root.path"; then',
    'REAL_PRODUCT_ROOT="$M055_HYPERPUSH_ROOT_RESOLVED"',
    'require_git_tracked() {',
    "require_git_tracked init \"$REAL_PRODUCT_ROOT\" 'scripts/verify-m051-s01.sh'",
    "require_git_tracked init \"$REAL_PRODUCT_ROOT\" '.github/workflows/deploy-landing.yml'",
    "require_git_tracked init \"$REAL_PRODUCT_ROOT\" 'scripts/verify-landing-surface.sh'",
    "require_git_tracked init \"$REAL_PRODUCT_ROOT\" 'mesher/scripts/verify-maintainer-surface.sh'",
    "bash -c 'cd \"$1\" && bash scripts/verify-m051-s01.sh' _ \"$REAL_PRODUCT_ROOT\"",
    "bash -c 'cd \"$1\" && bash scripts/verify-landing-surface.sh' _ \"$REAL_PRODUCT_ROOT\"",
    "bash -c 'cd \"$1\" && M055_HYPERPUSH_ROOT=\"$2\" bash scripts/verify-m055-s03.sh' _ \"$ROOT_DIR\" \"$REAL_PRODUCT_ROOT\"",
    'copy_pointed_bundle_or_fail',
    'capture_repo_metadata_or_fail',
    'language ref=',
    'product ref=',
    "'repoRole': 'language'",
    "'repoRole': 'product'",
    "'refSource': 'git:rev-parse:HEAD'",
    "'repoRootSource': product_root_source",
    "'materializeCheckOutputRoot': product_output_root",
    "'materializeCheckManifestFingerprint': product_fingerprint",
    'verifierEntrypoints',
    "'scripts/verify-m051-s01.sh'",
    "'scripts/verify-landing-surface.sh'",
    'language-proof-bundle',
    'product-proof-bundle',
    'retained-m055-s03-verify',
    'retained-m055-s03-proof-bundle',
    'retained-product-m051-s01-verify',
    'retained-product-m051-s01-proof-bundle',
    'verify-m051-s01.sh',
    'mesher.README.md',
    'mesher.env.example',
    'e2e_m051_s01.rs',
    'retained-m051-s01-artifacts',
    'retained-product-landing-surface-verify',
    'product-stage-summary.json',
    'product-stage-manifest.json',
    'retained-m055-s04-artifacts.manifest.txt',
    'verify-m055-s04: ok',
  ])
  requireOrder(errors, files.verifyM055S04, verifyM055S04, [
    'run_expect_success materialize-hyperpush',
    'run_expect_success product-m051-wrapper',
    'run_expect_success product-landing-wrapper',
    'run_expect_success language-m055-s03-wrapper',
    'begin_phase retain-language-m055-s03-verify',
    'begin_phase retain-language-m055-s03-proof-bundle',
    'begin_phase retain-product-m051-s01-verify',
    'begin_phase retain-product-m051-s01-proof-bundle',
    'begin_phase retain-product-landing-surface-verify',
    'begin_phase repo-metadata',
    'begin_phase m055-s04-bundle-shape',
  ])
  requireExcludes(errors, files.verifyM055S04, verifyM055S04, [
    '$ROOT_DIR/mesher/scripts/verify-maintainer-surface.sh',
    'git -C "$STAGED_PRODUCT_ROOT" rev-parse HEAD',
    'product ref=materialized:',
    "'ref': f'materialized:{product_fingerprint}'",
    'materializedFromLanguageRepoRef',
  ])

  requireIncludes(errors, 'scripts/tests/verify-m055-s04-contract.test.mjs', contractTest, [
    'validateS04AssembledVerifierContract',
    'current sources keep the M055 S04 assembled two-repo verifier contract intact',
    'contract fails closed when the M055 S04 verifier loses repo attribution metadata or product-root phases',
  ])

  return errors
}

function makeMinimalMeshLangRoot(baseRoot) {
  for (const relativePath of [files.repoIdentity, files.helper, files.verifyM051, files.verifyM053]) {
    copyRepoFile(baseRoot, relativePath)
  }
}

function makeStaleLocalVerifier(baseRoot) {
  writeExecutable(
    baseRoot,
    'mesher/scripts/verify-maintainer-surface.sh',
    `#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "\${BASH_SOURCE[0]}")/../.." && pwd)"
VERIFY_DIR="$ROOT_DIR/.tmp/m051-s01/verify"
mkdir -p "$VERIFY_DIR/proof-bundle"
printf 'ok\n' >"$VERIFY_DIR/status.txt"
printf 'complete\n' >"$VERIFY_DIR/current-phase.txt"
cat >"$VERIFY_DIR/phase-report.txt" <<'EOF'
init\tpassed
mesher-package-tests\tpassed
mesher-package-build\tpassed
mesher-postgres-start\tpassed
mesher-migrate-status\tpassed
mesher-migrate-up\tpassed
mesher-runtime-smoke\tpassed
mesher-bundle-shape\tpassed
EOF
printf 'local stale verifier\n' >"$VERIFY_DIR/full-contract.log"
printf '%s\n' "$VERIFY_DIR/proof-bundle" >"$VERIFY_DIR/latest-proof-bundle.txt"
printf 'stale local verifier\n' >"$ROOT_DIR/local-stale-verifier-ran.txt"
echo 'verify-maintainer-surface: ok (local stale)'
`,
  )
}

function makeSiblingProductVerifier(productRoot) {
  writeExecutable(
    productRoot,
    'mesher/scripts/verify-maintainer-surface.sh',
    `#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "\${BASH_SOURCE[0]}")/../.." && pwd)"
VERIFY_DIR="$ROOT_DIR/.tmp/m051-s01/verify"
mkdir -p "$VERIFY_DIR/proof-bundle"
printf 'ok\n' >"$VERIFY_DIR/status.txt"
printf 'complete\n' >"$VERIFY_DIR/current-phase.txt"
cat >"$VERIFY_DIR/phase-report.txt" <<'EOF'
init\tpassed
mesher-package-tests\tpassed
mesher-package-build\tpassed
mesher-postgres-start\tpassed
mesher-migrate-status\tpassed
mesher-migrate-up\tpassed
mesher-runtime-smoke\tpassed
mesher-bundle-shape\tpassed
EOF
printf 'delegated verifier from %s\n' "$ROOT_DIR" >"$VERIFY_DIR/full-contract.log"
printf '%s\n' "$VERIFY_DIR/proof-bundle" >"$VERIFY_DIR/latest-proof-bundle.txt"
printf '%s\n' "$ROOT_DIR" >"$ROOT_DIR/delegated-product-root.txt"
echo 'verify-maintainer-surface: ok'
`,
  )
}

function runScript(baseRoot, relativeScriptPath, env = {}) {
  return spawnSync('bash', [path.join(baseRoot, relativeScriptPath)], {
    cwd: baseRoot,
    encoding: 'utf8',
    env: { ...process.env, ...env },
  })
}

function createHostedVerifierRoot(t) {
  const tempRoot = mkTmpDir(t, 'verify-m055-s04-hosted-')
  makeMinimalMeshLangRoot(tempRoot)

  writeTo(
    tempRoot,
    'compiler/meshc/Cargo.toml',
    '[package]\nname = "meshc"\nversion = "1.2.3"\n',
  )
  writeTo(
    tempRoot,
    'compiler/meshpkg/Cargo.toml',
    '[package]\nname = "meshpkg"\nversion = "1.2.3"\n',
  )
  for (const workflow of ['authoritative-verification.yml', 'deploy-services.yml', 'release.yml']) {
    writeTo(tempRoot, `.github/workflows/${workflow}`, `name: ${workflow}\n`)
  }

  const fakeBin = path.join(tempRoot, 'fake-bin')
  fs.mkdirSync(fakeBin, { recursive: true })

  writeExecutable(
    tempRoot,
    'fake-bin/git',
    `#!/usr/bin/env bash
set -euo pipefail
if [[ "$1" != 'ls-remote' ]]; then
  echo "unexpected git command: $*" >&2
  exit 97
fi
for arg in "$@"; do
  case "$arg" in
    refs/heads/main)
      printf '1111111111111111111111111111111111111111 %s\n' "$arg"
      ;;
    refs/tags/v1.2.3)
      printf '2222222222222222222222222222222222222222 %s\n' "$arg"
      ;;
    'refs/tags/v1.2.3^{}')
      printf '3333333333333333333333333333333333333333 %s\n' "$arg"
      ;;
  esac
done
`,
  )

  writeExecutable(
    tempRoot,
    'fake-bin/gh',
    `#!/usr/bin/env bash
set -euo pipefail
command="$1"
subcommand="$2"
shift 2
workflow=''
branch=''
run_id=''
while (($#)); do
  case "$1" in
    --workflow)
      workflow="$2"
      shift 2
      ;;
    --branch)
      branch="$2"
      shift 2
      ;;
    --json|--event|--limit|-R)
      shift 2
      ;;
    101|202|303)
      run_id="$1"
      shift 1
      ;;
    *)
      shift 1
      ;;
  esac
done
if [[ "$command $subcommand" == 'run list' ]]; then
  case "$workflow:$branch" in
    authoritative-verification.yml:main)
      printf '[{"databaseId":101,"workflowName":"authoritative-verification.yml","event":"push","status":"completed","conclusion":"success","headBranch":"main","headSha":"1111111111111111111111111111111111111111","displayTitle":"authoritative","createdAt":"2026-04-07T00:00:00Z","url":"https://example.test/101"}]\n'
      ;;
    deploy-services.yml:main)
      printf '[{"databaseId":202,"workflowName":"deploy-services.yml","event":"push","status":"completed","conclusion":"success","headBranch":"main","headSha":"1111111111111111111111111111111111111111","displayTitle":"deploy","createdAt":"2026-04-07T00:00:00Z","url":"https://example.test/202"}]\n'
      ;;
    release.yml:v1.2.3)
      printf '[{"databaseId":303,"workflowName":"release.yml","event":"push","status":"completed","conclusion":"success","headBranch":"v1.2.3","headSha":"3333333333333333333333333333333333333333","displayTitle":"release","createdAt":"2026-04-07T00:00:00Z","url":"https://example.test/303"}]\n'
      ;;
    *)
      printf '[]\n'
      ;;
  esac
  exit 0
fi
if [[ "$command $subcommand" == 'run view' ]]; then
  case "$run_id" in
    101)
      printf '{"databaseId":101,"workflowName":"authoritative-verification.yml","event":"push","status":"completed","conclusion":"success","headBranch":"main","headSha":"1111111111111111111111111111111111111111","displayTitle":"authoritative","url":"https://example.test/101","jobs":[{"name":"Authoritative starter failover proof","status":"completed","conclusion":"success","steps":[{"name":"Starter failover"}]}]}\n'
      ;;
    202)
      printf '{"databaseId":202,"workflowName":"deploy-services.yml","event":"push","status":"completed","conclusion":"success","headBranch":"main","headSha":"1111111111111111111111111111111111111111","displayTitle":"deploy","url":"https://example.test/202","jobs":[{"name":"Deploy mesh-registry","status":"completed","conclusion":"success","steps":[{"name":"Deploy mesh-registry"}]},{"name":"Deploy mesh-packages website","status":"completed","conclusion":"success","steps":[{"name":"Deploy mesh-packages website"}]},{"name":"Post-deploy health checks","status":"completed","conclusion":"success","steps":[{"name":"Verify public surface contract"}]}]}\n'
      ;;
    303)
      printf '{"databaseId":303,"workflowName":"release.yml","event":"push","status":"completed","conclusion":"success","headBranch":"v1.2.3","headSha":"3333333333333333333333333333333333333333","displayTitle":"release","url":"https://example.test/303","jobs":[{"name":"Authoritative starter failover proof","status":"completed","conclusion":"success","steps":[{"name":"Starter failover"}]},{"name":"Create Release","status":"completed","conclusion":"success","steps":[{"name":"Create Release"}]}]}\n'
      ;;
    *)
      echo "unexpected run view id: $run_id" >&2
      exit 98
      ;;
  esac
  exit 0
fi
echo "unexpected gh command: $command $subcommand" >&2
exit 99
`,
  )

  return { tempRoot, fakeBin }
}

test('current sources keep the sibling product-root wrapper and repo-identity hosted verifier contract explicit', () => {
  const errors = validateS04RetargetContract(root)
  assert.deepEqual(errors, [], errors.join('\n'))
})

test('contract fails closed when workspace docs or verifiers drift back to local Mesher delegation or origin-derived repo slugs', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m055-s04-contract-')
  for (const relativePath of Object.values(files)) {
    copyRepoFile(tmpRoot, relativePath)
  }

  writeTo(
    tmpRoot,
    files.workspace,
    readFrom(tmpRoot, files.workspace).replace('## Mesh-lang compatibility boundaries', '## Drifted compatibility notes'),
  )
  writeTo(
    tmpRoot,
    files.helper,
    readFrom(tmpRoot, files.helper).replace('repo-identity:scripts/lib/repo-identity.json#languageRepo.slug', 'origin-remote'),
  )
  writeTo(
    tmpRoot,
    files.verifyM051,
    readFrom(tmpRoot, files.verifyM051).replace('m055_resolve_hyperpush_root "$ROOT_DIR"', 'printf %s "$ROOT_DIR/mesher"'),
  )
  writeTo(
    tmpRoot,
    files.verifyM053,
    readFrom(tmpRoot, files.verifyM053)
      .replace('m055_resolve_language_repo_slug "$ROOT_DIR"', 'printf %s "hyperpush-org/hyperpush-mono"')
      .concat("\n# stale fallback: git remote get-url origin\n"),
  )

  const errors = validateS04RetargetContract(tmpRoot)
  assert.ok(
    errors.some((error) => error.includes('Mesh-lang compatibility boundaries') || error.includes('origin remote') || error.includes('$ROOT_DIR/mesher') || error.includes('git remote get-url origin')),
    errors.join('\n'),
  )
})

test('verify-m051-s01 delegates only to the sibling product repo when M055_HYPERPUSH_ROOT is set', (t) => {
  const workspaceRoot = mkTmpDir(t, 'verify-m055-s04-wrapper-')
  const meshLangRoot = path.join(workspaceRoot, 'mesh-lang')
  const productRoot = path.join(workspaceRoot, 'hyperpush-mono')
  fs.mkdirSync(meshLangRoot, { recursive: true })
  fs.mkdirSync(productRoot, { recursive: true })

  makeMinimalMeshLangRoot(meshLangRoot)
  makeStaleLocalVerifier(meshLangRoot)
  makeSiblingProductVerifier(productRoot)

  const result = runScript(meshLangRoot, files.verifyM051, {
    M055_HYPERPUSH_ROOT: productRoot,
  })

  assert.equal(result.status, 0, `${result.stdout}\n${result.stderr}`)
  assert.match(result.stdout, /resolved product repo root:/)
  assert.match(result.stdout, /source=env:M055_HYPERPUSH_ROOT/)
  assert.match(result.stdout, new RegExp(productRoot.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')))
  assert.ok(fs.existsSync(path.join(productRoot, 'delegated-product-root.txt')), 'expected sibling product verifier to run')
  assert.ok(!fs.existsSync(path.join(meshLangRoot, 'local-stale-verifier-ran.txt')), 'stale local Mesher verifier should not run')
})

test('verify-m051-s01 fails closed when only the stale in-repo mesher path exists', (t) => {
  const workspaceRoot = mkTmpDir(t, 'verify-m055-s04-local-drift-')
  const meshLangRoot = path.join(workspaceRoot, 'mesh-lang')
  fs.mkdirSync(meshLangRoot, { recursive: true })

  makeMinimalMeshLangRoot(meshLangRoot)
  makeStaleLocalVerifier(meshLangRoot)

  const result = runScript(meshLangRoot, files.verifyM051)

  assert.notEqual(result.status, 0, result.stdout)
  assert.match(result.stderr, /stale in-repo mesher path/)
  assert.match(result.stderr, /not authoritative/)
  assert.match(result.stderr, /source=blessed-sibling:mesh-lang->hyperpush-mono/)
})

test('verify-m053-s03 derives the default repo slug from repo identity and records the source in retained artifacts', (t) => {
  const { tempRoot, fakeBin } = createHostedVerifierRoot(t)
  const verifyRoot = path.join(tempRoot, '.tmp', 'm053-s03-test')

  const result = runScript(tempRoot, files.verifyM053, {
    GH_TOKEN: 'test-gh-token',
    M053_S03_VERIFY_ROOT: verifyRoot,
    M053_S03_GH_BIN: path.join(fakeBin, 'gh'),
    M053_S03_GIT_BIN: path.join(fakeBin, 'git'),
  })

  assert.equal(result.status, 0, `${result.stdout}\n${result.stderr}`)
  assert.match(result.stdout, /verify-m053-s03: ok/)
  assert.match(
    result.stdout,
    new RegExp(`repository: hyperpush-org/mesh-lang \\(source=${hostedRepoSourceMarker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}\\)`),
  )

  const candidateRefs = JSON.parse(fs.readFileSync(path.join(verifyRoot, 'candidate-refs.json'), 'utf8'))
  const remoteRuns = JSON.parse(fs.readFileSync(path.join(verifyRoot, 'remote-runs.json'), 'utf8'))
  assert.equal(candidateRefs.repository, 'hyperpush-org/mesh-lang')
  assert.equal(candidateRefs.repositorySource, hostedRepoSourceMarker)
  assert.equal(remoteRuns.repository, 'hyperpush-org/mesh-lang')
  assert.equal(remoteRuns.repositorySource, hostedRepoSourceMarker)
  assert.equal(remoteRuns.workflows.length, 3)

  const preflightLog = fs.readFileSync(path.join(verifyRoot, 'gh-preflight.log'), 'utf8')
  assert.match(preflightLog, /source=repo-identity:scripts\/lib\/repo-identity\.json#languageRepo\.slug/)
})

test('current sources keep the M055 S04 assembled two-repo verifier contract intact', () => {
  const errors = validateS04AssembledVerifierContract(root)
  assert.deepEqual(errors, [], errors.join('\n'))
})

test('contract fails closed when the M055 S04 verifier loses repo attribution metadata or product-root phases', (t) => {
  const tmpRoot = mkTmpDir(t, 'verify-m055-s04-assembled-')
  for (const relativePath of [files.materializer, files.repoIdentity, files.verifyM055S03, files.verifyM055S04]) {
    copyRepoFile(tmpRoot, relativePath)
  }
  copyRepoFile(tmpRoot, 'scripts/tests/verify-m055-s04-contract.test.mjs')

  let verifier = readFrom(tmpRoot, files.verifyM055S04)
  verifier = verifier.replace(
    `run_expect_success product-landing-wrapper product-landing-wrapper 300 "$PRODUCT_LANDING_VERIFY_DIR" \\
  bash -c 'cd "$1" && bash scripts/verify-landing-surface.sh' _ "$REAL_PRODUCT_ROOT"\n`,
    '',
  )
  verifier = verifier.replace('PRODUCT_REPO_METADATA_PATH="$ARTIFACT_DIR/product-repo.meta.json"', 'PRODUCT_REPO_METADATA_PATH="$ARTIFACT_DIR/product.repo.json"')
  verifier = verifier.replace('PRODUCT_PROOF_BUNDLE_POINTER_PATH="$ARTIFACT_DIR/product-proof-bundle.txt"', 'PRODUCT_PROOF_BUNDLE_POINTER_PATH="$ARTIFACT_DIR/product.bundle.txt"')
  writeTo(tmpRoot, files.verifyM055S04, verifier)

  const errors = validateS04AssembledVerifierContract(tmpRoot)
  assert.ok(
    errors.some((error) => error.includes('product-landing-wrapper') || error.includes('product-repo.meta.json') || error.includes('product-proof-bundle.txt')),
    errors.join('\n'),
  )
})
