fn handler(request) do
  HTTP.response(200, "Hello from Mesh!")
end

fn main() do
  let r = HTTP.router()
  let r = HTTP.route(r, "/", handler)
  println("compiled")
end
