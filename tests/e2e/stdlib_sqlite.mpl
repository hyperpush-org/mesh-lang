# E2E test for SQLite CRUD lifecycle
# Tests: open, execute (DDL + DML), query, parameterized queries, close

fn run_db() -> Int!String do
  # SQLT-01: Open in-memory database
  let db = Sqlite.open(":memory:")?

  # SQLT-04: Execute DDL (CREATE TABLE)
  let _ = Sqlite.execute(db, "CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL, age TEXT NOT NULL)", [])?

  # SQLT-04 + SQLT-05: Insert with parameterized queries
  let inserted1 = Sqlite.execute(db, "INSERT INTO users (name, age) VALUES (?, ?)", ["Alice", "30"])?
  println("${inserted1}")

  let inserted2 = Sqlite.execute(db, "INSERT INTO users (name, age) VALUES (?, ?)", ["Bob", "25"])?
  println("${inserted2}")

  # SQLT-03: Query rows back
  let rows = Sqlite.query(db, "SELECT name, age FROM users ORDER BY name", [])?

  # Print each row (Map.get returns String directly for Map<String, String>)
  List.map(rows, fn(row) do
    let name = Map.get(row, "name")
    let age = Map.get(row, "age")
    println(name <> ":" <> age)
  end)

  # SQLT-05: Parameterized WHERE clause
  let filtered = Sqlite.query(db, "SELECT name FROM users WHERE age = ?", ["30"])?
  List.map(filtered, fn(row) do
    let name = Map.get(row, "name")
    println("filtered:" <> name)
  end)

  # SQLT-02: Close connection
  Sqlite.close(db)

  Ok(0)
end

fn main() do
  let r = run_db()
  case r do
    Ok(_) -> println("done")
    Err(msg) -> println("error: " <> msg)
  end
end
