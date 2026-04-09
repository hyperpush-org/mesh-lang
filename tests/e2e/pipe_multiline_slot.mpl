fn add(a :: Int, b :: Int) -> Int do
  a + b
end

fn mul(a :: Int, b :: Int) -> Int do
  a * b
end

fn main() do
  # Leading slot pipe: |2> at start of next line
  let a = 5
    |2> add(10)
  println("${a}")

  # Trailing slot pipe: |2> at end of line (new in Phase 126)
  let b = 5 |2>
    add(10)
  println("${b}")

  # Multi-stage with trailing slot pipe
  let c = 3 |2>
    add(2) |2>
    mul(4)
  println("${c}")
end
