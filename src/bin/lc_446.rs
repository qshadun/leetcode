struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let n = nums.len();
        let mut dp = vec![HashMap::<i64, i32>::new(); n];
        for i in 0..n {
            for j in 0..i {
                let diff = nums[i] as i64 - nums[j] as i64;
                let &sum = dp[j].get(&diff).unwrap_or(&0);
                ans += sum;
                *dp[i].entry(diff).or_insert(0) += sum + 1;

            }
        }
        ans
    }
}

fn main() {
    
}