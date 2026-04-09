# TCE countdown test.
# A self-recursive function counting down from 1,000,000 completes without stack overflow.
# Without tail-call elimination, this would overflow the stack.

fn countdown(n :: Int) -> Int do
  if n <= 0 do
    0
  else
    countdown(n - 1)
  end
end

fn main() do
  countdown(1000000)
  println("done")
end
