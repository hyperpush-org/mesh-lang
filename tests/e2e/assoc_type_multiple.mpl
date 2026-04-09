struct Transformer do
  factor :: Int
  label :: String
end

interface Mapper do
  type Input
  type Output
  fn apply(self) -> Self.Output
end

impl Mapper for Transformer do
  type Input = Int
  type Output = String
  fn apply(self) -> String do
    "mapped"
  end
end

fn main() do
  let t = Transformer { factor: 2, label: "test" }
  let result = t.apply()
  println(result)
end
