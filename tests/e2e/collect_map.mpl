fn main() do
  # Test 1: Enumerate + Map.collect
  let list = [100, 200, 300]
  let m = Iter.from(list) |> Iter.enumerate() |> Map.collect()
  println("${m}")

  # Test 2: Zip + Map.collect
  let keys = [10, 20, 30]
  let vals = [1, 2, 3]
  let m2 = Iter.from(keys) |> Iter.zip(Iter.from(vals)) |> Map.collect()
  println("${m2}")

  # Test 3: Map size check from test 1 result
  println("${Map.size(m)}")
end
