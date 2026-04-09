struct Point do
  x :: Int
  y :: Int
end deriving(Json)

fn main() do
  let r1 = Point.from_json("not json")
  case r1 do
    Ok(_) -> println("unexpected success")
    Err(e) -> println("parse error: ok")
  end

  let r2 = Point.from_json("{\"x\":1}")
  case r2 do
    Ok(_) -> println("unexpected success")
    Err(e) -> println("missing field: ok")
  end

  let r3 = Point.from_json("{\"x\":\"hello\",\"y\":2}")
  case r3 do
    Ok(_) -> println("unexpected success")
    Err(e) -> println("wrong type: ok")
  end
end
