struct Tag do
  id :: Int
end deriving(Eq)

fn main() do
  let a = Tag { id: 1 }
  let b = Tag { id: 1 }
  println("${a == b}")
end
