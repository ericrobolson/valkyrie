defmodule HandEvalTest do
  use ExUnit.Case
  doctest HandEval

  test "is_straight empty" do
    hand = []

    expected = true
    actual = HandEval.is_straight(hand)
    assert actual == expected
  end

  test "is_straight single card" do
    hand = [
      HandEval.make_card(1)
    ]

    expected = true
    actual = HandEval.is_straight(hand)
    assert actual == expected
  end

  test "is_straight two cards consecutive" do
    hand = [
      HandEval.make_card(2),
      HandEval.make_card(1)
    ]

    expected = true
    actual = HandEval.is_straight(hand)
    assert actual == expected
  end

  test "is_straight two cards not-consecutive" do
    hand = [
      HandEval.make_card(0),
      HandEval.make_card(2)
    ]

    expected = false
    actual = HandEval.is_straight(hand)
    assert actual == expected
  end

  test "is_straight three cards consecutive" do
    hand = [
      HandEval.make_card(3),
      HandEval.make_card(2),
      HandEval.make_card(4)
    ]

    expected = true
    actual = HandEval.is_straight(hand)
    assert actual == expected
  end

  test "is_straight three cards not-consecutive" do
    hand = [
      HandEval.make_card(3),
      HandEval.make_card(6),
      HandEval.make_card(4)
    ]

    expected = false
    actual = HandEval.is_straight(hand)
    assert actual == expected
  end

  test "is_straight four cards consecutive" do
    hand = [
      HandEval.make_card(3),
      HandEval.make_card(2),
      HandEval.make_card(4),
      HandEval.make_card(1)
    ]

    expected = true
    actual = HandEval.is_straight(hand)
    assert actual == expected
  end

  test "is_straight four cards not-consecutive" do
    hand = [
      HandEval.make_card(3),
      HandEval.make_card(6),
      HandEval.make_card(6),
      HandEval.make_card(4)
    ]

    expected = false
    actual = HandEval.is_straight(hand)
    assert actual == expected
  end

  test "is_straight five cards consecutive" do
    hand = [
      HandEval.make_card(3),
      HandEval.make_card(5),
      HandEval.make_card(2),
      HandEval.make_card(4),
      HandEval.make_card(1)
    ]

    expected = true
    actual = HandEval.is_straight(hand)
    assert actual == expected
  end

  test "is_straight five cards not-consecutive" do
    hand = [
      HandEval.make_card(3),
      HandEval.make_card(6),
      HandEval.make_card(6),
      HandEval.make_card(6),
      HandEval.make_card(4)
    ]

    expected = false
    actual = HandEval.is_straight(hand)
    assert actual == expected
  end

  test "is_straight five cards consecutive with ace high" do
    hand = [
      HandEval.make_card(8),
      HandEval.make_card(9),
      HandEval.make_card(10),
      HandEval.make_card(11),
      HandEval.make_card(12)
    ]

    expected = true
    actual = HandEval.is_straight(hand)
    assert actual == expected
  end

  test "is_straight five cards consecutive with ace low" do
    hand = [
      HandEval.make_card(0),
      HandEval.make_card(1),
      HandEval.make_card(2),
      HandEval.make_card(3),
      HandEval.make_card(12)
    ]

    expected = true
    actual = HandEval.is_straight(hand)
    assert actual == expected
  end

  def assert_card(card, expected, suit) do
    actual = HandEval.make_card(card)
    assert actual == {expected, suit}
  end

  def assert_suit(offset, suit) do
    assert_card(0 + offset, 2, suit)
    assert_card(1 + offset, 3, suit)
    assert_card(2 + offset, 4, suit)
    assert_card(3 + offset, 5, suit)
    assert_card(4 + offset, 6, suit)
    assert_card(5 + offset, 7, suit)
    assert_card(6 + offset, 8, suit)
    assert_card(7 + offset, 9, suit)
    assert_card(8 + offset, 10, suit)
    assert_card(9 + offset, 11, suit)
    assert_card(10 + offset, 12, suit)
    assert_card(11 + offset, 13, suit)
    assert_card(12 + offset, 14, suit)
  end

  test "Clubs" do
    assert_suit(0, :Clubs)
  end

  test "Diamonds" do
    assert_suit(13, :Diamonds)
  end

  test "Hearts" do
    assert_suit(26, :Hearts)
  end

  test "Spades" do
    assert_suit(39, :Spades)
  end
end
