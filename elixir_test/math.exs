defmodule Math do
  def zero?(0), do: true

  def zero?(x) when is_integer(x), do: false

  def sum_list([head | tail], accumulator) do
    sum_list(tail, head + accumulator)
  end

  def sum_list([], accumulator) do
    accumulator
  end

  def double_each([head | tail]) do
    [head * 2 | double_each(tail)]
  end

  def double_each([]) do
    []
  end
end

defmodule Concat do
  def join(a, b, separator \\ " "), do: a <> separator <> b
end

defmodule Recursion do
  def print_multiple_times(msg, n) when n <= 1 do
    IO.puts(msg)
  end

  def print_multiple_times(msg, n) do
    print_multiple_times(msg, 0)
    print_multiple_times(msg, n - 1)
  end
end

IO.puts(Math.double_each([1, 2, 3]))
