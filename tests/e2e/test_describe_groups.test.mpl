describe("Math operations") do
  test("addition") do
    assert(10 + 20 == 30)
    assert(1 + 1 == 2)
  end

  test("multiplication") do
    assert(10 * 20 == 200)
    assert(3 * 3 == 9)
  end

  test("subtraction") do
    assert(100 - 55 == 45)
  end
end

describe("String module") do
  test("length") do
    assert(String.length("hello") == 5)
    assert(String.length("") == 0)
  end

  test("contains") do
    assert(String.contains("foobar", "foo"))
    assert(String.contains("foobar", "bar"))
  end
end

test("top-level test also runs") do
  assert(true)
  assert(1 == 1)
end
