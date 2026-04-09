from Types.Todo import Todo

fn row_to_todo(row) -> Todo do
  Todo {
    id : Map.get(row, "id"),
    title : Map.get(row, "title"),
    completed : Map.get(row, "completed") == "true",
    created_at : Map.get(row, "created_at")
  }
end

fn rows_to_json_loop(rows, index :: Int, total :: Int, acc :: List < String >) -> List < String > do
  if index >= total do
    acc
  else
    let encoded = Json.encode(row_to_todo(List.get(rows, index)))
    rows_to_json_loop(rows, index + 1, total, List.append(acc, encoded))
  end
end

pub fn ensure_schema(db_path :: String) -> Int ! String do
  let db = Sqlite.open(db_path)?
  let applied = Sqlite.execute(
    db,
    "CREATE TABLE IF NOT EXISTS todos (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT NOT NULL, completed TEXT NOT NULL, created_at TEXT NOT NULL)",
    []
  )?
  Sqlite.close(db)
  Ok(applied)
end

pub fn list_todos(db_path :: String) -> String ! String do
  let db = Sqlite.open(db_path)?
  let rows = Sqlite.query(db, "SELECT id, title, completed, created_at FROM todos ORDER BY id", [])?
  Sqlite.close(db)
  let encoded = rows_to_json_loop(rows, 0, List.length(rows), List.new())
  Ok("[#{String.join(encoded, ",")}]")
end

pub fn get_todo(db_path :: String, id :: String) -> Todo ! String do
  let db = Sqlite.open(db_path)?
  let rows = Sqlite.query(db, "SELECT id, title, completed, created_at FROM todos WHERE id = ?", [id])?
  Sqlite.close(db)
  if List.length(rows) == 0 do
    Err("todo not found")
  else
    Ok(row_to_todo(List.get(rows, 0)))
  end
end

pub fn create_todo(db_path :: String, title :: String) -> Todo ! String do
  let db = Sqlite.open(db_path)?
  let created_at = DateTime.to_iso8601(DateTime.utc_now())
  let _ = Sqlite.execute(db, "INSERT INTO todos (title, completed, created_at) VALUES (?, ?, ?)", [title, "false", created_at])?
  let rows = Sqlite.query(db, "SELECT id, title, completed, created_at FROM todos WHERE id = last_insert_rowid()", [])?
  Sqlite.close(db)
  if List.length(rows) == 0 do
    Err("todo insert did not return a row")
  else
    Ok(row_to_todo(List.get(rows, 0)))
  end
end

pub fn toggle_todo(db_path :: String, id :: String) -> Todo ! String do
  let current = get_todo(db_path, id)?
  let db = Sqlite.open(db_path)?
  let next_completed = if current.completed do
    "false"
  else
    "true"
  end
  let updated = Sqlite.execute(db, "UPDATE todos SET completed = ? WHERE id = ?", [next_completed, id])?
  let rows = Sqlite.query(db, "SELECT id, title, completed, created_at FROM todos WHERE id = ?", [id])?
  Sqlite.close(db)
  if updated == 0 do
    Err("todo not found")
  else
    if List.length(rows) == 0 do
      Err("todo not found")
    else
      Ok(row_to_todo(List.get(rows, 0)))
    end
  end
end

pub fn delete_todo(db_path :: String, id :: String) -> String ! String do
  let current = get_todo(db_path, id)?
  let db = Sqlite.open(db_path)?
  let deleted = Sqlite.execute(db, "DELETE FROM todos WHERE id = ?", [id])?
  Sqlite.close(db)
  if deleted == 0 do
    Err("todo not found")
  else
    Ok(current.id)
  end
end
