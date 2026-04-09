# NEGATIVE TEST: This should fail to compile.
# The start function does not call spawn(), so it does not return Pid.
# Expected error: E0018 (InvalidChildStart)

supervisor BadSup do
  strategy: one_for_one
  max_restarts: 3
  max_seconds: 5

  child bad do
    start: fn -> 42 end
    restart: permanent
    shutdown: 5000
  end
end

fn main() do
  let sup = spawn(BadSup)
end
