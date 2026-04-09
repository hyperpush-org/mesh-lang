import File

fn check(path :: String) do
  let e = File.exists(path)
  if e do
    println("true")
  else
    println("false")
  end
end

fn main() do
  let path = "/tmp/mesh_test_exists.txt"
  check(path)
  let wr = File.write(path, "test")
  check(path)
  let dr = File.delete(path)
  println("")
end
