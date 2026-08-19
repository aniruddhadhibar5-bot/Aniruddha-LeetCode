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
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let dummy = Rc::new(RefCell::new(TreeNode::new(0)));
        let mut curr = Rc::clone(&dummy);
        
        Self::inorder(root, &mut curr);
        
        // Take the right child of the dummy node which holds the new root
        dummy.borrow_mut().right.take()
    }
    
    fn inorder(node_opt: Option<Rc<RefCell<TreeNode>>>, curr: &mut Rc<RefCell<TreeNode>>) {
        if let Some(node_rc) = node_opt {
            // 1. Traverse left subtree and detach it from current node
            let left = node_rc.borrow_mut().left.take();
            Self::inorder(left, curr);
            
            // 2. Detach right subtree from current node to isolate it
            let right = node_rc.borrow_mut().right.take();
            
            // 3. Link the current isolated node as the right child of the flattened chain
            curr.borrow_mut().right = Some(Rc::clone(&node_rc));
            *curr = node_rc;
            
            // 4. Traverse right subtree
            Self::inorder(right, curr);
        }
    }
}
