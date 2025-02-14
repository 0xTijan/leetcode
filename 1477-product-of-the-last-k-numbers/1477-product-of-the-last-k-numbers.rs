struct ProductOfNumbers {
    prefix_products: Vec<i32>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {
    fn new() -> Self {
        Self { prefix_products: vec![1] } // initialize with 1 for easy multiplication
    }
    
    fn add(&mut self, num: i32) {
        if num == 0 {
            // reset the prefix product list if 0 is encountered
            self.prefix_products = vec![1];
        } else {
            let last_product = *self.prefix_products.last().unwrap();
            self.prefix_products.push(last_product * num);
        }
    }
    
    fn get_product(&self, k: i32) -> i32 {
        let n = self.prefix_products.len();
        if k as usize >= n {
            return 0; // k exceeds the stored range - result must be 0
        }
        self.prefix_products[n - 1] / self.prefix_products[n - 1 - k as usize]
    }
}

/**
 * Your ProductOfNumbers object will be instantiated and called as such:
 * let obj = ProductOfNumbers::new();
 * obj.add(num);
 * let ret_2: i32 = obj.get_product(k);
 */