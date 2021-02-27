# This file is responsible for configuring your application
# and its dependencies with the aid of the Mix.Config module.
#
# This configuration file is loaded before any dependency and
# is restricted to this project.

# General application configuration
use Mix.Config

config :books,
  ecto_repos: [Books.Repo],
  generators: [binary_id: true]

# Configures the endpoint
config :books, BooksWeb.Endpoint,
  url: [host: "localhost"],
  secret_key_base: "TU4ylQ+h5MDyIX+pqIy2uZYp8J7kq1h7egmnBH84/Gcm7er94Nxl358OppGll5cR",
  render_errors: [view: BooksWeb.ErrorView, accepts: ~w(json), layout: false],
  pubsub_server: Books.PubSub,
  live_view: [signing_salt: "Fh/NHHuk"]

# Configures Elixir's Logger
config :logger, :console,
  format: "$time $metadata[$level] $message\n",
  metadata: [:request_id]

# Use Jason for JSON parsing in Phoenix
config :phoenix, :json_library, Jason

# Import environment specific config. This must remain at the bottom
# of this file so it overrides the configuration defined above.
import_config "#{Mix.env()}.exs"
