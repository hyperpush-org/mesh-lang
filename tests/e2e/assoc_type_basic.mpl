struct IntPair do
  a :: Int
  b :: Int
end

struct StrPair do
  a :: String
  b :: String
end

interface Container do
  type Item
  fn first(self) -> Self.Item
end

impl Container for IntPair do
  type Item = Int
  fn first(self) -> Int do
    42
  end
end

impl Container for StrPair do
  type Item = String
  fn first(self) -> String do
    "hello"
  end
end

fn main() do
  let ip = IntPair { a: 1, b: 2 }
  let sp = StrPair { a: "x", b: "y" }
  let i = ip.first()
  let s = sp.first()
  println(i.to_string())
  println(s)
end
