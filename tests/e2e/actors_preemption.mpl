# Preemptive scheduling test.
# A compute-heavy actor does not starve a message-waiting actor.
# The slow actor calls a function many times (triggering reduction checks).
# The fast actor just receives and prints.

fn busy_loop(n :: Int) -> Int do
  if n <= 0 do
    0
  else
    busy_loop(n - 1)
  end
end

fn slow_work() -> Int do
  busy_loop(500)
end

actor slow_actor() do
  receive do
    msg -> slow_work()
  end
end

actor fast_actor() do
  receive do
    msg -> println("fast done")
  end
end

fn main() do
  let slow = spawn(slow_actor)
  let fast = spawn(fast_actor)
  send(slow, 1)
  send(fast, 1)
  println("slow done")
end
