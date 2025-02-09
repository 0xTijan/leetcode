use std::collections::HashMap;

impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let l = nums.len();
        let total = (l * (l-1)) / 2;    // number of total pairs
        let mut good: i64 = 0;
        let mut count: HashMap<i32, i64> = HashMap::new();

        for i in 0..l {
            let diff = (i as i32) - nums[i]; // for a pair to be good the difference between index and value at index must be the same (i-nums[i] = j-num[j])
            let pair_num: i64 = match count.get(&diff) {
                Some(x) => *x,
                None => 0
            };
            good += pair_num;
            // update the count
            count.insert(diff, pair_num+1);
        }

        (total as i64) - good
    }
}