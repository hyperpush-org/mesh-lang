fn main() do
  let list = List.new()
  let list = List.append(list, 0)
  let list = List.append(list, 1)
  let list = List.append(list, 2)
  let list = List.append(list, 3)

  # Multi-clause closure with literal patterns
  let classify = fn 0 -> 0 | n -> 1 end
  let result = map(list, classify)

  # Use reduce to sum the classified values
  let sum = reduce(result, 0, fn acc, x -> acc + x end)
  println("${sum}")
end
