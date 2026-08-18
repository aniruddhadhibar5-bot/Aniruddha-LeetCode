defmodule Solution do
  @spec can_construct(ransom_note :: String.t(), magazine :: String.t()) :: boolean
  def can_construct(ransom_note, magazine) do
    # Step 1: Build a frequency map of characters in the magazine
    magazine_counts = 
      magazine
      |> String.graphemes()
      |> Enum.frequencies()

    # Step 2: Convert ransom_note to graphemes and verify availability
    ransom_note
    |> String.graphemes()
    |> check_letters(magazine_counts)
  end

  # Tail-recursive helper to verify character counts
  defp check_letters([], _counts), do: true
  defp check_letters([char | rest], counts) do
    current_count = Map.get(counts, char, 0)

    if current_count > 0 do
      # Decrement the count and continue checking the remaining characters
      updated_counts = Map.put(counts, char, current_count - 1)
      check_letters(rest, updated_counts)
    else
      # Character is missing or exhausted
      false
    end
  end
end
