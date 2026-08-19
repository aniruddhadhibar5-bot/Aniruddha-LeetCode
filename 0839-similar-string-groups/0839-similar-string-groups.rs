struct Dsu {
    parent: Vec<usize>,
    components: usize,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            components: n,
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            i
        } else {
            // Path compression optimization
            self.parent[i] = self.find(self.parent[i]);
            self.parent[i]
        }
    }

    fn union(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);
        if root_i != root_j {
            self.parent[root_i] = root_j;
            self.components -= 1;
        }
    }
}

impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let n = strs.len();
        let mut dsu = Dsu::new(n);

        // Helper closure to check if two strings are similar
        let is_similar = |s1: &str, s2: &str| -> bool {
            let mut diff = 0;
            let bytes1 = s1.as_bytes();
            let bytes2 = s2.as_bytes();

            for i in 0..bytes1.len() {
                if bytes1[i] != bytes2[i] {
                    diff += 1;
                    // Optimization: If differences exceed 2, they cannot be similar
                    if diff > 2 {
                        return false;
                    }
                }
            }
            true
        };

        // Check all pairs
        for i in 0..n {
            for j in (i + 1)..n {
                if is_similar(&strs[i], &strs[j]) {
                    dsu.union(i, j);
                }
            }
        }

        dsu.components as i32
    }
}
