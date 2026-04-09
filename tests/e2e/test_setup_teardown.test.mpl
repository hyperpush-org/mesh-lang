describe("Math with setup") do
  setup do
    assert(true)
  end

  teardown do
    assert(true)
  end

  test("basic addition") do
    assert(1 + 1 == 2)
    assert(10 + 20 == 30)
  end

  test("multiplication") do
    assert(3 * 4 == 12)
    assert(0 * 999 == 0)
  end
end

describe("String ops with setup and teardown") do
  setup do
    assert(true)
  end

  teardown do
    assert(true)
  end

  test("string length") do
    assert(String.length("hello") == 5)
    assert(String.length("") == 0)
  end

  test("string contains") do
    assert(String.contains("hello world", "world"))
    assert(String.contains("foobar", "foo"))
  end
end

test("top-level test with no describe") do
  assert(1 + 1 == 2)
  assert_eq("foo", "foo")
end
