impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut s: String = x.abs().to_string().chars().rev().collect();

        if x < 0 {
            // number is negative add -
            s.insert(0, '-');
        }

        match s.parse::<i32>() {
            Err(_) => 0,
            Ok(t) => t,
        }
    }
}