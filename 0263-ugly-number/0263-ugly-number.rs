impl Solution {
    pub fn is_ugly(mut n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        
        for factor in [2, 3, 5].iter() {
            while n % factor == 0 {
                n /= factor;
            }
        }
        
        n == 1
    }
}