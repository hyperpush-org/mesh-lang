fn main() do
  let list = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

  # Map: double each element
  let doubled = Iter.from(list) |> Iter.map(fn x -> x * 2 end) |> Iter.count()
  println(doubled.to_string())

  # Filter: keep even numbers
  let even_count = Iter.from(list) |> Iter.filter(fn x -> x % 2 == 0 end) |> Iter.count()
  println(even_count.to_string())

  # Map + Filter chain: double then keep > 10
  let big = Iter.from(list) |> Iter.map(fn x -> x * 2 end) |> Iter.filter(fn x -> x > 10 end) |> Iter.count()
  println(big.to_string())

  # Map then sum
  let sum = Iter.from(list) |> Iter.map(fn x -> x * 3 end) |> Iter.sum()
  println(sum.to_string())
end
