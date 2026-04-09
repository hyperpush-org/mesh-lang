fn main() do
  let s = Set.new()
  let s = Set.add(s, 1)
  let s = Set.add(s, 2)
  let s = Set.add(s, 1)
  let sz = Set.size(s)
  println("${sz}")
end
