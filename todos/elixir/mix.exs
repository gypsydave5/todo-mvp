defmodule TodoMvp.MixProject do
  use Mix.Project

  def project do
    [
      app: :todo_mvp,
      version: "0.1.0",
      elixir: "~> 1.7",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger, :cowboy, :plug],
      mod: {TodoMvp.Application, []}
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:cowboy, "~> 2.6"},
      {:plug_cowboy, "~> 2.0"},
      {:plug, "~> 1.7"}
    ]
  end
end
