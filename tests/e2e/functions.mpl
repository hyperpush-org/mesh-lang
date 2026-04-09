fn add(a :: Int, b :: Int) -> Int do
  a + b
end

fn double(x :: Int) -> Int do
  x * 2
end

fn main() do
  println("${add(3, 4)}")
  println("${double(5)}")
end
