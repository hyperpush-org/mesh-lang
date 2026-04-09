# Mesh HTTP benchmark server.
# Exposes two endpoints for wrk load testing:
#   GET /text  -> 200 text/plain  "Hello, World!"
#   GET /json  -> 200 application/json  {"message":"Hello, World!"}
# Listens on port 3000 (wrk default target).

fn handle_text(request) do
  HTTP.response(200, "Hello, World!")
end

fn handle_json(request) do
  HTTP.response(200, "{\"message\":\"Hello, World!\"}")
end

fn main() do
  HTTP.serve((HTTP.router()
    |> HTTP.on_get("/text", handle_text)
    |> HTTP.on_get("/json", handle_json)), 3000)
end
