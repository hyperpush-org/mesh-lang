use std::collections::BTreeMap;

use mesh_typeck::TypeckResult;

use crate::mir::{MirExpr, MirFunction, MirModule, MirType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclaredHandlerKind {
    Work,
    ServiceCall,
    ServiceCast,
    Route,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclaredHandlerPlanEntry {
    pub kind: DeclaredHandlerKind,
    pub runtime_registration_name: String,
    pub executable_symbol: String,
    pub replication_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclaredRuntimeRegistration {
    pub runtime_registration_name: String,
    pub executable_symbol: String,
    pub replication_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartupWorkRegistration {
    pub runtime_registration_name: String,
}

pub fn prepare_clustered_route_handler_plan<'a>(
    typecks: impl IntoIterator<Item = &'a TypeckResult>,
) -> Result<Vec<DeclaredHandlerPlanEntry>, String> {
    let mut planned_routes = BTreeMap::<String, u64>::new();

    for typeck in typecks {
        for metadata in typeck.clustered_route_wrappers.values() {
            let runtime_registration_name = metadata.runtime_name.trim();
            if runtime_registration_name.is_empty() {
                return Err(
                    "clustered route wrapper metadata is missing a runtime registration name"
                        .to_string(),
                );
            }

            let replication_count = u64::from(metadata.replication_count.value);
            if replication_count == 0 {
                return Err(format!(
                    "clustered route handler `{runtime_registration_name}` lowered with invalid replication count 0"
                ));
            }

            if let Some(existing_count) = planned_routes.get(runtime_registration_name) {
                if *existing_count != replication_count {
                    return Err(format!(
                        "clustered route handler `{runtime_registration_name}` lowered with conflicting replication counts {existing_count} and {replication_count}"
                    ));
                }
                continue;
            }

            planned_routes.insert(runtime_registration_name.to_string(), replication_count);
        }
    }

    Ok(planned_routes
        .into_iter()
        .map(
            |(runtime_registration_name, replication_count)| DeclaredHandlerPlanEntry {
                kind: DeclaredHandlerKind::Route,
                executable_symbol: declared_route_wrapper_name(&runtime_registration_name),
                runtime_registration_name,
                replication_count,
            },
        )
        .collect())
}

pub fn prepare_startup_work_registrations(
    plan: &[DeclaredHandlerPlanEntry],
) -> Vec<StartupWorkRegistration> {
    plan.iter()
        .filter(|entry| entry.kind == DeclaredHandlerKind::Work)
        .map(|entry| StartupWorkRegistration {
            runtime_registration_name: entry.runtime_registration_name.clone(),
        })
        .collect()
}

pub fn prepare_declared_runtime_handlers(
    mir: &mut MirModule,
    plan: &[DeclaredHandlerPlanEntry],
) -> Result<Vec<DeclaredRuntimeRegistration>, String> {
    let mut registrations = Vec::with_capacity(plan.len());

    for entry in plan {
        let executable_symbol = match entry.kind {
            DeclaredHandlerKind::Work => generate_declared_work_wrapper(
                mir,
                &entry.runtime_registration_name,
                &entry.executable_symbol,
            )?,
            DeclaredHandlerKind::ServiceCall | DeclaredHandlerKind::ServiceCast => {
                generate_declared_service_wrapper(
                    mir,
                    entry.kind,
                    &entry.runtime_registration_name,
                    &entry.executable_symbol,
                )?
            }
            DeclaredHandlerKind::Route => validate_declared_route_wrapper(
                mir,
                &entry.runtime_registration_name,
                &entry.executable_symbol,
            )?,
        };

        registrations.push(DeclaredRuntimeRegistration {
            runtime_registration_name: entry.runtime_registration_name.clone(),
            executable_symbol,
            replication_count: entry.replication_count,
        });
    }

    Ok(registrations)
}

fn generate_declared_work_wrapper(
    mir: &mut MirModule,
    runtime_registration_name: &str,
    executable_symbol: &str,
) -> Result<String, String> {
    let original = mir
        .functions
        .iter()
        .find(|func| func.name == executable_symbol)
        .cloned()
        .ok_or_else(|| {
            format!(
                "declared work target `{runtime_registration_name}` has no lowered function `{executable_symbol}`"
            )
        })?;

    let wrapper_name = declared_work_wrapper_name(runtime_registration_name);
    if mir.functions.iter().any(|func| func.name == wrapper_name) {
        return Ok(wrapper_name);
    }

    let body_name = format!("__actor_{}_body", wrapper_name);
    let hidden_continuity_params = vec![
        ("request_key".to_string(), MirType::String),
        ("attempt_id".to_string(), MirType::String),
    ];

    let (body_params, body_args) = match original.params.as_slice() {
        [] => (hidden_continuity_params.clone(), Vec::new()),
        [(request_name, request_ty), (attempt_name, attempt_ty)]
            if *request_ty == MirType::String && *attempt_ty == MirType::String =>
        {
            (
                original.params.clone(),
                vec![
                    MirExpr::Var(request_name.clone(), request_ty.clone()),
                    MirExpr::Var(attempt_name.clone(), attempt_ty.clone()),
                ],
            )
        }
        _ => {
            return Err(format!(
                "declared work target `{runtime_registration_name}` must use either `pub fn name() -> ...` or the legacy `(request_key :: String, attempt_id :: String)` signature"
            ))
        }
    };

    let call = MirExpr::Call {
        func: Box::new(MirExpr::Var(
            original.name.clone(),
            MirType::FnPtr(
                original
                    .params
                    .iter()
                    .map(|(_, ty)| ty.clone())
                    .collect::<Vec<_>>(),
                Box::new(original.return_type.clone()),
            ),
        )),
        args: body_args,
        ty: original.return_type.clone(),
    };

    let body = if original.return_type == MirType::Unit {
        call
    } else {
        MirExpr::Let {
            name: "__declared_work_result".to_string(),
            ty: original.return_type.clone(),
            value: Box::new(call),
            body: Box::new(MirExpr::Unit),
        }
    };

    mir.functions.push(MirFunction {
        name: body_name,
        params: body_params,
        return_type: MirType::Unit,
        body,
        is_closure_fn: false,
        captures: Vec::new(),
        has_tail_calls: false,
    });

    mir.functions.push(MirFunction {
        name: wrapper_name.clone(),
        params: vec![("__args_ptr".to_string(), MirType::Ptr)],
        return_type: MirType::Unit,
        body: MirExpr::Unit,
        is_closure_fn: false,
        captures: Vec::new(),
        has_tail_calls: false,
    });

    Ok(wrapper_name)
}

fn validate_declared_route_wrapper(
    mir: &mut MirModule,
    runtime_registration_name: &str,
    executable_symbol: &str,
) -> Result<String, String> {
    let route = mir
        .functions
        .iter()
        .find(|func| func.name == executable_symbol)
        .ok_or_else(|| {
            format!(
                "declared route target `{runtime_registration_name}` has no lowered function `{executable_symbol}`"
            )
        })?;

    match route.params.as_slice() {
        [(_, MirType::Ptr)] if route.return_type == MirType::Ptr && !route.is_closure_fn => {
            Ok(executable_symbol.to_string())
        }
        _ => Err(format!(
            "declared route target `{runtime_registration_name}` must lower to a bare `fn(Request) -> Response` shim, found `fn({}) -> {}`",
            route
                .params
                .iter()
                .map(|(_, ty)| ty.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            route.return_type
        )),
    }
}

fn generate_declared_service_wrapper(
    mir: &mut MirModule,
    kind: DeclaredHandlerKind,
    runtime_registration_name: &str,
    executable_symbol: &str,
) -> Result<String, String> {
    let helper = mir
        .functions
        .iter()
        .find(|func| func.name == executable_symbol)
        .cloned()
        .ok_or_else(|| {
            format!(
                "declared service target `{runtime_registration_name}` has no lowered helper `{executable_symbol}`"
            )
        })?;

    if helper.params.is_empty() {
        return Err(format!(
            "declared service target `{runtime_registration_name}` must lower to a helper with a pid parameter"
        ));
    }

    let (_service_name, helper_kind) = parse_service_helper_name(executable_symbol).ok_or_else(|| {
        format!(
            "declared service target `{runtime_registration_name}` lowered to unexpected helper `{executable_symbol}`"
        )
    })?;

    match (kind, helper_kind.as_str()) {
        (DeclaredHandlerKind::ServiceCall, "call")
        | (DeclaredHandlerKind::ServiceCast, "cast") => {}
        (DeclaredHandlerKind::ServiceCall, other) => {
            return Err(format!(
                "declared service_call target `{runtime_registration_name}` lowered to `{other}` helper `{executable_symbol}`"
            ))
        }
        (DeclaredHandlerKind::ServiceCast, other) => {
            return Err(format!(
                "declared service_cast target `{runtime_registration_name}` lowered to `{other}` helper `{executable_symbol}`"
            ))
        }
        (DeclaredHandlerKind::Work | DeclaredHandlerKind::Route, _) => unreachable!(),
    }

    let wrapper_name = declared_service_wrapper_name(kind, runtime_registration_name);
    if mir.functions.iter().any(|func| func.name == wrapper_name) {
        return Ok(wrapper_name);
    }

    let wrapper_param_types = helper
        .params
        .iter()
        .map(|(_, ty)| ty.clone())
        .collect::<Vec<_>>();
    let wrapper_args = helper
        .params
        .iter()
        .map(|(name, ty)| MirExpr::Var(name.clone(), ty.clone()))
        .collect::<Vec<_>>();

    let body = MirExpr::Call {
        func: Box::new(MirExpr::Var(
            helper.name.clone(),
            MirType::FnPtr(wrapper_param_types, Box::new(helper.return_type.clone())),
        )),
        args: wrapper_args,
        ty: helper.return_type.clone(),
    };

    mir.functions.push(MirFunction {
        name: wrapper_name.clone(),
        params: helper.params.clone(),
        return_type: helper.return_type.clone(),
        body,
        is_closure_fn: false,
        captures: Vec::new(),
        has_tail_calls: false,
    });

    Ok(wrapper_name)
}

pub fn declared_route_wrapper_name(runtime_registration_name: &str) -> String {
    format!(
        "__declared_route_{}",
        sanitize_runtime_name(runtime_registration_name)
    )
}

fn declared_work_wrapper_name(runtime_registration_name: &str) -> String {
    format!(
        "__declared_work_{}",
        sanitize_runtime_name(runtime_registration_name)
    )
}

fn declared_service_wrapper_name(
    kind: DeclaredHandlerKind,
    runtime_registration_name: &str,
) -> String {
    let kind_prefix = match kind {
        DeclaredHandlerKind::ServiceCall => "call",
        DeclaredHandlerKind::ServiceCast => "cast",
        DeclaredHandlerKind::Work | DeclaredHandlerKind::Route => unreachable!(),
    };
    format!(
        "__declared_service_{}_{}",
        kind_prefix,
        sanitize_runtime_name(runtime_registration_name)
    )
}

fn sanitize_runtime_name(runtime_registration_name: &str) -> String {
    runtime_registration_name
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect()
}

fn parse_service_helper_name(symbol: &str) -> Option<(String, String)> {
    let rest = symbol.strip_prefix("__service_")?;
    let (service_name, suffix) = rest.rsplit_once('_')?;
    if suffix == "start" {
        return Some((service_name.to_string(), "start".to_string()));
    }
    let (service_name, kind_and_method) = rest
        .split_once("_call_")
        .map(|(svc, method)| (svc.to_string(), format!("call:{method}")))
        .or_else(|| {
            rest.split_once("_cast_")
                .map(|(svc, method)| (svc.to_string(), format!("cast:{method}")))
        })?;
    let helper_kind = if kind_and_method.starts_with("call:") {
        "call"
    } else {
        "cast"
    };
    Some((service_name, helper_kind.to_string()))
}

#[cfg(test)]
mod tests {
    use rustc_hash::{FxHashMap, FxHashSet};

    use super::*;
    use mesh_typeck::ty::{Scheme, Ty, TyCon};
    use mesh_typeck::{ImportContext, ModuleExports};

    fn empty_module() -> MirModule {
        MirModule {
            functions: Vec::new(),
            structs: Vec::new(),
            sum_types: Vec::new(),
            entry_function: None,
            service_dispatch: std::collections::HashMap::new(),
        }
    }

    fn route_handler_ty() -> Ty {
        Ty::fun(
            vec![Ty::Con(TyCon::new("Request"))],
            Ty::Con(TyCon::new("Response")),
        )
    }

    fn route_handler_scheme() -> Scheme {
        Scheme::mono(route_handler_ty())
    }

    fn route_module_exports(module_name: &str, exported_handlers: &[&str]) -> ModuleExports {
        let mut functions = FxHashMap::default();
        for handler in exported_handlers {
            functions.insert((*handler).to_string(), route_handler_scheme());
        }

        ModuleExports {
            module_name: module_name.to_string(),
            functions,
            struct_defs: FxHashMap::default(),
            sum_type_defs: FxHashMap::default(),
            service_defs: FxHashMap::default(),
            actor_defs: FxHashMap::default(),
            private_names: FxHashSet::default(),
            type_aliases: FxHashMap::default(),
        }
    }

    fn typecheck_routes_in_module(source: &str, module_name: &str) -> TypeckResult {
        let parse = mesh_parser::parse(source);
        let mut import_ctx = ImportContext::empty();
        import_ctx.current_module = Some(module_name.to_string());
        mesh_typeck::check_with_imports(&parse, &import_ctx)
    }

    fn typecheck_routes_with_imports(
        source: &str,
        current_module: &str,
        imported_module: &str,
        imported_handler: &str,
    ) -> TypeckResult {
        let parse = mesh_parser::parse(source);
        let mut import_ctx = ImportContext::empty();
        import_ctx.current_module = Some(current_module.to_string());
        import_ctx.module_exports.insert(
            imported_module
                .rsplit('.')
                .next()
                .unwrap_or(imported_module)
                .to_string(),
            route_module_exports(imported_module, &[imported_handler]),
        );
        mesh_typeck::check_with_imports(&parse, &import_ctx)
    }

    #[test]
    fn m047_s02_startup_work_registrations_filter_out_service_handlers() {
        let registrations = prepare_startup_work_registrations(&[
            DeclaredHandlerPlanEntry {
                kind: DeclaredHandlerKind::Work,
                runtime_registration_name: "Work.handle_submit".to_string(),
                executable_symbol: "handle_submit".to_string(),
                replication_count: 2,
            },
            DeclaredHandlerPlanEntry {
                kind: DeclaredHandlerKind::ServiceCall,
                runtime_registration_name: "Services.Jobs.submit".to_string(),
                executable_symbol: "__service_jobs_call_submit".to_string(),
                replication_count: 3,
            },
            DeclaredHandlerPlanEntry {
                kind: DeclaredHandlerKind::ServiceCast,
                runtime_registration_name: "Services.Jobs.reset".to_string(),
                executable_symbol: "__service_jobs_cast_reset".to_string(),
                replication_count: 4,
            },
        ]);

        assert_eq!(
            registrations,
            vec![StartupWorkRegistration {
                runtime_registration_name: "Work.handle_submit".to_string(),
            }]
        );
    }

    #[test]
    fn m047_s07_startup_work_registrations_filter_out_route_handlers() {
        let registrations = prepare_startup_work_registrations(&[
            DeclaredHandlerPlanEntry {
                kind: DeclaredHandlerKind::Work,
                runtime_registration_name: "Work.handle_submit".to_string(),
                executable_symbol: "handle_submit".to_string(),
                replication_count: 2,
            },
            DeclaredHandlerPlanEntry {
                kind: DeclaredHandlerKind::Route,
                runtime_registration_name: "Api.Todos.handle_list_todos".to_string(),
                executable_symbol: "__declared_route_api_todos_handle_list_todos".to_string(),
                replication_count: 2,
            },
        ]);

        assert_eq!(
            registrations,
            vec![StartupWorkRegistration {
                runtime_registration_name: "Work.handle_submit".to_string(),
            }]
        );
    }

    #[test]
    fn m047_s07_clustered_route_handler_plan_dedupes_identical_wrappers() {
        let typeck = typecheck_routes_in_module(
            r#"
pub fn handle(req :: Request) -> Response do
  HTTP.response(200, "ok")
end

fn build() do
  let router = HTTP.router()
  let router = HTTP.on_get(router, "/one", HTTP.clustered(handle))
  router |> HTTP.on_get("/two", HTTP.clustered(handle))
end
"#,
            "App.Router",
        );
        assert!(
            typeck.errors.is_empty(),
            "expected route wrapper source to type-check cleanly, got {:?}",
            typeck.errors
        );

        let plan = prepare_clustered_route_handler_plan([&typeck])
            .expect("identical clustered route wrappers should dedupe cleanly");

        assert_eq!(
            plan,
            vec![DeclaredHandlerPlanEntry {
                kind: DeclaredHandlerKind::Route,
                runtime_registration_name: "App.Router.handle".to_string(),
                executable_symbol: "__declared_route_app_router_handle".to_string(),
                replication_count: 2,
            }]
        );
    }

    #[test]
    fn m047_s07_clustered_route_handler_plan_rejects_conflicting_replication_counts_across_modules()
    {
        let defaulted = typecheck_routes_with_imports(
            r#"
from Api.Todos import handle_list_todos

fn build() do
  HTTP.router() |> HTTP.on_get("/todos", HTTP.clustered(handle_list_todos))
end
"#,
            "App.RouterOne",
            "Api.Todos",
            "handle_list_todos",
        );
        let explicit = typecheck_routes_with_imports(
            r#"
from Api.Todos import handle_list_todos

fn build() do
  HTTP.router() |> HTTP.on_get("/todos", HTTP.clustered(3, handle_list_todos))
end
"#,
            "App.RouterTwo",
            "Api.Todos",
            "handle_list_todos",
        );

        let error = prepare_clustered_route_handler_plan([&defaulted, &explicit])
            .expect_err("conflicting imported route counts must fail closed");

        assert!(
            error.contains("Api.Todos.handle_list_todos")
                && error.contains("conflicting replication counts 2 and 3"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn m047_s07_declared_runtime_handlers_preserve_route_runtime_name_and_counts() {
        let mut mir = empty_module();
        mir.functions.push(MirFunction {
            name: "__declared_route_api_todos_handle_list_todos".to_string(),
            params: vec![("__request".to_string(), MirType::Ptr)],
            return_type: MirType::Ptr,
            body: MirExpr::Var("__request".to_string(), MirType::Ptr),
            is_closure_fn: false,
            captures: Vec::new(),
            has_tail_calls: false,
        });

        let registrations = prepare_declared_runtime_handlers(
            &mut mir,
            &[DeclaredHandlerPlanEntry {
                kind: DeclaredHandlerKind::Route,
                runtime_registration_name: "Api.Todos.handle_list_todos".to_string(),
                executable_symbol: declared_route_wrapper_name("Api.Todos.handle_list_todos"),
                replication_count: 3,
            }],
        )
        .expect("route shims should register without startup wrappers");

        assert_eq!(
            registrations,
            vec![DeclaredRuntimeRegistration {
                runtime_registration_name: "Api.Todos.handle_list_todos".to_string(),
                executable_symbol: "__declared_route_api_todos_handle_list_todos".to_string(),
                replication_count: 3,
            }]
        );
    }

    #[test]
    fn m047_s07_declared_route_handlers_reject_missing_lowered_symbol_before_registration() {
        let mut mir = empty_module();

        let error = prepare_declared_runtime_handlers(
            &mut mir,
            &[DeclaredHandlerPlanEntry {
                kind: DeclaredHandlerKind::Route,
                runtime_registration_name: "Api.Todos.handle_list_todos".to_string(),
                executable_symbol: declared_route_wrapper_name("Api.Todos.handle_list_todos"),
                replication_count: 2,
            }],
        )
        .expect_err("missing lowered route shim must fail before registration");

        assert!(
            error.contains(
                "declared route target `Api.Todos.handle_list_todos` has no lowered function `__declared_route_api_todos_handle_list_todos`"
            ),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn m047_s02_declared_runtime_handlers_preserve_default_and_explicit_replication_counts() {
        let mut mir = empty_module();
        mir.functions.push(MirFunction {
            name: "handle_submit".to_string(),
            params: vec![
                ("request_key".to_string(), MirType::String),
                ("attempt_id".to_string(), MirType::String),
            ],
            return_type: MirType::Int,
            body: MirExpr::IntLit(0, MirType::Int),
            is_closure_fn: false,
            captures: Vec::new(),
            has_tail_calls: false,
        });
        mir.functions.push(MirFunction {
            name: "handle_retry".to_string(),
            params: vec![
                ("request_key".to_string(), MirType::String),
                ("attempt_id".to_string(), MirType::String),
            ],
            return_type: MirType::Int,
            body: MirExpr::IntLit(1, MirType::Int),
            is_closure_fn: false,
            captures: Vec::new(),
            has_tail_calls: false,
        });

        let registrations = prepare_declared_runtime_handlers(
            &mut mir,
            &[
                DeclaredHandlerPlanEntry {
                    kind: DeclaredHandlerKind::Work,
                    runtime_registration_name: "Work.handle_submit".to_string(),
                    executable_symbol: "handle_submit".to_string(),
                    replication_count: 2,
                },
                DeclaredHandlerPlanEntry {
                    kind: DeclaredHandlerKind::Work,
                    runtime_registration_name: "Work.handle_retry".to_string(),
                    executable_symbol: "handle_retry".to_string(),
                    replication_count: 3,
                },
            ],
        )
        .expect("declared work helpers should wrap cleanly");

        assert_eq!(
            registrations,
            vec![
                DeclaredRuntimeRegistration {
                    runtime_registration_name: "Work.handle_submit".to_string(),
                    executable_symbol: "__declared_work_work_handle_submit".to_string(),
                    replication_count: 2,
                },
                DeclaredRuntimeRegistration {
                    runtime_registration_name: "Work.handle_retry".to_string(),
                    executable_symbol: "__declared_work_work_handle_retry".to_string(),
                    replication_count: 3,
                },
            ]
        );
        assert!(mir.functions.iter().any(|func| {
            func.name == "__declared_work_work_handle_submit"
                && func.params == vec![("__args_ptr".to_string(), MirType::Ptr)]
                && func.return_type == MirType::Unit
        }));
        assert!(mir.functions.iter().any(|func| {
            func.name == "__declared_work_work_handle_retry"
                && func.params == vec![("__args_ptr".to_string(), MirType::Ptr)]
                && func.return_type == MirType::Unit
        }));
    }

    #[test]
    fn m047_s05_declared_runtime_handlers_wrap_zero_arg_work_with_hidden_metadata() {
        let mut mir = empty_module();
        mir.functions.push(MirFunction {
            name: "add".to_string(),
            params: vec![],
            return_type: MirType::Int,
            body: MirExpr::IntLit(2, MirType::Int),
            is_closure_fn: false,
            captures: Vec::new(),
            has_tail_calls: false,
        });

        let registrations = prepare_declared_runtime_handlers(
            &mut mir,
            &[DeclaredHandlerPlanEntry {
                kind: DeclaredHandlerKind::Work,
                runtime_registration_name: "Work.add".to_string(),
                executable_symbol: "add".to_string(),
                replication_count: 2,
            }],
        )
        .expect("zero-arg declared work should wrap cleanly");

        assert_eq!(
            registrations,
            vec![DeclaredRuntimeRegistration {
                runtime_registration_name: "Work.add".to_string(),
                executable_symbol: "__declared_work_work_add".to_string(),
                replication_count: 2,
            }]
        );
        assert!(mir.functions.iter().any(|func| {
            func.name == "__actor___declared_work_work_add_body"
                && func.params
                    == vec![
                        ("request_key".to_string(), MirType::String),
                        ("attempt_id".to_string(), MirType::String),
                    ]
                && func.return_type == MirType::Unit
        }));
    }

    #[test]
    fn m047_s02_declared_runtime_handlers_reject_missing_lowered_symbol_before_registration() {
        let mut mir = empty_module();

        let error = prepare_declared_runtime_handlers(
            &mut mir,
            &[DeclaredHandlerPlanEntry {
                kind: DeclaredHandlerKind::Work,
                runtime_registration_name: "Work.handle_submit".to_string(),
                executable_symbol: "missing_handle_submit".to_string(),
                replication_count: 2,
            }],
        )
        .expect_err("missing lowered work symbol must fail before registration");

        assert!(
            error.contains(
                "declared work target `Work.handle_submit` has no lowered function `missing_handle_submit`"
            ),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn m047_s02_declared_service_handlers_reject_helper_kind_mismatch() {
        let mut mir = empty_module();
        mir.functions.push(MirFunction {
            name: "__service_jobs_cast_reset".to_string(),
            params: vec![("__pid".to_string(), MirType::Int)],
            return_type: MirType::Unit,
            body: MirExpr::Unit,
            is_closure_fn: false,
            captures: Vec::new(),
            has_tail_calls: false,
        });

        let error = prepare_declared_runtime_handlers(
            &mut mir,
            &[DeclaredHandlerPlanEntry {
                kind: DeclaredHandlerKind::ServiceCall,
                runtime_registration_name: "Services.Jobs.submit".to_string(),
                executable_symbol: "__service_jobs_cast_reset".to_string(),
                replication_count: 2,
            }],
        )
        .expect_err("call declarations must reject cast helpers");

        assert!(
            error.contains(
                "declared service_call target `Services.Jobs.submit` lowered to `cast` helper"
            ),
            "unexpected error: {error}"
        );
    }
}
