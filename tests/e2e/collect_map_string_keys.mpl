fn main() do
  # Map with string keys, to_list, then collect roundtrip
  let m = %{"a" => 1, "b" => 2, "c" => 3}
  let pairs = Map.to_list(m)
  let collected = pairs |> Iter.from() |> Map.collect()
  println("${Map.get(collected, "a")}")
  println("${Map.get(collected, "b")}")
  # Map.size on collected string-key map
  println("${Map.size(collected)}")
end
