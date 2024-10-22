impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let total_sum: i32 = nums.iter().sum();
        let mut left_sum = 0; // Initialize the left sum to 0

        for (i, &num) in nums.iter().enumerate() {
            // check if left_sum is equal to rigth sum
            if left_sum == total_sum - left_sum - num {
                return i as i32;    // pivot index found
            }

            // update left_sum
            left_sum += num;
        }

        -1
    }
}