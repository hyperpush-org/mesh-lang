//! FNV-1a 64-bit hash functions for the Hash protocol.
//!
//! These runtime functions are called from generated MIR to hash primitive
//! values and combine field hashes for struct-level hashing.

const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x00000100000001B3;

fn fnv1a_bytes(bytes: &[u8]) -> u64 {
    let mut hash = FNV_OFFSET_BASIS;
    for &b in bytes {
        hash ^= b as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

#[no_mangle]
pub extern "C" fn mesh_hash_int(value: i64) -> i64 {
    fnv1a_bytes(&value.to_le_bytes()) as i64
}

#[no_mangle]
pub extern "C" fn mesh_hash_float(value: f64) -> i64 {
    fnv1a_bytes(&value.to_bits().to_le_bytes()) as i64
}

#[no_mangle]
pub extern "C" fn mesh_hash_bool(value: i8) -> i64 {
    fnv1a_bytes(&[value as u8]) as i64
}

#[no_mangle]
pub extern "C" fn mesh_hash_string(s: *const crate::string::MeshString) -> i64 {
    unsafe { fnv1a_bytes((*s).as_str().as_bytes()) as i64 }
}

/// Combine two hash values (for struct field hashing).
#[no_mangle]
pub extern "C" fn mesh_hash_combine(hash_a: i64, hash_b: i64) -> i64 {
    let mut hash = hash_a as u64;
    for &b in &(hash_b as u64).to_le_bytes() {
        hash ^= b as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_int_deterministic() {
        assert_eq!(mesh_hash_int(42), mesh_hash_int(42));
        assert_ne!(mesh_hash_int(42), mesh_hash_int(43));
    }

    #[test]
    fn hash_bool_deterministic() {
        assert_eq!(mesh_hash_bool(1), mesh_hash_bool(1));
        assert_ne!(mesh_hash_bool(0), mesh_hash_bool(1));
    }

    #[test]
    fn hash_combine_order_matters() {
        let h1 = mesh_hash_int(1);
        let h2 = mesh_hash_int(2);
        assert_ne!(mesh_hash_combine(h1, h2), mesh_hash_combine(h2, h1));
    }
}
