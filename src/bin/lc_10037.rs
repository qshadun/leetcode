struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn maximum_set_size(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let s1: HashSet<_> = nums1.into_iter().collect();
        let s2: HashSet<_> = nums2.into_iter().collect();
        let n1 = s1.len();
        let n2 = s2.len();
        let inter = s1.intersection(&s2).count();
        let half = n / 2;
        n.min(half.min(n1 - inter) + half.min(n2 - inter) + inter) as i32
    }
}

fn main() {
    let n = Solution::maximum_set_size(vec![1,2,1,2], vec![1,1,1,1]);
    println!("{}", n);
}