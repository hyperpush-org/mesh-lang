fn main() do
  let nums = [3, 1, 4, 1, 5, 9, 2, 6]
  let sorted = List.sort(nums, fn(a, b) -> a - b end)
  let first = List.head(sorted)
  let last_sorted = List.sort(nums, fn(a, b) -> b - a end)
  let first_desc = List.head(last_sorted)
  println("${first}")
  println("${first_desc}")
  println("${List.length(sorted)}")
end
