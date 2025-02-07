use std::collections::HashMap;

impl Solution {
    pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ball_color: HashMap<i32, i32> = HashMap::new();     // ball -> color
        let mut color_freq: HashMap<i32, i32> = HashMap::new();     // color -> frequency
        let mut result: Vec<i32> = vec![0; queries.len()];          // result (number of colors after each query)

        for i in 0..queries.len() {
            let ball = queries[i][0];
            let color = queries[i][1];

            if let Some(prev_color) = ball_color.get(&ball) {
                // ball already exists
                // decrease old color - if 0 delete
                color_freq.insert(*prev_color, color_freq[&prev_color] - 1); 
                if color_freq[&prev_color] == 0 {
                    // no balls of this color anymore - delete
                    color_freq.remove(&prev_color); 
                }
            }

            // update ball color
            ball_color.insert(ball, color);
            // increment new color if exists - otherwise set to 1 (its a new color)
            color_freq.entry(color).and_modify(|v| *v += 1).or_insert(1);

            // set the number of distinc colors
            result[i] = color_freq.len() as i32;
        }

        result
    }
}