# Smoke test for trait method codegen pipeline (Phase 19).
# 
# STATUS: MIR lowering works correctly (108 tests pass).
# Full compilation blocked by typeck type identity issue:
#   "expected Point, found Point" -- self parameter type in impl method
#   is considered a different Point than the struct literal's Point.
#  This is a typeck gap (not codegen) to resolve in gap closure.
# 
# When this compiles and runs, it should print: a point

struct Point do
  x :: Int
  y :: Int
end

interface Describable do
  fn describe(self) -> String
end

impl Describable for Point do
  fn describe(self) -> String do
    "a point"
  end
end

fn main() do
  let p = Point { x: 1, y: 2 }
  println(describe(p))
end
