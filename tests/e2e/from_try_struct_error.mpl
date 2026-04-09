# Phase 77 gap closure: ? operator auto-converts String error to AppError struct via From.
# This is the exact success criterion #4 test case.

struct AppError do
  message :: String
end

impl From<String> for AppError do
  fn from(msg :: String) -> AppError do
    AppError { message: msg }
  end
end

fn risky() -> Int!String do
  Err("something failed")
end

fn process() -> Int!AppError do
  let n = risky()?
  Ok(n + 1)
end

fn main() do
  let result = process()
  case result do
    Ok(val) -> println("${val}")
    Err(e) -> println(e.message)
  end
end
