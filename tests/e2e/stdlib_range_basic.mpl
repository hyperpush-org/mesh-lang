fn main() do
  let r = Range.new(1, 4)
  let rlen = Range.length(r)
  println("${rlen}")
  let list = Range.to_list(r)
  let llen = List.length(list)
  println("${llen}")
  let h = List.head(list)
  println("${h}")
end
