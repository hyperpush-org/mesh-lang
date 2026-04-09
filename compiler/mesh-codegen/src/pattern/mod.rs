//! Pattern match compilation to decision trees.
//!
//! This module implements Maranget-style pattern match compilation, transforming
//! `MirExpr::Match` nodes into `DecisionTree` structures that map directly to
//! LLVM switch instructions and conditional branches.
//!
//! ## Decision Tree Nodes
//!
//! - `Leaf` -- execute an arm body with variable bindings
//! - `Switch` -- switch on sum type constructor tag
//! - `Test` -- test literal equality
//! - `Guard` -- evaluate a guard expression
//! - `Fail` -- runtime panic for non-exhaustive match

pub mod compile;

use crate::mir::{MirExpr, MirLiteral, MirType};

// ── AccessPath ──────────────────────────────────────────────────────

/// Describes how to reach a sub-value of the scrutinee.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccessPath {
    /// The scrutinee itself.
    Root,
    /// Field N of a tuple.
    TupleField(Box<AccessPath>, usize),
    /// Field N of a variant (variant name for disambiguation).
    VariantField(Box<AccessPath>, String, usize),
    /// Named field of a struct.
    StructField(Box<AccessPath>, String),
    /// Head element of a list (first element).
    ListHead(Box<AccessPath>),
    /// Tail of a list (remaining elements after head).
    ListTail(Box<AccessPath>),
}

// ── ConstructorTag ──────────────────────────────────────────────────

/// A tag identifying a sum type constructor variant.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstructorTag {
    /// The sum type name (e.g., "Shape").
    pub type_name: String,
    /// The variant name (e.g., "Circle").
    pub variant_name: String,
    /// The numeric tag value (0, 1, 2, ...).
    pub tag: u8,
    /// Number of fields this constructor carries.
    pub arity: usize,
}

// ── DecisionTree ────────────────────────────────────────────────────

/// A compiled decision tree for pattern matching.
///
/// Each node represents a runtime decision point that maps directly
/// to LLVM basic blocks and branch instructions.
#[derive(Debug, Clone)]
pub enum DecisionTree {
    /// Execute the arm body at `arm_index` with the given variable bindings.
    Leaf {
        arm_index: usize,
        bindings: Vec<(String, MirType, AccessPath)>,
    },
    /// Switch on a sum type constructor tag.
    Switch {
        scrutinee_path: AccessPath,
        cases: Vec<(ConstructorTag, DecisionTree)>,
        default: Option<Box<DecisionTree>>,
    },
    /// Test a literal value for equality.
    Test {
        scrutinee_path: AccessPath,
        value: MirLiteral,
        success: Box<DecisionTree>,
        failure: Box<DecisionTree>,
    },
    /// Evaluate a guard expression and branch.
    Guard {
        guard_expr: MirExpr,
        success: Box<DecisionTree>,
        failure: Box<DecisionTree>,
    },
    /// List deconstruction: test if list is non-empty, then bind head/tail.
    ListDecons {
        scrutinee_path: AccessPath,
        /// Element type for head value conversion (u64 -> actual type).
        elem_ty: MirType,
        /// Tree when list is non-empty (head/tail columns added).
        non_empty: Box<DecisionTree>,
        /// Tree when list is empty (fallthrough to next arm).
        empty: Box<DecisionTree>,
    },
    /// Runtime panic for non-exhaustive match (possible with guards).
    Fail {
        message: String,
        file: String,
        line: u32,
    },
}
