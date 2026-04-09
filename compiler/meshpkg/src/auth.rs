use std::path::PathBuf;

const CREDENTIALS_FILE: &str = "credentials";
const MESH_DIR: &str = ".mesh";

pub fn credentials_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(MESH_DIR)
        .join(CREDENTIALS_FILE)
}

/// Read the auth token from ~/.mesh/credentials.
/// Returns descriptive error if not logged in.
pub fn read_token() -> Result<String, String> {
    let path = credentials_path();
    let content = std::fs::read_to_string(&path)
        .map_err(|_| "Not logged in. Run `meshpkg login` first.".to_string())?;
    let table: toml::Table =
        toml::from_str(&content).map_err(|e| format!("Corrupted credentials file: {}", e))?;
    table
        .get("registry")
        .and_then(|r| r.get("token"))
        .and_then(|t| t.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| "No token found in credentials file.".to_string())
}

/// Write an auth token to ~/.mesh/credentials.
/// Creates ~/.mesh/ directory if it does not exist.
pub fn write_token(token: &str) -> Result<(), String> {
    let path = credentials_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("Failed to create ~/.mesh/: {}", e))?;
    }
    let content = format!("[registry]\ntoken = \"{}\"\n", token);
    std::fs::write(&path, content).map_err(|e| format!("Failed to write credentials: {}", e))
}
