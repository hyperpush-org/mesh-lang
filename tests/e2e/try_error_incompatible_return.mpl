fn might_fail(x :: Int) -> Int!String do
  if x == 0 do
    return Err("zero")
  end
  Ok(x)
end

fn bad_caller() -> Int do
  let val = might_fail(5)?
  val + 1
end

fn main() do
  println("${bad_caller()}")
end
