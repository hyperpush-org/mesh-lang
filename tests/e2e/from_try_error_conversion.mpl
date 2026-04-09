# Phase 77: ? operator correctly propagates errors through function chains.
# Tests chained ? desugaring with multiple Result-returning calls.

fn divide(a :: Int, b :: Int) -> Int!String do
  if b == 0 do
    return Err("divide by zero")
  end
  Ok(a / b)
end

fn compute(x :: Int) -> Int!String do
  let a = divide(x, 2)?
  let b = divide(a, 3)?
  Ok(b + 100)
end

fn main() do
  let r1 = compute(60)
  case r1 do
    Ok(val) -> println("${val}")
    Err(msg) -> println(msg)
  end
end
