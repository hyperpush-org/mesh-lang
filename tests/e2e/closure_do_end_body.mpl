fn main() do
  let list = List.new()
  let list = List.append(list, 1)
  let list = List.append(list, 2)
  let list = List.append(list, 3)

  let result = map(list, fn x do
    let doubled = x * 2
    let incremented = doubled + 1
    incremented
  end)

  let sum = reduce(result, 0, fn acc, x -> acc + x end)
  println("${sum}")
end
