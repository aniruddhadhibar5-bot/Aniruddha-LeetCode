from collections import deque

class MyStack:
    def __init__(self):
        self.q1 = deque()
        self.q2 = deque()

    def push(self, x):
        # Push into q2
        self.q2.append(x)
        # Move everything from q1 to q2
        while self.q1:
            self.q2.append(self.q1.popleft())
        # Swap queues
        self.q1, self.q2 = self.q2, self.q1

    def pop(self):
        return self.q1.popleft()

    def top(self):
        return self.q1[0]

    def empty(self):
        return not self.q1
