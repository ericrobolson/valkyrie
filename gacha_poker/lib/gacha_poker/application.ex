defmodule GachaPoker.Application do
  # See https://hexdocs.pm/elixir/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application

  def start(_type, _args) do
    children = [
      # Start the Ecto repository
      GachaPoker.Repo,
      # Start the Telemetry supervisor
      GachaPokerWeb.Telemetry,
      # Start the PubSub system
      {Phoenix.PubSub, name: GachaPoker.PubSub},
      # Start the Endpoint (http/https)
      GachaPokerWeb.Endpoint
      # Start a worker by calling: GachaPoker.Worker.start_link(arg)
      # {GachaPoker.Worker, arg}
    ]

    # See https://hexdocs.pm/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_one, name: GachaPoker.Supervisor]
    Supervisor.start_link(children, opts)
  end

  # Tell Phoenix to update the endpoint configuration
  # whenever the application is updated.
  def config_change(changed, _new, removed) do
    GachaPokerWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end
