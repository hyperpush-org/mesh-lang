# E2E test for SQLite JOIN runtime behavior
# Tests: INNER JOIN (rows from both tables), LEFT JOIN (NULL -> empty string for unmatched rows)
# Phase 107 Gap Closure: Verifies ROADMAP SC2 (NULL handling) and SC4 (multi-table fields)

fn run_db() -> Int!String do
  let db = Sqlite.open(":memory:")?

  # Create tables
  let _ = Sqlite.execute(db, "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL)", [])?
  let _ = Sqlite.execute(db, "CREATE TABLE profiles (id INTEGER PRIMARY KEY, user_id INTEGER, bio TEXT)", [])?

  # Insert users
  let _ = Sqlite.execute(db, "INSERT INTO users (id, name) VALUES (?, ?)", ["1", "Alice"])?
  let _ = Sqlite.execute(db, "INSERT INTO users (id, name) VALUES (?, ?)", ["2", "Bob"])?
  let _ = Sqlite.execute(db, "INSERT INTO users (id, name) VALUES (?, ?)", ["3", "Charlie"])?

  # Insert profiles (no profile for Charlie -- LEFT JOIN NULL test)
  let _ = Sqlite.execute(db, "INSERT INTO profiles (id, user_id, bio) VALUES (?, ?, ?)", ["1", "1", "Engineer"])?
  let _ = Sqlite.execute(db, "INSERT INTO profiles (id, user_id, bio) VALUES (?, ?, ?)", ["2", "2", "Designer"])?

  # INNER JOIN test (SC4: rows with fields from both tables)
  println("inner_join")
  let inner_rows = Sqlite.query(db, "SELECT u.name AS user_name, p.bio AS user_bio FROM users u INNER JOIN profiles p ON p.user_id = u.id ORDER BY u.name", [])?
  List.map(inner_rows, fn(row) do
    let name = Map.get(row, "user_name")
    let bio = Map.get(row, "user_bio")
    println(name <> ":" <> bio)
  end)

  # LEFT JOIN test (SC2: unmatched rows return NULL/empty)
  println("left_join")
  let left_rows = Sqlite.query(db, "SELECT u.name AS user_name, p.bio AS user_bio FROM users u LEFT JOIN profiles p ON p.user_id = u.id ORDER BY u.name", [])?
  List.map(left_rows, fn(row) do
    let name = Map.get(row, "user_name")
    let bio = Map.get(row, "user_bio")
    println(name <> ":" <> bio)
  end)

  # Multi-table JOIN test (SC4: columns from 3 tables)
  let _ = Sqlite.execute(db, "CREATE TABLE departments (id INTEGER PRIMARY KEY, dept_name TEXT NOT NULL)", [])?
  let _ = Sqlite.execute(db, "CREATE TABLE user_departments (user_id INTEGER, dept_id INTEGER)", [])?

  let _ = Sqlite.execute(db, "INSERT INTO departments (id, dept_name) VALUES (?, ?)", ["1", "Engineering"])?
  let _ = Sqlite.execute(db, "INSERT INTO departments (id, dept_name) VALUES (?, ?)", ["2", "Design"])?

  let _ = Sqlite.execute(db, "INSERT INTO user_departments (user_id, dept_id) VALUES (?, ?)", ["1", "1"])?
  let _ = Sqlite.execute(db, "INSERT INTO user_departments (user_id, dept_id) VALUES (?, ?)", ["2", "2"])?

  println("multi_join")
  let multi_rows = Sqlite.query(db, "SELECT u.name AS user_name, p.bio AS user_bio, d.dept_name AS dept FROM users u INNER JOIN profiles p ON p.user_id = u.id INNER JOIN user_departments ud ON ud.user_id = u.id INNER JOIN departments d ON d.id = ud.dept_id ORDER BY u.name", [])?
  List.map(multi_rows, fn(row) do
    let name = Map.get(row, "user_name")
    let bio = Map.get(row, "user_bio")
    let dept = Map.get(row, "dept")
    println(name <> ":" <> bio <> ":" <> dept)
  end)

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
