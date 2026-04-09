//! Mesh Term Format (STF) binary serializer/deserializer.
//!
//! STF is a self-describing binary format for encoding Mesh runtime values
//! for inter-node message transport. Each value is prefixed by a 1-byte
//! type tag, enabling recursive serialization/deserialization.
//!
//! ## Wire Layout
//!
//! Every STF payload starts with a version byte (`STF_VERSION`), followed
//! by a single encoded value (which may recursively contain nested values).
//!
//! ## Safety Invariant
//!
//! STF encode runs as a pure Rust function operating on raw pointers -- it
//! does NOT call any Mesh runtime functions that trigger `reduction_check`.
//! This means GC cannot trigger during serialization, so GC-managed objects
//! referenced by raw pointers remain valid throughout the encode operation.

// ── STF Version ──────────────────────────────────────────────────────────

/// Version byte written as the first byte of every STF payload.
pub const STF_VERSION: u8 = 1;

// ── Type Tag Constants ───────────────────────────────────────────────────

// Scalar types
pub const TAG_INT: u8 = 1; // i64, 8 bytes LE
pub const TAG_FLOAT: u8 = 2; // f64, 8 bytes LE (IEEE 754)
pub const TAG_BOOL_TRUE: u8 = 3; // no payload
pub const TAG_BOOL_FALSE: u8 = 4; // no payload
pub const TAG_STRING: u8 = 5; // u32 len + UTF-8 bytes
pub const TAG_UNIT: u8 = 6; // no payload

// Container types
pub const TAG_LIST: u8 = 10; // u32 count + count * encoded elements
pub const TAG_MAP: u8 = 11; // u8 key_type + u32 count + count * (key, value)
pub const TAG_SET: u8 = 12; // u32 count + count * encoded elements
pub const TAG_TUPLE: u8 = 13; // u8 arity + arity * encoded elements

// Composite types
pub const TAG_STRUCT: u8 = 20; // u16 name_len + name + u16 field_count + fields
pub const TAG_SUM_TYPE: u8 = 21; // u16 type_name_len + name + u8 variant_tag + u16 field_count + fields

// Identity types
pub const TAG_PID: u8 = 30; // u64 raw PID (includes node_id + creation + local_id)

// Option/Result (special-cased sum types for efficiency)
pub const TAG_OPTION_SOME: u8 = 40; // + encoded inner value
pub const TAG_OPTION_NONE: u8 = 41; // no payload
pub const TAG_RESULT_OK: u8 = 42; // + encoded inner value
pub const TAG_RESULT_ERR: u8 = 43; // + encoded inner value

// Error sentinel
pub const TAG_CLOSURE: u8 = 0xFF; // NEVER written -- triggers runtime error

// ── Safety Limits ────────────────────────────────────────────────────────

/// Maximum string length in bytes (16 MB).
const MAX_STRING_LEN: u32 = 16 * 1024 * 1024;

/// Maximum collection element count (1 million).
const MAX_COLLECTION_LEN: u32 = 1_000_000;

/// Maximum field name / type name length in bytes (64 KB).
const MAX_NAME_LEN: u16 = u16::MAX;

// ── StfType ──────────────────────────────────────────────────────────────

/// Type hint enum that mirrors Mesh's runtime type system.
///
/// The STF encoder requires type hints because Mesh stores all values as
/// uniform `u64` at runtime (type erasure). The codegen layer provides
/// these hints when emitting remote send calls.
#[derive(Debug, Clone, PartialEq)]
pub enum StfType {
    Int,
    Float,
    Bool,
    String,
    Unit,
    Pid,
    List(Box<StfType>),              // element type
    Map(Box<StfType>, Box<StfType>), // key type, value type
    Set(Box<StfType>),               // element type
    Tuple(Vec<StfType>),             // element types
    Struct(std::string::String, Vec<(std::string::String, StfType)>), // name, fields
    SumType(
        std::string::String,
        Vec<(std::string::String, Vec<StfType>)>,
    ), // name, variants
    OptionOf(Box<StfType>),          // inner type
    ResultOf(Box<StfType>, Box<StfType>), // ok type, err type
    Closure,                         // always errors
    FnPtr,                           // always errors
}

// ── StfError ─────────────────────────────────────────────────────────────

/// Errors that can occur during STF encode/decode.
#[derive(Debug, Clone, PartialEq)]
pub enum StfError {
    /// The input buffer was truncated or too short.
    UnexpectedEof,
    /// An unknown or unsupported type tag was encountered.
    InvalidTag(u8),
    /// The version byte does not match `STF_VERSION`.
    InvalidVersion(u8),
    /// Attempted to serialize a closure or function pointer.
    ClosureNotSerializable,
    /// A length field exceeds the safety limit.
    PayloadTooLarge(u32),
    /// A string payload contains invalid UTF-8.
    InvalidUtf8,
}

impl std::fmt::Display for StfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StfError::UnexpectedEof => write!(f, "STF: unexpected end of input"),
            StfError::InvalidTag(tag) => write!(f, "STF: invalid type tag 0x{:02X}", tag),
            StfError::InvalidVersion(v) => write!(f, "STF: unsupported version {}", v),
            StfError::ClosureNotSerializable => {
                write!(
                    f,
                    "STF: closures and function pointers cannot be serialized"
                )
            }
            StfError::PayloadTooLarge(len) => {
                write!(f, "STF: payload length {} exceeds safety limit", len)
            }
            StfError::InvalidUtf8 => write!(f, "STF: string payload is not valid UTF-8"),
        }
    }
}

// ── Encode ───────────────────────────────────────────────────────────────

use crate::string::{mesh_string_new, MeshString};

/// Encode a single Mesh value into the buffer (without version byte).
///
/// `value` is the raw `u64` representation of the Mesh value. The
/// `type_hint` tells the encoder how to interpret the bits.
///
/// # Safety
///
/// For `StfType::String`, `value` must be a valid pointer to a `MeshString`.
pub fn stf_encode(value: u64, type_hint: &StfType, buf: &mut Vec<u8>) -> Result<(), StfError> {
    match type_hint {
        StfType::Int => {
            buf.push(TAG_INT);
            buf.extend_from_slice(&(value as i64).to_le_bytes());
            Ok(())
        }
        StfType::Float => {
            buf.push(TAG_FLOAT);
            buf.extend_from_slice(&f64::from_bits(value).to_le_bytes());
            Ok(())
        }
        StfType::Bool => {
            if value != 0 {
                buf.push(TAG_BOOL_TRUE);
            } else {
                buf.push(TAG_BOOL_FALSE);
            }
            Ok(())
        }
        StfType::String => {
            buf.push(TAG_STRING);
            let mesh_str = unsafe { &*(value as *const MeshString) };
            let bytes = unsafe { mesh_str.as_bytes() };
            let len = bytes.len() as u32;
            if len > MAX_STRING_LEN {
                return Err(StfError::PayloadTooLarge(len));
            }
            buf.extend_from_slice(&len.to_le_bytes());
            buf.extend_from_slice(bytes);
            Ok(())
        }
        StfType::Unit => {
            buf.push(TAG_UNIT);
            Ok(())
        }
        StfType::Pid => {
            buf.push(TAG_PID);
            buf.extend_from_slice(&value.to_le_bytes());
            Ok(())
        }
        StfType::Closure | StfType::FnPtr => Err(StfError::ClosureNotSerializable),

        // ── Container types ────────────────────────────────────────
        StfType::List(elem_type) => {
            buf.push(TAG_LIST);
            let ptr = value as *const u8;
            let len = unsafe { *(ptr as *const u64) } as u32;
            if len > MAX_COLLECTION_LEN {
                return Err(StfError::PayloadTooLarge(len));
            }
            buf.extend_from_slice(&len.to_le_bytes());
            let data = unsafe { (ptr as *const u64).add(2) };
            for i in 0..len as usize {
                let elem = unsafe { *data.add(i) };
                stf_encode(elem, elem_type, buf)?;
            }
            Ok(())
        }

        StfType::Map(key_type, val_type) => {
            buf.push(TAG_MAP);
            let ptr = value as *const u8;
            let len = unsafe { *(ptr as *const u64) } as u32;
            if len > MAX_COLLECTION_LEN {
                return Err(StfError::PayloadTooLarge(len));
            }
            // Extract key_type_tag from upper 8 bits of cap field.
            let key_type_tag = unsafe { (*((ptr as *const u64).add(1))) >> 56 } as u8;
            buf.push(key_type_tag);
            buf.extend_from_slice(&len.to_le_bytes());
            // Entries start at offset 2 words. Each entry is [u64 key, u64 value].
            let entries = unsafe { (ptr as *const u64).add(2) };
            for i in 0..len as usize {
                let key = unsafe { *entries.add(i * 2) };
                let val = unsafe { *entries.add(i * 2 + 1) };
                stf_encode(key, key_type, buf)?;
                stf_encode(val, val_type, buf)?;
            }
            Ok(())
        }

        StfType::Set(elem_type) => {
            buf.push(TAG_SET);
            let ptr = value as *const u8;
            let len = unsafe { *(ptr as *const u64) } as u32;
            if len > MAX_COLLECTION_LEN {
                return Err(StfError::PayloadTooLarge(len));
            }
            buf.extend_from_slice(&len.to_le_bytes());
            let data = unsafe { (ptr as *const u64).add(2) };
            for i in 0..len as usize {
                let elem = unsafe { *data.add(i) };
                stf_encode(elem, elem_type, buf)?;
            }
            Ok(())
        }

        StfType::Tuple(elem_types) => {
            buf.push(TAG_TUPLE);
            let arity = elem_types.len() as u8;
            buf.push(arity);
            let ptr = value as *const u8;
            // Tuple layout: { u64 len, u64[len] data }
            let data = unsafe { (ptr as *const u64).add(1) };
            for (i, et) in elem_types.iter().enumerate() {
                let elem = unsafe { *data.add(i) };
                stf_encode(elem, et, buf)?;
            }
            Ok(())
        }

        // ── Composite types ───────────────────────────────────────
        StfType::Struct(name, fields) => {
            buf.push(TAG_STRUCT);
            // Write struct name.
            let name_bytes = name.as_bytes();
            let name_len = name_bytes.len() as u16;
            buf.extend_from_slice(&name_len.to_le_bytes());
            buf.extend_from_slice(name_bytes);
            // Write field count.
            let field_count = fields.len() as u16;
            buf.extend_from_slice(&field_count.to_le_bytes());
            // Struct value is contiguous u64 fields (no header).
            let ptr = value as *const u64;
            for (i, (field_name, field_type)) in fields.iter().enumerate() {
                let fn_bytes = field_name.as_bytes();
                let fn_len = fn_bytes.len() as u16;
                buf.extend_from_slice(&fn_len.to_le_bytes());
                buf.extend_from_slice(fn_bytes);
                let field_val = unsafe { *ptr.add(i) };
                stf_encode(field_val, field_type, buf)?;
            }
            Ok(())
        }

        StfType::SumType(type_name, variants) => {
            buf.push(TAG_SUM_TYPE);
            // Write type name.
            let tn_bytes = type_name.as_bytes();
            let tn_len = tn_bytes.len() as u16;
            buf.extend_from_slice(&tn_len.to_le_bytes());
            buf.extend_from_slice(tn_bytes);
            // Read variant tag from first byte of sum type layout.
            let variant_tag = unsafe { *(value as *const u8) };
            buf.push(variant_tag);
            // Look up the variant's field types.
            let variant_fields = if (variant_tag as usize) < variants.len() {
                &variants[variant_tag as usize].1
            } else {
                return Err(StfError::InvalidTag(variant_tag));
            };
            let field_count = variant_fields.len() as u16;
            buf.extend_from_slice(&field_count.to_le_bytes());
            // Fields start at offset 8 (after tag byte, padded to 8-byte alignment).
            let fields_ptr = unsafe { (value as *const u64).add(1) };
            for (i, ft) in variant_fields.iter().enumerate() {
                let fv = unsafe { *fields_ptr.add(i) };
                stf_encode(fv, ft, buf)?;
            }
            Ok(())
        }

        // ── Option/Result (special-cased) ─────────────────────────
        StfType::OptionOf(inner_type) => {
            // MeshOption layout: { tag: u8 at offset 0, value: *mut u8 at offset 8 }
            let tag = unsafe { *(value as *const u8) };
            if tag == 0 {
                // Some
                buf.push(TAG_OPTION_SOME);
                let inner_ptr = unsafe { *((value as *const u64).add(1)) };
                stf_encode(inner_ptr, inner_type, buf)?;
            } else {
                // None
                buf.push(TAG_OPTION_NONE);
            }
            Ok(())
        }

        StfType::ResultOf(ok_type, err_type) => {
            // Same layout as Option: { tag: u8, value: *mut u8 at offset 8 }
            let tag = unsafe { *(value as *const u8) };
            if tag == 0 {
                // Ok
                buf.push(TAG_RESULT_OK);
                let inner_ptr = unsafe { *((value as *const u64).add(1)) };
                stf_encode(inner_ptr, ok_type, buf)?;
            } else {
                // Err
                buf.push(TAG_RESULT_ERR);
                let inner_ptr = unsafe { *((value as *const u64).add(1)) };
                stf_encode(inner_ptr, err_type, buf)?;
            }
            Ok(())
        }
    }
}

/// Encode a Mesh value with the STF version header.
///
/// Returns a complete STF payload: `[version_byte][encoded_value]`.
///
/// # Safety
///
/// Same safety requirements as [`stf_encode`].
pub fn stf_encode_value(value: u64, type_hint: &StfType) -> Result<Vec<u8>, StfError> {
    let mut buf = Vec::with_capacity(64);
    buf.push(STF_VERSION);
    stf_encode(value, type_hint, &mut buf)?;
    Ok(buf)
}

// ── Decode ───────────────────────────────────────────────────────────────

/// Helper: read exactly `n` bytes from `data` at `*pos`, advancing `*pos`.
#[inline]
fn read_bytes<'a>(data: &'a [u8], pos: &mut usize, n: usize) -> Result<&'a [u8], StfError> {
    if *pos + n > data.len() {
        return Err(StfError::UnexpectedEof);
    }
    let slice = &data[*pos..*pos + n];
    *pos += n;
    Ok(slice)
}

/// Helper: read a single byte from `data` at `*pos`, advancing `*pos`.
#[inline]
fn read_u8(data: &[u8], pos: &mut usize) -> Result<u8, StfError> {
    if *pos >= data.len() {
        return Err(StfError::UnexpectedEof);
    }
    let b = data[*pos];
    *pos += 1;
    Ok(b)
}

/// Helper: read a little-endian u16 from `data` at `*pos`.
#[inline]
fn read_u16(data: &[u8], pos: &mut usize) -> Result<u16, StfError> {
    let bytes = read_bytes(data, pos, 2)?;
    Ok(u16::from_le_bytes(bytes.try_into().unwrap()))
}

/// Helper: read a little-endian u32 from `data` at `*pos`.
#[inline]
fn read_u32(data: &[u8], pos: &mut usize) -> Result<u32, StfError> {
    let bytes = read_bytes(data, pos, 4)?;
    Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
}

/// Helper: read a length-prefixed UTF-8 string (u16 len + bytes).
fn read_name(data: &[u8], pos: &mut usize) -> Result<std::string::String, StfError> {
    let len = read_u16(data, pos)?;
    if len > MAX_NAME_LEN {
        return Err(StfError::PayloadTooLarge(len as u32));
    }
    let bytes = read_bytes(data, pos, len as usize)?;
    std::str::from_utf8(bytes)
        .map(|s| s.to_string())
        .map_err(|_| StfError::InvalidUtf8)
}

/// Decode a single STF value from `data` starting at `*pos`.
///
/// Returns `(raw_u64_value, decoded_type)` and advances `*pos` past
/// the consumed bytes.
///
/// # Safety
///
/// For `TAG_STRING`, this allocates a new `MeshString` via the GC
/// allocator. The caller must ensure the GC arena is initialized.
pub fn stf_decode(data: &[u8], pos: &mut usize) -> Result<(u64, StfType), StfError> {
    let tag = read_u8(data, pos)?;
    match tag {
        TAG_INT => {
            let bytes = read_bytes(data, pos, 8)?;
            let val = i64::from_le_bytes(bytes.try_into().unwrap());
            Ok((val as u64, StfType::Int))
        }
        TAG_FLOAT => {
            let bytes = read_bytes(data, pos, 8)?;
            let bits = u64::from_le_bytes(bytes.try_into().unwrap());
            Ok((bits, StfType::Float))
        }
        TAG_BOOL_TRUE => Ok((1, StfType::Bool)),
        TAG_BOOL_FALSE => Ok((0, StfType::Bool)),
        TAG_STRING => {
            let len_bytes = read_bytes(data, pos, 4)?;
            let len = u32::from_le_bytes(len_bytes.try_into().unwrap());
            if len > MAX_STRING_LEN {
                return Err(StfError::PayloadTooLarge(len));
            }
            let str_bytes = read_bytes(data, pos, len as usize)?;
            // Validate UTF-8 before allocating.
            if std::str::from_utf8(str_bytes).is_err() {
                return Err(StfError::InvalidUtf8);
            }
            let str_ptr = mesh_string_new(str_bytes.as_ptr(), len as u64);
            Ok((str_ptr as u64, StfType::String))
        }
        TAG_UNIT => Ok((0, StfType::Unit)),
        TAG_PID => {
            let bytes = read_bytes(data, pos, 8)?;
            let raw_pid = u64::from_le_bytes(bytes.try_into().unwrap());
            Ok((raw_pid, StfType::Pid))
        }
        TAG_CLOSURE => Err(StfError::ClosureNotSerializable),

        // ── Container tags ─────────────────────────────────────────
        TAG_LIST => {
            let count = read_u32(data, pos)?;
            if count > MAX_COLLECTION_LEN {
                return Err(StfError::PayloadTooLarge(count));
            }
            // Allocate list: { len: u64, cap: u64, data: [u64; count] }
            let total = 16 + (count as usize) * 8;
            let ptr = crate::gc::mesh_gc_alloc_actor(total as u64, 8);
            unsafe {
                *(ptr as *mut u64) = count as u64; // len
                *((ptr as *mut u64).add(1)) = count as u64; // cap
            }
            let data_ptr = unsafe { (ptr as *mut u64).add(2) };
            let mut elem_type = StfType::Unit;
            for i in 0..count as usize {
                let (val, et) = stf_decode(data, pos)?;
                unsafe {
                    *data_ptr.add(i) = val;
                }
                if i == 0 {
                    elem_type = et;
                }
            }
            Ok((ptr as u64, StfType::List(Box::new(elem_type))))
        }

        TAG_MAP => {
            let key_type_tag = read_u8(data, pos)?;
            let count = read_u32(data, pos)?;
            if count > MAX_COLLECTION_LEN {
                return Err(StfError::PayloadTooLarge(count));
            }
            // Allocate map: { len: u64, cap|key_type: u64, entries: [(u64,u64); count] }
            let total = 16 + (count as usize) * 16;
            let ptr = crate::gc::mesh_gc_alloc_actor(total as u64, 8);
            unsafe {
                *(ptr as *mut u64) = count as u64; // len
                                                   // Store cap with key_type_tag in upper 8 bits.
                *((ptr as *mut u64).add(1)) = ((key_type_tag as u64) << 56) | (count as u64);
            }
            let entries_ptr = unsafe { (ptr as *mut u64).add(2) };
            let mut kt = StfType::Int;
            let mut vt = StfType::Unit;
            for i in 0..count as usize {
                let (key, key_t) = stf_decode(data, pos)?;
                let (val, val_t) = stf_decode(data, pos)?;
                unsafe {
                    *entries_ptr.add(i * 2) = key;
                    *entries_ptr.add(i * 2 + 1) = val;
                }
                if i == 0 {
                    kt = key_t;
                    vt = val_t;
                }
            }
            Ok((ptr as u64, StfType::Map(Box::new(kt), Box::new(vt))))
        }

        TAG_SET => {
            let count = read_u32(data, pos)?;
            if count > MAX_COLLECTION_LEN {
                return Err(StfError::PayloadTooLarge(count));
            }
            // Allocate set: { len: u64, cap: u64, data: [u64; count] }
            let total = 16 + (count as usize) * 8;
            let ptr = crate::gc::mesh_gc_alloc_actor(total as u64, 8);
            unsafe {
                *(ptr as *mut u64) = count as u64; // len
                *((ptr as *mut u64).add(1)) = count as u64; // cap
            }
            let data_ptr = unsafe { (ptr as *mut u64).add(2) };
            let mut elem_type = StfType::Unit;
            for i in 0..count as usize {
                let (val, et) = stf_decode(data, pos)?;
                unsafe {
                    *data_ptr.add(i) = val;
                }
                if i == 0 {
                    elem_type = et;
                }
            }
            Ok((ptr as u64, StfType::Set(Box::new(elem_type))))
        }

        TAG_TUPLE => {
            let arity = read_u8(data, pos)?;
            // Allocate tuple: { u64 len, u64[arity] data }
            let total = 8 + (arity as usize) * 8;
            let ptr = crate::gc::mesh_gc_alloc_actor(total as u64, 8);
            unsafe {
                *(ptr as *mut u64) = arity as u64;
            }
            let data_ptr = unsafe { (ptr as *mut u64).add(1) };
            let mut elem_types = Vec::with_capacity(arity as usize);
            for i in 0..arity as usize {
                let (val, et) = stf_decode(data, pos)?;
                unsafe {
                    *data_ptr.add(i) = val;
                }
                elem_types.push(et);
            }
            Ok((ptr as u64, StfType::Tuple(elem_types)))
        }

        // ── Composite tags ────────────────────────────────────────
        TAG_STRUCT => {
            let name = read_name(data, pos)?;
            let field_count = read_u16(data, pos)?;
            // Allocate struct: contiguous u64 fields (no header).
            let total = (field_count as usize) * 8;
            let ptr = if total > 0 {
                crate::gc::mesh_gc_alloc_actor(total as u64, 8)
            } else {
                std::ptr::null_mut()
            };
            let mut fields = Vec::with_capacity(field_count as usize);
            for i in 0..field_count as usize {
                let field_name = read_name(data, pos)?;
                let (val, ft) = stf_decode(data, pos)?;
                if !ptr.is_null() {
                    unsafe {
                        *((ptr as *mut u64).add(i)) = val;
                    }
                }
                fields.push((field_name, ft));
            }
            Ok((ptr as u64, StfType::Struct(name, fields)))
        }

        TAG_SUM_TYPE => {
            let type_name = read_name(data, pos)?;
            let variant_tag = read_u8(data, pos)?;
            let field_count = read_u16(data, pos)?;
            // Allocate sum type layout: { u8 tag at offset 0, fields at offset 8 }
            let total = 8 + (field_count as usize) * 8;
            let ptr = crate::gc::mesh_gc_alloc_actor(total as u64, 8);
            unsafe {
                *(ptr as *mut u8) = variant_tag;
            }
            let fields_ptr = unsafe { (ptr as *mut u64).add(1) };
            let mut field_types = Vec::with_capacity(field_count as usize);
            for i in 0..field_count as usize {
                let (val, ft) = stf_decode(data, pos)?;
                unsafe {
                    *fields_ptr.add(i) = val;
                }
                field_types.push(ft);
            }
            // Build a variants list with this variant's fields at the correct index.
            let mut variants = Vec::new();
            for idx in 0..=variant_tag {
                if idx == variant_tag {
                    variants.push((std::string::String::new(), field_types.clone()));
                } else {
                    variants.push((std::string::String::new(), Vec::new()));
                }
            }
            Ok((ptr as u64, StfType::SumType(type_name, variants)))
        }

        // ── Option/Result tags ────────────────────────────────────
        TAG_OPTION_SOME => {
            let (inner_val, inner_type) = stf_decode(data, pos)?;
            let opt_ptr = crate::option::alloc_option(0, inner_val as *mut u8);
            Ok((opt_ptr as u64, StfType::OptionOf(Box::new(inner_type))))
        }

        TAG_OPTION_NONE => {
            let opt_ptr = crate::option::alloc_option(1, std::ptr::null_mut());
            Ok((opt_ptr as u64, StfType::OptionOf(Box::new(StfType::Unit))))
        }

        TAG_RESULT_OK => {
            let (inner_val, inner_type) = stf_decode(data, pos)?;
            let ptr = crate::option::alloc_option(0, inner_val as *mut u8);
            Ok((
                ptr as u64,
                StfType::ResultOf(Box::new(inner_type), Box::new(StfType::Unit)),
            ))
        }

        TAG_RESULT_ERR => {
            let (inner_val, inner_type) = stf_decode(data, pos)?;
            let ptr = crate::option::alloc_option(1, inner_val as *mut u8);
            Ok((
                ptr as u64,
                StfType::ResultOf(Box::new(StfType::Unit), Box::new(inner_type)),
            ))
        }

        unknown => Err(StfError::InvalidTag(unknown)),
    }
}

/// Decode a complete STF payload (with version header).
///
/// Validates the version byte, then decodes the value.
///
/// # Safety
///
/// Same safety requirements as [`stf_decode`].
pub fn stf_decode_value(data: &[u8]) -> Result<(u64, StfType), StfError> {
    if data.is_empty() {
        return Err(StfError::UnexpectedEof);
    }
    if data[0] != STF_VERSION {
        return Err(StfError::InvalidVersion(data[0]));
    }
    let mut pos = 1;
    stf_decode(data, &mut pos)
}

// ── Tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gc::mesh_rt_init;

    #[test]
    fn test_int_roundtrip() {
        mesh_rt_init();
        let values: &[i64] = &[-1, 0, i64::MAX];
        for &v in values {
            let encoded = stf_encode_value(v as u64, &StfType::Int).unwrap();
            assert_eq!(encoded[0], STF_VERSION, "version byte");
            let (decoded, typ) = stf_decode_value(&encoded).unwrap();
            assert_eq!(typ, StfType::Int);
            assert_eq!(decoded as i64, v, "round-trip failed for {}", v);
        }
    }

    #[test]
    fn test_float_roundtrip() {
        mesh_rt_init();
        let values: &[f64] = &[3.14, -0.0, f64::INFINITY, f64::NAN];
        for &v in values {
            let bits = v.to_bits();
            let encoded = stf_encode_value(bits, &StfType::Float).unwrap();
            assert_eq!(encoded[0], STF_VERSION, "version byte");
            let (decoded, typ) = stf_decode_value(&encoded).unwrap();
            assert_eq!(typ, StfType::Float);
            // Compare bits, not f64 values (NaN != NaN).
            assert_eq!(decoded, bits, "round-trip failed for {} (bits)", v);
        }
    }

    #[test]
    fn test_bool_roundtrip() {
        mesh_rt_init();
        // true
        let encoded = stf_encode_value(1, &StfType::Bool).unwrap();
        let (val, typ) = stf_decode_value(&encoded).unwrap();
        assert_eq!(typ, StfType::Bool);
        assert_eq!(val, 1);

        // false
        let encoded = stf_encode_value(0, &StfType::Bool).unwrap();
        let (val, typ) = stf_decode_value(&encoded).unwrap();
        assert_eq!(typ, StfType::Bool);
        assert_eq!(val, 0);
    }

    #[test]
    fn test_string_roundtrip() {
        mesh_rt_init();
        let test_str = "hello";
        let mesh_str = mesh_string_new(test_str.as_ptr(), test_str.len() as u64);
        let encoded = stf_encode_value(mesh_str as u64, &StfType::String).unwrap();
        assert_eq!(encoded[0], STF_VERSION, "version byte");
        let (decoded_ptr, typ) = stf_decode_value(&encoded).unwrap();
        assert_eq!(typ, StfType::String);
        unsafe {
            let decoded_str = &*(decoded_ptr as *const MeshString);
            assert_eq!(decoded_str.as_str(), "hello");
        }
    }

    #[test]
    fn test_unit_roundtrip() {
        mesh_rt_init();
        let encoded = stf_encode_value(0, &StfType::Unit).unwrap();
        assert_eq!(encoded[0], STF_VERSION, "version byte");
        let (val, typ) = stf_decode_value(&encoded).unwrap();
        assert_eq!(typ, StfType::Unit);
        assert_eq!(val, 0);
    }

    #[test]
    fn test_pid_roundtrip() {
        mesh_rt_init();
        // A PID with node_id=5, creation=2, local_id=42
        let raw_pid: u64 = (5u64 << 48) | (2u64 << 40) | 42;
        let encoded = stf_encode_value(raw_pid, &StfType::Pid).unwrap();
        assert_eq!(encoded[0], STF_VERSION, "version byte");
        let (decoded, typ) = stf_decode_value(&encoded).unwrap();
        assert_eq!(typ, StfType::Pid);
        assert_eq!(decoded, raw_pid, "PID round-trip mismatch");
    }

    #[test]
    fn test_closure_rejected() {
        let result = stf_encode_value(0, &StfType::Closure);
        assert_eq!(result, Err(StfError::ClosureNotSerializable));
    }

    #[test]
    fn test_fnptr_rejected() {
        let result = stf_encode_value(0, &StfType::FnPtr);
        assert_eq!(result, Err(StfError::ClosureNotSerializable));
    }

    #[test]
    fn test_truncated_int_decode() {
        // Version byte + TAG_INT + only 4 bytes of payload (need 8).
        let mut buf = vec![STF_VERSION, TAG_INT, 0, 0, 0, 0];
        let result = stf_decode_value(&buf);
        assert_eq!(result, Err(StfError::UnexpectedEof));

        // Also test completely empty after tag.
        buf = vec![STF_VERSION, TAG_INT];
        let result = stf_decode_value(&buf);
        assert_eq!(result, Err(StfError::UnexpectedEof));
    }

    #[test]
    fn test_version_check() {
        // Wrong version byte.
        let buf = vec![99, TAG_INT, 0, 0, 0, 0, 0, 0, 0, 0];
        let result = stf_decode_value(&buf);
        assert_eq!(result, Err(StfError::InvalidVersion(99)));

        // Empty buffer.
        let result = stf_decode_value(&[]);
        assert_eq!(result, Err(StfError::UnexpectedEof));
    }

    // ── Container round-trip tests ────────────────────────────────

    /// Helper: allocate a list of i64 values on the GC heap.
    fn alloc_list_of_ints(values: &[i64]) -> *mut u8 {
        let count = values.len() as u64;
        let total = 16 + (count as usize) * 8; // header + data
        let ptr = crate::gc::mesh_gc_alloc_actor(total as u64, 8);
        unsafe {
            *(ptr as *mut u64) = count; // len
            *((ptr as *mut u64).add(1)) = count; // cap
            let data = (ptr as *mut u64).add(2);
            for (i, &v) in values.iter().enumerate() {
                *data.add(i) = v as u64;
            }
        }
        ptr
    }

    /// Helper: allocate a set of i64 values on the GC heap.
    fn alloc_set_of_ints(values: &[i64]) -> *mut u8 {
        let count = values.len() as u64;
        let total = 16 + (count as usize) * 8;
        let ptr = crate::gc::mesh_gc_alloc_actor(total as u64, 8);
        unsafe {
            *(ptr as *mut u64) = count; // len
            *((ptr as *mut u64).add(1)) = count; // cap
            let data = (ptr as *mut u64).add(2);
            for (i, &v) in values.iter().enumerate() {
                *data.add(i) = v as u64;
            }
        }
        ptr
    }

    #[test]
    fn test_list_int_roundtrip() {
        mesh_rt_init();
        let list = alloc_list_of_ints(&[10, 20, 30]);
        let ty = StfType::List(Box::new(StfType::Int));
        let encoded = stf_encode_value(list as u64, &ty).unwrap();
        let (decoded_ptr, decoded_type) = stf_decode_value(&encoded).unwrap();
        assert_eq!(decoded_type, StfType::List(Box::new(StfType::Int)));
        unsafe {
            let ptr = decoded_ptr as *const u64;
            let len = *ptr;
            assert_eq!(len, 3);
            let data = ptr.add(2);
            assert_eq!(*data as i64, 10);
            assert_eq!(*data.add(1) as i64, 20);
            assert_eq!(*data.add(2) as i64, 30);
        }
    }

    #[test]
    fn test_list_string_roundtrip() {
        mesh_rt_init();
        let s1 = mesh_string_new("hello".as_ptr(), 5);
        let s2 = mesh_string_new("world".as_ptr(), 5);
        // Allocate list of 2 string pointers.
        let list = crate::gc::mesh_gc_alloc_actor(16 + 2 * 8, 8);
        unsafe {
            *(list as *mut u64) = 2; // len
            *((list as *mut u64).add(1)) = 2; // cap
            *((list as *mut u64).add(2)) = s1 as u64; // data[0]
            *((list as *mut u64).add(3)) = s2 as u64; // data[1]
        }
        let ty = StfType::List(Box::new(StfType::String));
        let encoded = stf_encode_value(list as u64, &ty).unwrap();
        let (decoded_ptr, decoded_type) = stf_decode_value(&encoded).unwrap();
        assert_eq!(decoded_type, StfType::List(Box::new(StfType::String)));
        unsafe {
            let ptr = decoded_ptr as *const u64;
            assert_eq!(*ptr, 2); // len
            let d0 = *ptr.add(2) as *const MeshString;
            let d1 = *ptr.add(3) as *const MeshString;
            assert_eq!((*d0).as_str(), "hello");
            assert_eq!((*d1).as_str(), "world");
        }
    }

    #[test]
    fn test_map_roundtrip() {
        mesh_rt_init();
        // Create a map with 2 int->string entries.
        let v1 = mesh_string_new("alpha".as_ptr(), 5);
        let v2 = mesh_string_new("beta".as_ptr(), 4);
        // Map layout: { len, cap|key_type, entries: [(key, val), ...] }
        let total = 16 + 2 * 16; // header + 2 entries of 16 bytes
        let map = crate::gc::mesh_gc_alloc_actor(total as u64, 8);
        unsafe {
            *(map as *mut u64) = 2; // len
            *((map as *mut u64).add(1)) = (0u64 << 56) | 2; // key_type=0 (int), cap=2
            let entries = (map as *mut u64).add(2);
            *entries = 1; // key[0]
            *entries.add(1) = v1 as u64; // val[0]
            *entries.add(2) = 2; // key[1]
            *entries.add(3) = v2 as u64; // val[1]
        }
        let ty = StfType::Map(Box::new(StfType::Int), Box::new(StfType::String));
        let encoded = stf_encode_value(map as u64, &ty).unwrap();
        let (decoded_ptr, decoded_type) = stf_decode_value(&encoded).unwrap();
        assert_eq!(
            decoded_type,
            StfType::Map(Box::new(StfType::Int), Box::new(StfType::String))
        );
        unsafe {
            let ptr = decoded_ptr as *const u64;
            assert_eq!(*ptr, 2); // len
            let entries = ptr.add(2);
            assert_eq!(*entries, 1); // key[0]
            assert_eq!(*entries.add(2), 2); // key[1]
            let dv0 = *entries.add(1) as *const MeshString;
            let dv1 = *entries.add(3) as *const MeshString;
            assert_eq!((*dv0).as_str(), "alpha");
            assert_eq!((*dv1).as_str(), "beta");
        }
    }

    #[test]
    fn test_set_roundtrip() {
        mesh_rt_init();
        let set = alloc_set_of_ints(&[100, 200, 300]);
        let ty = StfType::Set(Box::new(StfType::Int));
        let encoded = stf_encode_value(set as u64, &ty).unwrap();
        let (decoded_ptr, decoded_type) = stf_decode_value(&encoded).unwrap();
        assert_eq!(decoded_type, StfType::Set(Box::new(StfType::Int)));
        unsafe {
            let ptr = decoded_ptr as *const u64;
            assert_eq!(*ptr, 3); // len
            let data = ptr.add(2);
            assert_eq!(*data as i64, 100);
            assert_eq!(*data.add(1) as i64, 200);
            assert_eq!(*data.add(2) as i64, 300);
        }
    }

    #[test]
    fn test_tuple_roundtrip() {
        mesh_rt_init();
        let s = mesh_string_new("hi".as_ptr(), 2);
        // Tuple layout: { len: u64, data: [u64; len] }
        let total = 8 + 3 * 8;
        let tuple = crate::gc::mesh_gc_alloc_actor(total as u64, 8);
        unsafe {
            *(tuple as *mut u64) = 3; // len/arity
            *((tuple as *mut u64).add(1)) = 42u64; // Int
            *((tuple as *mut u64).add(2)) = s as u64; // String
            *((tuple as *mut u64).add(3)) = 1u64; // Bool (true)
        }
        let ty = StfType::Tuple(vec![StfType::Int, StfType::String, StfType::Bool]);
        let encoded = stf_encode_value(tuple as u64, &ty).unwrap();
        let (decoded_ptr, decoded_type) = stf_decode_value(&encoded).unwrap();
        assert_eq!(
            decoded_type,
            StfType::Tuple(vec![StfType::Int, StfType::String, StfType::Bool])
        );
        unsafe {
            let ptr = decoded_ptr as *const u64;
            assert_eq!(*ptr, 3); // arity
            let data = ptr.add(1);
            assert_eq!(*data as i64, 42);
            let decoded_str = &*(*data.add(1) as *const MeshString);
            assert_eq!(decoded_str.as_str(), "hi");
            assert_eq!(*data.add(2), 1); // true
        }
    }

    // ── Composite round-trip tests ────────────────────────────────

    #[test]
    fn test_struct_roundtrip() {
        mesh_rt_init();
        let name_str = mesh_string_new("Alice".as_ptr(), 5);
        // Struct is contiguous u64 fields (no header): [name_ptr, age]
        let total = 2 * 8;
        let s = crate::gc::mesh_gc_alloc_actor(total as u64, 8);
        unsafe {
            *(s as *mut u64) = name_str as u64; // field 0: name
            *((s as *mut u64).add(1)) = 30u64; // field 1: age
        }
        let ty = StfType::Struct(
            "Person".to_string(),
            vec![
                ("name".to_string(), StfType::String),
                ("age".to_string(), StfType::Int),
            ],
        );
        let encoded = stf_encode_value(s as u64, &ty).unwrap();
        let (decoded_ptr, decoded_type) = stf_decode_value(&encoded).unwrap();
        match &decoded_type {
            StfType::Struct(name, fields) => {
                assert_eq!(name, "Person");
                assert_eq!(fields.len(), 2);
                assert_eq!(fields[0].0, "name");
                assert_eq!(fields[1].0, "age");
            }
            _ => panic!("expected Struct type"),
        }
        unsafe {
            let ptr = decoded_ptr as *const u64;
            let decoded_name = &*(*ptr as *const MeshString);
            assert_eq!(decoded_name.as_str(), "Alice");
            assert_eq!(*ptr.add(1) as i64, 30);
        }
    }

    #[test]
    fn test_option_some_roundtrip() {
        mesh_rt_init();
        // Create MeshOption with tag=0 (Some), value pointing to int 42.
        // For Option<Int>, the inner value is the i64 itself cast as a pointer.
        let opt_ptr = crate::option::alloc_option(0, 42u64 as *mut u8);
        let ty = StfType::OptionOf(Box::new(StfType::Int));
        let encoded = stf_encode_value(opt_ptr as u64, &ty).unwrap();
        let (decoded_ptr, decoded_type) = stf_decode_value(&encoded).unwrap();
        match &decoded_type {
            StfType::OptionOf(inner) => assert_eq!(**inner, StfType::Int),
            _ => panic!("expected OptionOf"),
        }
        unsafe {
            let opt = &*(decoded_ptr as *const crate::option::MeshOption);
            assert_eq!(opt.tag, 0); // Some
            assert_eq!(opt.value as u64, 42);
        }
    }

    #[test]
    fn test_option_none_roundtrip() {
        mesh_rt_init();
        let opt_ptr = crate::option::alloc_option(1, std::ptr::null_mut());
        let ty = StfType::OptionOf(Box::new(StfType::Int));
        let encoded = stf_encode_value(opt_ptr as u64, &ty).unwrap();
        let (decoded_ptr, _decoded_type) = stf_decode_value(&encoded).unwrap();
        unsafe {
            let opt = &*(decoded_ptr as *const crate::option::MeshOption);
            assert_eq!(opt.tag, 1); // None
            assert!(opt.value.is_null());
        }
    }

    #[test]
    fn test_result_ok_roundtrip() {
        mesh_rt_init();
        // Result<Int, String> with Ok(99)
        let res_ptr = crate::option::alloc_option(0, 99u64 as *mut u8);
        let ty = StfType::ResultOf(Box::new(StfType::Int), Box::new(StfType::String));
        let encoded = stf_encode_value(res_ptr as u64, &ty).unwrap();
        let (decoded_ptr, _decoded_type) = stf_decode_value(&encoded).unwrap();
        unsafe {
            let opt = &*(decoded_ptr as *const crate::option::MeshOption);
            assert_eq!(opt.tag, 0); // Ok
            assert_eq!(opt.value as u64, 99);
        }
    }

    #[test]
    fn test_result_err_roundtrip() {
        mesh_rt_init();
        // Result<Int, String> with Err("oops")
        let err_str = mesh_string_new("oops".as_ptr(), 4);
        let res_ptr = crate::option::alloc_option(1, err_str as *mut u8);
        let ty = StfType::ResultOf(Box::new(StfType::Int), Box::new(StfType::String));
        let encoded = stf_encode_value(res_ptr as u64, &ty).unwrap();
        let (decoded_ptr, _decoded_type) = stf_decode_value(&encoded).unwrap();
        unsafe {
            let opt = &*(decoded_ptr as *const crate::option::MeshOption);
            assert_eq!(opt.tag, 1); // Err
            let decoded_str = &*(opt.value as *const MeshString);
            assert_eq!(decoded_str.as_str(), "oops");
        }
    }

    // ── Nesting tests ─────────────────────────────────────────────

    #[test]
    fn test_nested_list_of_lists() {
        mesh_rt_init();
        // Create 2 inner lists of ints.
        let inner1 = alloc_list_of_ints(&[1, 2]);
        let inner2 = alloc_list_of_ints(&[3, 4, 5]);
        // Outer list holds 2 list pointers.
        let outer = crate::gc::mesh_gc_alloc_actor(16 + 2 * 8, 8);
        unsafe {
            *(outer as *mut u64) = 2;
            *((outer as *mut u64).add(1)) = 2;
            *((outer as *mut u64).add(2)) = inner1 as u64;
            *((outer as *mut u64).add(3)) = inner2 as u64;
        }
        let ty = StfType::List(Box::new(StfType::List(Box::new(StfType::Int))));
        let encoded = stf_encode_value(outer as u64, &ty).unwrap();
        let (decoded_ptr, _) = stf_decode_value(&encoded).unwrap();
        unsafe {
            let ptr = decoded_ptr as *const u64;
            assert_eq!(*ptr, 2); // outer len
            let d0 = *ptr.add(2) as *const u64; // inner list 0
            let d1 = *ptr.add(3) as *const u64; // inner list 1
            assert_eq!(*d0, 2); // inner list 0 len
            assert_eq!(*d0.add(2) as i64, 1);
            assert_eq!(*d0.add(3) as i64, 2);
            assert_eq!(*d1, 3); // inner list 1 len
            assert_eq!(*d1.add(2) as i64, 3);
            assert_eq!(*d1.add(3) as i64, 4);
            assert_eq!(*d1.add(4) as i64, 5);
        }
    }

    #[test]
    fn test_list_of_maps() {
        mesh_rt_init();
        // Create a map with 1 int->int entry: {10 => 20}
        let map1 = crate::gc::mesh_gc_alloc_actor(16 + 1 * 16, 8);
        unsafe {
            *(map1 as *mut u64) = 1; // len
            *((map1 as *mut u64).add(1)) = (0u64 << 56) | 1; // key_type=0, cap=1
            *((map1 as *mut u64).add(2)) = 10; // key
            *((map1 as *mut u64).add(3)) = 20; // val
        }
        // Create a map with 1 int->int entry: {30 => 40}
        let map2 = crate::gc::mesh_gc_alloc_actor(16 + 1 * 16, 8);
        unsafe {
            *(map2 as *mut u64) = 1;
            *((map2 as *mut u64).add(1)) = (0u64 << 56) | 1;
            *((map2 as *mut u64).add(2)) = 30;
            *((map2 as *mut u64).add(3)) = 40;
        }
        // Outer list of 2 maps.
        let outer = crate::gc::mesh_gc_alloc_actor(16 + 2 * 8, 8);
        unsafe {
            *(outer as *mut u64) = 2;
            *((outer as *mut u64).add(1)) = 2;
            *((outer as *mut u64).add(2)) = map1 as u64;
            *((outer as *mut u64).add(3)) = map2 as u64;
        }
        let ty = StfType::List(Box::new(StfType::Map(
            Box::new(StfType::Int),
            Box::new(StfType::Int),
        )));
        let encoded = stf_encode_value(outer as u64, &ty).unwrap();
        let (decoded_ptr, _) = stf_decode_value(&encoded).unwrap();
        unsafe {
            let ptr = decoded_ptr as *const u64;
            assert_eq!(*ptr, 2); // outer len
                                 // Check first decoded map
            let dm0 = *ptr.add(2) as *const u64;
            assert_eq!(*dm0, 1); // map0 len
            assert_eq!(*dm0.add(2), 10); // key
            assert_eq!(*dm0.add(3), 20); // val
                                         // Check second decoded map
            let dm1 = *ptr.add(3) as *const u64;
            assert_eq!(*dm1, 1); // map1 len
            assert_eq!(*dm1.add(2), 30);
            assert_eq!(*dm1.add(3), 40);
        }
    }

    // ── Error condition tests ─────────────────────────────────────

    #[test]
    fn test_collection_too_large() {
        mesh_rt_init();
        // Craft a payload: version + TAG_LIST + count > MAX_COLLECTION_LEN
        let big_count: u32 = MAX_COLLECTION_LEN + 1;
        let mut buf = vec![STF_VERSION, TAG_LIST];
        buf.extend_from_slice(&big_count.to_le_bytes());
        let result = stf_decode_value(&buf);
        assert_eq!(result, Err(StfError::PayloadTooLarge(big_count)));
    }

    #[test]
    fn test_string_too_large() {
        mesh_rt_init();
        // Craft a payload: version + TAG_STRING + len > MAX_STRING_LEN
        let big_len: u32 = MAX_STRING_LEN + 1;
        let mut buf = vec![STF_VERSION, TAG_STRING];
        buf.extend_from_slice(&big_len.to_le_bytes());
        let result = stf_decode_value(&buf);
        assert_eq!(result, Err(StfError::PayloadTooLarge(big_len)));
    }

    #[test]
    fn test_unknown_tag() {
        mesh_rt_init();
        let buf = vec![STF_VERSION, 0xFE];
        let result = stf_decode_value(&buf);
        assert_eq!(result, Err(StfError::InvalidTag(0xFE)));
    }

    // ── Empty container tests ─────────────────────────────────────

    #[test]
    fn test_empty_list_roundtrip() {
        mesh_rt_init();
        let list = alloc_list_of_ints(&[]);
        let ty = StfType::List(Box::new(StfType::Int));
        let encoded = stf_encode_value(list as u64, &ty).unwrap();
        let (decoded_ptr, _) = stf_decode_value(&encoded).unwrap();
        unsafe {
            assert_eq!(*(decoded_ptr as *const u64), 0);
        }
    }

    #[test]
    fn test_empty_map_roundtrip() {
        mesh_rt_init();
        let map = crate::gc::mesh_gc_alloc_actor(16, 8);
        unsafe {
            *(map as *mut u64) = 0;
            *((map as *mut u64).add(1)) = 0;
        }
        let ty = StfType::Map(Box::new(StfType::Int), Box::new(StfType::Int));
        let encoded = stf_encode_value(map as u64, &ty).unwrap();
        let (decoded_ptr, _) = stf_decode_value(&encoded).unwrap();
        unsafe {
            assert_eq!(*(decoded_ptr as *const u64), 0);
        }
    }
}
