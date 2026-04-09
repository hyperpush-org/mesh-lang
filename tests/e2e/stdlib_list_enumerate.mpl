fn main() do
  let pairs = List.enumerate([10, 20, 30])
  println("${List.length(pairs)}")
  let first_pair = List.head(pairs)
  let idx = Tuple.first(first_pair)
  let val = Tuple.second(first_pair)
  println("${idx}")
  println("${val}")
end
