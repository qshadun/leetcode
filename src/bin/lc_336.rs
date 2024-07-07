struct Solution;

use std::rc::Rc;
use std::boxed::Box;
use std::cell::RefCell;
use std::collections::LinkedList;

#[derive(Default, Debug)] 
struct Trie {
    next: [Option<Box<Trie>>;26],
    idx: Vec<i32>,
    end_index: i32,
}
impl Trie {
    fn new() -> Self {
        Trie {
            next: Default::default(),
            idx: vec![],
            end_index: -1,
        }
    }

    fn build(&mut self, words: &[String]) {
        for (i, w)in words.iter().enumerate() {
            let w: Vec<char> = w.chars().rev().collect();
            let mut cur_node = &mut *self;
            for (j, &c) in w.iter().enumerate() {
                if Trie::is_palindrome(&w[j..]) {
                    cur_node.idx.push(i as i32);
                }
                let next_idx = c as usize - 'a' as usize;

                if cur_node.next[next_idx].is_none() {
                    cur_node.next[next_idx] = Some(Box::new(Trie::new()));
                }
                cur_node = cur_node.next[next_idx].as_deref_mut().unwrap();
            }
            cur_node.end_index = i as i32;
        }
    }

    fn is_palindrome(word_slice: &[char]) -> bool {
        let mut i = 0;
        let mut j = word_slice.len() - 1;
        while i < j {
            if word_slice[i] != word_slice[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}
impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut trie = Trie::new();
        trie.build(&words);
        
        let mut ans = vec![];
        for (i, w) in words.iter().enumerate() {
            let w: Vec<char> = w.chars().collect();
            let mut cur_node = &mut trie;
            let mut has_match = true;
            for (j, &c) in w.iter().enumerate() {
                if cur_node.end_index != -1 && Trie::is_palindrome(&w[j..]) {
                    ans.push(vec![i as i32, cur_node.end_index as i32]);
                }
                match cur_node.next[c as usize - 'a' as usize].as_deref_mut() {
                    Some(next_node) => cur_node = next_node,
                    None => {
                        has_match = false;
                        break;
                    },
                }
            }
            if has_match {
                if cur_node.end_index != -1 && cur_node.end_index != i as i32 {
                    ans.push(vec![i as i32, cur_node.end_index]);
                }
                for j in cur_node.idx.iter() {
                    ans.push(vec![i as i32, *j]);
                }
            }
        }
        ans
    }
}

fn main() {
    let input: Vec<String> = vec!["abcd","dcba","lls","s","sssll"].into_iter().map(|x| String::from(x)).collect();
    let ans = Solution::palindrome_pairs(input);
    print!("{:?}", ans);
}