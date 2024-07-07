struct Solution;

impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let target = target as usize;
        let MOD = 1e9 as i32 + 7;
        let mut dp = vec![vec![0; target + 1]; n + 1];
        dp[0][0] = 1;
        for i in 1..(n as usize + 1) {
            let max_j = target.min(i * k);
            for j in 0..(max_j + 1) {
                let mut ways = 0;
                for d in 1..(k.min(j) + 1) {
                    ways = (ways + dp[i-1][j-d]) % MOD;
                }
                dp[i][j] = ways;
            }
        }
        dp[n][target]
    }
}

fn main() {

}