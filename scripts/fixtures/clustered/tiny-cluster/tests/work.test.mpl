import File
from Work import add

fn read_required_file(repo_relative :: String, package_relative :: String, tests_relative :: String) -> String do
  case File.read(repo_relative) do
    Ok(contents) -> contents
    Err( _) -> do
      case File.read(package_relative) do
        Ok(contents) -> contents
        Err( _) -> do
          case File.read(tests_relative) do
            Ok(contents) -> contents
            Err(message) -> do
              assert(false)
              message
            end
          end
        end
      end
    end
  end
end

fn assert_contains(haystack :: String, needle :: String) do
  assert(String.contains(haystack, needle))
end

fn assert_not_contains(haystack :: String, needle :: String) do
  assert(String.contains(haystack, needle) == false)
end

describe("tiny-cluster package smoke") do
  test("declared work stays trivial under the source-first contract") do
    assert(add() == 2)
  end

  test("manifest and source stay source-first and route-free") do
    let manifest = read_required_file("scripts/fixtures/clustered/tiny-cluster/mesh.toml", "mesh.toml", "../mesh.toml")
    let main_source = read_required_file("scripts/fixtures/clustered/tiny-cluster/main.mpl", "main.mpl", "../main.mpl")
    let work_source = read_required_file("scripts/fixtures/clustered/tiny-cluster/work.mpl", "work.mpl", "../work.mpl")

    assert_not_contains(manifest, "[cluster]")
    assert_not_contains(manifest, "declarations")
    assert_contains(work_source, "@cluster pub fn add()")
    assert_contains(work_source, "1 + 1")
    assert_not_contains(work_source, "declared_work_runtime_name")
    assert_not_contains(work_source, "clustered(work)")
    assert_not_contains(work_source, "Env.get_int")
    assert_not_contains(work_source, "Timer.sleep")
    assert_not_contains(work_source, "TINY_CLUSTER_WORK_DELAY_MS")
    assert_not_contains(work_source, "MESH_STARTUP_WORK_DELAY_MS")
    assert_not_contains(main_source, "HTTP.serve")
    assert_not_contains(main_source, "/work")
    assert_not_contains(main_source, "/status")
    assert_not_contains(main_source, "/health")
    assert_not_contains(work_source, "HTTP.serve")
    assert_not_contains(work_source, "/work")
    assert_not_contains(work_source, "/status")
    assert_not_contains(work_source, "/health")
    assert_not_contains(main_source, "Continuity.submit_declared_work")
    assert_not_contains(work_source, "Continuity.submit_declared_work")
    assert_not_contains(main_source, "Continuity.mark_completed")
    assert_not_contains(work_source, "Continuity.mark_completed")
    assert_contains(main_source, "Node.start_from_env()")
  end

  test("readme points operators to runtime-owned cli inspection") do
    let readme = read_required_file("scripts/fixtures/clustered/tiny-cluster/README.md", "README.md", "../README.md")

    assert_contains(readme, "@cluster pub fn add()")
    assert_contains(readme, "Work.add")
    assert_contains(readme, "meshc cluster status")
    assert_contains(readme, "meshc cluster continuity")
    assert_contains(readme, "meshc cluster diagnostics")
    assert_contains(readme, "Node.start_from_env()")
    assert_contains(readme, "runtime-owned")
    assert_contains(readme, "scripts/fixtures/clustered/tiny-cluster")
    assert_contains(readme, "cargo run -q -p meshc -- build scripts/fixtures/clustered/tiny-cluster")
    assert_contains(readme, "cargo run -q -p meshc -- test scripts/fixtures/clustered/tiny-cluster/tests")
    assert_contains(readme, "automatically starts the source-declared `@cluster` function")
    assert_not_contains(readme, "declared_work_runtime_name()")
    assert_not_contains(readme, "clustered(work)")
    assert_not_contains(readme, "[cluster]")
    assert_not_contains(readme, "TINY_CLUSTER_WORK_DELAY_MS")
    assert_not_contains(readme, "/work")
    assert_not_contains(readme, "/status")
    assert_not_contains(readme, "/health")
  end
end
