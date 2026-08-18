object Solution {
    def countPrimes(n: Int): Int = {
        if (n <= 2) return 0
        
        // false means prime, true means composite
        val isNotPrime = new Array[Boolean](n)
        
        // Sieve process
        var i = 2
        while (i * i < n) {
            if (!isNotPrime(i)) {
                var j = i * i
                while (j < n) {
                    isNotPrime(j) = true
                    j += i
                }
            }
            i += 1
        }
        
        // Count all remaining primes
        var count = 0
        var p = 2
        while (p < n) {
            if (!isNotPrime(p)) {
                count += 1
            }
            p += 1
        }
        
        count
    }
}
