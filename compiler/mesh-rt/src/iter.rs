//! Lazy iterator adapter handles and terminal operations for the Mesh runtime.
//!
//! This module provides:
//! - Type tag constants for generic iterator dispatch
//! - Generic next dispatch (`mesh_iter_generic_next`)
//! - Combinator adapter structs (MapAdapter, FilterAdapter, TakeAdapter,
//!   SkipAdapter, EnumerateAdapter, ZipAdapter) with `_new` and `_next` functions
//! - Terminal operations (count, sum, any, all, find, reduce)
//!
//! All combinator adapters are lazy -- they do not allocate intermediate
//! collections. Each adapter's `_next` function delegates to its source
//! iterator via `mesh_iter_generic_next` and applies the transformation
//! on-the-fly.

use crate::collections::list::alloc_pair;
use crate::collections::list::mesh_list_from_array;
use crate::collections::list::mesh_list_iter_next;
use crate::collections::map::mesh_map_iter_next;
use crate::collections::map::{mesh_map_new, mesh_map_put};
use crate::collections::range::mesh_range_iter_next;
use crate::collections::set::mesh_set_iter_next;
use crate::collections::set::{mesh_set_add, mesh_set_new};
use crate::gc::mesh_gc_alloc_actor;
use crate::option::{alloc_option, MeshOption};
use crate::string::{mesh_string_concat, mesh_string_new, MeshString};

// ── Type tag constants ──────────────────────────────────────────────────

pub const ITER_TAG_LIST: u8 = 0;
pub const ITER_TAG_MAP: u8 = 1;
pub const ITER_TAG_SET: u8 = 2;
pub const ITER_TAG_RANGE: u8 = 3;
pub const ITER_TAG_MAP_ADAPTER: u8 = 10;
pub const ITER_TAG_FILTER_ADAPTER: u8 = 11;
pub const ITER_TAG_TAKE_ADAPTER: u8 = 12;
pub const ITER_TAG_SKIP_ADAPTER: u8 = 13;
pub const ITER_TAG_ENUMERATE_ADAPTER: u8 = 14;
pub const ITER_TAG_ZIP_ADAPTER: u8 = 15;

// ── Generic next dispatch ───────────────────────────────────────────────

/// Generic next() dispatch. Reads the type tag (first byte of the iterator
/// handle) and delegates to the correct `_next` function.
#[no_mangle]
pub extern "C" fn mesh_iter_generic_next(iter: *mut u8) -> *mut u8 {
    unsafe {
        let tag = *iter; // First byte is the type tag
        match tag {
            ITER_TAG_LIST => mesh_list_iter_next(iter),
            ITER_TAG_MAP => mesh_map_iter_next(iter),
            ITER_TAG_SET => mesh_set_iter_next(iter),
            ITER_TAG_RANGE => mesh_range_iter_next(iter),
            ITER_TAG_MAP_ADAPTER => mesh_iter_map_next(iter),
            ITER_TAG_FILTER_ADAPTER => mesh_iter_filter_next(iter),
            ITER_TAG_TAKE_ADAPTER => mesh_iter_take_next(iter),
            ITER_TAG_SKIP_ADAPTER => mesh_iter_skip_next(iter),
            ITER_TAG_ENUMERATE_ADAPTER => mesh_iter_enumerate_next(iter),
            ITER_TAG_ZIP_ADAPTER => mesh_iter_zip_next(iter),
            _ => alloc_option(1, std::ptr::null_mut()) as *mut u8, // Unknown -> None
        }
    }
}

// ── Combinator Adapter Structs ──────────────────────────────────────────

// Closure calling type aliases (proven from list.rs)
type BareFn = unsafe extern "C" fn(u64) -> u64;
type ClosureFn = unsafe extern "C" fn(*mut u8, u64) -> u64;
type BareFn2 = unsafe extern "C" fn(u64, u64) -> u64;
type ClosureFn2 = unsafe extern "C" fn(*mut u8, u64, u64) -> u64;

// ── MapAdapter (tag=10) ─────────────────────────────────────────────────

/// Adapter state for Iter.map(iter, fn).
#[repr(C)]
struct MapAdapter {
    tag: u8,
    source: *mut u8,
    fn_ptr: *mut u8,
    env_ptr: *mut u8,
}

/// Create a lazy map adapter: Iter.map(source, fn_ptr, env_ptr).
#[no_mangle]
pub extern "C" fn mesh_iter_map(source: *mut u8, fn_ptr: *mut u8, env_ptr: *mut u8) -> *mut u8 {
    unsafe {
        let adapter = mesh_gc_alloc_actor(
            std::mem::size_of::<MapAdapter>() as u64,
            std::mem::align_of::<MapAdapter>() as u64,
        ) as *mut MapAdapter;
        (*adapter).tag = ITER_TAG_MAP_ADAPTER;
        (*adapter).source = source;
        (*adapter).fn_ptr = fn_ptr;
        (*adapter).env_ptr = env_ptr;
        adapter as *mut u8
    }
}

/// Advance the map adapter: call source next(), apply fn, return mapped value.
#[no_mangle]
pub extern "C" fn mesh_iter_map_next(adapter_ptr: *mut u8) -> *mut u8 {
    unsafe {
        let adapter = adapter_ptr as *mut MapAdapter;
        let option = mesh_iter_generic_next((*adapter).source);
        let option_ref = option as *mut MeshOption;
        if (*option_ref).tag == 1 {
            return option; // None -- propagate
        }
        let elem = (*option_ref).value as u64;
        let mapped = if (*adapter).env_ptr.is_null() {
            let f: BareFn = std::mem::transmute((*adapter).fn_ptr);
            f(elem)
        } else {
            let f: ClosureFn = std::mem::transmute((*adapter).fn_ptr);
            f((*adapter).env_ptr, elem)
        };
        alloc_option(0, mapped as *mut u8) as *mut u8
    }
}

// ── FilterAdapter (tag=11) ──────────────────────────────────────────────

/// Adapter state for Iter.filter(iter, fn).
#[repr(C)]
struct FilterAdapter {
    tag: u8,
    source: *mut u8,
    fn_ptr: *mut u8,
    env_ptr: *mut u8,
}

/// Create a lazy filter adapter: Iter.filter(source, fn_ptr, env_ptr).
#[no_mangle]
pub extern "C" fn mesh_iter_filter(source: *mut u8, fn_ptr: *mut u8, env_ptr: *mut u8) -> *mut u8 {
    unsafe {
        let adapter = mesh_gc_alloc_actor(
            std::mem::size_of::<FilterAdapter>() as u64,
            std::mem::align_of::<FilterAdapter>() as u64,
        ) as *mut FilterAdapter;
        (*adapter).tag = ITER_TAG_FILTER_ADAPTER;
        (*adapter).source = source;
        (*adapter).fn_ptr = fn_ptr;
        (*adapter).env_ptr = env_ptr;
        adapter as *mut u8
    }
}

/// Advance the filter adapter: loop calling source next() until predicate
/// passes or source is exhausted.
#[no_mangle]
pub extern "C" fn mesh_iter_filter_next(adapter_ptr: *mut u8) -> *mut u8 {
    unsafe {
        let adapter = adapter_ptr as *mut FilterAdapter;
        loop {
            let option = mesh_iter_generic_next((*adapter).source);
            let option_ref = option as *mut MeshOption;
            if (*option_ref).tag == 1 {
                return option; // None -- source exhausted
            }
            let elem = (*option_ref).value as u64;
            let passes = if (*adapter).env_ptr.is_null() {
                let f: BareFn = std::mem::transmute((*adapter).fn_ptr);
                f(elem)
            } else {
                let f: ClosureFn = std::mem::transmute((*adapter).fn_ptr);
                f((*adapter).env_ptr, elem)
            };
            if passes != 0 {
                return option; // Predicate passed -- return this element
            }
            // Predicate failed -- continue loop
        }
    }
}

// ── TakeAdapter (tag=12) ────────────────────────────────────────────────

/// Adapter state for Iter.take(iter, n).
#[repr(C)]
struct TakeAdapter {
    tag: u8,
    source: *mut u8,
    remaining: i64,
}

/// Create a lazy take adapter: Iter.take(source, n).
#[no_mangle]
pub extern "C" fn mesh_iter_take(source: *mut u8, n: i64) -> *mut u8 {
    unsafe {
        let adapter = mesh_gc_alloc_actor(
            std::mem::size_of::<TakeAdapter>() as u64,
            std::mem::align_of::<TakeAdapter>() as u64,
        ) as *mut TakeAdapter;
        (*adapter).tag = ITER_TAG_TAKE_ADAPTER;
        (*adapter).source = source;
        (*adapter).remaining = n;
        adapter as *mut u8
    }
}

/// Advance the take adapter: return None if remaining <= 0, else delegate
/// to source and decrement remaining.
#[no_mangle]
pub extern "C" fn mesh_iter_take_next(adapter_ptr: *mut u8) -> *mut u8 {
    unsafe {
        let adapter = adapter_ptr as *mut TakeAdapter;
        if (*adapter).remaining <= 0 {
            return alloc_option(1, std::ptr::null_mut()) as *mut u8; // None
        }
        (*adapter).remaining -= 1;
        mesh_iter_generic_next((*adapter).source)
    }
}

// ── SkipAdapter (tag=13) ────────────────────────────────────────────────

/// Adapter state for Iter.skip(iter, n).
#[repr(C)]
struct SkipAdapter {
    tag: u8,
    source: *mut u8,
    to_skip: i64,
    skipped: u8, // 0 = not yet skipped, 1 = done skipping
}

/// Create a lazy skip adapter: Iter.skip(source, n).
#[no_mangle]
pub extern "C" fn mesh_iter_skip(source: *mut u8, n: i64) -> *mut u8 {
    unsafe {
        let adapter = mesh_gc_alloc_actor(
            std::mem::size_of::<SkipAdapter>() as u64,
            std::mem::align_of::<SkipAdapter>() as u64,
        ) as *mut SkipAdapter;
        (*adapter).tag = ITER_TAG_SKIP_ADAPTER;
        (*adapter).source = source;
        (*adapter).to_skip = n;
        (*adapter).skipped = 0;
        adapter as *mut u8
    }
}

/// Advance the skip adapter: on first call, skip `n` elements from source,
/// then delegate to source.
#[no_mangle]
pub extern "C" fn mesh_iter_skip_next(adapter_ptr: *mut u8) -> *mut u8 {
    unsafe {
        let adapter = adapter_ptr as *mut SkipAdapter;
        if (*adapter).skipped == 0 {
            // Skip the first `to_skip` elements
            for _ in 0..(*adapter).to_skip {
                let option = mesh_iter_generic_next((*adapter).source);
                let option_ref = option as *mut MeshOption;
                if (*option_ref).tag == 1 {
                    (*adapter).skipped = 1;
                    return option; // Source exhausted during skip
                }
            }
            (*adapter).skipped = 1;
        }
        mesh_iter_generic_next((*adapter).source)
    }
}

// ── EnumerateAdapter (tag=14) ───────────────────────────────────────────

/// Adapter state for Iter.enumerate(iter).
#[repr(C)]
struct EnumerateAdapter {
    tag: u8,
    source: *mut u8,
    index: i64,
}

/// Create a lazy enumerate adapter: Iter.enumerate(source).
#[no_mangle]
pub extern "C" fn mesh_iter_enumerate(source: *mut u8) -> *mut u8 {
    unsafe {
        let adapter = mesh_gc_alloc_actor(
            std::mem::size_of::<EnumerateAdapter>() as u64,
            std::mem::align_of::<EnumerateAdapter>() as u64,
        ) as *mut EnumerateAdapter;
        (*adapter).tag = ITER_TAG_ENUMERATE_ADAPTER;
        (*adapter).source = source;
        (*adapter).index = 0;
        adapter as *mut u8
    }
}

/// Advance the enumerate adapter: call source next(), if Some wrap in
/// (index, value) pair, increment index.
#[no_mangle]
pub extern "C" fn mesh_iter_enumerate_next(adapter_ptr: *mut u8) -> *mut u8 {
    unsafe {
        let adapter = adapter_ptr as *mut EnumerateAdapter;
        let option = mesh_iter_generic_next((*adapter).source);
        let option_ref = option as *mut MeshOption;
        if (*option_ref).tag == 1 {
            return option; // None
        }
        let elem = (*option_ref).value as u64;
        let idx = (*adapter).index as u64;
        (*adapter).index += 1;
        let pair = alloc_pair(idx, elem);
        alloc_option(0, pair as *mut u8) as *mut u8
    }
}

// ── ZipAdapter (tag=15) ─────────────────────────────────────────────────

/// Adapter state for Iter.zip(iter_a, iter_b).
#[repr(C)]
struct ZipAdapter {
    tag: u8,
    source_a: *mut u8,
    source_b: *mut u8,
}

/// Create a lazy zip adapter: Iter.zip(source_a, source_b).
#[no_mangle]
pub extern "C" fn mesh_iter_zip(source_a: *mut u8, source_b: *mut u8) -> *mut u8 {
    unsafe {
        let adapter = mesh_gc_alloc_actor(
            std::mem::size_of::<ZipAdapter>() as u64,
            std::mem::align_of::<ZipAdapter>() as u64,
        ) as *mut ZipAdapter;
        (*adapter).tag = ITER_TAG_ZIP_ADAPTER;
        (*adapter).source_a = source_a;
        (*adapter).source_b = source_b;
        adapter as *mut u8
    }
}

/// Advance the zip adapter: call next() on both sources. If either is None,
/// return None. Otherwise return alloc_pair(a, b) wrapped in Some.
#[no_mangle]
pub extern "C" fn mesh_iter_zip_next(adapter_ptr: *mut u8) -> *mut u8 {
    unsafe {
        let adapter = adapter_ptr as *mut ZipAdapter;
        let opt_a = mesh_iter_generic_next((*adapter).source_a);
        let opt_a_ref = opt_a as *mut MeshOption;
        if (*opt_a_ref).tag == 1 {
            return opt_a; // None
        }
        let opt_b = mesh_iter_generic_next((*adapter).source_b);
        let opt_b_ref = opt_b as *mut MeshOption;
        if (*opt_b_ref).tag == 1 {
            return opt_b; // None
        }
        let a_val = (*opt_a_ref).value as u64;
        let b_val = (*opt_b_ref).value as u64;
        let pair = alloc_pair(a_val, b_val);
        alloc_option(0, pair as *mut u8) as *mut u8
    }
}

// ── Terminal Operations ─────────────────────────────────────────────────

/// Iter.count(iter) -- count elements until exhausted.
#[no_mangle]
pub extern "C" fn mesh_iter_count(iter: *mut u8) -> i64 {
    unsafe {
        let mut count: i64 = 0;
        loop {
            let option = mesh_iter_generic_next(iter);
            let opt_ref = option as *mut MeshOption;
            if (*opt_ref).tag == 1 {
                break;
            }
            count += 1;
        }
        count
    }
}

/// Iter.sum(iter) -- sum numeric (Int) elements until exhausted.
#[no_mangle]
pub extern "C" fn mesh_iter_sum(iter: *mut u8) -> i64 {
    unsafe {
        let mut sum: i64 = 0;
        loop {
            let option = mesh_iter_generic_next(iter);
            let opt_ref = option as *mut MeshOption;
            if (*opt_ref).tag == 1 {
                break;
            }
            sum += (*opt_ref).value as i64;
        }
        sum
    }
}

/// Iter.any(iter, fn) -- return 1 if any element passes predicate, 0 otherwise.
/// Short-circuits on first match.
#[no_mangle]
pub extern "C" fn mesh_iter_any(iter: *mut u8, fn_ptr: *mut u8, env_ptr: *mut u8) -> i8 {
    unsafe {
        loop {
            let option = mesh_iter_generic_next(iter);
            let opt_ref = option as *mut MeshOption;
            if (*opt_ref).tag == 1 {
                return 0; // Exhausted, none matched
            }
            let elem = (*opt_ref).value as u64;
            let result = if env_ptr.is_null() {
                let f: BareFn = std::mem::transmute(fn_ptr);
                f(elem)
            } else {
                let f: ClosureFn = std::mem::transmute(fn_ptr);
                f(env_ptr, elem)
            };
            if result != 0 {
                return 1; // Found a match
            }
        }
    }
}

/// Iter.all(iter, fn) -- return 1 if all elements pass predicate, 0 otherwise.
/// Short-circuits on first non-match.
#[no_mangle]
pub extern "C" fn mesh_iter_all(iter: *mut u8, fn_ptr: *mut u8, env_ptr: *mut u8) -> i8 {
    unsafe {
        loop {
            let option = mesh_iter_generic_next(iter);
            let opt_ref = option as *mut MeshOption;
            if (*opt_ref).tag == 1 {
                return 1; // All passed
            }
            let elem = (*opt_ref).value as u64;
            let result = if env_ptr.is_null() {
                let f: BareFn = std::mem::transmute(fn_ptr);
                f(elem)
            } else {
                let f: ClosureFn = std::mem::transmute(fn_ptr);
                f(env_ptr, elem)
            };
            if result == 0 {
                return 0; // Failed
            }
        }
    }
}

/// Iter.find(iter, fn) -- return Option: Some(elem) on first match, None if exhausted.
#[no_mangle]
pub extern "C" fn mesh_iter_find(iter: *mut u8, fn_ptr: *mut u8, env_ptr: *mut u8) -> *mut u8 {
    unsafe {
        loop {
            let option = mesh_iter_generic_next(iter);
            let opt_ref = option as *mut MeshOption;
            if (*opt_ref).tag == 1 {
                return alloc_option(1, std::ptr::null_mut()) as *mut u8; // None
            }
            let elem = (*opt_ref).value as u64;
            let result = if env_ptr.is_null() {
                let f: BareFn = std::mem::transmute(fn_ptr);
                f(elem)
            } else {
                let f: ClosureFn = std::mem::transmute(fn_ptr);
                f(env_ptr, elem)
            };
            if result != 0 {
                return alloc_option(0, elem as *mut u8) as *mut u8; // Some(elem)
            }
        }
    }
}

/// Iter.reduce(iter, init, fn) -- fold with accumulator.
#[no_mangle]
pub extern "C" fn mesh_iter_reduce(
    iter: *mut u8,
    init: u64,
    fn_ptr: *mut u8,
    env_ptr: *mut u8,
) -> u64 {
    unsafe {
        let mut acc = init;
        loop {
            let option = mesh_iter_generic_next(iter);
            let opt_ref = option as *mut MeshOption;
            if (*opt_ref).tag == 1 {
                break;
            }
            let elem = (*opt_ref).value as u64;
            acc = if env_ptr.is_null() {
                let f: BareFn2 = std::mem::transmute(fn_ptr);
                f(acc, elem)
            } else {
                let f: ClosureFn2 = std::mem::transmute(fn_ptr);
                f(env_ptr, acc, elem)
            };
        }
        acc
    }
}

// ── Collect Terminal Operations (Phase 79) ──────────────────────────

/// List.collect(iter) -- materialize iterator into a List.
/// Collects all elements into a safe Rust Vec, then builds the final
/// GC-allocated list via mesh_list_from_array in one shot.
/// This avoids mesh_list_builder_push which has NO bounds checking.
#[no_mangle]
pub extern "C" fn mesh_list_collect(iter: *mut u8) -> *mut u8 {
    unsafe {
        let mut elements: Vec<u64> = Vec::new();
        loop {
            let option = mesh_iter_generic_next(iter);
            let opt_ref = option as *mut MeshOption;
            if (*opt_ref).tag == 1 {
                break; // None -- done
            }
            elements.push((*opt_ref).value as u64);
        }
        mesh_list_from_array(elements.as_ptr(), elements.len() as i64)
    }
}

/// Map.collect(iter) -- materialize iterator of (key, value) tuples into a Map.
/// Expects each element to be a tuple pointer with layout { len: u64, key: u64, value: u64 }.
#[no_mangle]
pub extern "C" fn mesh_map_collect(iter: *mut u8) -> *mut u8 {
    unsafe {
        let mut map = mesh_map_new();
        loop {
            let option = mesh_iter_generic_next(iter);
            let opt_ref = option as *mut MeshOption;
            if (*opt_ref).tag == 1 {
                break; // None
            }
            let tuple_ptr = (*opt_ref).value as *mut u8;
            // Tuple layout: { u64 len=2, u64 key, u64 value }
            let key = *((tuple_ptr as *const u64).add(1));
            let val = *((tuple_ptr as *const u64).add(2));
            map = mesh_map_put(map, key, val);
        }
        map
    }
}

/// Map.collect(iter) variant for string keys -- materialize iterator of (key, value)
/// tuples into a Map with string key_type (KEY_TYPE_STR = 1).
/// Called by codegen when the type checker infers the map's key type as String.
#[no_mangle]
pub extern "C" fn mesh_map_collect_string_keys(iter: *mut u8) -> *mut u8 {
    unsafe {
        // Create map with string key_type from the start (key_type = 1)
        let mut map = crate::collections::map::mesh_map_new_typed(1);
        loop {
            let option = mesh_iter_generic_next(iter);
            let opt_ref = option as *mut MeshOption;
            if (*opt_ref).tag == 1 {
                break; // None
            }
            let tuple_ptr = (*opt_ref).value as *mut u8;
            // Tuple layout: { u64 len=2, u64 key, u64 value }
            let key = *((tuple_ptr as *const u64).add(1));
            let val = *((tuple_ptr as *const u64).add(2));
            map = mesh_map_put(map, key, val);
        }
        map
    }
}

/// Set.collect(iter) -- materialize iterator into a Set.
/// Duplicates are handled automatically by mesh_set_add.
#[no_mangle]
pub extern "C" fn mesh_set_collect(iter: *mut u8) -> *mut u8 {
    unsafe {
        let mut set = mesh_set_new();
        loop {
            let option = mesh_iter_generic_next(iter);
            let opt_ref = option as *mut MeshOption;
            if (*opt_ref).tag == 1 {
                break; // None
            }
            let elem = (*opt_ref).value as u64;
            set = mesh_set_add(set, elem);
        }
        set
    }
}

/// String.collect(iter) -- materialize string iterator into a single concatenated String.
/// Each yielded value is treated as a *const MeshString pointer (NOT an integer).
#[no_mangle]
pub extern "C" fn mesh_string_collect(iter: *mut u8) -> *mut u8 {
    unsafe {
        let mut result = mesh_string_new(std::ptr::null(), 0) as *mut u8;
        loop {
            let option = mesh_iter_generic_next(iter);
            let opt_ref = option as *mut MeshOption;
            if (*opt_ref).tag == 1 {
                break; // None
            }
            let str_ptr = (*opt_ref).value as *const MeshString;
            result = mesh_string_concat(result as *const MeshString, str_ptr) as *mut u8;
        }
        result
    }
}

// ── Tests ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::list::{
        mesh_list_from_array, mesh_list_get, mesh_list_iter_new, mesh_list_length,
    };
    use crate::collections::map::mesh_map_size;
    use crate::collections::set::mesh_set_size;
    use crate::gc::mesh_rt_init;

    fn init_runtime() {
        unsafe { mesh_rt_init() };
    }

    #[test]
    fn test_list_collect() {
        init_runtime();
        unsafe {
            // Build a list [10, 20, 30]
            let data: Vec<u64> = vec![10, 20, 30];
            let list = mesh_list_from_array(data.as_ptr(), 3);
            let iter = mesh_list_iter_new(list);
            let collected = mesh_list_collect(iter);

            assert_eq!(mesh_list_length(collected), 3);
            assert_eq!(mesh_list_get(collected, 0), 10);
            assert_eq!(mesh_list_get(collected, 1), 20);
            assert_eq!(mesh_list_get(collected, 2), 30);
        }
    }

    #[test]
    fn test_map_collect() {
        init_runtime();
        unsafe {
            // Build a list of 2 elements, then enumerate -> collect into map
            // enumerate produces (index, value) tuples
            let data: Vec<u64> = vec![100, 200];
            let list = mesh_list_from_array(data.as_ptr(), 2);
            let iter = mesh_list_iter_new(list);
            let enum_iter = mesh_iter_enumerate(iter);
            let collected_map = mesh_map_collect(enum_iter);

            assert_eq!(mesh_map_size(collected_map), 2);
        }
    }

    #[test]
    fn test_set_collect() {
        init_runtime();
        unsafe {
            // Build a list [1, 2, 2, 3] -> collect into set (dedup)
            let data: Vec<u64> = vec![1, 2, 2, 3];
            let list = mesh_list_from_array(data.as_ptr(), 4);
            let iter = mesh_list_iter_new(list);
            let collected_set = mesh_set_collect(iter);

            assert_eq!(mesh_set_size(collected_set), 3); // Deduplication: {1, 2, 3}
        }
    }
}
