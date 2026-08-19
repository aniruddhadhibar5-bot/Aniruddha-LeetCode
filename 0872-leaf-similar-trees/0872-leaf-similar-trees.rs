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
    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut leaves1 = Vec::new();
        let mut leaves2 = Vec::new();

        // Collect leaves from both trees
        Self::get_leaves(&root1, &mut leaves1);
        Self::get_leaves(&root2, &mut leaves2);

        // Compare the two sequence vectors
        leaves1 == leaves2
    }

    fn get_leaves(node_opt: &Option<Rc<RefCell<TreeNode>>>, leaves: &mut Vec<i32>) {
        if let Some(node_rc) = node_opt {
            let node = node_rc.borrow();

            // Check if current node is a leaf node
            if node.left.is_none() && node.right.is_none() {
                leaves.push(node.val);
                return;
            }

            // Recurse left first to ensure left-to-right sequence collection
            Self::get_leaves(&node.left, leaves);
            Self::get_leaves(&node.right, leaves);
        }
    }
}
