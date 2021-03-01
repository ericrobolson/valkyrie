defmodule ClientServer do
  @moduledoc """
  Documentation for `ClientServer`.
  """

  @default_server_port 1337
  @local_host {127, 0, 0, 1}

  def launch_server do
    launch_server(@default_server_port)
  end

  def launch_server(port) do
    IO.puts("Launching server on port #{port}")
    server = Socket.UDP.open!(port)
    serve(server)
  end

  def serve(server) do
    {data, client} = server |> Socket.Datagram.recv!()
    IO.puts("Received: #{data}, from #{inspect(client)}")

    serve(server)
  end

  def send_data(data, to) do
    server = Socket.UDP.open!()
    Socket.Datagram.send(server, data, to)
  end

  def send_data(data) do
    send_data(data, {@local_host, @default_server_port})
  end
end
