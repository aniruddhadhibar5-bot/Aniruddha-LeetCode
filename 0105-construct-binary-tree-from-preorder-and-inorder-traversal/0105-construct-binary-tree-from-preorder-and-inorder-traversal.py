class Solution:
    def buildTree(self, preorder, inorder):
        if not preorder or not inorder:
            return None

        # Map each value to its index in inorder for O(1) lookup
        inorder_index = {val: idx for idx, val in enumerate(inorder)}

        def helper(pre_left, pre_right, in_left, in_right):
            if pre_left > pre_right or in_left > in_right:
                return None

            # Root is always the first element in preorder
            root_val = preorder[pre_left]
            root = TreeNode(root_val)

            # Find root position in inorder
            in_root_idx = inorder_index[root_val]
            left_size = in_root_idx - in_left

            # Recursively build left and right subtrees
            root.left = helper(pre_left + 1, pre_left + left_size, in_left, in_root_idx - 1)
            root.right = helper(pre_left + left_size + 1, pre_right, in_root_idx + 1, in_right)

            return root

        return helper(0, len(preorder) - 1, 0, len(inorder) - 1)
