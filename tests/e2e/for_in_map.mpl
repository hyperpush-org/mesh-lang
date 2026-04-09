# For-in over Map: destructuring iteration with {k, v}
fn main() do
  let m = Map.new()
  let m = Map.put(m, 1, 10)
  let m = Map.put(m, 2, 20)
  let m = Map.put(m, 3, 30)

  # Iterate map entries, collecting values
  let vals = for {k, v} in m do
    v
  end

  # Print collected values (order depends on map iteration order)
  let total = List.length(vals)
  println("${total}")

  println("done")
end
