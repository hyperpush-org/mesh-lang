fn main() do
  let m = Map.new()
  let m = Map.put(m, "name", "Alice")
  let m = Map.put(m, "city", "Portland")
  let name = Map.get(m, "name")
  println(name)
  let sz = Map.size(m)
  println("${sz}")
  let has = Map.has_key(m, "name")
  println("${has}")
  let m = Map.put(m, "name", "Bob")
  let name2 = Map.get(m, "name")
  println(name2)
end
