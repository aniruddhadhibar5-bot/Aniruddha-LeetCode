class BSTIterator:

    def __init__(self, root):
        self.stack = []
        # Push the left branch of the tree starting from root
        self._push_left(root)

    def _push_left(self, node):
        while node:
            self.stack.append(node)
            node = node.left

    def next(self):
        # The top element of the stack is the next smallest element
        top_node = self.stack.pop()
        
        # If the node has a right child, process its left branch
        if top_node.right:
            self._push_left(top_node.right)
            
        return top_node.val

    def hasNext(self):
        # If the stack has elements, a next element exists
        return len(self.stack) > 0
