//! GC-managed Range for the Mesh runtime.
//!
//! A Range represents a half-open interval `[start, end)` of integers.
//! Layout: `{ i64 start, i64 end }` (16 bytes total).
//!
//! Ranges support conversion to List and higher-order operations (map, filter).

use crate::gc::mesh_gc_alloc_actor;

// ── Internal helpers ──────────────────────────────────────────────────

unsafe fn range_start(r: *const u8) -> i64 {
    *(r as *const i64)
}

unsafe fn range_end(r: *const u8) -> i64 {
    *((r as *const i64).add(1))
}

// ── Public API ────────────────────────────────────────────────────────

/// Create a new range `[start, end)`.
#[no_mangle]
pub extern "C" fn mesh_range_new(start: i64, end: i64) -> *mut u8 {
    unsafe {
        let p = mesh_gc_alloc_actor(16, 8);
        *(p as *mut i64) = start;
        *((p as *mut i64).add(1)) = end;
        p
    }
}

/// Convert a range to a List of integers.
#[no_mangle]
pub extern "C" fn mesh_range_to_list(range: *mut u8) -> *mut u8 {
    unsafe {
        let start = range_start(range);
        let end = range_end(range);
        let mut list = super::list::mesh_list_new();
        let mut i = start;
        while i < end {
            list = super::list::mesh_list_append(list, i as u64);
            i += 1;
        }
        list
    }
}

/// Apply a closure to each element of the range, returning a List.
#[no_mangle]
pub extern "C" fn mesh_range_map(range: *mut u8, fn_ptr: *mut u8, env_ptr: *mut u8) -> *mut u8 {
    type BareFn = unsafe extern "C" fn(u64) -> u64;
    type ClosureFn = unsafe extern "C" fn(*mut u8, u64) -> u64;

    unsafe {
        let start = range_start(range);
        let end = range_end(range);
        let mut list = super::list::mesh_list_new();

        if env_ptr.is_null() {
            let f: BareFn = std::mem::transmute(fn_ptr);
            let mut i = start;
            while i < end {
                let result = f(i as u64);
                list = super::list::mesh_list_append(list, result);
                i += 1;
            }
        } else {
            let f: ClosureFn = std::mem::transmute(fn_ptr);
            let mut i = start;
            while i < end {
                let result = f(env_ptr, i as u64);
                list = super::list::mesh_list_append(list, result);
                i += 1;
            }
        }
        list
    }
}

/// Filter elements of the range, returning a List of matching integers.
#[no_mangle]
pub extern "C" fn mesh_range_filter(range: *mut u8, fn_ptr: *mut u8, env_ptr: *mut u8) -> *mut u8 {
    type BareFn = unsafe extern "C" fn(u64) -> u64;
    type ClosureFn = unsafe extern "C" fn(*mut u8, u64) -> u64;

    unsafe {
        let start = range_start(range);
        let end = range_end(range);
        let mut list = super::list::mesh_list_new();

        if env_ptr.is_null() {
            let f: BareFn = std::mem::transmute(fn_ptr);
            let mut i = start;
            while i < end {
                if f(i as u64) != 0 {
                    list = super::list::mesh_list_append(list, i as u64);
                }
                i += 1;
            }
        } else {
            let f: ClosureFn = std::mem::transmute(fn_ptr);
            let mut i = start;
            while i < end {
                if f(env_ptr, i as u64) != 0 {
                    list = super::list::mesh_list_append(list, i as u64);
                }
                i += 1;
            }
        }
        list
    }
}

/// Return the number of elements in the range.
#[no_mangle]
pub extern "C" fn mesh_range_length(range: *mut u8) -> i64 {
    unsafe {
        let start = range_start(range);
        let end = range_end(range);
        (end - start).max(0)
    }
}

// ── Iterator handle ───────────────────────────────────────────────────

/// Internal iterator state for Range iteration.
#[repr(C)]
struct RangeIterator {
    tag: u8,
    current: i64,
    end: i64,
}

/// Create a new iterator handle for a range [start, end).
#[no_mangle]
pub extern "C" fn mesh_range_iter_new(start: i64, end: i64) -> *mut u8 {
    unsafe {
        let iter = mesh_gc_alloc_actor(
            std::mem::size_of::<RangeIterator>() as u64,
            std::mem::align_of::<RangeIterator>() as u64,
        ) as *mut RangeIterator;
        (*iter).tag = 3; // ITER_TAG_RANGE
        (*iter).current = start;
        (*iter).end = end;
        iter as *mut u8
    }
}

/// Advance the range iterator, returning Option (tag 0 = Some, tag 1 = None).
#[no_mangle]
pub extern "C" fn mesh_range_iter_next(iter_ptr: *mut u8) -> *mut u8 {
    unsafe {
        let iter = iter_ptr as *mut RangeIterator;
        if (*iter).current >= (*iter).end {
            crate::option::alloc_option(1, std::ptr::null_mut()) as *mut u8
        } else {
            let val = (*iter).current;
            (*iter).current += 1;
            crate::option::alloc_option(0, val as usize as *mut u8) as *mut u8
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::list;
    use crate::gc::mesh_rt_init;

    #[test]
    fn test_range_new() {
        mesh_rt_init();
        let r = mesh_range_new(1, 5);
        assert_eq!(mesh_range_length(r), 4);
    }

    #[test]
    fn test_range_to_list() {
        mesh_rt_init();
        let r = mesh_range_new(1, 4);
        let l = mesh_range_to_list(r);
        assert_eq!(list::mesh_list_length(l), 3);
        assert_eq!(list::mesh_list_get(l, 0), 1);
        assert_eq!(list::mesh_list_get(l, 1), 2);
        assert_eq!(list::mesh_list_get(l, 2), 3);
    }

    #[test]
    fn test_range_map() {
        mesh_rt_init();
        let r = mesh_range_new(1, 4);

        unsafe extern "C" fn double(x: u64) -> u64 {
            x * 2
        }

        let mapped = mesh_range_map(r, double as *mut u8, std::ptr::null_mut());
        assert_eq!(list::mesh_list_length(mapped), 3);
        assert_eq!(list::mesh_list_get(mapped, 0), 2);
        assert_eq!(list::mesh_list_get(mapped, 1), 4);
        assert_eq!(list::mesh_list_get(mapped, 2), 6);
    }

    #[test]
    fn test_range_filter() {
        mesh_rt_init();
        let r = mesh_range_new(1, 6);

        unsafe extern "C" fn is_even(x: u64) -> u64 {
            if x % 2 == 0 {
                1
            } else {
                0
            }
        }

        let filtered = mesh_range_filter(r, is_even as *mut u8, std::ptr::null_mut());
        assert_eq!(list::mesh_list_length(filtered), 2);
        assert_eq!(list::mesh_list_get(filtered, 0), 2);
        assert_eq!(list::mesh_list_get(filtered, 1), 4);
    }

    #[test]
    fn test_range_empty() {
        mesh_rt_init();
        let r = mesh_range_new(5, 5);
        assert_eq!(mesh_range_length(r), 0);
        let l = mesh_range_to_list(r);
        assert_eq!(list::mesh_list_length(l), 0);
    }

    #[test]
    fn test_range_inverted() {
        mesh_rt_init();
        let r = mesh_range_new(5, 3);
        assert_eq!(mesh_range_length(r), 0);
    }
}
