from typing import List;

class Solution:
    def arrayStringsAreEqual(self, word1: List[str], word2: List[str]) -> bool:
        g1 = (c for x in word1 for c in x)
        g2 = (c for x in word2 for c in x)
        return all(c1 == c2 for (c1, c2) in zip(g1, g2))

if __name__ == '__main__':
    sol = Solution()
    print(sol.arrayStringsAreEqual(['ab', 'c'], ['a', 'bc']))