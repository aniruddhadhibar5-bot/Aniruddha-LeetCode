MOD = 10**9 + 7

class Fancy:
    def __init__(self):
        self.seq = []
        self.mul = 1
        self.add = 0

    def append(self, val):
        # Normalize val by undoing current transformations
        inv_mul = pow(self.mul, MOD-2, MOD)  # modular inverse of mul
        normalized = (val - self.add) * inv_mul % MOD
        self.seq.append(normalized)

    def addAll(self, inc):
        self.add = (self.add + inc) % MOD

    def multAll(self, m):
        self.mul = (self.mul * m) % MOD
        self.add = (self.add * m) % MOD

    def getIndex(self, idx):
        if idx >= len(self.seq):
            return -1
        return (self.seq[idx] * self.mul + self.add) % MOD
