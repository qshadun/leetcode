#include <vector>
#include <iostream>
using namespace std;

class Solution {
public:
    int countRangeSum(vector<int>& nums, int lower, int upper) {
        int n = nums.size();
        vector<long> prefix_sum(n + 1, 0);
        for (int i = 0; i < n; i++) {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i];
        }
        return countWhileMergeSort(prefix_sum, 0, n + 1, lower, upper);
    }

    int countWhileMergeSort(vector<long>& prefix_sum, int start, int end, int lower, int upper) {
        if (end - start <= 1) {return 0;}
        int mid = start + (end - start) / 2;
        int count = countWhileMergeSort(prefix_sum, start, mid, lower, upper) + countWhileMergeSort(prefix_sum, mid, end, lower, upper);
        int j = mid, k = mid, t = mid;
        long cache[end - start];
        for (int i = start, r = 0; i < mid; ++i, ++r) {
            while(k < end && prefix_sum[k] - prefix_sum[i] < lower) ++k;
            while(j < end && prefix_sum[j] - prefix_sum[i] <= upper) ++j;
            count += j - k;
            while(t < end && prefix_sum[t] < prefix_sum[i]) cache[r++] = prefix_sum[t++];
            cache[r] = prefix_sum[i];
        }
        copy(cache, cache + (t - start), prefix_sum.begin() + start);
        return count;
    }
};

int main() {
    Solution sol;
    vector<int> nums{-2, 5, -1};
    cout << sol.countRangeSum(nums, -2, 2) << endl;
}