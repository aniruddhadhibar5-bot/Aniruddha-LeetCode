use std::collections::VecDeque;

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let total_squares = n * n;
        
        // 1. Flatten the 2D Boustrophedon matrix into a clean 1D array (1-indexed)
        let mut flat_board = vec![-1; total_squares + 1];
        let mut square_label = 1;
        let mut left_to_right = true;

        // Traverse from the bottom row up to the top row
        for r in (0..n).rev() {
            if left_to_right {
                for c in 0..n {
                    flat_board[square_label] = board[r][c];
                    square_label += 1;
                }
            } else {
                for c in (0..n).rev() {
                    flat_board[square_label] = board[r][c];
                    square_label += 1;
                }
            }
            left_to_right = !left_to_right; // Flip direction for the next row up
        }

        // 2. Perform Standard 1D Breadth-First Search (BFS)
        let mut queue = VecDeque::new();
        let mut visited = vec![false; total_squares + 1];

        queue.push_back((1, 0)); // (current_square, moves)
        visited[1] = true;

        while let Some((curr, moves)) = queue.pop_front() {
            if curr == total_squares {
                return moves;
            }

            // Simulate a standard 6-sided die roll
            let max_next = (curr + 6).min(total_squares);
            for next in (curr + 1)..=max_next {
                // If a snake/ladder exists, jump to it. Otherwise, stay at 'next'.
                let destination = if flat_board[next] != -1 {
                    flat_board[next] as usize
                } else {
                    next
                };

                if !visited[destination] {
                    visited[destination] = true;
                    queue.push_back((destination, moves + 1));
                }
            }
        }

        -1
    }
}
