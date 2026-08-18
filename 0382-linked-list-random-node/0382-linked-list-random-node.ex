# Definition for singly-linked list.
#
# defmodule ListNode do
#   @type t :: %__MODULE__{val: integer, next: ListNode.t() | nil}
#   defstruct val: 0, next: nil
# end

defmodule Solution do
  @spec init_(head :: ListNode.t() | nil) :: any
  def init_(head) do
    # Store the head of the list in the Process Dictionary
    :erlang.put(:head, head)
    :ok
  end

  @spec get_random() :: integer
  def get_random() do
    head = :erlang.get(:head)
    # Start Reservoir Sampling with index 1
    sample(head, nil, 1)
  end

  # Tail-recursive helper to sample elements on the fly
  defp sample(nil, chosen_val, _index), do: chosen_val
  defp sample(node, chosen_val, index) do
    # Generate a random integer from 1 to index
    # The probability of picking 1 is exactly 1 / index
    new_chosen = if :rand.uniform(index) == 1 do
      node.val
    else
      chosen_val
    end

    # Tail-recursively move to the next node
    sample(node.next, new_chosen, index + 1)
  end
end
