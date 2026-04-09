fn main() do
  let nums = [1, 2, 3, 4, 5]
  let has_even = List.any(nums, fn(x) -> x % 2 == 0 end)
  let all_pos = List.all(nums, fn(x) -> x > 0 end)
  let all_even = List.all(nums, fn(x) -> x % 2 == 0 end)
  let none_neg = List.any(nums, fn(x) -> x < 0 end)
  println("${has_even}")
  println("${all_pos}")
  println("${all_even}")
  println("${none_neg}")
end
