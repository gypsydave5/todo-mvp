defmodule TodoMvp.Server do
  use GenServer

  def start_link(opts \\ []) do
    GenServer.start_link(__MODULE__, :ok, opts)
  end

  def list_todos() do
    GenServer.call(__MODULE__, {:list})
  end

  def add_todo(todo) when is_binary(todo) do
    GenServer.call(__MODULE__, {:add, todo})
  end

  def remove_todo(id) do
    GenServer.call(__MODULE__, {:delete, id})
  end

  def toggle_todo(id) do
    GenServer.call(__MODULE__, {:toggle, id})
  end

  #######################
  #### SERVER ###########
  #######################
  def init(:ok) do
    {:ok, []}
  end

  def handle_call({:list}, _from, state) do
    {:reply, state, state}
  end

  def handle_call({:add, todo}, _from, state) do
    new_todo = %{name: todo, done: false, id: next_id(state)}
    new_state = state ++ [new_todo]
    {:reply, new_state, new_state}
  end

  def handle_call({:toggle, id}, _from, state) do
    todo = Enum.fetch!(state, id - 1)
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
    todo = Enum.fetch!(state, id - 1)
    new_state = state -- [todo]
    {:reply, new_state, new_state}
  end

  #### PRIVATE ######
  defp next_id(state) do
    length(state) + 1
  end
end
