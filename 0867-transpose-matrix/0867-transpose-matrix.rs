impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows = matrix.len();
        let cols = matrix[0].len();
        
        // Initialize a new result matrix with swapped dimensions (cols x rows)
        let mut result = vec![vec![0; rows]; cols];
        
        for r in 0..rows {
            for c in 0..cols {
                result[c][r] = matrix[r][c];
            }
        }
        
        result
    }
}
