fn main() do
  let a = String.to_int("42")
  let b = String.to_int("not a number")
  let c = String.to_float("3.14")
  let d = String.to_float("bad")
  case a do
    Some(n1) -> println("${n1}")
    None -> println("none")
  end
  case b do
    Some(n2) -> println("${n2}")
    None -> println("none")
  end
  case c do
    Some(f1) -> println("${f1}")
    None -> println("none")
  end
  case d do
    Some(f2) -> println("${f2}")
    None -> println("none")
  end
  let e = String.to_int("-100")
  case e do
    Some(n3) -> println("${n3}")
    None -> println("none")
  end
end
