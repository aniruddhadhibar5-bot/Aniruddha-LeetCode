class Solution {
    func countBits(_ n: Int) -> [Int] {
        var ans = Array(repeating: 0, count: n + 1)
        
        if n == 0 { return ans }
        
        for i in 1...n {
            // ans[i] = ans[i / 2] + (i % 2)
            ans[i] = ans[i >> 1] + (i & 1)
        }
        
        return ans
    }
}
