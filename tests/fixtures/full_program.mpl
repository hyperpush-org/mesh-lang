##! A sample Mesh module

module Counter do
  ## Create a new counter
  struct State do
    count :: Int
  end

  pub fn new() -> State do
    State { count: 0 }
  end

  pub fn increment(state) do
    let new_count = state.count + 1
    %State{ state | count: new_count }
  end

  pub fn display(state) do
    "Count is ${state.count}"
  end
end

# Main entry point
let counter = Counter.new()
let updated = counter |> Counter.increment() |> Counter.increment()
let message = Counter.display(updated)
