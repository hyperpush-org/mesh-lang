fn main() do
  let h256 = Crypto.hmac_sha256("Jefe", "what do ya want for nothing?")
  println(h256)
  let h512 = Crypto.hmac_sha512("Jefe", "what do ya want for nothing?")
  println(h512)
end
