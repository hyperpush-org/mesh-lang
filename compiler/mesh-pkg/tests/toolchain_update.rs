use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::Path;
use std::thread;
use std::time::Duration;

use tempfile::TempDir;

#[allow(dead_code)]
#[path = "../src/toolchain_update.rs"]
mod toolchain_update_impl;

use toolchain_update_impl::{
    build_windows_bootstrap_script, default_installer_url, download_installer_script,
    run_unix_installer_with_command, spawn_windows_bootstrap_command, unix_launcher_command,
    validate_installer_bytes, windows_launcher_command, write_script_file, LauncherCommand,
    ToolchainUpdateEnv, ToolchainUpdatePlatform,
};

fn env_from_pairs(pairs: &[(&str, &str)]) -> ToolchainUpdateEnv {
    let values: HashMap<String, String> = pairs
        .iter()
        .map(|(key, value)| ((*key).to_string(), (*value).to_string()))
        .collect();
    ToolchainUpdateEnv::from_lookup(|key| values.get(key).cloned())
}

fn serve_once(body: &[u8]) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("listener should bind");
    let addr = listener
        .local_addr()
        .expect("listener should have an address");
    let body = body.to_vec();

    thread::spawn(move || {
        let (mut stream, _) = listener
            .accept()
            .expect("server should accept one connection");
        let mut request = [0_u8; 1024];
        let _ = stream.read(&mut request);
        let headers = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\n",
            body.len()
        );
        stream
            .write_all(headers.as_bytes())
            .expect("headers should write");
        stream.write_all(&body).expect("body should write");
    });

    format!("http://{addr}/install.sh")
}

mod default_installer_url_tests {
    use super::*;

    #[test]
    fn selects_public_unix_installer_url() {
        let url = default_installer_url(&ToolchainUpdatePlatform::Unix)
            .expect("unix installer URL should resolve");
        assert_eq!(url, "https://meshlang.dev/install.sh");
    }

    #[test]
    fn selects_public_windows_installer_url() {
        let url = default_installer_url(&ToolchainUpdatePlatform::Windows)
            .expect("windows installer URL should resolve");
        assert_eq!(url, "https://meshlang.dev/install.ps1");
    }

    #[test]
    fn rejects_unsupported_platforms() {
        let error =
            default_installer_url(&ToolchainUpdatePlatform::Unsupported("plan9".to_string()))
                .expect_err("unsupported platforms should fail closed");
        assert_eq!(error.phase(), "plan-launcher");
        assert_eq!(error.platform(), "plan9");
        assert!(
            error.to_string().contains("unsupported host platform"),
            "unexpected error: {error}"
        );
    }
}

mod environment_tests {
    use super::*;

    #[test]
    fn installer_url_override_wins_for_both_platforms() {
        let env = env_from_pairs(&[(
            "MESH_UPDATE_INSTALLER_URL",
            "http://127.0.0.1:9000/custom.ps1",
        )]);

        let unix_url = env
            .installer_url_for(&ToolchainUpdatePlatform::Unix)
            .expect("unix override should resolve");
        let windows_url = env
            .installer_url_for(&ToolchainUpdatePlatform::Windows)
            .expect("windows override should resolve");

        assert_eq!(unix_url, "http://127.0.0.1:9000/custom.ps1");
        assert_eq!(windows_url, "http://127.0.0.1:9000/custom.ps1");
    }

    #[test]
    fn forwards_only_supported_mesh_install_overrides_in_fixed_order() {
        let env = env_from_pairs(&[
            (
                "MESH_INSTALL_RELEASE_API_URL",
                "http://127.0.0.1:9000/api/releases/latest.json",
            ),
            (
                "MESH_INSTALL_RELEASE_BASE_URL",
                "http://127.0.0.1:9000/download",
            ),
            ("MESH_INSTALL_DOWNLOAD_TIMEOUT_SEC", "20"),
            ("MESH_INSTALL_STRICT_PROOF", "1"),
            ("UNRELATED_ENV", "ignored"),
        ]);

        assert_eq!(
            env.forwarded_env(),
            &[
                (
                    "MESH_INSTALL_RELEASE_API_URL".to_string(),
                    "http://127.0.0.1:9000/api/releases/latest.json".to_string(),
                ),
                (
                    "MESH_INSTALL_RELEASE_BASE_URL".to_string(),
                    "http://127.0.0.1:9000/download".to_string(),
                ),
                (
                    "MESH_INSTALL_DOWNLOAD_TIMEOUT_SEC".to_string(),
                    "20".to_string(),
                ),
                ("MESH_INSTALL_STRICT_PROOF".to_string(), "1".to_string()),
            ]
        );
        assert_eq!(env.download_timeout(), Duration::from_secs(20));
    }
}

mod launcher_tests {
    use super::*;

    #[test]
    fn unix_launcher_feeds_stdin_to_bin_sh_with_yes_flag() {
        let launcher = unix_launcher_command();
        assert_eq!(launcher.program, "/bin/sh");
        assert_eq!(launcher.args, ["-s", "--", "--yes"]);
    }

    #[test]
    fn windows_launcher_uses_explicit_powershell_bootstrap_file() {
        let launcher = windows_launcher_command(Path::new(r"C:\Temp\mesh-update-bootstrap.ps1"))
            .expect("windows launcher should build");
        assert_eq!(launcher.program, "powershell.exe");
        assert_eq!(
            launcher.args,
            [
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-File",
                r"C:\Temp\mesh-update-bootstrap.ps1",
            ]
        );
    }

    #[test]
    fn windows_launcher_rejects_empty_bootstrap_path() {
        let error = windows_launcher_command(Path::new(""))
            .expect_err("empty bootstrap paths should fail closed");
        assert_eq!(error.phase(), "plan-launcher");
        assert!(error
            .to_string()
            .contains("bootstrap script path was empty"));
    }

    #[test]
    fn windows_bootstrap_script_waits_for_parent_and_runs_yes_installer() {
        let script = build_windows_bootstrap_script(
            Path::new(r"C:\Temp\install.ps1"),
            4242,
            "https://meshlang.dev/install.ps1",
        )
        .expect("bootstrap script should render");

        assert!(script.contains("$ParentPid = 4242"), "script was: {script}");
        assert!(
            script.contains("while ((Get-Process -Id $ParentPid -ErrorAction SilentlyContinue) -and ($Attempts -lt 100))"),
            "script was: {script}"
        );
        assert!(
            script.contains("& $InstallerPath -Yes"),
            "script was: {script}"
        );
    }
}

mod negative_tests {
    use super::*;

    #[test]
    fn download_reports_bad_url_in_download_phase() {
        let error = download_installer_script(
            "not a url",
            Duration::from_secs(1),
            &ToolchainUpdatePlatform::Unix,
        )
        .expect_err("malformed URLs should fail before execution");

        assert_eq!(error.phase(), "download");
        assert_eq!(error.installer_url(), "not a url");
        assert!(
            error.to_string().contains("not a url"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn download_rejects_empty_installer_body() {
        let url = serve_once(b"");
        let error =
            download_installer_script(&url, Duration::from_secs(5), &ToolchainUpdatePlatform::Unix)
                .expect_err("empty bodies should fail validation");

        assert_eq!(error.phase(), "download");
        assert_eq!(error.installer_url(), url.as_str());
        assert!(
            error.to_string().contains("empty"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn validation_rejects_non_utf8_installer_bytes() {
        let error = validate_installer_bytes(
            &[0xff, 0xfe, 0xfd],
            "http://127.0.0.1:9000/install.sh",
            &ToolchainUpdatePlatform::Unix,
        )
        .expect_err("non-text installer payloads should fail closed");

        assert_eq!(error.phase(), "download");
        assert!(
            error.to_string().contains("valid UTF-8"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn write_script_file_reports_write_installer_failures() {
        let temp = TempDir::new().expect("tempdir should create");
        let error = write_script_file(
            temp.path(),
            "echo hello",
            "https://meshlang.dev/install.ps1",
            &ToolchainUpdatePlatform::Windows,
        )
        .expect_err("writing to a directory should fail");

        assert_eq!(error.phase(), "write-installer");
        assert!(error
            .to_string()
            .contains(temp.path().to_string_lossy().as_ref()));
    }

    #[test]
    fn unix_spawn_failure_reports_spawn_launcher_phase() {
        let launcher = LauncherCommand {
            program: "__missing_mesh_unix_launcher__".to_string(),
            args: vec!["-s".to_string()],
        };
        let error = run_unix_installer_with_command(
            "#!/bin/sh\nexit 0\n",
            &[],
            "https://meshlang.dev/install.sh",
            &launcher,
        )
        .expect_err("missing launchers should fail closed");

        assert_eq!(error.phase(), "spawn-launcher");
        assert_eq!(error.launcher(), Some("__missing_mesh_unix_launcher__"));
    }

    #[test]
    fn windows_spawn_failure_reports_spawn_launcher_phase() {
        let launcher = LauncherCommand {
            program: "__missing_mesh_windows_launcher__".to_string(),
            args: vec!["-NoProfile".to_string()],
        };
        let error =
            spawn_windows_bootstrap_command(&launcher, &[], "https://meshlang.dev/install.ps1")
                .expect_err("missing powershell launchers should fail closed");

        assert_eq!(error.phase(), "spawn-launcher");
        assert_eq!(error.launcher(), Some("__missing_mesh_windows_launcher__"));
    }
}
