from typing import List
from collections import defaultdict

class Solution:
    def checkSubarraySum(self, nums: List[int], k: int) -> bool:
        if len(nums) < 2:
            return False
        prefix_sum = [0]
        prefix_sum_dict = defaultdict(list)
        prefix_sum_dict[0].append(0)
        for num in nums:
            prefix_sum.append(num + prefix_sum[-1])
            prefix_sum_dict[prefix_sum[-1]].append(len(prefix_sum) - 1)
            if len(prefix_sum_dict[prefix_sum[-1]]) > 2:
                return True
        total = prefix_sum[-1]
        if total < k:
            return False

        for i in range(len(prefix_sum)):
            if prefix_sum[i] > total-k:
                return False
            
            target = prefix_sum[i] 
            while target <= total:
                if target in prefix_sum_dict and prefix_sum_dict[target][-1] - i > 1:
                    return True
                target += k
        return False


s = Solution()
print(s.checkSubarraySum([1,2,12], 6))
