//! HTTP router for the Mesh runtime.
//!
//! Routes are checked with priority ordering: exact routes first, then
//! parameterized routes (`:param` segments), then wildcards. Within each
//! tier, first match wins (registration order).
//!
//! Supports:
//! - Exact match: `/api/health` matches `/api/health`
//! - Wildcard patterns: `/api/*` matches any path starting with `/api/`
//! - Path parameters: `/users/:id` matches `/users/42` and captures `id=42`
//! - Method-specific routing: `HTTP.on_get(r, "/path", handler)` matches only GET

use crate::string::MeshString;

/// A single middleware entry holding the middleware function pointer.
#[derive(Clone)]
pub struct MiddlewareEntry {
    /// Pointer to the middleware function.
    pub fn_ptr: *mut u8,
    /// Pointer to the middleware closure environment (null for bare functions).
    pub env_ptr: *mut u8,
}

/// A single route entry mapping a URL pattern to a handler.
pub struct RouteEntry {
    /// URL pattern (exact, wildcard ending with `/*`, or parameterized with `:name`).
    pub pattern: String,
    /// Optional HTTP method filter. None = any method, Some("GET") = only GET.
    pub method: Option<String>,
    /// Pointer to the handler function.
    pub handler_fn: *mut u8,
    /// Pointer to the handler closure environment (null for bare functions).
    pub handler_env: *mut u8,
    /// Declared handler runtime name when this route was lowered through HTTP.clustered(...).
    pub declared_handler_runtime_name: Option<String>,
    /// Registered replication count for a clustered route shim.
    pub replication_count: Option<u64>,
}

/// Router holding an ordered list of route entries and middleware.
pub struct MeshRouter {
    pub routes: Vec<RouteEntry>,
    pub middlewares: Vec<MiddlewareEntry>,
}

/// Check if a pattern has any parameterized segments (`:name`).
fn has_param_segments(pattern: &str) -> bool {
    pattern.split('/').any(|seg| seg.starts_with(':'))
}

/// Segment-based matching with parameter extraction.
///
/// Splits both pattern and path on `/`, filters empty segments, then compares
/// pairwise. Literal segments must match exactly; `:name` segments capture the
/// actual value into the returned params vec.
///
/// Returns `Some(params)` on match, `None` on mismatch.
fn match_segments(pattern: &str, path: &str) -> Option<Vec<(String, String)>> {
    let pat_segs: Vec<&str> = pattern.split('/').filter(|s| !s.is_empty()).collect();
    let path_segs: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if pat_segs.len() != path_segs.len() {
        return None;
    }
    let mut params = Vec::new();
    for (pat, actual) in pat_segs.iter().zip(path_segs.iter()) {
        if pat.starts_with(':') {
            params.push((pat[1..].to_string(), actual.to_string()));
        } else if pat != actual {
            return None;
        }
    }
    Some(params)
}

/// Check if a pattern matches a path (for exact and wildcard routes only).
///
/// - Exact match: "/api/health" matches "/api/health"
/// - Wildcard: "/api/*" matches "/api/users", "/api/users/123", etc.
/// - Root wildcard: "/*" matches everything
fn matches_pattern(pattern: &str, path: &str) -> bool {
    if pattern.ends_with("/*") {
        let prefix = &pattern[..pattern.len() - 1]; // strip the '*', keep the '/'
        path.starts_with(prefix) || path == &pattern[..pattern.len() - 2]
    } else {
        pattern == path
    }
}

/// Check if a pattern is a wildcard (ends with `/*`).
fn is_wildcard(pattern: &str) -> bool {
    pattern.ends_with("/*")
}

impl MeshRouter {
    /// Find the first route matching the given path and HTTP method.
    ///
    /// Returns (matched route entry, params) or None.
    /// Uses three-pass matching for priority:
    ///   1. Exact routes (no `:param`, no `*`) -- highest priority
    ///   2. Parameterized routes (`:param` segments) -- medium priority
    ///   3. Wildcard routes (`/*`) -- lowest priority (catch-all fallback)
    /// Within each pass, also checks method filtering.
    pub fn match_route(
        &self,
        path: &str,
        method: &str,
    ) -> Option<(&RouteEntry, Vec<(String, String)>)> {
        // First pass: exact routes only (no `:param` segments, no wildcards).
        for entry in &self.routes {
            if has_param_segments(&entry.pattern) || is_wildcard(&entry.pattern) {
                continue;
            }
            if let Some(ref m) = entry.method {
                if m != method {
                    continue;
                }
            }
            if matches_pattern(&entry.pattern, path) {
                return Some((entry, Vec::new()));
            }
        }

        // Second pass: parameterized routes (have `:param` segments).
        for entry in &self.routes {
            if !has_param_segments(&entry.pattern) {
                continue;
            }
            if let Some(ref m) = entry.method {
                if m != method {
                    continue;
                }
            }
            if let Some(params) = match_segments(&entry.pattern, path) {
                return Some((entry, params));
            }
        }

        // Third pass: wildcard routes (catch-all fallback).
        for entry in &self.routes {
            if !is_wildcard(&entry.pattern) {
                continue;
            }
            if let Some(ref m) = entry.method {
                if m != method {
                    continue;
                }
            }
            if matches_pattern(&entry.pattern, path) {
                return Some((entry, Vec::new()));
            }
        }

        None
    }
}

// ── Internal helper ──────────────────────────────────────────────────

/// Add a route with an optional method filter to the router.
/// Returns a NEW router pointer (immutable semantics).
fn route_with_method(
    router: *mut u8,
    pattern: *const MeshString,
    handler_fn: *mut u8,
    method: Option<&str>,
) -> *mut u8 {
    let handler_env: *mut u8 = std::ptr::null_mut();
    let clustered_metadata = crate::dist::node::lookup_declared_handler_route_metadata(handler_fn);
    unsafe {
        let old = &*(router as *const MeshRouter);
        let pat_str = (*pattern).as_str().to_string();

        let mut new_routes = Vec::with_capacity(old.routes.len() + 1);
        for entry in &old.routes {
            new_routes.push(RouteEntry {
                pattern: entry.pattern.clone(),
                method: entry.method.clone(),
                handler_fn: entry.handler_fn,
                handler_env: entry.handler_env,
                declared_handler_runtime_name: entry.declared_handler_runtime_name.clone(),
                replication_count: entry.replication_count,
            });
        }
        new_routes.push(RouteEntry {
            pattern: pat_str,
            method: method.map(|m| m.to_string()),
            handler_fn,
            handler_env,
            declared_handler_runtime_name: clustered_metadata
                .as_ref()
                .map(|metadata| metadata.runtime_name.clone()),
            replication_count: clustered_metadata.map(|metadata| metadata.replication_count),
        });

        let new_middlewares = old.middlewares.clone();

        let new_router = Box::new(MeshRouter {
            routes: new_routes,
            middlewares: new_middlewares,
        });
        Box::into_raw(new_router) as *mut u8
    }
}

// ── Public extern "C" API ──────────────────────────────────────────────

/// Create an empty router. Returns a pointer to a heap-allocated MeshRouter.
#[no_mangle]
pub extern "C" fn mesh_http_router() -> *mut u8 {
    let router = Box::new(MeshRouter {
        routes: Vec::new(),
        middlewares: Vec::new(),
    });
    Box::into_raw(router) as *mut u8
}

/// Add a route to the router (method-agnostic). Returns a NEW router pointer.
///
/// This is the existing `HTTP.route(router, pattern, handler)` -- matches
/// any HTTP method (backward compatible).
#[no_mangle]
pub extern "C" fn mesh_http_route(
    router: *mut u8,
    pattern: *const MeshString,
    handler_fn: *mut u8,
) -> *mut u8 {
    route_with_method(router, pattern, handler_fn, None)
}

/// Add a GET-only route. Returns a NEW router pointer.
#[no_mangle]
pub extern "C" fn mesh_http_route_get(
    router: *mut u8,
    pattern: *const MeshString,
    handler_fn: *mut u8,
) -> *mut u8 {
    route_with_method(router, pattern, handler_fn, Some("GET"))
}

/// Add a POST-only route. Returns a NEW router pointer.
#[no_mangle]
pub extern "C" fn mesh_http_route_post(
    router: *mut u8,
    pattern: *const MeshString,
    handler_fn: *mut u8,
) -> *mut u8 {
    route_with_method(router, pattern, handler_fn, Some("POST"))
}

/// Add a PUT-only route. Returns a NEW router pointer.
#[no_mangle]
pub extern "C" fn mesh_http_route_put(
    router: *mut u8,
    pattern: *const MeshString,
    handler_fn: *mut u8,
) -> *mut u8 {
    route_with_method(router, pattern, handler_fn, Some("PUT"))
}

/// Add a DELETE-only route. Returns a NEW router pointer.
#[no_mangle]
pub extern "C" fn mesh_http_route_delete(
    router: *mut u8,
    pattern: *const MeshString,
    handler_fn: *mut u8,
) -> *mut u8 {
    route_with_method(router, pattern, handler_fn, Some("DELETE"))
}

/// Add middleware to the router. Returns a NEW router pointer (immutable semantics).
///
/// The middleware function receives (request, next_closure) and returns a response.
/// Multiple middleware compose in registration order: first added = outermost.
#[no_mangle]
pub extern "C" fn mesh_http_use_middleware(router: *mut u8, middleware_fn: *mut u8) -> *mut u8 {
    unsafe {
        let old = &*(router as *const MeshRouter);

        // Copy all existing routes.
        let mut new_routes = Vec::with_capacity(old.routes.len());
        for entry in &old.routes {
            new_routes.push(RouteEntry {
                pattern: entry.pattern.clone(),
                method: entry.method.clone(),
                handler_fn: entry.handler_fn,
                handler_env: entry.handler_env,
                declared_handler_runtime_name: entry.declared_handler_runtime_name.clone(),
                replication_count: entry.replication_count,
            });
        }

        // Copy existing middleware and append the new one.
        let mut new_middlewares = old.middlewares.clone();
        new_middlewares.push(MiddlewareEntry {
            fn_ptr: middleware_fn,
            env_ptr: std::ptr::null_mut(),
        });

        let new_router = Box::new(MeshRouter {
            routes: new_routes,
            middlewares: new_middlewares,
        });
        Box::into_raw(new_router) as *mut u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dist::node::{
        clear_declared_handler_registry_for_test, mesh_register_declared_handler,
    };
    use crate::gc::mesh_rt_init;
    use crate::string::mesh_string_new;
    use std::sync::{Mutex, MutexGuard, OnceLock};

    fn plain_route(pattern: &str, method: Option<&str>, handler_fn: usize) -> RouteEntry {
        RouteEntry {
            pattern: pattern.to_string(),
            method: method.map(str::to_string),
            handler_fn: handler_fn as *mut u8,
            handler_env: std::ptr::null_mut(),
            declared_handler_runtime_name: None,
            replication_count: None,
        }
    }

    fn clustered_route_test_lock() -> MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
    }

    extern "C" fn clustered_route_handler(_request: *mut u8) -> *mut u8 {
        let body = mesh_string_new(b"ok".as_ptr(), 2);
        crate::http::server::mesh_http_response_new(200, body)
    }

    #[test]
    fn test_exact_match() {
        assert!(matches_pattern("/api/health", "/api/health"));
        assert!(!matches_pattern("/api/health", "/api/healthz"));
        assert!(!matches_pattern("/api/health", "/api"));
    }

    #[test]
    fn test_wildcard_match() {
        assert!(matches_pattern("/api/*", "/api/users"));
        assert!(matches_pattern("/api/*", "/api/users/123"));
        assert!(matches_pattern("/api/*", "/api/"));
        assert!(matches_pattern("/api/*", "/api"));
        assert!(!matches_pattern("/api/*", "/other"));
    }

    #[test]
    fn test_root_wildcard() {
        assert!(matches_pattern("/*", "/anything"));
        assert!(matches_pattern("/*", "/a/b/c"));
    }

    #[test]
    fn test_segment_matching() {
        let params = match_segments("/users/:id", "/users/42").unwrap();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].0, "id");
        assert_eq!(params[0].1, "42");

        let params = match_segments("/users/:user_id/posts/:post_id", "/users/7/posts/99").unwrap();
        assert_eq!(params.len(), 2);
        assert_eq!(params[0].0, "user_id");
        assert_eq!(params[0].1, "7");
        assert_eq!(params[1].0, "post_id");
        assert_eq!(params[1].1, "99");

        let params = match_segments("/api/users/:id/profile", "/api/users/42/profile").unwrap();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].0, "id");
        assert_eq!(params[0].1, "42");
    }

    #[test]
    fn test_segment_no_match() {
        assert!(match_segments("/users/:id", "/users/42/extra").is_none());
        assert!(match_segments("/users/:id/posts", "/users/42").is_none());
        assert!(match_segments("/users/:id", "/posts/42").is_none());
    }

    #[test]
    fn test_exact_beats_param() {
        let router = MeshRouter {
            routes: vec![
                plain_route("/users/:id", None, 1),
                plain_route("/users/me", None, 2),
            ],
            middlewares: vec![],
        };
        let (entry, params) = router.match_route("/users/me", "GET").unwrap();
        assert_eq!(entry.handler_fn as usize, 2);
        assert!(params.is_empty());

        let (entry, params) = router.match_route("/users/42", "GET").unwrap();
        assert_eq!(entry.handler_fn as usize, 1);
        assert_eq!(params[0].0, "id");
        assert_eq!(params[0].1, "42");
    }

    #[test]
    fn test_method_filtering() {
        let router = MeshRouter {
            routes: vec![
                plain_route("/users", Some("GET"), 1),
                plain_route("/users", Some("POST"), 2),
            ],
            middlewares: vec![],
        };
        let (entry, _) = router.match_route("/users", "GET").unwrap();
        assert_eq!(entry.handler_fn as usize, 1);

        let (entry, _) = router.match_route("/users", "POST").unwrap();
        assert_eq!(entry.handler_fn as usize, 2);

        assert!(router.match_route("/users", "DELETE").is_none());
    }

    #[test]
    fn test_method_agnostic_route() {
        let router = MeshRouter {
            routes: vec![plain_route("/health", None, 1)],
            middlewares: vec![],
        };
        assert!(router.match_route("/health", "GET").is_some());
        assert!(router.match_route("/health", "POST").is_some());
        assert!(router.match_route("/health", "DELETE").is_some());
    }

    #[test]
    fn test_router_match_order() {
        let router = MeshRouter {
            routes: vec![plain_route("/exact", None, 1), plain_route("/*", None, 2)],
            middlewares: vec![],
        };
        let (entry, _) = router.match_route("/exact", "GET").unwrap();
        assert_eq!(entry.handler_fn as usize, 1);

        let (entry, _) = router.match_route("/other", "GET").unwrap();
        assert_eq!(entry.handler_fn as usize, 2);
    }

    #[test]
    fn test_router_no_match() {
        let router = MeshRouter {
            routes: vec![plain_route("/only-this", None, 1)],
            middlewares: vec![],
        };
        assert!(router.match_route("/other", "GET").is_none());
    }

    #[test]
    fn test_mesh_http_router_and_route() {
        mesh_rt_init();

        let router = mesh_http_router();
        assert!(!router.is_null());

        let pattern = mesh_string_new(b"/hello".as_ptr(), 6);
        let handler_fn = 42usize as *mut u8;

        let router2 = mesh_http_route(router, pattern, handler_fn);
        assert!(!router2.is_null());

        unsafe {
            let r = &*(router2 as *const MeshRouter);
            assert_eq!(r.routes.len(), 1);
            assert_eq!(r.routes[0].pattern, "/hello");
            assert_eq!(r.routes[0].handler_fn as usize, 42);
            assert!(r.routes[0].method.is_none());
            assert!(r.routes[0].declared_handler_runtime_name.is_none());
            assert_eq!(r.routes[0].replication_count, None);
        }
    }

    #[test]
    fn router_registration_attaches_clustered_runtime_metadata() {
        let _guard = clustered_route_test_lock();
        mesh_rt_init();
        clear_declared_handler_registry_for_test();

        let runtime_name = "Api.Todos.handle_list_todos";
        let executable_name = "__declared_route_api_todos_handle_list_todos";
        mesh_register_declared_handler(
            runtime_name.as_ptr(),
            runtime_name.len() as u64,
            executable_name.as_ptr(),
            executable_name.len() as u64,
            2,
            clustered_route_handler as *const u8,
        );

        let router = mesh_http_router();
        let pattern = mesh_string_new(b"/todos".as_ptr(), 6);
        let routed = mesh_http_route_get(router, pattern, clustered_route_handler as *mut u8);

        unsafe {
            let router = &*(routed as *const MeshRouter);
            assert_eq!(router.routes.len(), 1);
            assert_eq!(
                router.routes[0].declared_handler_runtime_name.as_deref(),
                Some(runtime_name)
            );
            assert_eq!(router.routes[0].replication_count, Some(2));
        }

        clear_declared_handler_registry_for_test();
    }
}
