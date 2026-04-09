fn find_positive(a :: Int, b :: Int) -> Int? do
  if a > 0 do
    return Some(a)
  end
  if b > 0 do
    return Some(b)
  end
  None
end

fn add_positive(a :: Int, b :: Int) -> Int? do
  let x = find_positive(a, b)?
  Some(x + 100)
end

fn main() do
  let r = add_positive(-1, -2)
  case r do
    Some(val) -> println("${val}")
    None -> println("none")
  end
end
