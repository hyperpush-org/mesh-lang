# WHILE-01: Basic while loop -- body executes while condition is true
# Uses break to exit since Mesh has no mutable assignment
fn main() do
  # Test 1: while with break -- body runs at least once
  while true do
    println("loop ran")
    break
  end

  # WHILE-02: Condition initially false -- body executes zero times
  while false do
    println("should not print")
  end
  println("skipped")

  # WHILE-03: While returns Unit (no type error when used as expression)
  let result = while false do
    42
  end
  println("done")
end
