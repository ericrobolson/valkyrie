# This file is responsible for configuring your application
# and its dependencies with the aid of the Mix.Config module.
#
# This configuration file is loaded before any dependency and
# is restricted to this project.

# General application configuration
use Mix.Config

config :gacha_poker,
  ecto_repos: [GachaPoker.Repo]

# Configures the endpoint
config :gacha_poker, GachaPokerWeb.Endpoint,
  url: [host: "localhost"],
  secret_key_base: "KTBeX8zjwdUrggSWB5OhIsQlFrWPNZcSgtu3Z1Qdr/O5vk11Xi2uyJHvE9gaWBuo",
  render_errors: [view: GachaPokerWeb.ErrorView, accepts: ~w(html json), layout: false],
  pubsub_server: GachaPoker.PubSub,
  live_view: [signing_salt: "V9wrVJqp"]

# Configures Elixir's Logger
config :logger, :console,
  format: "$time $metadata[$level] $message\n",
  metadata: [:request_id]

# Use Jason for JSON parsing in Phoenix
config :phoenix, :json_library, Jason

# Import environment specific config. This must remain at the bottom
# of this file so it overrides the configuration defined above.
import_config "#{Mix.env()}.exs"
