impl Solution {
    pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
        let n = quiet.len();
        // Construct adjacency list: graph[b] contains all people 'a' who are richer than 'b'
        let mut graph = vec![Vec::new(); n];
        for pair in richer {
            let a = pair[0] as usize;
            let b = pair[1] as usize;
            graph[b].push(a);
        }

        // Initialize answers with -1 to signify unvisited/uncalculated states
        let mut answer = vec![-1; n];

        // Run memoized DFS for every person
        for i in 0..n {
            Self::dfs(i, &graph, &quiet, &mut answer);
        }

        answer
    }

    fn dfs(node: usize, graph: &Vec<Vec<usize>>, quiet: &Vec<i32>, answer: &mut Vec<i32>) -> usize {
        // If the answer for this person is already calculated, return it
        if answer[node] != -1 {
            return answer[node] as usize;
        }

        // Start by assuming the quietest person richer than or equal to 'node' is 'node' itself
        let mut quietest_person = node;

        // Check all people who are definitely richer than the current node
        for &richer_person in &graph[node] {
            let candidate = Self::dfs(richer_person, graph, quiet, answer);
            
            // If the candidate found has a lower quietness level, update our best choice
            if quiet[candidate] < quiet[quietest_person] {
                quietest_person = candidate;
            }
        }

        // Cache the result for this node
        answer[node] = quietest_person as i32;
        quietest_person
    }
}
