fn validate_positive(x :: Int) -> Int!String do
  if x <= 0 do
    return Err("must be positive")
  end
  Ok(x)
end

fn validate_small(x :: Int) -> Int!String do
  if x > 100 do
    return Err("too large")
  end
  Ok(x)
end

fn process(x :: Int) -> String!String do
  let v = validate_positive(x)?
  let w = validate_small(v)?
  Ok("valid: ${w}")
end

fn main() do
  let r1 = process(42)
  case r1 do
    Ok(s) -> println(s)
    Err(e) -> println("error: ${e}")
  end

  let r2 = process(-5)
  case r2 do
    Ok(s) -> println(s)
    Err(e) -> println("error: ${e}")
  end

  let r3 = process(200)
  case r3 do
    Ok(s) -> println(s)
    Err(e) -> println("error: ${e}")
  end
end
