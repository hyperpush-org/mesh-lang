fn double(x :: Int) -> Int do
  x * 2
end

fn add_one(x :: Int) -> Int do
  x + 1
end

fn main() do
  let result = 5 |> double |> add_one
  println("${result}")
end
