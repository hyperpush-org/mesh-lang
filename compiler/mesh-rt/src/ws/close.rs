//! WebSocket close handshake and frame validation (RFC 6455 Section 5.5.1, 7).
//!
//! Provides close frame parsing/building, text frame UTF-8 validation, and a
//! convenience `process_frame` function that handles protocol-level frame
//! dispatch (ping/pong, close echo, text validation).
//!
//! - [`parse_close_payload`]: Extract status code + reason from close frame payload
//! - [`build_close_payload`]: Build a close frame payload from code + reason
//! - [`validate_text_payload`]: UTF-8 validation for text frames (PROTO-05)
//! - [`send_close`]: Send a close frame with a given status code and reason
//! - [`process_frame`]: Handle one frame at the protocol level

use std::io::Write;

use super::frame::{write_frame, WsFrame, WsOpcode};

/// Well-known WebSocket close status codes per RFC 6455 Section 7.4.1.
pub struct WsCloseCode;

impl WsCloseCode {
    /// Normal closure (1000).
    pub const NORMAL: u16 = 1000;
    /// Going away (1001).
    pub const GOING_AWAY: u16 = 1001;
    /// Protocol error (1002) -- used for unknown opcodes (PROTO-09).
    pub const PROTOCOL_ERROR: u16 = 1002;
    /// Invalid frame payload data (1007) -- used for UTF-8 failure (PROTO-05).
    pub const INVALID_DATA: u16 = 1007;
    /// Message too big (1009) -- used when fragmented message exceeds 16 MiB (FRAG-03).
    pub const MESSAGE_TOO_BIG: u16 = 1009;
    /// Internal server error (1011) -- used when an actor crashes (Phase 60).
    pub const INTERNAL_ERROR: u16 = 1011;
}

/// Parse a close frame payload into (status_code, reason).
///
/// Per RFC 6455 Section 7.4.1:
/// - If payload >= 2 bytes: status code is the first 2 bytes (big-endian),
///   reason is the remaining bytes decoded as UTF-8 (lossy).
/// - If payload < 2 bytes: returns (1005, "") -- 1005 means "no status code present".
pub fn parse_close_payload(payload: &[u8]) -> (u16, String) {
    if payload.len() >= 2 {
        let code = u16::from_be_bytes([payload[0], payload[1]]);
        let reason = String::from_utf8_lossy(&payload[2..]).into_owned();
        (code, reason)
    } else {
        (1005, String::new())
    }
}

/// Build a close frame payload from a status code and reason string.
///
/// The payload is 2 bytes for the code (big-endian) followed by the reason
/// bytes. The reason is truncated to 123 bytes max so the total payload
/// stays within the 125-byte control frame limit (RFC 6455 Section 5.5).
pub fn build_close_payload(code: u16, reason: &str) -> Vec<u8> {
    let reason_bytes = reason.as_bytes();
    let max_reason_len = 123; // 125 - 2 bytes for code
    let truncated_len = reason_bytes.len().min(max_reason_len);

    let mut payload = Vec::with_capacity(2 + truncated_len);
    payload.extend_from_slice(&code.to_be_bytes());
    payload.extend_from_slice(&reason_bytes[..truncated_len]);
    payload
}

/// Validate that a text frame payload is valid UTF-8.
///
/// Per RFC 6455 Section 5.6, text frames MUST contain valid UTF-8.
/// Invalid UTF-8 triggers close code 1007 (PROTO-05).
pub fn validate_text_payload(payload: &[u8]) -> Result<(), ()> {
    std::str::from_utf8(payload).map(|_| ()).map_err(|_| ())
}

/// Send a close frame with the given status code and reason.
///
/// Builds the close payload and writes it as a close frame using the frame codec.
pub fn send_close<W: Write>(writer: &mut W, code: u16, reason: &str) -> Result<(), String> {
    let payload = build_close_payload(code, reason);
    write_frame(writer, WsOpcode::Close, &payload, true)
}

/// Process one WebSocket frame at the protocol level.
///
/// Handles control frames (ping, pong, close) and validates data frames
/// (text UTF-8 check). Returns:
/// - `Ok(Some(frame))` -- a data frame ready for the application (text, binary, continuation)
/// - `Ok(None)` -- a control frame that was handled internally (pong sent, pong received)
/// - `Err(msg)` -- protocol error or close; the connection should be terminated
pub fn process_frame<S: Write>(stream: &mut S, frame: WsFrame) -> Result<Option<WsFrame>, String> {
    match frame.opcode {
        WsOpcode::Text => {
            if validate_text_payload(&frame.payload).is_err() {
                send_close(stream, WsCloseCode::INVALID_DATA, "invalid UTF-8")?;
                return Err("invalid UTF-8 in text frame".to_string());
            }
            Ok(Some(frame))
        }
        WsOpcode::Binary => Ok(Some(frame)),
        WsOpcode::Close => {
            let (code, _reason) = parse_close_payload(&frame.payload);
            // Echo the close frame back with the same status code
            let echo_payload = build_close_payload(code, "");
            write_frame(stream, WsOpcode::Close, &echo_payload, true)?;
            Err("close".to_string())
        }
        WsOpcode::Ping => {
            // Respond with a Pong carrying the same payload
            write_frame(stream, WsOpcode::Pong, &frame.payload, true)?;
            Ok(None)
        }
        WsOpcode::Pong => {
            // Ignore -- Phase 61 will use Pong for heartbeat tracking
            Ok(None)
        }
        WsOpcode::Continuation => {
            // Recognized but not assembled -- Phase 61 handles fragmentation
            Ok(Some(frame))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ws::frame::read_frame;
    use std::io::Cursor;

    #[test]
    fn test_parse_close_normal() {
        // payload [0x03, 0xE8, b'o', b'k'] -> (1000, "ok")
        let payload = vec![0x03, 0xE8, b'o', b'k'];
        let (code, reason) = parse_close_payload(&payload);
        assert_eq!(code, 1000);
        assert_eq!(reason, "ok");
    }

    #[test]
    fn test_parse_close_empty() {
        // empty payload -> (1005, "")
        let (code, reason) = parse_close_payload(&[]);
        assert_eq!(code, 1005);
        assert_eq!(reason, "");
    }

    #[test]
    fn test_parse_close_code_only() {
        // payload [0x03, 0xE8] -> (1000, "")
        let payload = vec![0x03, 0xE8];
        let (code, reason) = parse_close_payload(&payload);
        assert_eq!(code, 1000);
        assert_eq!(reason, "");
    }

    #[test]
    fn test_build_close_payload() {
        let payload = build_close_payload(1000, "bye");
        assert_eq!(payload, vec![0x03, 0xE8, b'b', b'y', b'e']);
    }

    #[test]
    fn test_build_close_truncates_reason() {
        let long_reason = "x".repeat(200);
        let payload = build_close_payload(1000, &long_reason);
        assert_eq!(
            payload.len(),
            125,
            "payload should be capped at 125 bytes (2 + 123)"
        );
        assert_eq!(&payload[..2], &[0x03, 0xE8]);
    }

    #[test]
    fn test_validate_text_valid_utf8() {
        assert!(validate_text_payload(b"Hello").is_ok());
    }

    #[test]
    fn test_validate_text_invalid_utf8() {
        assert!(validate_text_payload(&[0xFF, 0xFE]).is_err());
    }

    #[test]
    fn test_process_text_frame() {
        let frame = WsFrame {
            fin: true,
            opcode: WsOpcode::Text,
            payload: b"Hello".to_vec(),
        };
        let mut writer = Vec::new();
        let result = process_frame(&mut writer, frame);
        assert!(result.is_ok());
        let opt = result.unwrap();
        assert!(opt.is_some());
        let f = opt.unwrap();
        assert_eq!(f.opcode, WsOpcode::Text);
        assert_eq!(f.payload, b"Hello");
    }

    #[test]
    fn test_process_binary_frame() {
        let frame = WsFrame {
            fin: true,
            opcode: WsOpcode::Binary,
            payload: vec![0x01, 0x02, 0x03],
        };
        let mut writer = Vec::new();
        let result = process_frame(&mut writer, frame);
        assert!(result.is_ok());
        let f = result.unwrap().unwrap();
        assert_eq!(f.opcode, WsOpcode::Binary);
        assert_eq!(f.payload, vec![0x01, 0x02, 0x03]);
    }

    #[test]
    fn test_process_close_frame() {
        // Create a close frame with code 1000
        let close_payload = build_close_payload(1000, "goodbye");
        let frame = WsFrame {
            fin: true,
            opcode: WsOpcode::Close,
            payload: close_payload,
        };
        let mut writer = Vec::new();
        let result = process_frame(&mut writer, frame);
        assert!(
            result.is_err(),
            "close frame should return Err to signal connection end"
        );
        assert_eq!(result.unwrap_err(), "close");

        // Verify the echoed close frame was written
        assert!(
            !writer.is_empty(),
            "should have written an echo close frame"
        );
        let mut cursor = Cursor::new(writer);
        let echo_frame = read_frame(&mut cursor).unwrap();
        assert_eq!(echo_frame.opcode, WsOpcode::Close);
        let (code, _) = parse_close_payload(&echo_frame.payload);
        assert_eq!(code, 1000);
    }

    #[test]
    fn test_process_ping_sends_pong() {
        let frame = WsFrame {
            fin: true,
            opcode: WsOpcode::Ping,
            payload: b"ping".to_vec(),
        };
        let mut writer = Vec::new();
        let result = process_frame(&mut writer, frame);
        assert!(result.is_ok());
        assert!(
            result.unwrap().is_none(),
            "ping should return None (handled internally)"
        );

        // Verify a pong frame was written with the same payload
        assert!(!writer.is_empty(), "should have written a pong frame");
        let mut cursor = Cursor::new(writer);
        let pong_frame = read_frame(&mut cursor).unwrap();
        assert_eq!(pong_frame.opcode, WsOpcode::Pong);
        assert_eq!(pong_frame.payload, b"ping");
    }

    #[test]
    fn test_process_invalid_utf8_text() {
        let frame = WsFrame {
            fin: true,
            opcode: WsOpcode::Text,
            payload: vec![0xFF, 0xFE],
        };
        let mut writer = Vec::new();
        let result = process_frame(&mut writer, frame);
        assert!(result.is_err(), "invalid UTF-8 should return Err");

        // Verify close frame with code 1007 was sent
        assert!(!writer.is_empty(), "should have sent a close frame");
        let mut cursor = Cursor::new(writer);
        let close_frame = read_frame(&mut cursor).unwrap();
        assert_eq!(close_frame.opcode, WsOpcode::Close);
        let (code, reason) = parse_close_payload(&close_frame.payload);
        assert_eq!(
            code,
            WsCloseCode::INVALID_DATA,
            "should send close code 1007"
        );
        assert_eq!(reason, "invalid UTF-8");
    }
}
