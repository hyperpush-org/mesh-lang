# Test: actors spawned with arguments receive correct values.
# Verifies that the wrapper function correctly deserializes args from the
# raw pointer buffer before calling the actor body.

fn add_and_print(a :: Int, b :: Int) -> Int do
  println("${a + b}")
  0
end

# Single-argument actor: receives initial value, adds to message, prints result.
actor adder(initial :: Int) do
  receive do
    msg -> add_and_print(initial, msg)
  end
end

# Multi-argument actor: computes a + b immediately, prints result.
actor calculator(a :: Int, b :: Int) do
  println("${a + b}")
end

fn busy_wait(n :: Int) -> Int do
  if n <= 0 do
    0
  else
    busy_wait(n - 1)
  end
end

fn main() do
  # Multi-arg actor: 30 + 12 = 42
  spawn(calculator, 30, 12)

  # Single-arg actor: initial=10, send 5, expect 15
  let pid = spawn(adder, 10)
  send(pid, 5)

  # Give actors time to process (busy wait triggers preemption/scheduling)
  busy_wait(10000)
  println("args test done")
end
