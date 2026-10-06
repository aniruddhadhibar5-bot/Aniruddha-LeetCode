class Solution(object):
    def minAddToMakeValid(self, s):
        open_needed = 0
        balance = 0
        
        for char in s:
            if char == '(':
                balance += 1
            else:
                if balance > 0:
                    balance -= 1
                else:
                    open_needed += 1
                    
        return open_needed + balance
