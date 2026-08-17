class MyQueue:
    def __init__(self):
        self.inStack = []
        self.outStack = []

    def push(self, x):
        # Always push into inStack
        self.inStack.append(x)

    def pop(self):
        self._move()
        return self.outStack.pop()

    def peek(self):
        self._move()
        return self.outStack[-1]

    def empty(self):
        return not self.inStack and not self.outStack

    def _move(self):
        # Transfer only when outStack is empty
        if not self.outStack:
            while self.inStack:
                self.outStack.append(self.inStack.pop())
