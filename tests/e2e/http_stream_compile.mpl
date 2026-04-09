fn main() do
  let req = Http.build(:get, "http://example.com")
  let handle = Http.stream(req, fn chunk do
    println(chunk)
    "ok"
  end)
  println("stream_called")
end
