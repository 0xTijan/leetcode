impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut substring = s.replace("AB", "").replace("CD", "");

        while substring.contains("AB") || substring.contains("CD") {
            substring = substring.replace("AB", "").replace("CD", "");
        }

        substring.len() as i32
    }
}