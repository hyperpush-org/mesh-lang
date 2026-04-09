# For-in over List: comprehension semantics, nested iteration, break, continue
fn main() do
  # Basic list comprehension: double each element
  let nums = [1, 2, 3]
  let doubled = for x in nums do
    x * 2
  end

  # Iterate result list and print
  for d in doubled do
    println("${d}")
  end

  println("---")

  # Continue skips element (not pushed to result list)
  let filtered = for x in [1, 2, 3, 4, 5] do
    if x == 3 do
      continue
    end
    x * 10
  end
  for f in filtered do
    println("${f}")
  end

  println("---")

  # Break returns partial list
  let partial = for x in [10, 20, 30, 40, 50] do
    if x == 30 do
      break
    end
    x
  end
  let len = List.length(partial)
  println("${len}")

  println("done")
end
