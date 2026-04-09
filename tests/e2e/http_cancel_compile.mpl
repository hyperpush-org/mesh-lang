fn main() do
  let req = Http.build(:get, "http://example.com")
  let handle = Http.stream(req, fn chunk ->
    "ok"
  end)
  Http.cancel(handle)
  println("cancel_called")
end
