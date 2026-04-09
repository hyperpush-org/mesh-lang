# For-in over Set: element iteration
fn main() do
  let s = Set.new()
  let s = Set.add(s, 10)
  let s = Set.add(s, 20)
  let s = Set.add(s, 30)

  # Iterate set elements, collecting them
  let elems = for x in s do
    x
  end

  let total = List.length(elems)
  println("${total}")

  println("done")
end
