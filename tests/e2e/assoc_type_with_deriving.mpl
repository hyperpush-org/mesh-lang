struct Wrapper do
  x :: Int
  y :: Int
end deriving(Display)

interface Container do
  type Item
  fn get(self) -> Self.Item
end

impl Container for Wrapper do
  type Item = Int
  fn get(self) -> Int do
    99
  end
end

fn main() do
  let w = Wrapper { x: 1, y: 2 }
  println(w.to_string())
  let val = w.get()
  println(val.to_string())
end
