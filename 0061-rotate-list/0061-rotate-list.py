class Solution:
    def rotateRight(self, head, k):
        if not head or not head.next or k == 0:
            return head
        
        # 1. Compute the length of the linked list and find the tail node
        length = 1
        tail = head
        while tail.next:
            tail = tail.next
            length += 1
            
        # 2. Close the loop to make it a circular linked list
        tail.next = head
        
        # 3. Find the new tail position: (length - (k % length)) steps from head
        k = k % length
        steps_to_new_tail = length - k
        
        new_tail = head
        for _ in range(steps_to_new_tail - 1):
            new_tail = new_tail.next
            
        # 4. Break the circular loop to establish the new head
        new_head = new_tail.next
        new_tail.next = None
        
        return new_head
