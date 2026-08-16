class Solution:
    def multiply(self, num1, num2):
        # Handle zero case
        if num1 == "0" or num2 == "0":
            return "0"

        # Initialize result array
        res = [0] * (len(num1) + len(num2))

        # Reverse both strings for easier multiplication
        num1, num2 = num1[::-1], num2[::-1]

        # Multiply each digit
        for i in range(len(num1)):
            for j in range(len(num2)):
                res[i + j] += int(num1[i]) * int(num2[j])
                # Carry over
                res[i + j + 1] += res[i + j] // 10
                res[i + j] %= 10

        # Remove leading zeros
        while len(res) > 1 and res[-1] == 0:
            res.pop()

        # Convert back to string
        return ''.join(map(str, res[::-1]))
