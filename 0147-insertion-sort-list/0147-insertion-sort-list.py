class Solution:
    def insertionSortList(self, head):
        if not head or not head.next:
            return head

        dummy = ListNode(0)
        curr = head

        while curr:
            prev = dummy
            # Find the correct position to insert current node
            while prev.next and prev.next.val < curr.val:
                prev = prev.next

            next_temp = curr.next
            curr.next = prev.next
            prev.next = curr
            curr = next_temp

        return dummy.next
