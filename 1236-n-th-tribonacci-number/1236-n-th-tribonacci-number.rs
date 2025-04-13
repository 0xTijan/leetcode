use std::collections::HashMap;

impl Solution {

    pub fn tribonacci(n: i32) -> i32 {
        let mut memo = HashMap::new();
        Self::_tribonacci(n, &mut memo)
    }

    pub fn _tribonacci(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        if let Some(&val) = memo.get(&n) {
            return val;
        }

        if n == 0 {
            return 0;
        } else if n == 1 || n == 2 {
            return 1;
        }

        let val = Self::_tribonacci(n-3, memo) + Self::_tribonacci(n-2, memo) + Self::_tribonacci(n-1, memo);
        memo.insert(n, val);

        val
    }
}