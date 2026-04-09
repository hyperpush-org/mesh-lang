fn main() do
  # Test 1: Set.collect with deduplication
  let list = [1, 2, 2, 3, 3, 3]
  let s = Iter.from(list) |> Set.collect()
  println("${Set.size(s)}")

  # Test 2: Set.collect from pipeline
  let s2 = Iter.from([1, 2, 3, 4, 5]) |> Iter.filter(fn x -> x > 2 end) |> Set.collect()
  println("${Set.size(s2)}")

  # Test 3: Set.contains check
  println("${Set.contains(s, 2)}")

  # Test 4: String.collect
  let words = ["hello", " ", "world"]
  let joined = Iter.from(words) |> String.collect()
  println(joined)

  # Test 5: String.collect from simple list
  let abc = Iter.from(["a", "b", "c"]) |> String.collect()
  println(abc)
end
