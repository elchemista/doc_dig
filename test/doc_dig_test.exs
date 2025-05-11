defmodule DocDigTest do
  use ExUnit.Case
  doctest DocDig

  test "test cv pdf" do
    {:ok, {text, _metadata}} = DocDig.extract_file("./test/sito.pdf")

    assert true == String.length(text) > 0
  end
end
