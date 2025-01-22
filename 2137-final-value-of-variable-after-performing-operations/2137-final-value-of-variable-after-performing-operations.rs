impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut counter = 0;

        for i in operations {
            if i.chars().nth(1) == Some('-') {
                counter -= 1;
            } else {
                counter += 1;
            }
        }
        
        counter
    }
}