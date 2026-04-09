fn main() do
  let a = [1, 2]
  let b = [3, 4]
  let combined = a ++ b
  let len = List.length(combined)
  println("${len}")
  let first = List.get(combined, 0)
  let last = List.get(combined, 3)
  println("${first}")
  println("${last}")
end
