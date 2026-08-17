class Solution:
    def maxProfit(self, prices):
        min_price = float('inf')
        max_profit = 0

        for price in prices:
            # Track the lowest price seen so far
            min_price = min(min_price, price)
            # Calculate profit if sold today
            max_profit = max(max_profit, price - min_price)

        return max_profit
