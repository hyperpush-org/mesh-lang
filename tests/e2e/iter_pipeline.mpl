fn main() do
  let list = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

  # Multi-combinator pipeline: map -> filter -> take -> count
  let result = Iter.from(list) |> Iter.map(fn x -> x * 2 end) |> Iter.filter(fn x -> x > 10 end) |> Iter.take(3) |> Iter.count()
  println(result.to_string())

  # Pipeline: filter -> map -> sum
  let result2 = Iter.from(list) |> Iter.filter(fn x -> x > 5 end) |> Iter.map(fn x -> x * 10 end) |> Iter.sum()
  println(result2.to_string())

  # Pipeline: skip -> take -> count (windowing)
  let window = Iter.from(list) |> Iter.skip(2) |> Iter.take(5) |> Iter.count()
  println(window.to_string())

  # Pipeline with closure capturing local variable
  let threshold = 3
  let above = Iter.from(list) |> Iter.filter(fn x -> x > threshold end) |> Iter.count()
  println(above.to_string())
end
