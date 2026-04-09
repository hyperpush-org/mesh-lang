from Runtime.Registry import get_db_path, get_rate_limiter
from Services.RateLimiter import allow_write
from Storage.Todos import create_todo, delete_todo, get_todo, list_todos, toggle_todo
from Types.Todo import Todo

fn todo_to_json(todo :: Todo) -> String do
  Json.encode(todo)
end

fn require_param(request, name :: String) -> String do
  let value = Request.param(request, name)
  case value do
    Some( param) -> param
    None -> ""
  end
end

fn title_from_body(body :: String) -> String do
  String.trim(Json.get(body, "title"))
end

fn not_found_response() do
  HTTP.response(404, json { error : "todo not found" })
end

fn rate_limited_response() do
  HTTP.response(429, json { error : "rate limited" })
end

fn allow_mutation() -> Bool do
  let limiter_pid = get_rate_limiter()
  allow_write(limiter_pid, "todo-write")
end

pub fn handle_list_todos(_request :: Request) -> Response do
  case list_todos(get_db_path()) do
    Ok( todos_json) -> HTTP.response(200, todos_json)
    Err( reason) -> HTTP.response(500, json { error : reason })
  end
end

pub fn handle_get_todo(request :: Request) -> Response do
  let id = require_param(request, "id")
  case get_todo(get_db_path(), id) do
    Ok( todo) -> HTTP.response(200, todo_to_json(todo))
    Err( reason) -> if reason == "todo not found" do
      not_found_response()
    else
      HTTP.response(500, json { error : reason })
    end
  end
end

pub fn handle_create_todo(request) do
  if allow_mutation() do
    let title = title_from_body(Request.body(request))
    if String.length(title) == 0 do
      HTTP.response(400, json { error : "title is required" })
    else
      case create_todo(get_db_path(), title) do
        Ok( todo) -> HTTP.response(201, todo_to_json(todo))
        Err( reason) -> HTTP.response(500, json { error : reason })
      end
    end
  else
    rate_limited_response()
  end
end

pub fn handle_toggle_todo(request) do
  if allow_mutation() do
    let id = require_param(request, "id")
    case toggle_todo(get_db_path(), id) do
      Ok( todo) -> HTTP.response(200, todo_to_json(todo))
      Err( reason) -> if reason == "todo not found" do
        not_found_response()
      else
        HTTP.response(500, json { error : reason })
      end
    end
  else
    rate_limited_response()
  end
end

pub fn handle_delete_todo(request) do
  if allow_mutation() do
    let id = require_param(request, "id")
    case delete_todo(get_db_path(), id) do
      Ok( deleted_id) -> HTTP.response(200, json { status : "deleted", id : deleted_id })
      Err( reason) -> if reason == "todo not found" do
        not_found_response()
      else
        HTTP.response(500, json { error : reason })
      end
    end
  else
    rate_limited_response()
  end
end
