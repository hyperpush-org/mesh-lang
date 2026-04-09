fn sum_list(xs) do
  case xs do
    head :: tail -> head + sum_list(tail)
    _ -> 0
  end
end

fn main() do
  let result = sum_list([1, 2, 3, 4, 5])
  println("${result}")
end
