impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        
        for x in 1..n+1 {
            if x % 3 == 0 && x % 5 == 0 {
                res.push("FizzBuzz".to_string());
            } else if x % 3 == 0 {
                res.push("Fizz".to_string());
            } else if x % 5 == 0 {
                res.push("Buzz".to_string());
            } else {
                res.push(x.to_string());
            }
        }

        res
    }
}