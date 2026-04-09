struct Empty do
  val :: Int
end deriving()

fn main() do
  let e = Empty { val: 42 }
  println("42")
end
