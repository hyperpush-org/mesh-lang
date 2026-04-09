struct Point do
  x :: Int
  y :: Int
end deriving(Eq, Ord, Display, Debug, Hash)

fn main() do
  let p = Point { x: 1, y: 2 }
  let q = Point { x: 1, y: 2 }
  let r = Point { x: 3, y: 4 }
  println("${p}")
  println("${p == q}")
  println("${p == r}")
end
