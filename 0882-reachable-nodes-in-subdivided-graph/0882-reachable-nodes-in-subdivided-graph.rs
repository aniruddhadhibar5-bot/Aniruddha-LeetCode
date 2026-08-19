use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
        let n = n as usize;
        
        // Build adjacency list: graph[u] = Vec<(v, weight)>
        let mut graph = vec![Vec::new(); n];
        for edge in &edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let cnt = edge[2];
            let weight = cnt + 1;
            graph[u].push((v, weight));
            graph[v].push((u, weight));
        }

        // Min-Heap for Dijkstra: stores (distance, node)
        let mut pq = BinaryHeap::new();
        let mut dist = vec![i32::MAX; n];

        dist[0] = 0;
        pq.push(Reverse((0, 0)));

        while let Some(Reverse((d, u))) = pq.pop() {
            if d > dist[u] {
                continue;
            }

            for &(v, weight) in &graph[u] {
                let next_d = d + weight;
                if next_d < dist[v] {
                    dist[v] = next_d;
                    pq.push(Reverse((next_d, v)));
                }
            }
        }

        // Count reachable main nodes
        let mut reachable_count = 0;
        for u in 0..n {
            if dist[u] <= max_moves {
                reachable_count += 1;
            }
        }

        // Count reachable subdivision nodes on each edge
        for edge in &edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let cnt = edge[2];

            let reached_from_u = (max_moves - dist[u]).max(0);
            let reached_from_v = (max_moves - dist[v]).max(0);

            reachable_count += cnt.min(reached_from_u + reached_from_v);
        }

        reachable_count
    }
}
