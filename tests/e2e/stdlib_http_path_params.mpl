fn me_handler(request) do
  HTTP.response(200, "me")
end

fn user_handler(request) do
  let param = Request.param(request, "id")
  case param do
    Some(id) -> HTTP.response(200, id)
    None -> HTTP.response(400, "no-id")
  end
end

fn post_handler(request) do
  HTTP.response(200, "posted")
end

fn fallback_handler(request) do
  HTTP.response(200, "fallback")
end

fn main() do
  let r = HTTP.router()
  let r = HTTP.on_get(r, "/users/me", me_handler)
  let r = HTTP.on_get(r, "/users/:id", user_handler)
  let r = HTTP.on_post(r, "/data", post_handler)
  let r = HTTP.route(r, "/*", fallback_handler)
  HTTP.serve(r, 18082)
end
