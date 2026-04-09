fn main() do
  let list = List.new()
  let list = List.append(list, 1)
  let list = List.append(list, 2)
  let list = List.append(list, 3)
  let list = List.append(list, 4)
  let list = List.append(list, 5)

  # Bare single param closure via pipe
  let doubled = list |> map(fn x -> x * 2 end)

  # Bare single param closure with filter via pipe
  let filtered = doubled |> filter(fn x -> x > 4 end)

  # Multi-param bare closure (reduce takes 3 args, kept as direct call)
  let sum = reduce(filtered, 0, fn acc, x -> acc + x end)

  println("${sum}")
end
