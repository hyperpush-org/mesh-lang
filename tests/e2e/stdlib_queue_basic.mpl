fn main() do
  let q = Queue.new()
  let q = Queue.push(q, 10)
  let q = Queue.push(q, 20)
  let sz = Queue.size(q)
  println("${sz}")
  let pk = Queue.peek(q)
  println("${pk}")
end
