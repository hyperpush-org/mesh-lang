import File

fn main() do
  let path = "/tmp/mesh_test_write_read.txt"
  let wr = File.write(path, "Hello, Mesh!")
  let read_result = File.read(path)
  let dr = File.delete(path)
  case read_result do
    Ok(contents) -> println(contents)
    Err(msg) -> println(msg)
  end
end
