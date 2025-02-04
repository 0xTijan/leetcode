use std::cmp::max;

// slightly modified version of kadene's algorithm

impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut max_sum = nums[0];
        let mut current_sum = nums[0];

        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] { 
                // add to existing subarray
                current_sum += nums[i];
            } else {
                // start a new subarray
                current_sum = nums[i];
            }
            max_sum = max(max_sum, current_sum);
        }

        max_sum
    }
}
