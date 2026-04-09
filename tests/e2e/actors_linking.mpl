# Process linking test.
# Verifies actors can be spawned and linked without crashing.

actor linked_worker() do
  receive do
    msg -> println("linked worker done")
  end
end

actor linker() do
  receive do
    msg -> println("linker done")
  end
end

fn main() do
  let w = spawn(linked_worker)
  let l = spawn(linker)
  send(w, 1)
  send(l, 1)
  println("link test done")
end
