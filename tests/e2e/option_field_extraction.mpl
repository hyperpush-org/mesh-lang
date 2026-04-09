fn main() do
  let opt = Some(42)
  let result = case opt do
    Some(x) -> x
    None -> 0
  end
  println("${result}")
end
