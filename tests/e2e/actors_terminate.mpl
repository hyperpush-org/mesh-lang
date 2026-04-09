# Terminate callback test.
# An actor with a terminate clause runs cleanup before exiting.

actor cleanup_actor() do
  receive do
    msg -> println("actor processing")
  end
terminate do
  println("cleanup executed")
end
end

fn main() do
  let pid = spawn(cleanup_actor)
  send(pid, 1)
  println("terminate test done")
end
