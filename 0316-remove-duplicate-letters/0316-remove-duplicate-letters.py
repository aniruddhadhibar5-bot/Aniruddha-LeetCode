class Solution(object):
    def removeDuplicateLetters(self, s):
        # Count remaining occurrences of each character
        from collections import Counter
        counter = Counter(s)
        
        stack = []
        in_stack = set()
        
        for ch in s:
            counter[ch] -= 1
            if ch in in_stack:
                continue
            # Maintain lexicographical order
            while stack and ch < stack[-1] and counter[stack[-1]] > 0:
                removed = stack.pop()
                in_stack.remove(removed)
            stack.append(ch)
            in_stack.add(ch)
        
        return "".join(stack)
