fn handler(request) do
  HTTP.response(200, "{\"status\":\"ok\"}")
end

fn main() do
  let r = HTTP.router()
  let r = HTTP.route(r, "/health", handler)
  HTTP.serve(r, 18080)
end

