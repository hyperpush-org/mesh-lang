use mesh_typeck::diagnostics::DiagnosticOptions;
use mesh_typeck::error::TypeError;
use mesh_typeck::ty::{Scheme, Ty, TyCon};
use mesh_typeck::{
    ClusteredRouteReplicationCountSource, ImportContext, ModuleExports, TypeckResult,
};
use rustc_hash::{FxHashMap, FxHashSet};

fn handler_ty() -> Ty {
    Ty::fun(
        vec![Ty::Con(TyCon::new("Request"))],
        Ty::Con(TyCon::new("Response")),
    )
}

fn handler_scheme() -> Scheme {
    Scheme::mono(handler_ty())
}

fn check_source(src: &str, import_ctx: ImportContext) -> TypeckResult {
    let parse = mesh_parser::parse(src);
    mesh_typeck::check_with_imports(&parse, &import_ctx)
}

fn check_source_in_module(src: &str, module_name: &str) -> TypeckResult {
    let mut import_ctx = ImportContext::empty();
    import_ctx.current_module = Some(module_name.to_string());
    check_source(src, import_ctx)
}

fn module_exports(
    module_name: &str,
    exported_handlers: &[&str],
    private_handlers: &[&str],
) -> ModuleExports {
    let mut functions = FxHashMap::default();
    for handler in exported_handlers {
        functions.insert((*handler).to_string(), handler_scheme());
    }

    let mut private_names = FxHashSet::default();
    for handler in private_handlers {
        private_names.insert((*handler).to_string());
    }

    ModuleExports {
        module_name: module_name.to_string(),
        functions,
        struct_defs: FxHashMap::default(),
        sum_type_defs: FxHashMap::default(),
        service_defs: FxHashMap::default(),
        actor_defs: FxHashMap::default(),
        private_names,
        type_aliases: FxHashMap::default(),
    }
}

fn metadata_by_runtime_name<'a>(
    result: &'a TypeckResult,
    runtime_name: &str,
) -> &'a mesh_typeck::ClusteredRouteWrapperMetadata {
    result
        .clustered_route_wrappers
        .values()
        .find(|metadata| metadata.runtime_name == runtime_name)
        .unwrap_or_else(|| panic!("missing clustered route metadata for {runtime_name:?}"))
}

fn assert_no_generic_wrapper_noise(result: &TypeckResult) {
    assert!(
        !result.errors.iter().any(|error| matches!(
            error,
            TypeError::UnboundVariable { .. } | TypeError::NoSuchField { .. }
        )),
        "expected wrapper-specific diagnostics, got: {:?}",
        result.errors
    );
}

#[test]
fn m047_s07_clustered_route_wrapper_accepts_direct_and_pipe_forms_and_tracks_counts() {
    let src = r#"
import Api.Todos

pub fn handle_local(req :: Request) -> Response do
  HTTP.response(200, "ok")
end

fn build() do
  let router = HTTP.router()
  let router = HTTP.on_get(router, "/local", HTTP.clustered(handle_local))
  router |> HTTP.on_get("/todos", HTTP.clustered(3, Todos.handle_list_todos))
end
"#;

    let mut import_ctx = ImportContext::empty();
    import_ctx.current_module = Some("App.Router".to_string());
    import_ctx.module_exports.insert(
        "Todos".to_string(),
        module_exports("Api.Todos", &["handle_list_todos"], &[]),
    );

    let result = check_source(src, import_ctx);
    assert!(
        result.errors.is_empty(),
        "expected no errors, got: {:?}",
        result.errors
    );
    assert_eq!(result.clustered_route_wrappers.len(), 2);

    let local = metadata_by_runtime_name(&result, "App.Router.handle_local");
    assert_eq!(local.handler_name, "handle_local");
    assert_eq!(local.defining_module.as_deref(), Some("App.Router"));
    assert_eq!(local.replication_count.value, 2);
    assert_eq!(
        local.replication_count.source,
        ClusteredRouteReplicationCountSource::Default
    );

    let imported = metadata_by_runtime_name(&result, "Api.Todos.handle_list_todos");
    assert_eq!(imported.handler_name, "handle_list_todos");
    assert_eq!(imported.defining_module.as_deref(), Some("Api.Todos"));
    assert_eq!(imported.replication_count.value, 3);
    assert_eq!(
        imported.replication_count.source,
        ClusteredRouteReplicationCountSource::Explicit
    );
}

#[test]
fn m047_s07_clustered_route_wrapper_preserves_imported_bare_handler_origin() {
    let src = r#"
from Api.Todos import handle_list_todos

fn build() do
  HTTP.router() |> HTTP.on_get("/todos", HTTP.clustered(handle_list_todos))
end
"#;

    let mut import_ctx = ImportContext::empty();
    import_ctx.current_module = Some("App.Router".to_string());
    import_ctx.module_exports.insert(
        "Todos".to_string(),
        module_exports("Api.Todos", &["handle_list_todos"], &[]),
    );

    let result = check_source(src, import_ctx);
    assert!(
        result.errors.is_empty(),
        "expected no errors, got: {:?}",
        result.errors
    );
    assert_eq!(result.clustered_route_wrappers.len(), 1);

    let metadata = metadata_by_runtime_name(&result, "Api.Todos.handle_list_todos");
    assert_eq!(metadata.defining_module.as_deref(), Some("Api.Todos"));
    assert_eq!(metadata.replication_count.value, 2);
    assert_eq!(
        metadata.replication_count.source,
        ClusteredRouteReplicationCountSource::Default
    );
}

#[test]
fn m047_s07_clustered_route_wrapper_rejects_non_route_position() {
    let src = r#"
pub fn handle(req :: Request) -> Response do
  HTTP.response(200, "ok")
end

fn build() do
  let wrapped = HTTP.clustered(handle)
  wrapped
end
"#;

    let result = check_source_in_module(src, "App.Router");
    assert_no_generic_wrapper_noise(&result);
    assert!(
        result.errors.iter().any(|error| {
            matches!(
                error,
                TypeError::HttpClusteredOutsideRouteHandlerPosition { .. }
            )
        }),
        "expected non-route-position error, got: {:?}",
        result.errors
    );

    let rendered = result.render_errors(src, "test.mpl", &DiagnosticOptions::colorless());
    assert!(
        rendered
            .iter()
            .any(|diag| diag.contains("E0049") && diag.contains("route handler position")),
        "expected focused non-route-position diagnostic, got: {:?}",
        rendered
    );
}

#[test]
fn m047_s07_clustered_route_wrapper_rejects_closure_handler() {
    let src = r#"
fn build() do
  HTTP.router() |> HTTP.on_get("/x", HTTP.clustered(fn (req) -> req end))
end
"#;

    let result = check_source_in_module(src, "App.Router");
    assert_no_generic_wrapper_noise(&result);
    assert!(
        result.errors.iter().any(|error| {
            matches!(
                error,
                TypeError::HttpClusteredInvalidArguments { reason, .. }
                    if reason.contains("bare handler reference")
            )
        }),
        "expected invalid handler reference error, got: {:?}",
        result.errors
    );
}

#[test]
fn m047_s07_clustered_route_wrapper_rejects_private_handler() {
    let src = r#"
fn hidden(req :: Request) -> Response do
  HTTP.response(200, "ok")
end

fn build() do
  HTTP.router() |> HTTP.on_get("/x", HTTP.clustered(hidden))
end
"#;

    let result = check_source_in_module(src, "App.Router");
    assert_no_generic_wrapper_noise(&result);
    assert!(
        result.errors.iter().any(|error| {
            matches!(
                error,
                TypeError::HttpClusteredPrivateHandler { handler_name, .. }
                    if handler_name == "hidden"
            )
        }),
        "expected private-handler error, got: {:?}",
        result.errors
    );
}

#[test]
fn m047_s07_clustered_route_wrapper_rejects_conflicting_replication_counts() {
    let src = r#"
pub fn handle(req :: Request) -> Response do
  HTTP.response(200, "ok")
end

fn build() do
  let router = HTTP.router()
  let router = HTTP.on_get(router, "/one", HTTP.clustered(handle))
  HTTP.on_get(router, "/two", HTTP.clustered(3, handle))
end
"#;

    let result = check_source_in_module(src, "App.Router");
    assert_no_generic_wrapper_noise(&result);
    assert!(
        result.errors.iter().any(|error| {
            matches!(
                error,
                TypeError::HttpClusteredConflictingReplicationCount {
                    first_count,
                    current_count,
                    ..
                } if *first_count == 2 && *current_count == 3
            )
        }),
        "expected conflicting-count error, got: {:?}",
        result.errors
    );
}

#[test]
fn m047_s07_clustered_route_wrapper_rejects_imported_origin_drift() {
    let src = r#"
from Api.Todos import handle_list_todos

fn build() do
  HTTP.router() |> HTTP.on_get("/todos", HTTP.clustered(handle_list_todos))
end
"#;

    let mut import_ctx = ImportContext::empty();
    import_ctx.current_module = Some("App.Router".to_string());
    import_ctx.module_exports.insert(
        "Todos".to_string(),
        module_exports("", &["handle_list_todos"], &[]),
    );

    let result = check_source(src, import_ctx);
    assert_no_generic_wrapper_noise(&result);
    assert!(
        result.errors.iter().any(|error| {
            matches!(
                error,
                TypeError::HttpClusteredImportedOriginMissing { handler_name, .. }
                    if handler_name == "handle_list_todos"
            )
        }),
        "expected imported-origin diagnostic, got: {:?}",
        result.errors
    );
}
