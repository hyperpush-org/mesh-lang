//! JSON field extraction runtime functions for Mesh.
//!
//! Provides `mesh_json_get` and `mesh_json_get_nested` intrinsics that extract
//! string fields from JSON strings using serde_json. These replace PostgreSQL
//! JSONB parsing roundtrips (`$1::jsonb->>'key'`) with in-process extraction.
//!
//! Both functions follow the COALESCE pattern used at all 5 call sites:
//! - If the field exists and is a string, return the string value
//! - If the field exists and is a number/bool/null, convert to string
//! - If the field is missing or JSON is invalid, return empty string ""

use crate::string::{mesh_string_new, MeshString};

// ── Pure Rust helpers (testable without GC) ─────────────────────────

/// Extract a top-level field from a JSON string by key.
/// Returns the field value as a string, or empty string if missing/invalid.
fn json_get_field(json_str: &str, key: &str) -> String {
    let val: serde_json::Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(_) => return String::new(),
    };
    value_to_string(val.get(key))
}

/// Extract a nested field from a JSON string by two path segments.
/// Traverses `value[path1][path2]` and returns the leaf as a string.
/// Returns empty string if any segment is missing or JSON is invalid.
fn json_get_nested_field(json_str: &str, path1: &str, path2: &str) -> String {
    let val: serde_json::Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(_) => return String::new(),
    };
    let nested = val.get(path1).and_then(|v| v.get(path2));
    value_to_string(nested)
}

/// Convert an optional serde_json::Value to a string representation.
/// Matches PostgreSQL `->>` operator behavior:
/// - String values are returned directly (no quotes)
/// - Numbers, bools are converted to string representation
/// - Null and missing values return empty string
fn value_to_string(val: Option<&serde_json::Value>) -> String {
    match val {
        None => String::new(),
        Some(serde_json::Value::String(s)) => s.clone(),
        Some(serde_json::Value::Number(n)) => n.to_string(),
        Some(serde_json::Value::Bool(b)) => {
            if *b {
                "true".to_string()
            } else {
                "false".to_string()
            }
        }
        Some(serde_json::Value::Null) => String::new(),
        // Arrays/objects: return JSON string representation (matches PG ->> on complex types)
        Some(other) => other.to_string(),
    }
}

// ── Extern C wrappers ───────────────────────────────────────────────

/// Extract a top-level string field from a JSON string.
///
/// `Json.get(body, "user_id")` replaces
/// `Pool.query(pool, "SELECT COALESCE($1::jsonb->>'user_id', '') AS user_id", [body])`
///
/// Returns empty string on invalid JSON or missing key (matches COALESCE behavior).
#[no_mangle]
pub extern "C" fn mesh_json_get(json_ptr: *mut u8, key_ptr: *mut u8) -> *mut u8 {
    unsafe {
        let json_str = (*(json_ptr as *const MeshString)).as_str();
        let key = (*(key_ptr as *const MeshString)).as_str();
        let result = json_get_field(json_str, key);
        mesh_string_new(result.as_ptr(), result.len() as u64) as *mut u8
    }
}

/// Extract a nested string field from a JSON string (two levels deep).
///
/// `Json.get_nested(message, "filters", "level")` replaces
/// `Pool.query(pool, "SELECT COALESCE($1::jsonb->'filters'->>'level', '') AS level", [message])`
///
/// Returns empty string on invalid JSON or missing path (matches COALESCE behavior).
#[no_mangle]
pub extern "C" fn mesh_json_get_nested(
    json_ptr: *mut u8,
    path1_ptr: *mut u8,
    path2_ptr: *mut u8,
) -> *mut u8 {
    unsafe {
        let json_str = (*(json_ptr as *const MeshString)).as_str();
        let path1 = (*(path1_ptr as *const MeshString)).as_str();
        let path2 = (*(path2_ptr as *const MeshString)).as_str();
        let result = json_get_nested_field(json_str, path1, path2);
        mesh_string_new(result.as_ptr(), result.len() as u64) as *mut u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_get_string_field() {
        let result = json_get_field(r#"{"user_id":"abc123","name":"Test"}"#, "user_id");
        assert_eq!(result, "abc123");
    }

    #[test]
    fn test_json_get_number_field() {
        let result = json_get_field(r#"{"count":42}"#, "count");
        assert_eq!(result, "42");
    }

    #[test]
    fn test_json_get_bool_field() {
        let result = json_get_field(r#"{"enabled":true}"#, "enabled");
        assert_eq!(result, "true");
    }

    #[test]
    fn test_json_get_null_field() {
        let result = json_get_field(r#"{"field":null}"#, "field");
        assert_eq!(result, "");
    }

    #[test]
    fn test_json_get_missing_field() {
        let result = json_get_field(r#"{"other":"value"}"#, "missing");
        assert_eq!(result, "");
    }

    #[test]
    fn test_json_get_invalid_json() {
        let result = json_get_field("not valid json", "key");
        assert_eq!(result, "");
    }

    #[test]
    fn test_json_get_empty_string() {
        let result = json_get_field("", "key");
        assert_eq!(result, "");
    }

    #[test]
    fn test_json_get_nested_string() {
        let result = json_get_nested_field(
            r#"{"filters":{"level":"error","environment":"production"}}"#,
            "filters",
            "level",
        );
        assert_eq!(result, "error");
    }

    #[test]
    fn test_json_get_nested_environment() {
        let result = json_get_nested_field(
            r#"{"filters":{"level":"error","environment":"production"}}"#,
            "filters",
            "environment",
        );
        assert_eq!(result, "production");
    }

    #[test]
    fn test_json_get_nested_missing_outer() {
        let result = json_get_nested_field(r#"{"other":"value"}"#, "filters", "level");
        assert_eq!(result, "");
    }

    #[test]
    fn test_json_get_nested_missing_inner() {
        let result =
            json_get_nested_field(r#"{"filters":{"level":"error"}}"#, "filters", "missing");
        assert_eq!(result, "");
    }

    #[test]
    fn test_json_get_nested_invalid_json() {
        let result = json_get_nested_field("not json", "a", "b");
        assert_eq!(result, "");
    }

    #[test]
    fn test_json_get_nested_outer_not_object() {
        let result =
            json_get_nested_field(r#"{"filters":"string_not_object"}"#, "filters", "level");
        assert_eq!(result, "");
    }

    #[test]
    fn test_json_get_dynamic_key() {
        // Simulates the pipeline.mpl pattern: extract_condition_field(pool, condition_json, field)
        let json = r#"{"threshold":"100","window_minutes":"5"}"#;
        assert_eq!(json_get_field(json, "threshold"), "100");
        assert_eq!(json_get_field(json, "window_minutes"), "5");
        assert_eq!(json_get_field(json, "nonexistent"), "");
    }

    #[test]
    fn test_json_get_enabled_with_default() {
        // Simulates the alerts.mpl pattern: COALESCE($1::jsonb->>'enabled', 'true')
        let with_field = json_get_field(r#"{"enabled":"false"}"#, "enabled");
        assert_eq!(with_field, "false");

        let without_field = json_get_field(r#"{}"#, "enabled");
        assert_eq!(without_field, ""); // caller defaults to "true" when empty
    }
}
