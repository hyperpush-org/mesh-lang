fn fib(0) = 0
fn fib(1) = 1
fn fib(n) = fib(n - 1) + fib(n - 2)

fn to_string(true) = "yes"
fn to_string(false) = "no"

fn double(x) = x * 2

fn square(x :: Int) -> Int do
  x * x
end

fn main() do
  println("${fib(10)}")
  println(to_string(true))
  println(to_string(false))
  println("${double(21)}")
  println("${square(6)}")
end
