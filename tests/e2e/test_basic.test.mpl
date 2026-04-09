test("arithmetic is correct") do
  assert(1 + 1 == 2)
  assert(3 * 4 == 12)
  assert(10 - 3 == 7)
  assert(15 / 3 == 5)
end

test("boolean logic works") do
  assert(true)
  assert(1 == 1)
  assert(5 > 3)
  assert(2 < 10)
end

test("string operations") do
  let s = "hello"
  assert(String.length(s) == 5)
  assert(String.contains(s, "ell"))
  assert(String.starts_with(s, "he"))
  assert(String.ends_with(s, "lo"))
end

test("assert_raises catches failing assertion") do
  assert_raises(fn() do
    assert(false)
  end)
end

test("assert_raises catches nested failure") do
  assert_raises(fn() do
    assert(1 == 2)
  end)
end
