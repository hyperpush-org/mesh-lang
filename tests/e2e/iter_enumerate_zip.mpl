fn main() do
  let list = [10, 20, 30]

  # Enumerate: count elements (verifies enumerate produces tuples)
  let n = Iter.from(list) |> Iter.enumerate() |> Iter.count()
  println(n.to_string())

  # Zip: combine two iterators, count pairs
  let a = [1, 2, 3]
  let b = [4, 5, 6]
  let pairs = Iter.from(a) |> Iter.zip(Iter.from(b)) |> Iter.count()
  println(pairs.to_string())

  # Zip with unequal lengths: shorter determines count
  let short = [1, 2]
  let long = [10, 20, 30, 40]
  let zipped = Iter.from(short) |> Iter.zip(Iter.from(long)) |> Iter.count()
  println(zipped.to_string())
end
