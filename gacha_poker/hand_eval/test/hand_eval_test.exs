defmodule HandEvalTest do
  use ExUnit.Case
  doctest HandEval

  test "1 makes 2h" do
    card = 1
    expected = <<0, 0, 0, 1>>
    assert HandEval.make_card(card) == expected
  end

  test "2 makes 3h" do
    card = 2
    expected = <<0, 0, 0, 2>>
    assert HandEval.make_card(card) == expected
  end

  test "3 makes 4h" do
    card = 3
    expected = <<0, 0, 0, 3>>
    assert HandEval.make_card(card) == expected
  end

  test "4 makes 5h" do
    card = 4
    expected = <<0, 0, 0, 4>>
    assert HandEval.make_card(card) == expected
  end

  test "5 makes 6h" do
    card = 5
    expected = <<0, 0, 0, 5>>
    assert HandEval.make_card(card) == expected
  end

  test "6 makes 7h" do
    card = 6
    expected = <<0, 0, 0, 6>>
    assert HandEval.make_card(card) == expected
  end

  test "7 makes 7h" do
    card = 7
    expected = <<0, 0, 0, 7>>
    assert HandEval.make_card(card) == expected
  end

  test "8 makes 8h" do
    card = 8
    expected = <<0, 0, 0, 8>>
    assert HandEval.make_card(card) == expected
  end

  test "9 makes 9h" do
    card = 9
    expected = <<0, 0, 0, 9>>
    assert HandEval.make_card(card) == expected
  end

  test "10 makes 10h" do
    card = 10
    expected = <<0, 0, 0, 10>>
    assert HandEval.make_card(card) == expected
  end

  test "11 makes Jh" do
    card = 11
    expected = <<0, 0, 0, 11>>
    assert HandEval.make_card(card) == expected
  end

  test "12 makes Qh" do
    card = 12
    expected = <<0, 0, 0, 12>>
    assert HandEval.make_card(card) == expected
  end

  test "13 makes Kh" do
    card = 13
    expected = <<0, 0, 0, 13>>
    assert HandEval.make_card(card) == expected
  end

  test "14 makes Ah" do
    card = 14
    expected = <<0, 0, 0, 14>>
    assert HandEval.make_card(card) == expected
  end

  test "15 makes 2d" do
    card = 15
    expected = <<0, 0, 1, 0>>
    assert HandEval.make_card(card) == expected
  end

  test "16 makes 3d" do
    card = 16
    expected = <<0, 0, 2, 0>>
    assert HandEval.make_card(card) == expected
  end
end
