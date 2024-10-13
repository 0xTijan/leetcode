use std::collections::HashMap;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut magazine_count = HashMap::new();

        for x in magazine.chars() {
            *magazine_count.entry(x).or_insert(0) += 1;
        }

        for x in ransom_note.chars() {
            let count = magazine_count.entry(x).or_insert(0);
            if *count == 0 {
                return false;
            }
            *count -= 1;
        }

        true
    }
}