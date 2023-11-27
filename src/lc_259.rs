struct Solution;


impl Solution {
    pub fn three_sum_smaller(mut nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() < 3 {
            return 0;
        }
        nums.sort_unstable();
        let mut ans = 0;
        for i in 0..nums.len() - 2 {
            ans += Solution::two_sum_smaller(&nums[i+1..], target - nums[i]);
        }
        ans
    }

    fn two_sum_smaller(nums: &[i32], target: i32) -> i32 {
        let mut ans = 0;
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            if nums[left] + nums[right] < target {
                ans += (right - left) as i32;
                left += 1;
            } else {
                right -= 1;
            }
        }
        ans
    }
}