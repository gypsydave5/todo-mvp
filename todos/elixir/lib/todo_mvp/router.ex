defmodule TodoMvp.Router do
  use Plug.Router
  require EEx

  @template "lib/todo_mvp/template.html.eex"

  plug(Plug.Static,
    from: :todo_mvp,
    at: "/static"
  )

  plug(Plug.Logger)

  plug(:match)
  plug(:dispatch)

  get "/" do
    response =
      TodoMvp.list()
      |> serve_file()

    send_resp(conn, 200, response)
  end

  post "/" do
    response =
      read_response(conn)
      |> String.replace("+", " ")
      |> build_response(&TodoMvp.add/1)
      |> serve_file()

    send_resp(conn, 200, response)
  end

  post "/done" do
    response =
      read_response(conn)
      |> String.to_integer()
      |> build_response(&TodoMvp.toggle/1)
      |> serve_file()

    send_resp(conn, 200, response)
  end

  post "/not-done" do
    response =
      read_response(conn)
      |> String.to_integer()
      |> build_response(&TodoMvp.toggle/1)
      |> serve_file()

    send_resp(conn, 200, response)
  end

  post "/delete" do
    response =
      read_response(conn)
      |> String.to_integer()
      |> build_response(&TodoMvp.delete/1)
      |> serve_file()

    send_resp(conn, 200, response)
  end

  match(_, do: send_resp(conn, 404, "Whoops"))

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
