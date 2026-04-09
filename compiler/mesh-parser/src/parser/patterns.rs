//! Pattern parser for Mesh.
//!
//! Parses patterns used in match arms, let bindings, and destructuring.
//! Patterns include: wildcard (`_`), identifier, literal, tuple, struct,
//! constructor, or-pattern, as-pattern, and cons-pattern.
//!
//! Pattern grammar (precedence, lowest to highest):
//! ```text
//! pattern       = as_pattern
//! as_pattern    = cons_pattern ["as" IDENT]
//! cons_pattern  = or_pattern ("::" cons_pattern)?
//! or_pattern    = primary_pattern ("|" primary_pattern)*
//! primary_pattern = wildcard | literal | tuple | constructor | ident
//! ```

use crate::syntax_kind::SyntaxKind;

use super::{MarkClosed, Parser};

/// Parse a pattern (top-level entry point).
///
/// Handles the full pattern grammar including or-patterns and as-patterns.
pub(crate) fn parse_pattern(p: &mut Parser) -> Option<MarkClosed> {
    parse_as_pattern(p)
}

/// Parse an as-pattern: `pattern as name`
///
/// If the inner pattern is followed by an IDENT with text "as", wraps
/// the pattern in an AS_PAT node.
fn parse_as_pattern(p: &mut Parser) -> Option<MarkClosed> {
    let inner = parse_cons_pattern(p)?;

    // Check for `as` binding (contextual keyword -- "as" is just an IDENT)
    if p.at(SyntaxKind::IDENT) && p.current_text() == "as" {
        let m = p.open_before(inner);
        p.advance(); // "as" ident

        // Parse the binding name
        if p.at(SyntaxKind::IDENT) {
            let name = p.open();
            p.advance();
            p.close(name, SyntaxKind::IDENT_PAT);
        } else {
            p.error("expected identifier after `as`");
        }

        Some(p.close(m, SyntaxKind::AS_PAT))
    } else {
        Some(inner)
    }
}

/// Parse a cons pattern: `head :: tail`
///
/// Cons patterns destructure lists into head element and tail list.
/// Right-associative: `a :: b :: c` parses as `a :: (b :: c)`.
/// Only valid in pattern position -- `::` is not used as an expression operator.
fn parse_cons_pattern(p: &mut Parser) -> Option<MarkClosed> {
    let head = parse_or_pattern(p)?;

    if p.at(SyntaxKind::COLON_COLON) {
        let m = p.open_before(head);
        p.advance(); // ::
        parse_cons_pattern(p); // right-associative: tail can be another cons
        Some(p.close(m, SyntaxKind::CONS_PAT))
    } else {
        Some(head)
    }
}

/// Parse an or-pattern: `pattern | pattern | ...`
///
/// If the primary pattern is followed by BAR tokens, wraps all alternatives
/// in an OR_PAT node.
fn parse_or_pattern(p: &mut Parser) -> Option<MarkClosed> {
    let first = parse_primary_pattern(p)?;

    if p.at(SyntaxKind::BAR) {
        let m = p.open_before(first);
        while p.eat(SyntaxKind::BAR) {
            parse_primary_pattern(p);
        }
        Some(p.close(m, SyntaxKind::OR_PAT))
    } else {
        Some(first)
    }
}

/// Parse a primary pattern (no or/as wrapping).
///
/// Primary patterns:
/// - `_` -> WILDCARD_PAT
/// - `42`, `"hello"`, `true`, `false`, `nil` -> LITERAL_PAT
/// - `-42` (negative literal) -> LITERAL_PAT
/// - `(p1, p2, ...)` -> TUPLE_PAT
/// - `Name.Variant(args)` -> CONSTRUCTOR_PAT (qualified)
/// - `Variant(args)` -> CONSTRUCTOR_PAT (unqualified, starts with uppercase + parens)
/// - `ident` -> IDENT_PAT
fn parse_primary_pattern(p: &mut Parser) -> Option<MarkClosed> {
    match p.current() {
        // Wildcard: _
        // The lexer emits `_` as an Ident token, so check the text.
        SyntaxKind::IDENT if p.current_text() == "_" => {
            let m = p.open();
            p.advance(); // _
            Some(p.close(m, SyntaxKind::WILDCARD_PAT))
        }

        // Literal patterns: numbers, booleans, nil
        SyntaxKind::INT_LITERAL | SyntaxKind::FLOAT_LITERAL => {
            let m = p.open();
            p.advance();
            Some(p.close(m, SyntaxKind::LITERAL_PAT))
        }

        SyntaxKind::TRUE_KW | SyntaxKind::FALSE_KW | SyntaxKind::NIL_KW => {
            let m = p.open();
            p.advance();
            Some(p.close(m, SyntaxKind::LITERAL_PAT))
        }

        // String literal pattern
        SyntaxKind::STRING_START => {
            let m = p.open();
            // Consume the whole string (STRING_START...STRING_END)
            p.advance(); // STRING_START
            loop {
                match p.current() {
                    SyntaxKind::STRING_CONTENT => p.advance(),
                    SyntaxKind::STRING_END => {
                        p.advance();
                        break;
                    }
                    SyntaxKind::EOF => {
                        p.error("unterminated string in pattern");
                        break;
                    }
                    _ => {
                        // Interpolated strings are not valid patterns
                        p.error("string interpolation not allowed in patterns");
                        break;
                    }
                }
            }
            Some(p.close(m, SyntaxKind::LITERAL_PAT))
        }

        // Negative number literal: -42
        SyntaxKind::MINUS
            if matches!(
                p.nth(1),
                SyntaxKind::INT_LITERAL | SyntaxKind::FLOAT_LITERAL
            ) =>
        {
            let m = p.open();
            p.advance(); // -
            p.advance(); // number
            Some(p.close(m, SyntaxKind::LITERAL_PAT))
        }

        // Tuple pattern: (p1, p2, ...)
        SyntaxKind::L_PAREN => {
            let m = p.open();
            p.advance(); // (

            if !p.at(SyntaxKind::R_PAREN) {
                parse_pattern(p);
                while p.eat(SyntaxKind::COMMA) {
                    if p.at(SyntaxKind::R_PAREN) {
                        break; // trailing comma
                    }
                    parse_pattern(p);
                }
            }

            p.expect(SyntaxKind::R_PAREN);
            Some(p.close(m, SyntaxKind::TUPLE_PAT))
        }

        // Identifier-starting patterns: plain ident, constructor, qualified constructor
        SyntaxKind::IDENT => {
            let text = p.current_text().to_string();
            let starts_upper = text.starts_with(|c: char| c.is_uppercase());

            // Check for qualified constructor: Name.Variant or Name.Variant(args)
            if p.nth(1) == SyntaxKind::DOT && p.nth(2) == SyntaxKind::IDENT {
                // Qualified: Type.Variant or Type.Variant(args)
                let m = p.open();
                p.advance(); // type name
                p.advance(); // .
                p.advance(); // variant name

                // Optional argument list
                if p.at(SyntaxKind::L_PAREN) {
                    p.advance(); // (
                    if !p.at(SyntaxKind::R_PAREN) {
                        parse_pattern(p);
                        while p.eat(SyntaxKind::COMMA) {
                            if p.at(SyntaxKind::R_PAREN) {
                                break;
                            }
                            parse_pattern(p);
                        }
                    }
                    p.expect(SyntaxKind::R_PAREN);
                }

                Some(p.close(m, SyntaxKind::CONSTRUCTOR_PAT))
            } else if starts_upper && p.nth(1) == SyntaxKind::L_PAREN {
                // Unqualified constructor with args: Variant(args)
                let m = p.open();
                p.advance(); // variant name

                p.advance(); // (
                if !p.at(SyntaxKind::R_PAREN) {
                    parse_pattern(p);
                    while p.eat(SyntaxKind::COMMA) {
                        if p.at(SyntaxKind::R_PAREN) {
                            break;
                        }
                        parse_pattern(p);
                    }
                }
                p.expect(SyntaxKind::R_PAREN);

                Some(p.close(m, SyntaxKind::CONSTRUCTOR_PAT))
            } else {
                // Plain identifier pattern (could be nullary constructor resolved later)
                let m = p.open();
                p.advance(); // ident
                Some(p.close(m, SyntaxKind::IDENT_PAT))
            }
        }

        _ => {
            p.error("expected pattern");
            None
        }
    }
}
