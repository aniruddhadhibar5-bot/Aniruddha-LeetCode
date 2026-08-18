class Solution {

    /**
     * @param Integer $n
     * @return Integer
     */
    public function hammingWeight(int $n): int {
        $count = 0;
        
        while ($n > 0) {
            // Clear the lowest set bit
            $n &= ($n - 1);
            $count++;
        }
        
        return $count;
    }
}
