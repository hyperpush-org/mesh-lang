fn first_or_default(xs) do
  case xs do
    head :: _ -> head
    _ -> "empty"
  end
end

fn main() do
  println(first_or_default(["hello", "world"]))
  println(first_or_default([]))
end
