$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$RootDir = (Resolve-Path (Join-Path $PSScriptRoot '..' '..')).Path
$env:M034_S03_LIB_ONLY = '1'
. (Join-Path $RootDir 'scripts/verify-m034-s03.ps1')
Remove-Item Env:M034_S03_LIB_ONLY -ErrorAction SilentlyContinue

$TestRoot = Join-Path $RootDir '.tmp/m034-s03/last-exitcode-test'
Remove-Item -Recurse -Force $TestRoot -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Path $TestRoot -Force | Out-Null

$script:RunDir = $TestRoot
$script:LastStdoutPath = ''
$script:LastStderrPath = ''
$script:LastLogPath = ''

if (Get-Variable -Name LASTEXITCODE -Scope Global -ErrorAction SilentlyContinue) {
    Remove-Variable -Name LASTEXITCODE -Scope Global -ErrorAction SilentlyContinue
}

Invoke-LoggedCommand -Phase 'unit' -Label 'unset-last-exitcode' -Display 'Write-Output ok' -Command {
    Write-Output 'ok'
}

$stdoutPath = Join-Path $TestRoot 'unset-last-exitcode.stdout'
$stderrPath = Join-Path $TestRoot 'unset-last-exitcode.stderr'
$logPath = Join-Path $TestRoot 'unset-last-exitcode.log'

if (-not (Test-Path $stdoutPath)) {
    throw 'stdout artifact was not written'
}
if (-not (Test-Path $stderrPath)) {
    throw 'stderr artifact was not written'
}
if (-not (Test-Path $logPath)) {
    throw 'combined log artifact was not written'
}
$stdoutContent = Get-Content $stdoutPath -Raw
$stderrContent = Get-Content $stderrPath -Raw
if ($null -eq $stdoutContent) { $stdoutContent = '' }
if ($null -eq $stderrContent) { $stderrContent = '' }
if ($stdoutContent.Trim() -ne 'ok') {
    throw 'stdout artifact did not capture command output'
}
if ($stderrContent.Trim() -ne '') {
    throw 'stderr artifact should stay empty for the unset LASTEXITCODE success case'
}
$logContent = Get-Content $logPath -Raw
if (-not $logContent.Contains('display: Write-Output ok')) {
    throw 'combined log did not keep the display text'
}
if (-not $logContent.Contains('exit_code: 0')) {
    throw 'combined log did not preserve the successful exit code'
}
if (-not $logContent.Contains("stdout_path: $stdoutPath")) {
    throw 'combined log did not preserve the stdout artifact path'
}
if (-not $logContent.Contains("stderr_path: $stderrPath")) {
    throw 'combined log did not preserve the stderr artifact path'
}
if (-not $logContent.Contains('[stdout]')) {
    throw 'combined log did not preserve stdout section'
}
if (-not $logContent.Contains('ok')) {
    throw 'combined log did not preserve stdout content'
}
if ($script:LastStdoutPath -ne $stdoutPath -or $script:LastStderrPath -ne $stderrPath -or $script:LastLogPath -ne $logPath) {
    throw 'Invoke-LoggedCommand did not update the tracked artifact paths'
}

Write-Host 'verify-m034-s03-last-exitcode: ok'
