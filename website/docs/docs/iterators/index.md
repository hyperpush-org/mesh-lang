---
title: Iterators
description: Lazy iterator pipelines in Mesh with Iter.from, map, filter, reduce, collect, and custom Iterable types.
---

# Iterators

Mesh provides a lazy iterator protocol for composing data transformations as pipelines. Instead of creating intermediate lists at each step, iterators process elements one at a time -- no extra allocations until you collect the final result. Combined with the pipe operator `|>`, iterator pipelines read naturally from left to right.

## Creating Iterators

Use `Iter.from()` to create an iterator from any collection:

```mesh
fn main() do
  let list = [1, 2, 3, 4, 5]
  let iter = Iter.from(list)

  # Count elements to consume the iterator
  let n = Iter.from(list) |> Iter.count()
  println(n.to_string())
end
```

`Iter.from()` works with lists, maps, and sets. The returned iterator is lazy -- it does nothing until you consume it with a terminal operation or collect.

### Custom Iterables

You can make your own types iterable by implementing the `Iterable` interface. This lets your type work with `for...in` loops:

```mesh
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

  # for-in over user-defined Iterable
  let doubled = for x in evens do
    x * 2
  end
  println(doubled.to_string())

  # Iteration with side effects
  for x in evens do
    println(x.to_string())
  end
end
```

The `Iterable` interface requires two associated types (`Item` and `Iter`) and an `iter` method that returns an iterator handle.

## Lazy Combinators

Combinators transform an iterator into a new iterator without consuming it. Because they are lazy, no work happens until a terminal operation or collect drives the pipeline. You can chain as many combinators as you need.

### map

`Iter.map` transforms each element by applying a function:

```mesh
fn main() do
  let list = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

  # Double each element, then sum
  let sum = Iter.from(list) |> Iter.map(fn x -> x * 3 end) |> Iter.sum()
  println(sum.to_string())
end
```

### filter

`Iter.filter` keeps only elements that satisfy a predicate:

```mesh
fn main() do
  let list = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

  # Count even numbers
  let even_count = Iter.from(list) |> Iter.filter(fn x -> x % 2 == 0 end) |> Iter.count()
  println(even_count.to_string())
end
```

`map` and `filter` compose naturally. Chain them to build multi-step transformations:

```mesh
fn main() do
  let list = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

  # Double each element, then keep only those greater than 10
  let big = Iter.from(list) |> Iter.map(fn x -> x * 2 end) |> Iter.filter(fn x -> x > 10 end) |> Iter.count()
  println(big.to_string())
end
```

### take and skip

`Iter.take` limits an iterator to the first N elements. `Iter.skip` discards the first N elements and yields the rest:

```mesh
fn main() do
  let list = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

  # Sum of first 3 elements: 1 + 2 + 3 = 6
  let first3 = Iter.from(list) |> Iter.take(3) |> Iter.sum()
  println(first3.to_string())

  # Skip first 7, sum remaining: 8 + 9 + 10 = 27
  let last3 = Iter.from(list) |> Iter.skip(7) |> Iter.sum()
  println(last3.to_string())
end
```

`take` is especially useful for short-circuiting -- once it has yielded N elements, the pipeline stops processing. Combined with `skip`, you can create sliding windows over data:

```mesh
fn main() do
  let list = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

  # Window: skip first 2, then take 5
  let window = Iter.from(list) |> Iter.skip(2) |> Iter.take(5) |> Iter.count()
  println(window.to_string())
end
```

### enumerate

`Iter.enumerate` pairs each element with its zero-based index, producing `{index, value}` tuples:

```mesh
fn main() do
  let list = [10, 20, 30]

  # Enumerate produces 3 pairs: {0, 10}, {1, 20}, {2, 30}
  let n = Iter.from(list) |> Iter.enumerate() |> Iter.count()
  println(n.to_string())
end
```

Enumerated iterators are commonly used with `Map.collect` to build index-keyed maps from lists (see [Collecting Results](#collecting-results)).

### zip

`Iter.zip` combines two iterators element-by-element into pairs. The resulting iterator stops when the shorter input is exhausted:

```mesh
fn main() do
  let a = [1, 2, 3]
  let b = [4, 5, 6]
  let pairs = Iter.from(a) |> Iter.zip(Iter.from(b)) |> Iter.count()
  println(pairs.to_string())

  # Unequal lengths: shorter determines count
  let short = [1, 2]
  let long = [10, 20, 30, 40]
  let zipped = Iter.from(short) |> Iter.zip(Iter.from(long)) |> Iter.count()
  println(zipped.to_string())
end
```

## Terminal Operations

Terminal operations consume an iterator and produce a single value. Once a terminal runs, the iterator is exhausted.

### count

`Iter.count` returns the number of elements in the iterator:

```mesh
fn main() do
  let list = [1, 2, 3, 4, 5]
  let c = Iter.from(list) |> Iter.count()
  println(c.to_string())
end
```

### sum

`Iter.sum` adds all integer elements together:

```mesh
fn main() do
  let list = [1, 2, 3, 4, 5]
  let s = Iter.from(list) |> Iter.sum()
  println(s.to_string())
end
```

### any and all

`Iter.any` returns `true` if any element satisfies the predicate. `Iter.all` returns `true` only if every element satisfies it:

```mesh
fn main() do
  let list = [1, 2, 3, 4, 5]

  # any: is there an even number?
  let has_even = Iter.from(list) |> Iter.any(fn x -> x % 2 == 0 end)
  println(has_even.to_string())

  # all: are all elements positive?
  let all_pos = Iter.from(list) |> Iter.all(fn x -> x > 0 end)
  println(all_pos.to_string())

  # all: are all elements even? (false)
  let all_even = Iter.from(list) |> Iter.all(fn x -> x % 2 == 0 end)
  println(all_even.to_string())
end
```

Both `any` and `all` short-circuit -- `any` stops as soon as it finds a match, and `all` stops as soon as it finds a non-match.

### find

`Iter.find` returns the first element that satisfies the predicate:

```mesh
fn main() do
  let list = [1, 2, 3, 4, 5]
  let found = Iter.from(list) |> Iter.find(fn x -> x > 3 end)
  println(found.to_string())
end
```

Like `any`, `find` short-circuits as soon as a matching element is found.

### reduce

`Iter.reduce` folds all elements into a single value using an accumulator and a combining function:

```mesh
fn main() do
  let list = [1, 2, 3, 4, 5]

  # Product: 1 * 2 * 3 * 4 * 5 = 120
  let product = Iter.from(list) |> Iter.reduce(1, fn acc, x -> acc * x end)
  println(product.to_string())

  # Sum via reduce: 0 + 1 + 2 + 3 + 4 + 5 = 15
  let sum = Iter.from(list) |> Iter.reduce(0, fn acc, x -> acc + x end)
  println(sum.to_string())
end
```

The first argument to `reduce` is the initial accumulator value. The function receives the current accumulator and the next element, and returns the new accumulator.

## Collecting Results

Lazy pipelines produce iterators, not collections. To materialize the result into a concrete data structure, use a collect function at the end of the pipeline.

### List.collect

`List.collect` gathers all elements from an iterator into a list:

```mesh
fn main() do
  let list = [1, 2, 3]

  # Map and collect into a new list
  let doubled = Iter.from(list) |> Iter.map(fn x -> x * 2 end) |> List.collect()
  println("${doubled}")

  # Filter and collect
  let big = Iter.from([1, 2, 3, 4, 5]) |> Iter.filter(fn x -> x > 3 end) |> List.collect()
  println("${big}")
end
```

### Map.collect

`Map.collect` builds a map from an iterator of key-value pairs. Use `Iter.enumerate` to pair elements with indices, or `Iter.zip` to combine separate key and value iterators:

```mesh
fn main() do
  # Enumerate: indices become keys
  let list = [100, 200, 300]
  let m = Iter.from(list) |> Iter.enumerate() |> Map.collect()
  println("${m}")

  # Zip: combine key and value lists
  let keys = [10, 20, 30]
  let vals = [1, 2, 3]
  let m2 = Iter.from(keys) |> Iter.zip(Iter.from(vals)) |> Map.collect()
  println("${m2}")
end
```

### Set.collect

`Set.collect` gathers elements into a set, automatically removing duplicates:

```mesh
fn main() do
  let list = [1, 2, 2, 3, 3, 3]
  let s = Iter.from(list) |> Set.collect()
  println("${Set.size(s)}")

  # Pipeline into set
  let s2 = Iter.from([1, 2, 3, 4, 5]) |> Iter.filter(fn x -> x > 2 end) |> Set.collect()
  println("${Set.size(s2)}")
end
```

### String.collect

`String.collect` concatenates all string elements from an iterator into a single string:

```mesh
fn main() do
  let words = ["hello", " ", "world"]
  let joined = Iter.from(words) |> String.collect()
  println(joined)

  let abc = Iter.from(["a", "b", "c"]) |> String.collect()
  println(abc)
end
```

## Building Pipelines

The real power of iterators comes from composing multiple combinators into a single pipeline. Each step is lazy -- elements flow through the pipeline one at a time, and short-circuiting combinators like `take` stop processing early.

```mesh
fn main() do
  let list = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

  # Multi-step pipeline: double, keep values > 10, take first 3, count
  let result = Iter.from(list) |> Iter.map(fn x -> x * 2 end) |> Iter.filter(fn x -> x > 10 end) |> Iter.take(3) |> Iter.count()
  println(result.to_string())

  # Filter, transform, and sum
  let result2 = Iter.from(list) |> Iter.filter(fn x -> x > 5 end) |> Iter.map(fn x -> x * 10 end) |> Iter.sum()
  println(result2.to_string())

  # Closures capture variables from the surrounding scope
  let threshold = 3
  let above = Iter.from(list) |> Iter.filter(fn x -> x > threshold end) |> Iter.count()
  println(above.to_string())
end
```

In the first pipeline, `take(3)` ensures only three elements pass through even though the source list has ten. The `map` and `filter` steps before it only run as many times as needed -- no wasted computation.

Pipelines that end with a collect operation produce a concrete collection:

```mesh
fn main() do
  let list = [1, 2, 3]

  # Transform and materialize as a list
  let doubled = Iter.from(list) |> Iter.map(fn x -> x * 2 end) |> List.collect()
  println("${doubled}")
end
```

## Next Steps

- [Type System](/docs/type-system/) -- interfaces, associated types, and traits that power the iterator protocol
- [Syntax Cheatsheet](/docs/cheatsheet/) -- quick reference for all Mesh syntax
