fn main() do
  let m = Map.new()
  let m = Map.put(m, 1, 10)
  let m = Map.put(m, 2, 20)
  let v = Map.get(m, 1)
  println("${v}")
  let sz = Map.size(m)
  println("${sz}")
end
