struct Item do
  name :: String
  count :: Int
end deriving(Row)

fn main() do
  let row = Map.new()
  let row = Map.put(row, "name", "Widget")

  let result = Item.from_row(row)
  case result do
    Ok(_) -> println("unexpected ok")
    Err(e) -> println(e)
  end
end
