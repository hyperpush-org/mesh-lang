fn main() do
  let xs = [10, 20, 30, 40, 50]
  let first3 = List.take(xs, 3)
  println("${List.length(first3)}")
  println("${List.head(first3)}")
  println("${List.last(first3)}")

  let last2 = List.drop(xs, 3)
  println("${List.length(last2)}")
  println("${List.head(last2)}")

  let over = List.take(xs, 100)
  println("${List.length(over)}")

  let drop_all = List.drop(xs, 100)
  println("${List.length(drop_all)}")
end
