struct User do
  name :: String
  age :: Int
  score :: Float
  active :: Bool
end deriving(Row)

fn main() do
  let row = Map.new()
  let row = Map.put(row, "name", "Alice")
  let row = Map.put(row, "age", "30")
  let row = Map.put(row, "score", "95.5")
  let row = Map.put(row, "active", "t")

  let result = User.from_row(row)
  case result do
    Ok(u) -> println("${u.name} ${u.age} ${u.score} ${u.active}")
    Err(e) -> println("Error: ${e}")
  end
end
