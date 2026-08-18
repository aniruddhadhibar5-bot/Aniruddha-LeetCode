class Solution {

    /**
     * @param Integer $n
     * @return Integer
     */
    public function reverseBits(int $n): int {
        $res = 0;
        
        for ($i = 0; $i < 32; $i++) {
            // Shift result left and append the rightmost bit of n
            $res = ($res << 1) | ($n & 1);
            // Shift n right to check the next bit
            $n >>= 1;
        }

        // Convert the 64-bit unsigned/positive value into a 32-bit signed integer
        if ($res & 0x80000000) {
            $res -= 0x100000000;
        }

        return $res;
    }
}
