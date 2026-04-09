//! Runtime panic handler for Mesh programs.
//!
//! Called when a Mesh program encounters an unrecoverable error at runtime,
//! such as a non-exhaustive match failure (guarded arms edge case).
//!
//! Uses `panic!()` rather than `abort()` so that actor crash isolation
//! via `catch_unwind` can intercept handler failures without bringing
//! down the entire process.

/// Panic with a source-located error message.
///
/// Triggers a Rust `panic!()` with a formatted source-located error
/// message. In actor contexts, this is caught by `catch_unwind` for
/// crash isolation. Outside actors, the panic unwinds and terminates
/// the process as usual.
///
/// # Safety
///
/// `msg` must point to `msg_len` valid UTF-8 bytes.
/// `file` must point to `file_len` valid UTF-8 bytes.
#[no_mangle]
pub extern "C" fn mesh_panic(
    msg: *const u8,
    msg_len: u64,
    file: *const u8,
    file_len: u64,
    line: u32,
) -> ! {
    unsafe {
        let msg = std::str::from_utf8_unchecked(std::slice::from_raw_parts(msg, msg_len as usize));
        let file =
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(file, file_len as usize));
        panic!("Mesh panic at {}:{}: {}", file, line, msg);
    }
}
