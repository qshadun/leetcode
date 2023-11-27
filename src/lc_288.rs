use std::collections::HashMap;
struct ValidWordAbbr {
    dict: HashMap<String, String>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ValidWordAbbr {

    fn new(dictionary: Vec<String>) -> Self {
        let mut dict: HashMap<String, String> = HashMap::new();
        for word in dictionary {
            let abb = ValidWordAbbr::abbrev(&word);
            match dict.get(&abb) {
                Some(x) => {
                    if !(*x).is_empty() && *x != word {
                        dict.insert(abb, String::from(""));
                    }
                }
                None => {
                    dict.insert(abb, word);
                },
            }
        }
        ValidWordAbbr { dict, }
    }
    
    fn is_unique(&self, word: String) -> bool {
        let abb = ValidWordAbbr::abbrev(&word);
        match self.dict.get(&abb) {
            Some(x) => *x == word,
            None => true,
        }
    }

    fn abbrev(word: &str) -> String {
        if word.len() <= 2 {
            word.to_string()
        } else {
            format!("{}{}{}", word.chars().next().unwrap(), word.len() - 2, word.chars().last().unwrap())
        }
    }
}