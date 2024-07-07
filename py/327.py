from typing import List

class Solution:
    def countRangeSum(self, nums: List[int], lower: int, upper: int) -> int:
        n = len(nums)
        sums = [0] * (n + 1)
        for i in range(n):
            sums[i + 1] = sums[i] + nums[i]
        return self.countWhileMergeSort(sums, 0, n + 1, lower, upper)

    def countWhileMergeSort(self, sums, start, end, lower, upper):
        if end - start <= 1:
            return 0
        mid = (start + end) // 2
        count = self.countWhileMergeSort(sums, start, mid, lower, upper) + \
                self.countWhileMergeSort(sums, mid, end, lower, upper)
        j, k, t = mid, mid, mid
        cache = [0] * (end - start)
        r = 0
        for i in range(start, mid):
            while k < end and sums[k] - sums[i] < lower:
                k += 1
            while j < end and sums[j] - sums[i] <= upper:
                j += 1
            while t < end and sums[t] < sums[i]:
                cache[r] = sums[t]
                r += 1
                t += 1
            cache[r] = sums[i]
            r += 1
            count += j - k
        for i in range(t - start):
            sums[start + i] = cache[i]
        return count

        

if __name__ == '__main__':
    sol = Solution()
    print(sol.countRangeSum([-2, 5, -1], -2, 2))