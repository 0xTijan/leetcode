impl Solution {
    pub fn number_of_cuts(n: i32) -> i32 {
        // only one part - whole circle
        if n == 1 {
            return 0;
        }
        if n % 2 == 0 {
            // even parts -> cuts = parts / 2
            return n / 2;
        } else {
            // odd parts -> cuts = parts
            return n;
        }
    }
}