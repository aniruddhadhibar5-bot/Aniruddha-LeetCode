impl Solution {
    pub fn prime_palindrome(mut n: i32) -> i32 {
        loop {
            // 11 is the only even-length prime palindrome
            if 8 <= n && n <= 11 {
                return 11;
            }
            
            // Greedily skip even-digit length intervals
            if 1000 <= n && n <= 9999 {
                n = 10001; // Skip 4-digit numbers
            } else if 100000 <= n && n <= 999999 {
                n = 1000001; // Skip 6-digit numbers
            } else if 10000000 <= n && n <= 99999999 {
                n = 100000001; // Skip 8-digit numbers
            }
            
            // Check palindrome property first (very fast), then test primality
            if Self::is_palindrome(n) && Self::is_prime(n) {
                return n;
            }
            
            n += 1;
        }
    }
    
    // Helper function to check if a number reads the same backward
    fn is_palindrome(mut num: i32) -> bool {
        let original = num;
        let mut reversed = 0;
        while num > 0 {
            reversed = reversed * 10 + num % 10;
            num /= 10;
        }
        original == reversed
    }
    
    // Helper function to test if a number is prime using trial division
    fn is_prime(num: i32) -> bool {
        if num < 2 { return false; }
        if num == 2 { return true; }
        if num % 2 == 0 { return false; }
        
        let mut i = 3;
        while i * i <= num {
            if num % i == 0 {
                return false;
            }
            i += 2;
        }
        true
    }
}
