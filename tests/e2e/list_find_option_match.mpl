fn main() do
  let nums = [10, 20, 30, 40, 50]

  let found = List.find(nums, fn(x) -> x > 25 end)
  case found do
    Some(x) -> println("found: ${x}")
    None -> println("not found")
  end

  let missing = List.find(nums, fn(x) -> x > 100 end)
  case missing do
    Some(x) -> println("found: ${x}")
    None -> println("not found")
  end
end
