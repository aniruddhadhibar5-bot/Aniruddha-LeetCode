// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn subtree_with_all_deepest(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::dfs(&root).0
    }

    // Helper function returning: (Best subtree node candidate, depth of this branch)
    fn dfs(node_opt: &Option<Rc<RefCell<TreeNode>>>) -> (Option<Rc<RefCell<TreeNode>>>, i32) {
        if let Some(node_rc) = node_opt {
            let node = node_rc.borrow();
            
            // Recursively evaluate the left and right subtrees
            let (left_node, left_depth) = Self::dfs(&node.left);
            let (right_node, right_depth) = Self::dfs(&node.right);

            if left_depth == right_depth {
                // If both sides reach the exact same deepest depth, 
                // this current node is the lowest common ancestor (LCA)
                (Some(Rc::clone(node_rc)), left_depth + 1)
            } else if left_depth > right_depth {
                // Left side is deeper, pass up the left candidate
                (left_node, left_depth + 1)
            } else {
                // Right side is deeper, pass up the right candidate
                (right_node, right_depth + 1)
            }
        } else {
            // Base case: Null node has a depth of 0
            (None, 0)
        }
    }
}
