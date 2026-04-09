# GC bounded memory test.
# A long-running actor receives many messages, each triggering string
# allocations that become unreachable. Without GC, memory would grow
# unboundedly. With mark-sweep GC at yield points, unreachable allocations
# are reclaimed and memory stays bounded.
#
# 50 messages x 200 iterations x ~200 bytes/iter = ~2 MB total allocations.
# GC threshold is 256 KiB, so GC must trigger many times for this to succeed.

fn alloc_work(remaining :: Int, acc :: Int) -> Int do
  if remaining == 0 do
    acc
  else
    let temp = "hello world this is a string that takes up some memory"
    let temp2 = temp <> " and some more data appended to it"
    let len = string_length(temp2)
    alloc_work(remaining - 1, acc + len)
  end
end

actor worker() do
  receive do
    msg -> alloc_work(200, 0)
  end
end

fn send_messages(pid :: Pid, remaining :: Int) -> Int do
  if remaining == 0 do
    0
  else
    send(pid, 1)
    send_messages(pid, remaining - 1)
  end
end

fn main() do
  let pid = spawn(worker)
  send_messages(pid, 50)
  println("gc bounded memory test done")
end
