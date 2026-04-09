# Phase 75: User-defined arithmetic operators with Output associated type

struct Vec2 do
  x :: Float
  y :: Float
end

impl Add for Vec2 do
  type Output = Vec2
  fn add(self, other :: Vec2) -> Vec2 do
    Vec2 { x: self.x + other.x, y: self.y + other.y }
  end
end

impl Sub for Vec2 do
  type Output = Vec2
  fn sub(self, other :: Vec2) -> Vec2 do
    Vec2 { x: self.x - other.x, y: self.y - other.y }
  end
end

impl Mul for Vec2 do
  type Output = Vec2
  fn mul(self, other :: Vec2) -> Vec2 do
    Vec2 { x: self.x * other.x, y: self.y * other.y }
  end
end

fn main() do
  let v1 = Vec2 { x: 1.0, y: 2.0 }
  let v2 = Vec2 { x: 3.0, y: 4.0 }

  # Test Add
  let sum = v1 + v2
  println("${sum.x}")
  println("${sum.y}")

  # Test Sub
  let diff = v1 - v2
  println("${diff.x}")
  println("${diff.y}")

  # Test Mul
  let prod = v1 * v2
  println("${prod.x}")
  println("${prod.y}")

  # Backward compat: primitive arithmetic still works
  let a = 1 + 2
  println("${a}")
  let b = 3.0 * 4.0
  println("${b}")

  # Operator chaining: v1 + v2 + v3 (Output = Vec2 feeds back into Add)
  let v3 = Vec2 { x: 10.0, y: 20.0 }
  let chained = v1 + v2 + v3
  println("${chained.x}")
  println("${chained.y}")
end
