defmodule TodoMvp.Router do
  @moduledoc """
  Router module.
  """
  use Plug.Router
  require EEx

  @template "lib/todo_mvp/template.html.eex"

  alias TodoMvp.Server

  @doc """
  Serve static files from /static
  """
  plug(Plug.Static, from: :todo_mvp, at: "/static")
  plug(Plug.Logger)
  plug(:match)
  plug(:dispatch)

  @doc """
  GET "/" gets the list of Todos.
  """
  get "/" do
    response =
      Server.list()
      |> serve_file()

    send_resp(conn, 200, response)
  end

  @doc """
  POST "/" adds a Todo.
  """
  post "/" do
    response =
      read_response(conn)
      |> String.replace("+", " ")
      |> build_response(&Server.add/1)
      |> serve_file()

    send_resp(conn, 200, response)
  end

  @doc """
  POST "/done" toggles a todo.
  """
  post "/done" do
    response =
      read_response(conn)
      |> build_response(&Server.toggle/1)
      |> serve_file()

    send_resp(conn, 200, response)
  end

  @doc """
  POST "/not-done" toggles a todo.
  """
  post "/not-done" do
    response =
      read_response(conn)
      |> build_response(&Server.toggle/1)
      |> serve_file()

    send_resp(conn, 200, response)
  end

  @doc """
  POST "/delete" deletes a todo.
  """
  post "/delete" do
    response =
      read_response(conn)
      |> build_response(&Server.remove/1)
      |> serve_file()

    send_resp(conn, 200, response)
  end

  @doc """
  Match a 404 Page
  """
  match(_, do: send_resp(conn, 404, "No page exists here."))

  ####################
  ##### Private ######
  ####################
  defp read_response(conn) do
    {:ok, body, _conn} = read_body(conn)
    "item=" <> item = body
    item
  end

  defp build_response(item, f) do
    f.(item)
  end

  defp serve_file(todos) do
    EEx.eval_file(@template, todos: todos)
  end
end
