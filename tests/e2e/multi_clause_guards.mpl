# Multi-clause functions with guard clauses (when keyword)
# Tests: guard expressions, pattern + guard combos

fn abs(n) when n < 0 = -n
fn abs(n) = n

fn classify(n) when n > 0 = "positive"
fn classify(n) when n < 0 = "negative"
fn classify(n) = "zero"

fn main() do
  println("${abs(-5)}")
  println("${abs(3)}")
  println(classify(10))
  println(classify(-3))
  println(classify(0))
end
