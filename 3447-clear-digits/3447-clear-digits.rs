impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut clear = String::new();
        
        for c in s.chars() {
            // check if character is a digit
            if c.is_digit(10) {
                // it is a digit: do not push to clear and remove last letter
                clear.pop();
            } else {
                // push letter to clear
                clear.push(c);
            }
        }
        
        clear
    }
}
