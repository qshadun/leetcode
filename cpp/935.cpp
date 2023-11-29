class Solution {
public:
    int knightDialer(int n) {
        cache = vector(n + 1, vector(10, 0));
        int ans = 0;
        for (int i = 0; i < 10; i++) {
            ans = (ans + dp(i, n - 1)) % MOD;
        }
        return ans;
    }
private:
    vector<vector<int>> possible_moves = {
            {4, 6},
            {6, 8},
            {7, 9},
            {4, 8},
            {0, 3, 9},
            {},
            {0, 1, 7},
            {2, 6},
            {1, 3},
            {2, 4},
    };
    vector<vector<int>> cache;
    const int MOD = (int) 1e9 + 7;
    int dp(int num, int moves) {
        if (moves == 0) {
            return 1;
        }
        if (cache[moves][num] != 0) {
            return cache[moves][num];
        }
        int result = 0;
        for (int nextMove: possible_moves[num]) {
            result = (result +  dp(nextMove, moves - 1)) % MOD;
        }
        cache[moves][num] = result;
        return result;
    }
};