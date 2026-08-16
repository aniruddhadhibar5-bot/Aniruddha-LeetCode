class Solution:
    def isNumber(self, s):
        s = s.strip()
        num_seen = False
        dot_seen = False
        e_seen = False

        for i, ch in enumerate(s):
            if ch.isdigit():
                num_seen = True
            elif ch in ['+', '-']:
                # Sign must be at start or right after e/E
                if i > 0 and s[i - 1] not in ['e', 'E']:
                    return False
            elif ch == '.':
                # Dot cannot appear after e/E or appear twice
                if dot_seen or e_seen:
                    return False
                dot_seen = True
            elif ch in ['e', 'E']:
                # e/E must appear once and only after a number
                if e_seen or not num_seen:
                    return False
                e_seen = True
                num_seen = False  # reset for exponent part
            else:
                return False

        return num_seen
