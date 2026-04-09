# E2E test for PostgreSQL CRUD lifecycle
# Tests: connect, execute (DDL + DML), query, parameterized queries, close

fn run_db() -> Int!String do
  # PG-01: Connect to PostgreSQL
  let conn = Pg.connect("postgres://mesh_test:mesh_test@localhost:5432/mesh_test")?

  # Drop table if exists (idempotent cleanup)
  let _ = Pg.execute(conn, "DROP TABLE IF EXISTS mesh_e2e_test", [])

  # PG-04: Execute DDL (CREATE TABLE)
  let created = Pg.execute(conn, "CREATE TABLE mesh_e2e_test (id SERIAL PRIMARY KEY, name TEXT NOT NULL, age INTEGER NOT NULL)", [])?
  println("created: ${created}")

  # PG-04 + PG-05: Insert with parameterized queries ($1, $2)
  let ins1 = Pg.execute(conn, "INSERT INTO mesh_e2e_test (name, age) VALUES ($1, $2)", ["Alice", "30"])?
  println("inserted: ${ins1}")

  let ins2 = Pg.execute(conn, "INSERT INTO mesh_e2e_test (name, age) VALUES ($1, $2)", ["Bob", "25"])?
  println("inserted: ${ins2}")

  # PG-03: Query all rows
  let rows = Pg.query(conn, "SELECT id, name, age FROM mesh_e2e_test ORDER BY name", [])?
  List.map(rows, fn(row) do
    let name = Map.get(row, "name")
    let age = Map.get(row, "age")
    println(name <> " is " <> age)
  end)

  # PG-05: Query with parameter
  let filtered = Pg.query(conn, "SELECT name FROM mesh_e2e_test WHERE age > $1", ["26"])?
  List.map(filtered, fn(row) do
    let name = Map.get(row, "name")
    println("older: " <> name)
  end)

  # PG-02: Close connection
  Pg.close(conn)

  Ok(0)
end

fn main() do
  let r = run_db()
  case r do
    Ok(_) -> println("done")
    Err(msg) -> println("error: " <> msg)
  end
end
