fn main() do
  let list = List.new()
  let list = List.append(list, 1)
  let list = List.append(list, 2)
  let list = List.append(list, 3)
  let list = List.append(list, 4)
  let list = List.append(list, 5)

  # Chained pipes with closures: map |> filter |> reduce
  # [1,2,3,4,5] -> map +1 [2,3,4,5,6] -> filter >3 [4,5,6] -> sum 15
  let result = list |> map(fn x -> x + 1 end) |> filter(fn x -> x > 3 end) |> reduce(0, fn acc, x -> acc + x end)

  println("${result}")
end
