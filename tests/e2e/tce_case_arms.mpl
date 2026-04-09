# TCE case arms test.
# Self-recursive tail calls through case/match arms are correctly eliminated.
# Chain: process(2, 0) -> process(1, 20) -> process(0, 30) -> returns 30.

fn process(cmd :: Int, acc :: Int) -> Int do
  case cmd do
    0 -> acc
    1 -> process(0, acc + 10)
    2 -> process(1, acc + 20)
    _ -> process(0, acc)
  end
end

fn main() do
  let result = process(2, 0)
  println("${result}")
end
