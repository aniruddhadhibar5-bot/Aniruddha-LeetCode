class Solution(object):
    def numSteps(self, s):
        # 1. Convert the binary string to a base-2 integer
        num = int(s, 2)
        steps = 0
        
        # 2. Simulate the process until we reach 1
        while num > 1:
            if num % 2 == 0:
                num //= 2
            else:
                num += 1
            steps += 1
            
        return steps
