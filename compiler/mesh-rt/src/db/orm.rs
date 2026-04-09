//! ORM SQL generation module for the Mesh runtime.
//!
//! Provides four `extern "C"` SQL builder functions that produce correctly
//! quoted, parameterized PostgreSQL SQL from structured inputs:
//!
//! - `mesh_orm_build_select`: SELECT with columns, WHERE, ORDER BY, LIMIT, OFFSET
//! - `mesh_orm_build_insert`: INSERT INTO with VALUES and RETURNING
//! - `mesh_orm_build_update`: UPDATE with SET, WHERE, and RETURNING
//! - `mesh_orm_build_delete`: DELETE FROM with WHERE and RETURNING
//!
//! All functions accept Mesh runtime types (MeshString pointers, List pointers)
//! and return MeshString pointers. SQL identifiers are double-quoted per
//! PostgreSQL convention, and parameters use $N placeholders.

use crate::collections::list::{mesh_list_get, mesh_list_length};
use crate::string::{mesh_string_new, MeshString};

// ── Helpers ──────────────────────────────────────────────────────────

/// Quote a SQL identifier with double quotes (PostgreSQL convention).
/// Escapes embedded double quotes by doubling them.
fn quote_ident(name: &str) -> String {
    format!("\"{}\"", name.replace('"', "\"\""))
}

/// Quote a SQL identifier, but pass `*` through unquoted.
/// Used in RETURNING clauses where `*` means "all columns", not a column named `*`.
fn quote_ident_or_star(name: &str) -> String {
    if name == "*" {
        "*".to_string()
    } else {
        quote_ident(name)
    }
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

/// Create a MeshString from a Rust &str and return as *mut u8.
unsafe fn rust_string_to_mesh(s: &str) -> *mut u8 {
    mesh_string_new(s.as_ptr(), s.len() as u64) as *mut u8
}

// ── Pure Rust SQL builders (testable without GC) ─────────────────────

/// Build a SELECT SQL string from pure Rust types.
fn build_select_sql(
    table: &str,
    columns: &[String],
    wheres: &[String],
    orders: &[String],
    limit: i64,
    offset: i64,
) -> String {
    let mut sql = String::new();

    // SELECT clause
    sql.push_str("SELECT ");
    if columns.is_empty() {
        sql.push('*');
    } else {
        let quoted: Vec<String> = columns.iter().map(|c| quote_ident(c)).collect();
        sql.push_str(&quoted.join(", "));
    }

    // FROM clause
    sql.push_str(" FROM ");
    sql.push_str(&quote_ident(table));

    // WHERE clause
    let mut param_idx = 1;
    if !wheres.is_empty() {
        sql.push_str(" WHERE ");
        let mut conditions = Vec::new();
        for w in wheres {
            // Format: "column op" e.g. "name =" or "age >" or "status IS NULL"
            if let Some(space_pos) = w.find(' ') {
                let col = &w[..space_pos];
                let op = w[space_pos + 1..].trim();
                if op == "IS NULL" || op == "IS NOT NULL" {
                    conditions.push(format!("{} {}", quote_ident(col), op));
                } else {
                    conditions.push(format!("{} {} ${}", quote_ident(col), op, param_idx));
                    param_idx += 1;
                }
            } else {
                // Just a column name, default to = operator
                conditions.push(format!("{} = ${}", quote_ident(w), param_idx));
                param_idx += 1;
            }
        }
        sql.push_str(&conditions.join(" AND "));
    }

    // ORDER BY clause
    if !orders.is_empty() {
        sql.push_str(" ORDER BY ");
        let order_parts: Vec<String> = orders
            .iter()
            .map(|o| {
                if let Some(space_pos) = o.rfind(' ') {
                    let col = &o[..space_pos];
                    let dir = &o[space_pos + 1..];
                    format!("{} {}", quote_ident(col), dir.to_uppercase())
                } else {
                    format!("{} ASC", quote_ident(o))
                }
            })
            .collect();
        sql.push_str(&order_parts.join(", "));
    }

    // LIMIT
    if limit >= 0 {
        sql.push_str(&format!(" LIMIT {}", limit));
    }

    // OFFSET
    if offset >= 0 {
        sql.push_str(&format!(" OFFSET {}", offset));
    }

    sql
}

/// Build an INSERT SQL string from pure Rust types.
/// Public within crate for use by repo.rs write operations.
pub(crate) fn build_insert_sql_pure(
    table: &str,
    columns: &[String],
    returning: &[String],
) -> String {
    build_insert_sql(table, columns, returning)
}

fn build_insert_sql(table: &str, columns: &[String], returning: &[String]) -> String {
    let mut sql = String::new();

    sql.push_str("INSERT INTO ");
    sql.push_str(&quote_ident(table));

    // Column list
    sql.push_str(" (");
    let quoted_cols: Vec<String> = columns.iter().map(|c| quote_ident(c)).collect();
    sql.push_str(&quoted_cols.join(", "));
    sql.push(')');

    // VALUES clause with $N placeholders
    sql.push_str(" VALUES (");
    let params: Vec<String> = (1..=columns.len()).map(|i| format!("${}", i)).collect();
    sql.push_str(&params.join(", "));
    sql.push(')');

    // RETURNING clause
    if !returning.is_empty() {
        sql.push_str(" RETURNING ");
        let quoted_ret: Vec<String> = returning.iter().map(|c| quote_ident_or_star(c)).collect();
        sql.push_str(&quoted_ret.join(", "));
    }

    sql
}

/// Build an UPDATE SQL string from pure Rust types.
/// Public within crate for use by repo.rs write operations.
pub(crate) fn build_update_sql_pure(
    table: &str,
    set_columns: &[String],
    wheres: &[String],
    returning: &[String],
) -> String {
    build_update_sql(table, set_columns, wheres, returning)
}

fn build_update_sql(
    table: &str,
    set_columns: &[String],
    wheres: &[String],
    returning: &[String],
) -> String {
    let mut sql = String::new();
    let mut param_idx = 1;

    sql.push_str("UPDATE ");
    sql.push_str(&quote_ident(table));

    // SET clause
    sql.push_str(" SET ");
    let set_parts: Vec<String> = set_columns
        .iter()
        .map(|c| {
            let part = format!("{} = ${}", quote_ident(c), param_idx);
            param_idx += 1;
            part
        })
        .collect();
    sql.push_str(&set_parts.join(", "));

    // WHERE clause (parameters continue from SET)
    if !wheres.is_empty() {
        sql.push_str(" WHERE ");
        let mut conditions = Vec::new();
        for w in wheres {
            if let Some(space_pos) = w.find(' ') {
                let col = &w[..space_pos];
                let op = w[space_pos + 1..].trim();
                if op == "IS NULL" || op == "IS NOT NULL" {
                    conditions.push(format!("{} {}", quote_ident(col), op));
                } else {
                    conditions.push(format!("{} {} ${}", quote_ident(col), op, param_idx));
                    param_idx += 1;
                }
            } else {
                conditions.push(format!("{} = ${}", quote_ident(w), param_idx));
                param_idx += 1;
            }
        }
        sql.push_str(&conditions.join(" AND "));
    }

    // RETURNING clause
    if !returning.is_empty() {
        sql.push_str(" RETURNING ");
        let quoted_ret: Vec<String> = returning.iter().map(|c| quote_ident_or_star(c)).collect();
        sql.push_str(&quoted_ret.join(", "));
    }

    sql
}

/// Build a DELETE SQL string from pure Rust types.
/// Public within crate for use by repo.rs write operations.
pub(crate) fn build_delete_sql_pure(
    table: &str,
    wheres: &[String],
    returning: &[String],
) -> String {
    build_delete_sql(table, wheres, returning)
}

fn build_delete_sql(table: &str, wheres: &[String], returning: &[String]) -> String {
    let mut sql = String::new();
    let mut param_idx = 1;

    sql.push_str("DELETE FROM ");
    sql.push_str(&quote_ident(table));

    // WHERE clause
    if !wheres.is_empty() {
        sql.push_str(" WHERE ");
        let mut conditions = Vec::new();
        for w in wheres {
            if let Some(space_pos) = w.find(' ') {
                let col = &w[..space_pos];
                let op = w[space_pos + 1..].trim();
                if op == "IS NULL" || op == "IS NOT NULL" {
                    conditions.push(format!("{} {}", quote_ident(col), op));
                } else {
                    conditions.push(format!("{} {} ${}", quote_ident(col), op, param_idx));
                    param_idx += 1;
                }
            } else {
                conditions.push(format!("{} = ${}", quote_ident(w), param_idx));
                param_idx += 1;
            }
        }
        sql.push_str(&conditions.join(" AND "));
    }

    // RETURNING clause
    if !returning.is_empty() {
        sql.push_str(" RETURNING ");
        let quoted_ret: Vec<String> = returning.iter().map(|c| quote_ident_or_star(c)).collect();
        sql.push_str(&quoted_ret.join(", "));
    }

    sql
}

/// Build an INSERT ... ON CONFLICT ... DO UPDATE SET SQL string from pure Rust types.
/// Public within crate for use by repo.rs upsert operations.
///
/// Generated SQL pattern:
/// ```sql
/// INSERT INTO "table" ("col1", "col2") VALUES ($1, $2)
/// ON CONFLICT ("unique_col") DO UPDATE SET "col1" = EXCLUDED."col1", "col2" = EXCLUDED."col2"
/// RETURNING *
/// ```
pub(crate) fn build_upsert_sql_pure(
    table: &str,
    columns: &[String],
    conflict_targets: &[String],
    update_columns: &[String],
    returning: &[String],
) -> String {
    let mut sql = String::new();

    sql.push_str("INSERT INTO ");
    sql.push_str(&quote_ident(table));

    // Column list
    sql.push_str(" (");
    let quoted_cols: Vec<String> = columns.iter().map(|c| quote_ident(c)).collect();
    sql.push_str(&quoted_cols.join(", "));
    sql.push(')');

    // VALUES clause with $N placeholders
    sql.push_str(" VALUES (");
    let params: Vec<String> = (1..=columns.len()).map(|i| format!("${}", i)).collect();
    sql.push_str(&params.join(", "));
    sql.push(')');

    // ON CONFLICT clause
    sql.push_str(" ON CONFLICT (");
    let quoted_targets: Vec<String> = conflict_targets.iter().map(|c| quote_ident(c)).collect();
    sql.push_str(&quoted_targets.join(", "));
    sql.push(')');

    // DO UPDATE SET clause using EXCLUDED references
    sql.push_str(" DO UPDATE SET ");
    let set_parts: Vec<String> = update_columns
        .iter()
        .map(|c| format!("{} = EXCLUDED.{}", quote_ident(c), quote_ident(c)))
        .collect();
    sql.push_str(&set_parts.join(", "));

    // RETURNING clause
    if !returning.is_empty() {
        sql.push_str(" RETURNING ");
        let quoted_ret: Vec<String> = returning.iter().map(|c| quote_ident_or_star(c)).collect();
        sql.push_str(&quoted_ret.join(", "));
    }

    sql
}

// ── Extern C functions ───────────────────────────────────────────────

/// Build a parameterized SELECT query.
///
/// # Signature
///
/// `mesh_orm_build_select(table: ptr, columns: ptr, where_clauses: ptr,
///     order_by: ptr, limit: i64, offset: i64) -> ptr (MeshString)`
///
/// - `table`: table name string
/// - `columns`: List<String> of column names (empty = SELECT *)
/// - `where_clauses`: List<String> where each entry is "column op" (e.g. "name =", "age >")
/// - `order_by`: List<String> where each entry is "column direction" (e.g. "name ASC")
/// - `limit`: -1 means no limit, otherwise LIMIT N
/// - `offset`: -1 means no offset, otherwise OFFSET N
#[no_mangle]
pub extern "C" fn mesh_orm_build_select(
    table: *const MeshString,
    columns: *mut u8,
    where_clauses: *mut u8,
    order_by: *mut u8,
    limit: i64,
    offset: i64,
) -> *mut u8 {
    unsafe {
        let table_name = (*table).as_str();
        let cols = list_to_strings(columns);
        let wheres = list_to_strings(where_clauses);
        let orders = list_to_strings(order_by);
        let sql = build_select_sql(table_name, &cols, &wheres, &orders, limit, offset);
        rust_string_to_mesh(&sql)
    }
}

/// Build a parameterized INSERT query.
///
/// # Signature
///
/// `mesh_orm_build_insert(table: ptr, columns: ptr, returning: ptr) -> ptr (MeshString)`
///
/// - `columns`: List<String> of column names for the VALUES clause
/// - `returning`: List<String> for RETURNING clause (empty = no RETURNING)
#[no_mangle]
pub extern "C" fn mesh_orm_build_insert(
    table: *const MeshString,
    columns: *mut u8,
    returning: *mut u8,
) -> *mut u8 {
    unsafe {
        let table_name = (*table).as_str();
        let cols = list_to_strings(columns);
        let ret = list_to_strings(returning);
        let sql = build_insert_sql(table_name, &cols, &ret);
        rust_string_to_mesh(&sql)
    }
}

/// Build a parameterized UPDATE query.
///
/// # Signature
///
/// `mesh_orm_build_update(table: ptr, set_columns: ptr, where_clauses: ptr,
///     returning: ptr) -> ptr (MeshString)`
///
/// - `set_columns`: List<String> of column names for SET clause ($N from 1)
/// - `where_clauses`: List<String> of "column op" entries (params continue after SET)
/// - `returning`: List<String> for RETURNING clause
#[no_mangle]
pub extern "C" fn mesh_orm_build_update(
    table: *const MeshString,
    set_columns: *mut u8,
    where_clauses: *mut u8,
    returning: *mut u8,
) -> *mut u8 {
    unsafe {
        let table_name = (*table).as_str();
        let set_cols = list_to_strings(set_columns);
        let wheres = list_to_strings(where_clauses);
        let ret = list_to_strings(returning);
        let sql = build_update_sql(table_name, &set_cols, &wheres, &ret);
        rust_string_to_mesh(&sql)
    }
}

/// Build a parameterized DELETE query.
///
/// # Signature
///
/// `mesh_orm_build_delete(table: ptr, where_clauses: ptr, returning: ptr) -> ptr (MeshString)`
///
/// - `where_clauses`: List<String> of "column op" entries
/// - `returning`: List<String> for RETURNING clause
#[no_mangle]
pub extern "C" fn mesh_orm_build_delete(
    table: *const MeshString,
    where_clauses: *mut u8,
    returning: *mut u8,
) -> *mut u8 {
    unsafe {
        let table_name = (*table).as_str();
        let wheres = list_to_strings(where_clauses);
        let ret = list_to_strings(returning);
        let sql = build_delete_sql(table_name, &wheres, &ret);
        rust_string_to_mesh(&sql)
    }
}

// ── Unit tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── quote_ident tests ────────────────────────────────────────────

    #[test]
    fn test_quote_ident_simple() {
        assert_eq!(quote_ident("users"), "\"users\"");
    }

    #[test]
    fn test_quote_ident_reserved_word() {
        assert_eq!(quote_ident("table"), "\"table\"");
    }

    #[test]
    fn test_quote_ident_escaped_double_quote() {
        assert_eq!(quote_ident("my\"col"), "\"my\"\"col\"");
    }

    // ── build_select_sql tests ───────────────────────────────────────

    #[test]
    fn test_select_all() {
        let sql = build_select_sql("users", &[], &[], &[], -1, -1);
        assert_eq!(sql, "SELECT * FROM \"users\"");
    }

    #[test]
    fn test_select_with_columns() {
        let sql = build_select_sql("users", &["id".into(), "name".into()], &[], &[], -1, -1);
        assert_eq!(sql, "SELECT \"id\", \"name\" FROM \"users\"");
    }

    #[test]
    fn test_select_with_where() {
        let sql = build_select_sql(
            "users",
            &[],
            &["name =".into(), "age >".into()],
            &[],
            -1,
            -1,
        );
        assert_eq!(
            sql,
            "SELECT * FROM \"users\" WHERE \"name\" = $1 AND \"age\" > $2"
        );
    }

    #[test]
    fn test_select_with_is_null() {
        let sql = build_select_sql(
            "users",
            &[],
            &["deleted_at IS NULL".into(), "name =".into()],
            &[],
            -1,
            -1,
        );
        assert_eq!(
            sql,
            "SELECT * FROM \"users\" WHERE \"deleted_at\" IS NULL AND \"name\" = $1"
        );
    }

    #[test]
    fn test_select_full() {
        let sql = build_select_sql(
            "users",
            &["id".into(), "name".into()],
            &["name =".into()],
            &["name ASC".into()],
            10,
            20,
        );
        assert_eq!(
            sql,
            "SELECT \"id\", \"name\" FROM \"users\" WHERE \"name\" = $1 ORDER BY \"name\" ASC LIMIT 10 OFFSET 20"
        );
    }

    #[test]
    fn test_select_default_operator() {
        let sql = build_select_sql("users", &[], &["id".into()], &[], -1, -1);
        assert_eq!(sql, "SELECT * FROM \"users\" WHERE \"id\" = $1");
    }

    #[test]
    fn test_select_order_default_direction() {
        let sql = build_select_sql("users", &[], &[], &["name".into()], -1, -1);
        assert_eq!(sql, "SELECT * FROM \"users\" ORDER BY \"name\" ASC");
    }

    #[test]
    fn test_select_multiple_orders() {
        let sql = build_select_sql(
            "users",
            &[],
            &[],
            &["name ASC".into(), "age DESC".into()],
            -1,
            -1,
        );
        assert_eq!(
            sql,
            "SELECT * FROM \"users\" ORDER BY \"name\" ASC, \"age\" DESC"
        );
    }

    // ── build_insert_sql tests ───────────────────────────────────────

    #[test]
    fn test_insert_basic() {
        let sql = build_insert_sql(
            "users",
            &["name".into(), "email".into()],
            &["id".into(), "name".into()],
        );
        assert_eq!(
            sql,
            "INSERT INTO \"users\" (\"name\", \"email\") VALUES ($1, $2) RETURNING \"id\", \"name\""
        );
    }

    #[test]
    fn test_insert_returning_star() {
        let sql = build_insert_sql("users", &["name".into(), "email".into()], &["*".into()]);
        assert_eq!(
            sql,
            "INSERT INTO \"users\" (\"name\", \"email\") VALUES ($1, $2) RETURNING *"
        );
    }

    #[test]
    fn test_insert_no_returning() {
        let sql = build_insert_sql("users", &["name".into()], &[]);
        assert_eq!(sql, "INSERT INTO \"users\" (\"name\") VALUES ($1)");
    }

    // ── build_update_sql tests ───────────────────────────────────────

    #[test]
    fn test_update_basic() {
        let sql = build_update_sql(
            "users",
            &["name".into(), "email".into()],
            &["id =".into()],
            &["id".into()],
        );
        assert_eq!(
            sql,
            "UPDATE \"users\" SET \"name\" = $1, \"email\" = $2 WHERE \"id\" = $3 RETURNING \"id\""
        );
    }

    #[test]
    fn test_update_no_where_no_returning() {
        let sql = build_update_sql("users", &["name".into()], &[], &[]);
        assert_eq!(sql, "UPDATE \"users\" SET \"name\" = $1");
    }

    // ── build_delete_sql tests ───────────────────────────────────────

    #[test]
    fn test_delete_basic() {
        let sql = build_delete_sql("users", &["id =".into()], &[]);
        assert_eq!(sql, "DELETE FROM \"users\" WHERE \"id\" = $1");
    }

    #[test]
    fn test_delete_with_returning() {
        let sql = build_delete_sql("users", &["id =".into()], &["id".into()]);
        assert_eq!(
            sql,
            "DELETE FROM \"users\" WHERE \"id\" = $1 RETURNING \"id\""
        );
    }

    #[test]
    fn test_delete_no_where() {
        let sql = build_delete_sql("users", &[], &[]);
        assert_eq!(sql, "DELETE FROM \"users\"");
    }

    // ── build_upsert_sql tests ──────────────────────────────────────────

    #[test]
    fn test_build_upsert_sql() {
        let sql = build_upsert_sql_pure(
            "issues",
            &[
                "project_id".into(),
                "fingerprint".into(),
                "title".into(),
                "level".into(),
                "event_count".into(),
            ],
            &["project_id".into(), "fingerprint".into()],
            &["title".into(), "level".into(), "event_count".into()],
            &["*".into()],
        );
        assert_eq!(
            sql,
            "INSERT INTO \"issues\" (\"project_id\", \"fingerprint\", \"title\", \"level\", \"event_count\") VALUES ($1, $2, $3, $4, $5) ON CONFLICT (\"project_id\", \"fingerprint\") DO UPDATE SET \"title\" = EXCLUDED.\"title\", \"level\" = EXCLUDED.\"level\", \"event_count\" = EXCLUDED.\"event_count\" RETURNING *"
        );
    }
}
