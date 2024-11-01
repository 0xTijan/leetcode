const values: [(i32, &str); 13] = [
    (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"),
    (100, "C"), (90, "XC"), (50, "L"), (40, "XL"),
    (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I")
];

impl Solution {
    pub fn int_to_roman(x: i32) -> String {
        let mut num = x;
        let mut result = String::new();

        // loop through all values
        for &(value, symbol) in &values {
            // while the number is bigger then value from values, reduce the number and add roman letter (of that value) to result
            while num >= value {
                result.push_str(symbol);
                num -= value;
            }
        }

        result
    }
}