fn main() do
  let m = %{1 => 10, 2 => 20, 3 => 30}
  let v = Map.get(m, 2)
  println("${v}")
  let sz = Map.size(m)
  println("${sz}")
end
