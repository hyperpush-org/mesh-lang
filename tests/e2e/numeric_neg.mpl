# Phase 75: User-defined Neg trait for unary minus

struct Point do
  x :: Float
  y :: Float
end

impl Neg for Point do
  type Output = Point
  fn neg(self) -> Point do
    Point { x: 0.0 - self.x, y: 0.0 - self.y }
  end
end

fn main() do
  let p = Point { x: 3.0, y: 7.0 }

  # Test Neg on user type
  let neg_p = -p
  println("${neg_p.x}")
  println("${neg_p.y}")

  # Primitive neg still works
  let a = -42
  println("${a}")
  let b = -3.5
  println("${b}")
end
