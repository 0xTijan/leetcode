impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // make sure that the number is positive
        if x < 0 {
            return false;
        }

        // convert to string and reverse digits
        let x_string: String = x.to_string().chars().rev().collect();
        // parse to i32
        let y_result = x_string.parse::<i32>();

        match y_result {
            Ok(y) => {
                // compare
                if x == y {
                    return true;
                }
            },
            // parsing failed
            Err(_) => return false
        }

        false
    }
}