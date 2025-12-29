impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort();
        let mut d: i32 = arr[0] - arr[1];
  
        for i in 2..arr.len() {
            let prev = arr[i-1];
            let curr = arr[i];
            let new_d = prev - curr;
            if d != new_d {
                return false;
            }
        }

        true
    }
}