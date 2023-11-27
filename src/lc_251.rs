use std::iter::{Peekable, Flatten};
use std::vec::IntoIter;
struct Vector2D {
    iter: Peekable<Flatten<IntoIter<Vec<i32>>>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Vector2D {

    fn new(vec: Vec<Vec<i32>>) -> Self {
        
        let iter = vec.into_iter().flatten().peekable();
        
        Vector2D { iter }
    }
    
    fn next(&mut self) -> i32 {
        self.iter.next().unwrap()
        
    }
    
    fn has_next(&mut self) -> bool {
        self.iter.peek().is_some()
    }
}
