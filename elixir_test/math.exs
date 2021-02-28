defmodule Math do
    def zero?(0), do: true

    def zero?(x) when is_integer(x), do: false
end

defmodule Concat do
    def join(a, b, separator \\ " "), do: a <> separator <> b
end