fn may_fail(x :: Int) -> Int!String do
  if x < 0 do
    return Err("negative: ${x}")
  end
  Ok(x * 2)
end

fn main() do
  let r1 = may_fail(5)
  case r1 do
    Ok(v) -> println("ok: ${v}")
    Err(e) -> println(e)
  end

  let r2 = may_fail(-3)
  case r2 do
    Ok(v) -> println("ok: ${v}")
    Err(e) -> println(e)
  end
end
