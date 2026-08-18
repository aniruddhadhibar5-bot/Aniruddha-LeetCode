use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        // Build the adjacency list graph: Map<Node, Vec<(Neighbor, Weight)>>
        let mut graph: HashMap<String, Vec<(String, f64)>> = HashMap::new();

        for (i, eq) in equations.iter().enumerate() {
            let u = &eq[0];
            let v = &eq[1];
            let weight = values[i];

            graph.entry(u.clone()).or_default().push((v.clone(), weight));
            graph.entry(v.clone()).or_default().push((u.clone(), 1.0 / weight));
        }

        let mut results = Vec::with_capacity(queries.len());

        // Process each query
        for q in queries {
            let start = &q[0];
            let end = &q[1];

            // If either variable wasn't in the equations, it is undefined
            if !graph.contains_key(start) || !graph.contains_key(end) {
                results.push(-1.0);
                continue;
            }

            // If finding division by itself, answer is 1.0
            if start == end {
                results.push(1.0);
                continue;
            }

            // Perform DFS to find path from start to end
            let mut visited = HashSet::new();
            let mut path_weight = -1.0;
            
            Self::dfs(start, end, 1.0, &graph, &mut visited, &mut path_weight);
            results.push(path_weight);
        }

        results
    }

    fn dfs(
        curr: &str,
        target: &str,
        curr_weight: f64,
        graph: &HashMap<String, Vec<(String, f64)>>,
        visited: &mut HashSet<String>,
        ans: &mut f64,
    ) -> bool {
        if curr == target {
            *ans = curr_weight;
            return true;
        }

        visited.insert(curr.to_string());

        if let Some(neighbors) = graph.get(curr) {
            for (next_node, edge_weight) in neighbors {
                if !visited.contains(next_node) {
                    if Self::dfs(next_node, target, curr_weight * edge_weight, graph, visited, ans) {
                        return true;
                    }
                }
            }
        }

        false
    }
}
