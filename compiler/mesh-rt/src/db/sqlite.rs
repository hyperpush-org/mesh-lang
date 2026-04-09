//! SQLite C FFI wrapper functions for the Mesh runtime.
//!
//! Provides four extern "C" functions that Mesh programs call to interact
//! with SQLite databases:
//! - `mesh_sqlite_open`: Open a database connection
//! - `mesh_sqlite_close`: Close a connection
//! - `mesh_sqlite_execute`: Execute a write query (INSERT/UPDATE/DELETE/CREATE)
//! - `mesh_sqlite_query`: Execute a read query (SELECT), returns rows
//!
//! Connection handles are opaque u64 values (Box::into_raw as u64) for GC
//! safety. The GC never traces integer values, so the connection won't be
//! corrupted by garbage collection.

use libsqlite3_sys::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

use crate::collections::list::{mesh_list_append, mesh_list_new};
use crate::collections::map::{mesh_map_new_typed, mesh_map_put};
use crate::io::alloc_result;
use crate::string::{mesh_string_new, MeshString};

/// Wrapper around a raw SQLite database pointer.
struct SqliteConn {
    db: *mut sqlite3,
}

/// RAII guard ensuring sqlite3_finalize is always called on a prepared
/// statement, even when an error causes an early return.
struct StmtGuard {
    stmt: *mut sqlite3_stmt,
}

impl Drop for StmtGuard {
    fn drop(&mut self) {
        if !self.stmt.is_null() {
            unsafe {
                sqlite3_finalize(self.stmt);
            }
        }
    }
}

/// SQLITE_TRANSIENT tells SQLite to copy bound parameter data immediately.
/// It is defined as ((void(*)(void*))-1) in the C API, which is -1 cast to
/// a destructor function pointer.
const SQLITE_TRANSIENT_VALUE: isize = -1;

/// Extract a Rust &str from a raw MeshString pointer.
///
/// # Safety
///
/// The pointer must reference a valid MeshString allocation.
unsafe fn mesh_str_to_rust(s: *const MeshString) -> &'static str {
    (*s).as_str()
}

/// Create a MeshString from a Rust &str and return as *mut u8.
fn rust_str_to_mesh(s: &str) -> *mut u8 {
    mesh_string_new(s.as_ptr(), s.len() as u64) as *mut u8
}

/// Create an error MeshResult from a Rust string.
fn err_result(msg: &str) -> *mut u8 {
    let s = rust_str_to_mesh(msg);
    alloc_result(1, s) as *mut u8
}

fn box_u64_payload(value: u64) -> *mut u8 {
    Box::into_raw(Box::new(value)) as *mut u8
}

fn box_i64_payload(value: i64) -> *mut u8 {
    Box::into_raw(Box::new(value)) as *mut u8
}

#[cfg(test)]
unsafe fn unbox_u64_payload(ptr: *mut u8) -> u64 {
    *(ptr as *const u64)
}

#[cfg(test)]
unsafe fn unbox_i64_payload(ptr: *mut u8) -> i64 {
    *(ptr as *const i64)
}

/// Create an error MeshResult from a sqlite3 error message.
unsafe fn sqlite_err_result(db: *mut sqlite3) -> *mut u8 {
    let c_msg = sqlite3_errmsg(db);
    if c_msg.is_null() {
        err_result("unknown SQLite error")
    } else {
        let msg = CStr::from_ptr(c_msg).to_string_lossy();
        err_result(&msg)
    }
}

/// Read the MeshList of MeshString parameters and bind them to a prepared
/// statement using sqlite3_bind_text with SQLITE_TRANSIENT.
///
/// MeshList layout: `{ len: u64, cap: u64, data: [u64; cap] }`
/// Each element is a u64 that is actually a pointer to a MeshString.
///
/// Returns Ok(()) on success, Err(error_string) on bind failure.
unsafe fn bind_params(
    db: *mut sqlite3,
    stmt: *mut sqlite3_stmt,
    params: *mut u8,
) -> Result<(), *mut u8> {
    let len = *(params as *const u64);
    let data_ptr = (params as *const u64).add(2); // skip len + cap

    // We need to keep CStrings alive until all binds are complete.
    let mut cstrings = Vec::with_capacity(len as usize);

    for i in 0..len as usize {
        let param_ptr = *data_ptr.add(i) as *const MeshString;
        let param_str = mesh_str_to_rust(param_ptr);
        let cstr = match CString::new(param_str) {
            Ok(c) => c,
            Err(_) => return Err(err_result("parameter contains null byte")),
        };
        cstrings.push(cstr);
    }

    for (i, cstr) in cstrings.iter().enumerate() {
        let destructor: Option<unsafe extern "C" fn(*mut std::ffi::c_void)> =
            std::mem::transmute::<isize, Option<unsafe extern "C" fn(*mut std::ffi::c_void)>>(
                SQLITE_TRANSIENT_VALUE,
            );
        let rc = sqlite3_bind_text(stmt, (i + 1) as c_int, cstr.as_ptr(), -1, destructor);
        if rc != SQLITE_OK {
            return Err(sqlite_err_result(db));
        }
    }

    Ok(())
}

/// Open a SQLite database.
///
/// # Signature
///
/// `mesh_sqlite_open(path: *const MeshString) -> *mut u8 (MeshResult<u64, String>)`
///
/// Returns MeshResult with tag 0 (Ok) containing the connection handle as
/// a u64, or tag 1 (Err) containing an error message string.
#[no_mangle]
pub extern "C" fn mesh_sqlite_open(path: *const MeshString) -> *mut u8 {
    unsafe {
        let path_str = mesh_str_to_rust(path);
        let c_path = match CString::new(path_str) {
            Ok(c) => c,
            Err(_) => return err_result("path contains null byte"),
        };

        let mut db: *mut sqlite3 = std::ptr::null_mut();
        let rc = sqlite3_open_v2(
            c_path.as_ptr(),
            &mut db,
            SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE,
            std::ptr::null(),
        );

        if rc != SQLITE_OK {
            let result = sqlite_err_result(db);
            if !db.is_null() {
                sqlite3_close(db);
            }
            return result;
        }

        let conn = Box::new(SqliteConn { db });
        let handle = Box::into_raw(conn) as u64;
        alloc_result(0, box_u64_payload(handle)) as *mut u8
    }
}

/// Close a SQLite database connection.
///
/// # Signature
///
/// `mesh_sqlite_close(conn_handle: u64)`
///
/// Recovers the Box<SqliteConn> from the handle, calls sqlite3_close,
/// and lets Box::drop free the Rust memory.
#[no_mangle]
pub extern "C" fn mesh_sqlite_close(conn_handle: u64) {
    unsafe {
        let conn = Box::from_raw(conn_handle as *mut SqliteConn);
        sqlite3_close(conn.db);
        // Box drops, freeing Rust memory
    }
}

/// Execute a write SQL statement (INSERT, UPDATE, DELETE, CREATE TABLE, etc.).
///
/// # Signature
///
/// `mesh_sqlite_execute(conn_handle: u64, sql: *const MeshString, params: *mut u8)
///     -> *mut u8 (MeshResult<Int, String>)`
///
/// Parameters are bound as text via sqlite3_bind_text. Returns the number
/// of rows affected (via sqlite3_changes) on success.
#[no_mangle]
pub extern "C" fn mesh_sqlite_execute(
    conn_handle: u64,
    sql: *const MeshString,
    params: *mut u8,
) -> *mut u8 {
    unsafe {
        let conn = &*(conn_handle as *const SqliteConn);
        let sql_str = mesh_str_to_rust(sql);
        let sql_cstr = match CString::new(sql_str) {
            Ok(c) => c,
            Err(_) => return err_result("SQL contains null byte"),
        };

        let mut stmt: *mut sqlite3_stmt = std::ptr::null_mut();
        let rc = sqlite3_prepare_v2(
            conn.db,
            sql_cstr.as_ptr(),
            -1,
            &mut stmt,
            std::ptr::null_mut(),
        );
        if rc != SQLITE_OK {
            return sqlite_err_result(conn.db);
        }

        let _guard = StmtGuard { stmt };

        // Bind parameters
        if let Err(e) = bind_params(conn.db, stmt, params) {
            return e;
        }

        // Execute
        let step_rc = sqlite3_step(stmt);
        if step_rc != SQLITE_DONE && step_rc != SQLITE_ROW {
            return sqlite_err_result(conn.db);
        }

        let changes = sqlite3_changes(conn.db) as i64;
        alloc_result(0, box_i64_payload(changes)) as *mut u8
    }
}

/// Execute a read SQL statement (SELECT) and return rows.
///
/// # Signature
///
/// `mesh_sqlite_query(conn_handle: u64, sql: *const MeshString, params: *mut u8)
///     -> *mut u8 (MeshResult<List<Map<String, String>>, String>)`
///
/// Each row is a Map<String, String> where keys are column names and values
/// are the text representation of column values. NULL columns become empty
/// strings.
#[no_mangle]
pub extern "C" fn mesh_sqlite_query(
    conn_handle: u64,
    sql: *const MeshString,
    params: *mut u8,
) -> *mut u8 {
    unsafe {
        let conn = &*(conn_handle as *const SqliteConn);
        let sql_str = mesh_str_to_rust(sql);
        let sql_cstr = match CString::new(sql_str) {
            Ok(c) => c,
            Err(_) => return err_result("SQL contains null byte"),
        };

        let mut stmt: *mut sqlite3_stmt = std::ptr::null_mut();
        let rc = sqlite3_prepare_v2(
            conn.db,
            sql_cstr.as_ptr(),
            -1,
            &mut stmt,
            std::ptr::null_mut(),
        );
        if rc != SQLITE_OK {
            return sqlite_err_result(conn.db);
        }

        let _guard = StmtGuard { stmt };

        // Bind parameters
        if let Err(e) = bind_params(conn.db, stmt, params) {
            return e;
        }

        // Get column info
        let col_count = sqlite3_column_count(stmt) as usize;
        let mut col_names: Vec<String> = Vec::with_capacity(col_count);
        for i in 0..col_count {
            let name_ptr = sqlite3_column_name(stmt, i as c_int);
            if name_ptr.is_null() {
                col_names.push(format!("column{}", i));
            } else {
                let name = CStr::from_ptr(name_ptr).to_string_lossy().into_owned();
                col_names.push(name);
            }
        }

        // Iterate rows
        let mut result_list = mesh_list_new();

        loop {
            let step_rc = sqlite3_step(stmt);
            if step_rc == SQLITE_DONE {
                break;
            }
            if step_rc != SQLITE_ROW {
                return sqlite_err_result(conn.db);
            }

            // Create a string-keyed map for this row (key_type = 1 = string)
            let mut row_map = mesh_map_new_typed(1);

            for col in 0..col_count {
                let col_type = sqlite3_column_type(stmt, col as c_int);
                let value_str = if col_type == SQLITE_NULL {
                    String::new()
                } else {
                    let text_ptr = sqlite3_column_text(stmt, col as c_int);
                    if text_ptr.is_null() {
                        String::new()
                    } else {
                        CStr::from_ptr(text_ptr as *const c_char)
                            .to_string_lossy()
                            .into_owned()
                    }
                };

                let key_mesh = rust_str_to_mesh(&col_names[col]);
                let val_mesh = rust_str_to_mesh(&value_str);
                row_map = mesh_map_put(row_map, key_mesh as u64, val_mesh as u64);
            }

            result_list = mesh_list_append(result_list, row_map as u64);
        }

        alloc_result(0, result_list) as *mut u8
    }
}

// ── Transaction Management ──────────────────────────────────────────────

/// Execute a bare SQL command (BEGIN/COMMIT/ROLLBACK) on a SQLite connection.
/// Returns a MeshResult: Ok(null) on success, Err(message) on failure.
fn sqlite_simple_exec(conn: &SqliteConn, sql: &str) -> *mut u8 {
    let sql_cstr = match CString::new(sql) {
        Ok(c) => c,
        Err(_) => return err_result("SQL contains null byte"),
    };
    unsafe {
        let rc = sqlite3_exec(
            conn.db,
            sql_cstr.as_ptr(),
            None,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        if rc != SQLITE_OK {
            sqlite_err_result(conn.db)
        } else {
            alloc_result(0, std::ptr::null_mut()) as *mut u8
        }
    }
}

/// Begin a SQLite transaction.
///
/// # Signature
///
/// `mesh_sqlite_begin(conn_handle: u64) -> *mut u8 (MeshResult<Unit, String>)`
///
/// Sends `BEGIN` and returns Ok(()) or Err(error_message).
#[no_mangle]
pub extern "C" fn mesh_sqlite_begin(conn_handle: u64) -> *mut u8 {
    let conn = unsafe { &*(conn_handle as *const SqliteConn) };
    sqlite_simple_exec(conn, "BEGIN")
}

/// Commit a SQLite transaction.
///
/// # Signature
///
/// `mesh_sqlite_commit(conn_handle: u64) -> *mut u8 (MeshResult<Unit, String>)`
///
/// Sends `COMMIT` and returns Ok(()) or Err(error_message).
#[no_mangle]
pub extern "C" fn mesh_sqlite_commit(conn_handle: u64) -> *mut u8 {
    let conn = unsafe { &*(conn_handle as *const SqliteConn) };
    sqlite_simple_exec(conn, "COMMIT")
}

/// Rollback a SQLite transaction.
///
/// # Signature
///
/// `mesh_sqlite_rollback(conn_handle: u64) -> *mut u8 (MeshResult<Unit, String>)`
///
/// Sends `ROLLBACK` and returns Ok(()) or Err(error_message).
#[no_mangle]
pub extern "C" fn mesh_sqlite_rollback(conn_handle: u64) -> *mut u8 {
    let conn = unsafe { &*(conn_handle as *const SqliteConn) };
    sqlite_simple_exec(conn, "ROLLBACK")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gc::mesh_rt_init;
    use crate::io::MeshResult;

    /// Helper to create a MeshString from a byte literal.
    fn mk_str(s: &[u8]) -> *mut MeshString {
        mesh_string_new(s.as_ptr(), s.len() as u64)
    }

    #[test]
    fn test_open_close() {
        mesh_rt_init();

        // Open an in-memory database
        let path = mk_str(b":memory:");
        let result = mesh_sqlite_open(path);
        assert!(!result.is_null());

        let r = unsafe { &*(result as *const MeshResult) };
        assert_eq!(r.tag, 0, "open should succeed");

        let handle = unsafe { unbox_u64_payload(r.value) };
        assert_ne!(handle, 0, "handle should be non-zero");

        // Close it
        mesh_sqlite_close(handle);
    }

    #[test]
    fn test_execute_create_table() {
        mesh_rt_init();

        // Open
        let path = mk_str(b":memory:");
        let result = mesh_sqlite_open(path);
        let r = unsafe { &*(result as *const MeshResult) };
        assert_eq!(r.tag, 0);
        let handle = unsafe { unbox_u64_payload(r.value) };

        // Create table
        let sql = mk_str(b"CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT)");
        let empty_params = mesh_list_new();
        let exec_result = mesh_sqlite_execute(handle, sql, empty_params);
        let er = unsafe { &*(exec_result as *const MeshResult) };
        assert_eq!(er.tag, 0, "CREATE TABLE should succeed");

        mesh_sqlite_close(handle);
    }

    #[test]
    fn test_insert_and_query() {
        mesh_rt_init();

        // Open
        let path = mk_str(b":memory:");
        let result = mesh_sqlite_open(path);
        let r = unsafe { &*(result as *const MeshResult) };
        assert_eq!(r.tag, 0);
        let handle = unsafe { unbox_u64_payload(r.value) };

        // Create table
        let sql = mk_str(b"CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, age TEXT)");
        let empty_params = mesh_list_new();
        let exec_result = mesh_sqlite_execute(handle, sql, empty_params);
        let er = unsafe { &*(exec_result as *const MeshResult) };
        assert_eq!(er.tag, 0);

        // Insert a row with params
        let insert_sql = mk_str(b"INSERT INTO users (name, age) VALUES (?, ?)");
        let mut params = mesh_list_new();
        let name_val = mk_str(b"Alice");
        let age_val = mk_str(b"30");
        params = mesh_list_append(params, name_val as u64);
        params = mesh_list_append(params, age_val as u64);

        let insert_result = mesh_sqlite_execute(handle, insert_sql, params);
        let ir = unsafe { &*(insert_result as *const MeshResult) };
        assert_eq!(ir.tag, 0, "INSERT should succeed");
        assert_eq!(
            unsafe { unbox_i64_payload(ir.value) },
            1,
            "should affect 1 row"
        );

        // Query
        let query_sql = mk_str(b"SELECT name, age FROM users");
        let empty_params2 = mesh_list_new();
        let query_result = mesh_sqlite_query(handle, query_sql, empty_params2);
        let qr = unsafe { &*(query_result as *const MeshResult) };
        assert_eq!(qr.tag, 0, "SELECT should succeed");

        // The result is a MeshList with 1 row
        let list_ptr = qr.value;
        let list_len = unsafe { *(list_ptr as *const u64) };
        assert_eq!(list_len, 1, "should have 1 row");

        mesh_sqlite_close(handle);
    }
}
