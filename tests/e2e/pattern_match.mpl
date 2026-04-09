fn describe(x :: Int) -> String do
  case x do
    0 -> "zero"
    1 -> "one"
    _ -> "other"
  end
end

fn main() do
  println(describe(0))
  println(describe(1))
  println(describe(42))
end
