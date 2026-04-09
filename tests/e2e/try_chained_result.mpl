fn step1(x :: Int) -> Int!String do
  if x < 0 do
    return Err("negative input")
  end
  Ok(x * 2)
end

fn step2(x :: Int) -> Int!String do
  if x > 100 do
    return Err("too large")
  end
  Ok(x + 1)
end

fn pipeline(x :: Int) -> Int!String do
  let a = step1(x)?
  let b = step2(a)?
  Ok(b)
end

fn main() do
  let r1 = pipeline(10)
  case r1 do
    Ok(val) -> println("${val}")
    Err(msg) -> println(msg)
  end
  let r2 = pipeline(-5)
  case r2 do
    Ok(val2) -> println("${val2}")
    Err(msg2) -> println(msg2)
  end
  let r3 = pipeline(60)
  case r3 do
    Ok(val3) -> println("${val3}")
    Err(msg3) -> println(msg3)
  end
end
