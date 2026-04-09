fn main() do
  let result = IO.read_line()
  case result do
    Ok(line) -> println(line)
    Err(msg) -> println("error")
  end
end
