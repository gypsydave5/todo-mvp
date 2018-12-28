defmodule TodoMvp do
  alias TodoMvp.Server

  def add(todo), do: Server.add_todo(todo)
  def list(), do: Server.list_todos()
  def toggle(id), do: Server.toggle_todo(id)
  def delete(id), do: Server.remove_todo(id)
end
