public class Solution {
    public int KthGrammar(int n, int k) {
        // Base case: the first row always starts with 0
        if (n == 1) return 0;
        
        // Find the parent's position in the previous row
        int parent = KthGrammar(n - 1, (k + 1) / 2);
        
        // If k is odd, it's the same as the parent. If even, it's flipped.
        if (k % 2 == 1) {
            return parent;
        } else {
            return parent == 0 ? 1 : 0;
        }
    }
}
