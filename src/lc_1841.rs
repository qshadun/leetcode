struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        let mut counter: HashMap<i32, i32> = HashMap::new();
        let mut ans = 0;
        const MOD: i32 = 1e9 as i32 + 7;
        for num in nums {
            let delta = num - Solution::rev(num);
            let v: &mut i32 = counter.entry(delta).or_insert(0);
            ans = (ans + *v) % MOD;
            *v += 1;
        }
        ans
    }

    fn rev(mut num: i32) -> i32 {
        let mut ans = 0;
        while num != 0 {
            ans = ans * 10 + num % 10;
            num /= 10;
        }
        ans
    }
}