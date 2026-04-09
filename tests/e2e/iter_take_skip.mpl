fn main() do
  let list = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

  # Take first 3
  let first3 = Iter.from(list) |> Iter.take(3) |> Iter.sum()
  println(first3.to_string())

  # Skip first 7, then sum remaining
  let last3 = Iter.from(list) |> Iter.skip(7) |> Iter.sum()
  println(last3.to_string())

  # Take 0
  let zero = Iter.from(list) |> Iter.take(0) |> Iter.count()
  println(zero.to_string())

  # Skip all
  let skip_all = Iter.from(list) |> Iter.skip(100) |> Iter.count()
  println(skip_all.to_string())
end
