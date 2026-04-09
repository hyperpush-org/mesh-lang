//! Tuple utility functions for the Mesh runtime.
//!
//! Tuples in Mesh are GC-allocated structs with a length prefix followed by
//! N u64 elements. Layout: `{ u64 len, u64[len] elements }`.
//!
//! These utilities provide runtime access to tuple elements by index.

/// Return the element at `index` in the tuple. Panics if out of bounds.
#[no_mangle]
pub extern "C" fn mesh_tuple_nth(tuple: *mut u8, index: i64) -> u64 {
    unsafe {
        let len = *(tuple as *const u64);
        if index < 0 || index as u64 >= len {
            panic!(
                "mesh_tuple_nth: index {} out of bounds (len {})",
                index, len
            );
        }
        let data = (tuple as *const u64).add(1);
        *data.add(index as usize)
    }
}

/// Return the first element of the tuple. Panics if empty.
#[no_mangle]
pub extern "C" fn mesh_tuple_first(tuple: *mut u8) -> u64 {
    mesh_tuple_nth(tuple, 0)
}

/// Return the second element of the tuple. Panics if fewer than 2 elements.
#[no_mangle]
pub extern "C" fn mesh_tuple_second(tuple: *mut u8) -> u64 {
    mesh_tuple_nth(tuple, 1)
}

/// Return the number of elements in the tuple.
#[no_mangle]
pub extern "C" fn mesh_tuple_size(tuple: *mut u8) -> i64 {
    unsafe { *(tuple as *const u64) as i64 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gc::{mesh_gc_alloc, mesh_rt_init};

    /// Helper to create a GC-allocated tuple with the given elements.
    fn make_tuple(elems: &[u64]) -> *mut u8 {
        unsafe {
            let total = 8 + elems.len() * 8; // len field + elements
            let p = mesh_gc_alloc(total as u64, 8);
            *(p as *mut u64) = elems.len() as u64;
            let data = (p as *mut u64).add(1);
            for (i, &e) in elems.iter().enumerate() {
                *data.add(i) = e;
            }
            p
        }
    }

    #[test]
    fn test_tuple_nth() {
        mesh_rt_init();
        let t = make_tuple(&[10, 20, 30]);
        assert_eq!(mesh_tuple_nth(t, 0), 10);
        assert_eq!(mesh_tuple_nth(t, 1), 20);
        assert_eq!(mesh_tuple_nth(t, 2), 30);
    }

    #[test]
    fn test_tuple_first_second() {
        mesh_rt_init();
        let t = make_tuple(&[100, 200, 300]);
        assert_eq!(mesh_tuple_first(t), 100);
        assert_eq!(mesh_tuple_second(t), 200);
    }

    #[test]
    fn test_tuple_size() {
        mesh_rt_init();
        let t = make_tuple(&[1, 2, 3, 4]);
        assert_eq!(mesh_tuple_size(t), 4);
    }

    #[test]
    fn test_tuple_empty() {
        mesh_rt_init();
        let t = make_tuple(&[]);
        assert_eq!(mesh_tuple_size(t), 0);
    }
}
