fn main() do
  let list = List.new()
  let list = List.append(list, 1)
  let list = List.append(list, 2)
  let list = List.append(list, 3)
  let len = List.length(list)
  println("${len}")
  let h = List.head(list)
  println("${h}")
end
