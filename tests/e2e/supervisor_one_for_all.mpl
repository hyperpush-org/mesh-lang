# Test: one_for_all strategy.
# Verifies: supervisor with one_for_all compiles and runs.

actor worker() do
  receive do
    msg -> println("worker alive")
  end
end

supervisor AllSup do
  strategy: one_for_all
  max_restarts: 3
  max_seconds: 5

  child w1 do
    start: fn -> spawn(worker) end
    restart: permanent
    shutdown: 5000
  end

  child w2 do
    start: fn -> spawn(worker) end
    restart: permanent
    shutdown: 5000
  end
end

fn main() do
  let sup = spawn(AllSup)
  println("one_for_all supervisor started")
end
