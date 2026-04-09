fn do_crash(0) = 0

fn crash_handler(request) do
  do_crash(42)
  HTTP.response(200, "unreachable")
end

fn health_handler(request) do
  HTTP.response(200, "{\"status\":\"ok\"}")
end

fn main() do
  let r = HTTP.router()
  let r = HTTP.route(r, "/crash", crash_handler)
  let r = HTTP.route(r, "/health", health_handler)
  HTTP.serve(r, 18081)
end
