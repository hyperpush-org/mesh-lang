//! Maranget's usefulness algorithm for exhaustiveness and redundancy checking.
//!
//! This module implements Algorithm U from Luc Maranget's paper
//! "Warnings for Pattern Matching" (2007). It operates on an abstract
//! pattern representation (`Pat`), not AST nodes directly. Translation
//! from AST patterns to `Pat` happens elsewhere (Plan 04-04).
//!
//! The core predicate `is_useful(matrix, row, type_info)` determines whether
//! a new pattern row adds any coverage to the existing matrix. Both
//! exhaustiveness (is wildcard useful after all arms?) and redundancy
//! (is each arm useful given prior arms?) are expressed via `is_useful`.

use rustc_hash::{FxHashMap, FxHashSet};

/// The kind of a literal pattern value.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LitKind {
    Int,
    Float,
    Bool,
    String,
}

/// Abstract pattern representation for exhaustiveness checking.
///
/// These are NOT AST nodes -- they are a simplified representation
/// used only by the usefulness algorithm.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Pat {
    /// Matches anything (wildcard `_` or variable binding).
    Wildcard,
    /// Matches a specific constructor with arguments.
    Constructor {
        name: String,
        type_name: String,
        args: Vec<Pat>,
    },
    /// Matches a specific literal value.
    Literal { value: String, ty: LitKind },
    /// Matches any of the alternatives (or-pattern).
    Or { alternatives: Vec<Pat> },
}

/// A row in the pattern matrix (one match arm's patterns).
pub type PatternRow = Vec<Pat>;

/// The pattern matrix: each row corresponds to one match arm.
#[derive(Clone, Debug)]
pub struct PatternMatrix {
    pub rows: Vec<PatternRow>,
}

/// Signature of a constructor (name + arity).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConstructorSig {
    pub name: String,
    pub arity: usize,
}

/// Type information needed for exhaustiveness checking.
///
/// Tells the algorithm what constructors a type has, so it can
/// determine if all cases are covered.
#[derive(Clone, Debug)]
pub enum TypeInfo {
    /// A sum type with known, finite variants.
    SumType { variants: Vec<ConstructorSig> },
    /// Bool type (two constructors: true, false).
    Bool,
    /// A literal type with infinite inhabitants (Int, Float, String).
    Infinite,
}

/// Registry mapping type names to their complete constructor sets.
///
/// This is essential for nested specialization: when checking
/// `Option<Shape>` patterns, after specializing by `Some`, the inner
/// column needs the complete set of `Shape` constructors. The registry
/// provides this lookup.
#[derive(Clone, Debug, Default)]
pub struct TypeRegistry {
    types: FxHashMap<String, TypeInfo>,
}

impl TypeRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a type's info in the registry.
    pub fn register(&mut self, name: impl Into<String>, info: TypeInfo) {
        self.types.insert(name.into(), info);
    }

    /// Look up type info by name.
    pub fn lookup(&self, name: &str) -> Option<&TypeInfo> {
        self.types.get(name)
    }
}

// ── Constructor abstraction ──────────────────────────────────────────

/// A unified constructor representation used internally by the algorithm.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Constructor {
    /// A named constructor from a sum type (or Bool literal).
    Named { name: String, arity: usize },
    /// A literal value (acts as a nullary constructor).
    Literal { value: String, ty: LitKind },
}

impl Constructor {
    fn arity(&self) -> usize {
        match self {
            Constructor::Named { arity, .. } => *arity,
            Constructor::Literal { .. } => 0,
        }
    }

    fn name_key(&self) -> String {
        match self {
            Constructor::Named { name, .. } => name.clone(),
            Constructor::Literal { value, ty } => format!("{:?}:{}", ty, value),
        }
    }
}

// ── Specialize & Default matrices ────────────────────────────────────

/// Specialize the matrix by a constructor.
///
/// For each row in the matrix:
/// - If row[0] matches the same constructor: replace row[0] with its args, keep rest.
/// - If row[0] is Wildcard: replace row[0] with N wildcards (N = ctor arity), keep rest.
/// - If row[0] is a different constructor: drop the row.
/// - If row[0] is Or: expand each alternative.
fn specialize_matrix(matrix: &PatternMatrix, ctor: &Constructor) -> PatternMatrix {
    let mut rows = Vec::new();
    for row in &matrix.rows {
        specialize_row_into(&mut rows, row, ctor);
    }
    PatternMatrix { rows }
}

/// Specialize a single row by a constructor, appending results to `out`.
fn specialize_row_into(out: &mut Vec<PatternRow>, row: &[Pat], ctor: &Constructor) {
    if row.is_empty() {
        return;
    }

    let head = &row[0];
    let rest = &row[1..];

    match head {
        Pat::Constructor { name, args, .. } => {
            let head_key = name.clone();
            if head_key == ctor.name_key() {
                let mut new_row: Vec<Pat> = args.clone();
                new_row.extend_from_slice(rest);
                out.push(new_row);
            }
            // Different constructor: drop the row
        }
        Pat::Literal {
            value,
            ty: LitKind::Bool,
        } => {
            // Bool literals treated as named constructors "true"/"false"
            if *value == ctor.name_key() {
                out.push(rest.to_vec());
            }
        }
        Pat::Literal { value, ty } => {
            let head_key = format!("{:?}:{}", ty, value);
            if head_key == ctor.name_key() {
                out.push(rest.to_vec());
            }
        }
        Pat::Wildcard => {
            // Wildcard matches any constructor: expand to N wildcards
            let mut new_row: Vec<Pat> = vec![Pat::Wildcard; ctor.arity()];
            new_row.extend_from_slice(rest);
            out.push(new_row);
        }
        Pat::Or { alternatives } => {
            // Expand each alternative
            for alt in alternatives {
                let mut expanded_row = vec![alt.clone()];
                expanded_row.extend_from_slice(rest);
                specialize_row_into(out, &expanded_row, ctor);
            }
        }
    }
}

/// Compute the default matrix.
///
/// For each row in the matrix:
/// - If row[0] is Wildcard: keep row[1..]
/// - If row[0] is Or: expand each alternative, keep wildcards
/// - Otherwise: drop the row
fn default_matrix(matrix: &PatternMatrix) -> PatternMatrix {
    let mut rows = Vec::new();
    for row in &matrix.rows {
        if row.is_empty() {
            continue;
        }
        match &row[0] {
            Pat::Wildcard => {
                rows.push(row[1..].to_vec());
            }
            Pat::Or { alternatives } => {
                for alt in alternatives {
                    if matches!(alt, Pat::Wildcard) {
                        rows.push(row[1..].to_vec());
                        break;
                    }
                }
            }
            _ => {}
        }
    }
    PatternMatrix { rows }
}

// ── Type info inference for nested columns ───────────────────────────

/// Infer the `TypeInfo` for a column created by specialization,
/// using the type registry for complete constructor sets.
fn infer_type_info_for_column(
    matrix: &PatternMatrix,
    row: &[Pat],
    col: usize,
    registry: &TypeRegistry,
) -> TypeInfo {
    // Check if any pattern in this column has a type_name we can look up
    let type_name = find_type_name_in_column(matrix, row, col);

    if let Some(ref tn) = type_name {
        if let Some(info) = registry.lookup(tn) {
            return info.clone();
        }
    }

    // Check for bool literals
    if check_column_for_bool(matrix, row, col) {
        return TypeInfo::Bool;
    }

    // Default: infinite type
    TypeInfo::Infinite
}

/// Find the type_name of constructor patterns in a specific column.
fn find_type_name_in_column(matrix: &PatternMatrix, row: &[Pat], col: usize) -> Option<String> {
    for mrow in &matrix.rows {
        if col < mrow.len() {
            if let Some(tn) = extract_type_name(&mrow[col]) {
                return Some(tn);
            }
        }
    }
    if col < row.len() {
        if let Some(tn) = extract_type_name(&row[col]) {
            return Some(tn);
        }
    }
    None
}

/// Check if a column contains bool literal patterns.
fn check_column_for_bool(matrix: &PatternMatrix, row: &[Pat], col: usize) -> bool {
    for mrow in &matrix.rows {
        if col < mrow.len() {
            if matches!(
                &mrow[col],
                Pat::Literal {
                    ty: LitKind::Bool,
                    ..
                }
            ) {
                return true;
            }
        }
    }
    if col < row.len() {
        if matches!(
            &row[col],
            Pat::Literal {
                ty: LitKind::Bool,
                ..
            }
        ) {
            return true;
        }
    }
    false
}

/// Extract the type_name from a pattern, if it is a constructor.
fn extract_type_name(pat: &Pat) -> Option<String> {
    match pat {
        Pat::Constructor { type_name, .. } if !type_name.is_empty() => Some(type_name.clone()),
        Pat::Or { alternatives } => {
            for alt in alternatives {
                if let Some(tn) = extract_type_name(alt) {
                    return Some(tn);
                }
            }
            None
        }
        _ => None,
    }
}

// ── Collect head constructors ────────────────────────────────────────

/// Collect all constructors that appear in the first column of the matrix.
fn collect_head_constructors(matrix: &PatternMatrix) -> Vec<Constructor> {
    let mut seen = FxHashSet::default();
    let mut result = Vec::new();
    for row in &matrix.rows {
        if row.is_empty() {
            continue;
        }
        collect_constructors_from_pat(&row[0], &mut seen, &mut result);
    }
    result
}

/// Recursively collect constructors from a pattern (handling Or).
fn collect_constructors_from_pat(
    pat: &Pat,
    seen: &mut FxHashSet<String>,
    result: &mut Vec<Constructor>,
) {
    match pat {
        Pat::Constructor { name, args, .. } => {
            let ctor = Constructor::Named {
                name: name.clone(),
                arity: args.len(),
            };
            if seen.insert(ctor.name_key()) {
                result.push(ctor);
            }
        }
        Pat::Literal {
            value,
            ty: LitKind::Bool,
        } => {
            let ctor = Constructor::Named {
                name: value.clone(),
                arity: 0,
            };
            if seen.insert(ctor.name_key()) {
                result.push(ctor);
            }
        }
        Pat::Literal { value, ty } => {
            let ctor = Constructor::Literal {
                value: value.clone(),
                ty: ty.clone(),
            };
            if seen.insert(ctor.name_key()) {
                result.push(ctor);
            }
        }
        Pat::Or { alternatives } => {
            for alt in alternatives {
                collect_constructors_from_pat(alt, seen, result);
            }
        }
        Pat::Wildcard => {}
    }
}

// ── Registry building from patterns ──────────────────────────────────

/// Recursively scan a pattern, registering constructor types in the registry.
fn collect_types_from_pattern(pat: &Pat, registry: &mut TypeRegistry) {
    match pat {
        Pat::Constructor {
            name,
            type_name,
            args,
        } => {
            if !type_name.is_empty() {
                let entry =
                    registry
                        .types
                        .entry(type_name.clone())
                        .or_insert_with(|| TypeInfo::SumType {
                            variants: Vec::new(),
                        });
                if let TypeInfo::SumType { variants } = entry {
                    if !variants.iter().any(|v| v.name == *name) {
                        variants.push(ConstructorSig {
                            name: name.clone(),
                            arity: args.len(),
                        });
                    }
                }
            }
            for arg in args {
                collect_types_from_pattern(arg, registry);
            }
        }
        Pat::Or { alternatives } => {
            for alt in alternatives {
                collect_types_from_pattern(alt, registry);
            }
        }
        Pat::Wildcard | Pat::Literal { .. } => {}
    }
}

// ── Core algorithm ───────────────────────────────────────────────────

/// Core usefulness predicate (Algorithm U).
///
/// Returns `true` if `row` is useful with respect to `matrix` --
/// i.e., there exists a value matched by `row` but not by any row
/// in `matrix`.
///
/// Builds an internal type registry from the patterns for nested type
/// resolution. For complete nested exhaustiveness, use the registry-based
/// functions `check_exhaustiveness` and `check_redundancy` instead.
pub fn is_useful(matrix: &PatternMatrix, row: &[Pat], type_info: &[TypeInfo]) -> bool {
    let mut registry = TypeRegistry::new();
    for mrow in &matrix.rows {
        for pat in mrow {
            collect_types_from_pattern(pat, &mut registry);
        }
    }
    for pat in row {
        collect_types_from_pattern(pat, &mut registry);
    }
    is_useful_inner(matrix, row, type_info, &registry)
}

/// Internal recursive implementation of `is_useful` with an explicit registry.
fn is_useful_inner(
    matrix: &PatternMatrix,
    row: &[Pat],
    type_info: &[TypeInfo],
    registry: &TypeRegistry,
) -> bool {
    // Base case 1: empty matrix (0 rows) -- any pattern is useful
    if matrix.rows.is_empty() {
        return true;
    }

    // Base case 2: empty row (0 columns)
    if row.is_empty() {
        return false;
    }

    let head = &row[0];
    let col_type = type_info.first();

    match head {
        // Case 3a: Constructor -- specialize by it
        Pat::Constructor { name, args, .. } => {
            let ctor = Constructor::Named {
                name: name.clone(),
                arity: args.len(),
            };
            let spec_matrix = specialize_matrix(matrix, &ctor);
            let mut spec_row: Vec<Pat> = args.clone();
            spec_row.extend_from_slice(&row[1..]);

            let inner_type_info = build_specialized_type_info(
                &spec_matrix,
                &spec_row,
                ctor.arity(),
                &type_info[1..],
                registry,
            );

            is_useful_inner(&spec_matrix, &spec_row, &inner_type_info, registry)
        }

        // Case 3d: Bool literal -- treated as named constructor
        Pat::Literal {
            value,
            ty: LitKind::Bool,
        } => {
            let ctor = Constructor::Named {
                name: value.clone(),
                arity: 0,
            };
            let spec_matrix = specialize_matrix(matrix, &ctor);
            is_useful_inner(&spec_matrix, &row[1..], &type_info[1..], registry)
        }

        // Case 3d: Non-bool literal
        Pat::Literal { value, ty } => {
            let ctor = Constructor::Literal {
                value: value.clone(),
                ty: ty.clone(),
            };
            let spec_matrix = specialize_matrix(matrix, &ctor);
            is_useful_inner(&spec_matrix, &row[1..], &type_info[1..], registry)
        }

        // Case 3c: Or-pattern -- useful if ANY alternative is useful
        Pat::Or { alternatives } => alternatives.iter().any(|alt| {
            let mut new_row = vec![alt.clone()];
            new_row.extend_from_slice(&row[1..]);
            is_useful_inner(matrix, &new_row, type_info, registry)
        }),

        // Case 3b: Wildcard
        Pat::Wildcard => {
            let all_constructors = all_constructors_for_type(col_type);

            match all_constructors {
                Some(ctors) => {
                    // Finite type with known constructors from type_info.
                    let head_ctors = collect_head_constructors(matrix);
                    let head_keys: FxHashSet<String> =
                        head_ctors.iter().map(|c| c.name_key()).collect();
                    let all_keys: FxHashSet<String> = ctors.iter().map(|c| c.name_key()).collect();

                    if all_keys.iter().all(|k| head_keys.contains(k)) {
                        // Complete: all constructors covered in matrix.
                        // Wildcard useful if ANY specialization reveals usefulness.
                        ctors.iter().any(|c| {
                            let spec_matrix = specialize_matrix(matrix, c);
                            let mut spec_row = vec![Pat::Wildcard; c.arity()];
                            spec_row.extend_from_slice(&row[1..]);

                            let inner_type_info = build_specialized_type_info(
                                &spec_matrix,
                                &spec_row,
                                c.arity(),
                                &type_info[1..],
                                registry,
                            );

                            is_useful_inner(&spec_matrix, &spec_row, &inner_type_info, registry)
                        })
                    } else {
                        // Incomplete: use default matrix.
                        let def = default_matrix(matrix);
                        is_useful_inner(&def, &row[1..], &type_info[1..], registry)
                    }
                }
                None => {
                    // Infinite type: use default matrix.
                    let def = default_matrix(matrix);
                    is_useful_inner(&def, &row[1..], &type_info[1..], registry)
                }
            }
        }
    }
}

/// Get all constructors for a type from TypeInfo.
fn all_constructors_for_type(col_type: Option<&TypeInfo>) -> Option<Vec<Constructor>> {
    let ti = col_type?;
    match ti {
        TypeInfo::SumType { variants } => Some(
            variants
                .iter()
                .map(|v| Constructor::Named {
                    name: v.name.clone(),
                    arity: v.arity,
                })
                .collect(),
        ),
        TypeInfo::Bool => Some(vec![
            Constructor::Named {
                name: "true".to_string(),
                arity: 0,
            },
            Constructor::Named {
                name: "false".to_string(),
                arity: 0,
            },
        ]),
        TypeInfo::Infinite => None,
    }
}

/// Build type info for columns created by specialization.
fn build_specialized_type_info(
    spec_matrix: &PatternMatrix,
    spec_row: &[Pat],
    ctor_arity: usize,
    remaining_type_info: &[TypeInfo],
    registry: &TypeRegistry,
) -> Vec<TypeInfo> {
    let mut result = Vec::with_capacity(ctor_arity + remaining_type_info.len());
    for col_idx in 0..ctor_arity {
        result.push(infer_type_info_for_column(
            spec_matrix,
            spec_row,
            col_idx,
            registry,
        ));
    }
    result.extend_from_slice(remaining_type_info);
    result
}

// ── Public API ───────────────────────────────────────────────────────

/// Check whether a match expression is exhaustive.
///
/// Returns `None` if exhaustive, or `Some(witnesses)` with example
/// patterns that are not covered.
///
/// The `registry` provides complete constructor sets for all types
/// that may appear in nested patterns. For simple (non-nested) checks,
/// an empty registry suffices.
pub fn check_exhaustiveness(
    arms: &[Pat],
    scrutinee_type: &TypeInfo,
    registry: &TypeRegistry,
) -> Option<Vec<Pat>> {
    let matrix = PatternMatrix {
        rows: arms.iter().map(|arm| vec![arm.clone()]).collect(),
    };

    let wildcard_row = vec![Pat::Wildcard];
    let type_info = vec![scrutinee_type.clone()];

    if is_useful_inner(&matrix, &wildcard_row, &type_info, registry) {
        let witnesses = find_witnesses(arms, scrutinee_type, registry);
        Some(witnesses)
    } else {
        None
    }
}

/// Check for redundant (unreachable) arms in a match expression.
///
/// Returns the indices (0-based) of arms that are unreachable.
///
/// The `registry` provides complete constructor sets for all types.
pub fn check_redundancy(
    arms: &[Pat],
    scrutinee_type: &TypeInfo,
    registry: &TypeRegistry,
) -> Vec<usize> {
    let mut redundant = Vec::new();
    let type_info = vec![scrutinee_type.clone()];

    for i in 0..arms.len() {
        let prior_matrix = PatternMatrix {
            rows: arms[..i].iter().map(|arm| vec![arm.clone()]).collect(),
        };
        let row = vec![arms[i].clone()];

        if !is_useful_inner(&prior_matrix, &row, &type_info, registry) {
            redundant.push(i);
        }
    }

    redundant
}

/// Find witness patterns for non-exhaustive match.
fn find_witnesses(arms: &[Pat], scrutinee_type: &TypeInfo, registry: &TypeRegistry) -> Vec<Pat> {
    match scrutinee_type {
        TypeInfo::SumType { variants } => {
            let mut missing = Vec::new();
            for v in variants {
                let ctor_pat = Pat::Constructor {
                    name: v.name.clone(),
                    type_name: String::new(),
                    args: vec![Pat::Wildcard; v.arity],
                };
                let matrix = PatternMatrix {
                    rows: arms.iter().map(|arm| vec![arm.clone()]).collect(),
                };
                let type_info = vec![scrutinee_type.clone()];

                if is_useful_inner(&matrix, &[ctor_pat.clone()], &type_info, registry) {
                    missing.push(ctor_pat);
                }
            }
            if missing.is_empty() {
                vec![Pat::Wildcard]
            } else {
                missing
            }
        }
        TypeInfo::Bool => {
            let mut missing = Vec::new();
            for val in &["true", "false"] {
                let lit_pat = Pat::Literal {
                    value: val.to_string(),
                    ty: LitKind::Bool,
                };
                let matrix = PatternMatrix {
                    rows: arms.iter().map(|arm| vec![arm.clone()]).collect(),
                };
                let type_info = vec![scrutinee_type.clone()];

                if is_useful_inner(&matrix, &[lit_pat.clone()], &type_info, registry) {
                    missing.push(lit_pat);
                }
            }
            missing
        }
        TypeInfo::Infinite => {
            vec![Pat::Wildcard]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Helper constructors ──────────────────────────────────────────

    fn wildcard() -> Pat {
        Pat::Wildcard
    }

    fn ctor(name: &str, type_name: &str, args: Vec<Pat>) -> Pat {
        Pat::Constructor {
            name: name.to_string(),
            type_name: type_name.to_string(),
            args,
        }
    }

    fn lit_int(value: i64) -> Pat {
        Pat::Literal {
            value: value.to_string(),
            ty: LitKind::Int,
        }
    }

    fn lit_bool(value: bool) -> Pat {
        Pat::Literal {
            value: value.to_string(),
            ty: LitKind::Bool,
        }
    }

    fn or_pat(alternatives: Vec<Pat>) -> Pat {
        Pat::Or { alternatives }
    }

    fn bool_type() -> TypeInfo {
        TypeInfo::Bool
    }

    fn int_type() -> TypeInfo {
        TypeInfo::Infinite
    }

    fn shape_type() -> TypeInfo {
        TypeInfo::SumType {
            variants: vec![
                ConstructorSig {
                    name: "Circle".to_string(),
                    arity: 1,
                },
                ConstructorSig {
                    name: "Point".to_string(),
                    arity: 0,
                },
            ],
        }
    }

    fn option_shape_type() -> TypeInfo {
        TypeInfo::SumType {
            variants: vec![
                ConstructorSig {
                    name: "Some".to_string(),
                    arity: 1,
                },
                ConstructorSig {
                    name: "None".to_string(),
                    arity: 0,
                },
            ],
        }
    }

    fn matrix(rows: Vec<Vec<Pat>>) -> PatternMatrix {
        PatternMatrix { rows }
    }

    /// Registry with Shape and Option type info for nested tests.
    fn test_registry() -> TypeRegistry {
        let mut reg = TypeRegistry::new();
        reg.register("Shape", shape_type());
        reg.register("Option", option_shape_type());
        reg
    }

    fn empty_registry() -> TypeRegistry {
        TypeRegistry::new()
    }

    // ── is_useful base cases ─────────────────────────────────────────

    #[test]
    fn test_is_useful_empty_matrix_returns_true() {
        // Any pattern is useful against an empty matrix
        let m = matrix(vec![]);
        assert!(is_useful(&m, &[wildcard()], &[int_type()]));
    }

    #[test]
    fn test_is_useful_empty_row_returns_false() {
        // No more columns to match -- row is not useful
        let m = matrix(vec![vec![]]);
        assert!(!is_useful(&m, &[], &[]));
    }

    #[test]
    fn test_is_useful_empty_matrix_empty_row_returns_true() {
        // 0 rows, 0 columns: pattern is useful (no existing coverage)
        let m = matrix(vec![]);
        assert!(is_useful(&m, &[], &[]));
    }

    // ── Bool exhaustiveness ──────────────────────────────────────────

    #[test]
    fn test_bool_exhaustive() {
        // match x { true -> ..., false -> ... } is exhaustive
        let result = check_exhaustiveness(
            &[lit_bool(true), lit_bool(false)],
            &bool_type(),
            &empty_registry(),
        );
        assert!(result.is_none(), "Bool [true, false] should be exhaustive");
    }

    #[test]
    fn test_bool_non_exhaustive() {
        // match x { true -> ... } is NOT exhaustive, missing false
        let result = check_exhaustiveness(&[lit_bool(true)], &bool_type(), &empty_registry());
        assert!(result.is_some(), "Bool [true] should NOT be exhaustive");
        let witnesses = result.unwrap();
        assert!(!witnesses.is_empty());
    }

    #[test]
    fn test_bool_wildcard_exhaustive() {
        // match x { _ -> ... } is exhaustive for Bool
        let result = check_exhaustiveness(&[wildcard()], &bool_type(), &empty_registry());
        assert!(result.is_none(), "Bool [_] should be exhaustive");
    }

    // ── Sum type exhaustiveness ──────────────────────────────────────

    #[test]
    fn test_sum_type_exhaustive() {
        // match shape { Circle(_) -> ..., Point -> ... } is exhaustive
        let result = check_exhaustiveness(
            &[
                ctor("Circle", "Shape", vec![wildcard()]),
                ctor("Point", "Shape", vec![]),
            ],
            &shape_type(),
            &test_registry(),
        );
        assert!(
            result.is_none(),
            "Shape [Circle(_), Point] should be exhaustive"
        );
    }

    #[test]
    fn test_sum_type_non_exhaustive() {
        // match shape { Circle(_) -> ... } is NOT exhaustive, missing Point
        let result = check_exhaustiveness(
            &[ctor("Circle", "Shape", vec![wildcard()])],
            &shape_type(),
            &test_registry(),
        );
        assert!(
            result.is_some(),
            "Shape [Circle(_)] should NOT be exhaustive"
        );
    }

    #[test]
    fn test_sum_type_wildcard_exhaustive() {
        // match shape { _ -> ... } is exhaustive
        let result = check_exhaustiveness(&[wildcard()], &shape_type(), &test_registry());
        assert!(result.is_none(), "Shape [_] should be exhaustive");
    }

    // ── Redundancy checking ──────────────────────────────────────────

    #[test]
    fn test_redundant_arm_after_wildcard() {
        // match shape { _ -> ..., Circle(_) -> ... }
        // arm 1 (Circle) is redundant because _ catches everything
        let result = check_redundancy(
            &[wildcard(), ctor("Circle", "Shape", vec![wildcard()])],
            &shape_type(),
            &test_registry(),
        );
        assert_eq!(result, vec![1], "Arm 1 should be redundant after wildcard");
    }

    #[test]
    fn test_no_redundancy() {
        // match shape { Circle(_) -> ..., Point -> ... }
        // No redundant arms
        let result = check_redundancy(
            &[
                ctor("Circle", "Shape", vec![wildcard()]),
                ctor("Point", "Shape", vec![]),
            ],
            &shape_type(),
            &test_registry(),
        );
        assert!(result.is_empty(), "No arms should be redundant");
    }

    #[test]
    fn test_duplicate_arm_redundant() {
        // match shape { Circle(_) -> ..., Circle(_) -> ..., Point -> ... }
        // arm 1 is redundant
        let result = check_redundancy(
            &[
                ctor("Circle", "Shape", vec![wildcard()]),
                ctor("Circle", "Shape", vec![wildcard()]),
                ctor("Point", "Shape", vec![]),
            ],
            &shape_type(),
            &test_registry(),
        );
        assert_eq!(result, vec![1], "Duplicate Circle arm should be redundant");
    }

    // ── Nested patterns ──────────────────────────────────────────────

    #[test]
    fn test_nested_exhaustive() {
        // match opt_shape {
        //   Some(Circle(_)) -> ...,
        //   Some(Point) -> ...,
        //   None -> ...
        // }
        let result = check_exhaustiveness(
            &[
                ctor(
                    "Some",
                    "Option",
                    vec![ctor("Circle", "Shape", vec![wildcard()])],
                ),
                ctor("Some", "Option", vec![ctor("Point", "Shape", vec![])]),
                ctor("None", "Option", vec![]),
            ],
            &option_shape_type(),
            &test_registry(),
        );
        assert!(
            result.is_none(),
            "Option<Shape> fully covered should be exhaustive"
        );
    }

    #[test]
    fn test_nested_non_exhaustive() {
        // match opt_shape {
        //   Some(Circle(_)) -> ...,
        //   None -> ...
        // }
        // Missing Some(Point)
        let result = check_exhaustiveness(
            &[
                ctor(
                    "Some",
                    "Option",
                    vec![ctor("Circle", "Shape", vec![wildcard()])],
                ),
                ctor("None", "Option", vec![]),
            ],
            &option_shape_type(),
            &test_registry(),
        );
        assert!(
            result.is_some(),
            "Option<Shape> missing Some(Point) should NOT be exhaustive"
        );
    }

    // ── Or-patterns ──────────────────────────────────────────────────

    #[test]
    fn test_or_pattern_exhaustive() {
        // match shape { Circle(_) | Point -> ... } is exhaustive
        let result = check_exhaustiveness(
            &[or_pat(vec![
                ctor("Circle", "Shape", vec![wildcard()]),
                ctor("Point", "Shape", vec![]),
            ])],
            &shape_type(),
            &test_registry(),
        );
        assert!(
            result.is_none(),
            "Shape [Circle(_) | Point] should be exhaustive"
        );
    }

    #[test]
    fn test_or_pattern_non_exhaustive() {
        // match shape { Circle(_) | Circle(_) -> ... } NOT exhaustive (missing Point)
        let result = check_exhaustiveness(
            &[or_pat(vec![
                ctor("Circle", "Shape", vec![wildcard()]),
                ctor("Circle", "Shape", vec![wildcard()]),
            ])],
            &shape_type(),
            &test_registry(),
        );
        assert!(
            result.is_some(),
            "Shape [Circle(_) | Circle(_)] should NOT be exhaustive"
        );
    }

    // ── Literal patterns ─────────────────────────────────────────────

    #[test]
    fn test_literal_with_wildcard_exhaustive() {
        // match x { 1 -> ..., 2 -> ..., _ -> ... } is exhaustive
        let result = check_exhaustiveness(
            &[lit_int(1), lit_int(2), wildcard()],
            &int_type(),
            &empty_registry(),
        );
        assert!(result.is_none(), "Int [1, 2, _] should be exhaustive");
    }

    #[test]
    fn test_literal_without_wildcard_non_exhaustive() {
        // match x { 1 -> ..., 2 -> ... } NOT exhaustive for Int (infinite)
        let result =
            check_exhaustiveness(&[lit_int(1), lit_int(2)], &int_type(), &empty_registry());
        assert!(result.is_some(), "Int [1, 2] should NOT be exhaustive");
    }

    #[test]
    fn test_literal_wildcard_only_exhaustive() {
        // match x { _ -> ... } is exhaustive for Int
        let result = check_exhaustiveness(&[wildcard()], &int_type(), &empty_registry());
        assert!(result.is_none(), "Int [_] should be exhaustive");
    }

    // ── is_useful with sum type constructors ─────────────────────────

    #[test]
    fn test_is_useful_constructor_against_different_constructor() {
        // Matrix has Circle(_), testing Point -- should be useful
        let m = matrix(vec![vec![ctor("Circle", "Shape", vec![wildcard()])]]);
        assert!(is_useful(
            &m,
            &[ctor("Point", "Shape", vec![])],
            &[shape_type()],
        ));
    }

    #[test]
    fn test_is_useful_constructor_against_same_constructor() {
        // Matrix has Circle(_), testing Circle(_) -- NOT useful
        let m = matrix(vec![vec![ctor("Circle", "Shape", vec![wildcard()])]]);
        assert!(!is_useful(
            &m,
            &[ctor("Circle", "Shape", vec![wildcard()])],
            &[shape_type()],
        ));
    }

    #[test]
    fn test_is_useful_wildcard_after_all_constructors() {
        // Matrix has [Circle(_), Point], testing _ -- NOT useful (type is complete)
        let m = matrix(vec![
            vec![ctor("Circle", "Shape", vec![wildcard()])],
            vec![ctor("Point", "Shape", vec![])],
        ]);
        assert!(!is_useful(&m, &[wildcard()], &[shape_type()]));
    }

    #[test]
    fn test_is_useful_wildcard_after_partial_constructors() {
        // Matrix has [Circle(_)], testing _ -- useful (Point not covered)
        let m = matrix(vec![vec![ctor("Circle", "Shape", vec![wildcard()])]]);
        assert!(is_useful(&m, &[wildcard()], &[shape_type()]));
    }

    // ── is_useful with literals ──────────────────────────────────────

    #[test]
    fn test_is_useful_new_literal_value() {
        // Matrix has [1], testing 2 -- useful
        let m = matrix(vec![vec![lit_int(1)]]);
        assert!(is_useful(&m, &[lit_int(2)], &[int_type()]));
    }

    #[test]
    fn test_is_useful_duplicate_literal_value() {
        // Matrix has [1], testing 1 -- NOT useful
        let m = matrix(vec![vec![lit_int(1)]]);
        assert!(!is_useful(&m, &[lit_int(1)], &[int_type()]));
    }

    // ── Multi-column patterns ────────────────────────────────────────

    #[test]
    fn test_is_useful_multi_column() {
        // Matrix: [[true, true], [false, false]]
        // Test: [true, false] -- should be useful
        let m = matrix(vec![
            vec![lit_bool(true), lit_bool(true)],
            vec![lit_bool(false), lit_bool(false)],
        ]);
        assert!(is_useful(
            &m,
            &[lit_bool(true), lit_bool(false)],
            &[bool_type(), bool_type()],
        ));
    }

    #[test]
    fn test_is_useful_multi_column_not_useful() {
        // Matrix: [[true, _], [false, _]]
        // Test: [true, true] -- NOT useful (first row covers it)
        let m = matrix(vec![
            vec![lit_bool(true), wildcard()],
            vec![lit_bool(false), wildcard()],
        ]);
        assert!(!is_useful(
            &m,
            &[lit_bool(true), lit_bool(true)],
            &[bool_type(), bool_type()],
        ));
    }

    // ── Bool redundancy edge cases ───────────────────────────────────

    #[test]
    fn test_bool_true_false_true_redundant() {
        // match b { true -> ..., false -> ..., true -> ... }
        // arm 2 is redundant
        let result = check_redundancy(
            &[lit_bool(true), lit_bool(false), lit_bool(true)],
            &bool_type(),
            &empty_registry(),
        );
        assert_eq!(result, vec![2]);
    }

    // ── TypeInfo for nested specialization ───────────────────────────

    #[test]
    fn test_nested_specialization_type_info() {
        // When we specialize Option by Some, the inner column needs
        // Shape type info. This tests that the algorithm correctly
        // handles nested type information via recursive specialization.
        //
        // Matrix: [Some(Circle(_)), None]
        // Test: Some(Point) -- should be useful
        let m = matrix(vec![
            vec![ctor(
                "Some",
                "Option",
                vec![ctor("Circle", "Shape", vec![wildcard()])],
            )],
            vec![ctor("None", "Option", vec![])],
        ]);
        // For this test, is_useful builds registry from patterns.
        // Patterns mention Circle and Point for Shape, so registry is complete.
        let result = is_useful(
            &m,
            &[ctor("Some", "Option", vec![ctor("Point", "Shape", vec![])])],
            &[option_shape_type()],
        );
        assert!(
            result,
            "Some(Point) should be useful when only Some(Circle(_)) and None are covered"
        );
    }
}
