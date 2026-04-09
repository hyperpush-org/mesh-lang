struct Pair do
  a :: Int
  b :: Int
end

fn main() do
  let p = Pair { a: 10, b: 20 }
  let q = Pair { a: 10, b: 20 }
  println("${p == q}")
end
