use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen_digits = HashSet::new();

        for n in nums {
            if seen_digits.contains(&n) {
                return true;
            }
            seen_digits.insert(n);
        }

        false
    }
}