# Accumulator service proving state persistence across calls.
# Adds 1 + 2 + 3 = 6.
# Expected output: 6\n

service Accumulator do
  fn init() -> Int do
    0
  end

  call Add(n :: Int) :: Int do |state|
    (state + n, state + n)
  end
end

fn main() do
  let pid = Accumulator.start()
  let _ = Accumulator.add(pid, 1)
  let _ = Accumulator.add(pid, 2)
  let result = Accumulator.add(pid, 3)
  println("${result}")
end
