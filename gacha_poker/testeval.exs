defmodule HandEval do
  @moduledoc """
  This is a poker hand evaluator.
  """


  @doc """
  Simple test func
  """
  def hello do
    true
  end

end


defmodule HandEval do
  use ExUnit.Case
  doctest HandEval

  test "hello returns true" do
    assert HandEval.hello() == true
  end
end