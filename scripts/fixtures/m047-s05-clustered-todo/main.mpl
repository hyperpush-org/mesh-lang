from Api.Router import build_router
from Runtime.Registry import start_registry
from Services.RateLimiter import start_rate_limiter
from Storage.Todos import ensure_schema

fn log_bootstrap(status :: BootstrapStatus) do
  println(
    "[todo-api] runtime bootstrap mode=#{status.mode} node=#{status.node_name} cluster_port=#{status.cluster_port} discovery_seed=#{status.discovery_seed}"
  )
end

fn log_bootstrap_failure(reason :: String) do
  println("[todo-api] runtime bootstrap failed reason=#{reason}")
end

fn env_positive_int(name :: String, default_value :: Int) -> Int do
  let raw = Env.get(name, "")
  if raw == "" do
    default_value
  else
    let value = Env.get_int(name, -1)
    if value > 0 do
      value
    else
      default_value
    end
  end
end

fn start_http_runtime(port :: Int, db_path :: String, window_seconds :: Int, max_requests :: Int) do
  let limiter_pid = start_rate_limiter(window_seconds, max_requests)
  start_registry(db_path, limiter_pid, window_seconds, max_requests)
  println(
    "[todo-api] Runtime ready port=#{port} db_path=#{db_path} write_limit_window_seconds=#{window_seconds} write_limit_max=#{max_requests}"
  )
  let router = build_router()
  HTTP.serve(router, port)
end

fn main() do
  case Node.start_from_env() do
    Ok(status) -> log_bootstrap(status)
    Err(reason) -> log_bootstrap_failure(reason)
  end

  let port = env_positive_int("PORT", 8080)
  let db_path = Env.get("TODO_DB_PATH", "todo.sqlite3")
  let window_seconds = env_positive_int("TODO_RATE_LIMIT_WINDOW_SECONDS", 60)
  let max_requests = env_positive_int("TODO_RATE_LIMIT_MAX_REQUESTS", 5)

  case ensure_schema(db_path) do
    Ok(_) -> start_http_runtime(port, db_path, window_seconds, max_requests)
    Err(reason) -> println("[todo-api] Database init failed: #{reason}")
  end
end
