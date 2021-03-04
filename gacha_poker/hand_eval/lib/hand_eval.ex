use Bitwise

defmodule HandEval do
  @moduledoc """
  Documentation for `HandEval`.
  Based on Pokersource Poker Eval Evaluator https://www.codingthewheel.com/archives/poker-hand-evaluator-roundup/
  """

  @ace_rank 14

  defp is_flush_inner([a, b]) do
    a == b
  end

  defp is_flush_inner([a, b | tail]) do
    is_flush_inner([a, b]) && is_flush_inner([b] ++ tail)
  end

  defp is_flush_inner(cards) when length(cards) <= 1 do
    true
  end

  def is_flush(hand) do
    cards = Enum.sort(Enum.map(hand, fn {_rank, suit} -> suit end), :desc)
    is_flush_inner(cards)
  end

  defp is_straight_inner(cards) when length(cards) <= 1 do
    true
  end

  defp is_straight_inner([first, second]) do
    delta = first - second
    delta == 1
  end

  defp is_straight_inner([first, second | tail]) do
    is_straight_inner([first, second]) &&
      is_straight_inner([second | tail])
  end

  def is_straight(hand) do
    cards = Enum.sort(Enum.map(hand, fn {rank, _suit} -> rank end), :desc)

    if length(cards) <= 1 do
      true
    else
      [head | tail] = cards

      # Do check for ace
      if head == @ace_rank do
        is_straight_inner([head | tail]) || is_straight_inner(tail ++ [1])
      else
        is_straight_inner(cards)
      end
    end
  end

  @doc """
  Creates a card representation from a number, 0-51
  """
  def make_card(cardnum) do
    cards_in_suit = 13

    suit =
      cond do
        cardnum < cards_in_suit -> :Clubs
        cardnum < cards_in_suit * 2 -> :Diamonds
        cardnum < cards_in_suit * 3 -> :Hearts
        true -> :Spades
      end

    rank =
      case rem(cardnum, 13) do
        0 -> 2
        1 -> 3
        2 -> 4
        3 -> 5
        4 -> 6
        5 -> 7
        6 -> 8
        7 -> 9
        8 -> 10
        9 -> 11
        10 -> 12
        11 -> 13
        12 -> @ace_rank
      end

    {rank, suit}
  end
end
