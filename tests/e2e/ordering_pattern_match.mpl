fn main() do
  let ord = compare(3, 5)
  let result = case ord do
    Less -> 1
    Equal -> 2
    Greater -> 3
  end
  println("${result}")
end
