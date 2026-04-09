# Unit tests for mesh-slug package.
# Run with: meshc test mesh-slug/tests/

from Slug import slugify, truncate, is_valid

describe("Slug.slugify — default separator") do
  test("basic: lowercases and hyphenates") do
    assert_eq(slugify("Hello World!"), "hello-world")
  end
  test("consecutive spaces collapse to one separator") do
    assert_eq(slugify("foo  bar"), "foo-bar")
  end
  test("leading and trailing special chars stripped") do
    assert_eq(slugify("  hello  "), "hello")
  end
  test("empty string returns empty string") do
    assert_eq(slugify(""), "")
  end
  test("all-special input returns empty string") do
    assert_eq(slugify("!!!"), "")
  end
  test("already valid slug unchanged") do
    assert_eq(slugify("hello-world"), "hello-world")
  end
  test("long string with mixed content") do
    assert_eq(slugify("The Quick Brown Fox!"), "the-quick-brown-fox")
  end
end

describe("Slug.slugify — custom separator") do
  test("underscore separator") do
    assert_eq(slugify("Hello World!", "_"), "hello_world")
  end
  test("double-hyphen separator") do
    assert_eq(slugify("Hello World!", "--"), "hello--world")
  end
  test("custom sep with consecutive spaces") do
    assert_eq(slugify("foo  bar", "_"), "foo_bar")
  end
end

describe("Slug.truncate") do
  test("truncates at last separator boundary") do
    assert_eq(truncate("hello-world-foo", 11), "hello-world")
  end
  test("no truncation when within max") do
    assert_eq(truncate("hello-world-foo", 15), "hello-world-foo")
  end
  test("truncates to first word when max is small") do
    assert_eq(truncate("hello-world-foo", 5), "hello")
  end
  test("empty string returns empty string") do
    assert_eq(truncate("", 10), "")
  end
  test("single word shorter than max") do
    assert_eq(truncate("hello", 10), "hello")
  end
end

describe("Slug.is_valid") do
  test("valid hyphenated slug returns true") do
    assert(is_valid("hello-world"))
  end
  test("single word returns true") do
    assert(is_valid("hello"))
  end
  test("digits allowed") do
    assert(is_valid("hello123"))
  end
  test("all digits allowed") do
    assert(is_valid("123"))
  end
  test("uppercase letters return false") do
    assert(is_valid("Hello-World") == false)
  end
  test("spaces return false") do
    assert(is_valid("hello world") == false)
  end
  test("leading hyphen returns false") do
    assert(is_valid("-hello") == false)
  end
  test("trailing hyphen returns false") do
    assert(is_valid("hello-") == false)
  end
  test("consecutive hyphens return false") do
    assert(is_valid("hello--world") == false)
  end
  test("empty string returns false") do
    assert(is_valid("") == false)
  end
  test("special chars return false") do
    assert(is_valid("hello!world") == false)
  end
end
