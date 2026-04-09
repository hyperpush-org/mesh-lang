type Color do
  Red
  Green
  Blue
end

fn describe(c :: Color) -> Int do
  case c do
    Red -> 1
    Green -> 2
    Blue -> 3
  end
end

fn main() do
  println("${describe(Red)}")
  println("${describe(Green)}")
  println("${describe(Blue)}")
end
