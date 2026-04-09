//! Monomorphization pass.
//!
//! Takes a MIR module and ensures all functions use only concrete types.
//! Since the type checker already resolves concrete types at each call site,
//! this pass primarily:
//! 1. Collects all reachable functions starting from the entry point.
//! 2. Removes unreachable functions.
//! 3. In future: creates specialized copies of generic functions for each
//!    concrete type instantiation.
//!
//! For Phase 5, all types are already concrete after lowering (the type checker
//! resolves all generics), so monomorphization is mainly a reachability pass.

use std::collections::HashSet;

use super::{MirExpr, MirModule};

/// Run the monomorphization pass on a MIR module.
///
/// This collects all reachable functions starting from the entry point
/// (or all top-level functions if no entry point exists), plus any explicit
/// extra roots that the caller needs to preserve, and removes any unreachable
/// functions. In the future, this will also specialize generic functions for each
/// concrete type instantiation.
pub fn monomorphize(module: &mut MirModule) {
    monomorphize_with_roots(module, &[]);
}

/// Run the monomorphization pass while preserving additional root symbols.
pub fn monomorphize_with_roots(module: &mut MirModule, extra_roots: &[String]) {
    let reachable = collect_reachable_functions(module, extra_roots);

    // Keep only reachable functions (plus closure functions that may be
    // referenced transitively).
    module.functions.retain(|f| reachable.contains(&f.name));
}

/// Collect the names of all reachable functions starting from the entry point.
fn collect_reachable_functions(module: &MirModule, extra_roots: &[String]) -> HashSet<String> {
    let mut reachable = HashSet::new();
    let mut worklist: Vec<String> = Vec::new();

    // Start from the entry function, or all functions if no entry.
    if let Some(ref entry) = module.entry_function {
        worklist.push(entry.clone());
    } else {
        // No entry point: keep all functions reachable.
        for f in &module.functions {
            worklist.push(f.name.clone());
        }
    }
    for root in extra_roots {
        worklist.push(root.clone());
    }

    while let Some(name) = worklist.pop() {
        if reachable.contains(&name) {
            continue;
        }
        reachable.insert(name.clone());

        // If this is a service loop function, add all handler functions from
        // the dispatch table as reachable. The loop body is MirExpr::Unit
        // (codegen generates the dispatch inline), so handler functions are
        // not referenced from MIR expressions -- only from the dispatch table.
        if let Some((call_handlers, cast_handlers)) = module.service_dispatch.get(&name) {
            for (_, handler_fn, _) in call_handlers {
                if !reachable.contains(handler_fn) {
                    worklist.push(handler_fn.clone());
                }
            }
            for (_, handler_fn, _) in cast_handlers {
                if !reachable.contains(handler_fn) {
                    worklist.push(handler_fn.clone());
                }
            }
        }

        // If this is an actor wrapper function, add the body function as reachable.
        // Actor wrappers have body: MirExpr::Unit and a single __args_ptr param.
        // The body function is named __actor_{name}_body and is called by codegen,
        // not referenced in MIR expressions.
        if let Some(func) = module.functions.iter().find(|f| f.name == name) {
            if func.params.len() == 1 && func.params[0].0 == "__args_ptr" {
                let body_fn_name = format!("__actor_{}_body", name);
                if module.functions.iter().any(|f| f.name == body_fn_name) {
                    if !reachable.contains(&body_fn_name) {
                        worklist.push(body_fn_name);
                    }
                }
            }
        }

        // Find the function and scan its body for referenced functions.
        if let Some(func) = module.functions.iter().find(|f| f.name == name) {
            let mut refs = Vec::new();
            collect_function_refs(&func.body, &mut refs);
            for r in refs {
                if !reachable.contains(&r) {
                    worklist.push(r);
                }
            }
        }
    }

    reachable
}

/// Recursively collect function names referenced in an expression.
fn collect_function_refs(expr: &MirExpr, refs: &mut Vec<String>) {
    match expr {
        MirExpr::Call { func, args, .. } => {
            if let MirExpr::Var(name, _) = func.as_ref() {
                refs.push(name.clone());
            }
            collect_function_refs(func, refs);
            for arg in args {
                collect_function_refs(arg, refs);
            }
        }
        MirExpr::ClosureCall { closure, args, .. } => {
            collect_function_refs(closure, refs);
            for arg in args {
                collect_function_refs(arg, refs);
            }
        }
        MirExpr::MakeClosure {
            fn_name, captures, ..
        } => {
            refs.push(fn_name.clone());
            for cap in captures {
                collect_function_refs(cap, refs);
            }
        }
        MirExpr::BinOp { lhs, rhs, .. } => {
            collect_function_refs(lhs, refs);
            collect_function_refs(rhs, refs);
        }
        MirExpr::UnaryOp { operand, .. } => {
            collect_function_refs(operand, refs);
        }
        MirExpr::If {
            cond,
            then_body,
            else_body,
            ..
        } => {
            collect_function_refs(cond, refs);
            collect_function_refs(then_body, refs);
            collect_function_refs(else_body, refs);
        }
        MirExpr::Let { value, body, .. } => {
            collect_function_refs(value, refs);
            collect_function_refs(body, refs);
        }
        MirExpr::Block(exprs, _) => {
            for e in exprs {
                collect_function_refs(e, refs);
            }
        }
        MirExpr::Match {
            scrutinee, arms, ..
        } => {
            collect_function_refs(scrutinee, refs);
            for arm in arms {
                if let Some(guard) = &arm.guard {
                    collect_function_refs(guard, refs);
                }
                collect_function_refs(&arm.body, refs);
            }
        }
        MirExpr::StructLit { fields, .. } => {
            for (_, val) in fields {
                collect_function_refs(val, refs);
            }
        }
        MirExpr::StructUpdate {
            base, overrides, ..
        } => {
            collect_function_refs(base, refs);
            for (_, val) in overrides {
                collect_function_refs(val, refs);
            }
        }
        MirExpr::FieldAccess { object, .. } => {
            collect_function_refs(object, refs);
        }
        MirExpr::ConstructVariant { fields, .. } => {
            for f in fields {
                collect_function_refs(f, refs);
            }
        }
        MirExpr::Return(val) => {
            collect_function_refs(val, refs);
        }
        MirExpr::Var(name, _) => {
            // Variable references to known functions also count.
            refs.push(name.clone());
        }
        MirExpr::IntLit(_, _)
        | MirExpr::FloatLit(_, _)
        | MirExpr::BoolLit(_, _)
        | MirExpr::StringLit(_, _)
        | MirExpr::Panic { .. }
        | MirExpr::Unit => {}
        // Actor primitives
        MirExpr::ActorSpawn {
            func,
            args,
            terminate_callback,
            ..
        } => {
            collect_function_refs(func, refs);
            for arg in args {
                collect_function_refs(arg, refs);
            }
            if let Some(cb) = terminate_callback {
                collect_function_refs(cb, refs);
            }
        }
        MirExpr::ActorSend {
            target, message, ..
        } => {
            collect_function_refs(target, refs);
            collect_function_refs(message, refs);
        }
        MirExpr::ActorReceive {
            arms,
            timeout_ms,
            timeout_body,
            ..
        } => {
            for arm in arms {
                if let Some(guard) = &arm.guard {
                    collect_function_refs(guard, refs);
                }
                collect_function_refs(&arm.body, refs);
            }
            if let Some(tm) = timeout_ms {
                collect_function_refs(tm, refs);
            }
            if let Some(tb) = timeout_body {
                collect_function_refs(tb, refs);
            }
        }
        MirExpr::ActorSelf { .. } => {}
        MirExpr::ActorLink { target, .. } => {
            collect_function_refs(target, refs);
        }
        MirExpr::ListLit { elements, .. } => {
            for elem in elements {
                collect_function_refs(elem, refs);
            }
        }
        MirExpr::SupervisorStart { children, .. } => {
            // Each child spec references a start function by name.
            for child in children {
                if !child.start_fn.is_empty() {
                    refs.push(child.start_fn.clone());
                }
            }
        }
        // Loop primitives
        MirExpr::While { cond, body, .. } => {
            collect_function_refs(cond, refs);
            collect_function_refs(body, refs);
        }
        MirExpr::Break | MirExpr::Continue => {}
        MirExpr::ForInRange {
            start,
            end,
            filter,
            body,
            ..
        } => {
            collect_function_refs(start, refs);
            collect_function_refs(end, refs);
            if let Some(f) = filter {
                collect_function_refs(f, refs);
            }
            collect_function_refs(body, refs);
        }
        MirExpr::ForInList {
            collection,
            filter,
            body,
            ..
        } => {
            collect_function_refs(collection, refs);
            if let Some(f) = filter {
                collect_function_refs(f, refs);
            }
            collect_function_refs(body, refs);
        }
        MirExpr::ForInMap {
            collection,
            filter,
            body,
            ..
        } => {
            collect_function_refs(collection, refs);
            if let Some(f) = filter {
                collect_function_refs(f, refs);
            }
            collect_function_refs(body, refs);
        }
        MirExpr::ForInSet {
            collection,
            filter,
            body,
            ..
        } => {
            collect_function_refs(collection, refs);
            if let Some(f) = filter {
                collect_function_refs(f, refs);
            }
            collect_function_refs(body, refs);
        }
        MirExpr::ForInIterator {
            iterator,
            filter,
            body,
            next_fn,
            iter_fn,
            ..
        } => {
            collect_function_refs(iterator, refs);
            if let Some(f) = filter {
                collect_function_refs(f, refs);
            }
            collect_function_refs(body, refs);
            // Mark the next() function as reachable.
            refs.push(next_fn.clone());
            // Mark the iter() function as reachable (if Iterable path).
            if !iter_fn.is_empty() {
                refs.push(iter_fn.clone());
            }
        }
        // TCE: TailCall args may reference functions.
        MirExpr::TailCall { args, .. } => {
            for arg in args {
                collect_function_refs(arg, refs);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{MirFunction, MirType};

    #[test]
    fn monomorphize_keeps_reachable_functions() {
        let mut module = MirModule {
            functions: vec![
                MirFunction {
                    name: "main".to_string(),
                    params: vec![],
                    return_type: MirType::Int,
                    body: MirExpr::Call {
                        func: Box::new(MirExpr::Var(
                            "helper".to_string(),
                            MirType::FnPtr(vec![], Box::new(MirType::Int)),
                        )),
                        args: vec![],
                        ty: MirType::Int,
                    },
                    is_closure_fn: false,
                    captures: vec![],
                    has_tail_calls: false,
                },
                MirFunction {
                    name: "helper".to_string(),
                    params: vec![],
                    return_type: MirType::Int,
                    body: MirExpr::IntLit(42, MirType::Int),
                    is_closure_fn: false,
                    captures: vec![],
                    has_tail_calls: false,
                },
                MirFunction {
                    name: "unused".to_string(),
                    params: vec![],
                    return_type: MirType::Int,
                    body: MirExpr::IntLit(0, MirType::Int),
                    is_closure_fn: false,
                    captures: vec![],
                    has_tail_calls: false,
                },
            ],
            structs: vec![],
            sum_types: vec![],
            entry_function: Some("main".to_string()),
            service_dispatch: std::collections::HashMap::new(),
        };

        monomorphize(&mut module);

        let names: Vec<&str> = module.functions.iter().map(|f| f.name.as_str()).collect();
        assert!(names.contains(&"main"));
        assert!(names.contains(&"helper"));
        assert!(
            !names.contains(&"unused"),
            "unused function should be removed"
        );
    }

    #[test]
    fn monomorphize_keeps_all_without_entry() {
        let mut module = MirModule {
            functions: vec![
                MirFunction {
                    name: "foo".to_string(),
                    params: vec![],
                    return_type: MirType::Unit,
                    body: MirExpr::Unit,
                    is_closure_fn: false,
                    captures: vec![],
                    has_tail_calls: false,
                },
                MirFunction {
                    name: "bar".to_string(),
                    params: vec![],
                    return_type: MirType::Unit,
                    body: MirExpr::Unit,
                    is_closure_fn: false,
                    captures: vec![],
                    has_tail_calls: false,
                },
            ],
            structs: vec![],
            sum_types: vec![],
            entry_function: None,
            service_dispatch: std::collections::HashMap::new(),
        };

        monomorphize(&mut module);

        assert_eq!(module.functions.len(), 2);
    }
}
