fn main() do
  let list = [1, 2, 3, 4, 5]

  # Count (TERM-01)
  let c = Iter.from(list) |> Iter.count()
  println(c.to_string())

  # Sum (TERM-02)
  let s = Iter.from(list) |> Iter.sum()
  println(s.to_string())

  # Any - true case (TERM-03)
  let has_even = Iter.from(list) |> Iter.any(fn x -> x % 2 == 0 end)
  println(has_even.to_string())

  # Any - false case
  let has_big = Iter.from(list) |> Iter.any(fn x -> x > 100 end)
  println(has_big.to_string())

  # All - true case (TERM-03)
  let all_pos = Iter.from(list) |> Iter.all(fn x -> x > 0 end)
  println(all_pos.to_string())

  # All - false case
  let all_even = Iter.from(list) |> Iter.all(fn x -> x % 2 == 0 end)
  println(all_even.to_string())

  # Reduce (TERM-05)
  let product = Iter.from(list) |> Iter.reduce(1, fn acc, x -> acc * x end)
  println(product.to_string())

  # Reduce sum
  let sum2 = Iter.from(list) |> Iter.reduce(0, fn acc, x -> acc + x end)
  println(sum2.to_string())
end
