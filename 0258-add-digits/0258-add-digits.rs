impl Solution {
    pub fn add_digits(mut num: i32) -> i32 {
        // solve with recursion
        if num < 10 {
            return num
        }

        // sum all integers
        let mut sum = 0;
        while num > 0 {
            sum += num % 10;
            num = num / 10;
        }

        // call this function with that sum
        Self::add_digits(sum)
    }
}