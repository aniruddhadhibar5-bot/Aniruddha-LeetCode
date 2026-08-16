class Solution:
    def simplifyPath(self, path):
        stack = []
        parts = path.split('/')

        for part in parts:
            if part == '' or part == '.':
                # Skip empty or current directory
                continue
            elif part == '..':
                # Go up one level if possible
                if stack:
                    stack.pop()
            else:
                # Valid directory/file name
                stack.append(part)

        return '/' + '/'.join(stack)
