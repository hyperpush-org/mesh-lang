fn safe_divide(a :: Int, b :: Int) -> Int!String do
  if b == 0 do
    return Err("division by zero")
  end
  Ok(a / b)
end

fn compute(x :: Int) -> Int!String do
  let result = safe_divide(x, 0)?
  Ok(result + 10)
end

fn main() do
  let r = compute(20)
  case r do
    Ok(val) -> println("${val}")
    Err(msg) -> println(msg)
  end
end
