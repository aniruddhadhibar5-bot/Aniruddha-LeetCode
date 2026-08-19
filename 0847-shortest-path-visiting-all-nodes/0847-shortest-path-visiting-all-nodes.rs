use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        if n <= 1 {
            return 0;
        }

        let target_mask = (1 << n) - 1;
        
        // visited[node][mask] keeps track of whether a state has been processed
        let mut visited = vec![vec![false; 1 << n]; n];
        let mut queue = VecDeque::new();

        // Initialize the queue with all possible starting nodes
        for i in 0..n {
            let mask = 1 << i;
            visited[i][mask] = true;
            // Queue stores: (current_node, visited_mask, current_path_length)
            queue.push_back((i, mask, 0));
        }

        while let Some((node, mask, steps)) = queue.pop_front() {
            // If all nodes have been visited, return the number of steps taken
            if mask == target_mask {
                return steps;
            }

            // Explore all neighbors of the current node
            for &neighbor in &graph[node] {
                let next_node = neighbor as usize;
                let next_mask = mask | (1 << next_node);

                // If this state combination (node + set of visited nodes) is unique, traverse it
                if !visited[next_node][next_mask] {
                    visited[next_node][next_mask] = true;
                    queue.push_back((next_node, next_mask, steps + 1));
                }
            }
        }

        0
    }
}
