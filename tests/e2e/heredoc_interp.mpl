fn main() do
  let id = 42
  let name = "Alice"
  let body = """
    {"id": #{id}, "name": "#{name}"}
    """
  println(body)
end
