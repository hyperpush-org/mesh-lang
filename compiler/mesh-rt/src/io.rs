//! Console I/O runtime functions for the Mesh standard library.
//!
//! Provides stdin reading and stderr output. Functions match the Mesh
//! module `IO` with `read_line` and `eprintln`.

use crate::gc::mesh_gc_alloc_actor;
use crate::string::{mesh_string_new, MeshString};

/// Tagged result value for Mesh's Result<T, E> representation.
///
/// Layout matches the codegen layout for sum types:
/// - tag 0 = Ok (first variant)
/// - tag 1 = Err (second variant)
///
/// The value pointer points to the payload (a MeshString in both cases).
#[repr(C)]
pub struct MeshResult {
    pub tag: u8,
    pub value: *mut u8,
}

/// Allocate a MeshResult on the GC heap.
pub(crate) fn alloc_result(tag: u8, value: *mut u8) -> *mut MeshResult {
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

/// Allocate a MeshResult on the GC heap with the given tag and value.
/// Tag 0 = Ok, tag 1 = Err.
#[no_mangle]
pub extern "C" fn mesh_alloc_result(tag: i64, value: *mut u8) -> *mut u8 {
    alloc_result(tag as u8, value) as *mut u8
}

/// Check if a MeshResult is Ok (tag == 0). Returns 1 for Ok, 0 for Err.
#[no_mangle]
pub extern "C" fn mesh_result_is_ok(result: *mut u8) -> i64 {
    let r = result as *const MeshResult;
    unsafe {
        if (*r).tag == 0 {
            1
        } else {
            0
        }
    }
}

/// Extract the value from a MeshResult (Ok or Err payload).
#[no_mangle]
pub extern "C" fn mesh_result_unwrap(result: *mut u8) -> *mut u8 {
    let r = result as *const MeshResult;
    unsafe { (*r).value }
}

/// Read a line from stdin. Returns a MeshResult (tag 0 = Ok with string,
/// tag 1 = Err with error message string).
///
/// The trailing newline is stripped from the result.
#[no_mangle]
pub extern "C" fn mesh_io_read_line() -> *mut MeshResult {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            // Strip trailing newline
            if input.ends_with('\n') {
                input.pop();
                if input.ends_with('\r') {
                    input.pop();
                }
            }
            let s = mesh_string_new(input.as_ptr(), input.len() as u64);
            alloc_result(0, s as *mut u8)
        }
        Err(e) => {
            let msg = e.to_string();
            let s = mesh_string_new(msg.as_ptr(), msg.len() as u64);
            alloc_result(1, s as *mut u8)
        }
    }
}

/// Print a Mesh string to stderr with a trailing newline.
#[no_mangle]
pub extern "C" fn mesh_io_eprintln(s: *const MeshString) {
    unsafe {
        let text = (*s).as_str();
        eprintln!("{}", text);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gc::mesh_rt_init;

    #[test]
    fn test_alloc_result_ok() {
        mesh_rt_init();
        let s = mesh_string_new(b"hello".as_ptr(), 5);
        let result = alloc_result(0, s as *mut u8);
        unsafe {
            assert_eq!((*result).tag, 0);
            let value = (*result).value as *const MeshString;
            assert_eq!((*value).as_str(), "hello");
        }
    }

    #[test]
    fn test_alloc_result_err() {
        mesh_rt_init();
        let s = mesh_string_new(b"error".as_ptr(), 5);
        let result = alloc_result(1, s as *mut u8);
        unsafe {
            assert_eq!((*result).tag, 1);
            let value = (*result).value as *const MeshString;
            assert_eq!((*value).as_str(), "error");
        }
    }

    #[test]
    fn test_eprintln_does_not_crash() {
        mesh_rt_init();
        let s = mesh_string_new(b"test error".as_ptr(), 10);
        // Just verify it doesn't panic/crash
        mesh_io_eprintln(s);
    }
}
