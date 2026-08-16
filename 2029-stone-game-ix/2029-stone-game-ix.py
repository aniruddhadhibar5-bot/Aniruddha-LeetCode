class Solution:
    def stoneGameIX(self, stones):
        # Step 1: Count the frequency of each remainder when divided by 3
        cnt0 = 0
        cnt1 = 0
        cnt2 = 0
        
        for stone in stones:
            remainder = stone % 3
            if remainder == 0:
                cnt0 += 1
            elif remainder == 1:
                cnt1 += 1
            else:
                cnt2 += 1
                
        # Step 2: Game theory evaluation based on the parity of 0-remainder stones
        # If there is an even number of 0-stones, they neutralize each other.
        # Alice wins if both remainder-1 and remainder-2 stones are available.
        if cnt0 % 2 == 0:
            return cnt1 > 0 and cnt2 > 0
            
        # If there is an odd number of 0-stones, the turn order dynamics change.
        # Alice wins if one remainder group heavily outnumbers the other by more than 2.
        return cnt1 - cnt2 > 2 or cnt2 - cnt1 > 2
