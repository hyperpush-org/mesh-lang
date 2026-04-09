fn main() do
  # Test: zip string keys with int values, then collect into a Map<String, Int>
  let keys = ["a", "b", "c"]
  let vals = [1, 2, 3]
  let m = keys |> Iter.from() |> Iter.zip(Iter.from(vals)) |> Map.collect()
  println("${Map.get(m, "a")}")
  println("${Map.get(m, "b")}")
  println("${Map.size(m)}")
end
