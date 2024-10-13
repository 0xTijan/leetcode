impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut sum = 0;

        for n in nums {
            sum += n;
            result.push(sum);
        }

        result
    }
}