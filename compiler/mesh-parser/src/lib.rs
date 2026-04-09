//! Mesh parser: recursive descent parser producing a rowan-based CST.
//!
//! This crate transforms the token stream from `mesh-lexer` into a lossless
//! concrete syntax tree (CST) using the `rowan` library. The CST preserves
//! all tokens including whitespace and comments, enabling future tooling
//! (formatter, LSP) to work from the same tree.

pub mod ast;
pub mod cst;
pub mod error;
mod parser;
pub mod syntax_kind;

pub use ast::AstNode;
pub use cst::{SyntaxElement, SyntaxNode, SyntaxToken};
pub use error::ParseError;
pub use syntax_kind::SyntaxKind;

/// Result of parsing a Mesh source file.
///
/// Contains the green tree (the immutable, cheap-to-clone CST) and any
/// parse errors encountered. With the current first-error-only strategy,
/// `errors` will contain at most one error.
pub struct Parse {
    green: rowan::GreenNode,
    errors: Vec<ParseError>,
}

impl Parse {
    /// Build the syntax tree root from the green node.
    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green.clone())
    }

    /// Parse errors encountered during parsing.
    pub fn errors(&self) -> &[ParseError] {
        &self.errors
    }

    /// Whether parsing completed without errors.
    pub fn ok(&self) -> bool {
        self.errors.is_empty()
    }

    /// Convenience: get the typed `SourceFile` AST root.
    ///
    /// This casts the root `SyntaxNode` to `SourceFile`. Should always
    /// succeed for a parse result from [`parse()`].
    pub fn tree(&self) -> ast::item::SourceFile {
        ast::item::SourceFile::cast(self.syntax()).expect("root node is always SOURCE_FILE")
    }
}

/// Parse a Mesh source file into a CST.
///
/// This is the main entry point for the parser. It lexes the source,
/// parses the token stream, and returns a [`Parse`] result containing
/// the syntax tree and any errors.
pub fn parse(source: &str) -> Parse {
    let tokens = mesh_lexer::Lexer::tokenize(source);
    let mut p = parser::Parser::new(tokens, source);
    parser::parse_source_file(&mut p);
    let (green, errors) = p.build_tree();
    Parse { green, errors }
}

/// Parse a single expression from Mesh source code.
///
/// This is primarily useful for testing the expression parser in isolation.
/// Wraps the expression in a SOURCE_FILE root node.
pub fn parse_expr(source: &str) -> Parse {
    let tokens = mesh_lexer::Lexer::tokenize(source);
    let mut p = parser::Parser::new(tokens, source);
    let root = p.open();
    parser::expressions::expr(&mut p);
    // Consume any remaining tokens (newlines, EOF).
    while !p.at(SyntaxKind::EOF) {
        p.advance();
    }
    p.advance(); // EOF
    p.close(root, SyntaxKind::SOURCE_FILE);
    let (green, errors) = p.build_tree();
    Parse { green, errors }
}

/// Parse a block of Mesh statements from source code.
///
/// This parses the source as a block body (sequence of statements separated
/// by newlines), wrapped in a SOURCE_FILE root node. Useful for testing
/// let bindings, return expressions, and multi-statement blocks.
pub fn parse_block(source: &str) -> Parse {
    let tokens = mesh_lexer::Lexer::tokenize(source);
    let mut p = parser::Parser::new(tokens, source);
    let root = p.open();
    parser::expressions::parse_block_body(&mut p);
    // Consume EOF.
    if p.at(SyntaxKind::EOF) {
        p.advance();
    }
    p.close(root, SyntaxKind::SOURCE_FILE);
    let (green, errors) = p.build_tree();
    Parse { green, errors }
}

/// Format a syntax tree as an indented debug string.
///
/// Each node is printed as `KIND` with children indented. Tokens show
/// `KIND "text"`. This is useful for snapshot testing the tree structure.
pub fn debug_tree(node: &SyntaxNode) -> String {
    let mut buf = String::new();
    debug_tree_recursive(node, &mut buf, 0);
    buf
}

fn debug_tree_recursive(node: &SyntaxNode, buf: &mut String, indent: usize) {
    let kind = node.kind();
    let prefix = "  ".repeat(indent);
    let range = node.text_range();
    buf.push_str(&format!(
        "{}{:?}@{:?}..{:?}\n",
        prefix,
        kind,
        range.start(),
        range.end()
    ));
    for child in node.children_with_tokens() {
        match child {
            rowan::NodeOrToken::Node(n) => {
                debug_tree_recursive(&n, buf, indent + 1);
            }
            rowan::NodeOrToken::Token(t) => {
                let t_kind = t.kind();
                let text = t.text();
                let t_range = t.text_range();
                buf.push_str(&format!(
                    "{}  {:?}@{:?}..{:?} {:?}\n",
                    prefix,
                    t_kind,
                    t_range.start(),
                    t_range.end(),
                    text,
                ));
            }
        }
    }
}
