fn main() do
  let same = Crypto.secure_compare("hello", "hello")
  let diff = Crypto.secure_compare("hello", "world")
  let diff_len = Crypto.secure_compare("hi", "hello")
  println("${same}")
  println("${diff}")
  println("${diff_len}")
end
