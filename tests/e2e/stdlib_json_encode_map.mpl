fn main() do
  let result = JSON.encode_int(100)
  println(result)
  let s = JSON.encode_string("test")
  println(s)
  let b = JSON.encode_bool(true)
  println(b)
end
