fn main() do
  let missing = Env.get_int("MESH_INT_MISSING_VAR_99999", 8080)
  println("${missing}")
  let valid = Env.get_int("MESH_INT_TEST_VAR", 8080)
  println("${valid}")
  let bad = Env.get_int("MESH_INT_BAD_VAR", 8080)
  println("${bad}")
  let neg = Env.get_int("MESH_INT_NEG_VAR", 8080)
  println("${neg}")
end
