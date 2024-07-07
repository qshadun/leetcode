struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn max_sub_array_len(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut prefix_sum = 0;
        let mut indexes = HashMap::new();
        for (i, num)in nums.into_iter().enumerate() {
            prefix_sum += num;
            if prefix_sum == k {
                ans = i + 1;
            } else {
                let target = prefix_sum - k;
                if indexes.contains_key(&target) {
                    ans = ans.max(i - indexes[&target]);
                }
            }
            indexes.entry(prefix_sum).or_insert(i);
        }
        ans as i32
    }
}
fn main() {

}