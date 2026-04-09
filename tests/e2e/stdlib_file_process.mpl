import File
import String

fn main() do
  let path = "/tmp/mesh_test_process_in.txt"
  let wr = File.write(path, "hello world")
  let read_result = File.read(path)
  let dr = File.delete(path)
  case read_result do
    Ok(contents) -> println(String.to_upper(contents))
    Err(msg) -> println(msg)
  end
end
