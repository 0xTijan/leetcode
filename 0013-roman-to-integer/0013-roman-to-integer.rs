use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let roman_values: HashMap<char, i32> = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);

        let mut res = 0;
        let mut prev = 0;

        for x in s.chars().rev() {
            if let Some(curr_val) = roman_values.get(&x) {
                // check if value of prev is smaller then the current value
                if prev > *curr_val {
                    // handle deduction
                    res -= curr_val;
                } else {
                    res += curr_val;
                }
                prev = *curr_val;
            }
        }

        res
    }
}