defmodule GameCompilerTest do
  use ExUnit.Case
  doctest GameCompiler

  test "greets the world" do
    assert GameCompiler.hello() == :world
  end
end
