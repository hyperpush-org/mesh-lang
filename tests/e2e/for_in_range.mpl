# FORIN-02: Basic range iteration
# FORIN-08: Loop variable scoped to body
fn main() do
  # Basic range: prints 0 through 4
  for i in 0..5 do
    println("${i}")
  end

  # Separator
  println("---")

  # Reuse i as loop variable (scoped to body)
  for i in 10..13 do
    println("${i}")
  end

  println("done")
end
