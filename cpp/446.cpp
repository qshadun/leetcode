#include <unordered_map>
#include <vector>

using namespace std;

#pragma GCC optimize("O3", "unroll-loops")
class Solution {
public:
    int numberOfArithmeticSlices(vector<int>& nums) {
        int n = nums.size();
        int ans = 0;
        vector<unordered_map<long long, int>> dp(n);
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < i; j++) {
                long long diff = (long long) nums[i] - (long long) nums[j];
                int sum = 0;
                if (dp[j].find(diff) != dp[j].end()) {
                    sum = dp[j][diff];
                }
                dp[i][diff] += sum + 1;
                ans += sum;
            }
        }
        return ans;
    }
};