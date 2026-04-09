# Service with multiple call/cast operations E2E test.
# A simple Store service with Get, Set, and Clear operations.
# Expected output: 100\n200\n0\n

service Store do
  fn init(start_val :: Int) -> Int do
    start_val
  end

  call Get() :: Int do |state|
    (state, state)
  end

  call Set(value :: Int) :: Int do |_state|
    (value, value)
  end

  cast Clear() do |_state|
    0
  end
end

fn main() do
  let pid = Store.start(100)
  let v1 = Store.get(pid)
  println("${v1}")
  let v2 = Store.set(pid, 200)
  println("${v2}")
  Store.clear(pid)
  let v3 = Store.get(pid)
  println("${v3}")
end
