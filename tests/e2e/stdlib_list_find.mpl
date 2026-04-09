fn main() do
  let nums = [1, 2, 3, 4, 5]
  let found = List.find(nums, fn(x) -> x > 3 end)
  let not_found = List.find(nums, fn(x) -> x > 10 end)
  println("ok")
end
