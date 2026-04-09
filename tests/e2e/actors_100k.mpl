# 100K actor benchmark.
# Spawns 100,000 actors that each receive a message and exit.
# Uses batched spawning to avoid stack overflow from deep recursion.

actor tiny() do
  receive do
    msg -> 0
  end
end

fn spawn_batch(n :: Int) -> Int do
  if n <= 0 do
    0
  else
    let pid = spawn(tiny)
    send(pid, 1)
    spawn_batch(n - 1)
  end
end

fn spawn_100k(batches :: Int) -> Int do
  if batches <= 0 do
    0
  else
    spawn_batch(1000)
    spawn_100k(batches - 1)
  end
end

fn main() do
  spawn_100k(100)
  println("100000 actors done")
end
