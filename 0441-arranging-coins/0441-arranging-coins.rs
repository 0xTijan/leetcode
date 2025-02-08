impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        // quadratic equation using the quadratic formula (x^2 + x - 2n = 0)
        let n: f64 = n as f64;
        return ((-1.0 + (1.0+8.0*n).sqrt()) / 2.0) as i32;
    }
}