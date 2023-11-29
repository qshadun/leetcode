#[allow(dead_code)]
#[allow(non_snake_case)]
struct Solution;

#[allow(dead_code)]
#[allow(non_snake_case)]
impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        let moves = vec![
            vec![4, 6],
            vec![6, 8],
            vec![7, 9],
            vec![4, 8],
            vec![3, 9, 0],
            vec![],
            vec![1, 7, 0],
            vec![2, 6],
            vec![1, 3],
            vec![2, 4]
        ];
        let MOD = i32::pow(10, 9) + 7;
        let mut ans = 0;
        let mut dp = vec![vec![0; 10]; n as usize];
        for i in 0..10 {
            dp[0][i] = 1;
        }
        for remain in 1..n as usize {
            for i in 0..10usize {
                for &next_pos in moves[i].iter() {
                    dp[remain][i] = (dp[remain][i] + dp[remain - 1][next_pos]) % MOD;
                }
                
            }
        }
        for i in 0..10 {
            ans = (ans + dp[n as usize - 1][i]) % MOD;
        }

        ans

    }
}

fn main() {

}