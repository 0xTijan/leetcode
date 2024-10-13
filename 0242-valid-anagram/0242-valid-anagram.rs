use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut letters_s = HashMap::new();

        for x in s.chars() {
            *letters_s.entry(x).or_insert(0) += 1;
        }

        let mut letters_t = HashMap::new();

        for x in t.chars() {
            *letters_t.entry(x).or_insert(0) += 1;
        }

        letters_s == letters_t
    }
}