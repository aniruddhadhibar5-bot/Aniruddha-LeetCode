class Solution:
    def maxProfit(self, prices):
        buy1 = buy2 = float('-inf')
        sell1 = sell2 = 0

        for price in prices:
            buy1 = max(buy1, -price)          # first buy
            sell1 = max(sell1, buy1 + price)  # first sell
            buy2 = max(buy2, sell1 - price)   # second buy
            sell2 = max(sell2, buy2 + price)  # second sell

        return sell2
