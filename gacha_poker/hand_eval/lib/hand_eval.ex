defmodule HandEval do
  @moduledoc """
  Documentation for `HandEval`.
  Based on Pokersource Poker Eval Evaluator https://www.codingthewheel.com/archives/poker-hand-evaluator-roundup/
  """

  @doc """
  Creates a binary card representation for a card in the deck.
  LSB is 2 of Hearts, MSB is Ace of Spades
  """
  def make_card(cardnum) do
    suit_num = 13

    heart_boundary = suit_num + 1
    diamond_boundary = heart_boundary + suit_num
    clubs_boundary = diamond_boundary + suit_num
    # spades_boundary = clubs_boundary + suit_num

    cond do
      # Hearts
      cardnum <= heart_boundary ->
        <<0, 0, 0, cardnum>>

      # Diamonds
      cardnum <= diamond_boundary ->
        <<0, 0, cardnum - heart_boundary, 0>>

      # Clubs
      cardnum <= clubs_boundary ->
        <<0, cardnum - diamond_boundary, 0, 0>>

      # Spades
      cardnum <= clubs_boundary ->
        <<cardnum - clubs_boundary, 0, 0, 0>>

      true ->
        <<0, 0, 0, 0>>
    end
  end
end
