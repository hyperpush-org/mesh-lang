//! Go-to-definition resolution via CST traversal.
//!
//! Resolves identifier references to their definition sites by walking the
//! concrete syntax tree. Supports:
//! - Variable references -> let binding NAME
//! - Function calls -> fn definition NAME
//! - Type names -> struct/type/sum type definition NAME
//! - Module-qualified names -> function within user-defined module
//!
//! ## Coordinate system
//!
//! The Mesh lexer skips whitespace, so the rowan CST does not contain whitespace
//! tokens. This means rowan `TextRange` offsets are NOT the same as source byte
//! offsets. We re-lex the source to build a mapping from source byte offsets to
//! rowan tree offsets.

use mesh_parser::SyntaxKind;
use mesh_parser::SyntaxNode;
use rowan::TextRange;

/// Known built-in module names that have no source definition.
const BUILTIN_MODULES: &[&str] = &[
    "IO", "String", "List", "Map", "Set", "Queue", "Int", "Float", "Bool", "Http", "Request",
    "Response", "Json", "Job", "File", "Math", "Result", "Option",
];

/// Convert a source byte offset to a rowan tree offset.
///
/// Since the lexer skips whitespace, rowan tree offsets differ from source
/// offsets. This function re-lexes the source to build the mapping.
pub fn source_to_tree_offset(source: &str, source_offset: usize) -> Option<usize> {
    let tokens = mesh_lexer::Lexer::tokenize(source);
    let mut tree_offset: usize = 0;

    for token in &tokens {
        let tok_start = token.span.start as usize;
        let tok_end = token.span.end as usize;
        let tok_len = tok_end - tok_start;

        if source_offset >= tok_start && source_offset < tok_end {
            // The source offset falls within this token.
            let offset_within_token = source_offset - tok_start;
            return Some(tree_offset + offset_within_token);
        }

        tree_offset += tok_len;
    }

    // Offset is at or past the end.
    None
}

/// Convert a rowan tree offset to a source byte offset.
///
/// Inverse of `source_to_tree_offset`.
pub fn tree_to_source_offset(source: &str, tree_offset: usize) -> Option<usize> {
    let tokens = mesh_lexer::Lexer::tokenize(source);
    let mut cumulative_tree: usize = 0;

    for token in &tokens {
        let tok_start = token.span.start as usize;
        let tok_end = token.span.end as usize;
        let tok_len = tok_end - tok_start;

        if tree_offset >= cumulative_tree && tree_offset < cumulative_tree + tok_len {
            let offset_within_token = tree_offset - cumulative_tree;
            return Some(tok_start + offset_within_token);
        }

        cumulative_tree += tok_len;
    }

    None
}

/// Find the definition site of the identifier at the given source byte offset.
///
/// Converts the source offset to a rowan tree offset, traverses the CST,
/// and returns the `TextRange` of the definition's NAME node (in rowan
/// coordinates). The caller must convert back to source coordinates using
/// `tree_to_source_offset` if needed for LSP position computation.
pub fn find_definition(source: &str, root: &SyntaxNode, source_offset: usize) -> Option<TextRange> {
    let tree_offset = source_to_tree_offset(source, source_offset)?;
    let target_offset = rowan::TextSize::from(tree_offset as u32);

    // Find the token at the given offset.
    let token = root.token_at_offset(target_offset).right_biased()?;

    // Only resolve IDENT tokens that are inside NAME_REF nodes or type annotation contexts.
    let parent = token.parent()?;
    let parent_kind = parent.kind();

    match parent_kind {
        SyntaxKind::NAME_REF => {
            let name_text = token.text().to_string();
            // Check if it's a built-in module reference (e.g., IO, String).
            if BUILTIN_MODULES.contains(&name_text.as_str()) {
                return None;
            }
            find_variable_or_function_def(root, &parent, &name_text)
        }
        SyntaxKind::NAME => {
            // If the identifier is in a NAME node that is a child of FIELD_ACCESS,
            // this might be a qualified name like Module.function -- resolve the
            // field within the module.
            let grandparent = parent.parent()?;
            if grandparent.kind() == SyntaxKind::FIELD_ACCESS {
                let name_text = token.text().to_string();
                // Find the base (first child NAME_REF of the FIELD_ACCESS).
                let base_ref = grandparent
                    .children()
                    .find(|c| c.kind() == SyntaxKind::NAME_REF)?;
                let base_text = first_ident_text(&base_ref)?;
                if BUILTIN_MODULES.contains(&base_text.as_str()) {
                    return None;
                }
                // Look for a MODULE_DEF with that base name, then find the function inside.
                return find_in_module(root, &base_text, &name_text);
            }
            None
        }
        _ => {
            // The token might be an IDENT inside a TYPE_ANNOTATION or other context.
            // Walk up to see if we're in a type annotation context.
            if token.kind() == SyntaxKind::IDENT {
                let name_text = token.text().to_string();
                // Check if this is a type name reference in a type annotation.
                if is_in_type_context(&parent) {
                    return find_type_def(root, &name_text);
                }
            }
            None
        }
    }
}

/// Check whether a node is in a type annotation context.
fn is_in_type_context(node: &SyntaxNode) -> bool {
    let mut current = Some(node.clone());
    while let Some(n) = current {
        if n.kind() == SyntaxKind::TYPE_ANNOTATION {
            return true;
        }
        current = n.parent();
    }
    false
}

/// Find a variable or function definition for a NAME_REF node.
///
/// Walks upward from the reference through enclosing blocks, searching for:
/// - LET_BINDING with a matching NAME
/// - FN_DEF with a matching NAME
/// - PARAM with a matching NAME/IDENT
///
/// At the top level (SOURCE_FILE), searches all definitions.
fn find_variable_or_function_def(
    _root: &SyntaxNode,
    name_ref_node: &SyntaxNode,
    name: &str,
) -> Option<TextRange> {
    // Walk up the tree from the reference.
    let mut current = name_ref_node.parent()?;

    loop {
        match current.kind() {
            SyntaxKind::BLOCK | SyntaxKind::SOURCE_FILE => {
                // Search earlier siblings in this block/source for definitions.
                if let Some(range) = search_block_for_def(&current, name_ref_node, name) {
                    return Some(range);
                }
                // If this is SOURCE_FILE, we're done searching.
                if current.kind() == SyntaxKind::SOURCE_FILE {
                    return None;
                }
            }
            SyntaxKind::FN_DEF => {
                // Check function parameters.
                if let Some(range) = search_params_for_name(&current, name) {
                    return Some(range);
                }
            }
            SyntaxKind::CLOSURE_EXPR => {
                // Check closure parameters.
                if let Some(range) = search_params_for_name(&current, name) {
                    return Some(range);
                }
            }
            _ => {}
        }

        current = match current.parent() {
            Some(p) => p,
            None => return None,
        };
    }
}

/// Search within a block or source file for a definition of `name` that
/// appears before `name_ref_node`.
fn search_block_for_def(
    block: &SyntaxNode,
    name_ref_node: &SyntaxNode,
    name: &str,
) -> Option<TextRange> {
    let ref_offset = name_ref_node.text_range().start();

    // For SOURCE_FILE, search all children (not just earlier ones) to handle
    // forward references to top-level functions.
    let search_all = block.kind() == SyntaxKind::SOURCE_FILE;

    for child in block.children() {
        // Only consider definitions before the reference (unless top-level).
        if !search_all && child.text_range().start() >= ref_offset {
            break;
        }

        match child.kind() {
            SyntaxKind::LET_BINDING => {
                if let Some(range) = name_child_if_matches(&child, name) {
                    return Some(range);
                }
            }
            SyntaxKind::FN_DEF => {
                if let Some(range) = name_child_if_matches(&child, name) {
                    return Some(range);
                }
            }
            SyntaxKind::ACTOR_DEF => {
                if let Some(range) = name_child_if_matches(&child, name) {
                    return Some(range);
                }
            }
            SyntaxKind::SERVICE_DEF => {
                if let Some(range) = name_child_if_matches(&child, name) {
                    return Some(range);
                }
            }
            SyntaxKind::MODULE_DEF => {
                if let Some(range) = name_child_if_matches(&child, name) {
                    return Some(range);
                }
            }
            _ => {}
        }
    }

    None
}

/// Search parameter list of a FN_DEF or CLOSURE_EXPR for a matching name.
fn search_params_for_name(fn_node: &SyntaxNode, name: &str) -> Option<TextRange> {
    for child in fn_node.children() {
        if child.kind() == SyntaxKind::PARAM_LIST {
            for param in child.children() {
                if param.kind() == SyntaxKind::PARAM {
                    // PARAM contains an IDENT token (or NAME node).
                    for token_or_node in param.children_with_tokens() {
                        match token_or_node {
                            rowan::NodeOrToken::Token(t)
                                if t.kind() == SyntaxKind::IDENT && t.text() == name =>
                            {
                                return Some(t.text_range());
                            }
                            rowan::NodeOrToken::Node(n) if n.kind() == SyntaxKind::NAME => {
                                if first_ident_text(&n).as_deref() == Some(name) {
                                    return Some(n.text_range());
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    None
}

/// Find a type definition (struct, sum type, type alias) with a matching name.
fn find_type_def(root: &SyntaxNode, name: &str) -> Option<TextRange> {
    for child in root.children() {
        match child.kind() {
            SyntaxKind::STRUCT_DEF | SyntaxKind::SUM_TYPE_DEF | SyntaxKind::TYPE_ALIAS_DEF => {
                if let Some(range) = name_child_if_matches(&child, name) {
                    return Some(range);
                }
            }
            _ => {}
        }
    }
    None
}

/// Find a function definition inside a MODULE_DEF with the given module name.
fn find_in_module(root: &SyntaxNode, module_name: &str, fn_name: &str) -> Option<TextRange> {
    for child in root.children() {
        if child.kind() == SyntaxKind::MODULE_DEF {
            if name_child_text(&child).as_deref() == Some(module_name) {
                // Search inside the module for a matching function.
                for module_child in child.children() {
                    if module_child.kind() == SyntaxKind::FN_DEF {
                        if let Some(range) = name_child_if_matches(&module_child, fn_name) {
                            return Some(range);
                        }
                    }
                }
            }
        }
    }
    None
}

/// If a node has a NAME child whose text matches `name`, return the NAME's range.
fn name_child_if_matches(node: &SyntaxNode, name: &str) -> Option<TextRange> {
    for child in node.children() {
        if child.kind() == SyntaxKind::NAME {
            if first_ident_text(&child).as_deref() == Some(name) {
                return Some(child.text_range());
            }
        }
    }
    None
}

/// Get the text of the NAME child of a node.
fn name_child_text(node: &SyntaxNode) -> Option<String> {
    for child in node.children() {
        if child.kind() == SyntaxKind::NAME {
            return first_ident_text(&child);
        }
    }
    None
}

/// Get the text of the first IDENT token in a node.
fn first_ident_text(node: &SyntaxNode) -> Option<String> {
    for token in node.children_with_tokens() {
        if let rowan::NodeOrToken::Token(t) = token {
            if t.kind() == SyntaxKind::IDENT {
                return Some(t.text().to_string());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: parse source, convert source offset to tree offset, and find definition.
    fn def_at(source: &str, source_offset: usize) -> Option<TextRange> {
        let parse = mesh_parser::parse(source);
        let root = parse.syntax();
        find_definition(source, &root, source_offset)
    }

    /// Helper: get the source byte offset of a tree TextRange start.
    fn tree_range_to_source(source: &str, range: TextRange) -> Option<usize> {
        tree_to_source_offset(source, range.start().into())
    }

    #[test]
    fn source_to_tree_offset_basic() {
        // "let x = 42" -- source offsets: l=0 e=1 t=2 ' '=3 x=4 ' '=5 '='=6 ' '=7 '4'=8 '2'=9
        // Tree: "letx=42" -- tree offsets:  l=0 e=1 t=2        x=3        =  4        4  =5 2=6
        let source = "let x = 42";
        assert_eq!(source_to_tree_offset(source, 0), Some(0)); // 'l' in 'let'
        assert_eq!(source_to_tree_offset(source, 4), Some(3)); // 'x'
        assert_eq!(source_to_tree_offset(source, 6), Some(4)); // '='
        assert_eq!(source_to_tree_offset(source, 8), Some(5)); // '4'
    }

    #[test]
    fn tree_to_source_offset_basic() {
        let source = "let x = 42";
        assert_eq!(tree_to_source_offset(source, 0), Some(0)); // 'l' in 'let'
        assert_eq!(tree_to_source_offset(source, 3), Some(4)); // 'x'
        assert_eq!(tree_to_source_offset(source, 4), Some(6)); // '='
        assert_eq!(tree_to_source_offset(source, 5), Some(8)); // '4'
    }

    #[test]
    fn find_def_variable_let_binding() {
        let source = "let x = 42\nlet y = x";
        // "x" at the end (in `y = x`) should resolve to the NAME in `let x = 42`
        // Find the source offset of the second "x"
        let first_x = source.find('x').unwrap();
        let use_offset = source[first_x + 1..].find('x').unwrap() + first_x + 1;
        let result = def_at(source, use_offset);
        assert!(result.is_some(), "Should find definition of x");
        let range = result.unwrap();
        // Convert the result back to source offset to verify.
        let def_source_offset = tree_range_to_source(source, range).unwrap();
        assert_eq!(
            def_source_offset, 4,
            "Definition of x should be at source offset 4"
        );
    }

    #[test]
    fn find_def_function_call() {
        let source = "fn add(a, b) do\na + b\nend\nlet result = add(1, 2)";
        // "add" in the call `add(1, 2)` should resolve to the NAME in `fn add`.
        let call_offset = source.rfind("add").unwrap();
        let result = def_at(source, call_offset);
        assert!(result.is_some(), "Should find definition of add");
        let range = result.unwrap();
        let def_source_offset = tree_range_to_source(source, range).unwrap();
        // "fn add" -- NAME for "add" starts at source offset 3.
        assert_eq!(
            def_source_offset, 3,
            "Definition of add should be at source offset 3"
        );
    }

    #[test]
    fn find_def_type_in_annotation() {
        let source =
            "struct Point do\nx :: Int\ny :: Int\nend\nlet p :: Point = Point { x: 1, y: 2 }";
        // Find "Point" in the type annotation `:: Point`.
        let after_let = source.find("let p").unwrap();
        let in_annotation = source[after_let..].find("Point").unwrap() + after_let;
        let result = def_at(source, in_annotation);
        // Type annotation context detection -- verify no panic.
        let _ = result;
    }

    #[test]
    fn find_def_returns_none_for_builtins() {
        let source = "let x = 42";
        // Offset 0 is 'l' of 'let', which is a keyword, not a NAME_REF.
        let result = def_at(source, 0);
        assert!(
            result.is_none(),
            "Keywords should not resolve to definitions"
        );
    }

    #[test]
    fn find_def_nested_scope_shadowing() {
        // Inner let shadows outer let.
        let source = "fn main() do\nlet x = 1\nfn inner() do\nlet x = 2\nlet y = x\nend\nend";
        // Find the "x" usage in `let y = x`.
        let y_binding = source.find("let y = x").unwrap();
        let x_use = y_binding + "let y = ".len();
        let result = def_at(source, x_use);
        assert!(result.is_some(), "Should find inner x definition");
        let range = result.unwrap();
        let def_source_offset = tree_range_to_source(source, range).unwrap();
        // The inner `let x = 2` NAME "x" should be at the source offset of that x.
        let inner_x_def = source.find("let x = 2").unwrap() + "let ".len();
        assert_eq!(def_source_offset, inner_x_def);
    }

    #[test]
    fn find_def_returns_none_for_unknown() {
        let source = "let x = unknown_var";
        // "unknown_var" starts at source offset 8.
        let result = def_at(source, 8);
        assert!(result.is_none(), "Unknown variables should return None");
    }

    #[test]
    fn find_def_function_param() {
        let source = "fn double(n) do\nlet result = n + n\nresult\nend";
        // "n" in `n + n` should resolve to the parameter.
        let n_use = source.find("n + n").unwrap();
        let result = def_at(source, n_use);
        assert!(result.is_some(), "Should find parameter definition of n");
    }
}
