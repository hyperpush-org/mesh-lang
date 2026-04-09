# Phase 76: User-defined Iterable with built-in runtime iterator

struct EvenNumbers do
  items :: List<Int>
end

impl Iterable for EvenNumbers do
  type Item = Int
  type Iter = ListIterator
  fn iter(self) -> ListIterator do
    Iter.from(self.items)
  end
end

fn make_evens() -> EvenNumbers do
  EvenNumbers { items: [2, 4, 6, 8, 10] }
end

fn main() do
  let evens = make_evens()

  # Test 1: for-in over user-defined Iterable
  let doubled = for x in evens do
    x * 2
  end
  # doubled should be [4, 8, 12, 16, 20]
  println(doubled.to_string())

  # Test 2: simple iteration with side effects
  for x in evens do
    println(x.to_string())
  end
end
