defmodule GachaPoker.Repo do
  use Ecto.Repo,
    otp_app: :gacha_poker,
    adapter: Ecto.Adapters.Postgres
end
