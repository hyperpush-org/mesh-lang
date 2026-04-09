fn main() do
  let m1 = Map.put(Map.put(Map.new(), 1, 10), 2, 20)
  let m2 = Map.put(Map.put(Map.new(), 2, 200), 3, 30)
  let merged = Map.merge(m1, m2)
  println("${Map.size(merged)}")
  println("${Map.get(merged, 1)}")
  println("${Map.get(merged, 2)}")
  println("${Map.get(merged, 3)}")

  let as_list = Map.to_list(m1)
  println("${List.length(as_list)}")

  let rebuilt = Map.from_list(as_list)
  println("${Map.size(rebuilt)}")
  println("${Map.get(rebuilt, 1)}")
  println("${Map.get(rebuilt, 2)}")
end
