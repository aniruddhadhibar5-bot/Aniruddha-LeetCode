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
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        // Full binary trees cannot have an even number of nodes
        if n % 2 == 0 {
            return vec![];
        }
        
        let n = n as usize;
        // memo[i] will store all valid full binary trees of size i
        let mut memo = vec![vec![]; n + 1];
        
        // Base case: a full binary tree of size 1 is just a single leaf node
        memo[1] = vec![Some(Rc::new(RefCell::new(TreeNode::new(0))))];

        // Iteratively compute full binary trees for all odd sizes up to n
        for i in (3..=n).step_by(2) {
            let mut current_trees = Vec::new();
            
            // Partition the remaining i - 1 nodes between left and right subtrees
            for left_size in (1..i).step_by(2) {
                let right_size = i - 1 - left_size;
                
                // Combine every left subtree configuration with every right subtree configuration
                for left_tree in &memo[left_size] {
                    for right_tree in &memo[right_size] {
                        let root = Rc::new(RefCell::new(TreeNode::new(0)));
                        root.borrow_mut().left = left_tree.clone();
                        root.borrow_mut().right = right_tree.clone();
                        current_trees.push(Some(root));
                    }
                }
            }
            memo[i] = current_trees;
        }

        memo[n].clone()
    }
}
