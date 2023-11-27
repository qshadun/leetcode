use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }
        let mut dist = vec![501 as i16; 1_000_001];
        dist[source as usize] = 0;
        let mut visited = HashSet::new();
        let mut count = 0;
        while visited.len() < routes.len() && count < routes.len() {
            for bus in 0..routes.len() {
                if visited.contains(&bus) {
                    continue;
                }
                let mut min_dist = 501 as i16;
                for i in routes[bus].iter() {
                    min_dist = std::cmp::min(min_dist, dist[*i as usize] + 1)
                }
                if min_dist < 501 {
                    visited.insert(bus);
                    for i in routes[bus].iter() {
                        dist[*i as usize] = std::cmp::min(dist[*i as usize], min_dist);
                    }
                }
            }
            count += 1;
        } 
        if dist[target as usize] != 501 {
            dist[target as usize] as i32
        } else {
            -1
        }
    }
}