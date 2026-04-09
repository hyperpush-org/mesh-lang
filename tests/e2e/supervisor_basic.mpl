# Basic supervisor test.
# Verifies: supervisor block compiles and runs with the runtime.

actor worker() do
  receive do
    msg -> println("worker got message")
  end
end

supervisor WorkerSup do
  strategy: one_for_one
  max_restarts: 3
  max_seconds: 5

  child w1 do
    start: fn -> spawn(worker) end
    restart: permanent
    shutdown: 5000
  end
end

fn main() do
  let sup = spawn(WorkerSup)
  println("supervisor started")
end
