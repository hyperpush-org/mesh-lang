fn main() do
  let nested = [[1, 2], [3, 4]]
  let len = List.length(nested)
  println("${len}")
  let inner = List.get(nested, 0)
  let inner_len = List.length(inner)
  println("${inner_len}")
  let val = List.get(inner, 1)
  println("${val}")
end
