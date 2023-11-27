struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        for edge in adjacent_pairs.iter() {
            let a = edge[0];
            let b = edge[1];
            let mut n1 = graph.entry(a).or_insert(vec![]);
            n1.push(b);
            let mut n2 = graph.entry(b).or_insert(vec![]);
            n2.push(a);
        }
        
        let mut ans = vec![];
        let mut prev = 0;
        let mut cur = 0;
        for (k, v) in graph.iter() {
            if v.len() == 1 {
                prev = *k;
                cur = v[0];
                break;
            }
        }
        for _ in 2..graph.len() {
            for num in graph[cur] {
                if num != prev {
                    prev = cur;
                    cur = num;
                    ans.push(num);
                    break;
                }
            }
        }
        ans
    }
}