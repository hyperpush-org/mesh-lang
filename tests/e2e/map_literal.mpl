fn main() do
  let m = %{"name" => "Alice", "age" => "30"}
  let name = Map.get(m, "name")
  println(name)
  let age = Map.get(m, "age")
  println(age)
  let sz = Map.size(m)
  println("${sz}")
end
