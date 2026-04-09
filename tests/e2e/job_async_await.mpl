# Job async/await E2E test.
# Verifies: Job.async spawns work, Job.await collects Result.
# Expected output: 42\n

fn main() do
  let job = Job.async(fn() -> 42 end)
  let result = Job.await(job)
  case result do
    Ok(val) -> println("${val}")
    Err(msg) -> println(msg)
  end
end
