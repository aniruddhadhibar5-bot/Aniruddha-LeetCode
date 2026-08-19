use std::collections::VecDeque;

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let n = graph.len();
        // None represents unvisited nodes. Some(0) or Some(1) represents the two independent sets.
        let mut colors: Vec<Option<i32>> = vec![None; n];
        
        // Loop over all nodes to safely handle disconnected graph components
        for i in 0..n {
            if colors[i].is_some() {
                continue;
            }
            
            // Initialize BFS for this unvisited component
            let mut queue = VecDeque::new();
            colors[i] = Some(0);
            queue.push_back(i);
            
            while let Some(node) = queue.pop_front() {
                let current_color = colors[node].unwrap();
                let neighbor_color = 1 - current_color; // Toggles between 0 and 1
                
                for &neighbor in &graph[node] {
                    let neighbor_idx = neighbor as usize;
                    
                    match colors[neighbor_idx] {
                        // If the neighbor hasn't been colored, color it and add to queue
                        None => {
                            colors[neighbor_idx] = Some(neighbor_color);
                            queue.push_back(neighbor_idx);
                        }
                        // If the neighbor already has a color, ensure there is no conflict
                        Some(color) => {
                            if color == current_color {
                                return false; // Found adjacent nodes sharing the same color
                            }
                        }
                    }
                }
            }
        }
        
        true
    }
}
