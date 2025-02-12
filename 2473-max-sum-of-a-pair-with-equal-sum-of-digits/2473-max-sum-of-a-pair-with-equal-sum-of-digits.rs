use std::collections::HashMap;

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut sums: HashMap<i32, Vec<i32>> = HashMap::new();   // sum_of_digits -> highest 2 nums [x, y]
        let mut max_sum: i32 = -1;  // default is -1
        
        for n in nums {
            let sum_of_digits = Self::calculate_sum(n);

            // update hashmap
            let entry = sums.entry(sum_of_digits).or_insert_with(Vec::new);
            if entry.len() < 2 {
                // less then 2 nums - push
                entry.push(n);
            } else {
                // find the smallest number in vector
                let mut min_value = entry[0];
                let mut min_index = 0;

                if entry[1] < min_value {
                    min_value = entry[1];
                    min_index = 1;
                }

                // if n is bigger then the smallest in vector replace it
                if n > min_value {
                    entry[min_index] = n;
                }
            }

            // update max sum
            if let Some(value_in_map) = sums.get(&sum_of_digits) {
                if value_in_map.len() == 2 {
                    let _sum = value_in_map[0] + value_in_map[1];
                    if _sum > max_sum {
                        max_sum = _sum;
                    }
                }
            }
        }

        max_sum
    }

    pub fn calculate_sum(mut num: i32) -> i32 {
        let mut sum = 0;
        while num > 0 {
            sum += num % 10;    // get last digit
            num /= 10;          // remove last digit
        }
        sum
    }
}