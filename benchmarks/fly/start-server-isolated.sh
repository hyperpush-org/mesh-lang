#!/usr/bin/env bash
set -e

REPO_ROOT="/app"

# Validate LANG env var
if [ -z "$LANG" ]; then
  echo "ERROR: LANG env var is required (one of: Mesh, Go, Rust, Elixir)" >&2
  exit 1
fi

case "$LANG" in
  Mesh|Go|Rust|Elixir)
    ;;
  *)
    echo "ERROR: Unrecognised LANG='$LANG'. Must be one of: Mesh, Go, Rust, Elixir" >&2
    exit 1
    ;;
esac

SERVER_PID=""

cleanup() {
  echo "Stopping $LANG server..."
  if [ -n "$SERVER_PID" ]; then
    kill "$SERVER_PID" 2>/dev/null || true
  fi
}
trap cleanup EXIT INT TERM

wait_for_server() {
  local port=$1
  local name=$2
  local max_wait=60
  local elapsed=0
  echo "Waiting for $name on port $port..."
  until curl -sf "http://localhost:$port/text" > /dev/null 2>&1; do
    sleep 1
    elapsed=$((elapsed + 1))
    if [ $elapsed -ge $max_wait ]; then
      echo "ERROR: $name on port $port failed to start within ${max_wait}s" >&2
      exit 1
    fi
  done
  echo "$name ready on port $port"
}

# Start the server for the selected language
case "$LANG" in
  Mesh)
    PORT=3000
    if command -v meshc &> /dev/null; then
      meshc build "$REPO_ROOT/benchmarks/mesh"
      "$REPO_ROOT/benchmarks/mesh/mesh" &
      SERVER_PID=$!
    else
      echo "ERROR: meshc not found in PATH" >&2
      exit 1
    fi
    ;;
  Go)
    PORT=3001
    cd "$REPO_ROOT/benchmarks/go" && go run . &
    SERVER_PID=$!
    ;;
  Rust)
    PORT=3002
    "$REPO_ROOT/benchmarks/rust/target/release/bench" &
    SERVER_PID=$!
    ;;
  Elixir)
    PORT=3003
    cd "$REPO_ROOT/benchmarks/elixir" && MIX_ENV=prod mix run --no-halt &
    SERVER_PID=$!
    ;;
esac

wait_for_server "$PORT" "$LANG"
echo "SERVER_READY"
echo ""
echo "=== $LANG server running on port $PORT (PID: $SERVER_PID) ==="
echo "Sampling RSS every 2s to stdout"
echo ""

# Sample RSS every 2s and log to stdout
# Load gen VM can retrieve this via: fly logs --machine <id> | grep '^RSS,'
while true; do
  rss=$(grep VmRSS /proc/$SERVER_PID/status 2>/dev/null | awk '{print $2}')
  echo "RSS,$LANG,$(date +%s),${rss:-0}"
  sleep 2
done
