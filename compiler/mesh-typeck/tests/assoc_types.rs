//! Integration tests for associated types in Mesh trait system.
//!
//! Tests cover:
//! - Declaring associated types in interface definitions
//! - Binding associated types in impl blocks
//! - Self.Item resolution in method return types
//! - Self.Item resolution in method parameter types
//! - Validation: missing associated types
//! - Validation: extra associated types
//! - Type-checking method bodies against resolved associated types

use mesh_typeck::error::TypeError;
use mesh_typeck::ty::Ty;
use mesh_typeck::TypeckResult;

// ── Helpers ────────────────────────────────────────────────────────────

/// Parse Mesh source and run the type checker.
fn check_source(src: &str) -> TypeckResult {
    let parse = mesh_parser::parse(src);
    mesh_typeck::check(&parse)
}

/// Assert that the result has no errors and the final expression type
/// matches the expected type.
fn assert_result_type(result: &TypeckResult, expected: Ty) {
    assert!(
        result.errors.is_empty(),
        "expected no errors, got: {:?}",
        result.errors
    );
    let actual = result
        .result_type
        .as_ref()
        .expect("expected a result type from inference");
    let actual_str = format!("{}", actual);
    let expected_str = format!("{}", expected);
    assert_eq!(
        actual_str, expected_str,
        "expected type `{}`, got `{}`",
        expected_str, actual_str
    );
}

/// Assert that the result contains an error matching the given predicate.
fn assert_has_error<F: Fn(&TypeError) -> bool>(result: &TypeckResult, pred: F, desc: &str) {
    assert!(
        result.errors.iter().any(|e| pred(e)),
        "expected error matching `{}`, got errors: {:?}",
        desc,
        result.errors
    );
}

// ── Interface with associated type ────────────────────────────────────

/// 1. Interface with associated type declaration should register without errors.
#[test]
fn test_interface_with_assoc_type() {
    let result = check_source(
        "interface Iterator do\n\
         \x20 type Item\n\
         \x20 fn next(self) -> Int\n\
         end",
    );
    assert!(
        result.errors.is_empty(),
        "interface with associated type should register without errors, got: {:?}",
        result.errors
    );
    // Verify the trait was registered with the associated type.
    let trait_def = result.trait_registry.get_trait("Iterator");
    assert!(trait_def.is_some(), "Iterator trait should be registered");
    let trait_def = trait_def.unwrap();
    assert_eq!(trait_def.associated_types.len(), 1);
    assert_eq!(trait_def.associated_types[0].name, "Item");
}

/// 2. Impl block with matching associated type binding -- no errors.
#[test]
fn test_impl_with_assoc_type_binding() {
    let result = check_source(
        "interface Iterator do\n\
         \x20 type Item\n\
         \x20 fn next(self) -> Int\n\
         end\n\
         impl Iterator for Int do\n\
         \x20 type Item = String\n\
         \x20 fn next(self) -> Int do\n\
         \x20\x20  42\n\
         \x20 end\n\
         end",
    );
    assert!(
        result.errors.is_empty(),
        "impl with matching associated type should type-check without errors, got: {:?}",
        result.errors
    );
}

/// 3. Missing associated type binding produces MissingAssocType error.
#[test]
fn test_missing_assoc_type_binding() {
    let result = check_source(
        "interface Iterator do\n\
         \x20 type Item\n\
         \x20 fn next(self) -> Int\n\
         end\n\
         impl Iterator for Int do\n\
         \x20 fn next(self) -> Int do\n\
         \x20\x20  42\n\
         \x20 end\n\
         end",
    );
    assert_has_error(
        &result,
        |e| {
            matches!(e, TypeError::MissingAssocType { trait_name, assoc_name, .. }
            if trait_name == "Iterator" && assoc_name == "Item")
        },
        "MissingAssocType for Iterator.Item",
    );
}

/// 4. Extra associated type binding produces ExtraAssocType error.
#[test]
fn test_extra_assoc_type_binding() {
    let result = check_source(
        "interface Printable do\n\
         \x20 fn to_string(self) -> String\n\
         end\n\
         impl Printable for Int do\n\
         \x20 type Output = String\n\
         \x20 fn to_string(self) -> String do\n\
         \x20\x20  \"int\"\n\
         \x20 end\n\
         end",
    );
    assert_has_error(
        &result,
        |e| {
            matches!(e, TypeError::ExtraAssocType { trait_name, assoc_name, .. }
            if trait_name == "Printable" && assoc_name == "Output")
        },
        "ExtraAssocType for Output in Printable",
    );
}

/// 5. Self.Item resolution in return type -- method returning Self.Item
/// resolves to the concrete bound type.
#[test]
fn test_self_item_return_type() {
    let result = check_source(
        "interface Container do\n\
         \x20 type Item\n\
         \x20 fn get(self) -> String\n\
         end\n\
         impl Container for Int do\n\
         \x20 type Item = String\n\
         \x20 fn get(self) -> Self.Item do\n\
         \x20\x20  \"hello\"\n\
         \x20 end\n\
         end\n\
         42",
    );
    // The impl should type-check: Self.Item resolves to String,
    // the trait declares get -> String, so no signature mismatch,
    // and the body returns "hello" which is a String.
    assert!(
        result.errors.is_empty(),
        "Self.Item in return type should resolve, got: {:?}",
        result.errors
    );
}

/// 6. Self.Item in parameter type resolves to the bound type.
#[test]
fn test_self_item_param_type() {
    let result = check_source(
        "interface Sink do\n\
         \x20 type Item\n\
         \x20 fn push(self, val: Int) -> Int\n\
         end\n\
         impl Sink for Int do\n\
         \x20 type Item = String\n\
         \x20 fn push(self, val: Self.Item) -> Int do\n\
         \x20\x20  42\n\
         \x20 end\n\
         end\n\
         42",
    );
    assert!(
        result.errors.is_empty(),
        "Self.Item in param type should resolve, got: {:?}",
        result.errors
    );
}

/// 7. Multiple associated types in a single interface.
#[test]
fn test_multiple_assoc_types() {
    let result = check_source(
        "interface Mapper do\n\
         \x20 type Input\n\
         \x20 type Output\n\
         \x20 fn apply(self) -> Int\n\
         end\n\
         impl Mapper for Int do\n\
         \x20 type Input = String\n\
         \x20 type Output = Bool\n\
         \x20 fn apply(self) -> Int do\n\
         \x20\x20  42\n\
         \x20 end\n\
         end",
    );
    assert!(
        result.errors.is_empty(),
        "multiple associated types should work, got: {:?}",
        result.errors
    );
    // Verify both types were registered.
    let trait_def = result.trait_registry.get_trait("Mapper").unwrap();
    assert_eq!(trait_def.associated_types.len(), 2);
}

/// 8. resolve_associated_type on TraitRegistry returns the bound type.
#[test]
fn test_resolve_associated_type_api() {
    let result = check_source(
        "interface Container do\n\
         \x20 type Item\n\
         \x20 fn get(self) -> Int\n\
         end\n\
         impl Container for Int do\n\
         \x20 type Item = String\n\
         \x20 fn get(self) -> Int do\n\
         \x20\x20  42\n\
         \x20 end\n\
         end",
    );
    assert!(result.errors.is_empty());

    // Use the resolve_associated_type API.
    let resolved = result
        .trait_registry
        .resolve_associated_type("Container", "Item", &Ty::int());
    assert_eq!(resolved, Some(Ty::string()));

    // Non-existent bindings should return None.
    let not_found =
        result
            .trait_registry
            .resolve_associated_type("Container", "NonExistent", &Ty::int());
    assert_eq!(not_found, None);
}

/// 9. Interface with no associated types -- backward compat.
#[test]
fn test_interface_no_assoc_types_compat() {
    let result = check_source(
        "interface Greet do\n\
         \x20 fn hello(self) -> String\n\
         end\n\
         impl Greet for Int do\n\
         \x20 fn hello(self) -> String do\n\
         \x20\x20  \"hi\"\n\
         \x20 end\n\
         end",
    );
    assert!(
        result.errors.is_empty(),
        "backward compat: interface with no assoc types should work, got: {:?}",
        result.errors
    );
}
