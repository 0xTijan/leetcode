use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagrams: HashMap<Vec<i32>, Vec<String>> = HashMap::new();

        for s in strs {
            // count letters in string with vector of size 26 - each element represents a letter
            let mut char_count = vec![0; 26];

            // count the characters
            for c in s.chars() {
                // convert character into its position in alphabet
                char_count[(c as u8 - b'a') as usize] += 1;
            }

            // add string to hashmap of anagrams
            anagrams.entry(char_count).or_insert(Vec::new()).push(s);
        }

        // return all values in hashmap in vector
        anagrams.into_values().collect()
    }
}