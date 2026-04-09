test("assert_raises catches panics") do
  # Test that assert_raises properly catches a panicking closure.
  # The closure panics via a failing assert inside it.
  assert_raises(fn() do
    assert(false)
  end)
end
