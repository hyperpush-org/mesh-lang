//! Wadler-Lindig format IR for Mesh code formatting.
//!
//! This module defines the intermediate representation used between the CST
//! walker and the printer. The IR captures formatting intent (groups, indentation,
//! line breaks) without committing to a specific layout until printing time.

/// A document IR node in the Wadler-Lindig style.
///
/// The printer decides at each `Group` boundary whether to render flat (all on
/// one line) or broken (with line breaks and indentation).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormatIR {
    /// Literal text to emit verbatim.
    Text(String),
    /// A space in flat mode; a newline + indent in broken mode.
    Space,
    /// Always emits a newline + current indentation, regardless of mode.
    Hardline,
    /// Increase indentation for the child IR by the configured indent size.
    Indent(Box<FormatIR>),
    /// Try to render the child flat (on one line). If it exceeds the remaining
    /// line width, render in broken mode instead.
    Group(Box<FormatIR>),
    /// Emit different content depending on whether the enclosing group is
    /// in flat or broken mode.
    IfBreak {
        flat: Box<FormatIR>,
        broken: Box<FormatIR>,
    },
    /// A sequence of IR nodes rendered in order.
    Concat(Vec<FormatIR>),
    /// Produces no output.
    Empty,
}

// ── Helper constructors ─────────────────────────────────────────────────

/// Create a `Text` node from a string-like value.
pub fn text(s: impl Into<String>) -> FormatIR {
    FormatIR::Text(s.into())
}

/// Create a `Space` node (space in flat mode, newline in broken mode).
pub fn space() -> FormatIR {
    FormatIR::Space
}

/// Create a `Hardline` node (always a newline).
pub fn hardline() -> FormatIR {
    FormatIR::Hardline
}

/// Create an `Indent` wrapper that increases indentation for its child.
pub fn indent(ir: FormatIR) -> FormatIR {
    FormatIR::Indent(Box::new(ir))
}

/// Create a `Group` that tries flat layout first, breaking if it exceeds width.
pub fn group(ir: FormatIR) -> FormatIR {
    FormatIR::Group(Box::new(ir))
}

/// Create a `Concat` from a vector of IR nodes.
pub fn concat(parts: Vec<FormatIR>) -> FormatIR {
    FormatIR::Concat(parts)
}

/// Create an `IfBreak` that selects content based on the enclosing group's mode.
pub fn if_break(flat: FormatIR, broken: FormatIR) -> FormatIR {
    FormatIR::IfBreak {
        flat: Box::new(flat),
        broken: Box::new(broken),
    }
}
