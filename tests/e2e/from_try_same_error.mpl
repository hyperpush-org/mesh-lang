# Phase 77: ? operator with same error type (backward compatibility test)

fn risky() -> Int!String do
  Err("fail")
end

fn process() -> Int!String do
  let n = risky()?
  Ok(n)
end

fn main() do
  let result = process()
  case result do
    Ok(n) -> println("ok: ${n}")
    Err(e) -> println("err: ${e}")
  end
end
