class Solution(object):
    def fractionToDecimal(self, numerator, denominator):
        if numerator == 0:
            return "0"
        
        res = []
        
        # Handle sign
        if (numerator < 0) ^ (denominator < 0):
            res.append("-")
        
        num, den = abs(numerator), abs(denominator)
        
        # Integer part
        res.append(str(num // den))
        remainder = num % den
        
        if remainder == 0:
            return "".join(res)
        
        res.append(".")
        
        # Fractional part
        seen = {}
        while remainder != 0:
            if remainder in seen:
                res.insert(seen[remainder], "(")
                res.append(")")
                break
            
            seen[remainder] = len(res)
            remainder *= 10
            res.append(str(remainder // den))
            remainder %= den
        
        return "".join(res)
