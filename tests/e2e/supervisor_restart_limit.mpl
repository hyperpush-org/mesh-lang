# Test: supervisor restart limits.
# Verifies: supervisor with restart limits compiles and runs.
# The supervisor should start and manage restart counting.

actor crasher() do
  receive do
    msg -> println("crasher running")
  end
end

supervisor LimitSup do
  strategy: one_for_one
  max_restarts: 2
  max_seconds: 5

  child c1 do
    start: fn -> spawn(crasher) end
    restart: permanent
    shutdown: 5000
  end
end

fn main() do
  let sup = spawn(LimitSup)
  println("restart limit test started")
end
