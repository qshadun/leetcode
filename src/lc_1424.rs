struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut q = VecDeque::<(usize, usize)>::new();
        q.push_back((0, 0));
        let mut ans = Vec::new();
        while !q.is_empty() {
            let (r, c) = q.pop_front().unwrap();
            ans.push(nums[r][c]);
            if c == 0 && r + 1 < nums.len() {
                q.push_back((r + 1, 0));
            }
            if c + 1 < nums[r].len() {
                q.push_back((r, c + 1));
            }
        }
        ans
    }
}