//! Typed AST layer on top of the rowan CST.
//!
//! The typed AST provides zero-cost wrappers around `SyntaxNode` with typed
//! accessor methods. Each wrapper holds a `SyntaxNode` reference and provides
//! methods to navigate to children by their kind, following the rust-analyzer
//! pattern.
//!
//! # Architecture
//!
//! - [`AstNode`] trait: every typed wrapper implements `cast()` and `syntax()`.
//! - `ast_node!` macro: generates boilerplate for each wrapper type.
//! - Helper functions: `child_node()` and `child_token()` for navigating children.
//!
//! # Zero-cost
//!
//! The wrappers are newtype structs around `SyntaxNode`. They add no runtime
//! overhead -- `cast()` is a single kind check, and accessor methods walk the
//! rowan tree directly.

pub mod expr;
pub mod item;
pub mod pat;

use crate::cst::{SyntaxNode, SyntaxToken};
use crate::syntax_kind::SyntaxKind;

/// Trait for typed AST nodes that wrap a rowan `SyntaxNode`.
///
/// Every typed AST wrapper implements this trait, providing:
/// - `cast()`: attempt to downcast a generic `SyntaxNode` into this type
/// - `syntax()`: access the underlying `SyntaxNode`
pub trait AstNode: Sized {
    /// Try to cast a generic `SyntaxNode` into this typed AST node.
    ///
    /// Returns `Some(Self)` if the node's kind matches, `None` otherwise.
    fn cast(node: SyntaxNode) -> Option<Self>;

    /// Access the underlying `SyntaxNode`.
    fn syntax(&self) -> &SyntaxNode;
}

/// Generate boilerplate for a typed AST node wrapper.
///
/// Creates a struct wrapping `SyntaxNode`, and implements `AstNode` with a
/// kind check against the specified `SyntaxKind` variant.
macro_rules! ast_node {
    ($name:ident, $kind:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub(crate) syntax: SyntaxNode,
        }

        impl AstNode for $name {
            fn cast(node: SyntaxNode) -> Option<Self> {
                if node.kind() == SyntaxKind::$kind {
                    Some(Self { syntax: node })
                } else {
                    None
                }
            }

            fn syntax(&self) -> &SyntaxNode {
                &self.syntax
            }
        }
    };
}

pub(crate) use ast_node;

/// Find the first child node that can be cast to type `N`.
pub fn child_node<N: AstNode>(parent: &SyntaxNode) -> Option<N> {
    parent.children().find_map(N::cast)
}

/// Find all child nodes that can be cast to type `N`.
pub fn child_nodes<'a, N: AstNode + 'a>(parent: &'a SyntaxNode) -> impl Iterator<Item = N> + 'a {
    parent.children().filter_map(N::cast)
}

/// Find the first child token with the given kind.
pub fn child_token(parent: &SyntaxNode, kind: SyntaxKind) -> Option<SyntaxToken> {
    parent
        .children_with_tokens()
        .filter_map(|it| it.into_token())
        .find(|it| it.kind() == kind)
}
