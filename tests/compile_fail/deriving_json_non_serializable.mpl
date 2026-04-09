struct BadStruct do
  name :: String
  worker :: Pid
end deriving(Json)

fn main() do
  println("should not compile")
end
