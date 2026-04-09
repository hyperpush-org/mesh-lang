//! WebSocket protocol layer (RFC 6455).
//!
//! Provides the complete low-level WebSocket wire protocol implementation:
//! - **Frame codec** (`frame`): Variable-length frame parsing and writing with XOR masking
//! - **Handshake** (`handshake`): HTTP upgrade with Sec-WebSocket-Accept validation
//! - **Close** (`close`): Close handshake, text UTF-8 validation, and protocol-level frame dispatch

pub mod close;
pub mod frame;
pub mod handshake;
pub mod rooms;
pub mod server;

pub use close::{
    build_close_payload, parse_close_payload, process_frame, send_close, validate_text_payload,
    WsCloseCode,
};
pub use frame::{apply_mask, read_frame, write_frame, WsFrame, WsOpcode};
pub use handshake::{perform_upgrade, write_bad_request};
pub use rooms::global_room_registry;
pub use server::{WS_BINARY_TAG, WS_CONNECT_TAG, WS_DISCONNECT_TAG, WS_TEXT_TAG};
