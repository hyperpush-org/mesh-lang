fn main() do
  let ss = ["hello"]
  let ss = List.append(ss, "world")
  let len = List.length(ss)
  println("${len}")
  let second = List.get(ss, 1)
  println(second)
end
