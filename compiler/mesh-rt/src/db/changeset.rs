//! Changeset validation pipeline for the Mesh runtime.
//!
//! Provides an opaque Changeset struct that accumulates validated changes
//! and errors. Each validator function clones the changeset, checks its
//! condition, adds errors if needed, and returns a new changeset. This
//! enables pipe-chain composition where all validators run without
//! short-circuiting.
//!
//! ## Changeset object layout (8 slots, 64 bytes)
//!
//! | Slot | Offset | Name        | Type                          |
//! |------|--------|-------------|-------------------------------|
//! |  0   |   0    | data        | *mut u8 (Map<String,String>)  |
//! |  1   |   8    | changes     | *mut u8 (Map<String,String>)  |
//! |  2   |  16    | errors      | *mut u8 (Map<String,String>)  |
//! |  3   |  24    | valid       | i64: 1 = valid, 0 = invalid   |
//! |  4   |  32    | field_types | *mut u8 (List<String>)         |
//! |  5   |  40    | table       | *mut u8 (MeshString or null)  |
//! |  6   |  48    | primary_key | *mut u8 (MeshString or null)  |
//! |  7   |  56    | action      | i64: 0 = insert, 1 = update   |

use crate::collections::list::{mesh_list_get, mesh_list_length, mesh_list_new};
use crate::collections::map::{
    mesh_map_get, mesh_map_has_key, mesh_map_new_typed, mesh_map_put, mesh_map_size,
};
use crate::gc::mesh_gc_alloc_actor;
use crate::string::{mesh_string_new, MeshString};

// ── Constants ────────────────────────────────────────────────────────

pub(crate) const CS_SLOTS: usize = 8;
pub(crate) const CS_SIZE: usize = CS_SLOTS * 8; // 64 bytes

pub(crate) const SLOT_DATA: usize = 0;
pub(crate) const SLOT_CHANGES: usize = 1;
pub(crate) const SLOT_ERRORS: usize = 2;
pub(crate) const SLOT_VALID: usize = 3;
pub(crate) const SLOT_FIELD_TYPES: usize = 4;
#[allow(dead_code)]
pub(crate) const SLOT_TABLE: usize = 5;
#[allow(dead_code)]
pub(crate) const SLOT_PK: usize = 6;
#[allow(dead_code)]
pub(crate) const SLOT_ACTION: usize = 7;

// ── Slot access helpers ──────────────────────────────────────────────

unsafe fn cs_get(cs: *mut u8, slot: usize) -> *mut u8 {
    *(cs.add(slot * 8) as *const *mut u8)
}

unsafe fn cs_set(cs: *mut u8, slot: usize, val: *mut u8) {
    *(cs.add(slot * 8) as *mut *mut u8) = val;
}

unsafe fn cs_get_int(cs: *mut u8, slot: usize) -> i64 {
    *(cs.add(slot * 8) as *const i64)
}

unsafe fn cs_set_int(cs: *mut u8, slot: usize, val: i64) {
    *(cs.add(slot * 8) as *mut i64) = val;
}

// ── String helpers ───────────────────────────────────────────────────

/// Create a MeshString from a Rust &str and return as *mut u8.
unsafe fn rust_str_to_mesh(s: &str) -> *mut u8 {
    mesh_string_new(s.as_ptr(), s.len() as u64) as *mut u8
}

/// Read a MeshString pointer as a Rust &str.
unsafe fn mesh_str_ref(ptr: *mut u8) -> &'static str {
    let ms = ptr as *const MeshString;
    (*ms).as_str()
}

/// Extract a Vec<String> from a Mesh List<String> pointer.
unsafe fn list_to_strings(list_ptr: *mut u8) -> Vec<String> {
    let len = mesh_list_length(list_ptr);
    let mut result = Vec::with_capacity(len as usize);
    for i in 0..len {
        let elem = mesh_list_get(list_ptr, i) as *const MeshString;
        if !elem.is_null() {
            result.push((*elem).as_str().to_string());
        }
    }
    result
}

// ── Allocation ───────────────────────────────────────────────────────

/// Allocate a fresh Changeset with empty maps and valid=1.
unsafe fn alloc_changeset() -> *mut u8 {
    let cs = mesh_gc_alloc_actor(CS_SIZE as u64, 8);
    std::ptr::write_bytes(cs, 0, CS_SIZE);
    cs_set(cs, SLOT_DATA, mesh_map_new_typed(1)); // string-keyed
    cs_set(cs, SLOT_CHANGES, mesh_map_new_typed(1));
    cs_set(cs, SLOT_ERRORS, mesh_map_new_typed(1));
    cs_set_int(cs, SLOT_VALID, 1); // valid until proven otherwise
    cs_set(cs, SLOT_FIELD_TYPES, mesh_list_new());
    cs_set_int(cs, SLOT_ACTION, 0); // insert by default
    cs
}

/// Clone a Changeset: allocate new 64 bytes and copy all data from source.
unsafe fn clone_changeset(src: *mut u8) -> *mut u8 {
    let dst = mesh_gc_alloc_actor(CS_SIZE as u64, 8);
    std::ptr::copy_nonoverlapping(src, dst, CS_SIZE);
    dst
}

// ── Type coercion ────────────────────────────────────────────────────

fn coerce_value(val: &str, sql_type: &str) -> Result<String, ()> {
    match sql_type {
        "TEXT" => Ok(val.to_string()),
        "BIGINT" => val
            .trim()
            .parse::<i64>()
            .map(|v| v.to_string())
            .map_err(|_| ()),
        "DOUBLE PRECISION" => val
            .trim()
            .parse::<f64>()
            .map(|v| v.to_string())
            .map_err(|_| ()),
        "BOOLEAN" => match val.to_lowercase().as_str() {
            "true" | "t" | "1" | "yes" => Ok("true".to_string()),
            "false" | "f" | "0" | "no" => Ok("false".to_string()),
            _ => Err(()),
        },
        _ => Ok(val.to_string()), // unknown type -- pass through
    }
}

// ── Cast functions ───────────────────────────────────────────────────

/// Changeset.cast(data, params, allowed) -- 3-arg version, no type coercion.
///
/// Filters `params` to only include keys present in `allowed` list.
/// Creates a new changeset with the filtered params as `changes`.
#[no_mangle]
pub extern "C" fn mesh_changeset_cast(data: *mut u8, params: *mut u8, allowed: *mut u8) -> *mut u8 {
    unsafe {
        let cs = alloc_changeset();
        cs_set(cs, SLOT_DATA, data);

        let allowed_names = list_to_strings(allowed);
        let mut changes = mesh_map_new_typed(1);

        for field_name in &allowed_names {
            let key_mesh = rust_str_to_mesh(field_name);
            let key_u64 = key_mesh as u64;
            if mesh_map_has_key(params, key_u64) != 0 {
                let val = mesh_map_get(params, key_u64);
                changes = mesh_map_put(changes, key_u64, val);
            }
        }

        cs_set(cs, SLOT_CHANGES, changes);
        cs
    }
}

/// Changeset.cast_with_types(data, params, allowed, field_types) -- 4-arg version with coercion.
///
/// Same as cast but additionally coerces string values based on SQL type metadata.
/// field_types is a List<String> of "field_name:SQL_TYPE" entries.
#[no_mangle]
pub extern "C" fn mesh_changeset_cast_with_types(
    data: *mut u8,
    params: *mut u8,
    allowed: *mut u8,
    field_types: *mut u8,
) -> *mut u8 {
    unsafe {
        let cs = alloc_changeset();
        cs_set(cs, SLOT_DATA, data);
        cs_set(cs, SLOT_FIELD_TYPES, field_types);

        // Build field_type lookup from "field:SQL_TYPE" entries
        let ft_entries = list_to_strings(field_types);
        let type_map: std::collections::HashMap<String, String> = ft_entries
            .iter()
            .filter_map(|entry| {
                let parts: Vec<&str> = entry.splitn(2, ':').collect();
                if parts.len() == 2 {
                    Some((parts[0].to_string(), parts[1].to_string()))
                } else {
                    None
                }
            })
            .collect();

        let allowed_names = list_to_strings(allowed);
        let mut changes = mesh_map_new_typed(1);
        let mut errors = mesh_map_new_typed(1);

        for field_name in &allowed_names {
            let key_mesh = rust_str_to_mesh(field_name);
            let key_u64 = key_mesh as u64;
            if mesh_map_has_key(params, key_u64) != 0 {
                let val = mesh_map_get(params, key_u64);
                let val_str = mesh_str_ref(val as *mut u8);

                if let Some(sql_type) = type_map.get(field_name) {
                    match coerce_value(val_str, sql_type) {
                        Ok(coerced) => {
                            let coerced_mesh = rust_str_to_mesh(&coerced);
                            changes = mesh_map_put(changes, key_u64, coerced_mesh as u64);
                        }
                        Err(()) => {
                            let err_msg = rust_str_to_mesh("is invalid");
                            errors = mesh_map_put(errors, key_u64, err_msg as u64);
                        }
                    }
                } else {
                    // No type info for this field -- pass through as-is
                    changes = mesh_map_put(changes, key_u64, val);
                }
            }
        }

        cs_set(cs, SLOT_CHANGES, changes);
        cs_set(cs, SLOT_ERRORS, errors);
        let has_errors = mesh_map_size(errors) > 0;
        cs_set_int(cs, SLOT_VALID, if has_errors { 0 } else { 1 });
        cs
    }
}

// ── Validators ───────────────────────────────────────────────────────

/// Changeset.validate_required(changeset, fields_list)
///
/// Checks that each field in fields_list exists and is non-empty in
/// either `changes` or `data`. Adds "can't be blank" error for missing fields.
#[no_mangle]
pub extern "C" fn mesh_changeset_validate_required(cs: *mut u8, fields: *mut u8) -> *mut u8 {
    unsafe {
        let new_cs = clone_changeset(cs);
        let field_names = list_to_strings(fields);
        let changes = cs_get(new_cs, SLOT_CHANGES);
        let data = cs_get(new_cs, SLOT_DATA);
        let mut errors = cs_get(new_cs, SLOT_ERRORS);

        for field in &field_names {
            let key_mesh = rust_str_to_mesh(field);
            let key_u64 = key_mesh as u64;

            // Check if field has a non-empty value in changes or data
            let is_present = if mesh_map_has_key(changes, key_u64) != 0 {
                let val = mesh_map_get(changes, key_u64);
                let s = mesh_str_ref(val as *mut u8);
                !s.is_empty()
            } else if mesh_map_has_key(data, key_u64) != 0 {
                let val = mesh_map_get(data, key_u64);
                let s = mesh_str_ref(val as *mut u8);
                !s.is_empty()
            } else {
                false
            };

            if !is_present {
                // Only add error if no error exists for this field yet
                if mesh_map_has_key(errors, key_u64) == 0 {
                    let msg = rust_str_to_mesh("can't be blank");
                    errors = mesh_map_put(errors, key_u64, msg as u64);
                }
            }
        }

        cs_set(new_cs, SLOT_ERRORS, errors);
        cs_set_int(
            new_cs,
            SLOT_VALID,
            if mesh_map_size(errors) > 0 { 0 } else { 1 },
        );
        new_cs
    }
}

/// Changeset.validate_length(changeset, field, min, max)
///
/// Checks that the string length of the field value is within [min, max].
/// Use -1 for "not set" (no bound). Only validates fields present in changes.
#[no_mangle]
pub extern "C" fn mesh_changeset_validate_length(
    cs: *mut u8,
    field: *mut u8,
    min: i64,
    max: i64,
) -> *mut u8 {
    unsafe {
        let new_cs = clone_changeset(cs);
        let changes = cs_get(new_cs, SLOT_CHANGES);
        let mut errors = cs_get(new_cs, SLOT_ERRORS);

        let field_str = mesh_str_ref(field);
        let key_mesh = rust_str_to_mesh(field_str);
        let key_u64 = key_mesh as u64;

        // Only validate if field exists in changes
        if mesh_map_has_key(changes, key_u64) != 0 {
            let val = mesh_map_get(changes, key_u64);
            let val_str = mesh_str_ref(val as *mut u8);
            let len = val_str.len() as i64;

            // Only add first error per field
            if mesh_map_has_key(errors, key_u64) == 0 {
                if min != -1 && len < min {
                    let msg = format!("should be at least {} character(s)", min);
                    let msg_mesh = rust_str_to_mesh(&msg);
                    errors = mesh_map_put(errors, key_u64, msg_mesh as u64);
                } else if max != -1 && len > max {
                    let msg = format!("should be at most {} character(s)", max);
                    let msg_mesh = rust_str_to_mesh(&msg);
                    errors = mesh_map_put(errors, key_u64, msg_mesh as u64);
                }
            }
        }

        cs_set(new_cs, SLOT_ERRORS, errors);
        cs_set_int(
            new_cs,
            SLOT_VALID,
            if mesh_map_size(errors) > 0 { 0 } else { 1 },
        );
        new_cs
    }
}

/// Changeset.validate_format(changeset, field, pattern)
///
/// Checks that the field value contains the pattern substring.
/// Adds "has invalid format" error if pattern is not found.
#[no_mangle]
pub extern "C" fn mesh_changeset_validate_format(
    cs: *mut u8,
    field: *mut u8,
    pattern: *mut u8,
) -> *mut u8 {
    unsafe {
        let new_cs = clone_changeset(cs);
        let changes = cs_get(new_cs, SLOT_CHANGES);
        let mut errors = cs_get(new_cs, SLOT_ERRORS);

        let field_str = mesh_str_ref(field);
        let pattern_str = mesh_str_ref(pattern);
        let key_mesh = rust_str_to_mesh(field_str);
        let key_u64 = key_mesh as u64;

        // Only validate if field exists in changes
        if mesh_map_has_key(changes, key_u64) != 0 {
            let val = mesh_map_get(changes, key_u64);
            let val_str = mesh_str_ref(val as *mut u8);

            if !val_str.contains(pattern_str) {
                // Only add error if no error exists for this field yet
                if mesh_map_has_key(errors, key_u64) == 0 {
                    let msg = rust_str_to_mesh("has invalid format");
                    errors = mesh_map_put(errors, key_u64, msg as u64);
                }
            }
        }

        cs_set(new_cs, SLOT_ERRORS, errors);
        cs_set_int(
            new_cs,
            SLOT_VALID,
            if mesh_map_size(errors) > 0 { 0 } else { 1 },
        );
        new_cs
    }
}

/// Changeset.validate_inclusion(changeset, field, allowed_values_list)
///
/// Checks that the field value is one of the allowed values.
/// Adds "is invalid" error if not found in the list.
#[no_mangle]
pub extern "C" fn mesh_changeset_validate_inclusion(
    cs: *mut u8,
    field: *mut u8,
    allowed_values: *mut u8,
) -> *mut u8 {
    unsafe {
        let new_cs = clone_changeset(cs);
        let changes = cs_get(new_cs, SLOT_CHANGES);
        let mut errors = cs_get(new_cs, SLOT_ERRORS);

        let field_str = mesh_str_ref(field);
        let key_mesh = rust_str_to_mesh(field_str);
        let key_u64 = key_mesh as u64;

        // Only validate if field exists in changes
        if mesh_map_has_key(changes, key_u64) != 0 {
            let val = mesh_map_get(changes, key_u64);
            let val_str = mesh_str_ref(val as *mut u8);

            // Check if value is in the allowed list
            let allowed = list_to_strings(allowed_values);
            let is_valid = allowed.iter().any(|a| a == val_str);

            if !is_valid {
                if mesh_map_has_key(errors, key_u64) == 0 {
                    let msg = rust_str_to_mesh("is invalid");
                    errors = mesh_map_put(errors, key_u64, msg as u64);
                }
            }
        }

        cs_set(new_cs, SLOT_ERRORS, errors);
        cs_set_int(
            new_cs,
            SLOT_VALID,
            if mesh_map_size(errors) > 0 { 0 } else { 1 },
        );
        new_cs
    }
}

/// Changeset.validate_number(changeset, field, gt, lt, gte, lte)
///
/// Checks that the field value (parsed as i64) is within the specified bounds.
/// Use -1 for "not set" (no bound). Adds appropriate error messages.
#[no_mangle]
pub extern "C" fn mesh_changeset_validate_number(
    cs: *mut u8,
    field: *mut u8,
    gt: i64,
    lt: i64,
    gte: i64,
    lte: i64,
) -> *mut u8 {
    unsafe {
        let new_cs = clone_changeset(cs);
        let changes = cs_get(new_cs, SLOT_CHANGES);
        let mut errors = cs_get(new_cs, SLOT_ERRORS);

        let field_str = mesh_str_ref(field);
        let key_mesh = rust_str_to_mesh(field_str);
        let key_u64 = key_mesh as u64;

        // Only validate if field exists in changes
        if mesh_map_has_key(changes, key_u64) != 0 {
            let val = mesh_map_get(changes, key_u64);
            let val_str = mesh_str_ref(val as *mut u8);

            // Only add first error per field
            if mesh_map_has_key(errors, key_u64) == 0 {
                match val_str.trim().parse::<i64>() {
                    Err(_) => {
                        let msg = rust_str_to_mesh("is not a number");
                        errors = mesh_map_put(errors, key_u64, msg as u64);
                    }
                    Ok(num) => {
                        let mut err_msg: Option<String> = None;
                        if gt != -1 && num <= gt {
                            err_msg = Some(format!("must be greater than {}", gt));
                        } else if lt != -1 && num >= lt {
                            err_msg = Some(format!("must be less than {}", lt));
                        } else if gte != -1 && num < gte {
                            err_msg = Some(format!("must be greater than or equal to {}", gte));
                        } else if lte != -1 && num > lte {
                            err_msg = Some(format!("must be less than or equal to {}", lte));
                        }
                        if let Some(msg) = err_msg {
                            let msg_mesh = rust_str_to_mesh(&msg);
                            errors = mesh_map_put(errors, key_u64, msg_mesh as u64);
                        }
                    }
                }
            }
        }

        cs_set(new_cs, SLOT_ERRORS, errors);
        cs_set_int(
            new_cs,
            SLOT_VALID,
            if mesh_map_size(errors) > 0 { 0 } else { 1 },
        );
        new_cs
    }
}

// ── Field accessors ──────────────────────────────────────────────────

/// Changeset.valid(changeset) -> Bool
///
/// Returns 1 (true) if changeset has no errors, 0 (false) otherwise.
/// Return type is i64 cast to *mut u8, matching the Bool convention.
#[no_mangle]
pub extern "C" fn mesh_changeset_valid(cs: *mut u8) -> *mut u8 {
    unsafe { cs_get_int(cs, SLOT_VALID) as *mut u8 }
}

/// Changeset.errors(changeset) -> Map<String,String>
///
/// Returns the errors map.
#[no_mangle]
pub extern "C" fn mesh_changeset_errors(cs: *mut u8) -> *mut u8 {
    unsafe { cs_get(cs, SLOT_ERRORS) }
}

/// Changeset.changes(changeset) -> Map<String,String>
///
/// Returns the changes map.
#[no_mangle]
pub extern "C" fn mesh_changeset_changes(cs: *mut u8) -> *mut u8 {
    unsafe { cs_get(cs, SLOT_CHANGES) }
}

/// Changeset.get_change(changeset, field) -> String
///
/// Returns the value of a field from the changes map, or empty string if not found.
#[no_mangle]
pub extern "C" fn mesh_changeset_get_change(cs: *mut u8, field: *mut u8) -> *mut u8 {
    unsafe {
        let changes = cs_get(cs, SLOT_CHANGES);
        let field_str = mesh_str_ref(field);
        let key_mesh = rust_str_to_mesh(field_str);
        let key_u64 = key_mesh as u64;

        if mesh_map_has_key(changes, key_u64) != 0 {
            mesh_map_get(changes, key_u64) as *mut u8
        } else {
            rust_str_to_mesh("")
        }
    }
}

/// Changeset.get_error(changeset, field) -> String
///
/// Returns the error message for a field, or empty string if no error.
#[no_mangle]
pub extern "C" fn mesh_changeset_get_error(cs: *mut u8, field: *mut u8) -> *mut u8 {
    unsafe {
        let errors = cs_get(cs, SLOT_ERRORS);
        let field_str = mesh_str_ref(field);
        let key_mesh = rust_str_to_mesh(field_str);
        let key_u64 = key_mesh as u64;

        if mesh_map_has_key(errors, key_u64) != 0 {
            mesh_map_get(errors, key_u64) as *mut u8
        } else {
            rust_str_to_mesh("")
        }
    }
}

// ── Constraint-to-changeset error mapping ───────────────────────────

/// Map a PostgreSQL SQLSTATE code and constraint name to a (field, message) pair.
///
/// Handles:
/// - `23505` (unique_violation): "has already been taken"
/// - `23503` (foreign_key_violation): "does not exist"
/// - `23502` (not_null_violation): "can't be blank"
///
/// Returns `None` for unknown SQLSTATE codes.
pub(crate) fn map_constraint_error(
    sqlstate: &str,
    constraint: &str,
    table: &str,
    column: &str,
) -> Option<(String, String)> {
    match sqlstate {
        "23505" => {
            // unique_violation
            let field = extract_field_from_constraint(constraint, table)
                .unwrap_or_else(|| "_base".to_string());
            Some((field, "has already been taken".to_string()))
        }
        "23503" => {
            // foreign_key_violation
            let field = extract_field_from_constraint(constraint, table)
                .unwrap_or_else(|| "_base".to_string());
            Some((field, "does not exist".to_string()))
        }
        "23502" => {
            // not_null_violation
            if !column.is_empty() {
                Some((column.to_string(), "can't be blank".to_string()))
            } else {
                Some(("_base".to_string(), "can't be blank".to_string()))
            }
        }
        _ => None,
    }
}

/// Extract a field name from a PostgreSQL constraint name using naming conventions.
///
/// PostgreSQL constraint names follow these conventions:
/// - `{table}_{column}_key` for unique constraints (e.g., "users_email_key" -> "email")
/// - `{table}_{column}_fkey` for foreign keys (e.g., "posts_user_id_fkey" -> "user_id")
/// - `{table}_pkey` for primary key (e.g., "users_pkey" -> None)
/// - `{table}_{column}_check` for check constraints
///
/// Returns the extracted field name, or None if the constraint name doesn't match.
pub(crate) fn extract_field_from_constraint(
    constraint_name: &str,
    table_name: &str,
) -> Option<String> {
    // Strip the {table}_ prefix
    let prefix = format!("{}_", table_name);
    let remainder = constraint_name.strip_prefix(&prefix)?;

    // Try each known suffix
    for suffix in &["_key", "_fkey", "_pkey", "_check"] {
        if let Some(field) = remainder.strip_suffix(suffix) {
            if field.is_empty() {
                return None; // e.g., "users_pkey" -> empty field
            }
            return Some(field.to_string());
        }
    }

    // No known suffix matched -- return None
    None
}

/// Add a constraint error to a changeset, returning a new changeset with the error added.
///
/// Clones the changeset, adds the error if no error exists for that field yet,
/// updates SLOT_VALID, and returns the new changeset pointer.
///
/// # Safety
///
/// The `cs` pointer must be a valid changeset allocation.
pub(crate) unsafe fn add_constraint_error_to_changeset(
    cs: *mut u8,
    field: &str,
    message: &str,
) -> *mut u8 {
    let new_cs = clone_changeset(cs);
    let mut errors = cs_get(new_cs, SLOT_ERRORS);

    let key_mesh = rust_str_to_mesh(field);
    let key_u64 = key_mesh as u64;

    // Only add if no error exists for this field yet
    if mesh_map_has_key(errors, key_u64) == 0 {
        let msg_mesh = rust_str_to_mesh(message);
        errors = mesh_map_put(errors, key_u64, msg_mesh as u64);
    }

    cs_set(new_cs, SLOT_ERRORS, errors);
    cs_set_int(
        new_cs,
        SLOT_VALID,
        if mesh_map_size(errors) > 0 { 0 } else { 1 },
    );
    new_cs
}
