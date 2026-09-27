class Solution:
    def reverseParentheses(self, s: str) -> str:
        stack = []
        for char in s:
            if char == ')':
                current = []
                while stack and stack[-1] != '(':
                    current.append(stack.pop())
                if stack:
                    stack.pop()  # Remove '('
                stack.extend(current)
            else:
                stack.append(char)
        return "".join(stack)
