#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

ARTIFACT_DIR=".tmp/m034-s04/workflows"
REUSABLE_WORKFLOW_PATH=".github/workflows/extension-release-proof.yml"
PUBLISH_WORKFLOW_PATH=".github/workflows/publish-extension.yml"
PHASE_REPORT_PATH="$ARTIFACT_DIR/phase-report.txt"
mkdir -p "$ARTIFACT_DIR"
: >"$PHASE_REPORT_PATH"

record_phase() {
  local phase_name="$1"
  local status="$2"
  printf '%s\t%s\n' "$phase_name" "$status" >>"$PHASE_REPORT_PATH"
}

fail_with_log() {
  local phase_name="$1"
  local command_text="$2"
  local reason="$3"
  local log_path="${4:-}"

  record_phase "$phase_name" "failed"
  echo "verification drift: ${reason}" >&2
  echo "first failing phase: ${phase_name}" >&2
  echo "failing command: ${command_text}" >&2
  echo "artifacts: ${ARTIFACT_DIR}" >&2
  if [[ -n "$log_path" && -f "$log_path" ]]; then
    echo "--- ${log_path} ---" >&2
    sed -n '1,320p' "$log_path" >&2
  fi
  exit 1
}

run_reusable_contract_check() {
  local phase_name="reusable"
  local command_text="ruby reusable workflow contract sweep ${REUSABLE_WORKFLOW_PATH}"
  local log_path="$ARTIFACT_DIR/reusable.log"

  record_phase "$phase_name" "started"
  echo "==> [${phase_name}] ${command_text}"
  if ! ruby - "$REUSABLE_WORKFLOW_PATH" "$ROOT_DIR" >"$log_path" 2>&1 <<'RUBY'
require "yaml"

workflow_path = ARGV.fetch(0)
root_dir = ARGV.fetch(1)
workflow = YAML.load_file(workflow_path)
raw = File.read(workflow_path)

errors = []

errors << "reusable workflow file is missing" unless File.file?(workflow_path)
errors << "workflow name must stay 'Extension release proof'" unless workflow["name"] == "Extension release proof"

on_key = if workflow.key?("on")
  "on"
elsif workflow.key?(true)
  true
else
  "on"
end
on_block = workflow[on_key]
unless on_block.is_a?(Hash) && on_block.keys == ["workflow_call"]
  errors << "workflow must trigger only via workflow_call"
end

call_block = on_block.is_a?(Hash) ? on_block["workflow_call"] : nil
outputs_block = call_block.is_a?(Hash) ? call_block["outputs"] : nil
expected_workflow_outputs = {
  "verified_vsix_path" => "${{ jobs.proof.outputs.verified_vsix_path }}",
  "verified_vsix_artifact_name" => "${{ jobs.proof.outputs.verified_vsix_artifact_name }}",
}
expected_workflow_outputs.each do |output_name, expected_value|
  output = outputs_block.is_a?(Hash) ? outputs_block[output_name] : nil
  unless output.is_a?(Hash) && output["value"] == expected_value
    errors << "workflow_call output #{output_name} must map to #{expected_value.inspect}"
  end
end

permissions = workflow["permissions"]
unless permissions.is_a?(Hash) && permissions == { "contents" => "read" }
  errors << "workflow permissions must stay read-only"
end

jobs = workflow["jobs"]
unless jobs.is_a?(Hash) && jobs.keys == ["proof"]
  errors << "workflow must define exactly one proof job"
end
job = jobs.is_a?(Hash) ? jobs["proof"] : nil
if job.is_a?(Hash)
  errors << "job name must stay 'Verify extension release proof'" unless job["name"] == "Verify extension release proof"
  errors << "job must run on ubuntu-24.04" unless job["runs-on"] == "ubuntu-24.04"
  unless job["timeout-minutes"].is_a?(Integer) && job["timeout-minutes"] >= 30
    errors << "proof job must declare timeout-minutes"
  end

  job_outputs = job["outputs"]
  {
    "verified_vsix_path" => "${{ steps.capture.outputs.verified_vsix_path }}",
    "verified_vsix_artifact_name" => "${{ steps.capture.outputs.verified_vsix_artifact_name }}",
  }.each do |output_name, expected_value|
    unless job_outputs.is_a?(Hash) && job_outputs[output_name] == expected_value
      errors << "proof job output #{output_name} must map to #{expected_value.inspect}"
    end
  end

  steps = job["steps"]
  unless steps.is_a?(Array)
    errors << "proof job must define steps"
    steps = []
  end

  find_step = lambda do |name|
    steps.find { |step| step.is_a?(Hash) && step["name"] == name }
  end

  checkout = find_step.call("Checkout")
  unless checkout.is_a?(Hash) && checkout["uses"] == "actions/checkout@v4"
    errors << "Checkout step must use actions/checkout@v4"
  end

  preflight = find_step.call("Verify extension proof entrypoint")
  unless preflight.is_a?(Hash) && preflight["run"].to_s.include?("test -f scripts/verify-m034-s04-extension.sh")
    errors << "workflow must fail early if scripts/verify-m034-s04-extension.sh is missing"
  end

  setup_node = find_step.call("Set up Node.js")
  if setup_node.is_a?(Hash)
    unless setup_node["uses"] == "actions/setup-node@v4"
      errors << "Set up Node.js step must use actions/setup-node@v4"
    end
    setup_with = setup_node["with"]
    unless setup_with.is_a?(Hash) && setup_with["node-version"] == 20
      errors << "Set up Node.js must pin node-version 20"
    end
    unless setup_with.is_a?(Hash) && setup_with["cache"] == "npm"
      errors << "Set up Node.js must enable npm cache"
    end
    unless setup_with.is_a?(Hash) && setup_with["cache-dependency-path"] == "tools/editors/vscode-mesh/package-lock.json"
      errors << "Set up Node.js cache path must target the extension package-lock"
    end
  else
    errors << "workflow must install Node.js for the extension verifier"
  end

  cache_llvm = find_step.call("Cache LLVM")
  if cache_llvm.is_a?(Hash)
    unless cache_llvm["uses"] == "actions/cache@v4"
      errors << "Cache LLVM step must use actions/cache@v4"
    end
    unless cache_llvm["id"] == "cache-llvm"
      errors << "Cache LLVM step must keep id cache-llvm"
    end
    cache_with = cache_llvm["with"]
    unless cache_with.is_a?(Hash) && cache_with["path"] == "~/llvm"
      errors << "Cache LLVM step must cache ~/llvm"
    end
    unless cache_with.is_a?(Hash) && cache_with["key"] == "llvm-21.1.8-v3-x86_64-unknown-linux-gnu"
      errors << "Cache LLVM key drifted away from the Linux x86_64 bootstrap"
    end
  else
    errors << "workflow must cache the LLVM toolchain"
  end

  install_llvm = find_step.call("Install LLVM 21 (Linux x86_64)")
  if install_llvm.is_a?(Hash)
    install_run = install_llvm["run"].to_s
    unless install_llvm["if"].to_s.include?("steps.cache-llvm.outputs.cache-hit != 'true'")
      errors << "LLVM install step must skip when the cache hits"
    end
    unless install_llvm["timeout-minutes"].is_a?(Integer) && install_llvm["timeout-minutes"] >= 5
      errors << "LLVM install step must declare timeout-minutes"
    end
    [
      'LLVM_VERSION="21.1.8"',
      'LLVM_ARCHIVE="LLVM-${LLVM_VERSION}-Linux-X64.tar.xz"',
      'llvmorg-${LLVM_VERSION}',
      'tar xf llvm.tar.xz --strip-components=1 -C "$HOME/llvm"',
    ].each do |needle|
      errors << "LLVM install step missing #{needle}" unless install_run.include?(needle)
    end
  else
    errors << "workflow must install LLVM 21 for Linux x86_64"
  end

  set_prefix = find_step.call("Set LLVM prefix (Linux tarball)")
  unless set_prefix.is_a?(Hash) && set_prefix["run"].to_s.include?('echo "LLVM_SYS_211_PREFIX=$HOME/llvm" >> "$GITHUB_ENV"')
    errors << "workflow must export LLVM_SYS_211_PREFIX from the Linux tarball location"
  end

  install_rust = find_step.call("Install Rust")
  if install_rust.is_a?(Hash)
    unless install_rust["uses"] == "dtolnay/rust-toolchain@stable"
      errors << "Install Rust step must use dtolnay/rust-toolchain@stable"
    end
    unless install_rust["timeout-minutes"].is_a?(Integer) && install_rust["timeout-minutes"] >= 5
      errors << "Install Rust step must declare timeout-minutes"
    end
    unless install_rust.fetch("with", {})["targets"] == "x86_64-unknown-linux-gnu"
      errors << "Install Rust step must target x86_64-unknown-linux-gnu"
    end
  else
    errors << "workflow must install the Rust toolchain"
  end

  cargo_cache = find_step.call("Cargo cache")
  if cargo_cache.is_a?(Hash)
    unless cargo_cache["uses"] == "Swatinem/rust-cache@v2"
      errors << "Cargo cache step must use Swatinem/rust-cache@v2"
    end
    unless cargo_cache.dig("with", "key") == "extension-release-proof-x86_64-unknown-linux-gnu"
      errors << "Cargo cache key drifted away from the reusable proof contract"
    end
  else
    errors << "workflow must cache Cargo outputs for the proof job"
  end

  proof = find_step.call("Run extension release proof")
  if proof.is_a?(Hash)
    unless proof["id"] == "proof"
      errors << "proof step id must stay 'proof'"
    end
    unless proof["shell"] == "bash"
      errors << "proof step must run under bash"
    end
    unless proof["timeout-minutes"].is_a?(Integer) && proof["timeout-minutes"] >= 10
      errors << "proof step must declare timeout-minutes"
    end
    proof_run = proof["run"].to_s
    unless proof_run.include?('bash scripts/verify-m034-s04-extension.sh')
      errors << "proof step must shell out to bash scripts/verify-m034-s04-extension.sh"
    end
    unless proof_run.include?('export EXPECTED_TAG="$GITHUB_REF_NAME"')
      errors << "proof step must forward tag refs into EXPECTED_TAG"
    end
  else
    errors << "workflow must contain the extension proof step"
  end

  capture = find_step.call("Capture verified VSIX metadata")
  if capture.is_a?(Hash)
    unless capture["id"] == "capture"
      errors << "capture step id must stay 'capture'"
    end
    capture_run = capture["run"].to_s
    [
      '.tmp/m034-s04/verify/verified-vsix-path.txt',
      'artifact_name="extension-release-vsix"',
      'echo "verified_vsix_path=$verified_vsix_path" >> "$GITHUB_OUTPUT"',
      'echo "verified_vsix_artifact_name=$artifact_name" >> "$GITHUB_OUTPUT"',
    ].each do |needle|
      errors << "capture step missing #{needle}" unless capture_run.include?(needle)
    end
  else
    errors << "workflow must capture the verified VSIX metadata"
  end

  upload_vsix = find_step.call("Upload verified VSIX")
  if upload_vsix.is_a?(Hash)
    unless upload_vsix["uses"] == "actions/upload-artifact@v4"
      errors << "verified VSIX upload must use actions/upload-artifact@v4"
    end
    upload_with = upload_vsix["with"]
    unless upload_with.is_a?(Hash) && upload_with["name"] == "${{ steps.capture.outputs.verified_vsix_artifact_name }}"
      errors << "verified VSIX upload must use the captured artifact name output"
    end
    unless upload_with.is_a?(Hash) && upload_with["path"] == "${{ steps.capture.outputs.verified_vsix_path }}"
      errors << "verified VSIX upload must use the captured VSIX path output"
    end
    unless upload_with.is_a?(Hash) && upload_with["if-no-files-found"] == "error"
      errors << "verified VSIX upload must fail when the artifact is missing"
    end
  else
    errors << "workflow must upload the verified VSIX artifact"
  end

  diagnostics = find_step.call("Upload extension proof diagnostics")
  if diagnostics.is_a?(Hash)
    unless diagnostics["uses"] == "actions/upload-artifact@v4"
      errors << "diagnostic upload must use actions/upload-artifact@v4"
    end
    unless diagnostics["if"].to_s.include?("failure()")
      errors << "diagnostic upload must run on failure"
    end
    unless diagnostics["timeout-minutes"].is_a?(Integer) && diagnostics["timeout-minutes"] >= 1
      errors << "diagnostic upload must declare timeout-minutes"
    end
    diagnostics_with = diagnostics["with"]
    unless diagnostics_with.is_a?(Hash) && diagnostics_with["name"] == "extension-release-proof-diagnostics"
      errors << "diagnostic upload artifact name drifted"
    end
    unless diagnostics_with.is_a?(Hash) && diagnostics_with["path"] == ".tmp/m034-s04/verify/**"
      errors << "diagnostic upload must retain .tmp/m034-s04/verify/**"
    end
    unless diagnostics_with.is_a?(Hash) && diagnostics_with["if-no-files-found"] == "error"
      errors << "diagnostic upload must fail when proof artifacts are missing"
    end
  else
    errors << "workflow must upload failure diagnostics"
  end
end

workflow_glob = File.join(root_dir, ".github/workflows/*.yml")
direct_proof_workflows = Dir.glob(workflow_glob).select do |path|
  File.read(path).include?("bash scripts/verify-m034-s04-extension.sh")
end.map { |path| File.expand_path(path) }
expected_direct_workflow = File.expand_path(workflow_path)
unless direct_proof_workflows == [expected_direct_workflow]
  errors << "the reusable proof workflow must be the only workflow file that directly runs bash scripts/verify-m034-s04-extension.sh"
end

if raw.scan("bash scripts/verify-m034-s04-extension.sh").length != 1
  errors << "reusable workflow must invoke bash scripts/verify-m034-s04-extension.sh exactly once"
end

[
  "npm ci",
  "npm run compile",
  "npm run package",
  "cargo test -q -p meshc --test e2e_lsp",
  "npx vsce package",
  "ls *.vsix",
  "continue-on-error",
].each do |forbidden|
  if raw.include?(forbidden)
    errors << "reusable workflow must stay thin and not inline proof logic (found #{forbidden.inspect})"
  end
end

if errors.empty?
  puts "reusable workflow contract ok"
else
  raise errors.join("\n")
end
RUBY
  then
    fail_with_log "$phase_name" "$command_text" "reusable workflow contract drifted" "$log_path"
  fi

  record_phase "$phase_name" "passed"
}

run_publish_contract_check() {
  local phase_name="publish"
  local command_text="ruby publish workflow contract sweep ${PUBLISH_WORKFLOW_PATH}"
  local log_path="$ARTIFACT_DIR/publish.log"

  record_phase "$phase_name" "started"
  echo "==> [${phase_name}] ${command_text}"
  if ! ruby - "$PUBLISH_WORKFLOW_PATH" >"$log_path" 2>&1 <<'RUBY'
require "yaml"

workflow_path = ARGV.fetch(0)
workflow = YAML.load_file(workflow_path)
raw = File.read(workflow_path)

errors = []

errors << "publish workflow file is missing" unless File.file?(workflow_path)
errors << "workflow name must stay 'Publish Extension'" unless workflow["name"] == "Publish Extension"

on_key = if workflow.key?("on")
  "on"
elsif workflow.key?(true)
  true
else
  "on"
end
on_block = workflow[on_key]
unless on_block.is_a?(Hash) && on_block.keys == ["push"]
  errors << "publish workflow must trigger only on push"
end
push_block = on_block.is_a?(Hash) ? on_block["push"] : nil
unless push_block.is_a?(Hash) && push_block["tags"] == ["ext-v*"]
  errors << "publish workflow push trigger must stay limited to ext-v* tags"
end

permissions = workflow["permissions"]
unless permissions.is_a?(Hash) && permissions == { "contents" => "read" }
  errors << "publish workflow permissions must stay read-only"
end

jobs = workflow["jobs"]
unless jobs.is_a?(Hash) && jobs.keys == ["proof", "publish"]
  errors << "publish workflow must define exactly proof and publish jobs"
end

proof = jobs.is_a?(Hash) ? jobs["proof"] : nil
if proof.is_a?(Hash)
  errors << "proof job name must stay 'Verify extension release proof'" unless proof["name"] == "Verify extension release proof"
  unless proof["uses"] == "./.github/workflows/extension-release-proof.yml"
    errors << "proof job must invoke the reusable workflow at ./.github/workflows/extension-release-proof.yml"
  end
else
  errors << "publish workflow must define the proof job"
end

publish = jobs.is_a?(Hash) ? jobs["publish"] : nil
if publish.is_a?(Hash)
  errors << "publish job name must stay 'Publish verified extension'" unless publish["name"] == "Publish verified extension"
  unless publish["needs"] == ["proof"]
    errors << "publish job must depend on the proof job"
  end
  errors << "publish job must stay on ubuntu-latest" unless publish["runs-on"] == "ubuntu-latest"
  unless publish["timeout-minutes"].is_a?(Integer) && publish["timeout-minutes"] >= 10
    errors << "publish job must declare timeout-minutes"
  end

  steps = publish["steps"]
  unless steps.is_a?(Array)
    errors << "publish job must define steps"
    steps = []
  end

  find_step = lambda do |name|
    steps.find { |step| step.is_a?(Hash) && step["name"] == name }
  end

  download = find_step.call("Download verified VSIX")
  if download.is_a?(Hash)
    unless download["uses"] == "actions/download-artifact@v4"
      errors << "Download verified VSIX step must use actions/download-artifact@v4"
    end
    download_with = download["with"]
    unless download_with.is_a?(Hash) && download_with["name"] == "${{ needs.proof.outputs.verified_vsix_artifact_name }}"
      errors << "Download verified VSIX step must use the proof artifact-name output"
    end
    unless download_with.is_a?(Hash) && download_with["path"] == "tools/editors/vscode-mesh/dist/"
      errors << "Download verified VSIX step must restore the artifact into tools/editors/vscode-mesh/dist/"
    end
  else
    errors << "publish job must download the verified VSIX artifact"
  end

  handoff = find_step.call("Confirm verified VSIX handoff")
  if handoff.is_a?(Hash)
    handoff_env = handoff["env"]
    unless handoff_env.is_a?(Hash) && handoff_env["VERIFIED_VSIX_PATH"] == "${{ needs.proof.outputs.verified_vsix_path }}"
      errors << "handoff check must read VERIFIED_VSIX_PATH from the proof output"
    end
    handoff_run = handoff["run"].to_s
    [
      'test -n "$VERIFIED_VSIX_PATH"',
      'test -f "$VERIFIED_VSIX_PATH"',
      'echo "Publishing verified VSIX: $VERIFIED_VSIX_PATH"',
    ].each do |needle|
      errors << "handoff check missing #{needle}" unless handoff_run.include?(needle)
    end
  else
    errors << "publish job must verify the exact VSIX handoff before publication"
  end

  open_vsx = find_step.call("Publish to Open VSX Registry")
  if open_vsx.is_a?(Hash)
    unless open_vsx["uses"] == "HaaLeo/publish-vscode-extension@v2"
      errors << "Open VSX publish step must use HaaLeo/publish-vscode-extension@v2"
    end
    open_vsx_with = open_vsx["with"]
    unless open_vsx_with.is_a?(Hash) && open_vsx_with["pat"] == "${{ secrets.OPEN_VSX_TOKEN }}"
      errors << "Open VSX publish step must use secrets.OPEN_VSX_TOKEN"
    end
    unless open_vsx_with.is_a?(Hash) && open_vsx_with["extensionFile"] == "${{ needs.proof.outputs.verified_vsix_path }}"
      errors << "Open VSX publish step must publish the proof job's exact VSIX path"
    end
    unless open_vsx_with.is_a?(Hash) && open_vsx_with["skipDuplicate"] == true
      errors << "Open VSX publish step must enable skipDuplicate for reroll-safe reruns"
    end
  else
    errors << "publish workflow must publish to Open VSX"
  end

  marketplace = find_step.call("Publish to Visual Studio Marketplace")
  if marketplace.is_a?(Hash)
    unless marketplace["uses"] == "HaaLeo/publish-vscode-extension@v2"
      errors << "Marketplace publish step must use HaaLeo/publish-vscode-extension@v2"
    end
    marketplace_with = marketplace["with"]
    unless marketplace_with.is_a?(Hash) && marketplace_with["pat"] == "${{ secrets.VS_MARKETPLACE_TOKEN }}"
      errors << "Marketplace publish step must use secrets.VS_MARKETPLACE_TOKEN"
    end
    unless marketplace_with.is_a?(Hash) && marketplace_with["registryUrl"] == "https://marketplace.visualstudio.com"
      errors << "Marketplace publish step must target the Visual Studio Marketplace registry URL"
    end
    unless marketplace_with.is_a?(Hash) && marketplace_with["extensionFile"] == "${{ needs.proof.outputs.verified_vsix_path }}"
      errors << "Marketplace publish step must publish the proof job's exact VSIX path"
    end
    unless marketplace_with.is_a?(Hash) && marketplace_with["skipDuplicate"] == true
      errors << "Marketplace publish step must enable skipDuplicate for reroll-safe reruns"
    end
  else
    errors << "publish workflow must publish to the Visual Studio Marketplace"
  end
end

unless raw.include?("# Trigger: git tag ext-vX.Y.Z && git push origin ext-vX.Y.Z")
  errors << "publish workflow must keep the generic ext-vX.Y.Z trigger example comment"
end
unless raw.include?("# Requires secrets: VS_MARKETPLACE_TOKEN, OPEN_VSX_TOKEN")
  errors << "publish workflow must list the required registry secrets"
end

if raw.include?("continue-on-error")
  errors << "publish workflow must not use continue-on-error"
end

[
  "ls *.vsix",
  "bash scripts/verify-m034-s04-extension.sh",
  "npm ci",
  "npm run compile",
  "npm run package",
  "npx vsce package",
].each do |forbidden|
  if raw.include?(forbidden)
    errors << "publish workflow must stay thin and avoid inline proof logic (found #{forbidden.inspect})"
  end
end

if raw.scan("./.github/workflows/extension-release-proof.yml").length != 1
  errors << "publish workflow must reference the reusable proof workflow exactly once"
end

if raw.scan("needs.proof.outputs.verified_vsix_path").length != 3
  errors << "publish workflow must use needs.proof.outputs.verified_vsix_path for the handoff check and both publish actions"
end

if raw.scan("needs.proof.outputs.verified_vsix_artifact_name").length != 1
  errors << "publish workflow must use needs.proof.outputs.verified_vsix_artifact_name exactly once for artifact download"
end

if raw.scan("skipDuplicate: true").length != 2
  errors << "publish workflow must enable skipDuplicate on both publish steps"
end

if errors.empty?
  puts "publish workflow contract ok"
else
  raise errors.join("\n")
end
RUBY
  then
    fail_with_log "$phase_name" "$command_text" "publish workflow contract drifted" "$log_path"
  fi

  record_phase "$phase_name" "passed"
}

run_full_contract_check() {
  local phase_name="full-contract"
  local command_text="full extension workflow contract sweep"
  local log_path="$ARTIFACT_DIR/full-contract.log"

  record_phase "$phase_name" "started"
  echo "==> [${phase_name}] ${command_text}"
  if ! (
    run_reusable_contract_check
    run_publish_contract_check
  ) >"$log_path" 2>&1; then
    fail_with_log "$phase_name" "$command_text" "workflow contract drifted" "$log_path"
  fi

  record_phase "$phase_name" "passed"
}

mode="${1:-all}"
case "$mode" in
  reusable)
    run_reusable_contract_check
    ;;
  publish)
    run_publish_contract_check
    ;;
  all)
    run_full_contract_check
    ;;
  *)
    echo "unknown mode: $mode" >&2
    echo "usage: bash scripts/verify-m034-s04-workflows.sh [reusable|publish|all]" >&2
    exit 1
    ;;
esac

echo "verify-m034-s04-workflows: ok (${mode})"
