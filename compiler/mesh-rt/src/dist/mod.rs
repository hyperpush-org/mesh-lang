//! Distribution subsystem for Mesh.
//!
//! Provides PID bit-packing helpers, the Mesh Term Format (STF) binary
//! serializer/deserializer, and the node identity/connection layer for
//! inter-node message transport.

pub mod bootstrap;
pub mod continuity;
pub mod discovery;
pub mod global;
pub mod node;
pub mod operator;
pub mod wire;
