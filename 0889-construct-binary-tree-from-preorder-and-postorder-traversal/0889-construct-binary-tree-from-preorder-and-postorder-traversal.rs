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
    pub fn construct_from_pre_post(preorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build(&preorder, &postorder)
    }

    fn build(pre: &[i32], post: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if pre.is_empty() {
            return None;
        }

        // The first element in preorder is always the root of the current subtree
        let root_val = pre[0];
        let mut root = TreeNode::new(root_val);

        // If there's only one element, it's a leaf node
        if pre.len() == 1 {
            return Some(Rc::new(RefCell::new(root)));
        }

        // pre[1] is the root value of the left subtree
        let left_root_val = pre[1];
        
        // Find where the left root value resides in the postorder array
        let mut left_subtree_len = 0;
        for i in 0..post.len() {
            if post[i] == left_root_val {
                left_subtree_len = i + 1;
                break;
            }
        }

        // Slice the arrays dynamically to build subtrees recursively
        root.left = Self::build(
            &pre[1..=left_subtree_len], 
            &post[0..left_subtree_len]
        );
        root.right = Self::build(
            &pre[(left_subtree_len + 1)..], 
            &post[left_subtree_len..(post.len() - 1)]
        );

        Some(Rc::new(RefCell::new(root)))
    }
}
