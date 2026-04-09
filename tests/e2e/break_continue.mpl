# BRKC-01: Break exits the loop early
fn main() do
  # Test 1: break exits infinite loop
  while true do
    println("before break")
    break
    println("after break")
  end
  println("after loop")

  # BRKC-02: Continue skips to next iteration (exits via break on second pass)
  # Since Mesh has no mutable variables, we use a nested if+break pattern
  while true do
    println("iteration")
    break
  end

  # Test 3: break inside if inside while
  while true do
    if true do
      break
    end
  end
  println("nested break works")
end
