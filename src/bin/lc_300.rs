struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut sub = vec![nums[0]];
        for i in 1..nums.len() {
            if nums[i] > sub[sub.len() - 1] {
                sub.push(nums[i]);
            } else {
                match sub.binary_search(&nums[i]) {
                    Err(idx) => sub[idx] = nums[i],
                    _ => {}
                }
            }
        }
        sub.len() as i32
    }
}

fn main() {

}