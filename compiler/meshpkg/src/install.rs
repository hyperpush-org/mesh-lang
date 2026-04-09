use std::io::Read as IoRead;
use std::path::Path;

use colored::Colorize;
use flate2::read::GzDecoder;
use sha2::{Digest, Sha256};
use tar::Archive;

use mesh_pkg::{LockedPackage, Lockfile, Manifest};

const LOCKFILE_NAME: &str = "mesh.lock";

fn dependency_declaration_snippet(name: &str, version: &str) -> String {
    format!("\"{}\" = \"{}\"", name, version)
}

fn named_install_follow_up(name: &str, version: &str) -> String {
    format!(
        "Named install does not edit mesh.toml; add {} if you want to declare this dependency.",
        dependency_declaration_snippet(name, version)
    )
}

fn named_install_result_json(name: &str, version: &str) -> String {
    serde_json::json!({
        "status": "ok",
        "name": name,
        "version": version,
        "lockfile": LOCKFILE_NAME,
        "manifest_changed": false,
    })
    .to_string()
}

pub fn run(
    project_dir: &Path,
    package_name: Option<&str>,
    registry: &str,
    json_mode: bool,
) -> Result<(), String> {
    match package_name {
        Some(name) => install_named(project_dir, name, registry, json_mode),
        None => install_all(project_dir, registry, json_mode),
    }
}

/// Install all registry dependencies declared in mesh.toml.
/// Uses mesh.lock exact pins if it exists; otherwise resolves from registry.
fn install_all(project_dir: &Path, registry: &str, json_mode: bool) -> Result<(), String> {
    let manifest_path = project_dir.join("mesh.toml");
    let manifest = Manifest::from_file(&manifest_path)?;

    let lock_path = project_dir.join(LOCKFILE_NAME);
    let existing_lock = if lock_path.exists() {
        Some(Lockfile::read(&lock_path)?)
    } else {
        None
    };

    let mut locked_packages: Vec<LockedPackage> = Vec::new();

    for (name, dep) in &manifest.dependencies {
        let version = match dep.registry_version() {
            Some(v) => v,
            None => continue, // skip git/path deps (handled by meshc deps)
        };

        // Check if already in lockfile
        let (resolved_version, sha256_opt) = if let Some(ref lock) = existing_lock {
            if let Some(entry) = lock.packages.iter().find(|p| p.name == *name) {
                (entry.version.clone(), entry.sha256.clone())
            } else {
                // Not in lock — resolve from registry
                let (v, s) = resolve_version(name, version, registry)?;
                (v, Some(s))
            }
        } else {
            // No lockfile — resolve from registry
            let (v, s) = resolve_version(name, version, registry)?;
            (v, Some(s))
        };

        let msg = format!("Downloading {}@{}...", name, resolved_version);
        let (tarball_bytes, actual_sha256) = crate::publish::with_spinner(&msg, json_mode, || {
            download_tarball(name, &resolved_version, registry)
        })?;

        // Verify SHA-256 if we have a lockfile entry
        if let Some(expected) = &sha256_opt {
            if *expected != actual_sha256 {
                return Err(format!(
                    "SHA-256 mismatch for {}@{}: expected {}, got {}",
                    name, resolved_version, expected, actual_sha256
                ));
            }
        }

        // Extract to .mesh/packages/<name>@<version>/
        let install_dir = project_dir
            .join(".mesh")
            .join("packages")
            .join(format!("{}@{}", name, resolved_version));
        std::fs::create_dir_all(&install_dir)
            .map_err(|e| format!("Failed to create {}: {}", install_dir.display(), e))?;

        extract_tarball(&tarball_bytes, &install_dir)?;

        let source_url = format!(
            "{}/api/v1/packages/{}/{}/download",
            registry, name, resolved_version
        );
        locked_packages.push(LockedPackage {
            name: name.clone(),
            version: resolved_version.clone(),
            source: source_url,
            revision: resolved_version.clone(),
            sha256: Some(actual_sha256.clone()),
        });

        if !json_mode {
            println!(
                "{} Installed {}@{}",
                "✓".green().bold(),
                name,
                resolved_version
            );
        }
    }

    // Write lockfile
    if !locked_packages.is_empty() || existing_lock.is_none() {
        // Merge with existing non-registry entries if lockfile existed
        let mut all_packages = locked_packages;
        if let Some(ref lock) = existing_lock {
            for pkg in &lock.packages {
                // Keep existing non-registry (git/path) entries
                if pkg.sha256.is_none() && !all_packages.iter().any(|p| p.name == pkg.name) {
                    all_packages.push(pkg.clone());
                }
            }
        }
        let lockfile = Lockfile::new(all_packages);
        lockfile.write(&lock_path)?;

        if json_mode {
            println!(
                "{}",
                serde_json::json!({"status": "ok", "lockfile": LOCKFILE_NAME})
            );
        } else {
            println!("{} Updated {}", "✓".green().bold(), LOCKFILE_NAME);
        }
    }

    Ok(())
}

/// Fetch and lock the latest release of a single named package.
/// Named install does not edit mesh.toml; it only downloads the package and updates mesh.lock.
fn install_named(
    project_dir: &Path,
    name: &str,
    registry: &str,
    json_mode: bool,
) -> Result<(), String> {
    // Resolve latest version from registry
    let (version, sha256) = resolve_latest(name, registry)?;

    let msg = format!("Downloading {}@{}...", name, version);
    let (tarball_bytes, actual_sha256) = crate::publish::with_spinner(&msg, json_mode, || {
        download_tarball(name, &version, registry)
    })?;

    // Verify SHA-256
    if sha256 != actual_sha256 {
        return Err(format!(
            "SHA-256 mismatch for {}@{}: expected {}, got {}",
            name, version, sha256, actual_sha256
        ));
    }

    // Extract to .mesh/packages/<name>@<version>/
    let install_dir = project_dir
        .join(".mesh")
        .join("packages")
        .join(format!("{}@{}", name, version));
    std::fs::create_dir_all(&install_dir)
        .map_err(|e| format!("Failed to create install dir: {}", e))?;
    extract_tarball(&tarball_bytes, &install_dir)?;

    // Update mesh.lock
    let lock_path = project_dir.join(LOCKFILE_NAME);
    let mut packages = if lock_path.exists() {
        Lockfile::read(&lock_path)?.packages
    } else {
        Vec::new()
    };

    // Replace or add the entry
    packages.retain(|p| p.name != name);
    packages.push(LockedPackage {
        name: name.to_string(),
        version: version.clone(),
        source: format!("{}/api/v1/packages/{}/{}/download", registry, name, version),
        revision: version.clone(),
        sha256: Some(actual_sha256),
    });
    Lockfile::new(packages).write(&lock_path)?;

    if json_mode {
        println!("{}", named_install_result_json(name, &version));
    } else {
        println!("{} Installed {}@{}", "✓".green().bold(), name, version);
        println!("  Updated {}", LOCKFILE_NAME);
        println!("  {}", named_install_follow_up(name, &version));
    }

    Ok(())
}

/// Download a package tarball from the registry. Returns (bytes, sha256_hex).
fn download_tarball(
    name: &str,
    version: &str,
    registry: &str,
) -> Result<(Vec<u8>, String), String> {
    let url = format!("{}/api/v1/packages/{}/{}/download", registry, name, version);
    let agent = ureq::Agent::new_with_defaults();
    let mut response = agent
        .get(&url)
        .call()
        .map_err(|e| format!("Failed to download {}@{}: {}", name, version, e))?;

    let mut buf = Vec::new();
    response
        .body_mut()
        .as_reader()
        .read_to_end(&mut buf)
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    let sha256: String = Sha256::digest(&buf)
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect();
    Ok((buf, sha256))
}

/// Resolve a version constraint against the registry (returns exact version + sha256).
/// For now, if the constraint is an exact version ("1.0.0"), use it directly.
fn resolve_version(
    name: &str,
    constraint: &str,
    registry: &str,
) -> Result<(String, String), String> {
    // In v14.0, only exact versions are supported (no semver range solving per REQUIREMENTS.md)
    // Query registry for metadata to get the sha256
    let url = format!("{}/api/v1/packages/{}/{}", registry, name, constraint);
    let agent = ureq::Agent::new_with_defaults();
    let mut response = agent.get(&url).call().map_err(|e| {
        format!(
            "Failed to query registry for {}@{}: {}",
            name, constraint, e
        )
    })?;
    let body = response
        .body_mut()
        .read_to_string()
        .map_err(|e| format!("Failed to read registry response: {}", e))?;
    let json: serde_json::Value = serde_json::from_str(&body)
        .map_err(|e| format!("Failed to parse registry response: {}", e))?;
    let sha256 = json["sha256"]
        .as_str()
        .ok_or_else(|| {
            format!(
                "Registry response missing sha256 for {}@{}",
                name, constraint
            )
        })?
        .to_string();
    Ok((constraint.to_string(), sha256))
}

/// Resolve latest version of a package from the registry.
fn resolve_latest(name: &str, registry: &str) -> Result<(String, String), String> {
    let url = format!("{}/api/v1/packages/{}", registry, name);
    let agent = ureq::Agent::new_with_defaults();
    let mut response = agent
        .get(&url)
        .call()
        .map_err(|e| format!("Failed to query registry for {}: {}", name, e))?;
    let body = response
        .body_mut()
        .read_to_string()
        .map_err(|e| format!("Failed to read registry response: {}", e))?;
    let json: serde_json::Value = serde_json::from_str(&body)
        .map_err(|e| format!("Failed to parse registry response: {}", e))?;
    let version = json["latest"]["version"]
        .as_str()
        .ok_or_else(|| format!("Registry response missing version for {}", name))?
        .to_string();
    let sha256 = json["latest"]["sha256"]
        .as_str()
        .ok_or_else(|| format!("Registry response missing sha256 for {}", name))?
        .to_string();
    Ok((version, sha256))
}

/// Extract a .tar.gz tarball to the given directory.
fn extract_tarball(bytes: &[u8], dest: &Path) -> Result<(), String> {
    let dec = GzDecoder::new(bytes);
    let mut archive = Archive::new(dec);
    archive
        .unpack(dest)
        .map_err(|e| format!("Failed to extract package to {}: {}", dest.display(), e))
}

#[cfg(test)]
mod tests {
    use super::{
        dependency_declaration_snippet, named_install_follow_up, named_install_result_json,
    };

    #[test]
    fn named_install_json_reports_lockfile_and_manifest_stability() {
        let json: serde_json::Value =
            serde_json::from_str(&named_install_result_json("acme/widget", "1.2.3"))
                .expect("named install JSON should parse");

        assert_eq!(json["status"], "ok");
        assert_eq!(json["name"], "acme/widget");
        assert_eq!(json["version"], "1.2.3");
        assert_eq!(json["lockfile"], "mesh.lock");
        assert_eq!(json["manifest_changed"], false);
    }

    #[test]
    fn named_install_follow_up_quotes_scoped_dependency_keys() {
        let declaration = dependency_declaration_snippet("acme/widget", "1.2.3");
        let follow_up = named_install_follow_up("acme/widget", "1.2.3");

        assert_eq!(declaration, "\"acme/widget\" = \"1.2.3\"");
        assert!(follow_up.contains("does not edit mesh.toml"));
        assert!(follow_up.contains(&declaration));
    }
}
