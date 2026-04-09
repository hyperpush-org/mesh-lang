# Counter service E2E test.
# Verifies: service definition, start, call (sync), cast (async).
# Expected output: 10\n15\n0\n

service Counter do
  fn init(start_val :: Int) -> Int do
    start_val
  end

  call GetCount() :: Int do |count|
    (count, count)
  end

  call Increment(amount :: Int) :: Int do |count|
    (count + amount, count + amount)
  end

  cast Reset() do |_count|
    0
  end
end

fn main() do
  let pid = Counter.start(10)
  let c1 = Counter.get_count(pid)
  println("${c1}")
  let c2 = Counter.increment(pid, 5)
  println("${c2}")
  Counter.reset(pid)
  let c3 = Counter.get_count(pid)
  println("${c3}")
end
