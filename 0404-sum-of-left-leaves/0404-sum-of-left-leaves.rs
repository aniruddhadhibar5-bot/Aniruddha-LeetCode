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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root, false)
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
        match node {
            None => 0,
            Some(n) => {
                let borrowed = n.borrow();
                
                // Base case: Check if this node is a leaf node
                if borrowed.left.is_none() && borrowed.right.is_none() {
                    return if is_left { borrowed.val } else { 0 };
                }
                
                // Recursively compute the sum from the left and right subtrees
                let left_sum = Self::dfs(&borrowed.left, true);
                let right_sum = Self::dfs(&borrowed.right, false);
                
                left_sum + right_sum
            }
        }
    }
}
