//! WebSocket frame codec (RFC 6455 Section 5.2-5.3).
//!
//! Provides the low-level frame parser and writer for the WebSocket wire
//! protocol. Frames are the smallest unit of WebSocket communication.
//!
//! - [`read_frame`]: Parse a single frame from a byte stream (handles masking)
//! - [`write_frame`]: Write an unmasked server frame to a byte stream
//! - [`apply_mask`]: Symmetric XOR masking per RFC 6455 Section 5.3

use std::io::{Read, Write};

/// Maximum payload size (16 MiB production limit) to prevent OOM from malicious lengths.
const MAX_PAYLOAD_SIZE: u64 = 16 * 1024 * 1024;

/// WebSocket frame opcodes per RFC 6455 Section 5.2.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WsOpcode {
    Continuation = 0x0,
    Text = 0x1,
    Binary = 0x2,
    Close = 0x8,
    Ping = 0x9,
    Pong = 0xA,
}

impl WsOpcode {
    /// Parse a 4-bit opcode value into a `WsOpcode`.
    ///
    /// Returns `Err` for reserved/unknown opcodes (RFC 6455 requires close
    /// with code 1002 for unknown opcodes; the caller decides the response).
    pub fn from_u8(byte: u8) -> Result<WsOpcode, String> {
        match byte {
            0x0 => Ok(WsOpcode::Continuation),
            0x1 => Ok(WsOpcode::Text),
            0x2 => Ok(WsOpcode::Binary),
            0x8 => Ok(WsOpcode::Close),
            0x9 => Ok(WsOpcode::Ping),
            0xA => Ok(WsOpcode::Pong),
            _ => Err(format!("unknown opcode: 0x{:X}", byte)),
        }
    }
}

/// A parsed WebSocket frame.
#[derive(Debug)]
pub struct WsFrame {
    /// FIN bit -- `true` if this is the final fragment of a message.
    pub fin: bool,
    /// The frame opcode (text, binary, close, ping, pong, continuation).
    pub opcode: WsOpcode,
    /// The unmasked payload bytes.
    pub payload: Vec<u8>,
}

/// Apply or remove the 4-byte XOR mask on a payload.
///
/// The operation is symmetric: applying the mask twice returns the original.
/// Per RFC 6455 Section 5.3.
pub fn apply_mask(payload: &mut [u8], mask_key: &[u8; 4]) {
    for (i, byte) in payload.iter_mut().enumerate() {
        *byte ^= mask_key[i % 4];
    }
}

/// Parse one WebSocket frame from the stream.
///
/// Handles all three payload length encodings (7-bit, 16-bit, 64-bit) and
/// XOR unmasking of client-to-server frames. Uses `read_exact` for all reads
/// -- the caller controls buffering.
pub fn read_frame<R: Read>(reader: &mut R) -> Result<WsFrame, String> {
    // Byte 0: FIN(1) RSV(3) Opcode(4)
    // Byte 1: MASK(1) Payload-Length(7)
    let mut header = [0u8; 2];
    reader
        .read_exact(&mut header)
        .map_err(|e| format!("read frame header: {}", e))?;

    let fin = (header[0] & 0x80) != 0;
    let rsv = (header[0] >> 4) & 0x07;
    if rsv != 0 {
        return Err("non-zero RSV bits without negotiated extensions".to_string());
    }
    let opcode_byte = header[0] & 0x0F;
    let opcode = WsOpcode::from_u8(opcode_byte)?;

    let masked = (header[1] & 0x80) != 0;
    let length_byte = header[1] & 0x7F;

    // Payload length: 3 encodings per RFC 6455 Section 5.2
    let payload_len: u64 = match length_byte {
        0..=125 => length_byte as u64,
        126 => {
            let mut buf = [0u8; 2];
            reader
                .read_exact(&mut buf)
                .map_err(|e| format!("read 16-bit length: {}", e))?;
            u16::from_be_bytes(buf) as u64
        }
        127 => {
            let mut buf = [0u8; 8];
            reader
                .read_exact(&mut buf)
                .map_err(|e| format!("read 64-bit length: {}", e))?;
            let len = u64::from_be_bytes(buf);
            if len >> 63 != 0 {
                return Err("MSB of 64-bit length must be 0".to_string());
            }
            len
        }
        _ => unreachable!(),
    };

    // Safety cap to prevent OOM from malicious lengths
    if payload_len > MAX_PAYLOAD_SIZE {
        return Err(format!(
            "payload length {} exceeds maximum {}",
            payload_len, MAX_PAYLOAD_SIZE
        ));
    }

    // Masking key (4 bytes, present only if MASK bit is set)
    let mask_key = if masked {
        let mut key = [0u8; 4];
        reader
            .read_exact(&mut key)
            .map_err(|e| format!("read mask key: {}", e))?;
        Some(key)
    } else {
        None
    };

    // Read payload
    let mut payload = vec![0u8; payload_len as usize];
    if payload_len > 0 {
        reader
            .read_exact(&mut payload)
            .map_err(|e| format!("read payload: {}", e))?;
    }

    // Unmask if needed (client-to-server frames MUST be masked)
    if let Some(key) = mask_key {
        apply_mask(&mut payload, &key);
    }

    Ok(WsFrame {
        fin,
        opcode,
        payload,
    })
}

/// Write one WebSocket frame to the stream (server-to-client, unmasked).
///
/// Server MUST NOT mask frames per RFC 6455 Section 5.1. Uses the three
/// payload length encodings depending on payload size.
pub fn write_frame<W: Write>(
    writer: &mut W,
    opcode: WsOpcode,
    payload: &[u8],
    fin: bool,
) -> Result<(), String> {
    // Byte 0: FIN + opcode
    let byte0 = if fin { 0x80 } else { 0x00 } | (opcode as u8);

    // Byte 1: MASK=0 + payload length (server MUST NOT mask)
    let len = payload.len();
    if len <= 125 {
        writer
            .write_all(&[byte0, len as u8])
            .map_err(|e| format!("write frame header: {}", e))?;
    } else if len <= 65535 {
        writer
            .write_all(&[byte0, 126])
            .map_err(|e| format!("write frame header: {}", e))?;
        writer
            .write_all(&(len as u16).to_be_bytes())
            .map_err(|e| format!("write 16-bit length: {}", e))?;
    } else {
        writer
            .write_all(&[byte0, 127])
            .map_err(|e| format!("write frame header: {}", e))?;
        writer
            .write_all(&(len as u64).to_be_bytes())
            .map_err(|e| format!("write 64-bit length: {}", e))?;
    }

    // Payload (no masking key for server-to-client)
    if !payload.is_empty() {
        writer
            .write_all(payload)
            .map_err(|e| format!("write payload: {}", e))?;
    }

    writer.flush().map_err(|e| format!("flush frame: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_mask_roundtrip() {
        let original = b"Hello".to_vec();
        let key = [0x37, 0xfa, 0x21, 0x3d];
        let mut masked = original.clone();
        apply_mask(&mut masked, &key);
        assert_ne!(masked, original, "masked should differ from original");
        apply_mask(&mut masked, &key);
        assert_eq!(masked, original, "unmasked should equal original");
    }

    #[test]
    fn test_read_7bit_text_frame() {
        // A masked text frame "Hi" (2 bytes) from client
        // FIN=1, opcode=0x1 (text), MASK=1, len=2, mask_key=[0,0,0,0], payload="Hi"
        let frame_bytes: Vec<u8> = vec![
            0x81, // FIN=1, opcode=0x1
            0x82, // MASK=1, len=2
            0, 0, 0, 0, // mask key (all zeros = payload unchanged)
            b'H', b'i', // payload
        ];
        let mut cursor = Cursor::new(frame_bytes);
        let frame = read_frame(&mut cursor).unwrap();
        assert!(frame.fin);
        assert_eq!(frame.opcode, WsOpcode::Text);
        assert_eq!(frame.payload, b"Hi");
    }

    #[test]
    fn test_read_16bit_length() {
        // A masked frame with 200-byte payload using 16-bit length encoding
        let payload = vec![0xABu8; 200];
        let mask_key = [0u8; 4]; // zero mask for simplicity

        let mut frame_bytes: Vec<u8> = Vec::new();
        frame_bytes.push(0x82); // FIN=1, opcode=Binary
        frame_bytes.push(0xFE); // MASK=1, len=126 (16-bit follows)
        frame_bytes.extend_from_slice(&200u16.to_be_bytes()); // 16-bit length
        frame_bytes.extend_from_slice(&mask_key); // mask key
        frame_bytes.extend_from_slice(&payload); // payload

        let mut cursor = Cursor::new(frame_bytes);
        let frame = read_frame(&mut cursor).unwrap();
        assert!(frame.fin);
        assert_eq!(frame.opcode, WsOpcode::Binary);
        assert_eq!(frame.payload.len(), 200);
        assert_eq!(frame.payload, payload);
    }

    #[test]
    fn test_read_64bit_length() {
        // A masked frame with 300-byte payload using 64-bit length encoding
        let payload = vec![0xCDu8; 300];
        let mask_key = [0u8; 4]; // zero mask for simplicity

        let mut frame_bytes: Vec<u8> = Vec::new();
        frame_bytes.push(0x82); // FIN=1, opcode=Binary
        frame_bytes.push(0xFF); // MASK=1, len=127 (64-bit follows)
        frame_bytes.extend_from_slice(&300u64.to_be_bytes()); // 64-bit length
        frame_bytes.extend_from_slice(&mask_key); // mask key
        frame_bytes.extend_from_slice(&payload); // payload

        let mut cursor = Cursor::new(frame_bytes);
        let frame = read_frame(&mut cursor).unwrap();
        assert!(frame.fin);
        assert_eq!(frame.opcode, WsOpcode::Binary);
        assert_eq!(frame.payload.len(), 300);
        assert_eq!(frame.payload, payload);
    }

    #[test]
    fn test_write_small_frame() {
        // Write a text frame "Hello" (unmasked server frame)
        let mut buf = Vec::new();
        write_frame(&mut buf, WsOpcode::Text, b"Hello", true).unwrap();
        assert_eq!(buf, vec![0x81, 0x05, b'H', b'e', b'l', b'l', b'o']);
    }

    #[test]
    fn test_write_medium_frame() {
        // Write a 200-byte frame, verify 16-bit length encoding
        let payload = vec![0x42u8; 200];
        let mut buf = Vec::new();
        write_frame(&mut buf, WsOpcode::Binary, &payload, true).unwrap();

        // Header: FIN=1 + opcode=Binary(0x2) = 0x82, len=126, then 200 as u16 BE
        assert_eq!(buf[0], 0x82);
        assert_eq!(buf[1], 126);
        assert_eq!(&buf[2..4], &200u16.to_be_bytes());
        assert_eq!(&buf[4..], &payload[..]);
    }

    #[test]
    fn test_unknown_opcode() {
        // Frame with opcode 0x03 (reserved)
        let frame_bytes: Vec<u8> = vec![
            0x83, // FIN=1, opcode=0x3 (reserved)
            0x00, // MASK=0, len=0
        ];
        let mut cursor = Cursor::new(frame_bytes);
        let result = read_frame(&mut cursor);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.contains("unknown opcode"),
            "error should mention unknown opcode, got: {}",
            err
        );
    }

    #[test]
    fn test_nonzero_rsv_rejected() {
        // Frame with RSV1 bit set: byte0 = 0xC1 (FIN=1, RSV1=1, opcode=Text)
        let frame_bytes: Vec<u8> = vec![
            0xC1, // FIN=1, RSV1=1, opcode=0x1
            0x00, // MASK=0, len=0
        ];
        let mut cursor = Cursor::new(frame_bytes);
        let result = read_frame(&mut cursor);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.contains("RSV"),
            "error should mention RSV bits, got: {}",
            err
        );
    }

    #[test]
    fn test_frame_roundtrip() {
        // Write a frame then read it back (unmasked server frame)
        let original_payload = b"round-trip test payload";
        let mut buf = Vec::new();
        write_frame(&mut buf, WsOpcode::Text, original_payload, true).unwrap();

        let mut cursor = Cursor::new(buf);
        let frame = read_frame(&mut cursor).unwrap();
        assert!(frame.fin);
        assert_eq!(frame.opcode, WsOpcode::Text);
        assert_eq!(frame.payload, original_payload);
    }
}
