use std::collections::HashMap;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        fn fib_main(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
            if let Some(&result) = memo.get(&n) {
                return result;
            }

            let result = fib_main(n - 1, memo) + fib_main(n - 2, memo);
            memo.insert(n, result);
            result
        }

        // decalare memo - with "edge" cases
        let mut memo: HashMap<i32, i32> = HashMap::from([(0, 0), (1, 1), (2, 1)]);
        fib_main(n, &mut memo)    // call main function
    }
}