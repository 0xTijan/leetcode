use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        
        // fill the minheap - using reverse
        for &num in &nums {
            heap.push(Reverse(num as i64)); // convert to i64 to prevent overflow
        }

        let mut count = 0;

        while let Some(Reverse(smallest)) = heap.pop() {
            if smallest >= k as i64 {
                // all elements are >= k - return count
                return count;
            }
            
            // get the second smallest element
            let second_smallest = match heap.pop() {
                Some(Reverse(val)) => val,
                None => break, // doesnt happen since the problem guarantees a solution
            };


            // compute the new element with given formula
            let new_val = smallest * 2 + second_smallest;
            // insert it into the heap
            heap.push(Reverse(new_val));

            count += 1;
        }

        count
    }
}