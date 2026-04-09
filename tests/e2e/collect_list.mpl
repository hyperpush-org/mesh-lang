fn main() do
  # Test 1: Basic List.collect -- pipe syntax
  let list = [1, 2, 3]
  let result = Iter.from(list) |> List.collect()
  println("${List.length(result)}")

  # Test 2: Map + collect pipeline
  let doubled = Iter.from(list) |> Iter.map(fn x -> x * 2 end) |> List.collect()
  println("${doubled}")

  # Test 3: Filter + collect
  let big = Iter.from([1, 2, 3, 4, 5]) |> Iter.filter(fn x -> x > 3 end) |> List.collect()
  println("${big}")

  # Test 4: Direct call syntax
  let iter = Iter.from([10, 20, 30])
  let direct = List.collect(iter)
  println("${direct}")

  # Test 5: Empty iterator via take(0)
  let empty = Iter.from([1, 2, 3]) |> Iter.take(0) |> List.collect()
  println("${List.length(empty)}")
end
