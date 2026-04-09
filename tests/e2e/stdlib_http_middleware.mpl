fn logger(request :: Request, next) -> Response do
  next(request)
end

fn auth_check(request :: Request, next) do
  let path = Request.path(request)
  let is_secret = String.starts_with(path, "/secret")
  if is_secret do
    HTTP.response(401, "Unauthorized")
  else
    next(request)
  end
end

fn handler(request :: Request) do
  HTTP.response(200, "hello-world")
end

fn secret_handler(request :: Request) do
  HTTP.response(200, "secret-data")
end

fn main() do
  let r = HTTP.router()
  let r = HTTP.use(r, logger)
  let r = HTTP.use(r, auth_check)
  let r = HTTP.route(r, "/hello", handler)
  let r = HTTP.route(r, "/secret", secret_handler)
  HTTP.serve(r, 18083)
end
