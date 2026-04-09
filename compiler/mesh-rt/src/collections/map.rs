//! GC-managed immutable Map for the Mesh runtime.
//!
//! A MeshMap stores key-value pairs where both keys and values are uniform
//! 8-byte (`u64`) values. Backed by a simple vector of `(u64, u64)` pairs
//! with linear scan -- efficient for the small maps typical in Phase 8.
//!
//! All mutation operations return a NEW map (immutable semantics).
//!
//! The upper 8 bits of the capacity field store a key_type tag:
//! - 0 = integer keys (compared by value)
//! - 1 = string keys (compared by content via mesh_string_eq)

use super::list::alloc_pair;
use crate::gc::mesh_gc_alloc_actor;
use std::ptr;

/// Map header: len (u64), cap (u64).
const HEADER_SIZE: usize = 16;
/// Each entry is a (key, value) pair = 16 bytes.
const ENTRY_SIZE: usize = 16;

/// Key type tag: integer keys (compared by value equality).
const KEY_TYPE_INT: u64 = 0;
/// Key type tag: string keys (compared by content via mesh_string_eq).
const KEY_TYPE_STR: u64 = 1;
/// Number of bits to shift for the key_type tag in the cap field.
const TAG_SHIFT: u64 = 56;
/// Mask for extracting the raw capacity (lower 56 bits).
const CAP_MASK: u64 = (1u64 << 56) - 1;

// ── Internal helpers ──────────────────────────────────────────────────

unsafe fn map_len(m: *const u8) -> u64 {
    *(m as *const u64)
}

/// Extract the key_type tag from the upper 8 bits of the cap field.
unsafe fn map_key_type(m: *const u8) -> u64 {
    (*((m as *const u64).add(1))) >> TAG_SHIFT
}

/// Extract the raw capacity (lower 56 bits of the cap field).
unsafe fn map_cap_raw(m: *const u8) -> u64 {
    (*((m as *const u64).add(1))) & CAP_MASK
}

#[allow(dead_code)]
unsafe fn map_cap(m: *const u8) -> u64 {
    map_cap_raw(m)
}

unsafe fn map_entries(m: *const u8) -> *const [u64; 2] {
    (m as *const u8).add(HEADER_SIZE) as *const [u64; 2]
}

unsafe fn map_entries_mut(m: *mut u8) -> *mut [u64; 2] {
    m.add(HEADER_SIZE) as *mut [u64; 2]
}

/// Check if two keys are equal, dispatching based on the map's key_type.
unsafe fn keys_equal(m: *const u8, a: u64, b: u64) -> bool {
    if map_key_type(m) == KEY_TYPE_STR {
        crate::string::mesh_string_eq(
            a as *const crate::string::MeshString,
            b as *const crate::string::MeshString,
        ) != 0
    } else {
        a == b
    }
}

unsafe fn alloc_map(cap: u64, key_type: u64) -> *mut u8 {
    let total = HEADER_SIZE + (cap as usize) * ENTRY_SIZE;
    let p = mesh_gc_alloc_actor(total as u64, 8);
    *(p as *mut u64) = 0; // len
    *((p as *mut u64).add(1)) = (key_type << TAG_SHIFT) | cap; // key_type tag + cap
    p
}

/// Find the index of a key, or return None.
unsafe fn find_key(m: *const u8, key: u64) -> Option<usize> {
    let len = map_len(m) as usize;
    let entries = map_entries(m);
    for i in 0..len {
        if keys_equal(m, (*entries.add(i))[0], key) {
            return Some(i);
        }
    }
    None
}

// ── Public API ────────────────────────────────────────────────────────

/// Create an empty map (integer keys, backward compatible).
#[no_mangle]
pub extern "C" fn mesh_map_new() -> *mut u8 {
    unsafe { alloc_map(0, KEY_TYPE_INT) }
}

/// Create an empty map with a specific key_type tag.
/// key_type: 0 = Int, 1 = String.
#[no_mangle]
pub extern "C" fn mesh_map_new_typed(key_type: i64) -> *mut u8 {
    unsafe { alloc_map(0, key_type as u64) }
}

/// Ensure a map has string key_type. If the map is empty and has integer key_type,
/// returns a new empty map with string key_type. Otherwise returns the map unchanged.
/// Used by codegen to tag maps before the first string-key put.
#[no_mangle]
pub extern "C" fn mesh_map_tag_string(map: *mut u8) -> *mut u8 {
    unsafe {
        if map_len(map) == 0 && map_key_type(map) != KEY_TYPE_STR {
            alloc_map(0, KEY_TYPE_STR)
        } else {
            map
        }
    }
}

/// Return a NEW map with the key-value pair added (or updated).
#[no_mangle]
pub extern "C" fn mesh_map_put(map: *mut u8, key: u64, value: u64) -> *mut u8 {
    unsafe {
        let len = map_len(map) as usize;
        let kt = map_key_type(map);

        // Check if key already exists -- replace.
        if let Some(idx) = find_key(map, key) {
            let new_map = alloc_map(len as u64, kt);
            *(new_map as *mut u64) = len as u64;
            ptr::copy_nonoverlapping(
                map_entries(map) as *const u8,
                map_entries_mut(new_map) as *mut u8,
                len * ENTRY_SIZE,
            );
            (*map_entries_mut(new_map).add(idx))[1] = value;
            return new_map;
        }

        // Add new entry.
        let new_len = len + 1;
        let new_map = alloc_map(new_len as u64, kt);
        *(new_map as *mut u64) = new_len as u64;
        if len > 0 {
            ptr::copy_nonoverlapping(
                map_entries(map) as *const u8,
                map_entries_mut(new_map) as *mut u8,
                len * ENTRY_SIZE,
            );
        }
        (*map_entries_mut(new_map).add(len))[0] = key;
        (*map_entries_mut(new_map).add(len))[1] = value;
        new_map
    }
}

/// Get the value for a key. Returns 0 if not found.
#[no_mangle]
pub extern "C" fn mesh_map_get(map: *mut u8, key: u64) -> u64 {
    unsafe {
        if let Some(idx) = find_key(map, key) {
            (*map_entries(map).add(idx))[1]
        } else {
            0
        }
    }
}

/// Returns 1 if the key exists, 0 otherwise.
#[no_mangle]
pub extern "C" fn mesh_map_has_key(map: *mut u8, key: u64) -> i8 {
    unsafe {
        if find_key(map, key).is_some() {
            1
        } else {
            0
        }
    }
}

/// Return a NEW map without the given key.
#[no_mangle]
pub extern "C" fn mesh_map_delete(map: *mut u8, key: u64) -> *mut u8 {
    unsafe {
        let len = map_len(map) as usize;
        let kt = map_key_type(map);
        match find_key(map, key) {
            Some(idx) => {
                let new_len = len - 1;
                let new_map = alloc_map(new_len as u64, kt);
                *(new_map as *mut u64) = new_len as u64;
                let src = map_entries(map);
                let dst = map_entries_mut(new_map);
                let mut j = 0;
                for i in 0..len {
                    if i != idx {
                        (*dst.add(j))[0] = (*src.add(i))[0];
                        (*dst.add(j))[1] = (*src.add(i))[1];
                        j += 1;
                    }
                }
                new_map
            }
            None => {
                // Key not found -- return a copy.
                let new_map = alloc_map(len as u64, kt);
                *(new_map as *mut u64) = len as u64;
                if len > 0 {
                    ptr::copy_nonoverlapping(
                        map_entries(map) as *const u8,
                        map_entries_mut(new_map) as *mut u8,
                        len * ENTRY_SIZE,
                    );
                }
                new_map
            }
        }
    }
}

/// Return the number of entries in the map.
#[no_mangle]
pub extern "C" fn mesh_map_size(map: *mut u8) -> i64 {
    unsafe { map_len(map) as i64 }
}

/// Return a List of all keys in the map.
#[no_mangle]
pub extern "C" fn mesh_map_keys(map: *mut u8) -> *mut u8 {
    unsafe {
        let len = map_len(map) as usize;
        let entries = map_entries(map);
        let mut list = super::list::mesh_list_new();
        for i in 0..len {
            list = super::list::mesh_list_append(list, (*entries.add(i))[0]);
        }
        list
    }
}

/// Return a List of all values in the map.
#[no_mangle]
pub extern "C" fn mesh_map_values(map: *mut u8) -> *mut u8 {
    unsafe {
        let len = map_len(map) as usize;
        let entries = map_entries(map);
        let mut list = super::list::mesh_list_new();
        for i in 0..len {
            list = super::list::mesh_list_append(list, (*entries.add(i))[1]);
        }
        list
    }
}

/// Get the key at index i (insertion order). Panics if out of bounds.
/// Used by for-in codegen for indexed map iteration.
#[no_mangle]
pub extern "C" fn mesh_map_entry_key(map: *mut u8, index: i64) -> u64 {
    unsafe {
        let len = map_len(map);
        if index < 0 || index as u64 >= len {
            panic!(
                "mesh_map_entry_key: index {} out of bounds (len {})",
                index, len
            );
        }
        let entries = map_entries(map);
        (*entries.add(index as usize))[0]
    }
}

/// Get the value at index i (insertion order). Panics if out of bounds.
/// Used by for-in codegen for indexed map iteration.
#[no_mangle]
pub extern "C" fn mesh_map_entry_value(map: *mut u8, index: i64) -> u64 {
    unsafe {
        let len = map_len(map);
        if index < 0 || index as u64 >= len {
            panic!(
                "mesh_map_entry_value: index {} out of bounds (len {})",
                index, len
            );
        }
        let entries = map_entries(map);
        (*entries.add(index as usize))[1]
    }
}

/// Convert a map to a human-readable MeshString: `%{k1 => v1, k2 => v2, ...}`.
///
/// `key_to_str` and `val_to_str` are bare function pointers `fn(u64) -> *mut u8`
/// that convert keys and values to MeshString pointers respectively.
#[no_mangle]
pub extern "C" fn mesh_map_to_string(
    map: *mut u8,
    key_to_str: *mut u8,
    val_to_str: *mut u8,
) -> *mut u8 {
    type ElemToStr = unsafe extern "C" fn(u64) -> *mut u8;

    unsafe {
        let len = map_len(map) as usize;
        let entries = map_entries(map);
        let kf: ElemToStr = std::mem::transmute(key_to_str);
        let vf: ElemToStr = std::mem::transmute(val_to_str);

        let mut result = crate::string::mesh_string_new(b"%{".as_ptr(), 2) as *mut u8;
        for i in 0..len {
            if i > 0 {
                let sep = crate::string::mesh_string_new(b", ".as_ptr(), 2) as *mut u8;
                result = crate::string::mesh_string_concat(
                    result as *const crate::string::MeshString,
                    sep as *const crate::string::MeshString,
                ) as *mut u8;
            }
            let key = (*entries.add(i))[0];
            let val = (*entries.add(i))[1];
            let key_str = kf(key);
            result = crate::string::mesh_string_concat(
                result as *const crate::string::MeshString,
                key_str as *const crate::string::MeshString,
            ) as *mut u8;
            let arrow = crate::string::mesh_string_new(b" => ".as_ptr(), 4) as *mut u8;
            result = crate::string::mesh_string_concat(
                result as *const crate::string::MeshString,
                arrow as *const crate::string::MeshString,
            ) as *mut u8;
            let val_str = vf(val);
            result = crate::string::mesh_string_concat(
                result as *const crate::string::MeshString,
                val_str as *const crate::string::MeshString,
            ) as *mut u8;
        }
        let close = crate::string::mesh_string_new(b"}".as_ptr(), 1) as *mut u8;
        result = crate::string::mesh_string_concat(
            result as *const crate::string::MeshString,
            close as *const crate::string::MeshString,
        ) as *mut u8;
        result
    }
}

/// Merge two maps. All entries from `a` are included; entries from `b`
/// overwrite duplicates from `a`. Returns a NEW merged map.
#[no_mangle]
pub extern "C" fn mesh_map_merge(a: *mut u8, b: *mut u8) -> *mut u8 {
    unsafe {
        let a_len = map_len(a) as usize;
        let b_len = map_len(b) as usize;
        let kt = map_key_type(a);

        // Start with a copy of `a`.
        let mut result = alloc_map((a_len + b_len) as u64, kt);
        *(result as *mut u64) = a_len as u64;
        if a_len > 0 {
            ptr::copy_nonoverlapping(
                map_entries(a) as *const u8,
                map_entries_mut(result) as *mut u8,
                a_len * ENTRY_SIZE,
            );
        }

        // Add/overwrite entries from `b`.
        let b_entries = map_entries(b);
        for i in 0..b_len {
            let key = (*b_entries.add(i))[0];
            let val = (*b_entries.add(i))[1];
            result = mesh_map_put(result, key, val);
        }

        result
    }
}

/// Convert a map to a list of (key, value) 2-tuples.
#[no_mangle]
pub extern "C" fn mesh_map_to_list(map: *mut u8) -> *mut u8 {
    unsafe {
        let len = map_len(map) as usize;
        let entries = map_entries(map);
        let list = super::list::mesh_list_builder_new(len as i64);
        for i in 0..len {
            let key = (*entries.add(i))[0];
            let val = (*entries.add(i))[1];
            let pair = alloc_pair(key, val);
            super::list::mesh_list_builder_push(list, pair as u64);
        }
        list
    }
}

/// Build a map from a list of (key, value) 2-tuples.
/// Defaults to KEY_TYPE_INT since runtime cannot detect key type.
#[no_mangle]
pub extern "C" fn mesh_map_from_list(list: *mut u8) -> *mut u8 {
    unsafe {
        let len = super::list::mesh_list_length(list);
        let mut map = mesh_map_new();
        let data = (list as *const u64).add(2); // skip len + cap header
        for i in 0..len as usize {
            let tuple_ptr = *data.add(i) as *mut u8;
            let key = *((tuple_ptr as *const u64).add(1)); // offset 1 = first tuple element
            let val = *((tuple_ptr as *const u64).add(2)); // offset 2 = second tuple element
            map = mesh_map_put(map, key, val);
        }
        map
    }
}

// ── Iterator handle ───────────────────────────────────────────────────

/// Internal iterator state for Map iteration.
#[repr(C)]
struct MapIterator {
    tag: u8,
    map: *mut u8,
    index: i64,
    size: i64,
}

/// Create a new iterator handle for a map.
#[no_mangle]
pub extern "C" fn mesh_map_iter_new(map: *mut u8) -> *mut u8 {
    unsafe {
        let size = mesh_map_size(map);
        let iter = crate::gc::mesh_gc_alloc_actor(
            std::mem::size_of::<MapIterator>() as u64,
            std::mem::align_of::<MapIterator>() as u64,
        ) as *mut MapIterator;
        (*iter).tag = 1; // ITER_TAG_MAP
        (*iter).map = map;
        (*iter).index = 0;
        (*iter).size = size;
        iter as *mut u8
    }
}

/// Advance the map iterator, returning Option<(K, V)> (tag 0 = Some, tag 1 = None).
/// The Some payload is a GC-allocated 2-tuple (key, value).
#[no_mangle]
pub extern "C" fn mesh_map_iter_next(iter_ptr: *mut u8) -> *mut u8 {
    unsafe {
        let iter = iter_ptr as *mut MapIterator;
        if (*iter).index >= (*iter).size {
            crate::option::alloc_option(1, std::ptr::null_mut()) as *mut u8
        } else {
            let key = mesh_map_entry_key((*iter).map, (*iter).index);
            let val = mesh_map_entry_value((*iter).map, (*iter).index);
            (*iter).index += 1;
            let pair = alloc_pair(key, val);
            crate::option::alloc_option(0, pair as *mut u8) as *mut u8
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gc::mesh_rt_init;

    #[test]
    fn test_map_new_is_empty() {
        mesh_rt_init();
        let map = mesh_map_new();
        assert_eq!(mesh_map_size(map), 0);
    }

    #[test]
    fn test_map_put_get() {
        mesh_rt_init();
        let map = mesh_map_new();
        let map = mesh_map_put(map, 42, 100);
        assert_eq!(mesh_map_size(map), 1);
        assert_eq!(mesh_map_get(map, 42), 100);
        assert_eq!(mesh_map_has_key(map, 42), 1);
        assert_eq!(mesh_map_has_key(map, 99), 0);
    }

    #[test]
    fn test_map_put_overwrite() {
        mesh_rt_init();
        let map = mesh_map_new();
        let map = mesh_map_put(map, 1, 10);
        let map = mesh_map_put(map, 1, 20);
        assert_eq!(mesh_map_size(map), 1);
        assert_eq!(mesh_map_get(map, 1), 20);
    }

    #[test]
    fn test_map_delete() {
        mesh_rt_init();
        let map = mesh_map_new();
        let map = mesh_map_put(map, 1, 10);
        let map = mesh_map_put(map, 2, 20);
        let map = mesh_map_delete(map, 1);
        assert_eq!(mesh_map_size(map), 1);
        assert_eq!(mesh_map_has_key(map, 1), 0);
        assert_eq!(mesh_map_get(map, 2), 20);
    }

    #[test]
    fn test_map_keys_values() {
        mesh_rt_init();
        let map = mesh_map_new();
        let map = mesh_map_put(map, 1, 10);
        let map = mesh_map_put(map, 2, 20);
        let keys = mesh_map_keys(map);
        let vals = mesh_map_values(map);
        assert_eq!(super::super::list::mesh_list_length(keys), 2);
        assert_eq!(super::super::list::mesh_list_length(vals), 2);
    }

    #[test]
    fn test_map_immutability() {
        mesh_rt_init();
        let map1 = mesh_map_new();
        let map2 = mesh_map_put(map1, 1, 10);
        // Original map unchanged.
        assert_eq!(mesh_map_size(map1), 0);
        assert_eq!(mesh_map_size(map2), 1);
    }

    #[test]
    fn test_map_string_keys() {
        mesh_rt_init();
        // Create a string-key map (key_type = 1).
        let map = mesh_map_new_typed(1);

        // Create string keys and values.
        let key1 = crate::string::mesh_string_new(b"name".as_ptr(), 4) as u64;
        let val1 = crate::string::mesh_string_new(b"Alice".as_ptr(), 5) as u64;
        let key2 = crate::string::mesh_string_new(b"city".as_ptr(), 4) as u64;
        let val2 = crate::string::mesh_string_new(b"Portland".as_ptr(), 8) as u64;

        let map = mesh_map_put(map, key1, val1);
        let map = mesh_map_put(map, key2, val2);

        assert_eq!(mesh_map_size(map), 2);

        // Look up with a DIFFERENT string pointer but same content.
        let lookup_key = crate::string::mesh_string_new(b"name".as_ptr(), 4) as u64;
        let got = mesh_map_get(map, lookup_key);
        assert_eq!(got, val1);

        let lookup_key2 = crate::string::mesh_string_new(b"city".as_ptr(), 4) as u64;
        assert_eq!(mesh_map_has_key(map, lookup_key2), 1);

        // Non-existent key.
        let missing = crate::string::mesh_string_new(b"missing".as_ptr(), 7) as u64;
        assert_eq!(mesh_map_has_key(map, missing), 0);
    }

    #[test]
    fn test_map_string_key_overwrite() {
        mesh_rt_init();
        let map = mesh_map_new_typed(1);

        let key = crate::string::mesh_string_new(b"name".as_ptr(), 4) as u64;
        let val1 = crate::string::mesh_string_new(b"Alice".as_ptr(), 5) as u64;
        let val2 = crate::string::mesh_string_new(b"Bob".as_ptr(), 3) as u64;

        let map = mesh_map_put(map, key, val1);
        // Use a different pointer for the same key content.
        let key2 = crate::string::mesh_string_new(b"name".as_ptr(), 4) as u64;
        let map = mesh_map_put(map, key2, val2);

        assert_eq!(mesh_map_size(map), 1);

        let lookup = crate::string::mesh_string_new(b"name".as_ptr(), 4) as u64;
        let got = mesh_map_get(map, lookup);
        assert_eq!(got, val2);
    }

    #[test]
    fn test_map_to_string() {
        mesh_rt_init();
        let map = mesh_map_new();
        let map = mesh_map_put(map, 1, 10);
        let map = mesh_map_put(map, 2, 20);

        let result = mesh_map_to_string(
            map,
            crate::string::mesh_int_to_string as *mut u8,
            crate::string::mesh_int_to_string as *mut u8,
        );
        let s = unsafe { &*(result as *const crate::string::MeshString) };
        let text = unsafe { s.as_str() };
        assert_eq!(text, "%{1 => 10, 2 => 20}");
    }

    #[test]
    fn test_map_to_string_empty() {
        mesh_rt_init();
        let map = mesh_map_new();

        let result = mesh_map_to_string(
            map,
            crate::string::mesh_int_to_string as *mut u8,
            crate::string::mesh_int_to_string as *mut u8,
        );
        let s = unsafe { &*(result as *const crate::string::MeshString) };
        let text = unsafe { s.as_str() };
        assert_eq!(text, "%{}");
    }

    #[test]
    fn test_map_entry_key_value() {
        mesh_rt_init();
        let map = mesh_map_new();
        let map = mesh_map_put(map, 10, 100);
        let map = mesh_map_put(map, 20, 200);
        let map = mesh_map_put(map, 30, 300);

        assert_eq!(mesh_map_entry_key(map, 0), 10);
        assert_eq!(mesh_map_entry_value(map, 0), 100);
        assert_eq!(mesh_map_entry_key(map, 1), 20);
        assert_eq!(mesh_map_entry_value(map, 1), 200);
        assert_eq!(mesh_map_entry_key(map, 2), 30);
        assert_eq!(mesh_map_entry_value(map, 2), 300);
    }
}
