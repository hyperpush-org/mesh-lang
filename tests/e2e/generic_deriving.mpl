struct Box<T> do
  value :: T
end deriving(Display, Eq)

fn main() do
  let b1 = Box { value: 42 }
  let b2 = Box { value: 42 }
  let b3 = Box { value: 99 }

  println("${b1}")

  let bs = Box { value: "hello" }
  println("${bs}")

  println("${b1 == b2}")
  println("${b1 == b3}")
end
