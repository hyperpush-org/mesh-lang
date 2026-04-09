fn main() do
  let s1 = Set.add(Set.add(Set.add(Set.new(), 1), 2), 3)
  let s2 = Set.add(Set.add(Set.new(), 2), 3)
  let diff = Set.difference(s1, s2)
  println("${Set.size(diff)}")
  println("${Set.contains(diff, 1)}")
  println("${Set.contains(diff, 2)}")

  let as_list = Set.to_list(s1)
  println("${List.length(as_list)}")

  let from = Set.from_list([1, 2, 2, 3, 3, 3])
  println("${Set.size(from)}")
  println("${Set.contains(from, 1)}")
  println("${Set.contains(from, 3)}")
end
