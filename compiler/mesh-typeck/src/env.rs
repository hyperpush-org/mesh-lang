//! Type environment with scope stack.
//!
//! The type environment maps variable names to their type schemes. It uses
//! a scope stack (Vec of HashMaps) so that entering a new scope (function,
//! block, let binding) pushes a new frame, and leaving pops it. Lookups
//! search from the innermost scope outward.

use rustc_hash::FxHashMap;

use crate::ty::Scheme;

/// A type environment: a stack of scopes mapping names to type schemes.
///
/// The stack grows as we enter nested scopes (functions, blocks) and shrinks
/// as we leave. Variable lookup searches from the top of the stack downward,
/// implementing lexical scoping.
pub struct TypeEnv {
    /// The scope stack. Index 0 is the outermost (global) scope.
    scopes: Vec<FxHashMap<String, Scheme>>,
}

impl TypeEnv {
    /// Create a new type environment with one empty global scope.
    pub fn new() -> Self {
        TypeEnv {
            scopes: vec![FxHashMap::default()],
        }
    }

    /// Push a new empty scope onto the stack.
    pub fn push_scope(&mut self) {
        self.scopes.push(FxHashMap::default());
    }

    /// Pop the top scope from the stack.
    ///
    /// # Panics
    ///
    /// Panics if called when only the global scope remains.
    pub fn pop_scope(&mut self) {
        assert!(self.scopes.len() > 1, "cannot pop the global scope");
        self.scopes.pop();
    }

    /// Insert a name-scheme binding into the current (topmost) scope.
    pub fn insert(&mut self, name: String, scheme: Scheme) {
        self.scopes
            .last_mut()
            .expect("scope stack should never be empty")
            .insert(name, scheme);
    }

    /// Look up a name, searching from the innermost scope outward.
    ///
    /// Returns the type scheme if found, or `None` if the name is not
    /// in any scope.
    pub fn lookup(&self, name: &str) -> Option<&Scheme> {
        for scope in self.scopes.iter().rev() {
            if let Some(scheme) = scope.get(name) {
                return Some(scheme);
            }
        }
        None
    }

    /// Number of scopes on the stack.
    pub fn depth(&self) -> usize {
        self.scopes.len()
    }
}

impl Default for TypeEnv {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ty::{Scheme, Ty};

    #[test]
    fn lookup_in_current_scope() {
        let mut env = TypeEnv::new();
        env.insert("x".into(), Scheme::mono(Ty::int()));

        assert!(env.lookup("x").is_some());
        assert!(env.lookup("y").is_none());
    }

    #[test]
    fn lookup_in_outer_scope() {
        let mut env = TypeEnv::new();
        env.insert("x".into(), Scheme::mono(Ty::int()));

        env.push_scope();
        // x should still be visible from the outer scope.
        assert!(env.lookup("x").is_some());
    }

    #[test]
    fn shadowing() {
        let mut env = TypeEnv::new();
        env.insert("x".into(), Scheme::mono(Ty::int()));

        env.push_scope();
        env.insert("x".into(), Scheme::mono(Ty::string()));

        // Inner scope x should shadow outer.
        let scheme = env.lookup("x").unwrap();
        assert_eq!(scheme.ty, Ty::string());

        env.pop_scope();
        // After popping, outer x is visible again.
        let scheme = env.lookup("x").unwrap();
        assert_eq!(scheme.ty, Ty::int());
    }

    #[test]
    fn scope_cleanup() {
        let mut env = TypeEnv::new();
        env.push_scope();
        env.insert("y".into(), Scheme::mono(Ty::bool()));
        assert!(env.lookup("y").is_some());

        env.pop_scope();
        // y should no longer be visible.
        assert!(env.lookup("y").is_none());
    }

    #[test]
    #[should_panic(expected = "cannot pop the global scope")]
    fn pop_global_scope_panics() {
        let mut env = TypeEnv::new();
        env.pop_scope(); // Should panic.
    }
}
