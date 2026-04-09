type Color do
  Red
  Green
  Blue
end deriving(Eq, Ord, Display, Debug, Hash)

fn main() do
  let r = Red
  let g = Green
  let b = Blue
  println("${r}")
  println("${g}")
  println("${b}")
  println("${r == r}")
  println("${r == g}")
end
