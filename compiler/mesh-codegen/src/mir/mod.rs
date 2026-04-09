//! Mid-level IR (MIR) definitions and lowering.
//!
//! The MIR is a desugared, closure-converted, monomorphized representation
//! that sits between the typed Rowan CST and LLVM IR. All types are concrete
//! (no type variables), closures are lifted to top-level functions with explicit
//! capture lists, pipe operators are desugared to function calls, and string
//! interpolation is compiled to runtime concat chains.

pub mod lower;
pub mod mono;
pub mod types;

use std::fmt;

// ── MirModule ─────────────────────────────────────────────────────────

/// Top-level compilation unit containing all functions, structs, and sum types.
#[derive(Debug, Clone)]
pub struct MirModule {
    /// All functions (including lifted closure functions).
    pub functions: Vec<MirFunction>,
    /// Struct type definitions.
    pub structs: Vec<MirStructDef>,
    /// Sum type definitions.
    pub sum_types: Vec<MirSumTypeDef>,
    /// Name of main() function if present.
    pub entry_function: Option<String>,
    /// Service dispatch tables for codegen.
    /// Maps service loop function name to (call_handlers, cast_handlers).
    /// Each handler entry: (type_tag, handler_fn_name, num_args).
    pub service_dispatch:
        std::collections::HashMap<String, (Vec<(u64, String, usize)>, Vec<(u64, String, usize)>)>,
}

// ── MirFunction ───────────────────────────────────────────────────────

/// A function in MIR -- either a user-defined function, a lifted closure,
/// or a monomorphized generic instantiation.
#[derive(Debug, Clone)]
pub struct MirFunction {
    /// Mangled name (e.g., "identity_Int" for monomorphized generics).
    pub name: String,
    /// Parameter names and their concrete types.
    pub params: Vec<(String, MirType)>,
    /// Concrete return type.
    pub return_type: MirType,
    /// Function body expression.
    pub body: MirExpr,
    /// If true, this is a lifted closure function; first param is env_ptr.
    pub is_closure_fn: bool,
    /// Captured variables (for closure functions only).
    pub captures: Vec<(String, MirType)>,
    /// Whether the function body contains TailCall nodes (set by TCE rewrite pass).
    /// When true, codegen wraps the function body in a loop.
    pub has_tail_calls: bool,
}

// ── MirType ───────────────────────────────────────────────────────────

/// A concrete MIR type. No type variables remain -- all types are fully resolved.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MirType {
    /// 64-bit signed integer.
    Int,
    /// 64-bit floating point.
    Float,
    /// Boolean (i8, 0 or 1).
    Bool,
    /// GC-managed string pointer.
    String,
    /// Unit type (void / empty tuple).
    Unit,
    /// Tuple of concrete types.
    Tuple(Vec<MirType>),
    /// Named struct reference.
    Struct(std::string::String),
    /// Named sum type reference.
    SumType(std::string::String),
    /// Known function pointer: params -> return.
    FnPtr(Vec<MirType>, Box<MirType>),
    /// Closure: {fn_ptr, env_ptr} with params -> return.
    Closure(Vec<MirType>, Box<MirType>),
    /// Raw pointer (for env_ptr, etc.).
    Ptr,
    /// Bottom type (never returns).
    Never,
    /// Actor PID, optionally typed with message type.
    /// Pid(None) = untyped Pid, Pid(Some(T)) = Pid<T>.
    Pid(Option<Box<MirType>>),
}

impl fmt::Display for MirType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MirType::Int => write!(f, "Int"),
            MirType::Float => write!(f, "Float"),
            MirType::Bool => write!(f, "Bool"),
            MirType::String => write!(f, "String"),
            MirType::Unit => write!(f, "Unit"),
            MirType::Tuple(elems) => {
                write!(f, "(")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, ")")
            }
            MirType::Struct(name) => write!(f, "{}", name),
            MirType::SumType(name) => write!(f, "{}", name),
            MirType::FnPtr(params, ret) => {
                write!(f, "fn(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ") -> {}", ret)
            }
            MirType::Closure(params, ret) => {
                write!(f, "closure(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ") -> {}", ret)
            }
            MirType::Ptr => write!(f, "Ptr"),
            MirType::Never => write!(f, "Never"),
            MirType::Pid(None) => write!(f, "Pid"),
            MirType::Pid(Some(msg_ty)) => write!(f, "Pid<{}>", msg_ty),
        }
    }
}

// ── MirExpr ───────────────────────────────────────────────────────────

/// A MIR expression node. Each variant carries its resolved type.
#[derive(Debug, Clone)]
pub enum MirExpr {
    /// Integer literal.
    IntLit(i64, MirType),
    /// Float literal.
    FloatLit(f64, MirType),
    /// Boolean literal.
    BoolLit(bool, MirType),
    /// String literal value.
    StringLit(std::string::String, MirType),
    /// Variable reference.
    Var(std::string::String, MirType),
    /// Binary operation.
    BinOp {
        op: BinOp,
        lhs: Box<MirExpr>,
        rhs: Box<MirExpr>,
        ty: MirType,
    },
    /// Unary operation.
    UnaryOp {
        op: UnaryOp,
        operand: Box<MirExpr>,
        ty: MirType,
    },
    /// Direct function call.
    Call {
        func: Box<MirExpr>,
        args: Vec<MirExpr>,
        ty: MirType,
    },
    /// Call through a closure (closure = {fn_ptr, env_ptr}).
    ClosureCall {
        closure: Box<MirExpr>,
        args: Vec<MirExpr>,
        ty: MirType,
    },
    /// If-then-else expression.
    If {
        cond: Box<MirExpr>,
        then_body: Box<MirExpr>,
        else_body: Box<MirExpr>,
        ty: MirType,
    },
    /// Let binding with continuation body.
    Let {
        name: std::string::String,
        ty: MirType,
        value: Box<MirExpr>,
        body: Box<MirExpr>,
    },
    /// Sequence of expressions; last is the value.
    Block(Vec<MirExpr>, MirType),
    /// Pattern match (NOT yet compiled to decision tree -- that is Plan 03).
    Match {
        scrutinee: Box<MirExpr>,
        arms: Vec<MirMatchArm>,
        ty: MirType,
    },
    /// Struct literal construction.
    StructLit {
        name: std::string::String,
        fields: Vec<(std::string::String, MirExpr)>,
        ty: MirType,
    },
    /// Struct update: copy base struct and overwrite specific fields.
    StructUpdate {
        base: Box<MirExpr>,
        overrides: Vec<(std::string::String, MirExpr)>,
        ty: MirType,
    },
    /// Field access on a struct.
    FieldAccess {
        object: Box<MirExpr>,
        field: std::string::String,
        ty: MirType,
    },
    /// Construct a variant of a sum type.
    ConstructVariant {
        type_name: std::string::String,
        variant: std::string::String,
        fields: Vec<MirExpr>,
        ty: MirType,
    },
    /// Create a closure object: {fn_ptr -> fn_name, env_ptr -> captures}.
    MakeClosure {
        fn_name: std::string::String,
        captures: Vec<MirExpr>,
        ty: MirType,
    },
    /// Return from function.
    Return(Box<MirExpr>),
    /// Runtime panic with message and source location.
    Panic {
        message: std::string::String,
        file: std::string::String,
        line: u32,
    },
    /// Unit value (empty tuple).
    Unit,

    // ── Actor primitives ──────────────────────────────────────────────
    /// Spawn a new actor process.
    ActorSpawn {
        /// The function to run as the actor body.
        func: Box<MirExpr>,
        /// Arguments to pass (initial state).
        args: Vec<MirExpr>,
        /// Priority level (0=normal, 1=high, 2=low).
        priority: u8,
        /// Optional terminate callback function.
        /// When present, this is a function that takes (state, reason) and runs cleanup.
        terminate_callback: Option<Box<MirExpr>>,
        /// Result type (Pid).
        ty: MirType,
    },
    /// Send a message to an actor.
    ActorSend {
        /// Target actor PID.
        target: Box<MirExpr>,
        /// Message to send.
        message: Box<MirExpr>,
        /// Result type (Unit -- fire-and-forget).
        ty: MirType,
    },
    /// Receive a message (blocking). Contains compiled match arms.
    ActorReceive {
        /// Match arms for incoming messages.
        arms: Vec<MirMatchArm>,
        /// Timeout in milliseconds (None = infinite wait).
        timeout_ms: Option<Box<MirExpr>>,
        /// Timeout body (executed if timeout fires).
        timeout_body: Option<Box<MirExpr>>,
        /// Result type.
        ty: MirType,
    },
    /// Get own PID.
    ActorSelf { ty: MirType },
    /// Link to another actor for supervision.
    ActorLink { target: Box<MirExpr>, ty: MirType },

    // ── Supervisor primitives ──────────────────────────────────────
    /// List literal: [e1, e2, ...]
    ListLit { elements: Vec<MirExpr>, ty: MirType },

    // ── Loop primitives ──────────────────────────────────────────────
    /// While loop: evaluates condition, if true executes body and repeats. Returns Unit.
    While {
        cond: Box<MirExpr>,
        body: Box<MirExpr>,
        ty: MirType, // Always MirType::Unit
    },
    /// Break: exit the innermost enclosing loop.
    Break,
    /// Continue: skip to the next iteration of the innermost enclosing loop.
    Continue,

    // ── TCE primitives ──────────────────────────────────────────────
    /// Self-recursive tail call (rewritten from Call during TCE pass).
    /// Codegen compiles this as parameter reassignment + branch to loop header.
    TailCall {
        /// Arguments for the recursive call (assigned to function parameters).
        args: Vec<MirExpr>,
        /// Result type (function return type, but ty() returns Never since it branches).
        ty: MirType,
    },

    /// For-in loop over an integer range: `for var in start..end do body end`.
    /// Desugared to integer counter iteration with no heap allocation.
    ForInRange {
        /// Loop variable name.
        var: String,
        /// Start value (inclusive).
        start: Box<MirExpr>,
        /// End value (exclusive).
        end: Box<MirExpr>,
        /// Optional filter expression (`when condition`).
        filter: Option<Box<MirExpr>>,
        /// Loop body.
        body: Box<MirExpr>,
        /// Result type (Ptr for comprehension semantics).
        ty: MirType,
    },

    /// For-in loop over a List: `for var in list do body end`.
    /// Desugared to indexed iteration with list builder.
    ForInList {
        var: String,
        collection: Box<MirExpr>,
        /// Optional filter expression (`when condition`).
        filter: Option<Box<MirExpr>>,
        body: Box<MirExpr>,
        elem_ty: MirType,
        body_ty: MirType,
        ty: MirType,
    },

    /// For-in loop over a Map: `for {k, v} in map do body end`.
    /// Desugared to indexed iteration over map entries.
    ForInMap {
        key_var: String,
        val_var: String,
        collection: Box<MirExpr>,
        /// Optional filter expression (`when condition`).
        filter: Option<Box<MirExpr>>,
        body: Box<MirExpr>,
        key_ty: MirType,
        val_ty: MirType,
        body_ty: MirType,
        ty: MirType,
    },

    /// For-in loop over a Set: `for var in set do body end`.
    /// Desugared to indexed iteration over set elements.
    ForInSet {
        var: String,
        collection: Box<MirExpr>,
        /// Optional filter expression (`when condition`).
        filter: Option<Box<MirExpr>>,
        body: Box<MirExpr>,
        elem_ty: MirType,
        body_ty: MirType,
        ty: MirType,
    },

    /// For-in loop over any type implementing Iterable/Iterator.
    /// Desugared to repeated next() calls with Option tag checking.
    ForInIterator {
        /// Loop variable name.
        var: String,
        /// The iterator expression (result of calling iter() on the collection,
        /// or the collection itself if it directly implements Iterator).
        iterator: Box<MirExpr>,
        /// Optional filter expression (`when condition`).
        filter: Option<Box<MirExpr>>,
        /// Loop body.
        body: Box<MirExpr>,
        /// Resolved element type (Item type for the concrete iterator type).
        elem_ty: MirType,
        /// Type of body expression (for list builder element conversion).
        body_ty: MirType,
        /// Mangled name for the next() function: "Iterator__next__TypeName".
        next_fn: String,
        /// Mangled name for the iter() function (if Iterable): "Iterable__iter__TypeName".
        /// Empty string if the type directly implements Iterator (no iter() call needed).
        iter_fn: String,
        /// Result type (Ptr for comprehension semantics -- list of body results).
        ty: MirType,
    },

    /// Start a supervisor with configured strategy, limits, and child specs.
    SupervisorStart {
        /// Supervisor name (for registration and debugging).
        name: String,
        /// Strategy enum value: 0=one_for_one, 1=one_for_all, 2=rest_for_one, 3=simple_one_for_one.
        strategy: u8,
        /// Max restarts within the time window.
        max_restarts: u32,
        /// Time window in seconds.
        max_seconds: u64,
        /// Child specs as MIR-level representations.
        children: Vec<MirChildSpec>,
        /// Result type (always Pid).
        ty: MirType,
    },
}

impl MirExpr {
    /// Get the type of this expression.
    pub fn ty(&self) -> &MirType {
        match self {
            MirExpr::IntLit(_, ty) => ty,
            MirExpr::FloatLit(_, ty) => ty,
            MirExpr::BoolLit(_, ty) => ty,
            MirExpr::StringLit(_, ty) => ty,
            MirExpr::Var(_, ty) => ty,
            MirExpr::BinOp { ty, .. } => ty,
            MirExpr::UnaryOp { ty, .. } => ty,
            MirExpr::Call { ty, .. } => ty,
            MirExpr::ClosureCall { ty, .. } => ty,
            MirExpr::If { ty, .. } => ty,
            MirExpr::Let { ty, .. } => ty,
            MirExpr::Block(_, ty) => ty,
            MirExpr::Match { ty, .. } => ty,
            MirExpr::StructLit { ty, .. } => ty,
            MirExpr::StructUpdate { ty, .. } => ty,
            MirExpr::FieldAccess { ty, .. } => ty,
            MirExpr::ConstructVariant { ty, .. } => ty,
            MirExpr::MakeClosure { ty, .. } => ty,
            MirExpr::Return(_) => &MirType::Never,
            MirExpr::Panic { .. } => &MirType::Never,
            MirExpr::Unit => &MirType::Unit,
            MirExpr::ActorSpawn { ty, .. } => ty,
            MirExpr::ActorSend { ty, .. } => ty,
            MirExpr::ActorReceive { ty, .. } => ty,
            MirExpr::ActorSelf { ty } => ty,
            MirExpr::ActorLink { ty, .. } => ty,
            MirExpr::ListLit { ty, .. } => ty,
            MirExpr::While { ty, .. } => ty,
            MirExpr::Break => &MirType::Never,
            MirExpr::Continue => &MirType::Never,
            MirExpr::TailCall { .. } => &MirType::Never,
            MirExpr::ForInRange { ty, .. } => ty,
            MirExpr::ForInList { ty, .. } => ty,
            MirExpr::ForInMap { ty, .. } => ty,
            MirExpr::ForInSet { ty, .. } => ty,
            MirExpr::ForInIterator { ty, .. } => ty,
            MirExpr::SupervisorStart { ty, .. } => ty,
        }
    }
}

/// A child specification in a supervisor's MIR representation.
#[derive(Debug, Clone)]
pub struct MirChildSpec {
    /// Child identifier string.
    pub id: String,
    /// Name of the start function (actor entry function).
    pub start_fn: String,
    /// Restart type: 0=permanent, 1=transient, 2=temporary.
    pub restart_type: u8,
    /// Shutdown timeout in milliseconds (0 = brutal_kill).
    pub shutdown_ms: u64,
    /// Child type: 0=worker, 1=supervisor.
    pub child_type: u8,
}

// ── MirMatchArm ───────────────────────────────────────────────────────

/// A match arm in MIR with pattern, optional guard, and body.
#[derive(Debug, Clone)]
pub struct MirMatchArm {
    /// The pattern to match.
    pub pattern: MirPattern,
    /// Optional guard expression.
    pub guard: Option<MirExpr>,
    /// The arm body.
    pub body: MirExpr,
}

// ── MirPattern ────────────────────────────────────────────────────────

/// A MIR pattern for match expressions.
#[derive(Debug, Clone)]
pub enum MirPattern {
    /// Matches anything, binds nothing.
    Wildcard,
    /// Binds a name to the matched value.
    Var(std::string::String, MirType),
    /// Matches a literal value.
    Literal(MirLiteral),
    /// Matches a sum type constructor with sub-patterns.
    Constructor {
        type_name: std::string::String,
        variant: std::string::String,
        fields: Vec<MirPattern>,
        bindings: Vec<(std::string::String, MirType)>,
    },
    /// Matches a tuple with sub-patterns.
    Tuple(Vec<MirPattern>),
    /// Or-pattern: matches if any alternative matches.
    Or(Vec<MirPattern>),
    /// List cons pattern: matches non-empty list, binding head element and tail list.
    ListCons {
        head: Box<MirPattern>,
        tail: Box<MirPattern>,
        elem_ty: MirType,
    },
}

// ── MirLiteral ────────────────────────────────────────────────────────

/// A literal value used in patterns.
#[derive(Debug, Clone)]
pub enum MirLiteral {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(std::string::String),
}

// ── BinOp / UnaryOp ──────────────────────────────────────────────────

/// Binary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    // Comparison
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    // Boolean
    And,
    Or,
    // String
    Concat,
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinOp::Add => write!(f, "+"),
            BinOp::Sub => write!(f, "-"),
            BinOp::Mul => write!(f, "*"),
            BinOp::Div => write!(f, "/"),
            BinOp::Mod => write!(f, "%"),
            BinOp::Eq => write!(f, "=="),
            BinOp::NotEq => write!(f, "!="),
            BinOp::Lt => write!(f, "<"),
            BinOp::Gt => write!(f, ">"),
            BinOp::LtEq => write!(f, "<="),
            BinOp::GtEq => write!(f, ">="),
            BinOp::And => write!(f, "and"),
            BinOp::Or => write!(f, "or"),
            BinOp::Concat => write!(f, "++"),
        }
    }
}

/// Unary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    /// Arithmetic negation.
    Neg,
    /// Boolean negation.
    Not,
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOp::Neg => write!(f, "-"),
            UnaryOp::Not => write!(f, "not"),
        }
    }
}

// ── MirStructDef ──────────────────────────────────────────────────────

/// A struct type definition in MIR.
#[derive(Debug, Clone)]
pub struct MirStructDef {
    /// Struct name.
    pub name: std::string::String,
    /// Field names and their concrete types.
    pub fields: Vec<(std::string::String, MirType)>,
}

// ── MirSumTypeDef ─────────────────────────────────────────────────────

/// A sum type definition in MIR.
#[derive(Debug, Clone)]
pub struct MirSumTypeDef {
    /// Sum type name.
    pub name: std::string::String,
    /// Variant definitions with sequential tag assignments.
    pub variants: Vec<MirVariantDef>,
}

/// A single variant in a MIR sum type definition.
#[derive(Debug, Clone)]
pub struct MirVariantDef {
    /// Variant name.
    pub name: std::string::String,
    /// Field types (positional).
    pub fields: Vec<MirType>,
    /// Tag value (0, 1, 2, ...).
    pub tag: u8,
}
