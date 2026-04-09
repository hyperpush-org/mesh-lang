use std::env;
use std::fmt;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const UPDATE_INSTALLER_URL_ENV: &str = "MESH_UPDATE_INSTALLER_URL";
const DEFAULT_UNIX_INSTALLER_URL: &str = "https://meshlang.dev/install.sh";
const DEFAULT_WINDOWS_INSTALLER_URL: &str = "https://meshlang.dev/install.ps1";
const DEFAULT_DOWNLOAD_TIMEOUT_SEC: u64 = 120;
const WINDOWS_BOOTSTRAP_SETTLE_MS: u64 = 50;
const FORWARDED_INSTALLER_ENV_KEYS: [&str; 4] = [
    "MESH_INSTALL_RELEASE_API_URL",
    "MESH_INSTALL_RELEASE_BASE_URL",
    "MESH_INSTALL_DOWNLOAD_TIMEOUT_SEC",
    "MESH_INSTALL_STRICT_PROOF",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolchainUpdateMode {
    Completed,
    DetachedBootstrap,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolchainUpdateOutcome {
    pub installer_url: String,
    pub mode: ToolchainUpdateMode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolchainUpdateError {
    phase: &'static str,
    platform: String,
    installer_url: String,
    launcher: Option<String>,
    detail: String,
}

impl ToolchainUpdateError {
    fn new(
        phase: &'static str,
        platform: impl Into<String>,
        installer_url: impl Into<String>,
        launcher: Option<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            phase,
            platform: platform.into(),
            installer_url: installer_url.into(),
            launcher,
            detail: detail.into(),
        }
    }

    pub fn phase(&self) -> &'static str {
        self.phase
    }

    pub fn platform(&self) -> &str {
        &self.platform
    }

    pub fn installer_url(&self) -> &str {
        &self.installer_url
    }

    pub fn launcher(&self) -> Option<&str> {
        self.launcher.as_deref()
    }
}

impl fmt::Display for ToolchainUpdateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.launcher {
            Some(launcher) => write!(
                f,
                "toolchain update {} failed on {} with launcher {} for installer {}: {}",
                self.phase, self.platform, launcher, self.installer_url, self.detail
            ),
            None => write!(
                f,
                "toolchain update {} failed on {} for installer {}: {}",
                self.phase, self.platform, self.installer_url, self.detail
            ),
        }
    }
}

impl std::error::Error for ToolchainUpdateError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ToolchainUpdatePlatform {
    Unix,
    Windows,
    Unsupported(String),
}

impl ToolchainUpdatePlatform {
    pub(crate) fn detect() -> Self {
        if cfg!(windows) {
            Self::Windows
        } else if cfg!(unix) {
            Self::Unix
        } else {
            Self::Unsupported(env::consts::OS.to_string())
        }
    }

    pub(crate) fn label(&self) -> &str {
        match self {
            Self::Unix => "unix",
            Self::Windows => "windows",
            Self::Unsupported(platform) => platform.as_str(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct ToolchainUpdateEnv {
    installer_url_override: Option<String>,
    download_timeout_sec: Option<String>,
    forwarded_env: Vec<(String, String)>,
}

impl ToolchainUpdateEnv {
    pub(crate) fn capture() -> Self {
        Self::from_lookup(|key| env::var(key).ok())
    }

    pub(crate) fn from_lookup<F>(lookup: F) -> Self
    where
        F: Fn(&str) -> Option<String>,
    {
        let installer_url_override = lookup(UPDATE_INSTALLER_URL_ENV);
        let download_timeout_sec = lookup("MESH_INSTALL_DOWNLOAD_TIMEOUT_SEC");
        let forwarded_env = FORWARDED_INSTALLER_ENV_KEYS
            .iter()
            .filter_map(|key| lookup(key).map(|value| ((*key).to_string(), value)))
            .collect();

        Self {
            installer_url_override,
            download_timeout_sec,
            forwarded_env,
        }
    }

    pub(crate) fn installer_url_for(
        &self,
        platform: &ToolchainUpdatePlatform,
    ) -> Result<String, ToolchainUpdateError> {
        if let Some(url) = &self.installer_url_override {
            return Ok(url.clone());
        }

        default_installer_url(platform).map(str::to_owned)
    }

    pub(crate) fn download_timeout(&self) -> Duration {
        let parsed = self
            .download_timeout_sec
            .as_deref()
            .and_then(|raw| raw.parse::<u64>().ok())
            .filter(|value| *value > 0)
            .unwrap_or(DEFAULT_DOWNLOAD_TIMEOUT_SEC);
        Duration::from_secs(parsed)
    }

    pub(crate) fn forwarded_env(&self) -> &[(String, String)] {
        &self.forwarded_env
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LauncherCommand {
    pub program: String,
    pub args: Vec<String>,
}

pub fn run_toolchain_update() -> Result<ToolchainUpdateOutcome, ToolchainUpdateError> {
    let platform = ToolchainUpdatePlatform::detect();
    let update_env = ToolchainUpdateEnv::capture();
    let installer_url = update_env.installer_url_for(&platform)?;
    let installer_text =
        download_installer_script(&installer_url, update_env.download_timeout(), &platform)?;

    match platform {
        ToolchainUpdatePlatform::Unix => {
            let launcher = unix_launcher_command();
            run_unix_installer_with_command(
                &installer_text,
                update_env.forwarded_env(),
                &installer_url,
                &launcher,
            )
        }
        ToolchainUpdatePlatform::Windows => launch_windows_bootstrap(
            &installer_text,
            update_env.forwarded_env(),
            &installer_url,
            std::process::id(),
        ),
        ToolchainUpdatePlatform::Unsupported(platform_name) => Err(ToolchainUpdateError::new(
            "plan-launcher",
            platform_name,
            installer_url,
            None,
            "unsupported host platform",
        )),
    }
}

pub(crate) fn default_installer_url(
    platform: &ToolchainUpdatePlatform,
) -> Result<&'static str, ToolchainUpdateError> {
    match platform {
        ToolchainUpdatePlatform::Unix => Ok(DEFAULT_UNIX_INSTALLER_URL),
        ToolchainUpdatePlatform::Windows => Ok(DEFAULT_WINDOWS_INSTALLER_URL),
        ToolchainUpdatePlatform::Unsupported(platform_name) => Err(ToolchainUpdateError::new(
            "plan-launcher",
            platform_name.clone(),
            "<unsupported-platform>",
            None,
            "unsupported host platform",
        )),
    }
}

pub(crate) fn download_installer_script(
    installer_url: &str,
    timeout: Duration,
    platform: &ToolchainUpdatePlatform,
) -> Result<String, ToolchainUpdateError> {
    let config = ureq::Agent::config_builder()
        .timeout_global(Some(timeout))
        .build();
    let agent = ureq::Agent::new_with_config(config);
    let mut response = agent.get(installer_url).call().map_err(|error| {
        ToolchainUpdateError::new(
            "download",
            platform.label(),
            installer_url,
            None,
            format!(
                "failed to fetch installer with timeout {}s: {}",
                timeout.as_secs(),
                error
            ),
        )
    })?;

    let mut bytes = Vec::new();
    response
        .body_mut()
        .as_reader()
        .read_to_end(&mut bytes)
        .map_err(|error| {
            ToolchainUpdateError::new(
                "download",
                platform.label(),
                installer_url,
                None,
                format!("failed to read installer response body: {}", error),
            )
        })?;

    validate_installer_bytes(&bytes, installer_url, platform)
}

pub(crate) fn validate_installer_bytes(
    bytes: &[u8],
    installer_url: &str,
    platform: &ToolchainUpdatePlatform,
) -> Result<String, ToolchainUpdateError> {
    if bytes.is_empty() {
        return Err(ToolchainUpdateError::new(
            "download",
            platform.label(),
            installer_url,
            None,
            "installer response body was empty",
        ));
    }

    let text = String::from_utf8(bytes.to_vec()).map_err(|_| {
        ToolchainUpdateError::new(
            "download",
            platform.label(),
            installer_url,
            None,
            "installer response body was not valid UTF-8 text",
        )
    })?;

    if text.trim().is_empty() {
        return Err(ToolchainUpdateError::new(
            "download",
            platform.label(),
            installer_url,
            None,
            "installer response body was empty",
        ));
    }

    Ok(text)
}

pub(crate) fn unix_launcher_command() -> LauncherCommand {
    LauncherCommand {
        program: "/bin/sh".to_string(),
        args: vec!["-s".to_string(), "--".to_string(), "--yes".to_string()],
    }
}

pub(crate) fn run_unix_installer_with_command(
    installer_text: &str,
    forwarded_env: &[(String, String)],
    installer_url: &str,
    launcher: &LauncherCommand,
) -> Result<ToolchainUpdateOutcome, ToolchainUpdateError> {
    let mut command = Command::new(&launcher.program);
    command
        .args(&launcher.args)
        .envs(forwarded_env.iter().cloned())
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let mut child = command.spawn().map_err(|error| {
        ToolchainUpdateError::new(
            "spawn-launcher",
            ToolchainUpdatePlatform::Unix.label(),
            installer_url,
            Some(launcher.program.clone()),
            format!("failed to spawn launcher: {}", error),
        )
    })?;

    {
        let mut stdin = child.stdin.take().ok_or_else(|| {
            ToolchainUpdateError::new(
                "spawn-launcher",
                ToolchainUpdatePlatform::Unix.label(),
                installer_url,
                Some(launcher.program.clone()),
                "launcher stdin was not available",
            )
        })?;
        stdin
            .write_all(installer_text.as_bytes())
            .map_err(|error| {
                ToolchainUpdateError::new(
                    "wait-launcher",
                    ToolchainUpdatePlatform::Unix.label(),
                    installer_url,
                    Some(launcher.program.clone()),
                    format!("failed to write installer to launcher stdin: {}", error),
                )
            })?;
    }

    let status = child.wait().map_err(|error| {
        ToolchainUpdateError::new(
            "wait-launcher",
            ToolchainUpdatePlatform::Unix.label(),
            installer_url,
            Some(launcher.program.clone()),
            format!("failed while waiting for installer process: {}", error),
        )
    })?;

    if !status.success() {
        return Err(ToolchainUpdateError::new(
            "wait-launcher",
            ToolchainUpdatePlatform::Unix.label(),
            installer_url,
            Some(launcher.program.clone()),
            format!("installer exited with status {}", status),
        ));
    }

    Ok(ToolchainUpdateOutcome {
        installer_url: installer_url.to_string(),
        mode: ToolchainUpdateMode::Completed,
    })
}

pub(crate) fn windows_launcher_command(
    bootstrap_path: &Path,
) -> Result<LauncherCommand, ToolchainUpdateError> {
    if bootstrap_path.as_os_str().is_empty() {
        return Err(ToolchainUpdateError::new(
            "plan-launcher",
            ToolchainUpdatePlatform::Windows.label(),
            DEFAULT_WINDOWS_INSTALLER_URL,
            Some("powershell.exe".to_string()),
            "bootstrap script path was empty",
        ));
    }

    Ok(LauncherCommand {
        program: "powershell.exe".to_string(),
        args: vec![
            "-NoProfile".to_string(),
            "-ExecutionPolicy".to_string(),
            "Bypass".to_string(),
            "-File".to_string(),
            bootstrap_path.to_string_lossy().into_owned(),
        ],
    })
}

pub(crate) fn build_windows_bootstrap_script(
    installer_path: &Path,
    parent_pid: u32,
    installer_url: &str,
) -> Result<String, ToolchainUpdateError> {
    if installer_path.as_os_str().is_empty() {
        return Err(ToolchainUpdateError::new(
            "plan-launcher",
            ToolchainUpdatePlatform::Windows.label(),
            installer_url,
            Some("powershell.exe".to_string()),
            "installer script path was empty",
        ));
    }

    let installer_literal = powershell_single_quote(installer_path);
    Ok(format!(
        "$ErrorActionPreference = 'Stop'\n\
$ParentPid = {parent_pid}\n\
$InstallerPath = '{installer_literal}'\n\
if (-not (Test-Path $InstallerPath)) {{\n\
    Write-Error \"installer script missing at $InstallerPath\"\n\
    exit 1\n\
}}\n\
$Attempts = 0\n\
while ((Get-Process -Id $ParentPid -ErrorAction SilentlyContinue) -and ($Attempts -lt 100)) {{\n\
    Start-Sleep -Milliseconds 200\n\
    $Attempts += 1\n\
}}\n\
try {{\n\
    & $InstallerPath -Yes\n\
    $ExitCode = $LASTEXITCODE\n\
}} catch {{\n\
    Write-Error $_\n\
    exit 1\n\
}}\n\
if ($null -eq $ExitCode) {{\n\
    $ExitCode = 0\n\
}}\n\
exit $ExitCode\n"
    ))
}

pub(crate) fn write_script_file(
    path: &Path,
    contents: &str,
    installer_url: &str,
    platform: &ToolchainUpdatePlatform,
) -> Result<(), ToolchainUpdateError> {
    fs::write(path, contents).map_err(|error| {
        ToolchainUpdateError::new(
            "write-installer",
            platform.label(),
            installer_url,
            Some(path.display().to_string()),
            format!("failed to write script file {}: {}", path.display(), error),
        )
    })
}

pub(crate) fn spawn_windows_bootstrap_command(
    launcher: &LauncherCommand,
    forwarded_env: &[(String, String)],
    installer_url: &str,
) -> Result<ToolchainUpdateOutcome, ToolchainUpdateError> {
    let mut command = Command::new(&launcher.program);
    command
        .args(&launcher.args)
        .envs(forwarded_env.iter().cloned())
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let mut child = command.spawn().map_err(|error| {
        ToolchainUpdateError::new(
            "spawn-launcher",
            ToolchainUpdatePlatform::Windows.label(),
            installer_url,
            Some(launcher.program.clone()),
            format!("failed to spawn launcher: {}", error),
        )
    })?;

    thread::sleep(Duration::from_millis(WINDOWS_BOOTSTRAP_SETTLE_MS));
    match child.try_wait() {
        Ok(Some(status)) if !status.success() => Err(ToolchainUpdateError::new(
            "bootstrap",
            ToolchainUpdatePlatform::Windows.label(),
            installer_url,
            Some(launcher.program.clone()),
            format!("bootstrap exited early with status {}", status),
        )),
        Ok(_) => Ok(ToolchainUpdateOutcome {
            installer_url: installer_url.to_string(),
            mode: ToolchainUpdateMode::DetachedBootstrap,
        }),
        Err(error) => Err(ToolchainUpdateError::new(
            "bootstrap",
            ToolchainUpdatePlatform::Windows.label(),
            installer_url,
            Some(launcher.program.clone()),
            format!("failed to inspect bootstrap status: {}", error),
        )),
    }
}

pub(crate) fn launch_windows_bootstrap(
    installer_text: &str,
    forwarded_env: &[(String, String)],
    installer_url: &str,
    parent_pid: u32,
) -> Result<ToolchainUpdateOutcome, ToolchainUpdateError> {
    let temp_dir = create_temp_script_dir(installer_url)?;
    let installer_path = temp_dir.join("install.ps1");
    write_script_file(
        &installer_path,
        installer_text,
        installer_url,
        &ToolchainUpdatePlatform::Windows,
    )?;

    let bootstrap_text =
        build_windows_bootstrap_script(&installer_path, parent_pid, installer_url)?;
    let bootstrap_path = temp_dir.join("mesh-update-bootstrap.ps1");
    write_script_file(
        &bootstrap_path,
        &bootstrap_text,
        installer_url,
        &ToolchainUpdatePlatform::Windows,
    )?;

    let launcher = windows_launcher_command(&bootstrap_path)?;
    spawn_windows_bootstrap_command(&launcher, forwarded_env, installer_url)
}

fn create_temp_script_dir(installer_url: &str) -> Result<PathBuf, ToolchainUpdateError> {
    let mut temp_dir = env::temp_dir();
    temp_dir.push(unique_temp_dir_name());
    fs::create_dir_all(&temp_dir).map_err(|error| {
        ToolchainUpdateError::new(
            "write-installer",
            ToolchainUpdatePlatform::Windows.label(),
            installer_url,
            Some(temp_dir.display().to_string()),
            format!(
                "failed to create temp script directory {}: {}",
                temp_dir.display(),
                error
            ),
        )
    })?;
    Ok(temp_dir)
}

fn unique_temp_dir_name() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    format!("mesh-toolchain-update-{}-{}", std::process::id(), nanos)
}

fn powershell_single_quote(path: &Path) -> String {
    path.to_string_lossy().replace('\'', "''")
}
