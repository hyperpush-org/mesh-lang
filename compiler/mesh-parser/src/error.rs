//! Parse error types for the Mesh parser.

use std::fmt;

use mesh_common::span::Span;

/// A parse error with location information and optional related span.
///
/// Parse errors carry the primary span where the problem was detected, a
/// human-readable message, and an optional related span for context (e.g.,
/// "opened here" for unclosed delimiters).
#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    /// Human-readable description of what went wrong.
    pub message: String,
    /// Primary source location where the error was detected.
    pub span: Span,
    /// Optional related location with context message (e.g., "block started here").
    pub related: Option<(String, Span)>,
}

impl ParseError {
    /// Create a new parse error with just a message and span.
    pub fn new(message: impl Into<String>, span: Span) -> Self {
        Self {
            message: message.into(),
            span,
            related: None,
        }
    }

    /// Create a parse error with a related span for additional context.
    pub fn with_related(
        message: impl Into<String>,
        span: Span,
        related_message: impl Into<String>,
        related_span: Span,
    ) -> Self {
        Self {
            message: message.into(),
            span,
            related: Some((related_message.into(), related_span)),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_error_new() {
        let err = ParseError::new("expected expression", Span::new(5, 10));
        assert_eq!(err.message, "expected expression");
        assert_eq!(err.span, Span::new(5, 10));
        assert!(err.related.is_none());
    }

    #[test]
    fn parse_error_with_related() {
        let err = ParseError::with_related(
            "expected `end` to close `do` block",
            Span::new(50, 53),
            "block started here",
            Span::new(10, 12),
        );
        assert_eq!(err.message, "expected `end` to close `do` block");
        assert_eq!(err.span, Span::new(50, 53));
        let (msg, span) = err.related.unwrap();
        assert_eq!(msg, "block started here");
        assert_eq!(span, Span::new(10, 12));
    }

    #[test]
    fn parse_error_display() {
        let err = ParseError::new("unexpected token", Span::new(0, 1));
        assert_eq!(err.to_string(), "unexpected token");
    }
}
