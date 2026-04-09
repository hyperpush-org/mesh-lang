fn main() do
  let missing = Env.get("MESH_TEST_MISSING_VAR_99999", "default_val")
  println(missing)
  let present = Env.get("MESH_TEST_VAR", "default_val")
  println(present)
  let empty = Env.get("MESH_TEST_EMPTY_VAR", "default_val")
  println(empty)
end
