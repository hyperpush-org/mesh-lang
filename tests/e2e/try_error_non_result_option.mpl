fn compute(x :: Int) -> Int!String do
  let val = x?
  Ok(val + 1)
end

fn main() do
  let r = compute(5)
  case r do
    Ok(v) -> println("${v}")
    Err(e) -> println(e)
  end
end
