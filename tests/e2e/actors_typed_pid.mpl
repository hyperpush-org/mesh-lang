# Typed Pid test.
# Spawn returns Pid<Int> and send accepts the correct message type.

actor int_receiver() do
  receive do
    n -> println("typed pid ok")
  end
end

fn main() do
  let pid = spawn(int_receiver)
  send(pid, 42)
  println("typed pid sent")
end
