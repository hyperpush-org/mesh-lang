//! Collection types for the Mesh runtime.
//!
//! Provides List, Map, Set, Tuple utilities, Range, and Queue -- all immutable,
//! GC-allocated, and using uniform 8-byte element representation.

pub mod list;
pub mod map;
pub mod queue;
pub mod range;
pub mod set;
pub mod tuple;
