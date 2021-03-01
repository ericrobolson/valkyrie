# ClientServer
launch `iex.bat -S mix`
launch server `ClientServer.launch_server`

Open new terminal
Launch client `iex.bat -S mix`
Send data with `ClientServer.send_data("HELLO WORLD")`
Observe it is sent to server terminal

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `clientserver` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:clientserver, "~> 0.1.0"}
  ]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at [https://hexdocs.pm/clientserver](https://hexdocs.pm/clientserver).

