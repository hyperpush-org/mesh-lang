//! Typed AST nodes for patterns.
//!
//! Covers: WildcardPat, IdentPat, LiteralPat, TuplePat, ConstructorPat, OrPat, AsPat.

use crate::ast::{ast_node, child_token, AstNode};
use crate::cst::{SyntaxNode, SyntaxToken};
use crate::syntax_kind::SyntaxKind;

// ── Pattern enum ─────────────────────────────────────────────────────────

/// Any pattern node.
#[derive(Debug, Clone)]
pub enum Pattern {
    Wildcard(WildcardPat),
    Ident(IdentPat),
    Literal(LiteralPat),
    Tuple(TuplePat),
    Constructor(ConstructorPat),
    Or(OrPat),
    As(AsPat),
    Cons(ConsPat),
}

impl Pattern {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        match node.kind() {
            SyntaxKind::WILDCARD_PAT => Some(Pattern::Wildcard(WildcardPat { syntax: node })),
            SyntaxKind::IDENT_PAT => Some(Pattern::Ident(IdentPat { syntax: node })),
            SyntaxKind::LITERAL_PAT => Some(Pattern::Literal(LiteralPat { syntax: node })),
            SyntaxKind::TUPLE_PAT => Some(Pattern::Tuple(TuplePat { syntax: node })),
            SyntaxKind::CONSTRUCTOR_PAT => {
                Some(Pattern::Constructor(ConstructorPat { syntax: node }))
            }
            SyntaxKind::OR_PAT => Some(Pattern::Or(OrPat { syntax: node })),
            SyntaxKind::AS_PAT => Some(Pattern::As(AsPat { syntax: node })),
            SyntaxKind::CONS_PAT => Some(Pattern::Cons(ConsPat { syntax: node })),
            _ => None,
        }
    }

    /// Access the underlying syntax node regardless of variant.
    pub fn syntax(&self) -> &SyntaxNode {
        match self {
            Pattern::Wildcard(n) => &n.syntax,
            Pattern::Ident(n) => &n.syntax,
            Pattern::Literal(n) => &n.syntax,
            Pattern::Tuple(n) => &n.syntax,
            Pattern::Constructor(n) => &n.syntax,
            Pattern::Or(n) => &n.syntax,
            Pattern::As(n) => &n.syntax,
            Pattern::Cons(n) => &n.syntax,
        }
    }
}

// ── Wildcard Pattern ─────────────────────────────────────────────────────

ast_node!(WildcardPat, WILDCARD_PAT);

// ── Identifier Pattern ───────────────────────────────────────────────────

ast_node!(IdentPat, IDENT_PAT);

impl IdentPat {
    /// The identifier text.
    pub fn name(&self) -> Option<SyntaxToken> {
        child_token(&self.syntax, SyntaxKind::IDENT)
    }
}

// ── Literal Pattern ──────────────────────────────────────────────────────

ast_node!(LiteralPat, LITERAL_PAT);

impl LiteralPat {
    /// The literal value token.
    pub fn token(&self) -> Option<SyntaxToken> {
        self.syntax
            .children_with_tokens()
            .filter_map(|it| it.into_token())
            .find(|t| {
                matches!(
                    t.kind(),
                    SyntaxKind::INT_LITERAL
                        | SyntaxKind::FLOAT_LITERAL
                        | SyntaxKind::TRUE_KW
                        | SyntaxKind::FALSE_KW
                        | SyntaxKind::NIL_KW
                        | SyntaxKind::STRING_START
                )
            })
    }
}

// ── Tuple Pattern ────────────────────────────────────────────────────────

ast_node!(TuplePat, TUPLE_PAT);

impl TuplePat {
    /// The sub-patterns in the tuple.
    pub fn patterns(&self) -> impl Iterator<Item = Pattern> + '_ {
        self.syntax.children().filter_map(Pattern::cast)
    }
}

// ── Constructor Pattern ──────────────────────────────────────────────────

ast_node!(ConstructorPat, CONSTRUCTOR_PAT);

impl ConstructorPat {
    /// Whether this is a qualified constructor (e.g., `Shape.Circle` vs `Some`).
    ///
    /// Qualified constructors have a DOT token between type name and variant name.
    pub fn is_qualified(&self) -> bool {
        self.syntax
            .children_with_tokens()
            .filter_map(|it| it.into_token())
            .any(|t| t.kind() == SyntaxKind::DOT)
    }

    /// The type qualifier name (e.g., "Shape" in `Shape.Circle`).
    ///
    /// Returns `None` for unqualified constructors like `Some(x)`.
    pub fn type_name(&self) -> Option<SyntaxToken> {
        if self.is_qualified() {
            // First IDENT is the type name
            self.syntax
                .children_with_tokens()
                .filter_map(|it| it.into_token())
                .find(|t| t.kind() == SyntaxKind::IDENT)
        } else {
            None
        }
    }

    /// The variant name (e.g., "Circle" in `Shape.Circle` or `Circle` in `Circle(r)`).
    ///
    /// For qualified constructors, this is the IDENT after the DOT.
    /// For unqualified constructors, this is the first (and only) IDENT.
    pub fn variant_name(&self) -> Option<SyntaxToken> {
        let idents: Vec<_> = self
            .syntax
            .children_with_tokens()
            .filter_map(|it| it.into_token())
            .filter(|t| t.kind() == SyntaxKind::IDENT)
            .collect();

        if self.is_qualified() {
            // Second IDENT is the variant name
            idents.into_iter().nth(1)
        } else {
            // First IDENT is the variant name
            idents.into_iter().next()
        }
    }

    /// The sub-patterns inside the constructor's parentheses.
    ///
    /// For `Circle(r)` this yields the `r` pattern.
    /// For nullary constructors like `Shape.Point`, this is empty.
    pub fn fields(&self) -> impl Iterator<Item = Pattern> + '_ {
        self.syntax.children().filter_map(Pattern::cast)
    }
}

// ── Or Pattern ──────────────────────────────────────────────────────────

ast_node!(OrPat, OR_PAT);

impl OrPat {
    /// The alternative patterns in this or-pattern.
    ///
    /// For `Circle(_) | Point` this yields both `Circle(_)` and `Point`.
    pub fn alternatives(&self) -> impl Iterator<Item = Pattern> + '_ {
        self.syntax.children().filter_map(Pattern::cast)
    }
}

// ── Cons Pattern ────────────────────────────────────────────────────────

ast_node!(ConsPat, CONS_PAT);

impl ConsPat {
    /// The head pattern (first element).
    ///
    /// For `h :: t`, this is `h`.
    pub fn head(&self) -> Option<Pattern> {
        self.syntax.children().find_map(Pattern::cast)
    }

    /// The tail pattern (remaining list).
    ///
    /// For `h :: t`, this is `t`.
    pub fn tail(&self) -> Option<Pattern> {
        self.syntax.children().filter_map(Pattern::cast).nth(1)
    }
}

// ── As Pattern ──────────────────────────────────────────────────────────

ast_node!(AsPat, AS_PAT);

impl AsPat {
    /// The inner pattern (before `as`).
    ///
    /// For `Circle(r) as c`, this is `Circle(r)`.
    pub fn pattern(&self) -> Option<Pattern> {
        self.syntax.children().find_map(Pattern::cast)
    }

    /// The binding name after `as`.
    ///
    /// For `Circle(r) as c`, this returns the token "c".
    /// The binding is stored as an IDENT_PAT child; we get its IDENT token.
    pub fn binding_name(&self) -> Option<SyntaxToken> {
        // The binding is the last IDENT_PAT child (after the inner pattern)
        let binding_pat = self.syntax.children().filter_map(Pattern::cast).last()?;
        match binding_pat {
            Pattern::Ident(ident_pat) => ident_pat.name(),
            _ => None,
        }
    }
}
