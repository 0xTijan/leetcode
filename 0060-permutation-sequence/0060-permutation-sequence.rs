impl Solution {
    pub fn get_permutation(n: i32, mut k: i32) -> String {
        let mut v: Vec<i32> = (1..=n).collect();
        let mut res: String = "".to_string();

        for i in (2..n+1).rev() {
            // chose correct permutation
            let factorial = Self::factorial(i - 1) as f32;
            let chosen: f32 = k as f32 / factorial;
            println!("{:#?}, {}, {:#?}", chosen, k, factorial);
            let mut element_index = chosen.ceil() as usize;
            if element_index == 0 && chosen == 0.0 {
                element_index = v.len();
            }
            let element = v[element_index - 1];
            res = res + &element.to_string();
            // update v and k
            k = k - factorial as i32 * chosen.floor() as i32;
            println!("new k {}", k);
            v.retain(|&x| x != element);
            println!("new n {:#?}", v);
        }  

        // add last element in v to string
        res = res + &v[0].to_string();

        res
    }

    pub fn factorial(n: i32) -> i32 {
        if n == 0 || n == 1 {
            return 1;
        } else {
            return n * Self::factorial(n - 1);
        }
    }
}