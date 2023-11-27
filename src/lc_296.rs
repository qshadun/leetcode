struct Solution;

impl Solution {
    pub fn min_total_distance(grid: Vec<Vec<i32>>) -> i32 {
        let rows = Solution::collect_rows(&grid);
        let cols = Solution::collect_cols(&grid);
        Solution::min_dist_1d(rows) + Solution::min_dist_1d(cols)
    }

    fn collect_rows(grid: &Vec<Vec<i32>>) -> Vec<i32> {
        let mut rows = vec![];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    rows.push(i as i32);
                }
            }
        }
        rows
    }

    fn collect_cols(grid: &Vec<Vec<i32>>) -> Vec<i32> {
        let mut cols = vec![];
        for j in 0..grid[0].len() {
            for i in 0..grid.len() {
                if grid[i][j] == 1 {
                    cols.push(j as i32);
                }
            }
        }
        cols
    }

    fn min_dist_1d(points: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = points.len() - 1;
        let mut result = 0;
        while i < j {
            result += points[j] - points[i];
            i += 1;
            j-= 1;
        }
        result

    }
}