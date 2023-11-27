struct Solution;

impl Solution {
    
    pub fn sort_vowels(s: String) -> String {
        let mut a = s.chars()
                    .fold([('A',0), ('E',0), ('I',0), ('O',0), ('U',0), 
                          ('a',0), ('e',0), ('i',0), ('o',0), ('u',0)], 
                        | mut a, c| {
                            match c {
                                'A' => a[0].1 += 1,
                                'E' => a[1].1 += 1,
                                'I' => a[2].1 += 1,
                                'O' => a[3].1 += 1,
                                'U' => a[4].1 += 1,
                                'a' => a[5].1 += 1,
                                'e' => a[6].1 += 1,
                                'i' => a[7].1 += 1,
                                'o' => a[8].1 += 1,
                                'u' => a[9].1 += 1,
                                _ => ()
                            };
                            a
                        }
                    );
        let mut pos = 0;
        s.chars()
         .map(| c | {
            match c {
                'A' | 'E' | 'I' | 'O' | 'U' | 'a' | 'e' | 'i' | 'o' | 'u' => {
                    while a[pos].1 == 0 {
                        pos += 1;
                    }
                    a[pos].1 -= 1;
                    a[pos].0
                },
                _ => c 
            }
         })
         .collect::<String>()                
    }
}

#[cfg(test)]
mod lc_2785_test {
    use super::*;
    #[test]
    fn case1() {
        let ans = Solution::sort_vowels(String::from("leetcode"));
        println!("{:?}", ans);
    }
}