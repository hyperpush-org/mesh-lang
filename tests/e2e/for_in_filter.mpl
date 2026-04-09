# For-in with when filter clause: range, list, map, set, empty, break, continue
fn main() do
  # 1. Range filter: even numbers from 0..10
  let evens = for i in 0..10 when i % 2 == 0 do
    i
  end
  for e in evens do
    println("${e}")
  end

  println("---")

  # 2. List filter: elements > 2, multiplied by 10
  let filtered = for x in [1, 2, 3, 4, 5] when x > 2 do
    x * 10
  end
  for f in filtered do
    println("${f}")
  end

  println("---")

  # 3. Map filter with destructuring: keep entries with value > 10
  let m = Map.new()
  let m = Map.put(m, 1, 5)
  let m = Map.put(m, 2, 15)
  let m = Map.put(m, 3, 25)
  let keys = for {k, v} in m when v > 10 do
    k
  end
  let klen = List.length(keys)
  println("${klen}")

  println("---")

  # 4. Set filter: keep elements > 15
  let s = Set.new()
  let s = Set.add(s, 10)
  let s = Set.add(s, 20)
  let s = Set.add(s, 30)
  let big = for x in s when x > 15 do
    x
  end
  let slen = List.length(big)
  println("${slen}")

  println("---")

  # 5. All-false filter: produces empty list
  let empty = for x in [1, 2, 3] when x > 100 do
    x
  end
  let elen = List.length(empty)
  println("${elen}")

  println("---")

  # 6. Break inside filtered loop: partial result from odd numbers
  let partial = for x in [1, 2, 3, 4, 5] when x % 2 == 1 do
    if x == 3 do
      break
    end
    x
  end
  let plen = List.length(partial)
  println("${plen}")

  println("---")

  # 7. Continue inside filtered loop: skip element 3 from elements > 1
  let skipped = for x in [1, 2, 3, 4, 5] when x > 1 do
    if x == 3 do
      continue
    end
    x
  end
  for sk in skipped do
    println("${sk}")
  end

  println("done")
end
