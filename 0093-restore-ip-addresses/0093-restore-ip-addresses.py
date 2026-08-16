class Solution:
    def restoreIpAddresses(self, s):
        res = []

        def backtrack(start, path):
            # If we have 4 parts and used all digits, it's a valid IP
            if len(path) == 4 and start == len(s):
                res.append('.'.join(path))
                return
            # If too many parts or digits left, stop
            if len(path) >= 4:
                return

            for length in range(1, 4):  # each part can be 1–3 digits
                if start + length > len(s):
                    break
                segment = s[start:start + length]
                # Skip invalid segments
                if (segment.startswith('0') and len(segment) > 1) or int(segment) > 255:
                    continue
                backtrack(start + length, path + [segment])

        backtrack(0, [])
        return res
