fn main() do
  let xs = [1, 2, 3]
  let len = List.length(xs)
  println("${len}")
  let first = List.get(xs, 0)
  println("${first}")
  let last = List.get(xs, 2)
  println("${last}")
end
