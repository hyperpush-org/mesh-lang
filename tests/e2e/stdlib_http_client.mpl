fn main() do
  let result = HTTP.get("http://example.com")
  case result do
    Ok(body) -> println("ok")
    Err(msg) -> println("error")
  end
end
