# Fun() type annotation end-to-end test
# Tests TYPE-01 (parsing), TYPE-02 (positions), TYPE-03 (unification)

# TYPE-01: Single-param function type in parameter
fn apply(f :: Fun(Int) -> String, x :: Int) -> String do
  f(x)
end

# TYPE-01: Zero-arity function type
fn run_thunk(thunk :: Fun() -> Int) -> Int do
  thunk()
end

# TYPE-01: Multi-param function type
fn apply2(f :: Fun(Int, Int) -> Int, a :: Int, b :: Int) -> Int do
  f(a, b)
end

# TYPE-02: Type alias using Fun()
type Mapper = Fun(Int) -> String

fn main() do
  # TYPE-03: Closure unifies with Fun(Int) -> String
  let result = apply(fn x -> "${x}" end, 42)
  println(result)

  # TYPE-03: Closure unifies with Fun() -> Int
  let val = run_thunk(fn -> 99 end)
  println("${val}")

  # TYPE-03: Closure unifies with Fun(Int, Int) -> Int
  let sum = apply2(fn a, b -> a + b end, 10, 20)
  println("${sum}")
end
