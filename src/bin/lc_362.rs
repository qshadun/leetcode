use std::collections::VecDeque;
struct HitCounter {
    total: i32,
    q: VecDeque<[i32;2]>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl HitCounter {

    fn new() -> Self {
        HitCounter {
            total: 0,
            q: VecDeque::new(),
        }
    }
    
    fn hit(&mut self, timestamp: i32) {
        if self.q.is_empty() || self.q.back().unwrap()[1] < timestamp {
            self.q.push_back([timestamp, 1]);
        } else {
            self.q.back_mut().unwrap()[1] += 1;
        }
        self.total += 1;
    }
    
    fn get_hits(&mut self, timestamp: i32) -> i32 {
        while !self.q.is_empty() && self.q.front().unwrap()[0] <= timestamp - 300 {
            let arr = self.q.pop_front().unwrap();
            self.total -= arr[1];
        }
        self.total
    }
}

/**
 * Your HitCounter object will be instantiated and called as such:
 * let obj = HitCounter::new();
 * obj.hit(timestamp);
 * let ret_2: i32 = obj.get_hits(timestamp);
 */
fn main() {
    let mut hc = HitCounter::new();
    hc.hit(1);
    hc.hit(2);
    hc.hit(3);
    hc.hit(300);
    println!("{}", hc.get_hits(301));

    let mut tt = (1,2,3);
    tt.0 += 1;
    println!("{:?}", tt);
}