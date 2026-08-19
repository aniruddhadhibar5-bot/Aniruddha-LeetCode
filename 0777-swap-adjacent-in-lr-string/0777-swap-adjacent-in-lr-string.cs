public class Solution {
    public bool CanTransform(string start, string result) {
        // Lengths must be equal as per constraints, but safe to check
        if (start.Length != result.Length) return false;
        
        int i = 0;
        int j = 0;
        int n = start.Length;
        
        while (i < n || j < n) {
            // Move i to the next non-'X' character in start
            while (i < n && start[i] == 'X') {
                i++;
            }
            
            // Move j to the next non-'X' character in result
            while (j < n && result[j] == 'X') {
                j++;
            }
            
            // If one string reaches the end, both must reach the end
            if (i == n || j == n) {
                return i == n && j == n;
            }
            
            // The relative order of 'L' and 'R' must match
            if (start[i] != result[j]) {
                return false;
            }
            
            // 'L' can only move to the left, so its index in start must be >= its index in result
            if (start[i] == 'L' && i < j) {
                return false;
            }
            
            // 'R' can only move to the right, so its index in start must be <= its index in result
            if (start[i] == 'R' && i > j) {
                return false;
            }
            
            // Move to the next characters
            i++;
            j++;
        }
        
        return true;
    }
}
