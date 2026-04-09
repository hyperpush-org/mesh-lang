# Comprehensive test: exercises all Mesh language features

# Functions
fn add(a :: Int, b :: Int) -> Int do
  a + b
end

fn double(x :: Int) -> Int do
  x * 2
end

fn negate(x :: Int) -> Int do
  0 - x
end

# Pattern matching on integers
fn describe_number(n :: Int) -> String do
  case n do
    0 -> "zero"
    1 -> "one"
    _ -> "other"
  end
end

# ADTs (sum types) # nullary constructors
type Color do
  Red
  Green
  Blue
end

# If/else
fn max(a :: Int, b :: Int) -> Int do
  if a > b do
    a
  else
    b
  end
end

fn abs_val(x :: Int) -> Int do
  if x > 0 do
    x
  else
    0 - x
  end
end

# Main: exercise all features
fn main() do
  # Functions
  println("${add(10, 20)}")
  println("${double(7)}")
  println("${negate(5)}")

  # Pipe operator
  let piped = 3 |> double
  println("${piped}")

  # Pattern matching
  println(describe_number(0))
  println(describe_number(1))
  println(describe_number(99))

  # ADTs (construction)
  let r = Red
  let g = Green
  let b = Blue
  println("red")
  println("green")
  println("blue")

  # Closures
  let factor = 3
  let triple = fn(x :: Int) -> x * factor end
  println("${triple(7)}")
  println("${triple(10)}")

  # If/else
  println("${max(10, 20)}")
  println("${abs_val(0 - 5)}")

  # String interpolation
  let greeting = "Mesh"
  println("Hello, ${greeting}!")
  let val = 42
  println("The answer is ${val}")

  # Nested operations
  let result = add(double(3), negate(2))
  println("${result}")

  # Boolean logic
  let t = true
  let f = false
  if t and not f do
    println("logic works")
  else
    println("logic broken")
  end
end
