impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, capacity: Vec<i32>) -> i32 {
        let mut capacity = capacity;
        capacity.sort_by(|a, b| b.cmp(a));
        let mut counter = 0;
        let mut total_apples: i32 = apple.iter().sum();

        for i in capacity {
            total_apples -= i;
            counter += 1;
            if total_apples <= 0 {
                break;
            }
        }

        return counter;
    }
}