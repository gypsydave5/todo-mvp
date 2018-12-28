defmodule TodoMvpTest do
  use ExUnit.Case
  doctest TodoMvp

  test "greets the world" do
    assert TodoMvp.hello() == :world
  end
end
