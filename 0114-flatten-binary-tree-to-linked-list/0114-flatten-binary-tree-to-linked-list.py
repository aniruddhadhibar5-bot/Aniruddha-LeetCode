class Solution:
    def flatten(self, root):
        curr = root
        while curr:
            if curr.left:
                # Find the rightmost node in the left subtree
                prev = curr.left
                while prev.right:
                    prev = prev.right
                # Connect that node's right pointer to current's right subtree
                prev.right = curr.right
                # Move left subtree to the right
                curr.right = curr.left
                curr.left = None
            # Move to the next node
            curr = curr.right
