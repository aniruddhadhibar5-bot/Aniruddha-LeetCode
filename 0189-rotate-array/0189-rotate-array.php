class Solution {

    /**
     * @param Integer[] $nums
     * @param Integer $k
     * @return NULL
     */
    public function rotate(array &$nums, int $k): void {
        $n = count($nums);
        if ($n < 2) {
            return;
        }

        // Normalize k to avoid redundant full rotations
        $k = $k % $n;
        if ($k === 0) {
            return;
        }

        // 1. Reverse the entire array
        $this->reverse($nums, 0, $n - 1);
        
        // 2. Reverse the first k elements
        $this->reverse($nums, 0, $k - 1);
        
        // 3. Reverse the remaining n - k elements
        $this->reverse($nums, $k, $n - 1);
    }

    /**
     * Helper function to reverse a portion of the array in-place
     */
    private function reverse(array &$nums, int $left, int $right): void {
        while ($left < $right) {
            $temp = $nums[$left];
            $nums[$left] = $nums[$right];
            $nums[$right] = $temp;
            $left++;
            $right--;
        }
    }
}
