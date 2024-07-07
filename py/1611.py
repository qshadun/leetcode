class Solution:
    def minimumOneBitOperations(self, n: int) -> int:
        
        if n == 0:
            return 0
        digits = []
        while n > 0:
            digits.append(n & 1)
            n >>= 1

        count_digits = len(digits)
        dp = [0] * count_digits
        dp[0] = 1
        for i in range(1, count_digits):
            dp[i] = dp[i - 1] * 2 + 1
        sign = 1
        i = count_digits - 1
        ans = 0
        while i >= 0:
            if digits[i] == 1:
                while i > 0 and digits[i - 1] == 1:
                    ans += 1
                    i -= 1
                ans += sign * dp[i]
                sign = -sign
            i -= 1
        
        return ans


if __name__ == '__main__':
    sol = Solution()
    print(sol.minimumOneBitOperations(333))
    print(sol.minimumOneBitOperations(9))
    print(sol.minimumOneBitOperations(6))
    print(sol.minimumOneBitOperations(3))