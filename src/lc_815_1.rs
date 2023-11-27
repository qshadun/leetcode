struct Solution;

use std::collections::{HashSet, HashMap, VecDeque};
impl Solution {
    pub fn num_buses_to_destination(mut routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }
        let n = routes.len();
        for route in routes.iter_mut() {
            route.sort();
        }

        let mut bus_to_bus: Vec<Vec<usize>> = vec![vec![]; n];
        for i in 0..n-1 {
            for j in i+1..n {
                if Solution::has_common_stop(&routes[i], &routes[j]) {
                    bus_to_bus[i].push(j);
                    bus_to_bus[j].push(i);
                }
            }
        }
        let mut visited: HashSet<usize> = HashSet::new();
        let mut dq: VecDeque<usize> = VecDeque::new();
        for bus in 0..n {
            if Solution::is_stop_at(&routes[bus], source) {
                visited.insert(bus);
                dq.push_back(bus);
            }
            
        }
        let mut step = 1;
        while !dq.is_empty() {
            let cur_size = dq.len();
            for _ in 0..cur_size {
                let bus = dq.pop_front().unwrap();
                if Solution::is_stop_at(&routes[bus], target) {
                    return step;
                }
                for next_bus in bus_to_bus[bus].iter() {
                    if !visited.contains(next_bus) {
                        visited.insert(*next_bus);
                        dq.push_back(*next_bus);
                    }
                }
            }
            step += 1;
        }
        -1
    }

    fn is_stop_at(route: &Vec<i32>, stop: i32) -> bool {
        match route.iter().find(|&&x| x == stop) {
            Some(_) => true,
            None => false,
        }
    }

    fn has_common_stop(route1: &Vec<i32>, route2: &Vec<i32>) -> bool {
        let mut i = 0;
        let mut j = 0;
        while i < route1.len() && j < route2.len() {
            if route1[i] == route2[j] {
                return true;
            }
            if route1[i] < route2[j] {
                i += 1;
            } else {
                j += 1;
            }
        }
        false
    }
}

#[cfg(test)]
mod lc_815_test {
    use super::*;
    #[test]
    fn case1() {
        let ans = Solution::num_buses_to_destination(vec![vec![1,2,7],vec![3,6,7]], 1, 6);
        println!("{:?}", ans);
    }
}
