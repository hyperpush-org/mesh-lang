# Multiple actors receiving messages test.

actor worker() do
  receive do
    msg -> println("worker done")
  end
end

fn main() do
  let w1 = spawn(worker)
  let w2 = spawn(worker)
  let w3 = spawn(worker)
  send(w1, 1)
  send(w2, 2)
  send(w3, 3)
  println("main sent all")
end
