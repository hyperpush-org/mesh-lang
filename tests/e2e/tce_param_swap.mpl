# TCE parameter swap test.
# A self-recursive function that swaps parameters produces correct results.
# This proves the two-phase argument evaluation is correct -- if args were stored
# sequentially, the swap would corrupt values before they are read.

fn swap_count(a :: Int, b :: Int, n :: Int) -> Int do
  if n <= 0 do
    println("${a}")
    println("${b}")
    0
  else
    swap_count(b, a, n - 1)
  end
end

fn main() do
  swap_count(1, 2, 100001)
end
