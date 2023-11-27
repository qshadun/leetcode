struct Graph {
    adj_list: Vec<Vec<(i32, i32)>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
use std::collections::BinaryHeap;
impl Graph {

    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let mut adj_list = vec![vec![]; n as usize];
        for edge in edges {
            adj_list[edge[0] as usize].push((edge[1], edge[2]));
        }
        Graph {
            adj_list,
        }
    }
    
    fn add_edge(&mut self, edge: Vec<i32>) {
        self.adj_list[edge[0] as usize].push((edge[1], edge[2]));
    }
    
    fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        let mut dist: Vec<i32> = vec![i32::MAX; self.adj_list.len()];
        dist[node1 as usize] = 0;
        let mut pq = BinaryHeap::new();
        pq.push((0, node1));

        while let Some((curr_cost, curr_node)) = pq.pop() {
            if curr_node == node2 {
                return -curr_cost;
            }
            if -curr_cost > dist[curr_node as usize] {
                continue;
            }
            for (next_node, next_cost) in self.adj_list[curr_node as usize].iter() {
                let new_cost = next_cost - curr_cost;
                if new_cost < dist[*next_node as usize] {
                    dist[*next_node as usize] = new_cost;
                    pq.push((-new_cost, *next_node));
                }
            }
        }
        -1
    }
}