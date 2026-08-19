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
use std::collections::VecDeque;

impl Solution {
    pub fn distance_k(root: Option<Rc<RefCell<TreeNode>>>, target: Option<Rc<RefCell<TreeNode>>>, k: i32) -> Vec<i32> {
        if root.is_none() || target.is_none() {
            return vec![];
        }
        
        // Extract the value of the target node
        let target_val = target.unwrap().borrow().val;
        
        // Max node value is 500 as per constraints, so an array of 501 handles all indices safely
        let mut adj = vec![vec![]; 501];
        
        // Step 1: Flatten the tree into an undirected graph mapping
        Self::build_graph(&root, &mut adj);
        
        // Step 2: Use BFS to explore nodes layer by layer up to distance k
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        let mut visited = vec![false; 501];
        
        queue.push_back((target_val, 0));
        visited[target_val as usize] = true;
        
        while let Some((node, dist)) = queue.pop_front() {
            // If we reached the target distance layer, record the node
            if dist == k {
                result.push(node);
                continue;
            }
            if dist > k {
                break;
            }
            
            // Traverse all neighbors (left child, right child, and parent)
            for &neighbor in &adj[node as usize] {
                if !visited[neighbor as usize] {
                    visited[neighbor as usize] = true;
                    queue.push_back((neighbor, dist + 1));
                }
            }
        }
        
        result
    }
    
    fn build_graph(node_opt: &Option<Rc<RefCell<TreeNode>>>, adj: &mut Vec<Vec<i32>>) {
        if let Some(node_rc) = node_opt {
            let node = node_rc.borrow();
            let u = node.val;
            
            // Link left child bidirectionally
            if let Some(left_rc) = &node.left {
                let v = left_rc.borrow().val;
                adj[u as usize].push(v);
                adj[v as usize].push(u);
                Self::build_graph(&node.left, adj);
            }
            
            // Link right child bidirectionally
            if let Some(right_rc) = &node.right {
                let v = right_rc.borrow().val;
                adj[u as usize].push(v);
                adj[v as usize].push(u);
                Self::build_graph(&node.right, adj);
            }
        }
    }
}
