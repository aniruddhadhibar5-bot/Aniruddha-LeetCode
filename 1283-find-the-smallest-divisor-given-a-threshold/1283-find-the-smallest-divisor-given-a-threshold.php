class Solution {

    /**
     * @param Integer[] $nums
     * @param Integer $threshold
     * @return Integer
     */
    function smallestDivisor($nums, $threshold) {
        $low = 1;
        $high = max($nums);
        $ans = $high;

        // Binary search for the smallest valid divisor
        while ($low <= $high) {
            $mid = $low + intdiv($high - $low, 2);
            
            if ($this->getSum($nums, $mid) <= $threshold) {
                $ans = $mid;       // Current divisor is valid, record it
                $high = $mid - 1;  // Try to find a smaller one on the left
            } else {
                $low = $mid + 1;   // Sum is too large, need a larger divisor
            }
        }

        return $ans;
    }

    /**
     * Helper function to calculate the sum of divisions rounded up
     */
    private function getSum($nums, $divisor) {
        $sum = 0;
        foreach ($nums as $num) {
            // Integer formula for ceiling division: ceil(a / b) = (a + b - 1) / b
            $sum += intdiv($num + $divisor - 1, $divisor);
        }
        return $sum;
    }
}
