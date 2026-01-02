impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            if i + 1 < nums.len() && nums[i] == nums[i + 1] {
                return nums[i];
            }
            if i + 2 < nums.len() && nums[i] == nums[i + 2] {
                return nums[i];
            }
            if i + 3 < nums.len() && nums[i] == nums[i + 3] {
                return nums[i];
            }
        }
        -1
    }
}