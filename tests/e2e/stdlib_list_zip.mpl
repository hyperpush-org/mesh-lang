fn main() do
  let a = [1, 2, 3]
  let b = [10, 20, 30]
  let zipped = List.zip(a, b)
  let first_pair = List.head(zipped)
  let t_first = Tuple.first(first_pair)
  let t_second = Tuple.second(first_pair)
  println("${t_first}")
  println("${t_second}")
  println("${List.length(zipped)}")

  let short = List.zip([1, 2], [10, 20, 30])
  println("${List.length(short)}")
end
