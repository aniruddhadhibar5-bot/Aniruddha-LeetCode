class Solution {
    func rob(_ root: TreeNode?) -> Int {
        let result = dfs(root)
        return max(result.rob, result.notRob)
    }
    
    private func dfs(_ node: TreeNode?) -> (rob: Int, notRob: Int) {
        // Base case: empty node yields 0 money either way
        guard let node = node else {
            return (0, 0)
        }
        
        // Bottom-up traversal
        let left = dfs(node.left)
        let right = dfs(node.right)
        
        // Option 1: Rob this house -> must skip children
        let robThisNode = node.val + left.notRob + right.notRob
        
        // Option 2: Skip this house -> take maximums from left and right subtrees
        let skipThisNode = max(left.rob, left.notRob) + max(right.rob, right.notRob)
        
        return (robThisNode, skipThisNode)
    }
}
