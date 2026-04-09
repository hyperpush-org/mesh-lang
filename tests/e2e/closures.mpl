fn main() do
  let n = 5
  let add_n = fn(x :: Int) -> x + n end
  println("${add_n(3)}")
  println("${add_n(10)}")
end
