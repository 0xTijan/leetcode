impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let mut stack = String::new();      // using string as "stack"
    
        for ch in s.chars() {
            stack.push(ch);                 // add charachter to stack
            
            // check if last letters in stack are the same as part
            if stack.ends_with(&part) {
                // remove this part
                for _ in 0..part.len() {
                    stack.pop();
                }
            }
        }
        
        stack    
    }
}