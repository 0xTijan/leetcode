impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
    
        for i in (0..digits.len()).rev() {
            // if less then 9 increment and return
            if digits[i] < 9 {
                digits[i] += 1;
                return digits;
            }
            // else set to 0 and continue to next
            digits[i] = 0;
        }
        
        // all digits are 9 - add 1 to start
        let mut result = vec![1];
        result.extend(digits);
        result
    }
}