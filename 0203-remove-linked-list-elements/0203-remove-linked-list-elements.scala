/**
 * Definition for singly-linked list.
 * class ListNode(_x: Int = 0, _next: ListNode = null) {
 *   var next: ListNode = _next
 *   var x: Int = _x
 * }
 */
object Solution {
    def removeElements(head: ListNode, `val`: Int): ListNode = {
        // Create a dummy node to handle head-removal edge cases easily
        val dummy = new ListNode(0, head)
        var curr = dummy

        while (curr.next != null) {
            if (curr.next.x == `val`) {
                // Bypass the node with the matching value
                curr.next = curr.next.next
            } else {
                // Advance the cursor forward if no removal is needed
                curr = curr.next
            }
        }

        dummy.next
    }
}
