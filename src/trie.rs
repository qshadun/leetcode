use std::boxed::Box;

#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_end: bool,
}


/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

impl Trie {

    fn new() -> Self {
        Trie {
            children: Default::default(),
            is_end: false,
        }
    }

    fn insert(&mut self, word: String) {
        if word.is_empty() {
            return
        }
        
        let mut cur_node = self;
        for c in word.chars() {
            let idx = c as usize - 'a' as usize;
            if cur_node.children[idx].is_none() {
                cur_node.children[idx] = Some(Box::new(Trie::new()));
                    
            }
            cur_node = cur_node.children[idx].as_deref_mut().unwrap();
        } 
        cur_node.is_end = true;

    }

    fn search(&self, word: String) -> bool {
        let mut cur_node = self;
        for c in word.chars() {
            let idx = c as usize - 'a' as usize;
            if cur_node.children[idx].is_none() {
                return false;
            }
            cur_node = cur_node.children[idx].as_ref().unwrap();
        } 
        cur_node.is_end
    }
    
    
    fn starts_with(&self, prefix: String) -> bool {
        let mut cur_node = self;
        for c in prefix.chars() {
            let idx = c as usize - 'a' as usize;
            if cur_node.children[idx].is_none() {
                return false;
            }
            cur_node = cur_node.children[idx].as_ref().unwrap();
        } 
        true
    }
}
