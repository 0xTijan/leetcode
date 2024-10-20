impl Solution {
    pub fn my_sqrt(num: i32) -> i32 {
        if num <= 0 {
            return 0;
        }

        let mut x: f64 = num as f64;

        // calculate root using newthons method
        loop {
            let root = 0.5 * (x + ((num as f64) / x));
            if (root-x).abs() < 0.0000001 {
                break;
            }
            x = root;
        }

        println!("{:?}", x);
        x.floor() as i32
    }
}