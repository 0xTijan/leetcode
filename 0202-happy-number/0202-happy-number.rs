use std::collections::HashSet;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut seen = HashSet::new();
        let mut current = n;

        while current != 1 {
            if seen.contains(&current) {
                return false;  // Cycle detected, not a happy number
            }
            seen.insert(current);
            current = Self::sum_of_squares(current);
        }
        
        true  // Reached 1, it's a happy number
    }
    
    fn sum_of_squares(mut n: i32) -> i32 {
        let mut sum = 0;
        while n > 0 {
            let digit = n % 10;
            sum += digit * digit;
            n /= 10;
        }
        sum
    }
}