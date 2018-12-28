defmodule TodoMvp.Server do
  @moduledoc """
  TodoMVP Server

  Genserver for the Todo MVP.

  A GenServer handles state in memory. If the G
  """
  use GenServer

  #######################
  #### Client ###########
  #######################
  @doc """
  Start the Genserver
  """
  def start_link(opts) do
    GenServer.start_link(__MODULE__, :ok, opts)
  end

  @doc """
  List all of the todos.
  """
  def list() do
    GenServer.call(__MODULE__, {:list})
  end

  @doc """
  Add a todo.
  """
  def add(todo) when is_binary(todo) do
    GenServer.call(__MODULE__, {:add, todo})
  end

  @doc """
  Remove a todo
  """
  def remove(id) do
    GenServer.call(__MODULE__, {:delete, id})
  end

  @doc """
  Toggle complete on a todo.
  """
  def toggle(id) do
    GenServer.call(__MODULE__, {:toggle, id})
  end

  #######################
  #### SERVER ###########
  #######################
  def init(:ok) do
    {:ok, []}
  end

  @doc """
  Handles the list call, responds with the list of todos.
  """
  def handle_call({:list}, _from, state) do
    {:reply, state, state}
  end

  @doc """
  Handles the add call. Adds a todo item, responds with the list of todos.
  """
  def handle_call({:add, todo}, _from, state) do
    new_todo = %{name: todo, done: false, id: generate_id()}
    new_state = state ++ [new_todo]
    {:reply, new_state, new_state}
  end

  @doc """
  Handles the toggle call, changes the todo to complete and responds with a list of todos. Expects the ID.
  """
  def handle_call({:toggle, id}, _from, state) do
    [todo] = Enum.filter(state, fn x -> x.id == id end)
    toggled_todo = %{todo | done: !todo.done}

    new_state =
      state
      |> Enum.map(fn x ->
        if x.id == id do
          toggled_todo
        else
          x
        end
      end)

    {:reply, new_state, new_state}
  end

  def handle_call({:delete, id}, _from, state) do
    new_state =
      state
      |> Enum.filter(fn x ->
        x.id != id
      end)

    {:reply, new_state, new_state}
  end

  #### PRIVATE ######
  defp generate_id() do
    :crypto.strong_rand_bytes(64) |> Base.url_encode64() |> binary_part(0, 64)
  end
end
