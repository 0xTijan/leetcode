impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        // k cant be even or divisible by 5 (no 11..1 number can be devided by 2, 4, 6, 8, ..., 5, ...)
        if k % 2 == 0 || k % 5 == 0 {
            return -1;
        }

        let mut len = 1;
        let mut n = 1;

        for i in 0..(k-1) {
            let remainder = n % k;
            if remainder == 0 {
                break;
            }
            len += 1;
            n = remainder * 10 + 1;
        }

        len
    }
}