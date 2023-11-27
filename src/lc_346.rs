struct MovingAverage {
    queue: Vec<i32>,
    size: usize,
    head: usize,
    count: usize,
    window_sum: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovingAverage {

    fn new(size: i32) -> Self {
        let queue = vec![0; size as usize];
        Self {
            queue,
            size: size as usize,
            head: 0,
            count: 0,
            window_sum: 0,
        }
    }
    
    fn next(&mut self, val: i32) -> f64 {
        self.count += 1;
        let tail = (self.head + 1) % self.size;
        self.window_sum += val - self.queue[tail];
        self.head = tail;
        self.queue[self.head] = val;
        self.window_sum as f64 / std::cmp::min(self.size, self.count) as f64
    }
}
