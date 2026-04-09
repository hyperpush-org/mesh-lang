struct Profile do
  name :: String
  bio :: Option<String>
  age :: Option<Int>
end deriving(Row)

fn main() do
  let row = Map.new()
  let row = Map.put(row, "name", "Bob")
  let row = Map.put(row, "bio", "")
  let row = Map.put(row, "age", "25")

  let result = Profile.from_row(row)
  case result do
    Ok(p) -> println(p.name)
    Err(e) -> println(e)
  end
end
