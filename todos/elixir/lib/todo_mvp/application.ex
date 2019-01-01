defmodule TodoMvp.Application do
  # See https://hexdocs.pm/elixir/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application
  require Logger

  def start(_type, _args) do
    # List all child processes to be supervised
    children = [
      {Plug.Cowboy, scheme: :http, plug: TodoMvp.Router, options: [port: 3000]},
      {TodoMvp.Server, [name: TodoMvp.Server]}
      # Starts a worker by calling: TodoMvp.Worker.start_link(arg)
      # {TodoMvp.Worker, arg},
    ]

    # See https://hexdocs.pm/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_one, name: TodoMvp.Supervisor]
    Logger.info("Starting application on port 3000.")
    Supervisor.start_link(children, opts)
  end
end
