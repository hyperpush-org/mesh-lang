fn main() do
  let result = List.flat_map([1, 2, 3], fn(x) -> [x, x * 10] end)
  println("${List.length(result)}")
  println("${List.head(result)}")
  println("${List.nth(result, 1)}")
  println("${List.nth(result, 2)}")

  let flat = List.flatten([[1, 2], [3, 4], [5]])
  println("${List.length(flat)}")
  println("${List.head(flat)}")
  println("${List.last(flat)}")
end
