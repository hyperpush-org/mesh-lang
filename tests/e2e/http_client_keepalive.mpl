fn main() do
  let client = Http.client()
  let req = Http.build(:get, "http://example.com")
  println("client_created")
  Http.client_close(client)
end
