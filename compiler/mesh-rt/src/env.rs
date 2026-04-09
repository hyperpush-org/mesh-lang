//! Environment variable and CLI argument access for the Mesh standard library.
//!
//! Provides `Env.get(key)` and `Env.args()` for Mesh programs.

use crate::gc::mesh_gc_alloc_actor;
use crate::option::{alloc_option, MeshOption};
use crate::string::{mesh_string_new, MeshString};

/// Get an environment variable by key. Returns MeshOption:
/// - tag 0, value = MeshString if the variable exists (Some)
/// - tag 1, value = null if the variable does not exist (None)
#[no_mangle]
pub extern "C" fn mesh_env_get(key: *const MeshString) -> *mut MeshOption {
    unsafe {
        let key_str = (*key).as_str();
        match std::env::var(key_str) {
            Ok(val) => {
                let s = mesh_string_new(val.as_ptr(), val.len() as u64);
                alloc_option(0, s as *mut u8)
            }
            Err(_) => alloc_option(1, std::ptr::null_mut()),
        }
    }
}

/// Get an environment variable by key, returning a default string if not set.
///
/// Returns the env var value as a MeshString pointer if set, or the provided
/// default MeshString pointer if the variable is not set.
#[no_mangle]
pub extern "C" fn mesh_env_get_with_default(
    key: *const MeshString,
    default: *const MeshString,
) -> *mut MeshString {
    unsafe {
        let key_str = (*key).as_str();
        match std::env::var(key_str) {
            Ok(val) => mesh_string_new(val.as_ptr(), val.len() as u64),
            Err(_) => default as *mut MeshString,
        }
    }
}

/// Get an environment variable by key and parse it as i64, returning a default if not set or invalid.
///
/// Returns the parsed integer value if the env var is set and parses successfully,
/// or the provided default if the variable is not set or cannot be parsed as i64.
#[no_mangle]
pub extern "C" fn mesh_env_get_int(key: *const MeshString, default: i64) -> i64 {
    let key_str = unsafe { (*key).as_str() };
    match std::env::var(key_str).ok() {
        Some(val) => val.parse::<i64>().unwrap_or(default),
        None => default,
    }
}

/// Return CLI arguments as a packed array of MeshString pointers.
///
/// Layout: `[u64 count, *mut MeshString arg0, *mut MeshString arg1, ...]`
///
/// This temporary representation will be replaced by proper List<String>
/// in Plan 02 when the List type is implemented.
#[no_mangle]
pub extern "C" fn mesh_env_args() -> *mut u8 {
    let args: Vec<String> = std::env::args().collect();
    let count = args.len();
    // Layout: u64 count + count * pointer-sized entries
    let ptr_size = std::mem::size_of::<*mut MeshString>();
    let total_size = std::mem::size_of::<u64>() + count * ptr_size;

    unsafe {
        let buf = mesh_gc_alloc_actor(total_size as u64, 8);
        // Write count
        *(buf as *mut u64) = count as u64;
        // Write string pointers
        let ptrs = buf.add(std::mem::size_of::<u64>()) as *mut *mut MeshString;
        for (i, arg) in args.iter().enumerate() {
            let s = mesh_string_new(arg.as_ptr(), arg.len() as u64);
            *ptrs.add(i) = s;
        }
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gc::mesh_rt_init;
    use crate::string::MeshString;

    #[test]
    fn test_env_get_existing() {
        mesh_rt_init();
        // PATH is almost always set
        let key = mesh_string_new(b"PATH".as_ptr(), 4);
        let result = mesh_env_get(key);
        unsafe {
            assert_eq!((*result).tag, 0, "PATH should exist");
            let value = (*result).value as *const MeshString;
            assert!(!value.is_null());
            assert!((*value).as_str().len() > 0, "PATH should be non-empty");
        }
    }

    #[test]
    fn test_env_get_missing() {
        mesh_rt_init();
        let key = mesh_string_new(b"MESH_NONEXISTENT_VAR_12345".as_ptr(), 25);
        let result = mesh_env_get(key);
        unsafe {
            assert_eq!((*result).tag, 1, "missing var should return None");
        }
    }

    #[test]
    fn test_env_get_with_default_missing() {
        mesh_rt_init();
        let key = mesh_string_new(b"MESH_NONEXISTENT_VAR_99999".as_ptr(), 25);
        let default = mesh_string_new(b"fallback".as_ptr(), 8);
        let result = mesh_env_get_with_default(key, default);
        unsafe {
            assert_eq!((*result).as_str(), "fallback");
        }
    }

    #[test]
    fn test_env_get_int_missing() {
        mesh_rt_init();
        let key = mesh_string_new(b"MESH_INT_NONEXISTENT_99999".as_ptr(), 25);
        let result = mesh_env_get_int(key, 8080);
        assert_eq!(result, 8080);
    }

    #[test]
    fn test_env_get_int_non_numeric() {
        mesh_rt_init();
        std::env::set_var("MESH_INT_NONNUMERIC_TEST", "not-a-number");
        let key = mesh_string_new(b"MESH_INT_NONNUMERIC_TEST".as_ptr(), 24);
        let result = mesh_env_get_int(key, 42);
        assert_eq!(result, 42);
        std::env::remove_var("MESH_INT_NONNUMERIC_TEST");
    }

    #[test]
    fn test_env_args() {
        mesh_rt_init();
        let buf = mesh_env_args();
        unsafe {
            let count = *(buf as *const u64);
            // There should be at least 1 arg (the test binary itself)
            assert!(count >= 1, "expected at least 1 arg, got {}", count);
        }
    }
}
