#include <vector>

class Solution {
public:
    std::vector<int> shuffle(std::vector<int>& nums, int n) {
        std::vector<int> ans(2 * n);
        
        for (int i = 0; i < n; i++) {
            ans[2 * i] = nums[i];       // Places x elements at even indices (0, 2, 4, ...)
            ans[2 * i + 1] = nums[i + n]; // Places y elements at odd indices (1, 3, 5, ...)
        }
        
        return ans;
    }
};
