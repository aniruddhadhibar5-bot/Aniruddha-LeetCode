class Solution:
    def generateTrees(self, n):
        if n == 0:
            return []

        def buildTrees(start, end):
            trees = []
            if start > end:
                trees.append(None)
                return trees

            for root_val in range(start, end + 1):
                left_trees = buildTrees(start, root_val - 1)
                right_trees = buildTrees(root_val + 1, end)

                for left in left_trees:
                    for right in right_trees:
                        root = TreeNode(root_val)
                        root.left = left
                        root.right = right
                        trees.append(root)
            return trees

        return buildTrees(1, n)
