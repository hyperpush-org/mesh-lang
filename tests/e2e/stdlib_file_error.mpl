import File

fn main() do
  let result = File.read("/tmp/mesh_nonexistent_file_999.txt")
  case result do
    Ok(contents) -> println(contents)
    Err(msg) -> println("error")
  end
end
