fn main() do
  let parts = String.split("hello,world,foo", ",")
  println("${List.length(parts)}")
  println(List.head(parts))
  let joined = String.join(parts, " - ")
  println(joined)
  let words = String.split("one two three", " ")
  let back = String.join(words, ",")
  println(back)
end
