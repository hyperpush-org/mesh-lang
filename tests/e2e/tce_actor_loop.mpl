# TCE actor loop test.
# A tail-recursive function called from an actor context runs 1M iterations
# without stack overflow. This proves TCE is compatible with the actor runtime.
# Without TCE, count_loop(0, 1000000) would overflow the stack.

fn count_loop(n :: Int, target :: Int) -> Int do
  if n >= target do
    n
  else
    count_loop(n + 1, target)
  end
end

actor worker() do
  receive do
    msg -> println("${count_loop(0, 1000000)}")
  end
end

fn main() do
  let pid = spawn(worker)
  send(pid, 1)
end
