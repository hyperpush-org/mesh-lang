use std::fmt;

use serde::Serialize;

use crate::span::Span;

/// A lexer error with location information.
///
/// Errors are collected during lexing rather than aborting immediately,
/// enabling error recovery and reporting multiple issues at once.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LexError {
    pub kind: LexErrorKind,
    pub span: Span,
}

impl LexError {
    /// Create a new lexer error.
    pub fn new(kind: LexErrorKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// The specific kind of lexer error.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum LexErrorKind {
    /// An unexpected character was encountered.
    UnexpectedCharacter(char),
    /// A string literal was not closed before end of input.
    UnterminatedString,
    /// A block comment (`#= ... =#`) was not closed before end of input.
    UnterminatedBlockComment,
    /// A string interpolation (`${...}`) was not closed before end of input.
    UnterminatedInterpolation,
    /// An invalid escape sequence was encountered in a string.
    InvalidEscapeSequence(char),
    /// A number literal could not be parsed.
    InvalidNumberLiteral(String),
}

impl fmt::Display for LexErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedCharacter(c) => write!(f, "unexpected character: {c:?}"),
            Self::UnterminatedString => write!(f, "unterminated string literal"),
            Self::UnterminatedBlockComment => write!(f, "unterminated block comment"),
            Self::UnterminatedInterpolation => write!(f, "unterminated string interpolation"),
            Self::InvalidEscapeSequence(c) => write!(f, "invalid escape sequence: \\{c}"),
            Self::InvalidNumberLiteral(s) => write!(f, "invalid number literal: {s}"),
        }
    }
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl std::error::Error for LexError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_error_display() {
        let err = LexError::new(LexErrorKind::UnexpectedCharacter('@'), Span::new(0, 1));
        assert_eq!(err.to_string(), "unexpected character: '@'");
    }

    #[test]
    fn lex_error_kind_display_all_variants() {
        assert_eq!(
            LexErrorKind::UnterminatedString.to_string(),
            "unterminated string literal"
        );
        assert_eq!(
            LexErrorKind::UnterminatedBlockComment.to_string(),
            "unterminated block comment"
        );
        assert_eq!(
            LexErrorKind::UnterminatedInterpolation.to_string(),
            "unterminated string interpolation"
        );
        assert_eq!(
            LexErrorKind::InvalidEscapeSequence('n').to_string(),
            "invalid escape sequence: \\n"
        );
        assert_eq!(
            LexErrorKind::InvalidNumberLiteral("0x".into()).to_string(),
            "invalid number literal: 0x"
        );
    }
}
