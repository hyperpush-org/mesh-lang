#!/usr/bin/env bash

clustered_fixture_repo_root() {
  if [[ -n "${ROOT_DIR:-}" ]]; then
    printf '%s\n' "$ROOT_DIR"
  else
    cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd
  fi
}

CLUSTERED_FIXTURE_REPO_ROOT="$(clustered_fixture_repo_root)"

TINY_CLUSTER_FIXTURE_ROOT_RELATIVE="scripts/fixtures/clustered/tiny-cluster"
TINY_CLUSTER_FIXTURE_ROOT="$CLUSTERED_FIXTURE_REPO_ROOT/$TINY_CLUSTER_FIXTURE_ROOT_RELATIVE"
TINY_CLUSTER_FIXTURE_TESTS="$TINY_CLUSTER_FIXTURE_ROOT/tests"
TINY_CLUSTER_FIXTURE_BINARY="$TINY_CLUSTER_FIXTURE_ROOT/tiny-cluster"

CLUSTER_PROOF_FIXTURE_ROOT_RELATIVE="scripts/fixtures/clustered/cluster-proof"
CLUSTER_PROOF_FIXTURE_ROOT="$CLUSTERED_FIXTURE_REPO_ROOT/$CLUSTER_PROOF_FIXTURE_ROOT_RELATIVE"
CLUSTER_PROOF_FIXTURE_TESTS="$CLUSTER_PROOF_FIXTURE_ROOT/tests"
CLUSTER_PROOF_FIXTURE_BINARY="$CLUSTER_PROOF_FIXTURE_ROOT/cluster-proof"

clustered_fixture_require_package_root() {
  local package_name="$1"
  local root="$2"
  local relative_root="$3"
  shift 3

  if [[ ! -d "$root" ]]; then
    echo "clustered fixture drift: missing ${package_name} fixture root at ${root} (expected ${relative_root})" >&2
    return 1
  fi

  local manifest_path="$root/mesh.toml"
  if [[ ! -f "$manifest_path" ]]; then
    echo "clustered fixture drift: missing ${package_name} fixture manifest at ${manifest_path}" >&2
    return 1
  fi

  if ! grep -Fq "name = \"${package_name}\"" "$manifest_path"; then
    echo "clustered fixture drift: ${manifest_path} is not the ${package_name} package manifest" >&2
    return 1
  fi

  local missing=()
  local relative_path
  for relative_path in "$@"; do
    if [[ ! -f "$root/$relative_path" ]]; then
      missing+=("$relative_path")
    fi
  done

  if (( ${#missing[@]} > 0 )); then
    echo "clustered fixture drift: ${package_name} fixture root ${root} is missing required files: ${missing[*]}" >&2
    return 1
  fi
}

clustered_fixture_require_tiny_cluster_root() {
  clustered_fixture_require_package_root \
    "tiny-cluster" \
    "$TINY_CLUSTER_FIXTURE_ROOT" \
    "$TINY_CLUSTER_FIXTURE_ROOT_RELATIVE" \
    "mesh.toml" \
    "main.mpl" \
    "work.mpl" \
    "README.md" \
    "tests/work.test.mpl"
}

clustered_fixture_require_cluster_proof_root() {
  clustered_fixture_require_package_root \
    "cluster-proof" \
    "$CLUSTER_PROOF_FIXTURE_ROOT" \
    "$CLUSTER_PROOF_FIXTURE_ROOT_RELATIVE" \
    "mesh.toml" \
    "main.mpl" \
    "work.mpl" \
    "README.md" \
    "Dockerfile" \
    "fly.toml" \
    "tests/work.test.mpl"
}

clustered_fixture_require_all() {
  clustered_fixture_require_tiny_cluster_root
  clustered_fixture_require_cluster_proof_root
}
