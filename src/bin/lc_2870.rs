struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut counter = HashMap::<i32, i32>::new();
        for num in nums {
            *counter.entry(num).or_default() += 1;
        }
        let mut ans = 0;
        for &x in counter.values() {
            if x == 1 {
                return -1;
            }
            if x % 3 == 0 {
                ans += x / 3;
            } else {
                ans += x / 3 + 1;
            }
        }
        ans
    }
}

fn main() {
    
}