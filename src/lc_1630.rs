struct Solution;
impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        let m = l.len();
        let mut ans = vec![false; m];
        for i in 0..m {
            let mut sub = nums[l[i] as usize..=r[i] as usize].to_vec();
            sub.sort_unstable();
            let delta = sub[1] - sub[0];
            let mut is_arith = true;
            for j in 2..sub.len() {
                if sub[j] - sub[j - 1] != delta {
                    is_arith = false;
                    break;
                }
            }
            ans[i] = is_arith;
        }
        ans
    }
}