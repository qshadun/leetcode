#include <vector>
#include <iostream>
using namespace std;

class Solution {
public:
    int threeSumSmaller(vector<int>& nums, int target) {
        sort(nums.begin(), nums.end());
        int ans = 0;
        for (int i = 0; i < nums.size() - 2; ++i) {
            ans += twoSumSmaller(nums, i + 1, target - nums[i]);
        }
        return ans;
    }

    int twoSumSmaller(vector<int>& nums, int start_index, int target) {
        int ans = 0;
        int left = start_index;
        int right = nums.size() - 1;
        while (left < right) {
            if (nums[left] + nums[right] < target) {
                ans += right - left;
                ++left;
            } else {
                --right;
            }
        }
        return ans;
    }
};

int main() {
    Solution sol;
    vector<int> vi{1, 2, 3};
    cout <<sol.threeSumSmaller(vi, 2) << endl;
    int const x = 5;
    cout << x << endl;

}