fn main() do
  let yes = string_contains("hello world", "world")
  let no = string_contains("hello world", "xyz")
  println("${yes}")
  println("${no}")
end
