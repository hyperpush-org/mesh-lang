//! File I/O runtime functions for the Mesh standard library.
//!
//! Provides file read, write, append, exists, and delete operations.
//! All fallible operations return MeshResult (tag 0 = Ok, tag 1 = Err).

use std::fs::{self, OpenOptions};
use std::io::Write;

use crate::gc::mesh_gc_alloc_actor;
use crate::io::MeshResult;
use crate::string::{mesh_string_new, MeshString};

/// Allocate a MeshResult on the GC heap.
fn alloc_result(tag: u8, value: *mut u8) -> *mut MeshResult {
    unsafe {
        let ptr = mesh_gc_alloc_actor(
            std::mem::size_of::<MeshResult>() as u64,
            std::mem::align_of::<MeshResult>() as u64,
        ) as *mut MeshResult;
        (*ptr).tag = tag;
        (*ptr).value = value;
        ptr
    }
}

/// Helper to create an Err result with a string message.
fn err_result(msg: &str) -> *mut MeshResult {
    let s = mesh_string_new(msg.as_ptr(), msg.len() as u64);
    alloc_result(1, s as *mut u8)
}

/// Read the entire contents of a file as a UTF-8 string.
///
/// Returns MeshResult:
/// - tag 0 (Ok): value = pointer to MeshString containing file contents
/// - tag 1 (Err): value = pointer to MeshString containing error message
#[no_mangle]
pub extern "C" fn mesh_file_read(path: *const MeshString) -> *mut MeshResult {
    unsafe {
        let path_str = (*path).as_str();
        match fs::read_to_string(path_str) {
            Ok(contents) => {
                let s = mesh_string_new(contents.as_ptr(), contents.len() as u64);
                alloc_result(0, s as *mut u8)
            }
            Err(e) => err_result(&e.to_string()),
        }
    }
}

/// Write content to a file, creating or overwriting it.
///
/// Returns MeshResult:
/// - tag 0 (Ok): value = null (Unit payload)
/// - tag 1 (Err): value = pointer to MeshString containing error message
#[no_mangle]
pub extern "C" fn mesh_file_write(
    path: *const MeshString,
    content: *const MeshString,
) -> *mut MeshResult {
    unsafe {
        let path_str = (*path).as_str();
        let content_str = (*content).as_str();
        match fs::write(path_str, content_str) {
            Ok(()) => alloc_result(0, std::ptr::null_mut()),
            Err(e) => err_result(&e.to_string()),
        }
    }
}

/// Append content to a file, creating it if it doesn't exist.
///
/// Returns MeshResult:
/// - tag 0 (Ok): value = null (Unit payload)
/// - tag 1 (Err): value = pointer to MeshString containing error message
#[no_mangle]
pub extern "C" fn mesh_file_append(
    path: *const MeshString,
    content: *const MeshString,
) -> *mut MeshResult {
    unsafe {
        let path_str = (*path).as_str();
        let content_str = (*content).as_str();
        match OpenOptions::new().append(true).create(true).open(path_str) {
            Ok(mut file) => match file.write_all(content_str.as_bytes()) {
                Ok(()) => alloc_result(0, std::ptr::null_mut()),
                Err(e) => err_result(&e.to_string()),
            },
            Err(e) => err_result(&e.to_string()),
        }
    }
}

/// Check if a file exists at the given path.
///
/// Returns 1 if the file exists, 0 otherwise.
#[no_mangle]
pub extern "C" fn mesh_file_exists(path: *const MeshString) -> i8 {
    unsafe {
        let path_str = (*path).as_str();
        if std::path::Path::new(path_str).exists() {
            1
        } else {
            0
        }
    }
}

/// Delete a file at the given path.
///
/// Returns MeshResult:
/// - tag 0 (Ok): value = null (Unit payload)
/// - tag 1 (Err): value = pointer to MeshString containing error message
#[no_mangle]
pub extern "C" fn mesh_file_delete(path: *const MeshString) -> *mut MeshResult {
    unsafe {
        let path_str = (*path).as_str();
        match fs::remove_file(path_str) {
            Ok(()) => alloc_result(0, std::ptr::null_mut()),
            Err(e) => err_result(&e.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gc::mesh_rt_init;

    fn make_string(s: &str) -> *const MeshString {
        mesh_string_new(s.as_ptr(), s.len() as u64)
    }

    #[test]
    fn test_file_write_and_read() {
        mesh_rt_init();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.txt");
        let path_str = path.to_str().unwrap();

        let path_mesh = make_string(path_str);
        let content = make_string("Hello, Mesh!");

        // Write
        let write_result = mesh_file_write(path_mesh, content);
        unsafe {
            assert_eq!((*write_result).tag, 0, "write should succeed");
        }

        // Read back
        let read_result = mesh_file_read(path_mesh);
        unsafe {
            assert_eq!((*read_result).tag, 0, "read should succeed");
            let value = (*read_result).value as *const MeshString;
            assert_eq!((*value).as_str(), "Hello, Mesh!");
        }
    }

    #[test]
    fn test_file_read_nonexistent() {
        mesh_rt_init();
        let path_mesh = make_string("/tmp/mesh_nonexistent_file_12345.txt");

        let result = mesh_file_read(path_mesh);
        unsafe {
            assert_eq!(
                (*result).tag,
                1,
                "reading nonexistent file should return Err"
            );
            let value = (*result).value as *const MeshString;
            assert!(!value.is_null());
            let msg = (*value).as_str();
            assert!(msg.contains("No such file"), "error msg: {}", msg);
        }
    }

    #[test]
    fn test_file_append() {
        mesh_rt_init();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("append_test.txt");
        let path_str = path.to_str().unwrap();

        let path_mesh = make_string(path_str);
        let content1 = make_string("Hello");
        let content2 = make_string(", Mesh!");

        // Append twice
        let r1 = mesh_file_append(path_mesh, content1);
        unsafe {
            assert_eq!((*r1).tag, 0);
        }

        let r2 = mesh_file_append(path_mesh, content2);
        unsafe {
            assert_eq!((*r2).tag, 0);
        }

        // Read back
        let read_result = mesh_file_read(path_mesh);
        unsafe {
            assert_eq!((*read_result).tag, 0);
            let value = (*read_result).value as *const MeshString;
            assert_eq!((*value).as_str(), "Hello, Mesh!");
        }
    }

    #[test]
    fn test_file_exists() {
        mesh_rt_init();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("exists_test.txt");
        let path_str = path.to_str().unwrap();

        let path_mesh = make_string(path_str);

        // Should not exist yet
        assert_eq!(mesh_file_exists(path_mesh), 0);

        // Create the file
        let content = make_string("test");
        mesh_file_write(path_mesh, content);

        // Should now exist
        assert_eq!(mesh_file_exists(path_mesh), 1);
    }

    #[test]
    fn test_file_delete() {
        mesh_rt_init();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("delete_test.txt");
        let path_str = path.to_str().unwrap();

        let path_mesh = make_string(path_str);
        let content = make_string("to be deleted");

        // Write
        mesh_file_write(path_mesh, content);
        assert_eq!(mesh_file_exists(path_mesh), 1);

        // Delete
        let del_result = mesh_file_delete(path_mesh);
        unsafe {
            assert_eq!((*del_result).tag, 0, "delete should succeed");
        }

        // Should not exist now
        assert_eq!(mesh_file_exists(path_mesh), 0);
    }

    #[test]
    fn test_file_delete_nonexistent() {
        mesh_rt_init();
        let path_mesh = make_string("/tmp/mesh_nonexistent_delete_12345.txt");

        let result = mesh_file_delete(path_mesh);
        unsafe {
            assert_eq!(
                (*result).tag,
                1,
                "deleting nonexistent file should return Err"
            );
        }
    }

    #[test]
    fn test_file_full_cycle() {
        mesh_rt_init();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("cycle_test.txt");
        let path_str = path.to_str().unwrap();
        let path_mesh = make_string(path_str);

        // 1. File does not exist
        assert_eq!(mesh_file_exists(path_mesh), 0);

        // 2. Write
        let content = make_string("initial content");
        let r = mesh_file_write(path_mesh, content);
        unsafe {
            assert_eq!((*r).tag, 0);
        }

        // 3. Exists
        assert_eq!(mesh_file_exists(path_mesh), 1);

        // 4. Read
        let r = mesh_file_read(path_mesh);
        unsafe {
            assert_eq!((*r).tag, 0);
            let v = (*r).value as *const MeshString;
            assert_eq!((*v).as_str(), "initial content");
        }

        // 5. Append
        let more = make_string(" + appended");
        let r = mesh_file_append(path_mesh, more);
        unsafe {
            assert_eq!((*r).tag, 0);
        }

        // 6. Read again
        let r = mesh_file_read(path_mesh);
        unsafe {
            assert_eq!((*r).tag, 0);
            let v = (*r).value as *const MeshString;
            assert_eq!((*v).as_str(), "initial content + appended");
        }

        // 7. Delete
        let r = mesh_file_delete(path_mesh);
        unsafe {
            assert_eq!((*r).tag, 0);
        }

        // 8. No longer exists
        assert_eq!(mesh_file_exists(path_mesh), 0);
    }
}
