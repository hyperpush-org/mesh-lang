//! REPL session state management.
//!
//! Tracks accumulated definitions, evaluation counter, and result history
//! across REPL interactions. Each new expression is wrapped in a unique
//! function name so it can be JIT-compiled and executed.

/// Persistent state for a REPL session.
///
/// Accumulates top-level definitions (functions, types, structs) so that
/// subsequent expressions can reference them. Each expression evaluation
/// gets a unique wrapper function name for JIT execution.
pub struct ReplSession {
    /// Accumulated source context (previous definitions).
    definitions: Vec<String>,
    /// Counter for generating unique wrapper function names.
    eval_counter: u64,
    /// History of evaluated results with types: (value_repr, type_name).
    results: Vec<(String, String)>,
}

impl ReplSession {
    /// Create a new empty REPL session.
    pub fn new() -> Self {
        Self {
            definitions: Vec::new(),
            eval_counter: 0,
            results: Vec::new(),
        }
    }

    /// Store a top-level definition (fn, let, type, struct, etc.) for future inputs.
    pub fn add_definition(&mut self, source: &str) {
        self.definitions.push(source.to_string());
    }

    /// Get all accumulated definitions as a single source string.
    pub fn definitions_source(&self) -> String {
        self.definitions.join("\n")
    }

    /// Wrap an expression in a unique `__repl_eval_N` function that includes
    /// all prior definitions as context.
    ///
    /// Returns `(full_source, wrapper_fn_name)`.
    pub fn wrap_expression(&mut self, expr: &str) -> (String, String) {
        let fn_name = format!("__repl_eval_{}", self.eval_counter);
        self.eval_counter += 1;

        let mut source = String::new();

        // Prepend all accumulated definitions
        for def in &self.definitions {
            source.push_str(def);
            source.push('\n');
        }

        // Wrap expression in a named function
        source.push_str(&format!("fn {}() do\n  {}\nend\n", fn_name, expr));

        (source, fn_name)
    }

    /// Record a result value and its type in the session history.
    pub fn record_result(&mut self, value: String, ty: String) {
        self.results.push((value, ty));
    }

    /// Get the result history.
    pub fn results(&self) -> &[(String, String)] {
        &self.results
    }

    /// Get the current evaluation counter (number of expressions evaluated).
    pub fn eval_counter(&self) -> u64 {
        self.eval_counter
    }

    /// Reset the session, clearing all definitions and history.
    pub fn reset(&mut self) {
        self.definitions.clear();
        self.eval_counter = 0;
        self.results.clear();
    }
}

impl Default for ReplSession {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_session_is_empty() {
        let session = ReplSession::new();
        assert!(session.definitions_source().is_empty());
        assert_eq!(session.eval_counter(), 0);
        assert!(session.results().is_empty());
    }

    #[test]
    fn test_add_definition() {
        let mut session = ReplSession::new();
        session.add_definition("fn add(a :: Int, b :: Int) :: Int do a + b end");
        assert!(session.definitions_source().contains("fn add"));
    }

    #[test]
    fn test_wrap_expression_basic() {
        let mut session = ReplSession::new();
        let (source, fn_name) = session.wrap_expression("1 + 2");
        assert_eq!(fn_name, "__repl_eval_0");
        assert!(source.contains("fn __repl_eval_0() do"));
        assert!(source.contains("1 + 2"));
    }

    #[test]
    fn test_wrap_expression_increments_counter() {
        let mut session = ReplSession::new();
        let (_, name1) = session.wrap_expression("1");
        let (_, name2) = session.wrap_expression("2");
        assert_eq!(name1, "__repl_eval_0");
        assert_eq!(name2, "__repl_eval_1");
        assert_eq!(session.eval_counter(), 2);
    }

    #[test]
    fn test_wrap_expression_includes_definitions() {
        let mut session = ReplSession::new();
        session.add_definition("fn double(x :: Int) :: Int do x * 2 end");
        let (source, _) = session.wrap_expression("double(5)");
        assert!(source.contains("fn double"));
        assert!(source.contains("double(5)"));
    }

    #[test]
    fn test_record_result() {
        let mut session = ReplSession::new();
        session.record_result("42".to_string(), "Int".to_string());
        session.record_result("hello".to_string(), "String".to_string());
        assert_eq!(session.results().len(), 2);
        assert_eq!(session.results()[0], ("42".to_string(), "Int".to_string()));
    }

    #[test]
    fn test_reset_clears_everything() {
        let mut session = ReplSession::new();
        session.add_definition("fn foo() do 1 end");
        session.record_result("1".to_string(), "Int".to_string());
        let _ = session.wrap_expression("1");

        session.reset();
        assert!(session.definitions_source().is_empty());
        assert_eq!(session.eval_counter(), 0);
        assert!(session.results().is_empty());
    }
}
