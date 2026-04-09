# Test: Result<MultiFieldStruct, String> construct + match roundtrip.
# Verifies the pointer-boxing fix for struct payloads in sum types.
# Previously, constructing Ok(Struct{x,y}) caused a buffer overflow
# because the struct (>8 bytes) was written into the 8-byte ptr slot
# of the {i8, ptr} sum type layout.

struct Pair do
  x :: Int
  y :: Int
end

struct Triple do
  a :: Int
  b :: Int
  c :: Int
end

fn make_ok_pair() -> Pair!String do
  Ok(Pair { x: 42, y: 99 })
end

fn make_err_pair() -> Pair!String do
  Err("error")
end

fn make_ok_triple() -> Triple!String do
  Ok(Triple { a: 10, b: 20, c: 30 })
end

fn extract_pair(result :: Pair!String) -> Int do
  case result do
    Ok(pair) -> pair.x + pair.y
    Err(_) -> -1
  end
end

fn extract_triple(result :: Triple!String) -> Int do
  case result do
    Ok(t) -> t.a + t.b + t.c
    Err(_) -> -1
  end
end

fn main() do
  let r1 = extract_pair(make_ok_pair())
  let r2 = extract_pair(make_err_pair())
  let r3 = extract_triple(make_ok_triple())
  println("${r1}")
  println("${r2}")
  println("${r3}")
end
