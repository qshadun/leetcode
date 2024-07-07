struct Solution;

impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let n = nums.len();
        let mut prefix_sum = vec![0i64; n + 1];
        for i in 0..n {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i] as i64;
        }
        Solution::count_while_merge_sort(&mut prefix_sum, 0, n + 1, lower as i64, upper as i64)

    }
    fn count_while_merge_sort(mut prefix_sum: &mut[i64], start: usize, end: usize, lower: i64, upper: i64) -> i32 {
        if end - start <= 1 {
            return 0
        }
        let mid = start + (end - start) / 2;
        let mut count = Solution::count_while_merge_sort(prefix_sum, start, mid, lower, upper) + 
                            Solution::count_while_merge_sort(prefix_sum, mid, end, lower, upper);
        let (mut j, mut k, mut t) = (mid, mid, mid);
        let mut cache = vec![0; end - start];
        let mut r = 0;
        for i in start..mid {
            while k < end && prefix_sum[k] - prefix_sum[i] < lower {k += 1};
            while j < end && prefix_sum[j] - prefix_sum[i] <= upper {j += 1};
            count += (j - k) as i32;
            while t < end && prefix_sum[t] < prefix_sum[i] {
                cache[r] = prefix_sum[t];
                r += 1;
                t += 1;
            }
            cache[r] = prefix_sum[i];
            r += 1;
        }
        prefix_sum[start..t].clone_from_slice(&cache[0..r]);
        count 
    }
}

fn main() {
    println!("{}", Solution::count_range_sum(vec![-2, 5, -1], -2, 2));
    
}