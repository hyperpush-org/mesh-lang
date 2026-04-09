fn main() do
  let ord = compare(5, 3)
  let result = case ord do
    Less -> 0
    Equal -> 1
    Greater -> 2
  end
  println("${result}")
end
