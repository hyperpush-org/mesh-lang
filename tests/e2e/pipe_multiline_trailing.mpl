fn double(x :: Int) -> Int do
  x * 2
end

fn add_one(x :: Int) -> Int do
  x + 1
end

fn negate(x :: Int) -> Int do
  0 - x
end

fn add(x :: Int, y :: Int) -> Int do
  x + y
end

fn main() do
  # Trailing |> form: pipe operator at end of line
  let a = 5 |>
    double |>
    add_one
  println("${a}")

  # Mixed: trailing first, then leading
  let b = 10 |>
    add(5)
    |> double
  println("${b}")
end
