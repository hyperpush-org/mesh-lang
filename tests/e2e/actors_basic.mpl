# Basic actor spawning and messaging test.
# Verifies: spawn, send, receive, actor exits after processing.

actor greeter() do
  receive do
    msg -> println("actor received")
  end
end

fn main() do
  let pid = spawn(greeter)
  send(pid, 1)
  println("main done")
end
