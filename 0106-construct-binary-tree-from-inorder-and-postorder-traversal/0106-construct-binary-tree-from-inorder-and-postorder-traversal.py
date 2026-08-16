class Solution:
    def buildTree(self, inorder, postorder):
        if not inorder or not postorder:
            return None

        # Map each value to its index in inorder for O(1) lookup
        inorder_index = {val: idx for idx, val in enumerate(inorder)}

        def helper(in_left, in_right):
            if in_left > in_right:
                return None

            # The last element in postorder is the root
            root_val = postorder.pop()
            root = TreeNode(root_val)

            # Find root position in inorder
            in_root_idx = inorder_index[root_val]

            # Build right subtree first (since postorder pops from end)
            root.right = helper(in_root_idx + 1, in_right)
            root.left = helper(in_left, in_root_idx - 1)

            return root

        return helper(0, len(inorder) - 1)
