# Phase 77: User-defined From trait implementation
# Wrapper.from(21) calls user-provided conversion, doubling the value.

struct Wrapper do
  value :: Int
end

impl From<Int> for Wrapper do
  fn from(n :: Int) -> Wrapper do
    Wrapper { value: n * 2 }
  end
end

fn main() do
  let w = Wrapper.from(21)
  println("${w.value}")
end
