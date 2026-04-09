describe("math operations") do
  test("addition") do
    assert(1 + 1 == 2)
    assert(100 + 200 == 300)
  end

  test("multiplication") do
    assert(3 * 3 == 9)
    assert(0 * 999 == 0)
  end
end

describe("string module") do
  test("length") do
    assert(String.length("") == 0)
    assert(String.length("abc") == 3)
    assert(String.length("hello world") == 11)
  end

  test("contains") do
    assert(String.contains("foobar", "oba"))
    assert(String.contains("foobar", "foo"))
    assert(String.contains("foobar", "bar"))
  end
end
